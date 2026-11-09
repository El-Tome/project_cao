//! Two rounds of one step on several edges can meet otherwise than across a
//! side face their edges share: through an end face, where one edge ends on a
//! side face of the other, or across a shared face, where their edges are not
//! parallel. There the step's value is reduced to the largest at which they do
//! not meet, as #698 reduces it for two edges facing each other across a side
//! face, rather than declined. An operation of the kernel on [`Body`], as the
//! step on several edges of #698 is.
//!
//! Closes #792.
//! - The value is reduced to the largest at which those rounds do not meet
//!   either — through an end face: on a block, the top's front edge ends on
//!   the right wall, which is a side face of the right wall's back vertical
//!   edge; asked more than half the wall's width, a hair more or well more,
//!   the two rounds would meet near that corner, and the step takes half, or
//!   a few hairs under, and lays both, each as alone at that value, neither
//!   meeting the other —
//!   `a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_radius_short_of_meeting`,
//!   `a_chamfer_on_two_edges_whose_flats_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_distance_short_of_meeting`
//! - The value is reduced to the largest at which those rounds do not meet
//!   either — across a shared face, two edges that are not parallel:
//!   alternate edges of a hexagonal prism's top, once their set-backs cross —
//!   `a_fillet_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_radius_short_of_meeting`,
//!   `a_chamfer_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_distance_short_of_meeting`
//! - Every step #698 lays — no test: held by the suite of #698 in
//!   `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs`, left as it
//!   was but for the two tests that held these meetings declined. The one on
//!   the wall one edge ends on keeps the step it lays short of the meeting,
//!   renamed
//!   `a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_is_laid_as_asked_short_of_it`,
//!   its decline past it now the reduction above; the one on the hexagonal
//!   prism goes the same way with the second criterion. Beyond the
//!   issue's wording, one step #698 lays that no test of its suite reaches
//!   and that a reduction through an end face could shrink: two edges whose
//!   rounds both end on one wall, side by side across its width but one at
//!   its top and the other at its floor, never meeting on it, take the value
//!   asked —
//!   `a_fillet_and_a_chamfer_on_two_edges_whose_rounds_end_on_one_wall_without_meeting_on_it_take_the_value_asked`

use std::collections::BTreeSet;
use std::f64::consts::{FRAC_1_SQRT_2, PI};

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane
/// or on a round: each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised block's faces answer to: its floor nought, its top
/// one, and its sides from the front round to the left, two to five.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const BACK: usize = 4;

/// The block the end face is met on: 30 along X, 10 across, 20 high.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;
const BLOCK_VOLUME: f64 = LENGTH * BREADTH * HEIGHT;

/// The radius, or the distance, at which the round of the block's top front
/// edge and that of its right wall's back edge touch, at the top of the right
/// wall: the first reaches its value back along that wall from the front, the
/// second its value forward from the back, and the wall is [`BREADTH`] wide.
const TOUCHING: f64 = BREADTH / 2.0;

/// What is asked of both edges, well more than [`TOUCHING`], which either
/// edge alone gives.
const MORE_THAN_HALF: f64 = 8.0;

/// One of the kernel's hairs on the block [`LENGTH`] long: twenty
/// tolerances, a tolerance a billionth of the reach.
const ONE_HAIR: f64 = 20.0 * 1e-9 * LENGTH;

/// How far under [`TOUCHING`] a reduced step may stop: the kernel keeps no
/// strip between two set-backs shorter than four hairs, which leaves the
/// value two hairs under; ten leave the step room to keep a wider strip, and
/// stand a thousand times under the drawing's tolerance.
const SHORT_OF_TOUCHING: f64 = 10.0 * ONE_HAIR;

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn profile(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// The part with an outline drawn on the ground raised `height` and joined
/// to it as `cao_part` joins a step: its floor numbered first, past the
/// part's numbers, then its top, then its walls, one for each run of the
/// outline.
fn raised_with(part: &Body, outline: Contour, height: f64) -> Body {
    let tool = part
        .tool_raised(&profile(outline), ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
}

/// The block [`LENGTH`] by [`BREADTH`] by [`HEIGHT`] standing on the
/// ground's origin, its floor numbered nought, its top one, and its sides two
/// to five from the front round to the left.
fn block() -> Body {
    raised_with(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(LENGTH, BREADTH)),
        HEIGHT,
    )
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| lies_between(name, first, second))
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// Whether a name is that of an edge between the faces numbered `first` and
/// `second`, whichever it lists first.
fn lies_between(name: &EdgeName, first: usize, second: usize) -> bool {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    name.between == sides || name.between == [other, one]
}

/// The two edges whose rounds would meet through an end face, in the order
/// the step names them: the top's front edge, which ends on the right wall,
/// and the right wall's back edge, upright, of which that wall is a side.
fn top_front_and_right_back(block: &Body) -> [EdgeName; 2] {
    [
        edge_between(block, TOP, FRONT),
        edge_between(block, RIGHT, BACK),
    ]
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// Holds that a step on two edges kept every face of the part it was given
/// and laid one face more for each edge, numbered `laid`, the next step
/// numbering its own past both.
fn kept_every_face_and_laid(part: &Body, body: &Body, laid: [usize; 2], what: &str) {
    let mut kept_and_laid = numbers(part);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block, and one face laid for each edge, {laid:?}"
    );
    assert_eq!(
        body.faces_end(),
        laid[1] + 1,
        "{what}: the next step numbers its faces past both laid"
    );
}

/// Holds that the faces numbered `first` and `second` share no edge: what one
/// edge's step laid does not meet what the other's did.
fn do_not_meet(body: &Body, [first, second]: [usize; 2], what: &str) {
    let between: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| lies_between(name, first, second))
        .collect();
    assert!(
        between.is_empty(),
        "{what}: faces {first} and {second} meet, by {between:?}"
    );
}

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// Holds that a step reports the value it was asked, and took no more than
/// the value at which two of its rounds touch, nor less than
/// [`SHORT_OF_TOUCHING`] under it.
fn reduced_to_touching(asked: f64, taken: f64, [expected_asked, touching]: [f64; 2], what: &str) {
    assert!(
        (asked - expected_asked).abs() < PLACE,
        "{what}: reports {asked} asked, where {expected_asked} was"
    );
    assert!(
        taken <= touching + PLACE && taken >= touching - SHORT_OF_TOUCHING,
        "{what}: took {taken}, where the largest value at which the two do not meet is \
         {touching}, or at most {SHORT_OF_TOUCHING} under it"
    );
}

/// Holds that each of the edges `names` alone takes `asked` whole, which is
/// what makes the limit the two edges' together.
fn alone_each_takes(
    step: impl Fn(&EdgeName) -> Result<f64, Declined>,
    names: &[EdgeName],
    asked: f64,
    what: &str,
) {
    for name in names {
        let taken = step(name)
            .unwrap_or_else(|declined| panic!("{what}, one edge alone: declined, {declined:?}"));
        assert!(
            (taken - asked).abs() < PLACE,
            "{what}: one edge alone takes {taken}, where it was asked {asked}"
        );
    }
}

/// Holds that a face's corners are the points expected, each of them a
/// corner, and every other corner on the curve `on` says it is on.
fn corners_are_or_on(
    body: &Body,
    face: usize,
    expected: &[DVec3],
    on: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            on(*corner) || expected.iter().any(|point| point.distance(*corner) < PLACE),
            "{what}: a corner at {corner}, where the corners are {expected:?}"
        );
    }
    for point in expected {
        assert!(
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    corners_are_or_on(body, face, expected, |_| false, what);
}

/// Holds that the face numbered `face` is flat and faces the way `facing`
/// points.
fn faces_toward(body: &Body, face: usize, facing: DVec3, what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    assert!(
        plane.normal.distance(facing) < PLACE,
        "{what}: face {face} faces {}, not {facing}",
        plane.normal
    );
}

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, the matter it holds what
/// arithmetic promised, and the matter its triangles enclose within `drawn`
/// of it — the drawing's tolerance over its rounds, where it has any.
fn sound_drawn_within(body: &Body, promised: f64, drawn: f64, what: &str) {
    assert!(
        body.is_exact(),
        "{what}: the exact kernel computed the body"
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "{what}: the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );
    let enclosed = enclosed(&triangles);
    assert!(
        (enclosed - promised).abs() <= drawn,
        "{what}: its triangles enclose {enclosed}, where arithmetic promised {promised}"
    );
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// The round a fillet is expected to lay: a cylinder of `radius` about the
/// line from `start` along `along` for `length`, turning about it from facing
/// the way the edge's first face does to facing the way its second does.
/// Facing each face's way where it meets it is what tangent to both comes to.
struct Round {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Round {
    /// The point of the axis `share` of the way along it.
    fn axis_at(&self, share: f64) -> DVec3 {
        self.start + self.along * self.length * share
    }

    /// The way the round faces `share` of the way round it, from its first
    /// face's way to its second's.
    fn facing_at(&self, share: f64) -> DVec3 {
        let [first, second] = self.facing;
        let turn = first.dot(second).acos() * share;
        let across = (second - first * first.dot(second)).normalize();
        first * turn.cos() + across * turn.sin()
    }

    /// Whether a point lies on the round: on its cylinder, between its two
    /// ends, and between its two sides.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let across = off - self.along * along;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && across.dot(first) >= least - PLACE
            && across.dot(second) >= least - PLACE
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that the face numbered `face` is the round expected: every corner
/// of its triangles on it, and every ray aimed square at its axis from
/// outside, a tenth to nine tenths of the way along it, meeting it first,
/// where it faces away from the axis along the ray.
fn is_the_round(body: &Body, face: usize, round: &Round, what: &str) {
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            round.holds(corner),
            "{what}: the round is drawn through {corner}, off the cylinder of {} about the line \
             from {} along {}",
            round.radius,
            round.start,
            round.along
        );
    }
    for along in [0.11, 0.47, 0.83] {
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = round.facing_at(share);
            let from = round.axis_at(along) + facing * (round.radius + 10.0);
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} toward the axis meets nothing")
            });
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} toward the axis meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 away from its axis",
                hit.normal
            );
        }
    }
}

/// The rounds a fillet of `radius` lays on the top's front edge, along the
/// whole length of the block, and on the right wall's back edge, up its
/// whole height: each its axis the radius in from both its faces.
fn top_front_and_right_back_rounds(radius: f64) -> [Round; 2] {
    [
        Round {
            start: DVec3::new(0.0, radius, HEIGHT - radius),
            along: DVec3::X,
            length: LENGTH,
            radius,
            facing: [DVec3::Z, -DVec3::Y],
        },
        Round {
            start: DVec3::new(LENGTH - radius, BREADTH - radius, 0.0),
            along: DVec3::Z,
            length: HEIGHT,
            radius,
            facing: [DVec3::X, DVec3::Y],
        },
    ]
}

/// What a step on the top's front edge and the right wall's back edge leaves
/// of the right wall, at `value`, and of the top: the right wall set back by
/// the value from the back and trimmed at its top front corner, the top set
/// back by it from the front and trimmed at its back right corner, a strip of
/// each left between the two cuts where they would otherwise meet. The
/// corners where the top front edge's cut trims the wall, and where the back
/// edge's trims the top, are the ends of what trims them.
fn right_wall_and_top(value: f64) -> [(usize, Vec<DVec3>); 2] {
    let x = LENGTH;
    [
        (
            RIGHT,
            vec![
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, BREADTH - value, 0.0),
                DVec3::new(x, BREADTH - value, HEIGHT),
                DVec3::new(x, value, HEIGHT),
                DVec3::new(x, 0.0, HEIGHT - value),
            ],
        ),
        (
            TOP,
            vec![
                DVec3::new(0.0, value, HEIGHT),
                DVec3::new(x, value, HEIGHT),
                DVec3::new(x, BREADTH - value, HEIGHT),
                DVec3::new(x - value, BREADTH, HEIGHT),
                DVec3::new(0.0, BREADTH, HEIGHT),
            ],
        ),
    ]
}

#[test]
fn a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_radius_short_of_meeting()
 {
    let block = block();
    let names = top_front_and_right_back(&block);
    let laid = [block.faces_end(), block.faces_end() + 1];

    for asked in [TOUCHING + ONE_HAIR, MORE_THAN_HALF] {
        alone_each_takes(
            |name| block.fillet(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!(
                "the fillet on the top's front edge or the right wall's back edge, asked {asked}"
            ),
        );

        let filleted = block
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the fillet on the top's front edge and the right wall's back edge, asked {asked}"
        );
        reduced_to_touching(filleted.asked, filleted.taken, [asked, TOUCHING], &what);
        let radius = filleted.taken;
        let body = &filleted.body;
        let rounds = top_front_and_right_back_rounds(radius);
        for ((round, number), edge) in rounds
            .iter()
            .zip(laid)
            .zip(["top's front", "right wall's back"])
        {
            is_the_round(
                body,
                number,
                round,
                &format!("{what}: face {number}, the {edge} edge rounded off at {radius}"),
            );
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&block, body, laid, &what);
        for (face, corners) in right_wall_and_top(radius) {
            corners_are_or_on(
                body,
                face,
                &corners,
                |corner| rounds.iter().any(|round| round.holds(corner)),
                &format!("{what}: face {face}, cut by both rounds and left between them"),
            );
        }
        let quarter = radius * radius * (1.0 - PI / 4.0);
        sound_drawn_within(
            body,
            BLOCK_VOLUME - quarter * (LENGTH + HEIGHT),
            DRAWN * reach_of(&block) * rounds.iter().map(Round::area).sum::<f64>(),
            &what,
        );
    }
}

/// The flats a chamfer of `distance` lays on the top's front edge, along the
/// whole length of the block, and on the right wall's back edge, up its
/// whole height: each set back by the distance along both its faces, its
/// corners where it meets the walls at its ends, and facing halfway between
/// its two faces.
fn top_front_and_right_back_flats(distance: f64) -> [([DVec3; 4], DVec3); 2] {
    let (x, y, z) = (LENGTH, BREADTH, HEIGHT);
    [
        (
            [
                DVec3::new(0.0, 0.0, z - distance),
                DVec3::new(x, 0.0, z - distance),
                DVec3::new(x, distance, z),
                DVec3::new(0.0, distance, z),
            ],
            DVec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        ),
        (
            [
                DVec3::new(x, y - distance, 0.0),
                DVec3::new(x, y - distance, z),
                DVec3::new(x - distance, y, z),
                DVec3::new(x - distance, y, 0.0),
            ],
            DVec3::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0),
        ),
    ]
}

#[test]
fn a_chamfer_on_two_edges_whose_flats_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_distance_short_of_meeting()
 {
    let block = block();
    let names = top_front_and_right_back(&block);
    let laid = [block.faces_end(), block.faces_end() + 1];

    for asked in [TOUCHING + ONE_HAIR, MORE_THAN_HALF] {
        alone_each_takes(
            |name| block.chamfer(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!(
                "the chamfer on the top's front edge or the right wall's back edge, asked {asked}"
            ),
        );

        let chamfered = block
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the chamfer on the top's front edge and the right wall's back edge, asked {asked}"
        );
        reduced_to_touching(chamfered.asked, chamfered.taken, [asked, TOUCHING], &what);
        let distance = chamfered.taken;
        let body = &chamfered.body;
        for (((corners, facing), number), edge) in top_front_and_right_back_flats(distance)
            .iter()
            .zip(laid)
            .zip(["top's front", "right wall's back"])
        {
            let which = format!("{what}: face {number}, the {edge} edge cut off at {distance}");
            faces_toward(body, number, *facing, &which);
            corners_are(body, number, corners, &which);
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&block, body, laid, &what);
        for (face, corners) in right_wall_and_top(distance) {
            corners_are(
                body,
                face,
                &corners,
                &format!("{what}: face {face}, cut by both flats and left between them"),
            );
        }
        let promised = BLOCK_VOLUME - distance * distance / 2.0 * (LENGTH + HEIGHT);
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_two_edges_whose_rounds_end_on_one_wall_without_meeting_on_it_take_the_value_asked()
 {
    let block = block();
    let names = [
        edge_between(&block, TOP, FRONT),
        edge_between(&block, FLOOR, BACK),
    ];
    let laid = [block.faces_end(), block.faces_end() + 1];
    let value = MORE_THAN_HALF;

    let filleted = block
        .fillet_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
    let chamfered = block
        .chamfer_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let what = format!(
        "the fillet on the top's front edge and the floor's back edge, asked {value}: their \
         rounds' ends overlap across the right wall's width, one at its top and the other at its \
         floor"
    );
    taken_as_asked(filleted.asked, filleted.taken, value, &what);
    let body = &filleted.body;
    let rounds = [
        Round {
            start: DVec3::new(0.0, value, HEIGHT - value),
            along: DVec3::X,
            length: LENGTH,
            radius: value,
            facing: [DVec3::Z, -DVec3::Y],
        },
        Round {
            start: DVec3::new(0.0, BREADTH - value, value),
            along: DVec3::X,
            length: LENGTH,
            radius: value,
            facing: [-DVec3::Z, DVec3::Y],
        },
    ];
    for (round, number) in rounds.iter().zip(laid) {
        is_the_round(
            body,
            number,
            round,
            &format!("{what}: face {number}, its edge rounded off as asked"),
        );
    }
    do_not_meet(body, laid, &what);
    kept_every_face_and_laid(&block, body, laid, &what);
    sound_drawn_within(
        body,
        BLOCK_VOLUME - 2.0 * value * value * (1.0 - PI / 4.0) * LENGTH,
        DRAWN * reach_of(&block) * rounds.iter().map(Round::area).sum::<f64>(),
        &what,
    );

    let what = format!(
        "the chamfer on the top's front edge and the floor's back edge, asked {value}: their \
         flats' ends overlap across the right wall's width, one at its top and the other at its \
         floor"
    );
    taken_as_asked(chamfered.asked, chamfered.taken, value, &what);
    let body = &chamfered.body;
    do_not_meet(body, laid, &what);
    kept_every_face_and_laid(&block, body, laid, &what);
    let promised = BLOCK_VOLUME - value * value * LENGTH;
    sound_drawn_within(body, promised, EXACT * promised, &what);
}
