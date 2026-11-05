//! The box round a face lying on a sphere or a torus, and the box and the
//! reach of a body holding such faces: a face covering part of its surface
//! reaches as far as its own points do, not as far as the whole surface.
//!
//! The bodies whose boxes are stated are turned about Y from the plane XY, a
//! turn by `θ` laying the profile's `(h, ρ)` at `(ρ cos θ, h, −ρ sin θ)`: the
//! boxes below are read off that, the profile and the turn.
//!
//! Closes #625.
//! - a face on a sphere or a torus that covers part of its surface is boxed
//!   by its own extent — the extremes of its edges and the points within it
//!   where a coordinate turns — in `boxed`, `bounds`, `reach` and the ray's
//!   `Boxes`; a face that covers the whole surface still spans the whole box —
//!   `a_face_covering_part_of_its_sphere_or_torus_is_boxed_by_its_own_extent`,
//!   `a_body_whose_rounds_cover_part_of_their_surfaces_is_bounded_and_reaches_by_their_own_extent`,
//!   `a_face_covering_its_whole_sphere_or_torus_still_spans_the_whole_box`;
//!   the ray's `Boxes` in `ray/tests/round.rs`
//! - every point of such a face lies in its box: sampled over a quarter
//!   torus, a hemisphere, a torus face holding the top of its tube, a sphere
//!   face holding a pole —
//!   `every_point_of_a_face_covering_part_of_its_sphere_or_torus_lies_in_its_box`;
//!   the ray's `Boxes` in `ray/tests/round.rs`
//!
//! Beside the four the issue names, the same guard holds a ring turned 135°,
//! whose turn holds the angle where z turns at 90° and not the one at 270°,
//! and three faces turned about an axis slanted to all three axes of the
//! world: a quarter torus, whose quarter holds where y is largest and z
//! smallest but neither opposite, a torus face holding the top of its tube,
//! and a sphere face holding a pole. About Y a coordinate either runs along
//! the axis or square to it; about a slanted axis every one leans on it, and
//! a box read wrongly there would be too small exactly where a real meeting
//! is. Their own boxes are not stated: stating them exactly is what boxing
//! them is. Each is held instead to the points where a coordinate turns over
//! its whole surface, those of them that lie on it.
//!
//! The guard holds one more, which the round campaign found boxed short (seed
//! 2722): a whole torus joined with a coaxial tube whose bore touches its
//! outer equator. The join bounds the torus's face by that circle, so its box
//! is read off its edges and the points it holds; the profile's one corner,
//! at the top of the tube, puts the face's seam where z turns, and the face
//! still holds the top of its tube there.

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use crate::brep::turned::tests::about_y;
use crate::brep::turned::tests::round::{apple, ball, bent, cap, ring, turned};
use crate::brep::{Body, FaceId, Location, Surface};
use crate::profile::{Contour, Frame, Run};
use crate::turning::{Axis, ON_THE_AXIS, Straight, Turn};

/// How far, against the size of a box, a box read off closed forms may stand
/// from the exact one: rounding, a thousandth of the kernel's own tolerance.
const ROUNDING: f64 = 1e-12;

/// How many steps a sampled face's surface is read in along each parameter.
const STEPS: u32 = 48;

/// A face lying on a sphere or a torus and covering part of it.
pub(in crate::brep) struct Partial {
    pub(in crate::brep) what: &'static str,
    pub(in crate::brep) body: Body,
    pub(in crate::brep) face: FaceId,
    /// The box the face's own points span.
    pub(in crate::brep) own: [DVec3; 2],
    /// The box the whole surface spans.
    pub(in crate::brep) whole: [DVec3; 2],
    /// Points of the face where it reaches each side of its own box.
    pub(in crate::brep) reached: Vec<DVec3>,
    /// A place the whole surface's box holds, a unit at least past the
    /// face's own.
    pub(in crate::brep) beyond: DVec3,
}

/// The upper half of the ring's tube, closed by the annulus at `h = 3`: a
/// torus face whose meridian arc passes the top of its tube, `y = 5`, where
/// no edge of it goes.
fn top_of_the_tube() -> Straight {
    bent(&[&[([3.0, 8.0], Some(([3.0, 6.0], -PI))), ([3.0, 4.0], None)]])
}

/// The half of the ball of radius 4 about `h = 10` below its equator, closed
/// by the disc of the equator: a sphere face holding its pole at `h = 6`,
/// whose sphere reaches on to `h = 14`.
fn bowl() -> Straight {
    bent(&[&[
        ([6.0, 0.0], Some(([10.0, 0.0], -FRAC_PI_2))),
        ([10.0, 4.0], None),
        ([10.0, 0.0], None),
    ]])
}

fn between(low: [f64; 3], high: [f64; 3]) -> [DVec3; 2] {
    [DVec3::from(low), DVec3::from(high)]
}

/// The faces of #625 that cover part of their sphere or torus, the bowl
/// besides, whose sphere reaches further from the origin than it does.
pub(in crate::brep) fn partial_rounds() -> Vec<Partial> {
    let crest = 6.0 * FRAC_1_SQRT_2;
    vec![
        partial(
            "a quarter torus",
            turned(&ring(), 90.0),
            between([0.0, 1.0, -8.0], [8.0, 5.0, 0.0]),
            between([-8.0, 1.0, -8.0], [8.0, 5.0, 8.0]),
            &[
                [8.0, 3.0, 0.0],
                [0.0, 3.0, -8.0],
                [crest, 5.0, -crest],
                [crest, 1.0, -crest],
            ],
            [-4.0, 3.0, 4.0],
        ),
        partial(
            "a ring turned 135°",
            turned(&ring(), 135.0),
            between([-8.0 * FRAC_1_SQRT_2, 1.0, -8.0], [8.0, 5.0, 0.0]),
            between([-8.0, 1.0, -8.0], [8.0, 5.0, 8.0]),
            &[
                [8.0, 3.0, 0.0],
                [-8.0 * FRAC_1_SQRT_2, 3.0, -8.0 * FRAC_1_SQRT_2],
                [0.0, 3.0, -8.0],
                [crest, 5.0, -crest],
                [crest, 1.0, -crest],
            ],
            [0.0, 3.0, 4.0],
        ),
        partial(
            "a hemisphere",
            turned(&ball(), 180.0),
            between([-5.0, -5.0, -5.0], [5.0, 5.0, 0.0]),
            between([-5.0, -5.0, -5.0], [5.0, 5.0, 5.0]),
            &[
                [5.0, 0.0, 0.0],
                [-5.0, 0.0, 0.0],
                [0.0, 5.0, 0.0],
                [0.0, -5.0, 0.0],
                [0.0, 0.0, -5.0],
            ],
            [0.0, 0.0, 3.0],
        ),
        partial(
            "a torus face holding the top of its tube",
            turned(&top_of_the_tube(), 360.0),
            between([-8.0, 3.0, -8.0], [8.0, 5.0, 8.0]),
            between([-8.0, 1.0, -8.0], [8.0, 5.0, 8.0]),
            &[
                [8.0, 3.0, 0.0],
                [-8.0, 3.0, 0.0],
                [0.0, 3.0, 8.0],
                [0.0, 3.0, -8.0],
                [6.0, 5.0, 0.0],
                [0.0, 5.0, -6.0],
            ],
            [6.0, 1.5, 0.0],
        ),
        partial(
            "a sphere face holding a pole",
            turned(&cap(), 360.0),
            between([-5.0, -3.0, -5.0], [5.0, 5.0, 5.0]),
            between([-5.0, -5.0, -5.0], [5.0, 5.0, 5.0]),
            &[
                [5.0, 0.0, 0.0],
                [-5.0, 0.0, 0.0],
                [0.0, 0.0, 5.0],
                [0.0, 0.0, -5.0],
                [0.0, 5.0, 0.0],
                [4.0, -3.0, 0.0],
            ],
            [0.0, -4.5, 0.0],
        ),
        partial(
            "a bowl",
            turned(&bowl(), 360.0),
            between([-4.0, 6.0, -4.0], [4.0, 10.0, 4.0]),
            between([-4.0, 6.0, -4.0], [4.0, 14.0, 4.0]),
            &[
                [4.0, 10.0, 0.0],
                [-4.0, 10.0, 0.0],
                [0.0, 10.0, 4.0],
                [0.0, 10.0, -4.0],
                [0.0, 6.0, 0.0],
            ],
            [0.0, 12.0, 0.0],
        ),
    ]
}

/// A partial face, its named points held to lie on it and its place beyond
/// held to stand inside the whole surface's box and outside its own.
fn partial(
    what: &'static str,
    body: Body,
    own: [DVec3; 2],
    whole: [DVec3; 2],
    reached: &[[f64; 3]],
    beyond: [f64; 3],
) -> Partial {
    let face = round_face(&body);
    let surface = body.surface(body.face(face).surface);
    let eps = body.scale().eps();
    let reached: Vec<DVec3> = reached.iter().copied().map(DVec3::from).collect();
    for &point in &reached {
        let at = surface.parameters(point);
        assert!(
            surface.point(at).distance(point) <= eps,
            "{what}: {point} lies on the surface"
        );
        assert_ne!(
            body.locate(face, at, eps),
            Ok(Location::Outside),
            "{what}: {point} lies on the face"
        );
    }
    let beyond = DVec3::from(beyond);
    assert!(
        within(beyond, whole, 0.0) && !within(beyond, own, 1.0),
        "{what}: {beyond} stands in the whole surface's box and a unit past the face's"
    );
    Partial {
        what,
        body,
        face,
        own,
        whole,
        reached,
        beyond,
    }
}

/// A face on a sphere or a torus covering part of it, with points of it its
/// box must hold besides those sampled.
pub(in crate::brep) struct Sampled {
    pub(in crate::brep) what: &'static str,
    pub(in crate::brep) body: Body,
    pub(in crate::brep) face: FaceId,
    pub(in crate::brep) named: Vec<DVec3>,
}

/// Every face the guard samples: the partial faces, with the points where
/// they reach each side of their own box, the faces turned about a slanted
/// axis, with the points of their surface where a coordinate turns that lie
/// on them, and a whole torus cut along its outer equator, with the top of
/// its tube.
pub(in crate::brep) fn sampled_rounds() -> Vec<Sampled> {
    partial_rounds()
        .into_iter()
        .map(|partial| Sampled {
            what: partial.what,
            body: partial.body,
            face: partial.face,
            named: partial.reached,
        })
        .chain(slanted_rounds())
        .chain([cut_along_its_outer_equator()])
        .collect()
}

/// The round campaign's seed 2722 in `random_turned_solids.rs`, laid as the
/// harness hands it over: a ring torus about the line along −Z through
/// `(2, 2)`, its tube of radius 5 at 5.5 from the axis, drawn as a whole
/// circle from one corner at the top of the tube, joined with a tube about
/// the same line whose bore, of radius 10.5, touches the torus's outer
/// equator. The join cuts the torus's face along that circle and leaves it
/// the whole of its torus, the top of the tube, `z = 0`, among it.
fn cut_along_its_outer_equator() -> Sampled {
    let whole_turn = |frame: Frame, axis: Axis, furthest: f64, contour: Contour| {
        let turn = Turn {
            axis,
            angle: TAU,
            resolution: 0.0,
            on_the_axis: ON_THE_AXIS * furthest,
        };
        let straight =
            Straight::of(&contour, &[], frame, &turn, 0.0).expect("the profile is straight");
        Body::turned(&straight, frame, &turn).expect("the profile turns")
    };
    let torus = whole_turn(
        Frame {
            origin: DVec3::new(0.0, 2.0, 0.0),
            u: DVec3::X,
            v: DVec3::Z,
        },
        Axis {
            origin: DVec2::new(2.0, 0.0),
            direction: -DVec2::Y,
        },
        10.5,
        Contour {
            corners: vec![DVec2::new(7.5, 0.0)],
            runs: vec![Run::Round {
                center: DVec2::new(7.5, -5.0),
                turn: TAU,
            }],
        },
    );
    let tube = whole_turn(
        Frame {
            origin: DVec3::new(2.0, 0.0, 0.0),
            u: DVec3::Y,
            v: DVec3::Z,
        },
        Axis {
            origin: DVec2::new(2.0, 0.0),
            direction: DVec2::Y,
        },
        11.0,
        Contour {
            corners: vec![
                DVec2::new(-8.5, -10.0),
                DVec2::new(-8.5, 0.0),
                DVec2::new(-9.0, 0.0),
                DVec2::new(-9.0, -10.0),
            ],
            runs: vec![Run::Straight; 4],
        },
    );
    let what = "a whole torus cut along its outer equator by a coaxial tube touching it";
    let body = torus.joined(&tube).expect("the tube joins the torus");
    let face = round_face(&body);
    assert!(
        !body.face(face).loops.is_empty(),
        "{what}: the join bounds the torus's face by the circle the tube touches"
    );
    let surface = body.surface(body.face(face).surface);
    let eps = body.scale().eps();
    let named: Vec<DVec3> = [
        [7.5, 2.0, 0.0],
        [-3.5, 2.0, 0.0],
        [2.0, 7.5, 0.0],
        [2.0, -3.5, 0.0],
    ]
    .map(DVec3::from)
    .to_vec();
    for &point in &named {
        assert!(
            surface.point(surface.parameters(point)).distance(point) <= eps,
            "{what}: {point}, at the top of the tube, lies on the torus"
        );
    }
    Sampled {
        what,
        body,
        face,
        named,
    }
}

/// A frame through `(2, −1, 3)` whose second axis, the one the fixtures
/// turn about, leans on all three axes of the world, along `(1, 2, 3)`; its
/// first, where a turn starts, is the part of `(1, 1, 0)` square to it.
fn slanted() -> Frame {
    let v = DVec3::new(1.0, 2.0, 3.0).normalize();
    let start = DVec3::new(1.0, 1.0, 0.0);
    Frame {
        origin: DVec3::new(2.0, -1.0, 3.0),
        u: (start - v * start.dot(v)).normalize(),
        v,
    }
}

fn slanted_rounds() -> Vec<Sampled> {
    let frame = slanted();
    let turned_slanted = |straight: &Straight, degrees: f64| {
        Body::turned(straight, frame, &about_y(degrees))
            .expect("the profile turns about a slanted axis")
    };
    let tube = torus_turning_points(frame.origin, frame.v, DVec2::new(3.0, 6.0), 2.0);
    let mut ball = sphere_turning_points(frame.origin, 5.0);
    ball.push(frame.origin + frame.v * 5.0);
    vec![
        held(
            "a quarter torus about a slanted axis",
            turned_slanted(&ring(), 90.0),
            tube.clone(),
            2,
        ),
        held(
            "a torus face holding the top of its tube about a slanted axis",
            turned_slanted(&top_of_the_tube(), 360.0),
            tube,
            3,
        ),
        held(
            "a sphere face holding a pole about a slanted axis",
            turned_slanted(&cap(), 360.0),
            ball,
            5,
        ),
    ]
}

/// The points where each coordinate is largest and smallest over the whole
/// torus about the line through `origin` along the unit `axis`, its meridian
/// circle of `radius` about `(h, ρ)`. A coordinate whose direction leans `k`
/// on the axis is largest `radius·k` further along the axis than the circle's
/// centre and `radius·√(1 − k²)` further from it, toward that direction, and
/// smallest at the mirror of that.
fn torus_turning_points(origin: DVec3, axis: DVec3, center: DVec2, radius: f64) -> Vec<DVec3> {
    DVec3::AXES
        .into_iter()
        .flat_map(|direction| {
            let k = direction.dot(axis);
            let square = (1.0 - k * k).sqrt();
            let away = (direction - axis * k) / square;
            let out = center.y + radius * square;
            [
                origin + axis * (center.x + radius * k) + away * out,
                origin + axis * (center.x - radius * k) - away * out,
            ]
        })
        .collect()
}

/// The points where each coordinate is largest and smallest over a sphere.
fn sphere_turning_points(center: DVec3, radius: f64) -> Vec<DVec3> {
    DVec3::AXES
        .into_iter()
        .flat_map(|direction| [center + direction * radius, center - direction * radius])
        .collect()
}

/// A face sampled for its box, named by those of `points` that lie on it,
/// every one of `points` held to lie on its surface and `inside` of them
/// within the face, where no edge goes: the slanted quarter holds where y is
/// largest and z smallest, the top of the tube where each coordinate is
/// largest, the cap where x and y are largest and smallest and z largest.
fn held(what: &'static str, body: Body, points: Vec<DVec3>, inside: usize) -> Sampled {
    let face = round_face(&body);
    let surface = body.surface(body.face(face).surface);
    let eps = body.scale().eps();
    let mut within_the_face = 0;
    let named: Vec<DVec3> = points
        .into_iter()
        .filter(|&point| {
            let at = surface.parameters(point);
            assert!(
                surface.point(at).distance(point) <= eps,
                "{what}: {point} lies on the surface"
            );
            match body.locate(face, at, eps) {
                Ok(Location::Inside) => {
                    within_the_face += 1;
                    true
                }
                Ok(Location::Boundary) => true,
                Ok(Location::Outside) => false,
                Err(declined) => panic!("{what}: {point} is not located: {declined:?}"),
            }
        })
        .collect();
    assert_eq!(
        within_the_face, inside,
        "{what}: how many of the points where a coordinate turns lie within the face"
    );
    Sampled {
        what,
        body,
        face,
        named,
    }
}

/// Faces covering the whole of their sphere or torus, with the box it spans:
/// a ball, a ring, and an apple, the lobe of a spindle torus on its own side
/// of the axis.
pub(in crate::brep) fn whole_rounds() -> Vec<(&'static str, Body, [DVec3; 2])> {
    vec![
        (
            "a ball",
            turned(&ball(), 360.0),
            between([-5.0, -5.0, -5.0], [5.0, 5.0, 5.0]),
        ),
        (
            "a ring",
            turned(&ring(), 360.0),
            between([-8.0, 1.0, -8.0], [8.0, 5.0, 8.0]),
        ),
        (
            "an apple",
            turned(&apple(), 360.0),
            between([-3.0, -2.0, -3.0], [3.0, 2.0, 3.0]),
        ),
    ]
}

/// The one face of a body lying on a sphere or a torus.
pub(in crate::brep) fn round_face(body: &Body) -> FaceId {
    let rounds: Vec<FaceId> = body
        .face_ids()
        .filter(|&face| {
            matches!(
                body.surface(body.face(face).surface),
                Surface::Sphere(_) | Surface::Torus(_)
            )
        })
        .collect();
    let [face] = rounds[..] else {
        panic!("one face on a sphere or a torus, not {}", rounds.len());
    };
    face
}

/// Points of a face on a sphere or a torus: a grid over its surface's
/// parameters, each kept where the face holds it.
pub(in crate::brep) fn points_of(body: &Body, face: FaceId) -> Vec<DVec3> {
    let surface = body.surface(body.face(face).surface);
    let (low, span) = match surface {
        Surface::Sphere(_) => (DVec2::new(-PI, -FRAC_PI_2), DVec2::new(TAU, PI)),
        Surface::Torus(torus) => (DVec2::new(-PI, torus.seam - PI), DVec2::splat(TAU)),
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
            panic!("a face on a sphere or a torus")
        }
    };
    let eps = body.scale().eps();
    let mut points = Vec::new();
    for step in 0..STEPS * STEPS {
        let grid = DVec2::new(f64::from(step % STEPS), f64::from(step / STEPS)) + 0.5;
        let at = low + span * grid / f64::from(STEPS);
        if body.locate(face, at, eps) == Ok(Location::Inside) {
            points.push(surface.point(at));
        }
    }
    points
}

/// Whether `point` lies in `[low, high]` grown by `room`.
fn within(point: DVec3, [low, high]: [DVec3; 2], room: f64) -> bool {
    point.cmpge(low - room).all() && point.cmple(high + room).all()
}

/// The largest coordinate, in absolute value, of a box's corners.
fn size([low, high]: [DVec3; 2]) -> f64 {
    low.abs().max(high.abs()).max_element()
}

/// Whether two boxes are one to rounding, against the size of the second.
fn same_box(found: [DVec3; 2], expected: [DVec3; 2]) -> bool {
    let tolerance = ROUNDING * size(expected);
    found
        .iter()
        .zip(expected)
        .all(|(found, expected)| (*found - expected).abs().max_element() <= tolerance)
}

#[test]
fn every_point_of_a_face_covering_part_of_its_sphere_or_torus_lies_in_its_box() {
    for round in sampled_rounds() {
        let what = round.what;
        let sampled = points_of(&round.body, round.face);
        assert!(
            sampled.len() >= 100,
            "{what}: {} points of the face sampled",
            sampled.len()
        );
        let found = round.body.boxed(round.face, 0.0);
        let rounding = ROUNDING * round.body.scale().reach();
        for point in sampled.iter().chain(&round.named) {
            assert!(
                within(*point, found, rounding),
                "{what}: {point} of the face lies outside its box {} .. {}",
                found[0],
                found[1]
            );
        }
    }
}

#[test]
fn a_face_covering_part_of_its_sphere_or_torus_is_boxed_by_its_own_extent() {
    let wrong: Vec<String> = partial_rounds()
        .iter()
        .filter_map(|partial| {
            let found = partial.body.boxed(partial.face, 0.0);
            (!same_box(found, partial.own)).then(|| {
                format!(
                    "{}: boxed {} .. {} where its points span {} .. {} and its whole surface {} .. {}",
                    partial.what,
                    found[0],
                    found[1],
                    partial.own[0],
                    partial.own[1],
                    partial.whole[0],
                    partial.whole[1]
                )
            })
        })
        .collect();
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn a_body_whose_rounds_cover_part_of_their_surfaces_is_bounded_and_reaches_by_their_own_extent() {
    let mut wrong = Vec::new();
    for partial in partial_rounds() {
        let what = partial.what;
        let (low, high) = partial.body.bounds().expect("a body has a box");
        if !same_box([low, high], partial.own) {
            wrong.push(format!(
                "{what}: bounded by {low} .. {high} where its points span {} .. {}",
                partial.own[0], partial.own[1]
            ));
        }
        let reach = size(partial.own);
        let found = partial.body.reach();
        if (found - reach).abs() > ROUNDING * reach {
            wrong.push(format!(
                "{what}: reaches {found} where its points reach {reach}"
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn a_face_covering_its_whole_sphere_or_torus_still_spans_the_whole_box() {
    for (what, body, whole) in whole_rounds() {
        let face = round_face(&body);
        let found = body.boxed(face, 0.0);
        assert!(
            same_box(found, whole),
            "{what}: boxed {} .. {} where its surface spans {} .. {}",
            found[0],
            found[1],
            whole[0],
            whole[1]
        );
        let (low, high) = body.bounds().expect("a body has a box");
        assert!(
            same_box([low, high], whole),
            "{what}: bounded by {low} .. {high} where its surface spans {} .. {}",
            whole[0],
            whole[1]
        );
        let reach = size(whole);
        assert!(
            (body.reach() - reach).abs() <= ROUNDING * reach,
            "{what}: reaches {} where its surface reaches {reach}",
            body.reach()
        );
    }
}
