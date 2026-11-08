//! A chamfer and a fillet take several straight edges of an exact body in one
//! step, at one value: each edge of the list cut off, or filled in where its
//! faces meet inwards, as the step of that one edge would (#696, #697, #778),
//! and the value reduced once for the whole step, to the largest at which
//! every round or flat fits with the others laid. Two rounds whose set-backs
//! meet across the face they share are one face, a full round: their edges
//! are parallel, so the two share one axis — edges that are not parallel are
//! out of this issue's reach. An operation of the kernel on [`Body`]; the
//! step of the history that calls it comes with the application, in #701.
//!
//! Closes #698.
//! - A fillet step given the names of several straight edges of a block that
//!   share no vertex, and one radius, leaves a closed body where each edge is
//!   rounded as the fillet of that one edge alone would round it. The body
//!   passes the soundness checks. The same holds for a chamfer and one
//!   distance —
//!   `a_fillet_on_the_four_long_edges_of_a_block_rounds_each_as_its_own_fillet_alone_would`,
//!   `a_chamfer_on_the_four_long_edges_of_a_block_cuts_each_as_its_own_chamfer_alone_would`
//! - Each round, or flat, is numbered on its own, in the order the step names
//!   its edge, from where the steps' numbers ended —
//!   `each_round_and_each_flat_of_a_step_on_several_edges_is_numbered_in_the_order_the_step_names_its_edge_from_where_the_steps_numbers_ended`
//! - The value is reduced once for the whole step, to the largest at which
//!   every round fits with the others laid. The step reports the value asked
//!   and the one value taken —
//!   `a_fillet_and_a_chamfer_on_several_edges_asked_more_than_one_of_them_gives_take_the_one_value_every_edge_fits_and_report_both`;
//!   a value that fits is taken as asked — the two tests of the first
//!   criterion, which ask a value every edge gives and hold that it is the one
//!   taken, and
//!   `a_fillet_and_a_chamfer_on_the_two_edges_either_side_of_a_face_take_a_value_that_fits_as_asked`
//! - On a block 10 wide, rounding the two edges on either side of that face,
//!   asked more than 5, takes 5, where either alone would take more: the face
//!   is gone, and the two rounds, of one axis and one radius, are one
//!   half-cylinder tangent to both sides — a **full round** — answering to
//!   both edges' numbers. The body passes the soundness checks. A chamfer
//!   takes 5 too, and its two flats meet in a ridge —
//!   `a_fillet_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_in_one_full_round`,
//!   `a_chamfer_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_ridge`
//! - A list mixing an edge whose faces meet outwards and one whose faces meet
//!   inwards cuts the first off and fills the second in, at one value —
//!   `a_fillet_and_a_chamfer_on_an_outward_edge_and_an_inward_one_cut_the_first_off_and_fill_the_second_in_at_one_value`
//! - Two edges of the list that share a vertex decline the step
//!   `Unsupported`, and nothing of it is laid —
//!   `a_fillet_and_a_chamfer_on_two_edges_sharing_a_vertex_are_declined_as_unsupported`;
//!   nothing laid is what a declined step is: the step hands back its reason
//!   and no body, and the part it was given is left as it was
//! - An edge of the list its name no longer finds declines the whole step
//!   with that edge's reason (`Lost`), as for one edge —
//!   `a_fillet_and_a_chamfer_on_a_list_holding_an_edge_its_name_no_longer_finds_are_declined_with_that_edge_s_reason`.
//!   Two names of the list finding the same edge decline it `Unsupported` —
//!   `a_fillet_and_a_chamfer_on_two_names_finding_the_same_edge_are_declined_as_unsupported`
//! - After a change of size upstream, the step rounds the same edges, found by
//!   their names —
//!   `a_fillet_and_a_chamfer_on_several_edges_take_the_edges_their_names_find_again_after_a_change_of_size`
//! - A list of one edge gives the body and the value the single-edge fillet
//!   and chamfer give —
//!   `a_fillet_and_a_chamfer_on_a_list_of_one_edge_give_the_body_and_the_value_the_step_of_that_edge_gives`;
//!   their tests pass unchanged: `a_fillet_rounds_one_straight_edge.rs`,
//!   `a_chamfer_cuts_one_straight_edge.rs` and
//!   `a_chamfer_and_a_fillet_fill_a_concave_edge.rs`, left as they were

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, EdgeName, Loop};
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
const LEFT: usize = 5;

/// The block the first criterion is held on: 30 along X, 10 across, 20 high.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;

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

/// A block of a length, a breadth and a height standing on the ground's
/// origin, joined as `cao_part` builds a step: its floor numbered nought, its
/// top one, and its sides two to five from the front round to the left.
fn block([length, breadth, height]: [f64; 3]) -> Body {
    joined_to(
        &Body::default(),
        [DVec2::ZERO, DVec2::new(length, breadth)],
        height,
    )
}

/// The part with a block standing on the ground between the corners `from`
/// and `to`, `height` high, joined to it as `cao_part` joins a step: the
/// block's faces numbered past the part's, its floor first, then its top,
/// then its sides from the front round to the left.
fn joined_to(part: &Body, [from, to]: [DVec2; 2], height: f64) -> Body {
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(from, to)),
            ground(),
            DVec3::Z * height,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
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

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// The numbers of the faces a step made: those the body answers to that the
/// part it was given did not.
fn faces_made(part: &Body, body: &Body) -> Vec<usize> {
    numbers(body).difference(&numbers(part)).copied().collect()
}

/// Holds that a step of one edge left every face of the part it was given and
/// made one face more, and gives back the number that face answers to.
fn made_one(part: &Body, body: &Body, what: &str) -> usize {
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}, alone: a face of the part went: {:?}",
        numbers(body)
    );
    let made = faces_made(part, body);
    assert_eq!(made.len(), 1, "{what}, alone: one face more: {made:?}");
    made[0]
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    corners_are_or_on(body, face, expected, |_| false, what);
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

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// Holds that a body took away from the part it was given what each of the
/// steps of one edge `alone` takes away from it, all of it and no more.
fn takes_what_each_alone_takes(part: &Body, body: &Body, alone: &[Body], what: &str) {
    let each: f64 = alone
        .iter()
        .map(|single| part.volume() - single.volume())
        .sum();
    let taken = part.volume() - body.volume();
    assert!(
        (taken - each).abs() <= EXACT * part.volume(),
        "{what}: the step took {taken} away, where the steps of each edge alone take {each}"
    );
}

/// The sides of the faces the face numbered `face` meets, one for each edge
/// it has, in order.
fn sides_met(body: &Body, face: usize) -> Vec<Vec<u32>> {
    let own = vec![face as u32];
    let mut met: Vec<Vec<u32>> = body
        .edge_names()
        .into_iter()
        .filter_map(|name| {
            let [first, second] = name.between;
            if first == own {
                Some(second)
            } else if second == own {
                Some(first)
            } else {
                None
            }
        })
        .collect();
    met.sort();
    met
}

/// The sides a face is expected to meet, one edge each, in the order
/// [`sides_met`] lists them.
fn sides(faces: &[usize]) -> Vec<Vec<u32>> {
    let sorted: BTreeSet<usize> = faces.iter().copied().collect();
    sorted.into_iter().map(|face| vec![face as u32]).collect()
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// One of the four edges of the block that run along its length, from its
/// left wall to its right: the two faces it lies between, the corner of the
/// left wall it starts at, and the ways the two faces face.
struct LongEdge {
    faces: [usize; 2],
    start: DVec3,
    facing: [DVec3; 2],
}

/// The block's four long edges, in the order the steps name them — not the
/// block's own: the top's front, the floor's back, the top's back and the
/// floor's front. No two share a vertex.
fn long_edges() -> [LongEdge; 4] {
    [
        LongEdge {
            faces: [TOP, FRONT],
            start: DVec3::new(0.0, 0.0, HEIGHT),
            facing: [DVec3::Z, -DVec3::Y],
        },
        LongEdge {
            faces: [FLOOR, BACK],
            start: DVec3::new(0.0, BREADTH, 0.0),
            facing: [-DVec3::Z, DVec3::Y],
        },
        LongEdge {
            faces: [TOP, BACK],
            start: DVec3::new(0.0, BREADTH, HEIGHT),
            facing: [DVec3::Z, DVec3::Y],
        },
        LongEdge {
            faces: [FLOOR, FRONT],
            start: DVec3::new(0.0, 0.0, 0.0),
            facing: [-DVec3::Z, -DVec3::Y],
        },
    ]
}

impl LongEdge {
    /// The edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        let [first, second] = self.faces;
        edge_between(part, first, second)
    }

    /// The round a fillet of `radius` lays on the edge: its axis the radius in
    /// from each face, along the whole length of the block.
    fn round(&self, radius: f64) -> Round {
        let [first, second] = self.facing;
        Round {
            start: self.start - (first + second) * radius,
            along: DVec3::X,
            length: LENGTH,
            radius,
            facing: self.facing,
        }
    }

    /// The flat a chamfer of `distance` lays on the edge: set back by it along
    /// each face, its corners at the left wall and at the right, facing
    /// halfway between the two faces.
    fn flat(&self, distance: f64) -> ([DVec3; 4], DVec3) {
        let [first, second] = self.facing;
        let on_first = self.start - second * distance;
        let on_second = self.start - first * distance;
        let along = DVec3::X * LENGTH;
        (
            [on_first, on_first + along, on_second + along, on_second],
            (first + second).normalize(),
        )
    }
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

    /// The two points where the round's sides meet a wall square to it,
    /// `end` of the way along it: on its first face, then on its second.
    fn sides_at(&self, end: f64) -> [DVec3; 2] {
        [0.0, 1.0].map(|share| self.axis_at(end) + self.facing_at(share) * self.radius)
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that the face numbered `face` is the round expected: every corner
/// of its triangles on it, and every ray aimed square at its axis from
/// outside meeting it first, where it faces away from the axis along the ray.
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

/// The one face among `made` whose triangles all lie on the round.
fn face_on(body: &Body, made: &[usize], round: &Round, what: &str) -> usize {
    let on: Vec<usize> = made
        .iter()
        .copied()
        .filter(|&face| {
            body.triangles_of(face)
                .flatten()
                .all(|corner| round.holds(corner))
        })
        .collect();
    assert_eq!(
        on.len(),
        1,
        "{what}: one face of {made:?} lies on the round about the line from {}",
        round.start
    );
    on[0]
}

/// The one flat face among `made` facing the way `facing` points.
fn face_facing(body: &Body, made: &[usize], facing: DVec3, what: &str) -> usize {
    let toward: Vec<usize> = made
        .iter()
        .copied()
        .filter(|&face| {
            body.plane_of(face)
                .is_some_and(|plane| plane.normal.distance(facing) < PLACE)
        })
        .collect();
    assert_eq!(
        toward.len(),
        1,
        "{what}: one face of {made:?} is flat and faces {facing}"
    );
    toward[0]
}

/// The corners of what is left of a long face of the block once two cuts set
/// it back to the line from `first` and the line from `second`, both points
/// on the left wall: from each, the length of the block along X.
fn strip(first: DVec3, second: DVec3) -> [DVec3; 4] {
    let along = DVec3::X * LENGTH;
    [first, first + along, second + along, second]
}

/// Holds that no edge of the body lies between the two faces of an edge the
/// step took: the new face stands between them.
fn no_longer_meet(body: &Body, [first, second]: [usize; 2], what: &str) {
    assert!(
        body.edge_names()
            .iter()
            .all(|name| !lies_between(name, first, second)),
        "{what}: faces {first} and {second} no longer meet, the new face stands between them"
    );
}

#[test]
fn a_fillet_on_the_four_long_edges_of_a_block_rounds_each_as_its_own_fillet_alone_would() {
    let radius = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let edges = long_edges();
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&block)).collect();

    let filleted = block
        .fillet_edges(&names, radius)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "the block's four long edges rounded in one step";
    taken_as_asked(filleted.asked, filleted.taken, radius, what);
    let body = &filleted.body;
    let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(radius)).collect();
    let area: f64 = rounds.iter().map(Round::area).sum();
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT - 4.0 * LENGTH * radius * radius * (1.0 - PI / 4.0),
        DRAWN * reach_of(&block) * area,
        what,
    );
    let made = faces_made(&block, body);
    assert_eq!(
        made.len(),
        4,
        "{what}: every face of the block kept, and one round more for each edge: {:?}",
        numbers(body)
    );
    assert!(
        numbers(&block).is_subset(&numbers(body)),
        "{what}: a face of the block went: {:?}",
        numbers(body)
    );
    let mut alone = Vec::new();
    for ((edge, name), round) in edges.iter().zip(&names).zip(&rounds) {
        let which = format!("the round between faces {:?}", edge.faces);
        let single = block
            .fillet(name, radius)
            .unwrap_or_else(|declined| panic!("{which}, alone: declined, {declined:?}"));
        let laid_alone = made_one(&block, &single.body, &which);
        is_the_round(
            &single.body,
            laid_alone,
            round,
            &format!("{which}, laid by its fillet alone"),
        );
        let laid = face_on(body, &made, round, &which);
        assert!(
            !body.is_flat(laid) && body.plane_of(laid).is_none(),
            "{which}: the face the fillet lays is curved, and offers no plane to draw on"
        );
        is_the_round(
            body,
            laid,
            round,
            &format!("{which}, laid with the three others"),
        );
        let [first, second] = edge.faces;
        assert_eq!(
            sides_met(body, laid),
            sides(&[first, second, RIGHT, LEFT]),
            "{which}: it meets its two faces along its sides, and the right and the left walls \
             at its ends, by one edge each, and no other round"
        );
        no_longer_meet(body, edge.faces, &which);
        alone.push(single.body);
    }
    takes_what_each_alone_takes(&block, body, &alone, what);
    for (face, first, second, which) in [
        (
            TOP,
            DVec3::new(0.0, radius, HEIGHT),
            DVec3::new(0.0, BREADTH - radius, HEIGHT),
            "what is left of the top, set back by the radius from the front and the back",
        ),
        (
            FLOOR,
            DVec3::new(0.0, radius, 0.0),
            DVec3::new(0.0, BREADTH - radius, 0.0),
            "what is left of the floor, set back by the radius from the front and the back",
        ),
        (
            FRONT,
            DVec3::new(0.0, 0.0, radius),
            DVec3::new(0.0, 0.0, HEIGHT - radius),
            "what is left of the front, set back by the radius from the floor and the top",
        ),
        (
            BACK,
            DVec3::new(0.0, BREADTH, radius),
            DVec3::new(0.0, BREADTH, HEIGHT - radius),
            "what is left of the back, set back by the radius from the floor and the top",
        ),
    ] {
        corners_are(body, face, &strip(first, second), which);
    }
    for (wall, end) in [(LEFT, 0.0), (RIGHT, 1.0)] {
        let ends: Vec<DVec3> = rounds
            .iter()
            .flat_map(|round| round.sides_at(end))
            .collect();
        corners_are_or_on(
            body,
            wall,
            &ends,
            |corner| rounds.iter().any(|round| round.holds(corner)),
            &format!("the wall at x = {}, its four corners rounded", end * LENGTH),
        );
    }
}

#[test]
fn a_chamfer_on_the_four_long_edges_of_a_block_cuts_each_as_its_own_chamfer_alone_would() {
    let distance = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let edges = long_edges();
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&block)).collect();

    let chamfered = block
        .chamfer_edges(&names, distance)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "the block's four long edges cut in one step";
    taken_as_asked(chamfered.asked, chamfered.taken, distance, what);
    let body = &chamfered.body;
    let promised = LENGTH * BREADTH * HEIGHT - 4.0 * LENGTH * distance * distance / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, what);
    let made = faces_made(&block, body);
    assert_eq!(
        made.len(),
        4,
        "{what}: every face of the block kept, and one flat more for each edge: {:?}",
        numbers(body)
    );
    assert!(
        numbers(&block).is_subset(&numbers(body)),
        "{what}: a face of the block went: {:?}",
        numbers(body)
    );
    let mut alone = Vec::new();
    for (edge, name) in edges.iter().zip(&names) {
        let which = format!("the flat between faces {:?}", edge.faces);
        let (corners, facing) = edge.flat(distance);
        let single = block
            .chamfer(name, distance)
            .unwrap_or_else(|declined| panic!("{which}, alone: declined, {declined:?}"));
        let cut_alone = made_one(&block, &single.body, &which);
        face_facing(
            &single.body,
            &[cut_alone],
            facing,
            &format!("{which}, cut by its chamfer alone"),
        );
        corners_are(
            &single.body,
            cut_alone,
            &corners,
            &format!("{which}, cut by its chamfer alone"),
        );
        let cut = face_facing(body, &made, facing, &which);
        corners_are(
            body,
            cut,
            &corners,
            &format!("{which}, cut with the three others"),
        );
        let [first, second] = edge.faces;
        assert_eq!(
            sides_met(body, cut),
            sides(&[first, second, RIGHT, LEFT]),
            "{which}: it meets its two faces along its sides, and the right and the left walls \
             at its ends, by one edge each, and no other flat"
        );
        no_longer_meet(body, edge.faces, &which);
        alone.push(single.body);
    }
    takes_what_each_alone_takes(&block, body, &alone, what);
    for (face, first, second, which) in [
        (
            TOP,
            DVec3::new(0.0, distance, HEIGHT),
            DVec3::new(0.0, BREADTH - distance, HEIGHT),
            "what is left of the top, set back by the distance from the front and the back",
        ),
        (
            FLOOR,
            DVec3::new(0.0, distance, 0.0),
            DVec3::new(0.0, BREADTH - distance, 0.0),
            "what is left of the floor, set back by the distance from the front and the back",
        ),
        (
            FRONT,
            DVec3::new(0.0, 0.0, distance),
            DVec3::new(0.0, 0.0, HEIGHT - distance),
            "what is left of the front, set back by the distance from the floor and the top",
        ),
        (
            BACK,
            DVec3::new(0.0, BREADTH, distance),
            DVec3::new(0.0, BREADTH, HEIGHT - distance),
            "what is left of the back, set back by the distance from the floor and the top",
        ),
    ] {
        corners_are(body, face, &strip(first, second), which);
    }
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        let octagon: Vec<DVec3> = edges
            .iter()
            .flat_map(|edge| {
                let (corners, _) = edge.flat(distance);
                [corners[0], corners[3]].map(|corner| corner + DVec3::X * x)
            })
            .collect();
        corners_are(
            body,
            wall,
            &octagon,
            &format!("the wall at x = {x}, its four corners cut"),
        );
    }
}

#[test]
fn each_round_and_each_flat_of_a_step_on_several_edges_is_numbered_in_the_order_the_step_names_its_edge_from_where_the_steps_numbers_ended()
 {
    let value = 3.0;
    let part = joined_to(
        &block([LENGTH, BREADTH, HEIGHT]),
        [DVec2::new(0.0, 40.0), DVec2::new(LENGTH, 40.0 + BREADTH)],
        HEIGHT,
    );
    let steps_end = 12;
    assert_eq!(
        part.faces_end(),
        steps_end,
        "the two blocks' steps numbered their twelve faces nought to eleven"
    );
    let edges = long_edges();
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&part)).collect();

    let filleted = part
        .fillet_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
    let chamfered = part
        .chamfer_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let laid = steps_end..steps_end + edges.len();
    for (what, body) in [
        ("the fillet", &filleted.body),
        ("the chamfer", &chamfered.body),
    ] {
        let mut kept_and_laid = numbers(&part);
        kept_and_laid.extend(laid.clone());
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every number the two blocks' faces answered to, and one for each edge from \
             {steps_end}, where the steps' numbers ended"
        );
        assert_eq!(
            body.faces_end(),
            laid.end,
            "{what}: the step names one face for each edge, and the next step numbers its own past \
             them"
        );
        let then = joined_to(body, [DVec2::new(0.0, -40.0), DVec2::new(10.0, -30.0)], 5.0);
        let mut after_the_next_step = kept_and_laid.clone();
        after_the_next_step.extend(laid.end..laid.end + 6);
        assert_eq!(
            numbers(&then),
            after_the_next_step,
            "{what}: a block joined after the step numbers its six faces past the last it laid"
        );
    }
    for (edge, number) in edges.iter().zip(laid) {
        let which = format!(
            "face {number}, laid for the edge between faces {:?}",
            edge.faces
        );
        is_the_round(
            &filleted.body,
            number,
            &edge.round(value),
            &format!("{which}, the fillet's round"),
        );
        let (corners, facing) = edge.flat(value);
        face_facing(
            &chamfered.body,
            &[number],
            facing,
            &format!("{which}, the chamfer's flat"),
        );
        corners_are(
            &chamfered.body,
            number,
            &corners,
            &format!("{which}, the chamfer's flat"),
        );
    }
}
