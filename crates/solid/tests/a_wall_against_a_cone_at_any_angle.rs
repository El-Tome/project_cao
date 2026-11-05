//! A wall against a cone at any angle (#577, lane 3): a radial hole through
//! a cone, a pin through a taper, two cones of other axes, an elliptic wall
//! against a cone — each held to a volume reasoned out apart from the kernel,
//! its listing and its triangles.
//!
//! Every reference is a quadrature over the lines or the rays of the tool,
//! each meeting the cone's quadric in a quadratic: along a hole's or a pin's
//! lines, the chord inside the cone, over the hole's section; from a pointed
//! tool's apex, the cube of each ray's reach inside the other cone, over the
//! tool's solid angle. Each tool is laid so that every line or ray enters
//! and leaves the cone through its wall, which keeps the integrand smooth,
//! and the rule — Gauss over the radius, trapezoids round the turn —
//! converges geometrically: sixteen nodes already agree with thirty-two to
//! the last digit or two.
//!
//! Closes #577.
//! - A slanted hole through a shaft, at 30° and at 80°, holds its volume. The
//!   reference is a quadrature written in the test — no test: held by
//!   `a_hole_slanted_at_30_degrees_through_a_shaft_holds_its_volume` and
//!   `a_hole_slanted_at_80_degrees_through_a_shaft_holds_its_volume`, in
//!   `two_walls_at_any_angle.rs`, which lane 2 delivered
//! - A radial hole through a cone, and a pin through a taper, hold their
//!   volume — `a_radial_hole_through_a_cone_holds_its_volume`,
//!   `a_pin_through_a_taper_holds_its_volume`
//! - Two cones of other axes, and an elliptic wall against a cone, hold their
//!   volume — `two_cones_crossing_square_hold_their_volume`,
//!   `two_cones_crossing_at_a_skew_angle_hold_their_volume`,
//!   `an_elliptic_wall_against_a_cone_holds_its_volume`
//! - Pairs a hair from a touch or a node are snapped. A lean a hair off square
//!   or parallel is laid exactly so, and one in the band is noted unstable —
//!   for a cone, `a_hole_a_hair_from_touching_a_cone_s_wall_inside_is_moved_onto_the_touch`,
//!   `a_radial_hole_a_hair_off_square_to_a_cone_is_laid_square`,
//!   `a_bore_a_hair_off_parallel_to_a_cone_s_axis_is_laid_parallel`,
//!   `a_radial_hole_leaning_in_the_band_off_square_to_a_cone_is_noted_unstable`;
//!   for two walls, held in `two_walls_at_any_angle.rs` by lane 2's tests. A
//!   cone has an axis, and only its axis is read square or parallel to
//!   another wall's: a ruling parallel to a wall's axis is no lean the kernel
//!   lays, the curve there being the one the cone's slope gives.
//! - #448's harness draws walls at any angle before the kernel code.
//!   Campaigns end under one failure in a thousand on fresh seeds, with no
//!   regression. A family that does not move in two rounds is reported and
//!   set aside — no test: held by
//!   `the_draw_of_walls_at_any_angle_draws_every_pair_the_issue_names` in
//!   `random_walls_at_any_angle.rs`, whose draw already draws cones and whose
//!   scope rule stops excusing a cone off its axis when the kernel builds
//!   it; the campaigns run by hand, and the rate and the families set aside
//!   (#706, #707) are a record in `docs/exact-kernel-failures.md`
//! - `REBUILT_BY` moves, and `docs/exact-kernel.md` says what is built and
//!   what still declines — no test: the number is a constant a test would
//!   only restate, and the document is prose
//! - Must not break: perpendicular and parallel pairs keep their bits, and
//!   everything #526 to #571 deliver — `the_cones_built_before_keep_their_bits`
//!   for cones; the walls' bits held by `perpendicular_and_parallel_pairs_keep_their_bits`
//!   in `two_walls_at_any_angle.rs`, and the rest by the gate as it stands

use std::f64::consts::{PI, TAU};

use cao_solid::Lean;
use cao_solid::brep::{Body, Declined, Surface};
use cao_solid::profile::{Contour, Frame, Run};
use cao_solid::soundness::{closed, listed, uncrossed};
use cao_solid::turning::{Axis, Corner, Straight, Turn};
use glam::{DVec2, DVec3};

/// How far a triangle may stand from the surface it stands for.
const MESH: f64 = 0.01;

/// A cone about Z narrowing as `r(z) = foot − slope·z`: the frustum's or
/// the taper's wall, its quadric `x² + y² = r(z)²`.
#[derive(Clone, Copy)]
struct Cone {
    foot: f64,
    slope: f64,
}

/// The frustum: radius 12 at its foot on XY, 4 at its top at 20.
const FRUSTUM: Cone = Cone {
    foot: 12.0,
    slope: 0.4,
};

/// The taper: radius 6 at its foot, 4 at its top at 20.
const TAPER: Cone = Cone {
    foot: 6.0,
    slope: 0.1,
};

const HEIGHT: f64 = 20.0;

impl Cone {
    /// The volume of the solid from its foot to `HEIGHT`.
    fn volume(self) -> f64 {
        let (low, high) = (self.foot, self.foot - self.slope * HEIGHT);
        PI * HEIGHT / 3.0 * (low * low + low * high + high * high)
    }

    /// The quadratic `a t² + b t + c` the line through `from` along `way`
    /// meets the quadric by, nought on the wall.
    fn quadratic(self, from: DVec3, way: DVec3) -> [f64; 3] {
        let radius = self.foot - self.slope * from.z;
        [
            way.x * way.x + way.y * way.y - self.slope * self.slope * way.z * way.z,
            2.0 * (from.x * way.x + from.y * way.y) + 2.0 * self.slope * radius * way.z,
            from.x * from.x + from.y * from.y - radius * radius,
        ]
    }

    /// Where the line through `from` along `way`, of unit length, enters and
    /// leaves the cone: a line running across it, `a > 0`, lies inside
    /// between the two roots.
    fn roots(self, from: DVec3, way: DVec3) -> [f64; 2] {
        let [a, b, c] = self.quadratic(from, way);
        let root = (b * b - 4.0 * a * c).max(0.0).sqrt();
        [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)]
    }

    fn chord(self, from: DVec3, way: DVec3) -> f64 {
        let [enter, leave] = self.roots(from, way);
        leave - enter
    }

    /// The cone about Z from its foot on XY to `HEIGHT`, turned whole.
    fn body(self) -> Body {
        let top = self.foot - self.slope * HEIGHT;
        turned(
            &[[0.0, 0.0], [HEIGHT, 0.0], [HEIGHT, top], [0.0, self.foot]],
            DVec3::ZERO,
            DVec3::Z,
            DVec3::X,
        )
    }
}

/// A profile of straight runs drawn `(along, away)` from `origin`, along
/// `along` and away towards `away`, its first run on the axis, turned whole.
fn turned(corners: &[[f64; 2]], origin: DVec3, along: DVec3, away: DVec3) -> Body {
    let corners: Vec<Corner> = corners
        .iter()
        .enumerate()
        .map(|(run, at)| Corner {
            at: DVec2::from(*at),
            run: run as u32,
            bend: None,
        })
        .collect();
    let runs = corners.len() as u32;
    let straight = Straight {
        side: 1.0,
        contours: vec![corners],
        runs,
        last_off_the_axis: Some(runs - 1),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::X,
        },
        angle: TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(
        &straight,
        Frame {
            origin,
            u: along,
            v: away,
        },
        &turn,
    )
    .expect("a profile turns")
}

/// Two directions square to `way`, of unit length, square to each other.
fn square_to(way: DVec3) -> (DVec3, DVec3) {
    let u = way.any_orthonormal_vector();
    (u, way.cross(u))
}

/// The nodes and weights of Gauss–Legendre's rule of `count` points on
/// `[0, 1]`, each node found by Newton from Tricomi's guess.
fn gauss(count: usize) -> Vec<(f64, f64)> {
    (1..=count)
        .map(|index| {
            let legendre = |x: f64| {
                let (mut before, mut now) = (1.0, x);
                for degree in 2..=count {
                    let degree = degree as f64;
                    (before, now) = (
                        now,
                        ((2.0 * degree - 1.0) * x * now - (degree - 1.0) * before) / degree,
                    );
                }
                (now, count as f64 * (x * now - before) / (x * x - 1.0))
            };
            let mut x = (PI * (index as f64 - 0.25) / (count as f64 + 0.5)).cos();
            for _ in 0..100 {
                let (value, slope) = legendre(x);
                let step = value / slope;
                x -= step;
                if step.abs() < 1e-16 {
                    break;
                }
            }
            let (_, slope) = legendre(x);
            ((x + 1.0) / 2.0, 1.0 / ((1.0 - x * x) * slope * slope))
        })
        .collect()
}

/// `∫∫ f` over the ellipse about `center` of half-axes `first` along `u`
/// and `second` along `v`: Gauss over the radius, trapezoids round the turn.
fn over_an_ellipse(
    center: DVec3,
    (u, first): (DVec3, f64),
    (v, second): (DVec3, f64),
    nodes: usize,
    f: impl Fn(DVec3) -> f64,
) -> f64 {
    let turns = 4 * nodes;
    gauss(nodes)
        .into_iter()
        .map(|(radius, weight)| {
            let round: f64 = (0..turns)
                .map(|step| {
                    let (sin, cos) = (TAU * step as f64 / turns as f64).sin_cos();
                    f(center + u * (first * radius * cos) + v * (second * radius * sin))
                })
                .sum();
            weight * first * second * radius * round * TAU / turns as f64
        })
        .sum()
}

/// What a cylinder of `radius` along `way` through `center` shares with
/// `cone`: each of its lines' chord inside the cone, over its section.
fn shared_with_a_cylinder(cone: Cone, center: DVec3, way: DVec3, radius: f64, nodes: usize) -> f64 {
    let (u, v) = square_to(way);
    over_an_ellipse(center, (u, radius), (v, radius), nodes, |from| {
        cone.chord(from, way)
    })
}

/// What a point of half-angle `half` whose apex stands at `apex`, pointing
/// along `way`, shares with `cone`: each ray's `(ρ_out³ − ρ_in³)/3` inside
/// the cone, over the point's solid angle.
fn shared_with_a_point(cone: Cone, apex: DVec3, way: DVec3, half: f64) -> f64 {
    let (u, v) = square_to(way);
    let turns = 64;
    gauss(16)
        .into_iter()
        .map(|(share, weight)| {
            let tilt = share * half;
            let round: f64 = (0..turns)
                .map(|step| {
                    let (sin, cos) = (TAU * step as f64 / turns as f64).sin_cos();
                    let ray = way * tilt.cos() + (u * cos + v * sin) * tilt.sin();
                    let [enter, leave] = cone.roots(apex, ray);
                    (leave.powi(3) - enter.powi(3)) / 3.0
                })
                .sum();
            weight * half * tilt.sin() * round * TAU / turns as f64
        })
        .sum()
}

/// A cylinder of `radius` along `way` through `center`, from `reach`
/// before it to `reach` past it.
fn cylinder(center: DVec3, way: DVec3, radius: f64, reach: f64) -> Body {
    elliptic(
        center,
        way,
        way.any_orthonormal_vector() * radius,
        radius,
        reach,
    )
}

/// An elliptic cylinder along `way` through `center`, its first half-axis
/// `first`, square to `way`, and the other `second` long.
fn elliptic(center: DVec3, way: DVec3, first: DVec3, second: f64, reach: f64) -> Body {
    let u = first.normalize();
    let v = way.cross(u);
    let outline = Contour {
        corners: vec![DVec2::X * first.length()],
        runs: vec![Run::Oval {
            center: DVec2::ZERO,
            first: DVec2::X * first.length(),
            second,
            turn: TAU,
        }],
    };
    Body::raised(
        &outline,
        &[],
        Frame {
            origin: center - way * reach,
            u,
            v,
        },
        way * 2.0 * reach,
    )
    .expect("a leaf raises")
}

/// A point of half-angle `half` whose apex stands at `apex`, along `way`,
/// `length` long.
fn point(apex: DVec3, way: DVec3, half: f64, length: f64) -> Body {
    let (away, _) = square_to(way);
    turned(
        &[[0.0, 0.0], [length, 0.0], [length, length * half.tan()]],
        apex,
        way,
        away,
    )
}

/// A result held to the volume reasoned out within `share`, its listing and
/// its triangles.
fn holds_within(result: Result<Body, Declined>, reasoned: f64, share: f64) -> Body {
    let body = result.expect("the kernel answers");
    let volume = body.volume();
    assert!(
        ((volume - reasoned) / reasoned).abs() <= share,
        "the exact volume is {volume}, the quadrature {reasoned}"
    );
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    let triangles = body.triangles(MESH);
    assert_eq!(closed(&triangles), Ok(()));
    assert_eq!(uncrossed(&triangles), Ok(()));
    body
}

fn holds(result: Result<Body, Declined>, reasoned: f64) -> Body {
    holds_within(result, reasoned, 1e-9)
}

const MIDDLE: DVec3 = DVec3::new(0.0, 0.0, 10.0);

/// The way at `degrees` off Z, leaning towards X.
fn off_z(degrees: f64) -> DVec3 {
    let (sin, cos) = degrees.to_radians().sin_cos();
    DVec3::new(sin, 0.0, cos)
}

#[test]
fn a_radial_hole_through_a_cone_holds_its_volume() {
    let hole = cylinder(MIDDLE, DVec3::X, 3.0, 30.0);
    holds(
        FRUSTUM.body().cut_by(&hole),
        FRUSTUM.volume() - shared_with_a_cylinder(FRUSTUM, MIDDLE, DVec3::X, 3.0, 16),
    );
}

#[test]
fn a_pin_through_a_taper_holds_its_volume() {
    let way = off_z(60.0);
    let pin = cylinder(MIDDLE, way, 1.5, 15.0);
    holds(
        TAPER.body().joined(&pin),
        TAPER.volume() + PI * 1.5 * 1.5 * 30.0
            - shared_with_a_cylinder(TAPER, MIDDLE, way, 1.5, 16),
    );
}

/// A point of 5° whose apex stands thirty before `through` along `way`,
/// sixty long: every ray of it enters and leaves the frustum through its
/// wall, between sixteen and forty from the apex.
fn crossing_point(through: DVec3, way: DVec3) -> (Body, f64) {
    let apex = through - way * 30.0;
    let half = 5f64.to_radians();
    (
        point(apex, way, half, 60.0),
        shared_with_a_point(FRUSTUM, apex, way, half),
    )
}

#[test]
fn two_cones_crossing_square_hold_their_volume() {
    let (tool, shared) = crossing_point(MIDDLE, DVec3::X);
    holds(FRUSTUM.body().cut_by(&tool), FRUSTUM.volume() - shared);
}

#[test]
fn two_cones_crossing_at_a_skew_angle_hold_their_volume() {
    let (tool, shared) = crossing_point(MIDDLE + DVec3::Y, off_z(60.0));
    holds(FRUSTUM.body().cut_by(&tool), FRUSTUM.volume() - shared);
}

#[test]
fn an_elliptic_wall_against_a_cone_holds_its_volume() {
    let hole = elliptic(MIDDLE, DVec3::X, DVec3::Y * 4.0, 2.0, 30.0);
    let shared = over_an_ellipse(MIDDLE, (DVec3::Y, 4.0), (DVec3::Z, 2.0), 16, |from| {
        FRUSTUM.chord(from, DVec3::X)
    });
    holds(FRUSTUM.body().cut_by(&hole), FRUSTUM.volume() - shared);
}

/// The sine each pair of distinct walls of a body leans off square or off
/// parallel by, for the pairs within a thousandth of either: what decision
/// 11 is to lay to nought.
fn hairs_off(body: &Body) -> Vec<f64> {
    let mut walls: Vec<Surface> = Vec::new();
    for face in body.listing().faces {
        if !walls.contains(&face.surface) {
            walls.push(face.surface);
        }
    }
    let axes: Vec<DVec3> = walls
        .iter()
        .filter_map(|wall| match wall {
            Surface::Cylinder(wall) => Some(wall.axis),
            Surface::Elliptic(wall) => Some(wall.axis),
            Surface::Cone(wall) => Some(wall.axis),
            _ => None,
        })
        .collect();
    let mut hairs = Vec::new();
    for (index, one) in axes.iter().enumerate() {
        for other in &axes[index + 1..] {
            let sine = one.cross(*other).length().min(one.dot(*other).abs());
            if sine < 1e-3 {
                hairs.push(sine);
            }
        }
    }
    hairs
}

/// A body whose walls a hair off square or off parallel were laid exactly
/// so, and noted as nothing unstable.
fn laid(body: &Body) {
    let hairs = hairs_off(body);
    assert!(
        !hairs.is_empty() && hairs.iter().all(|sine| *sine == 0.0),
        "walls left a hair off: {hairs:?}"
    );
    assert_eq!(body.unstable(), None);
}

/// The way along X leaning `degrees` off square to Z, towards Z.
fn off_square(degrees: f64) -> DVec3 {
    let (sin, cos) = degrees.to_radians().sin_cos();
    DVec3::new(cos, 0.0, sin)
}

#[test]
fn a_radial_hole_a_hair_off_square_to_a_cone_is_laid_square() {
    let hole = cylinder(MIDDLE, off_square(1e-7), 3.0, 30.0);
    let body = holds_within(
        FRUSTUM.body().cut_by(&hole),
        FRUSTUM.volume() - shared_with_a_cylinder(FRUSTUM, MIDDLE, DVec3::X, 3.0, 16),
        1e-8,
    );
    laid(&body);
}

/// A bore of radius 1 six off the frustum's axis, from below its foot to
/// above where it leaves its wall, leaning a hair off parallel to its axis.
/// Laid parallel, each of its lines, `ρ` from the axis, runs from the foot
/// to where the wall narrows to `ρ`: `(12 − ρ)/0.4`.
#[test]
fn a_bore_a_hair_off_parallel_to_a_cone_s_axis_is_laid_parallel() {
    let center = DVec3::new(6.0, 0.0, 10.0);
    let bore = cylinder(center, off_z(1e-7), 1.0, 15.0);
    let taken = over_an_ellipse(center, (DVec3::X, 1.0), (DVec3::Y, 1.0), 16, |from| {
        (FRUSTUM.foot - from.truncate().length()) / FRUSTUM.slope
    });
    let body = holds_within(FRUSTUM.body().cut_by(&bore), FRUSTUM.volume() - taken, 1e-8);
    laid(&body);
}

#[test]
fn a_radial_hole_leaning_in_the_band_off_square_to_a_cone_is_noted_unstable() {
    let lean = 1e-3;
    let way = off_square(lean);
    let body = holds(
        FRUSTUM.body().cut_by(&cylinder(MIDDLE, way, 3.0, 30.0)),
        FRUSTUM.volume() - shared_with_a_cylinder(FRUSTUM, MIDDLE, way, 3.0, 16),
    );
    let unstable = body.unstable().expect("the step is noted unstable");
    assert_eq!(
        unstable.lean,
        Lean::Across,
        "the hole's axis nearly square to the cone's"
    );
    assert!(
        ((unstable.degrees - lean) / lean).abs() <= 1e-6,
        "noted {} degrees off, drawn {lean}",
        unstable.degrees
    );
}

/// A hole of radius 2 along X at height 10, its axis as far from the
/// frustum's ruling on `+Y` as its radius, but for a hair: inside, it
/// touches the wall at one point. Its lines' chords vanish there, so the
/// rule converges more slowly: 256 nodes hold it to some 1e-10.
#[test]
fn a_hole_a_hair_from_touching_a_cone_s_wall_inside_is_moved_onto_the_touch() {
    let across = (1.0 + FRUSTUM.slope * FRUSTUM.slope).sqrt();
    let touching = FRUSTUM.foot - FRUSTUM.slope * 10.0 - 2.0 * across;
    let center = MIDDLE + DVec3::Y * touching;
    let hole = cylinder(center + DVec3::Y * 1e-8, DVec3::X, 2.0, 30.0);
    let body = holds(
        FRUSTUM.body().cut_by(&hole),
        FRUSTUM.volume() - shared_with_a_cylinder(FRUSTUM, center, DVec3::X, 2.0, 256),
    );
    let touch = center + DVec3::new(0.0, 1.0, FRUSTUM.slope) * (2.0 / across);
    let at = body.listing().vertices;
    assert!(
        at.iter().any(|vertex| vertex.distance(touch) <= 1e-9),
        "the two loops join at a node where the walls touch, {touch}: {at:?}"
    );
}

/// What a body is to the bit: every vertex, the ends and the middle of every
/// edge as its curve gives them, sorted, and the volume — folded into one
/// number.
fn bits(body: &Body) -> u64 {
    let listing = body.listing();
    let mut points: Vec<[u64; 3]> = listing
        .vertices
        .iter()
        .copied()
        .chain(listing.edges.iter().flat_map(|edge| {
            [edge.from, (edge.from + edge.to) / 2.0, edge.to].map(|t| edge.curve.point(t))
        }))
        .map(|point| point.to_array().map(f64::to_bits))
        .collect();
    points.sort_unstable();
    points
        .into_iter()
        .flatten()
        .chain([body.volume().to_bits()])
        .flat_map(u64::to_le_bytes)
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
        })
}

/// Pinned before any kernel code of lane 3; a Mac and Linux gave the same
/// bits, though their `sin` and `cos` may part in the last one (#541).
#[test]
fn the_cones_built_before_keep_their_bits() {
    let slab = |origin: DVec3, u: DVec3, v: DVec3| {
        Body::raised(
            &Contour::rectangle(DVec2::splat(-30.0), DVec2::splat(30.0)),
            &[],
            Frame { origin, u, v },
            u.cross(v) * 30.0,
        )
        .expect("a slab raises")
    };
    for (what, made, pinned) in [
        (
            "a bore about the frustum's axis",
            FRUSTUM
                .body()
                .cut_by(&cylinder(MIDDLE, DVec3::Z, 2.0, 15.0)),
            0x4ca2_12ba_ec05_fa12,
        ),
        (
            "a shaft about the frustum's axis joined through its top",
            FRUSTUM
                .body()
                .joined(&cylinder(MIDDLE, DVec3::Z, 6.0, 15.0)),
            0x5863_b24c_2cc3_0c40,
        ),
        (
            "the frustum cut by a plane at a slant",
            FRUSTUM.body().cut_by(&slab(
                MIDDLE,
                DVec3::X,
                DVec3::new(0.0, 30f64.to_radians().cos(), 30f64.to_radians().sin()),
            )),
            0x291f_f721_71c6_7fe8,
        ),
        (
            "a coaxial point in the frustum",
            FRUSTUM
                .body()
                .cut_by(&point(DVec3::Z * 25.0, -DVec3::Z, 15f64.to_radians(), 20.0)),
            0x1407_c1e0_36cf_6d22,
        ),
    ] {
        let body = made.unwrap_or_else(|declined| panic!("{what}: {declined:?}"));
        assert_eq!(bits(&body), pinned, "{what}");
    }
}
