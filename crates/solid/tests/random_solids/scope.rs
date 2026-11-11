//! The scope rule: which cases may ask the exact kernel for a curve it does
//! not build, read off the walls their leaves are bounded by, so that a
//! decline as unsupported is counted rather than failed.
//!
//! #552 split it in two: what asks beyond the conics — a cone against
//! anything off its axis, where their leaves' boxes meet — and the conics a
//! case's planes meet its cylinders and cones along, told kind by kind. The
//! kernel builds the conics, so only the first excuses a decline.
//!
//! Two walls at a skew angle, round or elliptic, ask nothing since #577
//! builds the curve they meet along — but for the cases it declines, two
//! walls nearly parallel past what decision 11 lays: within the kernel's
//! tolerance of each other along a stretch rather than at points, as
//! `relation/nearly_parallel.rs` measures it, leaning a sliver and meeting
//! at all, or closer than a chord's sag (`Case::asks_of_walls_along_a_stretch`).
//! A cone against a cylinder, an elliptic wall or a cone of another axis
//! asks nothing either since #577, but where the kernel still declines it:
//! within a hair of coaxial, an elliptic wall about its axis, a ruling along
//! the other's axis or two cones each within the other's slope, an apex on
//! the other wall, a curve on a cone face holding its apex, closer than a
//! chord's sag, and grazing
//! without meeting (`Case::asks_of_a_cone_off_its_axis`). A sphere or a
//! torus against a cone off its axis still asks.
//!
//! A sphere or a torus (#576) is built along circles alone: against a plane
//! square to its axis or holding it, and against a cylinder, a cone, a
//! sphere or a torus about that very axis. Any other wall whose leaf's box
//! meets the box of the part of the round the turn drew (#625) asks beyond
//! those circles — a plane at a slant, a block's side, a hole drilled off
//! the axis, a cone off it — and the kernel may decline the case as
//! unsupported, which is counted apart from the rule for the conics
//! (`asks_a_round_beyond_its_circles`).

use std::f64::consts::{PI, TAU};

use cao_solid::brep::{Cone, Cylinder, EllipticCylinder};
use cao_solid::profile::Run;
use glam::{DVec2, DVec3};

use super::{Case, Leaf, Outline, Oval};

/// The box a leaf spans, or nothing for a leaf that is no solid.
type Spanned = Option<(DVec3, DVec3)>;

/// A surface a leaf is bounded by, as the scope rule reads it: a plane by its
/// normal and a point of it, a cylinder, an elliptic cylinder or a cone by
/// its axis and a point of the axis, a cone besides by the furthest its leaf
/// reaches from the axis, and a sphere or a torus by the axis it was turned
/// about, the point of it its circle's centre stands level with and the box
/// the arc it was turned from sweeps over the leaf's turn.
enum Wall {
    Plane {
        normal: DVec3,
        point: DVec3,
    },
    Cylinder {
        axis: DVec3,
        point: DVec3,
    },
    Elliptic {
        axis: DVec3,
        point: DVec3,
    },
    Cone {
        axis: DVec3,
        point: DVec3,
        radius: f64,
    },
    Sphere {
        axis: DVec3,
        point: DVec3,
        swept: Spanned,
    },
    Torus {
        axis: DVec3,
        point: DVec3,
        swept: Spanned,
    },
}

/// How close to parallel, square or on one line two walls are taken as so:
/// the rounding of a direction of unit length, and that share of the reach.
const ALIGNED: f64 = 1e-12;

/// How far past a cone's widest rim, as a share of the reach, a plane
/// parallel to its axis still asks: ten times the kernel's tolerance, which
/// tells a face clear of another only four tolerances off it
/// (`combine/clear.rs`). Nearer, the plane grazes the rim within what the
/// kernel can tell, and may cut it along a hyperbola it does not build.
const GRAZED: f64 = 1e-8;

impl Wall {
    /// Whether this wall meets a cone of `axis` through `point`, whose leaf
    /// reaches `radius` from the axis, in a curve the kernel does not build:
    /// read for a plane — anything but square to the axis, holding it or
    /// parallel to it further off than the leaf reaches by more than the
    /// kernel tells clear — and for a sphere or a torus off the cone's axis.
    /// A cylinder, an elliptic wall or a cone is read by
    /// `asks_of_a_cone_off_its_axis` since #577 builds them.
    fn asks_of_a_cone(&self, axis: DVec3, point: DVec3, radius: f64, reach: f64) -> bool {
        let axis = axis.normalize();
        match *self {
            Wall::Plane { normal, point: on } => {
                let normal = normal.normalize();
                let square = normal.cross(axis).length() <= ALIGNED;
                let off = normal.dot(point - on).abs();
                let parallel = normal.dot(axis).abs() <= ALIGNED;
                let clear = parallel && (off <= ALIGNED * reach || off > radius + GRAZED * reach);
                !square && !clear
            }
            Wall::Cylinder {
                axis: other,
                point: on,
            }
            | Wall::Cone {
                axis: other,
                point: on,
                ..
            }
            | Wall::Sphere {
                axis: other,
                point: on,
                ..
            }
            | Wall::Torus {
                axis: other,
                point: on,
                ..
            } => !coaxial(other, on, axis, point, reach),
            Wall::Elliptic { .. } => true,
        }
    }

    /// Whether this wall meets a sphere or a torus turned about `axis`
    /// through `point` in a curve the kernel does not build (#576): anything
    /// but a plane square to the axis or holding it, or a cylinder, a cone,
    /// a sphere or a torus about the same line.
    fn asks_of_a_round(&self, axis: DVec3, point: DVec3, reach: f64) -> bool {
        let axis = axis.normalize();
        match *self {
            Wall::Plane { normal, point: on } => {
                let normal = normal.normalize();
                let square = normal.cross(axis).length() <= ALIGNED;
                let holding = normal.dot(axis).abs() <= ALIGNED
                    && normal.dot(point - on).abs() <= ALIGNED * reach;
                !square && !holding
            }
            Wall::Cylinder {
                axis: other,
                point: on,
            }
            | Wall::Cone {
                axis: other,
                point: on,
                ..
            }
            | Wall::Sphere {
                axis: other,
                point: on,
                ..
            }
            | Wall::Torus {
                axis: other,
                point: on,
                ..
            } => !coaxial(other, on, axis, point, reach),
            Wall::Elliptic { .. } => true,
        }
    }

    /// The box the part of a sphere or a torus the leaf's turn drew spans,
    /// as the kernel boxes a face on one by its own extent (#625) — the
    /// arc's sweep, not the whole surface nor the whole leaf: a shaft's
    /// rounded end reaches no further along the shaft than the cap; none for
    /// any other wall.
    fn round_box(&self) -> Spanned {
        match *self {
            Wall::Sphere { swept, .. } | Wall::Torus { swept, .. } => swept,
            Wall::Plane { .. }
            | Wall::Cylinder { .. }
            | Wall::Elliptic { .. }
            | Wall::Cone { .. } => None,
        }
    }
}

/// Whether the line along `other` through `on` is the line along `axis`
/// through `point`, within the rounding of a direction and that share of
/// the reach.
fn coaxial(other: DVec3, on: DVec3, axis: DVec3, point: DVec3, reach: f64) -> bool {
    let parallel = other.normalize().cross(axis).length() <= ALIGNED;
    let off = (on - point) - axis * (on - point).dot(axis);
    parallel && off.length() <= ALIGNED * reach
}

/// The planes a leaf is bounded by, the cylinders, the cones, the spheres
/// and the tori: a turn's planes square to its axis at each end of its
/// section and, short of a whole turn, the two it starts and ends on; its
/// cylinder, its cone when an edge of its section slopes, and for each edge
/// that bows a sphere or a torus as the arc's centre stands on the axis or
/// off it; a prism's two ends, its sides and the cylinders of its arcs,
/// circular or elliptic.
fn walls(leaf: &Leaf) -> Vec<Wall> {
    if let Some(turned) = leaf.as_turned() {
        let swept = turned.swept(1.0);
        let (axis, point) = (swept.along, swept.origin);
        let mut walls: Vec<Wall> = turned
            .section
            .ends()
            .iter()
            .map(|end| Wall::Plane {
                normal: axis,
                point: point + axis * *end,
            })
            .collect();
        if !turned.is_whole() {
            let end = DVec2::from_angle(turned.angle());
            walls.push(Wall::Plane {
                normal: swept.onward,
                point,
            });
            walls.push(Wall::Plane {
                normal: swept.onward * end.x - swept.out * end.y,
                point,
            });
        }
        walls.push(Wall::Cylinder { axis, point });
        if turned.section.slopes() {
            let radius = turned
                .section
                .pieces()
                .iter()
                .map(|piece| piece.away[1].max(piece.ending[1]))
                .fold(0.0, f64::max);
            walls.push(Wall::Cone {
                axis,
                point,
                radius,
            });
        }
        for piece in turned.section.pieces() {
            for (edge, bow) in piece.bows.into_iter().enumerate() {
                let Some(bow) = bow else {
                    continue;
                };
                let point = point + axis * bow.center.x;
                let swept = turned.arc_bounds(bow, piece.chord(edge), piece.side);
                walls.push(if bow.center.y == 0.0 {
                    Wall::Sphere { axis, point, swept }
                } else {
                    Wall::Torus { axis, point, swept }
                });
            }
        }
        return walls;
    }
    let Leaf::Prism {
        plane,
        outline,
        height,
    } = leaf
    else {
        return Vec::new();
    };
    let (base, u, v) = plane.frame();
    let normal = u.cross(v);
    let at = |place: DVec2| base + u * place.x + v * place.y;
    let mut walls = vec![
        Wall::Plane {
            normal,
            point: base,
        },
        Wall::Plane {
            normal,
            point: base + normal * *height,
        },
    ];
    let sides = |low: DVec2, high: DVec2| {
        [(u, low), (u, high), (v, low), (v, high)].map(|(normal, place)| Wall::Plane {
            normal,
            point: at(place),
        })
    };
    let round = |center: DVec2| Wall::Cylinder {
        axis: normal,
        point: at(center),
    };
    match outline {
        Outline::Rectangle { low, high } => walls.extend(sides(*low, *high)),
        Outline::Circle { center, .. } | Outline::Ring { center, .. } => walls.push(round(*center)),
        Outline::Rounded { low, high, radius } => {
            walls.extend(sides(*low, *high));
            walls.extend(
                [
                    *low + *radius,
                    *high - *radius,
                    DVec2::new(low.x + radius, high.y - radius),
                    DVec2::new(high.x - radius, low.y + radius),
                ]
                .map(round),
            );
        }
        Outline::Slot { from, to, radius } => {
            walls.extend(sides(from.min(*to) - *radius, from.max(*to) + *radius));
            walls.extend([*from, *to].map(round));
        }
        Outline::Star { corners, .. } => {
            walls.extend((0..corners.len()).map(|index| {
                let side = corners[(index + 1) % corners.len()] - corners[index];
                Wall::Plane {
                    normal: u * side.y - v * side.x,
                    point: at(corners[index]),
                }
            }));
        }
        Outline::Ellipse { .. }
        | Outline::Chorded { .. }
        | Outline::OvalSlot { .. }
        | Outline::OvalRing { .. } => {
            let (contour, holes) = outline
                .oval_contours()
                .expect("an outline an ellipse bounds");
            for contour in std::iter::once(&contour).chain(&holes) {
                for (index, run) in contour.runs.iter().enumerate() {
                    let corner = contour.corners[index];
                    walls.push(match *run {
                        Run::Straight => {
                            let next = contour.corners[(index + 1) % contour.corners.len()];
                            let side = next - corner;
                            Wall::Plane {
                                normal: u * side.y - v * side.x,
                                point: at(corner),
                            }
                        }
                        Run::Round { center, .. } => round(center),
                        Run::Oval { center, .. } => Wall::Elliptic {
                            axis: normal,
                            point: at(center),
                        },
                    });
                }
            }
        }
    }
    walls
}

/// The sine of a lean off parallel past which two walls are not read for
/// a stretch: #552's band, as `relation/perpendicular.rs` reads it.
const BAND: f64 = 1e-3;

/// The share of the reach along which two walls standing within a
/// tolerance of each other are declined, as `relation/perpendicular.rs`
/// reads it.
const STRETCH: f64 = 1e-2;

/// The sine under which two walls leaning past decision 11 are declined
/// wherever their faces meet, touching or crossing (#577's decision 1 of
/// lane 2's rounds).
const SLIVER: f64 = 2e-7;

/// The least `A` round a cone, as a share of the greatest, past which its
/// rulings meet another wall in a true quadratic: `meet/ruled.rs`'s.
const STEADY: f64 = 1e-6;

/// The tolerance a wall's triangles are cut to, as a share of the reach,
/// and how many heights across a box two walls are compared at for a
/// chord's sag, as `relation/nearly_parallel.rs` reads them.
const DRAWN_AT: f64 = 1e-3;
const HEIGHTS: usize = 9;

/// The kernel's tolerance, as a share of the reach, and the furthest
/// decision 11 turns an operand, in tolerances.
const TOLERANCE: f64 = 1e-9;
const HAIR: f64 = 20.0;

/// A cylinder of a leaf, round or elliptic, as the rule for walls along a
/// stretch reads it: its axis of unit length, a point of it, and how far its
/// wall reaches from the axis in a direction square to it.
#[derive(Clone, Copy, Debug)]
struct Barrel {
    axis: DVec3,
    point: DVec3,
    u: DVec3,
    v: DVec3,
    shape: Shape,
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Round(f64),
    Oval(Oval),
}

impl Barrel {
    /// The wall as the kernel holds it, its angles starting where the
    /// kernel's own start.
    fn exact(&self) -> Exact {
        match self.shape {
            Shape::Round(radius) => Exact::Round(Cylinder::about(self.point, self.axis, radius)),
            Shape::Oval(oval) => Exact::Oval(EllipticCylinder::about(
                self.point,
                self.axis,
                self.u * oval.first.x + self.v * oval.first.y,
                oval.second,
            )),
        }
    }

    fn reach(&self, direction: DVec3) -> f64 {
        match self.shape {
            Shape::Round(radius) => radius,
            Shape::Oval(oval) => {
                oval.support(DVec2::new(direction.dot(self.u), direction.dot(self.v)))
            }
        }
    }
}

/// A wall as the kernel holds it.
#[derive(Clone, Copy, Debug)]
enum Exact {
    Round(Cylinder),
    Oval(EllipticCylinder),
    Pointed(Cone),
}

/// The length along a cone's ruling at `height` along its axis.
fn along_the_ruling(cone: &Cone, height: f64) -> f64 {
    (height - cone.foot.x) / cone.ruling.x
}

impl Exact {
    fn origin_and_axis(&self) -> (DVec3, DVec3) {
        match self {
            Exact::Round(cylinder) => (cylinder.origin, cylinder.axis),
            Exact::Oval(oval) => (oval.origin, oval.axis),
            Exact::Pointed(cone) => (cone.origin, cone.axis),
        }
    }

    /// How far the wall reaches from its axis at `height` along it, as the
    /// kernel sizes its grid: a cylinder's radius, an elliptic wall's major
    /// half-axis, a cone's radius there, nought past its apex.
    fn size_at(&self, height: f64) -> f64 {
        match self {
            Exact::Round(cylinder) => cylinder.radius,
            Exact::Oval(oval) => oval.major,
            Exact::Pointed(cone) => cone.radius_at(along_the_ruling(cone, height)).max(0.0),
        }
    }

    fn point(&self, angle: f64, height: f64) -> DVec3 {
        match self {
            Exact::Round(cylinder) => cylinder.point(DVec2::new(angle, height)),
            Exact::Oval(oval) => oval.point(DVec2::new(angle, height)),
            Exact::Pointed(cone) => cone.point(DVec2::new(angle, along_the_ruling(cone, height))),
        }
    }

    /// How far a point stands from the wall: exactly from a cylinder or a
    /// cone, to first order from an elliptic wall, as the kernel measures it.
    fn gap(&self, point: DVec3) -> f64 {
        match self {
            Exact::Round(cylinder) => {
                let from = point - cylinder.origin;
                ((from - cylinder.axis * from.dot(cylinder.axis)).length() - cylinder.radius).abs()
            }
            Exact::Pointed(cone) => cone.distance(point).abs(),
            Exact::Oval(oval) => {
                let from = point - oval.origin;
                let [x, y] = [from.dot(oval.u), from.dot(oval.v)];
                let [a, b] = [oval.major * oval.major, oval.minor * oval.minor];
                let level = x * x / a + y * y / b - 1.0;
                let slope = 2.0 * DVec2::new(x / a, y / b).length();
                if slope > 0.0 {
                    level.abs() / slope
                } else {
                    f64::INFINITY
                }
            }
        }
    }

    /// Whether `point` stands inside the solid the wall bounds, as
    /// `meet/ruled/quadric.rs`'s `inside` reads it: a cone's off its
    /// meridian line, on its own nappe.
    fn contains(&self, point: DVec3) -> bool {
        match self {
            Exact::Round(cylinder) => {
                let from = point - cylinder.origin;
                (from - cylinder.axis * from.dot(cylinder.axis)).length() < cylinder.radius
            }
            Exact::Oval(oval) => {
                let from = point - oval.origin;
                (from.dot(oval.u) / oval.major).powi(2) + (from.dot(oval.v) / oval.minor).powi(2)
                    < 1.0
            }
            Exact::Pointed(cone) => {
                let from = point - cone.origin;
                let (h, rho) = (
                    from.dot(cone.axis),
                    (from - cone.axis * from.dot(cone.axis)).length(),
                );
                let line =
                    cone.ruling.x * cone.foot.y - cone.ruling.y * cone.foot.x + cone.ruling.y * h;
                (cone.ruling.x * rho).abs() < line.abs() && line * cone.ruling.x > 0.0
            }
        }
    }

    /// The square term `A` a line along `way` meets the wall's quadric by,
    /// as `meet/ruled/quadric.rs` writes it: a cone's read off its meridian.
    fn steepness(&self, way: DVec3) -> f64 {
        match self {
            Exact::Round(cylinder) => {
                [cylinder.u, cylinder.v]
                    .map(|e| way.dot(e).powi(2))
                    .iter()
                    .sum::<f64>()
                    / (cylinder.radius * cylinder.radius)
            }
            Exact::Oval(oval) => {
                way.dot(oval.u).powi(2) / (oval.major * oval.major)
                    + way.dot(oval.v).powi(2) / (oval.minor * oval.minor)
            }
            Exact::Pointed(cone) => {
                let across = way - cone.axis * way.dot(cone.axis);
                cone.ruling.x * cone.ruling.x * across.length_squared()
                    - cone.ruling.y * cone.ruling.y * way.dot(cone.axis).powi(2)
            }
        }
    }
}

/// The least `A` round `cone`, its rulings read against `other`, as a
/// share of the greatest: sampled at 720 angles, the least brought down by
/// eighty thirds, since a ruling parallel to the other wall's axis is a
/// double zero of `A` — `meet/ruled.rs`'s `steadiness`. Above `STEADY`,
/// every ruling meets the other wall in a true quadratic.
fn steadiness(cone: &Cone, other: &Exact) -> f64 {
    let a = |theta: f64| {
        let (sin, cos) = theta.sin_cos();
        let ruling = cone.axis * cone.ruling.x + (cone.u * cos + cone.v * sin) * cone.ruling.y;
        other.steepness(ruling)
    };
    let samples = 720;
    let step = TAU / samples as f64;
    let values: Vec<f64> = (0..samples).map(|rank| a(step * rank as f64)).collect();
    let greatest = values.iter().copied().fold(0.0, f64::max);
    let least = (0..samples)
        .min_by(|one, other| values[*one].total_cmp(&values[*other]))
        .map_or(0.0, |rank| step * rank as f64);
    let (mut from, mut to) = (least - step, least + step);
    for _ in 0..80 {
        let [one, two] = [(2.0 * from + to) / 3.0, (from + 2.0 * to) / 3.0];
        if a(one) < a(two) {
            to = two;
        } else {
            from = one;
        }
    }
    if greatest > 0.0 {
        a((from + to) / 2.0) / greatest
    } else {
        0.0
    }
}
/// `A`, `B` and `C` of the ruling of `cone` at `theta` against `other`'s
/// quadric, each with its derivative in `theta`: `meet/ruled/quadric.rs`'s.
fn terms(cone: &Cone, other: &Exact, theta: f64) -> [[f64; 2]; 3] {
    let (sin, cos) = theta.sin_cos();
    let radial = cone.u * cos + cone.v * sin;
    let turned = cone.v * cos - cone.u * sin;
    let foot = cone.origin + cone.axis * cone.foot.x + radial * cone.foot.y;
    let foot1 = turned * cone.foot.y;
    let d = cone.axis * cone.ruling.x + radial * cone.ruling.y;
    let d1 = turned * cone.ruling.y;
    let (origin, axis) = other.origin_and_axis();
    let g = foot - origin;
    let halves = |x: DVec3, y: DVec3, halves: [(DVec3, f64); 2]| {
        halves
            .iter()
            .map(|(e, s)| x.dot(*e) * y.dot(*e) / (s * s))
            .sum::<f64>()
    };
    let quadric = |halved: [(DVec3, f64); 2]| {
        let q = |x: DVec3, y: DVec3| halves(x, y, halved);
        [
            [q(d, d), 2.0 * q(d1, d)],
            [2.0 * q(g, d), 2.0 * (q(foot1, d) + q(g, d1))],
            [q(g, g) - 1.0, 2.0 * q(foot1, g)],
        ]
    };
    match other {
        Exact::Round(cylinder) => {
            quadric([(cylinder.u, cylinder.radius), (cylinder.v, cylinder.radius)])
        }
        Exact::Oval(oval) => quadric([(oval.u, oval.major), (oval.v, oval.minor)]),
        Exact::Pointed(along) => {
            let a = axis;
            let (wh, wr) = (along.ruling.x * along.ruling.x, along.ruling.y);
            let k = along.ruling.x * along.foot.y - wr * along.foot.x;
            let flat = |x: DVec3| x - a * x.dot(a);
            let (gf, g1f, df, d1f) = (flat(g), flat(foot1), flat(d), flat(d1));
            let lift = k + wr * g.dot(a);
            [
                [
                    wh * df.dot(df) - wr * wr * d.dot(a).powi(2),
                    2.0 * (wh * df.dot(d1f) - wr * wr * d.dot(a) * d1.dot(a)),
                ],
                [
                    2.0 * (wh * gf.dot(df) - wr * lift * d.dot(a)),
                    2.0 * (wh * (g1f.dot(df) + gf.dot(d1f))
                        - wr * (wr * foot1.dot(a) * d.dot(a) + lift * d1.dot(a))),
                ],
                [
                    wh * gf.dot(gf) - lift * lift,
                    2.0 * (wh * gf.dot(g1f) - lift * wr * foot1.dot(a)),
                ],
            ]
        }
    }
}

/// `D = B² − 4AC` of the ruling at `theta`, and its derivative.
fn discriminant(cone: &Cone, other: &Exact, theta: f64) -> [f64; 2] {
    let [[a, a1], [b, b1], [c, c1]] = terms(cone, other, theta);
    [b * b - 4.0 * a * c, 2.0 * b * b1 - 4.0 * (a1 * c + a * c1)]
}

/// Where `cone`'s rulings come nearest to `other`, read along them — the
/// double root at each angle `D` turns, kept on the cone's nappe —
/// `meet/ruled.rs`'s `nearest`, its turns found as the kernel finds them,
/// at 720 angles, each bracket halved eighty times.
fn nearest(cone: &Cone, other: &Exact) -> Vec<DVec3> {
    let samples = 720;
    let step = TAU / samples as f64;
    let slopes: Vec<f64> = (0..samples)
        .map(|rank| discriminant(cone, other, step * rank as f64)[1])
        .collect();
    (0..samples)
        .filter(|&rank| slopes[rank].signum() != slopes[(rank + 1) % samples].signum())
        .filter_map(|rank| {
            let (mut from, mut to) = (step * rank as f64, step * (rank + 1) as f64);
            let start = discriminant(cone, other, from)[1].signum();
            for _ in 0..80 {
                let middle = (from + to) / 2.0;
                if middle <= from || middle >= to {
                    break;
                }
                if discriminant(cone, other, middle)[1].signum() == start {
                    from = middle;
                } else {
                    to = middle;
                }
            }
            let theta = ((from + to) / 2.0).rem_euclid(TAU);
            let [[a, _], [b, _], _] = terms(cone, other, theta);
            let l = -b / (2.0 * a);
            (l >= cone.apex_at()).then(|| cone.point(DVec2::new(theta, l)))
        })
        .collect()
}

/// The root `l` on the side `sign` of the ruling of `cone` at `theta`
/// against `other`, free of cancellation: `meet/ruled.rs`'s `solved`.
fn root(cone: &Cone, other: &Exact, theta: f64, sign: f64) -> f64 {
    let [[a, _], [b, _], [c, _]] = terms(cone, other, theta);
    let root = sign * (b * b - 4.0 * a * c).max(0.0).sqrt();
    if -b * root >= 0.0 && a != 0.0 {
        (-b + root) / (2.0 * a)
    } else if -b - root != 0.0 {
        2.0 * c / (-b - root)
    } else {
        -b / (2.0 * a)
    }
}

/// The laps of the curve `cone`'s rulings meet `other` along, each a span
/// of `θ` its two roots run over or a whole turn on the root of a sign, as
/// `meet/ruled/configured.rs` lays them but for nodes: the apex inside
/// `other`, one turn; `D` positive all round, a turn on each root on the
/// nappe and on `other`'s own; else a span between two zeros of `D`, found
/// at 720 angles and halved eighty times, where it is positive.
fn laps(cone: &Cone, other: &Exact) -> Vec<(Option<[f64; 2]>, f64)> {
    if other.contains(cone.apex()) {
        return vec![(None, 1.0)];
    }
    let held = |point: DVec3| match other {
        Exact::Pointed(along) => along.radius_at(along.parameters(point).y) >= 0.0,
        Exact::Round(_) | Exact::Oval(_) => true,
    };
    let above = |theta: f64, sign: f64| {
        let l = root(cone, other, theta, sign);
        l >= cone.apex_at() && held(cone.point(DVec2::new(theta, l)))
    };
    let d = |theta: f64| discriminant(cone, other, theta)[0];
    let samples = 720;
    let at = |rank: usize| TAU * rank as f64 / samples as f64;
    let mut zeros: Vec<f64> = (0..samples)
        .filter(|&rank| d(at(rank)).signum() != d(at(rank + 1)).signum())
        .map(|rank| {
            let (mut from, mut to) = (at(rank), at(rank + 1));
            let start = d(from).signum();
            for _ in 0..80 {
                let middle = (from + to) / 2.0;
                if d(middle).signum() == start {
                    from = middle;
                } else {
                    to = middle;
                }
            }
            ((from + to) / 2.0).rem_euclid(TAU)
        })
        .collect();
    zeros.sort_by(f64::total_cmp);
    if zeros.is_empty() {
        if d(0.0) <= 0.0 {
            return Vec::new();
        }
        return [1.0, -1.0]
            .into_iter()
            .filter(|&sign| above(0.1, sign))
            .map(|sign| (None, sign))
            .collect();
    }
    let count = zeros.len();
    (0..count)
        .filter_map(|index| {
            let from = zeros[index];
            let to = zeros[(index + 1) % count] + if index + 1 == count { TAU } else { 0.0 };
            let middle = (from + to) / 2.0;
            (d(middle) > 0.0 && above(middle, 1.0)).then_some((Some([from, to]), 1.0))
        })
        .collect()
}

/// Whether every component of the curve `cone` meets `wall` along goes
/// round `cone`'s axis, as `meet.rs`'s `winds_round` reads it: read along
/// `cone`'s rulings, each lap a whole turn; read along the other cone's,
/// the steadier as the kernel picks it, `cone`'s angle winding round along
/// each lap, sampled at 2048 parameters.
fn goes_round(cone: &Cone, wall: &Exact) -> bool {
    let reader = match wall {
        Exact::Pointed(other)
            if steadiness(other, &Exact::Pointed(*cone)) > steadiness(cone, wall) =>
        {
            *other
        }
        _ => return laps(cone, wall).iter().all(|(span, _)| span.is_none()),
    };
    let read = Exact::Pointed(*cone);
    laps(&reader, &read).into_iter().all(|(span, sign)| {
        let samples = 2048;
        let angle = |rank: usize| {
            let t = TAU * rank as f64 / samples as f64;
            let (theta, sign) = match span {
                Some([from, to]) => (
                    (from + to) / 2.0 - (to - from) / 2.0 * t.cos(),
                    if t.sin() >= 0.0 { sign } else { -sign },
                ),
                None => (t, sign),
            };
            let point = reader.point(DVec2::new(theta, root(&reader, &read, theta, sign)));
            let from = point - cone.origin;
            from.dot(cone.v).atan2(from.dot(cone.u))
        };
        let turned: f64 = (0..samples)
            .map(|rank| {
                let step = angle(rank + 1) - angle(rank);
                step - TAU * (step / TAU).round()
            })
            .sum();
        (turned / TAU).round() != 0.0
    })
}

/// Whether a cone and a wall come closer to each other than the coarsest
/// drawn chord's sag without meeting within the tolerance, near the box
/// `[low, high]`: at a nearest approach of a cone's rulings to the other
/// wall, read along whichever of the two the kernel reads steadily, the gap
/// to the wall the point is not on over `eps` and under the larger of the
/// two walls' sags there —
/// `relation/nearly_parallel.rs`'s `grazing`.
fn grazing(cone: &Cone, wall: &Exact, low: DVec3, high: DVec3, reach: f64) -> bool {
    let eps = TOLERANCE * reach;
    let tolerance = DRAWN_AT * reach;
    let sag = |size: f64| size * (1.0 - (PI / divisions(size, tolerance) as f64).cos());
    let size = |point: DVec3| match wall {
        Exact::Round(cylinder) => cylinder.radius,
        Exact::Oval(oval) => oval.major,
        Exact::Pointed(other) => other.off_axis(point),
    };
    let mut approaches = Vec::new();
    if steadiness(cone, wall) > STEADY {
        approaches.extend(nearest(cone, wall).into_iter().map(|point| (point, *wall)));
    }
    if let Exact::Pointed(other) = wall
        && steadiness(other, &Exact::Pointed(*cone)) > STEADY
    {
        let on_the_other = nearest(other, &Exact::Pointed(*cone));
        approaches.extend(
            on_the_other
                .into_iter()
                .map(|point| (point, Exact::Pointed(*cone))),
        );
    }
    approaches.into_iter().any(|(point, facing)| {
        let room = sag(cone.off_axis(point)).max(sag(size(point)));
        let apart = facing.gap(point);
        apart > eps
            && apart <= room
            && point.cmpge(low - DVec3::splat(room)).all()
            && point.cmple(high + DVec3::splat(room)).all()
    })
}

/// How many steps a whole turn of a wall of `size` is cut into for its
/// triangles at `tolerance`: the kernel's `tessellation/sampling.rs`.
fn divisions(size: f64, tolerance: f64) -> usize {
    let ratio = tolerance / size;
    if ratio.is_nan() || ratio <= 0.0 {
        return 1024;
    }
    let needed = (PI / (1.0 - ratio.min(2.0)).acos()).ceil();
    ((needed / 4.0).ceil() * 4.0).clamp(16.0, 1024.0) as usize
}

/// Whether two walls stand closer to each other than a chord's sag over a
/// step of either's grid, in the box `[low, high]`: at `HEIGHTS` heights
/// across it, two consecutive steps of one wall's grid both within that
/// sag of the other wall, a cone's grid sized by its widest across the box
/// and its sag by its radius at each height, as `relation/nearly_parallel.rs`
/// reads it.
fn closer_than_a_sag(first: &Exact, second: &Exact, low: DVec3, high: DVec3, reach: f64) -> bool {
    let tolerance = DRAWN_AT * reach;
    [(first, second), (second, first)]
        .into_iter()
        .any(|(wall, other)| {
            let (origin, axis) = wall.origin_and_axis();
            let (from, to) = (0..8)
                .map(|corner: usize| {
                    let pick =
                        |bit: usize, at: usize| if corner & bit == 0 { low[at] } else { high[at] };
                    (DVec3::new(pick(1, 0), pick(2, 1), pick(4, 2)) - origin).dot(axis)
                })
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(from, to), height| {
                    (from.min(height), to.max(height))
                });
            let steps = divisions(wall.size_at(from).max(wall.size_at(to)), tolerance);
            (0..HEIGHTS).any(|rank| {
                let height = from + (to - from) * rank as f64 / (HEIGHTS - 1) as f64;
                let sag = wall.size_at(height) * (1.0 - (PI / steps as f64).cos());
                let near = |step: usize| {
                    let angle = TAU * (step % steps) as f64 / steps as f64;
                    other.gap(wall.point(angle, height)) <= sag
                };
                (0..steps).any(|step| near(step) && near(step + 1))
            })
        })
}

/// The cones of a leaf as the kernel holds them: each slanted edge of a
/// turned leaf's section that does not bow, from where its band starts to
/// where it ends, on the side of the axis it stands; each with whether its
/// face holds the apex, the edge reaching the axis at one end.
fn cones(leaf: &Leaf) -> Vec<(Cone, bool)> {
    let Some(turned) = leaf.as_turned() else {
        return Vec::new();
    };
    let section = &turned.section;
    let swept = turned.swept(1.0);
    let ends = section.ends();
    (0..section.bands.len())
        .flat_map(|index| {
            let (start, end) = section.edges(index);
            let (from, to) = (ends[index], ends[index + 1]);
            [0, 1]
                .into_iter()
                .filter(move |&edge| {
                    start[edge] != end[edge]
                        && section.bow(index, edge).is_none()
                        && start[edge] * end[edge] >= 0.0
                })
                .map(move |edge| {
                    let cone = Cone::through(
                        swept.origin,
                        swept.along,
                        [
                            DVec2::new(from, start[edge].abs()),
                            DVec2::new(to, end[edge].abs()),
                        ],
                    );
                    (cone, start[edge] == 0.0 || end[edge] == 0.0)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
/// Whether the kernel declines a cone against `wall`, off its axis (#577):
/// a cylinder or a cone about the cone's very axis is built as before; one
/// within a hair of it, the hair `HAIR` tolerances, is declined; so is an
/// elliptic wall about it once decision 11 has laid the two exactly along —
/// a sine up to a hair over the `turning`'s lever — the axes within a hair
/// of each other where it turns the later leaf (#857), assuming the turn
/// succeeds; so is a pair no ruling meets in a true quadratic all round, `A`
/// reaching nought — a wall's axis along a ruling, or two cones each with
/// rulings within the other's slope; an apex of either standing on the
/// other wall, `on_the_apex`; a curve on a cone face
/// holding its apex, `pointed`, a component of which does not go round the
/// cone's axis (`goes_round`), as `combine/related.rs`'s
/// `on_a_pointed_face` reads it; two walls in #552's band closer than a
/// chord's sag in the box `[low, high]`; and a cone and a wall grazing each
/// other closer than a sag without meeting. As `relation/conical.rs` and
/// `relation/nearly_parallel.rs` read it.
fn declines(
    cone: &Cone,
    pointed: bool,
    wall: &Exact,
    low: DVec3,
    high: DVec3,
    reach: f64,
    turning: &Turning,
) -> bool {
    let (origin, axis) = wall.origin_and_axis();
    let sine = cone.axis.cross(axis).length();
    let between = origin - cone.origin;
    let off = (between - cone.axis * cone.axis.dot(between)).length();
    let coaxial = |within: f64| sine * 2.0 <= TOLERANCE && off <= TOLERANCE * within;
    if matches!(wall, Exact::Round(_) | Exact::Pointed(_)) && coaxial(reach) {
        return false;
    }
    if coaxial(reach * HAIR) {
        return true;
    }
    let eps = TOLERANCE * reach;
    if matches!(wall, Exact::Oval(_)) && sine * turning.lever() <= HAIR * eps {
        let about = turning.about(sine, eps);
        let apart = cone.origin + cone.axis * cone.axis.dot(about - cone.origin) - origin;
        if (apart - axis * axis.dot(apart)).length() <= HAIR * eps {
            return true;
        }
    }
    let steady = steadiness(cone, wall) > STEADY
        || matches!(wall, Exact::Pointed(other) if steadiness(other, &Exact::Pointed(*cone)) > STEADY);
    let on_the_apex = wall.gap(cone.apex()) <= eps
        || matches!(wall, Exact::Pointed(other) if cone.distance(other.apex()).abs() <= eps);
    !steady
        || on_the_apex
        || (pointed && !goes_round(cone, wall))
        || (sine < BAND && closer_than_a_sag(&Exact::Pointed(*cone), wall, low, high, reach))
        || grazing(cone, wall, low, high, reach)
}
/// Where decision 11 turns the later of two leaves: `met`, the middle of
/// where their boxes overlap, and the later leaf's own box (`squared.rs`).
struct Turning {
    met: DVec3,
    low: DVec3,
    high: DVec3,
}

impl Turning {
    /// Half the diagonal of the turned leaf's box: the furthest a turn about
    /// its middle moves a point of it, per radian.
    fn lever(&self) -> f64 {
        (self.high - self.low).length() / 2.0
    }

    /// The point a turn by `sine` is made about: `met`, or the middle of the
    /// turned leaf's box where turning about `met` would sweep a point of it
    /// further than a hair.
    fn about(&self, sine: f64, eps: f64) -> DVec3 {
        let sweep = |about: DVec3| {
            sine.asin()
                * (about - self.low)
                    .abs()
                    .max((self.high - about).abs())
                    .length()
        };
        if sweep(self.met) <= HAIR * eps {
            self.met
        } else {
            (self.low + self.high) / 2.0
        }
    }
}

/// The barrels of a leaf: every round or elliptic wall of a prism — a
/// circle, a ring's two, a rounded rectangle's four corners, a slot's two
/// ends, the arcs an ellipse bounds — and of a turned leaf each edge off the
/// axis of a band that neither slopes nor bows, and the two edges of each
/// hole of its section.
fn barrels(leaf: &Leaf) -> Vec<Barrel> {
    if let Some(turned) = leaf.as_turned() {
        let section = &turned.section;
        let swept = turned.swept(1.0);
        let axis = swept.along.normalize();
        let u = swept.out;
        let v = axis.cross(u);
        let level = (0..section.bands.len()).flat_map(|index| {
            let (start, end) = section.edges(index);
            [0, 1]
                .into_iter()
                .filter(move |&edge| start[edge] == end[edge] && section.bow(index, edge).is_none())
                .map(move |edge| start[edge])
        });
        let holes = section.holes.iter().flat_map(|[low, high]| [low.y, high.y]);
        return level
            .chain(holes)
            .map(f64::abs)
            .filter(|radius| *radius > 0.0)
            .map(|radius| Barrel {
                axis,
                point: swept.origin,
                u,
                v,
                shape: Shape::Round(radius),
            })
            .collect();
    }
    let Leaf::Prism { plane, outline, .. } = leaf else {
        return Vec::new();
    };
    let (_, u, v) = plane.frame();
    let axis = u.cross(v).normalize();
    let barrel = |center: DVec2, shape: Shape| Barrel {
        axis,
        point: plane.to_world(center),
        u,
        v,
        shape,
    };
    let round = |center: DVec2, radius: f64| barrel(center, Shape::Round(radius));
    match outline {
        Outline::Circle { center, radius, .. } => vec![round(*center, *radius)],
        Outline::Ring {
            center,
            outer,
            inner,
        } => vec![round(*center, *outer), round(*center, *inner)],
        Outline::Rounded { low, high, radius } => [
            *low + *radius,
            *high - *radius,
            DVec2::new(low.x + radius, high.y - radius),
            DVec2::new(high.x - radius, low.y + radius),
        ]
        .map(|center| round(center, *radius))
        .to_vec(),
        Outline::Slot { from, to, radius } => vec![round(*from, *radius), round(*to, *radius)],
        Outline::Rectangle { .. } | Outline::Star { .. } => Vec::new(),
        Outline::Ellipse { .. }
        | Outline::Chorded { .. }
        | Outline::OvalSlot { .. }
        | Outline::OvalRing { .. } => {
            let Some((contour, holes)) = outline.oval_contours() else {
                return Vec::new();
            };
            std::iter::once(&contour)
                .chain(&holes)
                .flat_map(|contour| {
                    contour
                        .runs
                        .iter()
                        .zip(&contour.corners)
                        .filter_map(|(run, corner)| match *run {
                            Run::Straight => None,
                            Run::Round { center, .. } => {
                                Some(round(center, corner.distance(center)))
                            }
                            Run::Oval {
                                center,
                                first,
                                second,
                                ..
                            } => Some(barrel(
                                center,
                                Shape::Oval(Oval::new(DVec2::ZERO, first, second)),
                            )),
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        }
    }
}

/// Two barrels read as `relation/perpendicular.rs` reads two walls: square
/// to the first's axis, the second's stands at an offset affine in the
/// height along it, and the heights the box `[low, high]` spans.
struct Leaning<'a> {
    first: &'a Barrel,
    second: &'a Barrel,
    rate: DVec3,
    at_nought: DVec3,
    from: f64,
    to: f64,
}

impl<'a> Leaning<'a> {
    fn of(first: &'a Barrel, second: &'a Barrel, low: DVec3, high: DVec3) -> Leaning<'a> {
        let (axis, along) = (first.axis, second.axis);
        let between = second.point - first.point;
        let base = first.point.dot(axis);
        let (from, to) = (0..8)
            .map(|corner: usize| {
                let pick =
                    |bit: usize, at: usize| if corner & bit == 0 { low[at] } else { high[at] };
                DVec3::new(pick(1, 0), pick(2, 1), pick(4, 2)).dot(axis) - base
            })
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(from, to), height| {
                (from.min(height), to.max(height))
            });
        Leaning {
            first,
            second,
            rate: along / along.dot(axis) - axis,
            at_nought: between - along * (between.dot(axis) / along.dot(axis)),
            from,
            to,
        }
    }

    fn offset(&self, height: f64) -> DVec3 {
        self.at_nought + self.rate * height
    }

    /// The height where the offset is least, and the two walls' reaches
    /// along it there, clamped into the box.
    fn nearest(&self) -> (f64, f64, f64) {
        let squared = self.rate.length_squared();
        let nearest = if squared > 0.0 {
            -self.at_nought.dot(self.rate) / squared
        } else {
            (self.from + self.to) / 2.0
        };
        let way = self
            .offset(nearest.clamp(self.from, self.to))
            .normalize_or(self.first.axis.any_orthonormal_vector());
        (nearest, self.first.reach(way), self.second.reach(way))
    }

    /// The longest stretch of heights in the box along which the walls are
    /// one, or touch inside or outside, within `tolerance`: the kernel's
    /// quadratic in the height.
    fn stretch(&self, tolerance: f64) -> f64 {
        let squared = self.rate.length_squared();
        let (nearest, near, far) = self.nearest();
        let closest = self.offset(nearest).length_squared();
        let within_of = |bound: f64| {
            if bound * bound < closest {
                return None;
            }
            let half = if squared > 0.0 {
                ((bound * bound - closest) / squared).sqrt()
            } else {
                f64::INFINITY
            };
            Some([
                (nearest - half).max(self.from),
                (nearest + half).min(self.to),
            ])
        };
        let stretch = |target: f64| {
            let Some([start, end]) = within_of(target + tolerance) else {
                return 0.0;
            };
            match (target > tolerance)
                .then(|| within_of(target - tolerance))
                .flatten()
            {
                Some([inner, outer]) => (inner.min(end) - start).max(end - outer.max(start)),
                None => end - start,
            }
            .max(0.0)
        };
        stretch((near - far).abs()).max(stretch(near + far))
    }

    /// Whether the walls meet, touching or crossing, as the kernel reads it:
    /// the least offset, along the whole axis, within `tolerance` of their
    /// reaches' sum or under it, the greatest in the box over their
    /// difference.
    fn meet(&self, tolerance: f64) -> bool {
        let (nearest, near, far) = self.nearest();
        let least = self.offset(nearest).length();
        let most = self
            .offset(self.from)
            .length()
            .max(self.offset(self.to).length());
        least <= near + far + tolerance && most >= (near - far).abs() - tolerance
    }
}

/// Whether two directions are neither parallel nor square to each other,
/// by more than the rounding of a direction of unit length.
fn oblique(one: DVec3, other: DVec3) -> bool {
    let (one, other) = (one.normalize(), other.normalize());
    one.dot(other).abs() > ALIGNED && one.cross(other).length() > ALIGNED
}

/// How many pairs of a case's walls meet along each kind of conic, as the
/// scope rule reads them (#552): a plane neither square to a cylinder's axis
/// nor along it cuts an ellipse; a plane meeting a cone, near the box its
/// leaf spans, neither square to its axis nor holding it nor parallel to it
/// past its widest rim, cuts an ellipse, a parabola or a hyperbola, or
/// through its tip two lines, a ruling it touches along, or the tip alone,
/// as its angle with the axis is steeper than the cone's slope, as steep or
/// flatter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Conics {
    pub ellipses: usize,
    pub parabolas: usize,
    pub hyperbolas: usize,
    pub lines_through_an_apex: usize,
    pub apex_alone: usize,
    pub tangent_rulings: usize,
}

impl Conics {
    pub fn total(&self) -> usize {
        self.ellipses
            + self.parabolas
            + self.hyperbolas
            + self.lines_through_an_apex
            + self.apex_alone
            + self.tangent_rulings
    }
}

/// A cone of a turned leaf as the conics are told by: its tip, its axis of
/// unit length, and its slope off the axis in radians.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Slope {
    tip: DVec3,
    axis: DVec3,
    angle: f64,
}

impl Slope {
    /// The conic a plane through `on` square to `normal` cuts this cone
    /// along, a plane that neither stands square to its axis nor holds it.
    fn cut(&self, normal: DVec3, on: DVec3, reach: f64, conics: &mut Conics) {
        let normal = normal.normalize();
        let along = normal.dot(self.axis).abs();
        let with_the_axis = along.atan2(normal.cross(self.axis).length());
        let through_the_tip = normal.dot(self.tip - on).abs() <= ALIGNED * reach;
        let kind = if along <= ALIGNED {
            &mut conics.hyperbolas
        } else if (with_the_axis - self.angle).abs() <= ALIGNED {
            if through_the_tip {
                &mut conics.tangent_rulings
            } else {
                &mut conics.parabolas
            }
        } else if with_the_axis < self.angle {
            if through_the_tip {
                &mut conics.lines_through_an_apex
            } else {
                &mut conics.hyperbolas
            }
        } else if through_the_tip {
            &mut conics.apex_alone
        } else {
            &mut conics.ellipses
        };
        *kind += 1;
    }
}

/// The cones of a turned leaf: one for every slanted edge of its section,
/// read as drawn, either side of the axis.
fn slopes(leaf: &Leaf) -> Vec<Slope> {
    let Some(turned) = leaf.as_turned() else {
        return Vec::new();
    };
    let swept = turned.swept(1.0);
    let ends = turned.section.ends();
    let mut slopes: Vec<Slope> = Vec::new();
    for index in 0..turned.section.bands.len() {
        let (start, end) = turned.section.edges(index);
        let run = ends[index + 1] - ends[index];
        for edge in 0..2 {
            let (from, to) = (start[edge], end[edge]);
            if from == to || run <= 0.0 {
                continue;
            }
            let tip = ends[index] - from * run / (to - from);
            let slope = Slope {
                tip: swept.origin + swept.along * tip,
                axis: swept.along,
                angle: (to - from).abs().atan2(run),
            };
            if !slopes.contains(&slope) {
                slopes.push(slope);
            }
        }
    }
    slopes
}

/// A case's walls read leaf by leaf, with the box each leaf spans and its
/// cones, and how far the case reaches.
struct Walled {
    leaves: Vec<(Vec<Wall>, Spanned, Vec<Slope>)>,
    reach: f64,
}

/// The axes of the cylinders among a leaf's walls, circular or elliptic.
fn cylinders_among(walls: &[Wall]) -> impl Iterator<Item = DVec3> + '_ {
    walls.iter().filter_map(|wall| match wall {
        Wall::Cylinder { axis, .. } | Wall::Elliptic { axis, .. } => Some(*axis),
        _ => None,
    })
}

impl Walled {
    fn of(case: &Case) -> Walled {
        let leaves: Vec<(Vec<Wall>, Spanned, Vec<Slope>)> = case
            .leaves()
            .map(|leaf| {
                (
                    walls(leaf),
                    leaf.is_solid().then(|| leaf.bounds()).flatten(),
                    slopes(leaf),
                )
            })
            .collect();
        let reach = leaves
            .iter()
            .filter_map(|(_, bounds, _)| *bounds)
            .map(|(low, high)| low.abs().max(high.abs()).max_element())
            .fold(1.0, f64::max);
        Walled { leaves, reach }
    }

    fn all(&self) -> impl Iterator<Item = &Wall> {
        self.leaves.iter().flat_map(|(walls, _, _)| walls)
    }

    fn cylinders(&self) -> Vec<DVec3> {
        self.leaves
            .iter()
            .flat_map(|(walls, _, _)| cylinders_among(walls))
            .collect()
    }

    /// Whether two leaves' boxes meet, a millionth of the reach apart.
    fn near(&self, one: Spanned, other: Spanned) -> bool {
        let margin = 1e-6 * self.reach;
        match (one, other) {
            (Some((low, high)), Some((other_low, other_high))) => {
                (low - margin).cmple(other_high).all() && (other_low - margin).cmple(high).all()
            }
            _ => false,
        }
    }

    /// Every wall of a leaf whose box meets a cone's, with that cone's wall,
    /// its leaf's cones, and the wall met.
    fn against_cones(&self) -> Vec<(&Wall, &[Slope], &Wall)> {
        let mut met = Vec::new();
        for (walls, bounds, slopes) in &self.leaves {
            for cone in walls
                .iter()
                .filter(|wall| matches!(wall, Wall::Cone { .. }))
            {
                for (others, other_bounds, _) in &self.leaves {
                    if self.near(*bounds, *other_bounds) {
                        met.extend(others.iter().map(|other| (cone, slopes.as_slice(), other)));
                    }
                }
            }
        }
        met
    }
}

impl Case {
    /// Whether the case may ask the exact kernel for a curve that is no
    /// conic it builds (#552): a cone meeting a cylinder or a cone not about
    /// its very axis, or an elliptic wall about any (#571), where their
    /// leaves' boxes meet; or two walls nearly parallel along a stretch
    /// (#577, `asks_of_walls_along_a_stretch`). A plane never asks, nor do
    /// two walls at a skew angle since #577. The kernel may decline such a
    /// case as unsupported; it may also hold it, the leaves never meeting
    /// there.
    pub fn asks_beyond_the_conics(&self) -> bool {
        let walled = Walled::of(self);
        let coned = walled.against_cones().into_iter().any(|(cone, _, other)| {
            let Wall::Cone {
                axis,
                point,
                radius,
            } = *cone
            else {
                return false;
            };
            matches!(other, Wall::Sphere { .. } | Wall::Torus { .. })
                && other.asks_of_a_cone(axis, point, radius, walled.reach)
        });
        coned || self.asks_of_a_cone_off_its_axis() || self.asks_of_walls_along_a_stretch()
    }

    /// Whether a cone of one leaf and a cylinder, an elliptic wall or a cone
    /// of another, their boxes meeting, stand where the kernel still declines
    /// them (#577, `declines`), read in the box the two leaves share and at
    /// the reach of the leaves up to the later one.
    pub fn asks_of_a_cone_off_its_axis(&self) -> bool {
        let walled = Walled::of(self);
        let leaves: Vec<(&Leaf, Spanned)> = self
            .leaves()
            .map(|leaf| (leaf, leaf.is_solid().then(|| leaf.bounds()).flatten()))
            .collect();
        let reach_up_to = |last: usize| {
            leaves[..=last]
                .iter()
                .filter_map(|(_, bounds)| *bounds)
                .map(|(low, high)| low.abs().max(high.abs()).max_element())
                .fold(1.0, f64::max)
        };
        let walls = |leaf: &Leaf| -> Vec<Exact> {
            barrels(leaf)
                .iter()
                .map(Barrel::exact)
                .chain(
                    cones(leaf)
                        .into_iter()
                        .map(|(cone, _)| Exact::Pointed(cone)),
                )
                .collect()
        };
        leaves.iter().enumerate().any(|(index, (one, bounds))| {
            leaves
                .iter()
                .enumerate()
                .any(|(other_index, (other, other_bounds))| {
                    if other_index == index || !walled.near(*bounds, *other_bounds) {
                        return false;
                    }
                    let (Some((low, high)), Some((other_low, other_high))) = (bounds, other_bounds)
                    else {
                        return false;
                    };
                    let margin = DVec3::splat(1e-6 * walled.reach);
                    let (shared_low, shared_high) =
                        (low.max(*other_low) - margin, high.min(*other_high) + margin);
                    let reach = reach_up_to(index.max(other_index));
                    let (turned_low, turned_high) = if index > other_index {
                        (*low, *high)
                    } else {
                        (*other_low, *other_high)
                    };
                    let turning = Turning {
                        met: (low.max(*other_low) + high.min(*other_high)) / 2.0,
                        low: turned_low,
                        high: turned_high,
                    };
                    cones(one).iter().any(|(cone, pointed)| {
                        walls(other).iter().any(|wall| {
                            declines(
                                cone,
                                *pointed,
                                wall,
                                shared_low,
                                shared_high,
                                reach,
                                &turning,
                            )
                        })
                    })
                })
        })
    }

    /// Whether two walls of two leaves whose boxes meet, round or elliptic,
    /// lean off parallel past what decision 11 lays exactly — a turn moving
    /// the second leaf `HAIR` tolerances — and under #552's band, where #577
    /// declines them as unsupported: within the kernel's tolerance of one
    /// another along a hundredth of the reach or more, one size with their
    /// axes crossing or touching as the second's axis drifts across the line
    /// of touch (`relation/perpendicular.rs`); leaning by a sine under
    /// `SLIVER`, wherever they meet, touching or crossing. Read in the box the
    /// two leaves share, where the kernel reads the faces' own; or closer to
    /// one another than a chord's sag of either's grid over two steps of it,
    /// where their triangles would cross (`closer_than_a_sag`).
    pub fn asks_of_walls_along_a_stretch(&self) -> bool {
        let walled = Walled::of(self);
        let leaves: Vec<(&Leaf, Spanned)> = self
            .leaves()
            .map(|leaf| (leaf, leaf.is_solid().then(|| leaf.bounds()).flatten()))
            .collect();
        let reach_up_to = |last: usize| {
            leaves[..=last]
                .iter()
                .filter_map(|(_, bounds)| *bounds)
                .map(|(low, high)| low.abs().max(high.abs()).max_element())
                .fold(1.0, f64::max)
        };
        leaves.iter().enumerate().any(|(index, (one, bounds))| {
            leaves[index + 1..]
                .iter()
                .enumerate()
                .any(|(after, (other, other_bounds))| {
                    let reach = reach_up_to(index + 1 + after);
                    let eps = TOLERANCE * reach;
                    let (Some((low, high)), Some((other_low, other_high))) = (bounds, other_bounds)
                    else {
                        return false;
                    };
                    if !walled.near(*bounds, *other_bounds) {
                        return false;
                    }
                    let margin = DVec3::splat(1e-6 * walled.reach);
                    let (shared_low, shared_high) =
                        (low.max(*other_low) - margin, high.min(*other_high) + margin);
                    let lever = (*other_high - *other_low).length() / 2.0;
                    barrels(one).iter().any(|first| {
                        barrels(other).iter().any(|second| {
                            let sine = first.axis.cross(second.axis).length();
                            let parallel = sine * 2.0 <= TOLERANCE;
                            if parallel || sine * lever <= HAIR * eps || sine >= BAND {
                                return false;
                            }
                            let leaning = Leaning::of(first, second, shared_low, shared_high);
                            leaning.stretch(eps) >= STRETCH * reach
                                || (sine < SLIVER && leaning.meet(eps))
                                || closer_than_a_sag(
                                    &first.exact(),
                                    &second.exact(),
                                    shared_low,
                                    shared_high,
                                    reach,
                                )
                        })
                    })
                })
        })
    }

    /// Whether the case may ask the exact kernel for a curve beyond the
    /// circles it builds a sphere or a torus along (#576): a sphere or a
    /// torus of one leaf against a wall of a leaf whose box meets the box of
    /// the part of the round the turn drew — against another sphere or
    /// torus, that one's part's box — that is neither a plane square to the
    /// turn's axis or holding it, nor a cylinder, a cone, a sphere or a
    /// torus about that very axis. The kernel may decline such a case as
    /// unsupported; it may also hold it, the walls never meeting there. The
    /// round's leaf's box is not the one the kernel reads: it boxes a face on
    /// a sphere or a torus by its own extent (#625), the sweep of the arc it
    /// was turned from, which may stand well inside the leaf — a shaft's
    /// rounded end reaches no further along the shaft than its cap.
    pub fn asks_a_round_beyond_its_circles(&self) -> bool {
        let walled = Walled::of(self);
        walled.leaves.iter().any(|(walls, _, _)| {
            walls
                .iter()
                .filter_map(|wall| match wall {
                    Wall::Sphere { axis, point, .. } | Wall::Torus { axis, point, .. } => {
                        Some((*axis, *point, wall.round_box()))
                    }
                    _ => None,
                })
                .any(|(axis, point, spanned)| {
                    walled.leaves.iter().any(|(others, bounds, _)| {
                        others.iter().any(|other| {
                            let boxed = match other {
                                Wall::Sphere { .. } | Wall::Torus { .. } => other.round_box(),
                                _ => *bounds,
                            };
                            walled.near(spanned, boxed)
                                && other.asks_of_a_round(axis, point, walled.reach)
                        })
                    })
                })
        })
    }

    /// The conics the case's planes meet its cylinders and cones along, as
    /// `Conics` tells them.
    pub fn conics(&self) -> Conics {
        let walled = Walled::of(self);
        let mut conics = Conics::default();
        let cylinders = walled.cylinders();
        for wall in walled.all() {
            if let Wall::Plane { normal, .. } = wall {
                conics.ellipses += cylinders
                    .iter()
                    .filter(|axis| oblique(*normal, **axis))
                    .count();
            }
        }
        for (cone, slopes, other) in walled.against_cones() {
            let (
                Wall::Cone {
                    axis,
                    point,
                    radius,
                },
                Wall::Plane { normal, point: on },
            ) = (cone, other)
            else {
                continue;
            };
            if other.asks_of_a_cone(*axis, *point, *radius, walled.reach) {
                for slope in slopes {
                    slope.cut(*normal, *on, walled.reach, &mut conics);
                }
            }
        }
        conics
    }

    /// Whether some plane of the case meets a cylinder or a cone of it along
    /// a conic.
    pub fn meets_along_a_conic(&self) -> bool {
        self.conics().total() > 0
    }
}
