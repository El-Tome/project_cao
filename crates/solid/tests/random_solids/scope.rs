//! The scope rule: which cases may ask the exact kernel for a curve it does
//! not build, read off the walls their leaves are bounded by, so that a
//! decline as unsupported is counted rather than failed.
//!
//! #552 split it in two: what asks beyond the conics — a cone against
//! anything off its axis, where their leaves' boxes meet — and the conics a
//! case's planes meet its cylinders and cones along, told kind by kind. The
//! kernel builds the conics, so only the first excuses a decline.
//!
//! An elliptic wall (#571) is a cylinder to the rule: it asks against a cone
//! about any axis, its own included, where their leaves' boxes meet. Two
//! walls at a skew angle, round or elliptic, ask nothing since #577 builds
//! the curve they meet along — but for the cases it declines, two walls
//! nearly parallel past what decision 11 lays: within the kernel's
//! tolerance of each other along a stretch rather than at points, as
//! `relation/perpendicular.rs` measures it, or leaning a sliver and meeting
//! at all (`Case::asks_of_walls_along_a_stretch`).
//!
//! A sphere or a torus (#576) is built along circles alone: against a plane
//! square to its axis or holding it, and against a cylinder, a cone, a
//! sphere or a torus about that very axis. Any other wall whose leaf's box
//! meets its own asks beyond those circles — a plane at a slant, a block's
//! side, a hole drilled off the axis, a cone off it — and the kernel may
//! decline the case as unsupported, which is counted apart from the rule
//! for the conics (`asks_a_round_beyond_its_circles`).

use std::f64::consts::{PI, TAU};

use cao_solid::brep::{Cylinder, EllipticCylinder};
use cao_solid::profile::Run;
use glam::{DVec2, DVec3};

use super::{Case, Leaf, Outline, Oval};

/// The box a leaf spans, or nothing for a leaf that is no solid.
type Spanned = Option<(DVec3, DVec3)>;

/// A surface a leaf is bounded by, as the scope rule reads it: a plane by its
/// normal and a point of it, a cylinder, an elliptic cylinder or a cone by
/// its axis and a point of the axis, a cone besides by the furthest its leaf
/// reaches from the axis, and a sphere or a torus by the axis it was turned
/// about and the point of it its circle's centre stands level with, a sphere
/// besides by its radius and a torus by the circle's two radii.
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
        radius: f64,
    },
    Torus {
        axis: DVec3,
        point: DVec3,
        big: f64,
        radius: f64,
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
    /// anything but a plane square to the axis, holding it or parallel to it
    /// further off than the leaf reaches by more than the kernel tells
    /// clear, or a cylinder or a cone about the same line. An elliptic wall
    /// always does, about the cone's axis or off it (#571).
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

    /// The box the whole surface spans, as the kernel boxes a face on a
    /// sphere or a torus by its surface's own extent, which reaches far
    /// beyond the turn that drew a part of it (#576); none for any other
    /// wall.
    fn round_box(&self) -> Option<[DVec3; 2]> {
        match *self {
            Wall::Sphere { point, radius, .. } => {
                Some([point - DVec3::splat(radius), point + DVec3::splat(radius)])
            }
            Wall::Torus {
                axis,
                point,
                big,
                radius,
            } => {
                let axis = axis.normalize();
                let extent = |direction: DVec3| {
                    let along = direction.dot(axis);
                    let across = (direction - axis * along).length();
                    let at = direction.dot(point);
                    let spread = across * big + radius;
                    [at - spread, at + spread]
                };
                let [x, y, z] = DVec3::AXES.map(extent);
                Some([DVec3::new(x[0], y[0], z[0]), DVec3::new(x[1], y[1], z[1])])
            }
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
            for bow in piece.bows.into_iter().flatten() {
                let point = point + axis * bow.center.x;
                walls.push(if bow.center.y == 0.0 {
                    Wall::Sphere {
                        axis,
                        point,
                        radius: bow.radius.abs(),
                    }
                } else {
                    Wall::Torus {
                        axis,
                        point,
                        big: bow.center.y.abs(),
                        radius: bow.radius.abs(),
                    }
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

/// The tolerance a wall's triangles are cut to, as a share of the reach,
/// and how many heights across a box two walls are compared at for a
/// chord's sag, as `relation/perpendicular.rs` reads them.
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
}

impl Exact {
    fn size(&self) -> f64 {
        match self {
            Exact::Round(cylinder) => cylinder.radius,
            Exact::Oval(oval) => oval.major,
        }
    }

    fn origin_and_axis(&self) -> (DVec3, DVec3) {
        match self {
            Exact::Round(cylinder) => (cylinder.origin, cylinder.axis),
            Exact::Oval(oval) => (oval.origin, oval.axis),
        }
    }

    fn point(&self, angle: f64, height: f64) -> DVec3 {
        match self {
            Exact::Round(cylinder) => cylinder.point(DVec2::new(angle, height)),
            Exact::Oval(oval) => oval.point(DVec2::new(angle, height)),
        }
    }

    /// How far a point stands from the wall: exactly from a cylinder, to
    /// first order from an elliptic wall, as the kernel measures it.
    fn gap(&self, point: DVec3) -> f64 {
        match self {
            Exact::Round(cylinder) => {
                let from = point - cylinder.origin;
                ((from - cylinder.axis * from.dot(cylinder.axis)).length() - cylinder.radius).abs()
            }
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
/// sag of the other wall, as `relation/perpendicular.rs` reads it.
fn closer_than_a_sag(first: &Barrel, second: &Barrel, low: DVec3, high: DVec3, reach: f64) -> bool {
    let tolerance = DRAWN_AT * reach;
    let (one, other) = (first.exact(), second.exact());
    [(one, other), (other, one)]
        .into_iter()
        .any(|(wall, other)| {
            let steps = divisions(wall.size(), tolerance);
            let sag = wall.size() * (1.0 - (PI / steps as f64).cos());
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
            (0..HEIGHTS).any(|rank| {
                let height = from + (to - from) * rank as f64 / (HEIGHTS - 1) as f64;
                let near = |step: usize| {
                    let angle = TAU * (step % steps) as f64 / steps as f64;
                    other.gap(wall.point(angle, height)) <= sag
                };
                (0..steps).any(|step| near(step) && near(step + 1))
            })
        })
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
            !matches!(other, Wall::Plane { .. })
                && other.asks_of_a_cone(axis, point, radius, walled.reach)
        });
        coned || self.asks_of_walls_along_a_stretch()
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
                                || closer_than_a_sag(first, second, shared_low, shared_high, reach)
                        })
                    })
                })
        })
    }

    /// Whether the case may ask the exact kernel for a curve beyond the
    /// circles it builds a sphere or a torus along (#576): a sphere or a
    /// torus of one leaf against a wall of a leaf whose box meets the whole
    /// surface's, that is neither a plane square to the turn's axis or
    /// holding it, nor a cylinder, a cone, a sphere or a torus about that
    /// very axis. The kernel may decline such a case as unsupported; it may
    /// also hold it, the walls never meeting there. The leaf's box is not the
    /// one the kernel reads: it boxes a face on a sphere or a torus by the
    /// surface's own extent, which reaches far beyond the leaf the turn drew
    /// of it, so two round walls whose leaves stand apart may still be
    /// related and declined.
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
                .any(|(axis, point, round_box)| {
                    let spanned = round_box.map(|[low, high]| (low, high));
                    walled.leaves.iter().any(|(others, bounds, _)| {
                        others.iter().any(|other| {
                            let boxed = match other {
                                Wall::Sphere { .. } | Wall::Torus { .. } => {
                                    other.round_box().map(|[low, high]| (low, high))
                                }
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
