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
//!   `a_chamfer_on_two_edges_whose_flats_would_meet_on_the_wall_one_ends_on_asked_more_than_half_its_width_takes_the_largest_distance_short_of_meeting`;
//!   and, beyond the issue's block, the same where the wall leans from square
//!   to the edge that ends on it, the back upright's faces meeting at a sharp
//!   angle: the step takes the value at which the round square to the top's
//!   front edge, reaching further along the leaning wall than across it, and
//!   the back upright's set-back fill the wall's length —
//!   `a_fillet_and_a_chamfer_on_two_edges_whose_rounds_would_meet_on_a_leaning_wall_one_ends_on_take_the_largest_value_short_of_meeting`
//! - The value is reduced to the largest at which those rounds do not meet
//!   either — across a shared face, two edges that are not parallel:
//!   alternate edges of a hexagonal prism's top, once their set-backs cross;
//!   asked past it, a hair past or well past, the step takes the value at
//!   which the two set-backs touch on the top edge of the wall between them,
//!   a quarter of the side times the square root of three, or a few hairs
//!   under, and lays both, each as alone at that value, neither meeting the
//!   other —
//!   `a_fillet_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_radius_short_of_meeting`,
//!   `a_chamfer_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_distance_short_of_meeting`
//! - Every step #698 lays — no test: held by the suite of #698 in
//!   `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs`, left as it
//!   was but for the two tests that held these meetings declined. The one on
//!   the wall one edge ends on keeps the step it lays short of the meeting,
//!   renamed
//!   `a_fillet_on_two_edges_whose_rounds_would_meet_on_the_wall_one_ends_on_is_laid_as_asked_short_of_it`,
//!   its decline past it now the reduction above; the one on the hexagonal
//!   prism, where they would meet, goes, its check that each edge alone
//!   takes the value asked carried into the second criterion's tests. Beyond
//!   the issue's wording, one step #698 lays that no test of its suite
//!   reaches and that a reduction through an end face could shrink: two
//!   edges whose rounds both end on one wall, side by side across its width
//!   but one at its top and the other at its floor, never meeting on it,
//!   take the value asked —
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

/// How many of the part's hairs under the value at which two rounds touch a
/// reduced step may stop: the kernel keeps no strip between two set-backs
/// shorter than four hairs, which leaves the value two hairs under; ten leave
/// the step room to keep a wider strip, and stand a thousand times under the
/// drawing's tolerance.
const SHORT_BY: f64 = 10.0;

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
/// the value at which two of its rounds touch, nor less than [`SHORT_BY`] of
/// the part's hairs, `hair` long, under it.
fn reduced_to_touching(
    asked: f64,
    taken: f64,
    [expected_asked, touching]: [f64; 2],
    hair: f64,
    what: &str,
) {
    assert!(
        (asked - expected_asked).abs() < PLACE,
        "{what}: reports {asked} asked, where {expected_asked} was"
    );
    let least = touching - SHORT_BY * hair;
    assert!(
        taken <= touching + PLACE && taken >= least,
        "{what}: took {taken}, where the largest value at which the two do not meet is \
         {touching}, or at most {SHORT_BY} hairs under it, {least}"
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
    is_the_round_running_on(body, face, [round, round], what);
}

/// Holds that the face numbered `face` is a round running on past both ends
/// of its edge: every corner of its triangles on `running_on`, which reaches
/// past them, and every ray aimed at its axis over the edge, `over_the_edge`,
/// meeting it first, as [`is_the_round`] holds it of a round that does not
/// run on.
fn is_the_round_running_on(
    body: &Body,
    face: usize,
    [running_on, over_the_edge]: [&Round; 2],
    what: &str,
) {
    drawn_on(body, face, running_on, what);
    met_from_outside(body, face, over_the_edge, what);
}

/// Holds that every corner of the triangles of the face numbered `face` lies
/// on the round.
fn drawn_on(body: &Body, face: usize, round: &Round, what: &str) {
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
}

/// Holds that every ray aimed square at the round's axis from outside, a
/// tenth to nine tenths of the way along it, meets the face numbered `face`
/// first, where it faces away from the axis along the ray.
fn met_from_outside(body: &Body, face: usize, round: &Round, what: &str) {
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
        reduced_to_touching(
            filleted.asked,
            filleted.taken,
            [asked, TOUCHING],
            ONE_HAIR,
            &what,
        );
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
        reduced_to_touching(
            chamfered.asked,
            chamfered.taken,
            [asked, TOUCHING],
            ONE_HAIR,
            &what,
        );
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

/// The side of the hexagonal prism, and its height.
const HEXAGON_SIDE: f64 = 10.0;
const HEXAGON_HEIGHT: f64 = 5.0;

/// The numbers the hexagonal prism's walls answer to, its top being one:
/// its front two, along the bottom of the hexagon; four, two walls round,
/// whose top edge turns a third of a turn from the front's; and three,
/// between them, where both their rounds run on past their ends.
const HEXAGON_FRONT: usize = 2;
const HEXAGON_BETWEEN: usize = 3;
const HEXAGON_ACROSS: usize = 4;

/// One of the kernel's hairs on the hexagonal prism, which reaches
/// [`HEXAGON_SIDE`] from the origin: twenty tolerances, a tolerance a
/// billionth of the reach.
const HEXAGON_HAIR: f64 = 20.0 * 1e-9 * HEXAGON_SIDE;

/// What is asked of the two top edges past where their set-backs cross,
/// which either edge alone gives: under the prism's height.
const HEXAGON_ASKED: f64 = 4.9;

/// Half the hexagon's breadth across its flats: from its middle to its
/// front.
fn hexagon_half() -> f64 {
    HEXAGON_SIDE / 2.0 * 3.0f64.sqrt()
}

/// The radius, or the distance, at which the front's top edge and wall
/// four's set-backs on the top touch, on the top edge of wall three between
/// them. Each set-back is a line on the top the value in from its edge,
/// crossing wall three's top edge, which turns a sixth of a turn from both,
/// the value over the sine of that sixth from the corner it starts at:
/// two over the square root of three of it. The two touch where those
/// reaches fill the edge, a side long, at a quarter of the side times the
/// square root of three — half the distance across the equilateral triangle
/// the two edges' lines close on wall three's.
fn hexagon_touching() -> f64 {
    HEXAGON_SIDE * 3.0f64.sqrt() / 4.0
}

/// A regular hexagon of side [`HEXAGON_SIDE`] about the origin, its front the
/// side along the bottom, raised [`HEXAGON_HEIGHT`]: its walls numbered from
/// the front round, two to seven.
fn hexagonal_prism() -> Body {
    let half = hexagon_half();
    let side = HEXAGON_SIDE;
    raised_with(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(-side / 2.0, -half),
            DVec2::new(side / 2.0, -half),
            DVec2::new(side, 0.0),
            DVec2::new(side / 2.0, half),
            DVec2::new(-side / 2.0, half),
            DVec2::new(-side, 0.0),
        ]),
        HEXAGON_HEIGHT,
    )
}

/// The two edges whose set-backs cross on the top, in the order the step
/// names them: the front's top edge, and wall four's.
fn alternate_top_edges(prism: &Body) -> [EdgeName; 2] {
    [
        edge_between(prism, TOP, HEXAGON_FRONT),
        edge_between(prism, TOP, HEXAGON_ACROSS),
    ]
}

/// A way turned about the vertical by `turn`.
fn turned(way: DVec3, turn: f64) -> DVec3 {
    DVec3::new(
        way.x * turn.cos() - way.y * turn.sin(),
        way.x * turn.sin() + way.y * turn.cos(),
        way.z,
    )
}

/// The round a fillet of `radius` lays on the hexagon's front top edge,
/// turned about the vertical by `turn` — a third of a turn gives wall four's.
/// The first runs on past both its ends to the walls beside it, to where its
/// top meets them, the radius over the square root of three past each; the
/// second is the same round over the edge alone, between its ends.
fn hexagon_round(radius: f64, turn: f64) -> [Round; 2] {
    let half = hexagon_half();
    [
        HEXAGON_SIDE / 2.0 + radius / 3.0f64.sqrt(),
        HEXAGON_SIDE / 2.0,
    ]
    .map(|past| Round {
        start: turned(
            DVec3::new(-past, radius - half, HEXAGON_HEIGHT - radius),
            turn,
        ),
        along: turned(DVec3::X, turn),
        length: 2.0 * past,
        radius,
        facing: [DVec3::Z, turned(-DVec3::Y, turn)],
    })
}

/// The matter a step of `value` takes off along one top edge of the
/// hexagonal prism, of section `section` and of first moment `moment` about
/// the edge's wall: the section along the edge, a side long, and on past
/// both its ends, where the walls beside it lean out, the length growing by
/// two over the square root of three of the way in from the wall.
fn taken_off_one_top_edge(section: f64, moment: f64) -> f64 {
    HEXAGON_SIDE * section + 2.0 / 3.0f64.sqrt() * moment
}

/// What is left of the hexagon's top once both edges' set-backs at `value`
/// cut it: the front's set-back running from wall seven to wall three, wall
/// four's from wall three to wall five, a stretch of wall three's top edge
/// left between them, and the three far corners as they were.
fn hexagon_top(value: f64) -> Vec<DVec3> {
    let (half, reach) = (hexagon_half(), value / 3.0f64.sqrt());
    let side = HEXAGON_SIDE;
    [
        DVec2::new(-side / 2.0 - reach, value - half),
        DVec2::new(side / 2.0 + reach, value - half),
        DVec2::new(side - reach, -value),
        DVec2::new(side / 2.0 - 2.0 * reach, half),
        DVec2::new(-side / 2.0, half),
        DVec2::new(-side, 0.0),
    ]
    .map(|corner| corner.extend(HEXAGON_HEIGHT))
    .to_vec()
}

/// What is left of wall three once both edges' steps at `value` end on it:
/// its foot whole, its two upright edges cut down by the value, and its top
/// edge cut back from either end by two over the square root of three of
/// the value, a stretch left between. Between each cut upright and the top
/// edge, the end of what the step laid trims the wall.
fn hexagon_between(value: f64) -> Vec<DVec3> {
    let (half, reach) = (hexagon_half(), value / 3.0f64.sqrt());
    let (side, height) = (HEXAGON_SIDE, HEXAGON_HEIGHT);
    vec![
        DVec3::new(side / 2.0, -half, 0.0),
        DVec3::new(side, 0.0, 0.0),
        DVec3::new(side, 0.0, height - value),
        DVec3::new(side - reach, -value, height),
        DVec3::new(side / 2.0 + reach, value - half, height),
        DVec3::new(side / 2.0, -half, height - value),
    ]
}

/// The hexagonal prism's matter: six equilateral triangles of side
/// [`HEXAGON_SIDE`], raised [`HEXAGON_HEIGHT`].
fn hexagon_volume() -> f64 {
    3.0 * 3.0f64.sqrt() / 2.0 * HEXAGON_SIDE * HEXAGON_SIDE * HEXAGON_HEIGHT
}

#[test]
fn a_fillet_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_radius_short_of_meeting()
 {
    let prism = hexagonal_prism();
    let names = alternate_top_edges(&prism);
    let laid = [prism.faces_end(), prism.faces_end() + 1];
    let touching = hexagon_touching();

    for asked in [touching + HEXAGON_HAIR, HEXAGON_ASKED] {
        alone_each_takes(
            |name| prism.fillet(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!("the fillet on the hexagon's front top edge or wall four's, asked {asked}"),
        );

        let filleted = prism
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the fillet on the hexagon's front top edge and the top edge two walls round, asked \
             {asked}"
        );
        reduced_to_touching(
            filleted.asked,
            filleted.taken,
            [asked, touching],
            HEXAGON_HAIR,
            &what,
        );
        let radius = filleted.taken;
        let body = &filleted.body;
        let rounds = [0.0, 2.0 * PI / 3.0].map(|turn| hexagon_round(radius, turn));
        for ([running_on, over_the_edge], number) in rounds.iter().zip(laid) {
            is_the_round_running_on(
                body,
                number,
                [running_on, over_the_edge],
                &format!(
                    "{what}: face {number}, its edge rounded off at {radius}, running on past \
                     both its ends"
                ),
            );
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&prism, body, laid, &what);
        corners_are(
            body,
            TOP,
            &hexagon_top(radius),
            &format!("{what}: the top, set back by both rounds and left between them"),
        );
        corners_are_or_on(
            body,
            HEXAGON_BETWEEN,
            &hexagon_between(radius),
            |corner| {
                rounds
                    .iter()
                    .any(|[running_on, _]| running_on.holds(corner))
            },
            &format!("{what}: wall three, trimmed by both rounds' ends and left between them"),
        );
        let section = radius * radius * (1.0 - PI / 4.0);
        let moment = radius.powi(3) * (5.0 / 6.0 - PI / 4.0);
        sound_drawn_within(
            body,
            hexagon_volume() - 2.0 * taken_off_one_top_edge(section, moment),
            DRAWN
                * reach_of(&prism)
                * rounds
                    .iter()
                    .map(|[running_on, _]| running_on.area())
                    .sum::<f64>(),
            &what,
        );
    }
}

/// The flat a chamfer of `distance` lays on the hexagon's front top edge,
/// turned about the vertical by `turn` — a third of a turn gives wall four's:
/// set back by the distance down the wall, between the edge's ends, and in
/// along the top, running on to the walls beside it, the distance over the
/// square root of three past each end; facing halfway between the top and
/// the wall.
fn hexagon_flat(distance: f64, turn: f64) -> ([DVec3; 4], DVec3) {
    let (half, reach) = (hexagon_half(), distance / 3.0f64.sqrt());
    let end = HEXAGON_SIDE / 2.0;
    let top = HEXAGON_HEIGHT;
    (
        [
            DVec3::new(-end, -half, top - distance),
            DVec3::new(end, -half, top - distance),
            DVec3::new(end + reach, distance - half, top),
            DVec3::new(-end - reach, distance - half, top),
        ]
        .map(|corner| turned(corner, turn)),
        turned(DVec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2), turn),
    )
}

#[test]
fn a_chamfer_on_two_alternate_top_edges_of_a_hexagonal_prism_asked_past_where_their_set_backs_cross_takes_the_largest_distance_short_of_meeting()
 {
    let prism = hexagonal_prism();
    let names = alternate_top_edges(&prism);
    let laid = [prism.faces_end(), prism.faces_end() + 1];
    let touching = hexagon_touching();

    for asked in [touching + HEXAGON_HAIR, HEXAGON_ASKED] {
        alone_each_takes(
            |name| prism.chamfer(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!("the chamfer on the hexagon's front top edge or wall four's, asked {asked}"),
        );

        let chamfered = prism
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the chamfer on the hexagon's front top edge and the top edge two walls round, asked \
             {asked}"
        );
        reduced_to_touching(
            chamfered.asked,
            chamfered.taken,
            [asked, touching],
            HEXAGON_HAIR,
            &what,
        );
        let distance = chamfered.taken;
        let body = &chamfered.body;
        for (turn, number) in [0.0, 2.0 * PI / 3.0].into_iter().zip(laid) {
            let (corners, facing) = hexagon_flat(distance, turn);
            let which = format!("{what}: face {number}, its edge cut off at {distance}");
            faces_toward(body, number, facing, &which);
            corners_are(body, number, &corners, &which);
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&prism, body, laid, &what);
        corners_are(
            body,
            TOP,
            &hexagon_top(distance),
            &format!("{what}: the top, set back by both flats and left between them"),
        );
        corners_are(
            body,
            HEXAGON_BETWEEN,
            &hexagon_between(distance),
            &format!("{what}: wall three, trimmed by both flats' ends and left between them"),
        );
        let section = distance * distance / 2.0;
        let moment = distance.powi(3) / 6.0;
        let promised = hexagon_volume() - 2.0 * taken_off_one_top_edge(section, moment);
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

/// How much further along X the leaning block's right wall stands at the
/// back than at the front.
const LEANING_BY: f64 = 5.0;

/// What is asked of both edges of the leaning block past where they would
/// meet, which either edge alone gives.
const LEANING_ASKED: f64 = 6.0;

/// One of the kernel's hairs on the leaning block, which reaches
/// [`LENGTH`] and [`LEANING_BY`] from the origin: twenty tolerances, a
/// tolerance a billionth of the reach.
const LEANING_HAIR: f64 = 20.0 * 1e-9 * (LENGTH + LEANING_BY);

/// The block [`LENGTH`] long at its front, [`BREADTH`] across and
/// [`HEIGHT`] high, its right wall leaning out from square to the front, to
/// [`LEANING_BY`] further along at the back: its faces numbered as the
/// block's.
fn leaning_block() -> Body {
    raised_with(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(LENGTH, 0.0),
            DVec2::new(LENGTH + LEANING_BY, BREADTH),
            DVec2::new(0.0, BREADTH),
        ]),
        HEIGHT,
    )
}

/// The leaning wall's length across, the way along it from the front to
/// the back, and the way it faces.
fn leaning_wall() -> (f64, DVec3, DVec3) {
    let length = LEANING_BY.hypot(BREADTH);
    (
        length,
        DVec3::new(LEANING_BY, BREADTH, 0.0) / length,
        DVec3::new(BREADTH, -LEANING_BY, 0.0) / length,
    )
}

/// How far along the leaning wall a value of one sets back the back edge,
/// upright, whose faces meet at the angle whose cosine is the lean over the
/// wall's length: a fillet's radius over the tangent of half that angle, a
/// chamfer's distance as it is.
fn leaning_back_set_back(fillet: bool) -> f64 {
    let (length, _, _) = leaning_wall();
    if fillet {
        (length + LEANING_BY) / BREADTH
    } else {
        1.0
    }
}

/// The value at which the top's front edge's round, or flat, and the back
/// upright's touch at the top of the leaning wall. The first, set back the
/// value along the top square to its edge, reaches along the wall the value
/// times its length over the block's breadth; the second sets the wall back
/// from the back by the value times [`leaning_back_set_back`]. The two fill
/// the wall's length at the value this gives.
fn leaning_touching(fillet: bool) -> f64 {
    let (length, _, _) = leaning_wall();
    length / (length / BREADTH + leaning_back_set_back(fillet))
}

/// What a step at `value` on the top's front edge and the back upright
/// leaves of the leaning wall and of the top, the back upright set back
/// `back` along both its faces: the wall set back from the back and trimmed
/// at its top front corner, the top set back from the front and trimmed at
/// its back right corner, a strip of each left between the two cuts.
fn leaning_wall_and_top(value: f64, back: f64) -> [(usize, Vec<DVec3>); 2] {
    let (length, along, _) = leaning_wall();
    let foot = DVec3::new(LENGTH, 0.0, 0.0);
    let cut = foot + along * (length - back);
    let up = DVec3::Z * HEIGHT;
    let reached = DVec3::new(LENGTH + value * LEANING_BY / BREADTH, value, HEIGHT);
    [
        (
            RIGHT,
            vec![
                foot,
                cut,
                cut + up,
                reached,
                foot + DVec3::Z * (HEIGHT - value),
            ],
        ),
        (
            TOP,
            vec![
                DVec3::new(0.0, value, HEIGHT),
                reached,
                cut + up,
                DVec3::new(LENGTH + LEANING_BY - back, BREADTH, HEIGHT),
                DVec3::new(0.0, BREADTH, HEIGHT),
            ],
        ),
    ]
}

/// The leaning block's matter: its outline, a trapezoid, raised
/// [`HEIGHT`].
fn leaning_volume() -> f64 {
    (LENGTH + LEANING_BY / 2.0) * BREADTH * HEIGHT
}

/// The matter a step takes off along the top's front edge of the leaning
/// block, of section `section` and of first moment `moment` about the front:
/// the section along the front, and on past its end, where the leaning wall
/// stands further along by the lean over the breadth of the way in.
fn leaning_taken_off_the_top_front(section: f64, moment: f64) -> f64 {
    LENGTH * section + LEANING_BY / BREADTH * moment
}

#[test]
fn a_fillet_and_a_chamfer_on_two_edges_whose_rounds_would_meet_on_a_leaning_wall_one_ends_on_take_the_largest_value_short_of_meeting()
 {
    let block = leaning_block();
    let names = top_front_and_right_back(&block);
    let laid = [block.faces_end(), block.faces_end() + 1];
    let (length, _, facing_out) = leaning_wall();
    let angle = (LEANING_BY / length).acos();

    let touching = leaning_touching(true);
    for asked in [touching + LEANING_HAIR, LEANING_ASKED] {
        alone_each_takes(
            |name| block.fillet(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!(
                "the fillet on the leaning block's top front edge or back upright, asked {asked}"
            ),
        );

        let filleted = block
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("the fillet, asked {asked}: declined, {declined:?}"));

        let what = format!(
            "the fillet on the leaning block's top front edge and back upright, asked {asked}"
        );
        reduced_to_touching(
            filleted.asked,
            filleted.taken,
            [asked, touching],
            LEANING_HAIR,
            &what,
        );
        let radius = filleted.taken;
        let back = radius * leaning_back_set_back(true);
        let body = &filleted.body;
        let along_the_top = |length| Round {
            start: DVec3::new(0.0, radius, HEIGHT - radius),
            along: DVec3::X,
            length,
            radius,
            facing: [DVec3::Z, -DVec3::Y],
        };
        let running_on = along_the_top(LENGTH + radius * LEANING_BY / BREADTH);
        let upright = Round {
            start: DVec3::new(LENGTH + LEANING_BY - back, BREADTH - radius, 0.0),
            along: DVec3::Z,
            length: HEIGHT,
            radius,
            facing: [facing_out, DVec3::Y],
        };
        is_the_round_running_on(
            body,
            laid[0],
            [&running_on, &along_the_top(LENGTH)],
            &format!(
                "{what}: face {}, the top's front edge rounded off at {radius}, running on to \
                 the leaning wall",
                laid[0]
            ),
        );
        is_the_round(
            body,
            laid[1],
            &upright,
            &format!(
                "{what}: face {}, the back upright rounded off at {radius}",
                laid[1]
            ),
        );
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&block, body, laid, &what);
        for (face, corners) in leaning_wall_and_top(radius, back) {
            corners_are_or_on(
                body,
                face,
                &corners,
                |corner| running_on.holds(corner) || upright.holds(corner),
                &format!("{what}: face {face}, cut by both rounds and left between them"),
            );
        }
        let top_front = leaning_taken_off_the_top_front(
            radius * radius * (1.0 - PI / 4.0),
            radius.powi(3) * (5.0 / 6.0 - PI / 4.0),
        );
        let upright_off = HEIGHT * radius * radius * (back / radius - (PI - angle) / 2.0);
        sound_drawn_within(
            body,
            leaning_volume() - top_front - upright_off,
            DRAWN * reach_of(&block) * (running_on.area() + upright.area()),
            &what,
        );
    }

    let touching = leaning_touching(false);
    for asked in [touching + LEANING_HAIR, LEANING_ASKED] {
        alone_each_takes(
            |name| block.chamfer(name, asked).map(|single| single.taken),
            &names,
            asked,
            &format!(
                "the chamfer on the leaning block's top front edge or back upright, asked {asked}"
            ),
        );

        let chamfered = block
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| {
                panic!("the chamfer, asked {asked}: declined, {declined:?}")
            });

        let what = format!(
            "the chamfer on the leaning block's top front edge and back upright, asked {asked}"
        );
        reduced_to_touching(
            chamfered.asked,
            chamfered.taken,
            [asked, touching],
            LEANING_HAIR,
            &what,
        );
        let distance = chamfered.taken;
        let body = &chamfered.body;
        let [(_, wall), _] = leaning_wall_and_top(distance, distance);
        let flats = [
            (
                vec![
                    DVec3::new(0.0, 0.0, HEIGHT - distance),
                    DVec3::new(LENGTH, 0.0, HEIGHT - distance),
                    wall[3],
                    DVec3::new(0.0, distance, HEIGHT),
                ],
                DVec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            ),
            (
                vec![
                    wall[1],
                    wall[2],
                    DVec3::new(LENGTH + LEANING_BY - distance, BREADTH, HEIGHT),
                    DVec3::new(LENGTH + LEANING_BY - distance, BREADTH, 0.0),
                ],
                (facing_out + DVec3::Y).normalize(),
            ),
        ];
        for ((corners, facing), number) in flats.iter().zip(laid) {
            let which = format!("{what}: face {number}, its edge cut off at {distance}");
            faces_toward(body, number, *facing, &which);
            corners_are(body, number, corners, &which);
        }
        do_not_meet(body, laid, &what);
        kept_every_face_and_laid(&block, body, laid, &what);
        for (face, corners) in leaning_wall_and_top(distance, distance) {
            corners_are(
                body,
                face,
                &corners,
                &format!("{what}: face {face}, cut by both flats and left between them"),
            );
        }
        let top_front =
            leaning_taken_off_the_top_front(distance * distance / 2.0, distance.powi(3) / 6.0);
        let upright_off = HEIGHT * distance * distance * angle.sin() / 2.0;
        let promised = leaning_volume() - top_front - upright_off;
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}
