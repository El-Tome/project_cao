//! A bore and a face standing at a slant to it, a plane neither along nor
//! square to its axis or another bore at a skew angle, whose boxes meet
//! though the faces themselves stand apart: the exact kernel cuts them
//! rather than declining, and declines only where the faces truly cross
//! along a curve it does not build (#526).
//!
//! Every part here is turned and moved off the planes of the origin as a
//! whole, so that the tilted cylinders and their triangles are exercised too:
//! the result must keep every rule the soundness harness holds a solid to.

use std::f64::consts::{PI, TAU};

use cao_solid::brep::{Body, Declined};
use cao_solid::profile::{Contour, Frame, Run};
use cao_solid::soundness::{Random, closed, enclosed, listed, repeatable, uncrossed};
use glam::{DQuat, DVec2, DVec3};

/// How far a triangle may stand from the surface it stands for.
const MESH: f64 = 0.01;

/// The hexagon's corners stand this far from its axis.
const CORNER: f64 = 20.0;
const HEIGHT: f64 = 30.0;

/// How far the hexagon's sides stand from its axis.
fn apothem() -> f64 {
    CORNER * (PI / 6.0).cos()
}

/// Where a part is laid in the world: turned, then moved off the origin.
#[derive(Debug)]
struct Pose {
    turn: DQuat,
    shift: DVec3,
}

impl Pose {
    /// Standing as drawn, turned about its own upright axis only, and turned
    /// about axes at a slant: the boxes round the faces grow with the slant,
    /// and meet where the faces do not.
    fn all() -> Vec<Pose> {
        [
            (DVec3::Z, 0.0),
            (DVec3::Z, 0.26),
            (DVec3::new(1.0, 2.0, 3.0), 0.7),
            (DVec3::X, 0.5),
            (DVec3::new(1.0, 1.0, 0.0), 0.9),
            (DVec3::new(0.3, -1.0, 2.0), 1.3),
        ]
        .into_iter()
        .map(|(axis, angle)| Pose {
            turn: DQuat::from_axis_angle(axis.normalize(), angle),
            shift: DVec3::new(3.0, -7.0, 11.0),
        })
        .collect()
    }

    /// Turned about an axis drawn at random, by an angle drawn at random.
    fn drawn(random: &mut Random) -> Pose {
        let axis = DVec3::new(
            random.between(-1.0, 1.0),
            random.between(-1.0, 1.0),
            random.between(0.1, 1.0),
        );
        Pose {
            turn: DQuat::from_axis_angle(axis.normalize(), random.between(0.0, PI)),
            shift: DVec3::new(
                random.between(-20.0, 20.0),
                random.between(-20.0, 20.0),
                random.between(-20.0, 20.0),
            ),
        }
    }

    fn frame(&self, origin: DVec3, u: DVec3, v: DVec3) -> Frame {
        Frame {
            origin: self.turn * origin + self.shift,
            u: self.turn * u,
            v: self.turn * v,
        }
    }

    fn raised(&self, outline: Contour, origin: DVec3, u: DVec3, v: DVec3, travel: DVec3) -> Body {
        Body::raised(&outline, &[], self.frame(origin, u, v), self.turn * travel)
            .expect("a leaf raises")
    }
}

/// A whole circle, handed as one run all the way round from one corner.
fn circle(radius: f64) -> Contour {
    Contour {
        corners: vec![DVec2::X * radius],
        runs: vec![Run::Round {
            center: DVec2::ZERO,
            turn: TAU,
        }],
    }
}

/// The outward normal of the hexagon's side `side`, square to its axis.
fn outward(side: usize) -> DVec3 {
    let angle = side as f64 * PI / 3.0;
    DVec3::new(angle.cos(), angle.sin(), 0.0)
}

/// A hexagonal prism standing on its floor, its side 0 facing along X.
fn hexagon(pose: &Pose) -> Body {
    let corners = (0..6)
        .map(|corner| {
            let angle = (2 * corner - 1) as f64 * PI / 6.0;
            DVec2::new(angle.cos(), angle.sin()) * CORNER
        })
        .collect();
    pose.raised(
        Contour::straight(corners),
        DVec3::ZERO,
        DVec3::X,
        DVec3::Y,
        DVec3::Z * HEIGHT,
    )
}

fn hexagon_volume() -> f64 {
    3.0 * 3.0f64.sqrt() / 2.0 * CORNER * CORNER * HEIGHT
}

/// A bore square into the side `side` of the hexagon, `aside` along the side
/// from its middle and at mid-height, `depth` deep: raised from a hair
/// outside the side, inwards.
fn bore(pose: &Pose, side: usize, aside: f64, radius: f64, depth: f64) -> Body {
    bore_at(pose, side, DVec2::new(aside, HEIGHT / 2.0), radius, depth)
}

/// A bore square into the side `side` of the hexagon about `at`: along the
/// side from its middle, and up from the floor.
fn bore_at(pose: &Pose, side: usize, at: DVec2, radius: f64, depth: f64) -> Body {
    let normal = outward(side);
    let along = DVec3::Z.cross(normal);
    let entry = normal * (apothem() + 1.0) + along * at.x + DVec3::Z * at.y;
    pose.raised(
        circle(radius),
        entry,
        along,
        DVec3::Z,
        -normal * (depth + 1.0),
    )
}

fn bored(radius: f64, depth: f64) -> f64 {
    PI * radius * radius * depth
}

/// A result held to every rule, in every pose.
fn keeps_every_rule(made: impl Fn(&Pose) -> Result<Body, Declined>, arithmetic: f64) {
    for pose in Pose::all() {
        holds(|| made(&pose), arithmetic, &format!("{pose:?}"));
    }
}

/// A result held to every rule: its listing stands on its geometry, its
/// volume is the arithmetic's, its triangles are closed and uncrossed and
/// enclose that volume within their sagitta, and the same operation answered
/// twice gives the same triangles.
fn holds(made: impl Fn() -> Result<Body, Declined>, arithmetic: f64, case: &str) {
    let body = made().unwrap_or_else(|declined| panic!("{declined:?} in {case}"));
    assert_eq!(
        listed(&body.listing(), body.scale().reach()),
        Ok(()),
        "{case}"
    );
    let volume = body.volume();
    assert!(
        ((volume - arithmetic) / arithmetic).abs() <= 1e-9,
        "the exact volume is {volume}, the arithmetic {arithmetic}, in {case}"
    );
    let triangles = body.triangles(MESH);
    assert_eq!(closed(&triangles), Ok(()), "{case}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{case}");
    let drawn = enclosed(&triangles);
    assert!(
        ((drawn - arithmetic) / arithmetic).abs() <= 1e-3,
        "the triangles enclose {drawn}, the arithmetic {arithmetic}, in {case}"
    );
    let again = made().expect("the kernel answers twice");
    assert_eq!(
        repeatable(&triangles, &again.triangles(MESH)),
        Ok(()),
        "{case}"
    );
}

#[test]
fn a_bore_square_to_one_side_of_a_hexagonal_prism_is_cut() {
    keeps_every_rule(
        |pose| hexagon(pose).cut_by(&bore(pose, 0, 0.0, 4.0, 6.0)),
        hexagon_volume() - bored(4.0, 6.0),
    );
}

#[test]
fn a_wide_bore_beside_the_middle_of_a_side_is_cut() {
    keeps_every_rule(
        |pose| hexagon(pose).cut_by(&bore(pose, 0, 3.0, 5.0, 6.0)),
        hexagon_volume() - bored(5.0, 6.0),
    );
}

#[test]
fn two_holes_in_two_adjacent_sides_of_a_hexagon_that_do_not_meet_are_cut() {
    keeps_every_rule(
        |pose| {
            hexagon(pose)
                .cut_by(&bore(pose, 0, 4.0, 3.0, 4.0))?
                .cut_by(&bore(pose, 1, -4.0, 3.0, 4.0))
        },
        hexagon_volume() - 2.0 * bored(3.0, 4.0),
    );
}

/// Two bores drawn at random into two adjacent sides, the part laid at
/// random: where they cross each other or a side at a slant the operation is
/// declined, and otherwise it holds every rule with the arithmetic's volume —
/// neither crossing anything but its own side, each takes its own matter.
#[test]
fn two_bores_drawn_at_random_into_adjacent_sides_are_cut_or_declined_as_unsupported() {
    const CASES: u64 = 60;
    let mut cut = 0;
    for seed in 0..CASES {
        let mut random = Random::seeded(seed);
        let pose = Pose::drawn(&mut random);
        let side = random.below(6);
        let drawn: Vec<(usize, DVec2, f64, f64)> = [side, (side + 1) % 6]
            .into_iter()
            .map(|side| {
                let radius = random.between(1.0, 5.0);
                let at = DVec2::new(
                    random.between(-9.0, 9.0),
                    random.between(radius + 1.0, HEIGHT - radius - 1.0),
                );
                (side, at, radius, random.between(1.0, 15.0))
            })
            .collect();
        let made = || {
            drawn
                .iter()
                .try_fold(hexagon(&pose), |part, &(side, at, radius, depth)| {
                    part.cut_by(&bore_at(&pose, side, at, radius, depth))
                })
        };
        match made() {
            Err(Declined::Unsupported) => {}
            _ => {
                let arithmetic = drawn
                    .iter()
                    .fold(hexagon_volume(), |left, &(_, _, radius, depth)| {
                        left - bored(radius, depth)
                    });
                holds(
                    made,
                    arithmetic,
                    &format!("seed {seed}: {drawn:?} in {pose:?}"),
                );
                cut += 1;
            }
        }
    }
    assert!(cut >= CASES / 4, "only {cut} of {CASES} cut");
}

#[test]
fn a_boss_standing_square_on_one_side_of_a_hexagon_is_joined() {
    keeps_every_rule(
        |pose| hexagon(pose).joined(&boss(pose, 0, 0.0, 4.0, 6.0)),
        hexagon_volume() + bored(4.0, 6.0),
    );
}

/// A boss standing square on the side `side` of the hexagon, `aside` along
/// the side from its middle and at mid-height, `height` high: raised from a
/// hair inside the side, outwards.
fn boss(pose: &Pose, side: usize, aside: f64, radius: f64, height: f64) -> Body {
    let normal = outward(side);
    let along = DVec3::Z.cross(normal);
    let entry = normal * (apothem() - 1.0) + along * aside + DVec3::Z * HEIGHT / 2.0;
    pose.raised(
        circle(radius),
        entry,
        along,
        DVec3::Z,
        normal * (height + 1.0),
    )
}

/// A block whose side towards X is chamfered from its top down to its floor's
/// edge: drawn in the plane XZ and raised along Y.
fn chamfered(pose: &Pose) -> Body {
    pose.raised(
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(40.0, 0.0),
            DVec2::new(30.0, 10.0),
            DVec2::new(0.0, 10.0),
        ]),
        DVec3::ZERO,
        DVec3::X,
        DVec3::Z,
        -DVec3::Y * 20.0,
    )
}

fn chamfered_volume() -> f64 {
    (40.0 + 30.0) / 2.0 * 10.0 * 20.0
}

/// A hole standing along Z from below the block's floor up to `top`, about
/// `(x, y)`.
fn standing(pose: &Pose, x: f64, y: f64, radius: f64, top: f64) -> Body {
    pose.raised(
        circle(radius),
        DVec3::new(x, y, -1.0),
        DVec3::X,
        DVec3::Y,
        DVec3::Z * (top + 1.0),
    )
}

#[test]
fn a_blind_hole_beneath_a_chamfer_is_cut() {
    keeps_every_rule(
        |pose| chamfered(pose).cut_by(&standing(pose, 33.0, -10.0, 2.0, 3.0)),
        chamfered_volume() - bored(2.0, 3.0),
    );
}

#[test]
fn a_bore_crossing_a_slanted_side_is_still_declined() {
    for pose in Pose::all() {
        assert_eq!(
            chamfered(&pose).cut_by(&standing(&pose, 33.0, -10.0, 2.0, 12.0)),
            Err(Declined::Unsupported),
            "{pose:?}"
        );
    }
}

#[test]
fn a_bore_square_to_one_side_poking_through_the_next_is_still_declined() {
    for pose in Pose::all() {
        assert_eq!(
            hexagon(&pose).cut_by(&bore(&pose, 0, 8.0, 3.0, 8.0)),
            Err(Declined::Unsupported),
            "{pose:?}"
        );
    }
}
