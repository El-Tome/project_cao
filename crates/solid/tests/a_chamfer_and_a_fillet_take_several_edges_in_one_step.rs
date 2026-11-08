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
//!   `a_fillet_on_several_edges_asked_more_than_one_of_them_gives_lays_every_round_at_the_one_radius_they_all_fit_and_reports_both`,
//!   `a_chamfer_on_several_edges_asked_more_than_one_of_them_gives_cuts_every_flat_at_the_one_distance_they_all_fit_and_reports_both`,
//!   each on a list whose limit is one edge's own faces, and on a list where
//!   that edge limits two set-backs on a shared face before they meet;
//!   a value that fits is taken as asked — the two tests of the first
//!   criterion, which ask a value every edge gives and hold that it is the one
//!   taken, and
//!   `a_fillet_and_a_chamfer_on_the_two_edges_either_side_of_a_face_take_a_value_that_fits_as_asked`;
//!   and, beyond the issue's wording, an outward edge and an inward one
//!   facing each other across a tread they share: the fillet takes half its
//!   depth, a round and a hollow of two axes meeting tangentially in an S —
//!   `a_fillet_on_a_tread_s_nose_and_foot_asked_more_than_half_its_depth_takes_half_a_round_and_a_hollow_meeting_in_an_s`;
//!   the chamfer, whose two flats would lie in one plane, one slope, is
//!   declined `Unsupported` until #791 builds it —
//!   `a_chamfer_on_a_tread_s_nose_and_foot_asked_half_its_depth_or_more_is_declined_as_unsupported`
//! - On a block 10 wide, rounding the two edges on either side of that face,
//!   asked more than 5, takes 5, where either alone would take more: the face
//!   is gone, and the two rounds, of one axis and one radius, are one
//!   half-cylinder tangent to both sides — a **full round** — answering to
//!   both edges' numbers. The body passes the soundness checks. A chamfer
//!   takes 5 too, and its two flats meet in a ridge —
//!   `a_fillet_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_in_one_full_round`,
//!   `a_chamfer_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_ridge`
//!   — and, beyond the issue's block, the same full round where the two sides
//!   are not square to the face and meet it at two angles, the edges still
//!   parallel: their set-backs sum to its width, and the two axes are one —
//!   `a_fillet_on_the_two_top_edges_of_a_prism_whose_sides_slant_takes_the_radius_whose_set_backs_meet_in_one_full_round`
//!   — also beyond its wording: a meeting pair named apart in the list still
//!   lays one full round, answering to the first's number and the third's,
//!   the edge named between them numbered between —
//!   `a_fillet_on_a_meeting_pair_named_apart_in_the_list_lays_one_full_round_answering_to_both_their_numbers_the_edge_between_numbered_between`;
//!   and an edge whose set-backs would meet two others', the four long edges
//!   of a square bar asked more than half its side, declines the fillet
//!   `Unsupported` — rounds that close on themselves are out of the issue's
//!   scope, built by #790 — where less than half lays all four —
//!   `a_fillet_on_the_four_long_edges_of_a_square_bar_asked_more_than_half_its_side_is_declined_as_unsupported`,
//!   `a_fillet_and_a_chamfer_on_the_four_long_edges_of_a_square_bar_asked_less_than_half_its_side_lay_each_at_the_value_asked`
//! - In a slot 10 wide, filleting the two edges of its floor, asked more than
//!   5, takes 5: the floor is gone, and the two fills, of one axis and one
//!   radius, are one hollow half-cylinder — a round-bottomed slot — answering
//!   to both edges' numbers. The body passes the soundness checks. A chamfer
//!   takes 5 too, and its two flats meet in a valley —
//!   `a_fillet_on_the_two_floor_edges_of_a_slot_10_wide_asked_5_or_more_takes_5_in_one_round_bottom`,
//!   `a_chamfer_on_the_two_floor_edges_of_a_slot_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_valley`;
//!   beyond the issue's wording, a slot that widens two thirds of the way
//!   along, its two floor edges ending on different faces, declines the
//!   fillet `Unsupported` where their set-backs would meet —
//!   `a_fillet_on_the_two_floor_edges_of_a_slot_that_widens_past_the_end_of_one_is_declined_as_unsupported`;
//!   the chamfer there takes 5, its flats meeting in a valley along the
//!   narrow part and the floor left beyond the widening —
//!   `a_chamfer_on_the_two_floor_edges_of_a_slot_that_widens_past_the_end_of_one_takes_5_a_valley_along_its_narrow_part`
//! - A list mixing an edge whose faces meet outwards and one whose faces meet
//!   inwards cuts the first off and fills the second in, at one value —
//!   `a_fillet_and_a_chamfer_on_an_outward_edge_and_an_inward_one_cut_the_first_off_and_fill_the_second_in_at_one_value`,
//!   at a value both fit, and at one each kind limits for the other
//! - Two edges of the list that share a vertex decline the step
//!   `Unsupported`, and nothing of it is laid —
//!   `a_fillet_and_a_chamfer_on_two_edges_sharing_a_vertex_are_declined_as_unsupported`;
//!   nothing laid is what a declined step is: the step hands back its reason
//!   and no body, and the part it was given is left as it was. Two edges
//!   sharing a vertex share a face wherever three faces meet there; sharing
//!   none, they meet where four faces or more do, and each alone is declined
//!   there already — on a block with another standing on its top, over its
//!   corner, both edges at the corner where they cross — so no list reaches
//!   past the step of one edge to that case
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
use cao_solid::{Body, Declined, EdgeName, Loop, Lost};
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
    raised_with(part, Contour::rectangle(from, to), height)
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

/// Holds that a step reports the value it was asked, and the one smaller
/// value every edge of its list gives.
fn reduced(asked: f64, taken: f64, [expected_asked, expected_taken]: [f64; 2], what: &str) {
    assert!(
        (asked - expected_asked).abs() < PLACE,
        "{what}: reports {asked} asked, where {expected_asked} was"
    );
    assert!(
        (taken - expected_taken).abs() < PLACE,
        "{what}: took {taken}, where every edge gives {expected_taken}"
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
    sides_met_by(body, &[face])
}

/// The sides of the faces the face answering to all of `numbers`, and to
/// nothing else, meets, one for each edge it has, in order.
fn sides_met_by(body: &Body, numbers: &[usize]) -> Vec<Vec<u32>> {
    let own: Vec<u32> = numbers.iter().map(|&number| number as u32).collect();
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

/// An edge of a block [`LENGTH`] long that runs along its length, from its
/// left wall to its right: the two faces it lies between, the corner of the
/// left wall it starts at, and the ways the two faces face.
#[derive(Clone)]
struct LongEdge {
    faces: [usize; 2],
    start: DVec3,
    facing: [DVec3; 2],
}

/// The block's four long edges, in the order the steps name them — not the
/// block's own: the top's front, the floor's back, the top's back and the
/// floor's front. No two share a vertex.
fn long_edges() -> [LongEdge; 4] {
    long_edges_of(BREADTH, HEIGHT)
}

/// The four long edges of a block [`LENGTH`] long, `breadth` across and
/// `height` high, in the order of [`long_edges`].
fn long_edges_of(breadth: f64, height: f64) -> [LongEdge; 4] {
    [
        LongEdge {
            faces: [TOP, FRONT],
            start: DVec3::new(0.0, 0.0, height),
            facing: [DVec3::Z, -DVec3::Y],
        },
        LongEdge {
            faces: [FLOOR, BACK],
            start: DVec3::new(0.0, breadth, 0.0),
            facing: [-DVec3::Z, DVec3::Y],
        },
        LongEdge {
            faces: [TOP, BACK],
            start: DVec3::new(0.0, breadth, height),
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

    /// The same edge on the block moved, whole, by `by`.
    fn moved(&self, by: DVec3) -> LongEdge {
        LongEdge {
            start: self.start + by,
            ..self.clone()
        }
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
    is_made_of(body, face, &[round], what);
}

/// Holds that the face numbered `face` is the rounds `parts` laid end to end
/// about one axis, as [`is_the_round`] holds it of one: every corner of its
/// triangles on one of them, and every ray aimed square at the axis from
/// outside, over each, meeting the face first.
fn is_made_of(body: &Body, face: usize, parts: &[&Round], what: &str) {
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            parts.iter().any(|round| round.holds(corner)),
            "{what}: the round is drawn through {corner}, off the cylinder of {} about the line \
             from {} along {}",
            parts[0].radius,
            parts[0].start,
            parts[0].along
        );
    }
    for round in parts {
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

/// The one flat face among `made` whose corners are the points expected.
fn flat_at(body: &Body, made: &[usize], corners: &[DVec3], what: &str) -> usize {
    let at: Vec<usize> = made
        .iter()
        .copied()
        .filter(|&face| {
            body.plane_of(face).is_some_and(|plane| {
                let near = |point: &DVec3, among: &[DVec3]| {
                    among.iter().any(|other| other.distance(*point) < PLACE)
                };
                plane.corners.iter().all(|corner| near(corner, corners))
                    && corners.iter().all(|point| near(point, &plane.corners))
            })
        })
        .collect();
    assert_eq!(
        at.len(),
        1,
        "{what}: one face of {made:?} is flat with its corners at {corners:?}"
    );
    at[0]
}

/// How wide the narrow block's top is: the most its top front edge gives.
const NARROW: f64 = 4.0;

/// The numbers the narrow block's top and front answer to, past the wide
/// block's six.
const NARROW_TOP: usize = TOP + 6;
const NARROW_FRONT: usize = FRONT + 6;

/// The part the one value is held on: the block of the first criterion, 10
/// across, and beside it, off the line of its edges, a block as long and as
/// high but [`NARROW`] across, from 40 to 44 along Y.
fn wide_and_narrow() -> Body {
    joined_to(
        &block([LENGTH, BREADTH, HEIGHT]),
        [DVec2::new(0.0, 40.0), DVec2::new(LENGTH, 40.0 + NARROW)],
        HEIGHT,
    )
}

/// The matter the two blocks of [`wide_and_narrow`] hold.
const WIDE_AND_NARROW: f64 = LENGTH * (BREADTH + NARROW) * HEIGHT;

/// The lists the one value is held on, each with what it is and the two
/// corners, on the left wall, of what is left of the wide block's top once
/// every edge of the list is set back by [`NARROW`]. The narrow block's top
/// front edge gives no more than that, its top taken whole; the wide block's
/// edges give more. With the wide block's top front edge, its top is left 6
/// wide; with both edges of its top, 2 wide, the two set-backs stopped short
/// of meeting by the narrow edge.
fn narrow_lists() -> [(&'static str, Vec<LongEdge>, [DVec3; 2]); 2] {
    let [top_front, _, top_back, _] = long_edges();
    let narrow = LongEdge {
        faces: [NARROW_TOP, NARROW_FRONT],
        start: DVec3::new(0.0, 40.0, HEIGHT),
        facing: [DVec3::Z, -DVec3::Y],
    };
    [
        (
            "the wide block's top front edge and the narrow block's",
            vec![top_front.clone(), narrow.clone()],
            [
                DVec3::new(0.0, NARROW, HEIGHT),
                DVec3::new(0.0, BREADTH, HEIGHT),
            ],
        ),
        (
            "both edges of the wide block's top and the narrow block's top front edge",
            vec![top_front, top_back, narrow],
            [
                DVec3::new(0.0, NARROW, HEIGHT),
                DVec3::new(0.0, BREADTH - NARROW, HEIGHT),
            ],
        ),
    ]
}

#[test]
fn a_fillet_on_several_edges_asked_more_than_one_of_them_gives_lays_every_round_at_the_one_radius_they_all_fit_and_reports_both()
 {
    let asked = 6.0;
    let part = wide_and_narrow();
    for (list, edges, [first, second]) in narrow_lists() {
        let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&part)).collect();

        let filleted = part
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("{list}: declined, {declined:?}"));

        let what = format!("the fillet on {list}, asked {asked}");
        let body = &filleted.body;
        let made = faces_made(&part, body);
        let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(NARROW)).collect();
        for (edge, round) in edges.iter().zip(&rounds) {
            let which = format!(
                "{what}: the round between faces {:?}, of the one radius {NARROW}",
                edge.faces
            );
            let laid = face_on(body, &made, round, &which);
            is_the_round(body, laid, round, &which);
        }
        assert_eq!(
            made.len(),
            edges.len(),
            "{what}: one round for each edge: {made:?}"
        );
        reduced(filleted.asked, filleted.taken, [asked, NARROW], &what);
        let area: f64 = rounds.iter().map(Round::area).sum();
        sound_drawn_within(
            body,
            WIDE_AND_NARROW - edges.len() as f64 * LENGTH * NARROW * NARROW * (1.0 - PI / 4.0),
            DRAWN * reach_of(&part) * area,
            &what,
        );
        corners_are(
            body,
            TOP,
            &strip(first, second),
            &format!("{what}: what is left of the wide block's top, set back by {NARROW}"),
        );
        corners_are(
            body,
            FRONT,
            &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, HEIGHT - NARROW)),
            &format!("{what}: what is left of the wide block's front, set back by {NARROW}"),
        );
        assert!(
            !body.has_face(NARROW_TOP),
            "{what}: the narrow block's top, {NARROW} wide, is taken whole"
        );
    }
}

#[test]
fn a_chamfer_on_several_edges_asked_more_than_one_of_them_gives_cuts_every_flat_at_the_one_distance_they_all_fit_and_reports_both()
 {
    let asked = 6.0;
    let part = wide_and_narrow();
    for (list, edges, [first, second]) in narrow_lists() {
        let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&part)).collect();

        let chamfered = part
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("{list}: declined, {declined:?}"));

        let what = format!("the chamfer on {list}, asked {asked}");
        let body = &chamfered.body;
        let made = faces_made(&part, body);
        for edge in &edges {
            let (corners, facing) = edge.flat(NARROW);
            let which = format!(
                "{what}: the flat between faces {:?}, set back by the one distance {NARROW}",
                edge.faces
            );
            let cut = flat_at(body, &made, &corners, &which);
            face_facing(body, &[cut], facing, &which);
        }
        assert_eq!(
            made.len(),
            edges.len(),
            "{what}: one flat for each edge: {made:?}"
        );
        reduced(chamfered.asked, chamfered.taken, [asked, NARROW], &what);
        let promised = WIDE_AND_NARROW - edges.len() as f64 * LENGTH * NARROW * NARROW / 2.0;
        sound_drawn_within(body, promised, EXACT * promised, &what);
        corners_are(
            body,
            TOP,
            &strip(first, second),
            &format!("{what}: what is left of the wide block's top, set back by {NARROW}"),
        );
        corners_are(
            body,
            FRONT,
            &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, HEIGHT - NARROW)),
            &format!("{what}: what is left of the wide block's front, set back by {NARROW}"),
        );
        assert!(
            !body.has_face(NARROW_TOP),
            "{what}: the narrow block's top, {NARROW} wide, is taken whole"
        );
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_the_two_edges_either_side_of_a_face_take_a_value_that_fits_as_asked() {
    let value = 4.9;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, _, top_back, _] = long_edges();
    let edges = [top_front, top_back];
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&block)).collect();

    let filleted = block
        .fillet_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
    let chamfered = block
        .chamfer_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let top_left = strip(
        DVec3::new(0.0, value, HEIGHT),
        DVec3::new(0.0, BREADTH - value, HEIGHT),
    );
    let what = format!("the fillet on both edges of a top {BREADTH} wide, asked {value}");
    taken_as_asked(filleted.asked, filleted.taken, value, &what);
    let body = &filleted.body;
    let made = faces_made(&block, body);
    let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(value)).collect();
    for (edge, round) in edges.iter().zip(&rounds) {
        let which = format!("{what}: the round between faces {:?}", edge.faces);
        let laid = face_on(body, &made, round, &which);
        is_the_round(body, laid, round, &which);
    }
    let area: f64 = rounds.iter().map(Round::area).sum();
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT - 2.0 * LENGTH * value * value * (1.0 - PI / 4.0),
        DRAWN * reach_of(&block) * area,
        &what,
    );
    corners_are(
        body,
        TOP,
        &top_left,
        &format!("{what}: what is left of the top, a strip between the two rounds"),
    );

    let what = format!("the chamfer on both edges of a top {BREADTH} wide, asked {value}");
    taken_as_asked(chamfered.asked, chamfered.taken, value, &what);
    let body = &chamfered.body;
    let made = faces_made(&block, body);
    for edge in &edges {
        let (corners, _) = edge.flat(value);
        flat_at(
            body,
            &made,
            &corners,
            &format!("{what}: the flat between faces {:?}", edge.faces),
        );
    }
    let promised = LENGTH * BREADTH * HEIGHT - 2.0 * LENGTH * value * value / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, &what);
    corners_are(
        body,
        TOP,
        &top_left,
        &format!("{what}: what is left of the top, a strip between the two flats"),
    );
}

/// What is asked of both edges of the top of the block 10 wide: more than
/// half its width, which either edge alone gives.
const MORE_THAN_HALF: f64 = 8.0;

/// Holds that each of the edges `names` alone takes more than `most`, asked
/// `asked`, which is what makes the limit the two edges' together.
fn alone_each_takes_more(
    step: impl Fn(&EdgeName) -> Result<f64, Declined>,
    names: &[EdgeName],
    most: f64,
    what: &str,
) {
    for name in names {
        let taken = step(name)
            .unwrap_or_else(|declined| panic!("{what}, one edge alone: declined, {declined:?}"));
        assert!(
            taken > most + PLACE,
            "{what}: one edge alone takes {taken}, more than {most}"
        );
    }
}

/// Holds that the numbers `laid` answer to one and the same face, which is
/// curved: drawn with the same triangles whichever is asked.
fn one_curved_face_answers_to(body: &Body, laid: &[usize], what: &str) {
    let drawn: Vec<Vec<[DVec3; 3]>> = laid
        .iter()
        .map(|&number| body.triangles_of(number).collect())
        .collect();
    assert!(
        !drawn[0].is_empty() && drawn.iter().all(|each| *each == drawn[0]),
        "{what}: faces {laid:?} are one face, drawn with the same triangles"
    );
    for &number in laid {
        assert!(
            !body.is_flat(number) && body.plane_of(number).is_none(),
            "{what}: face {number} is curved, and offers no plane to draw on"
        );
    }
}

#[test]
fn a_fillet_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_in_one_full_round()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, _, top_back, _] = long_edges();
    let names = [top_front.name(&block), top_back.name(&block)];
    let half = BREADTH / 2.0;
    alone_each_takes_more(
        |name| {
            block
                .fillet(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the fillet on the top's front or back edge",
    );

    let filleted = block
        .fillet_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!("the fillet on both edges of a top {BREADTH} wide, asked {MORE_THAN_HALF}");
    reduced(
        filleted.asked,
        filleted.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &filleted.body;
    let laid = [block.faces_end(), block.faces_end() + 1];
    assert!(!body.has_face(TOP), "{what}: the top is gone");
    let mut kept_and_laid = numbers(&block);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top, and the round answering to {laid:?}"
    );
    assert_eq!(
        body.faces_end(),
        laid[1] + 1,
        "{what}: the next step numbers its faces past both the round answers to"
    );
    one_curved_face_answers_to(body, &laid, &what);
    let axis = DVec3::new(0.0, half, HEIGHT - half);
    let halves = [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
        start: axis,
        along: DVec3::X,
        length: LENGTH,
        radius: half,
        facing,
    });
    is_made_of(
        body,
        laid[0],
        &[&halves[0], &halves[1]],
        &format!(
            "{what}: the full round, a half-cylinder of {half} about the line {half} under the \
             middle of the top, tangent to the front and the back"
        ),
    );
    assert_eq!(
        sides_met_by(body, &laid),
        sides(&[FRONT, RIGHT, BACK, LEFT]),
        "{what}: the full round meets the front and the back along its sides, and the right and \
         the left walls at its ends, by one edge each"
    );
    corners_are(
        body,
        FRONT,
        &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, HEIGHT - half)),
        &format!("{what}: what is left of the front, up to where the round takes over"),
    );
    corners_are(
        body,
        BACK,
        &strip(
            DVec3::new(0.0, BREADTH, 0.0),
            DVec3::new(0.0, BREADTH, HEIGHT - half),
        ),
        &format!("{what}: what is left of the back, up to where the round takes over"),
    );
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        corners_are_or_on(
            body,
            wall,
            &[
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, BREADTH, 0.0),
                DVec3::new(x, 0.0, HEIGHT - half),
                DVec3::new(x, BREADTH, HEIGHT - half),
            ],
            |corner| halves.iter().any(|round| round.holds(corner)),
            &format!("{what}: the wall at x = {x}, its top a half-circle"),
        );
    }
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT - LENGTH * (BREADTH * half - PI * half * half / 2.0),
        DRAWN * reach_of(&block) * PI * half * LENGTH,
        &what,
    );
}

#[test]
fn a_chamfer_on_the_two_edges_either_side_of_a_face_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_ridge()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, _, top_back, _] = long_edges();
    let names = [top_front.name(&block), top_back.name(&block)];
    let half = BREADTH / 2.0;
    alone_each_takes_more(
        |name| {
            block
                .chamfer(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the chamfer on the top's front or back edge",
    );

    let chamfered = block
        .chamfer_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!("the chamfer on both edges of a top {BREADTH} wide, asked {MORE_THAN_HALF}");
    reduced(
        chamfered.asked,
        chamfered.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &chamfered.body;
    let laid = [block.faces_end(), block.faces_end() + 1];
    assert!(!body.has_face(TOP), "{what}: the top is gone");
    let mut kept_and_laid = numbers(&block);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top, and a flat for each edge"
    );
    for (edge, number) in [&top_front, &top_back].into_iter().zip(laid) {
        let (corners, facing) = edge.flat(half);
        let which = format!(
            "{what}: face {number}, the flat between faces {:?}",
            edge.faces
        );
        face_facing(body, &[number], facing, &which);
        corners_are(body, number, &corners, &which);
    }
    edge_between(body, laid[0], laid[1]);
    assert_eq!(
        sides_met(body, laid[0]),
        sides(&[FRONT, RIGHT, LEFT, laid[1]]),
        "{what}: the front flat meets the front, the two walls, and the back flat along the \
         ridge, by one edge each"
    );
    assert_eq!(
        sides_met(body, laid[1]),
        sides(&[RIGHT, BACK, LEFT, laid[0]]),
        "{what}: the back flat meets the back, the two walls, and the front flat along the \
         ridge, by one edge each"
    );
    corners_are(
        body,
        FRONT,
        &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, HEIGHT - half)),
        &format!("{what}: what is left of the front"),
    );
    corners_are(
        body,
        BACK,
        &strip(
            DVec3::new(0.0, BREADTH, 0.0),
            DVec3::new(0.0, BREADTH, HEIGHT - half),
        ),
        &format!("{what}: what is left of the back"),
    );
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        corners_are(
            body,
            wall,
            &[
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, BREADTH, 0.0),
                DVec3::new(x, BREADTH, HEIGHT - half),
                DVec3::new(x, half, HEIGHT),
                DVec3::new(x, 0.0, HEIGHT - half),
            ],
            &format!("{what}: the wall at x = {x}, a pentagon with the ridge at its peak"),
        );
    }
    let promised = LENGTH * BREADTH * HEIGHT - 2.0 * LENGTH * half * half / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, &what);
}

/// The section of a prism lying along X, drawn on the plane of Y and Z: its
/// bottom 16 wide, its top 10 wide, from 2 to 12 at a height of 10, and its
/// two sides slanting in at two angles, so that a round asks a different
/// set-back along the top at either end. Raised along X, its numbers are its
/// start at x = 0 nought, its end one, then its bottom two, its right side
/// three, its top four and its left side five.
const SLANTED: [DVec2; 4] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(16.0, 0.0),
    DVec2::new(12.0, 10.0),
    DVec2::new(2.0, 10.0),
];
const SLANTED_START: usize = 0;
const SLANTED_END: usize = 1;
const SLANTED_RIGHT: usize = 3;
const SLANTED_TOP: usize = 4;
const SLANTED_LEFT: usize = 5;

/// A point of the [`SLANTED`] section's plane, across and up, `x` along the
/// prism.
fn on_section(at: DVec2, x: f64) -> DVec3 {
    DVec3::new(x, at.x, at.y)
}

/// The [`SLANTED`] section raised along X for [`LENGTH`].
fn slanted_prism() -> Body {
    let section = Frame {
        origin: DVec3::ZERO,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::straight(SLANTED.to_vec())),
            section,
            DVec3::X * LENGTH,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// One end of the slanted prism's top: the corner of the section it stands
/// on, the far end of its side, and the way that side faces.
struct SlantedEnd {
    corner: DVec2,
    foot: DVec2,
    outward: DVec2,
}

impl SlantedEnd {
    /// The tangent of half the angle the top and the side meet at, inside
    /// the matter: what a round's radius is divided by to give its set-back
    /// along either.
    fn half_tangent(&self, along_top: DVec2) -> f64 {
        let down = (self.foot - self.corner).normalize();
        let cosine = down.dot(along_top);
        (1.0 - cosine * cosine).sqrt() / (1.0 + cosine)
    }

    /// The angle the top and the side meet at, inside the matter.
    fn angle(&self, along_top: DVec2) -> f64 {
        (self.foot - self.corner).normalize().dot(along_top).acos()
    }
}

#[test]
fn a_fillet_on_the_two_top_edges_of_a_prism_whose_sides_slant_takes_the_radius_whose_set_backs_meet_in_one_full_round()
 {
    let asked = 20.0;
    let prism = slanted_prism();
    let names = [
        edge_between(&prism, SLANTED_TOP, SLANTED_LEFT),
        edge_between(&prism, SLANTED_RIGHT, SLANTED_TOP),
    ];
    let left = SlantedEnd {
        corner: SLANTED[3],
        foot: SLANTED[0],
        outward: DVec2::new(-10.0, 2.0).normalize(),
    };
    let right = SlantedEnd {
        corner: SLANTED[2],
        foot: SLANTED[1],
        outward: DVec2::new(10.0, 4.0).normalize(),
    };
    let width = SLANTED[2].x - SLANTED[3].x;
    let (left_tangent, right_tangent) =
        (left.half_tangent(DVec2::X), right.half_tangent(-DVec2::X));
    let radius = width / (1.0 / left_tangent + 1.0 / right_tangent);
    alone_each_takes_more(
        |name| prism.fillet(name, asked).map(|single| single.taken),
        &names,
        radius,
        "the fillet on one top edge of the slanted prism",
    );

    let filleted = prism
        .fillet_edges(&names, asked)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!(
        "the fillet on both top edges of the slanted prism, asked {asked}, its top {width} wide"
    );
    reduced(filleted.asked, filleted.taken, [asked, radius], &what);
    let body = &filleted.body;
    let laid = [prism.faces_end(), prism.faces_end() + 1];
    assert!(!body.has_face(SLANTED_TOP), "{what}: the top is gone");
    one_curved_face_answers_to(body, &laid, &what);
    let left_set_back = radius / left_tangent;
    let right_set_back = radius / right_tangent;
    let axis = SLANTED[3] + DVec2::new(left_set_back, -radius);
    assert!(
        axis.distance(SLANTED[2] + DVec2::new(-right_set_back, -radius)) < PLACE,
        "the two rounds' axes, {radius} under the top where each set-back ends, are one"
    );
    let facing = |way: DVec2| DVec3::new(0.0, way.x, way.y);
    let parts = [
        [facing(left.outward), DVec3::Z],
        [DVec3::Z, facing(right.outward)],
    ]
    .map(|facing| Round {
        start: on_section(axis, 0.0),
        along: DVec3::X,
        length: LENGTH,
        radius,
        facing,
    });
    is_made_of(
        body,
        laid[0],
        &[&parts[0], &parts[1]],
        &format!(
            "{what}: the full round, of {radius} about one axis, tangent to both sides and \
             turning from one to the other over where the top was"
        ),
    );
    assert_eq!(
        sides_met_by(body, &laid),
        sides(&[SLANTED_START, SLANTED_END, SLANTED_RIGHT, SLANTED_LEFT]),
        "{what}: the full round meets the two sides along its own, and the start and the end at \
         its ends, by one edge each"
    );
    let left_tangent_point = left.corner + (left.foot - left.corner).normalize() * left_set_back;
    let right_tangent_point =
        right.corner + (right.foot - right.corner).normalize() * right_set_back;
    for (side, foot, tangent_point) in [
        (SLANTED_LEFT, left.foot, left_tangent_point),
        (SLANTED_RIGHT, right.foot, right_tangent_point),
    ] {
        corners_are(
            body,
            side,
            &strip(on_section(foot, 0.0), on_section(tangent_point, 0.0)),
            &format!("{what}: what is left of side {side}, up to where the round takes over"),
        );
    }
    for (cap, x) in [(SLANTED_START, 0.0), (SLANTED_END, LENGTH)] {
        corners_are_or_on(
            body,
            cap,
            &[
                left.foot,
                right.foot,
                left_tangent_point,
                right_tangent_point,
            ]
            .map(|at| on_section(at, x)),
            |corner| parts.iter().any(|round| round.holds(corner)),
            &format!("{what}: the end at x = {x}, its top an arc"),
        );
    }
    let section = (16.0 + width) / 2.0 * 10.0;
    let taken: f64 = [
        (&left, DVec2::X, left_tangent),
        (&right, -DVec2::X, right_tangent),
    ]
    .into_iter()
    .map(|(end, along_top, tangent)| {
        radius * radius * (1.0 / tangent - (PI - end.angle(along_top)) / 2.0)
    })
    .sum();
    let turn = (PI - left.angle(DVec2::X)) + (PI - right.angle(-DVec2::X));
    sound_drawn_within(
        body,
        (section - taken) * LENGTH,
        DRAWN * reach_of(&prism) * turn * radius * LENGTH,
        &what,
    );
}

/// The outline of an L: an arm 30 long and 6 deep along the front, and at its
/// left end a stem 12 wide reaching back to 14. Raised by [`ELL_HEIGHT`], its
/// walls are numbered from the front round: the front two, the arm's right
/// end three, the arm's back four, the stem's right side five, the stem's
/// back six and the left seven. The arm's back and the stem's right side meet
/// inwards at [`INNER_CORNER`], and a fill set back along both can take no
/// more than the stem's right side, [`STEM_RIGHT_LENGTH`] long.
const ELL_OUTLINE: [DVec2; 6] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(30.0, 0.0),
    DVec2::new(30.0, 6.0),
    DVec2::new(12.0, 6.0),
    DVec2::new(12.0, 14.0),
    DVec2::new(0.0, 14.0),
];
const ELL_HEIGHT: f64 = 10.0;
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;
const INNER_CORNER: DVec2 = DVec2::new(12.0, 6.0);
const STEM_RIGHT_LENGTH: f64 = 8.0;
const ELL_VOLUME: f64 = (30.0 * 6.0 + 12.0 * 8.0) * ELL_HEIGHT;

/// The numbers the block beside the L answers to for its top and its front,
/// past the L's eight.
const BESIDE_TOP: usize = TOP + 8;
const BESIDE_FRONT: usize = FRONT + 8;

/// The L, and beside it, off the line of every edge it has, a block
/// [`LENGTH`] long, `breadth` across from 40 along Y, and [`HEIGHT`] high,
/// whose top front edge meets outwards.
fn ell_and_block(breadth: f64) -> Body {
    let ell = raised_with(
        &Body::default(),
        Contour::straight(ELL_OUTLINE.to_vec()),
        ELL_HEIGHT,
    );
    joined_to(
        &ell,
        [DVec2::new(0.0, 40.0), DVec2::new(LENGTH, 40.0 + breadth)],
        HEIGHT,
    )
}

/// The block's top front edge, beside the L.
fn beside_top_front() -> LongEdge {
    LongEdge {
        faces: [BESIDE_TOP, BESIDE_FRONT],
        start: DVec3::new(0.0, 40.0, HEIGHT),
        facing: [DVec3::Z, -DVec3::Y],
    }
}

/// The round a fillet lays in a concave corner: a cylinder of `radius` about
/// the line from `start` along `along` for `length`, that line standing in
/// the air the two faces leave between them. The round turns from facing the
/// way the edge's first face does to facing the way its second does, and
/// everywhere faces its axis, a radius away against the way it faces.
struct Hollow {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Hollow {
    /// The round of `radius` in the L's inner corner, the radius out from
    /// the arm's back and from the stem's right side, from the floor to the
    /// top.
    fn in_the_ell(radius: f64) -> Hollow {
        Hollow {
            start: (INNER_CORNER + DVec2::splat(radius)).extend(0.0),
            along: DVec3::Z,
            length: ELL_HEIGHT,
            radius,
            facing: [DVec3::Y, DVec3::X],
        }
    }

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
    /// ends, and between its two sides — no further round from the way it
    /// faces halfway than either side is.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let toward_the_axis = self.along * along - off;
        let halfway = self.facing_at(0.5);
        let least = self.radius * self.facing[0].dot(halfway);
        (toward_the_axis.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && toward_the_axis.dot(halfway) >= least - PLACE
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that the face numbered `face` is the hollow round expected: every
/// corner of its triangles on it, and every ray aimed from its axis, in the
/// air the round leaves, out to it meeting it first, where it faces back
/// along the ray.
fn is_the_hollow(body: &Body, face: usize, hollow: &Hollow, what: &str) {
    is_hollow_made_of(body, face, &[hollow], what);
}

/// Holds that the face numbered `face` is the hollow rounds `parts` laid end
/// to end about one axis, as [`is_the_hollow`] holds it of one: every corner
/// of its triangles on one of them, and every ray aimed from the axis out to
/// each meeting the face first.
fn is_hollow_made_of(body: &Body, face: usize, parts: &[&Hollow], what: &str) {
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            parts.iter().any(|hollow| hollow.holds(corner)),
            "{what}: the round is drawn through {corner}, off the cylinder of {} about the line \
             from {} along {}",
            parts[0].radius,
            parts[0].start,
            parts[0].along
        );
    }
    for hollow in parts {
        for along in [0.11, 0.47, 0.83] {
            for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
                let facing = hollow.facing_at(share);
                let from = hollow.axis_at(along);
                let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                    panic!("{what}: a ray from {from} on the axis meets nothing")
                });
                assert_eq!(
                    hit.face, face,
                    "{what}: a ray from {from} on the axis meets face {} before the round",
                    hit.face
                );
                assert!(
                    hit.normal.distance(facing) < PLACE,
                    "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                     back toward its axis",
                    hit.normal
                );
            }
        }
    }
}

/// The lists mixing the block's outward edge and the L's inward one, each
/// with what it shows, how wide the block is, the value asked, what the
/// outward edge and the inward one each take alone, and the one value every
/// edge of the list gives.
fn mixed_lists() -> [(&'static str, f64, f64, [f64; 2], f64); 3] {
    [
        ("at a value both fit", BREADTH, 3.0, [3.0, 3.0], 3.0),
        (
            "the outward edge limiting the inward one, the block's top 5 wide taken whole",
            5.0,
            7.0,
            [5.0, 7.0],
            5.0,
        ),
        (
            "the inward edge limiting the outward one, the stem's right side taken whole",
            BREADTH,
            9.0,
            [9.0, STEM_RIGHT_LENGTH],
            STEM_RIGHT_LENGTH,
        ),
    ]
}

#[test]
fn a_fillet_and_a_chamfer_on_an_outward_edge_and_an_inward_one_cut_the_first_off_and_fill_the_second_in_at_one_value()
 {
    for (case, breadth, asked, each_alone, taken) in mixed_lists() {
        let part = ell_and_block(breadth);
        let outward = beside_top_front();
        let names = [
            outward.name(&part),
            edge_between(&part, ARM_BACK, STEM_RIGHT),
        ];
        let laid = [part.faces_end(), part.faces_end() + 1];
        let block_volume = LENGTH * breadth * HEIGHT;

        let filleted = part
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("the fillet {case}: declined, {declined:?}"));
        let chamfered = part
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("the chamfer {case}: declined, {declined:?}"));

        let what = format!("the fillet {case}, asked {asked}");
        let alone: Vec<f64> = names
            .iter()
            .map(|name| part.fillet(name, asked).expect("each edge alone").taken)
            .collect();
        assert!(
            alone
                .iter()
                .zip(each_alone)
                .all(|(took, expected)| (took - expected).abs() < PLACE),
            "{what}: the outward edge and the inward one alone take {alone:?}, where they give \
             {each_alone:?}"
        );
        reduced(filleted.asked, filleted.taken, [asked, taken], &what);
        let body = &filleted.body;
        let round = outward.round(taken);
        let hollow = Hollow::in_the_ell(taken);
        is_the_round(
            body,
            laid[0],
            &round,
            &format!("{what}: the block's top front edge rounded off at {taken}"),
        );
        is_the_hollow(
            body,
            laid[1],
            &hollow,
            &format!("{what}: the L's inner corner filled in at {taken}"),
        );
        let quarter = taken * taken * (1.0 - PI / 4.0);
        sound_drawn_within(
            body,
            ELL_VOLUME + block_volume - quarter * LENGTH + quarter * ELL_HEIGHT,
            DRAWN * reach_of(&part) * (round.area() + hollow.area()),
            &what,
        );

        let what = format!("the chamfer {case}, asked {asked}");
        let alone: Vec<f64> = names
            .iter()
            .map(|name| part.chamfer(name, asked).expect("each edge alone").taken)
            .collect();
        assert!(
            alone
                .iter()
                .zip(each_alone)
                .all(|(took, expected)| (took - expected).abs() < PLACE),
            "{what}: the outward edge and the inward one alone take {alone:?}, where they give \
             {each_alone:?}"
        );
        reduced(chamfered.asked, chamfered.taken, [asked, taken], &what);
        let body = &chamfered.body;
        let (cut, facing) = outward.flat(taken);
        let which = format!("{what}: the block's top front edge cut off at {taken}");
        face_facing(body, &[laid[0]], facing, &which);
        corners_are(body, laid[0], &cut, &which);
        let on_arm_back = INNER_CORNER + DVec2::X * taken;
        let on_stem_right = INNER_CORNER + DVec2::Y * taken;
        let filled = [on_arm_back, on_stem_right]
            .map(|at| [at.extend(0.0), at.extend(ELL_HEIGHT)])
            .concat();
        let which = format!("{what}: the L's inner corner filled in at {taken}");
        face_facing(
            body,
            &[laid[1]],
            DVec3::new(1.0, 1.0, 0.0).normalize(),
            &which,
        );
        corners_are(body, laid[1], &filled, &which);
        let triangle = taken * taken / 2.0;
        let promised = ELL_VOLUME + block_volume - triangle * LENGTH + triangle * ELL_HEIGHT;
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}

#[test]
fn a_fillet_on_a_meeting_pair_named_apart_in_the_list_lays_one_full_round_answering_to_both_their_numbers_the_edge_between_numbered_between()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let [top_front, floor_back, top_back, _] = long_edges();
    let names = [
        top_front.name(&block),
        floor_back.name(&block),
        top_back.name(&block),
    ];
    let half = BREADTH / 2.0;

    let filleted = block
        .fillet_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!(
        "the fillet on the top's front edge, the floor's back edge and the top's back edge, asked \
         {MORE_THAN_HALF}"
    );
    reduced(
        filleted.asked,
        filleted.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &filleted.body;
    let end = block.faces_end();
    let (pair, between) = ([end, end + 2], end + 1);
    let mut kept_and_laid = numbers(&block);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(end..end + 3);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the block but its top, the full round answering to {pair:?} and \
         the floor's back round to {between}"
    );
    assert_eq!(
        body.faces_end(),
        end + 3,
        "{what}: the next step numbers its faces past the three the edges were given"
    );
    one_curved_face_answers_to(body, &pair, &what);
    let axis = DVec3::new(0.0, half, HEIGHT - half);
    let halves = [[-DVec3::Y, DVec3::Z], [DVec3::Z, DVec3::Y]].map(|facing| Round {
        start: axis,
        along: DVec3::X,
        length: LENGTH,
        radius: half,
        facing,
    });
    is_made_of(
        body,
        pair[0],
        &[&halves[0], &halves[1]],
        &format!("{what}: the full round over where the top was"),
    );
    let floor_round = floor_back.round(half);
    assert!(
        !body.is_flat(between) && body.plane_of(between).is_none(),
        "{what}: face {between} is curved"
    );
    is_the_round(
        body,
        between,
        &floor_round,
        &format!("{what}: face {between}, the floor's back edge rounded off at {half}"),
    );
    assert_eq!(
        sides_met_by(body, &pair),
        sides(&[FRONT, RIGHT, BACK, LEFT]),
        "{what}: the full round meets the front, the back and the two walls, by one edge each"
    );
    assert_eq!(
        sides_met(body, between),
        sides(&[FLOOR, RIGHT, BACK, LEFT]),
        "{what}: the floor's back round meets the floor, the back and the two walls, by one edge \
         each"
    );
    corners_are(
        body,
        BACK,
        &strip(
            DVec3::new(0.0, BREADTH, half),
            DVec3::new(0.0, BREADTH, HEIGHT - half),
        ),
        &format!("{what}: what is left of the back, between its two rounds"),
    );
    sound_drawn_within(
        body,
        LENGTH * BREADTH * HEIGHT
            - LENGTH * (BREADTH * half - PI * half * half / 2.0)
            - LENGTH * half * half * (1.0 - PI / 4.0),
        DRAWN * reach_of(&block) * (PI * half * LENGTH + floor_round.area()),
        &what,
    );
}

/// The side of the square bar: its four long edges' set-backs meet across
/// every long face at half of it.
const BAR: f64 = 10.0;

#[test]
fn a_fillet_on_the_four_long_edges_of_a_square_bar_asked_more_than_half_its_side_is_declined_as_unsupported()
 {
    let bar = block([LENGTH, BAR, BAR]);
    let names: Vec<EdgeName> = long_edges_of(BAR, BAR)
        .iter()
        .map(|edge| edge.name(&bar))
        .collect();

    let declined = bar.fillet_edges(&names, MORE_THAN_HALF).err();

    assert_eq!(
        declined,
        Some(Declined::Unsupported),
        "every edge's set-backs would meet two others', across the face on either side of it"
    );
}

#[test]
fn a_fillet_and_a_chamfer_on_the_four_long_edges_of_a_square_bar_asked_less_than_half_its_side_lay_each_at_the_value_asked()
 {
    let value = 4.0;
    let bar = block([LENGTH, BAR, BAR]);
    let edges = long_edges_of(BAR, BAR);
    let names: Vec<EdgeName> = edges.iter().map(|edge| edge.name(&bar)).collect();
    let laid = bar.faces_end()..bar.faces_end() + edges.len();

    let filleted = bar
        .fillet_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));
    let chamfered = bar
        .chamfer_edges(&names, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let what = format!("the fillet on the four long edges of a bar {BAR} square, asked {value}");
    taken_as_asked(filleted.asked, filleted.taken, value, &what);
    let body = &filleted.body;
    let mut area = 0.0;
    for (edge, number) in edges.iter().zip(laid.clone()) {
        let round = edge.round(value);
        is_the_round(
            body,
            number,
            &round,
            &format!(
                "{what}: face {number}, the round between faces {:?}",
                edge.faces
            ),
        );
        area += round.area();
    }
    sound_drawn_within(
        body,
        LENGTH * BAR * BAR - 4.0 * LENGTH * value * value * (1.0 - PI / 4.0),
        DRAWN * reach_of(&bar) * area,
        &what,
    );
    corners_are(
        body,
        TOP,
        &strip(
            DVec3::new(0.0, value, BAR),
            DVec3::new(0.0, BAR - value, BAR),
        ),
        &format!("{what}: what is left of the top, a strip between two rounds"),
    );

    let what = format!("the chamfer on the four long edges of a bar {BAR} square, asked {value}");
    taken_as_asked(chamfered.asked, chamfered.taken, value, &what);
    let body = &chamfered.body;
    for (edge, number) in edges.iter().zip(laid) {
        let (corners, facing) = edge.flat(value);
        let which = format!(
            "{what}: face {number}, the flat between faces {:?}",
            edge.faces
        );
        face_facing(body, &[number], facing, &which);
        corners_are(body, number, &corners, &which);
    }
    let promised = LENGTH * BAR * BAR - 4.0 * LENGTH * value * value / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, &what);
    corners_are(
        body,
        TOP,
        &strip(
            DVec3::new(0.0, value, BAR),
            DVec3::new(0.0, BAR - value, BAR),
        ),
        &format!("{what}: what is left of the top, a strip between two flats"),
    );
}

/// How far across the slotted block reaches, and where the slot's two walls
/// stand across it, 10 apart, and how high its floor is.
const SLOTTED_BREADTH: f64 = 30.0;
const SLOT_WALLS: [f64; 2] = [10.0, 20.0];
const SLOT_FLOOR_HEIGHT: f64 = 10.0;

/// The numbers the cut that makes the slot gives its floor and its two walls,
/// past the block's six: its floor six, the wall at y = 10 eight and the
/// wall at y = 20 ten. Its top, its right and its left stand outside the
/// block and are gone.
const SLOT_FLOOR: usize = 6;
const SLOT_NEAR_WALL: usize = 8;
const SLOT_FAR_WALL: usize = 10;

/// A block [`LENGTH`] long, [`SLOTTED_BREADTH`] across and [`HEIGHT`] high,
/// with a slot cut along its whole length between [`SLOT_WALLS`], from its top
/// down to [`SLOT_FLOOR_HEIGHT`]. The slot's two floor edges meet inwards,
/// facing each other across its floor 10 wide, and run from the block's left
/// wall to its right, where the slot opens.
fn slotted() -> Body {
    let solid = block([LENGTH, SLOTTED_BREADTH, HEIGHT]);
    let [near, far] = SLOT_WALLS;
    let tool = solid
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(-1.0, near),
                DVec2::new(LENGTH + 1.0, far),
            )),
            Frame {
                origin: DVec3::Z * SLOT_FLOOR_HEIGHT,
                ..ground()
            },
            DVec3::Z * HEIGHT,
        )
        .expect("the tool raises");
    solid.difference(&tool).expect("the slot is cut")
}

/// The matter the slotted block holds.
const SLOTTED_VOLUME: f64 = LENGTH
    * (SLOTTED_BREADTH * HEIGHT - (SLOT_WALLS[1] - SLOT_WALLS[0]) * (HEIGHT - SLOT_FLOOR_HEIGHT));

/// The names of the slot's two floor edges, the near wall's first.
fn slot_floor_edges(slotted: &Body) -> [EdgeName; 2] {
    [
        edge_between(slotted, SLOT_FLOOR, SLOT_NEAR_WALL),
        edge_between(slotted, SLOT_FLOOR, SLOT_FAR_WALL),
    ]
}

/// The corners, on the left wall, of what is left of each of the slot's walls
/// once its floor is filled in to `up` above it.
fn slot_walls_from(up: f64) -> [[DVec3; 4]; 2] {
    SLOT_WALLS.map(|across| {
        strip(
            DVec3::new(0.0, across, SLOT_FLOOR_HEIGHT + up),
            DVec3::new(0.0, across, HEIGHT),
        )
    })
}

#[test]
fn a_fillet_on_the_two_floor_edges_of_a_slot_10_wide_asked_5_or_more_takes_5_in_one_round_bottom() {
    let slotted = slotted();
    let names = slot_floor_edges(&slotted);
    let half = (SLOT_WALLS[1] - SLOT_WALLS[0]) / 2.0;
    alone_each_takes_more(
        |name| {
            slotted
                .fillet(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the fillet on one floor edge of the slot",
    );

    for asked in [MORE_THAN_HALF, half] {
        let filleted = slotted
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!("the fillet on both floor edges of a slot 10 wide, asked {asked}");
        reduced(filleted.asked, filleted.taken, [asked, half], &what);
        let body = &filleted.body;
        let laid = [slotted.faces_end(), slotted.faces_end() + 1];
        assert!(
            !body.has_face(SLOT_FLOOR),
            "{what}: the slot's floor is gone"
        );
        let mut kept_and_laid = numbers(&slotted);
        kept_and_laid.remove(&SLOT_FLOOR);
        kept_and_laid.extend(laid);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the slotted block but the slot's floor, and the round bottom \
             answering to {laid:?}"
        );
        assert_eq!(
            body.faces_end(),
            laid[1] + 1,
            "{what}: the next step numbers its faces past both the round bottom answers to"
        );
        one_curved_face_answers_to(body, &laid, &what);
        let axis = DVec3::new(
            0.0,
            (SLOT_WALLS[0] + SLOT_WALLS[1]) / 2.0,
            SLOT_FLOOR_HEIGHT + half,
        );
        let halves = [[DVec3::Y, DVec3::Z], [DVec3::Z, -DVec3::Y]].map(|facing| Hollow {
            start: axis,
            along: DVec3::X,
            length: LENGTH,
            radius: half,
            facing,
        });
        is_hollow_made_of(
            body,
            laid[0],
            &[&halves[0], &halves[1]],
            &format!(
                "{what}: the round bottom, a hollow half-cylinder of {half} about the line {half} \
                 above the middle of the floor, tangent to both walls"
            ),
        );
        assert!(
            body.edge_names()
                .iter()
                .all(|name| !lies_between(name, laid[0], laid[1])),
            "{what}: no edge stands between {} and {}: they are one face",
            laid[0],
            laid[1]
        );
        assert_eq!(
            sides_met_by(body, &laid),
            sides(&[RIGHT, LEFT, SLOT_NEAR_WALL, SLOT_FAR_WALL]),
            "{what}: the round bottom meets the slot's two walls along its sides, and the block's \
             right and left walls at its ends, where the slot opens, by one edge each"
        );
        for (wall, corners) in [SLOT_NEAR_WALL, SLOT_FAR_WALL]
            .into_iter()
            .zip(slot_walls_from(half))
        {
            corners_are(
                body,
                wall,
                &corners,
                &format!("{what}: what is left of the slot's wall {wall}, above the round bottom"),
            );
        }
        let [near, far] = SLOT_WALLS;
        for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
            corners_are_or_on(
                body,
                wall,
                &[
                    DVec3::new(x, 0.0, 0.0),
                    DVec3::new(x, SLOTTED_BREADTH, 0.0),
                    DVec3::new(x, SLOTTED_BREADTH, HEIGHT),
                    DVec3::new(x, far, HEIGHT),
                    DVec3::new(x, far, SLOT_FLOOR_HEIGHT + half),
                    DVec3::new(x, near, SLOT_FLOOR_HEIGHT + half),
                    DVec3::new(x, near, HEIGHT),
                    DVec3::new(x, 0.0, HEIGHT),
                ],
                |corner| halves.iter().any(|hollow| hollow.holds(corner)),
                &format!("{what}: the wall at x = {x}, the slot's bottom a half-circle on it"),
            );
        }
        sound_drawn_within(
            body,
            SLOTTED_VOLUME + LENGTH * ((far - near) * half - PI * half * half / 2.0),
            DRAWN * reach_of(&slotted) * PI * half * LENGTH,
            &what,
        );
    }
}

#[test]
fn a_chamfer_on_the_two_floor_edges_of_a_slot_10_wide_asked_more_than_5_takes_5_its_flats_meeting_in_a_valley()
 {
    let slotted = slotted();
    let names = slot_floor_edges(&slotted);
    let half = (SLOT_WALLS[1] - SLOT_WALLS[0]) / 2.0;
    alone_each_takes_more(
        |name| {
            slotted
                .chamfer(name, MORE_THAN_HALF)
                .map(|single| single.taken)
        },
        &names,
        half,
        "the chamfer on one floor edge of the slot",
    );

    let chamfered = slotted
        .chamfer_edges(&names, MORE_THAN_HALF)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = format!("the chamfer on both floor edges of a slot 10 wide, asked {MORE_THAN_HALF}");
    reduced(
        chamfered.asked,
        chamfered.taken,
        [MORE_THAN_HALF, half],
        &what,
    );
    let body = &chamfered.body;
    let laid = [slotted.faces_end(), slotted.faces_end() + 1];
    assert!(
        !body.has_face(SLOT_FLOOR),
        "{what}: the slot's floor is gone"
    );
    let mut kept_and_laid = numbers(&slotted);
    kept_and_laid.remove(&SLOT_FLOOR);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the slotted block but the slot's floor, and a flat for each edge"
    );
    let [near, far] = SLOT_WALLS;
    let middle = (near + far) / 2.0;
    let valley = DVec3::new(0.0, middle, SLOT_FLOOR_HEIGHT);
    for (number, across, facing) in [
        (laid[0], near, DVec3::new(0.0, 1.0, 1.0)),
        (laid[1], far, DVec3::new(0.0, -1.0, 1.0)),
    ] {
        let which = format!("{what}: face {number}, the flat from the wall at y = {across}");
        face_facing(body, &[number], facing.normalize(), &which);
        corners_are(
            body,
            number,
            &strip(valley, DVec3::new(0.0, across, SLOT_FLOOR_HEIGHT + half)),
            &format!("{which}, down to the valley at the middle of the floor"),
        );
    }
    edge_between(body, laid[0], laid[1]);
    assert_eq!(
        sides_met(body, laid[0]),
        sides(&[RIGHT, LEFT, SLOT_NEAR_WALL, laid[1]]),
        "{what}: the near flat meets its wall, the block's two walls, and the far flat along the \
         valley, by one edge each"
    );
    assert_eq!(
        sides_met(body, laid[1]),
        sides(&[RIGHT, SLOT_FAR_WALL, LEFT, laid[0]]),
        "{what}: the far flat meets its wall, the block's two walls, and the near flat along the \
         valley, by one edge each"
    );
    for (wall, corners) in [SLOT_NEAR_WALL, SLOT_FAR_WALL]
        .into_iter()
        .zip(slot_walls_from(half))
    {
        corners_are(
            body,
            wall,
            &corners,
            &format!("{what}: what is left of the slot's wall {wall}, above its flat"),
        );
    }
    for (wall, x) in [(LEFT, 0.0), (RIGHT, LENGTH)] {
        corners_are(
            body,
            wall,
            &[
                DVec3::new(x, 0.0, 0.0),
                DVec3::new(x, SLOTTED_BREADTH, 0.0),
                DVec3::new(x, SLOTTED_BREADTH, HEIGHT),
                DVec3::new(x, far, HEIGHT),
                DVec3::new(x, far, SLOT_FLOOR_HEIGHT + half),
                DVec3::new(x, middle, SLOT_FLOOR_HEIGHT),
                DVec3::new(x, near, SLOT_FLOOR_HEIGHT + half),
                DVec3::new(x, near, HEIGHT),
                DVec3::new(x, 0.0, HEIGHT),
            ],
            &format!("{what}: the wall at x = {x}, the slot a V on it"),
        );
    }
    let promised = SLOTTED_VOLUME + LENGTH * (far - near) * half / 2.0;
    sound_drawn_within(body, promised, EXACT * promised, &what);
}

/// The part with a block standing on the ground between the corners `from`
/// and `to`, raised from `from_height` by `height`, cut from it as `cao_part`
/// cuts a step: the tool's faces numbered past the part's.
fn cut_from(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(from, to)),
            Frame {
                origin: DVec3::Z * from_height,
                ..ground()
            },
            DVec3::Z * height,
        )
        .expect("the tool raises");
    part.difference(&tool).expect("the cut is built")
}

/// A block of a length, a breadth and a height whose corner stands at
/// `origin`, numbered as [`block`] numbers it.
fn block_at(origin: DVec3, [length, breadth, height]: [f64; 3]) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(
            &profile(Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth))),
            Frame { origin, ..ground() },
            DVec3::Z * height,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

#[test]
fn a_fillet_and_a_chamfer_on_two_edges_sharing_a_vertex_are_declined_as_unsupported() {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let top_front = edge_between(&block, TOP, FRONT);

    for (what, other) in [
        (
            "the top's front edge and its right edge, meeting at the top's front right corner",
            edge_between(&block, TOP, RIGHT),
        ),
        (
            "the top's front edge and the front's right edge, meeting at the same corner",
            edge_between(&block, FRONT, RIGHT),
        ),
    ] {
        let names = [top_front.clone(), other];

        let filleted = block.fillet_edges(&names, 3.0).err();
        let chamfered = block.chamfer_edges(&names, 3.0).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {what}"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {what}"
        );
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_a_list_holding_an_edge_its_name_no_longer_finds_are_declined_with_that_edge_s_reason()
 {
    let drawn = block([LENGTH, BREADTH, BREADTH]);
    let floor_back = edge_between(&drawn, FLOOR, BACK);
    let top_front = edge_between(&drawn, TOP, FRONT);
    let rabbeted = cut_from(
        &drawn,
        [DVec2::new(-1.0, -1.0), DVec2::new(LENGTH + 1.0, 2.0)],
        8.0,
        3.0,
    );
    let notched = cut_from(
        &drawn,
        [DVec2::new(10.0, -1.0), DVec2::new(20.0, 3.0)],
        7.0,
        4.0,
    );

    for (what, part, lost) in [
        (
            "a rabbet all along the top's front edge",
            rabbeted,
            Lost::Gone,
        ),
        (
            "a notch a third of the way along the top's front edge",
            notched,
            Lost::Split { now: 2 },
        ),
    ] {
        assert_eq!(part.lost(&top_front), Some(lost), "{what}");
        assert_eq!(
            part.lost(&floor_back),
            None,
            "{what}: the floor's back edge is still found"
        );
        let names = [floor_back.clone(), top_front.clone()];

        let filleted = part.fillet_edges(&names, 1.0).err();
        let chamfered = part.chamfer_edges(&names, 1.0).err();

        assert_eq!(
            filleted,
            Some(Declined::Lost(lost)),
            "the fillet after {what}"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Lost(lost)),
            "the chamfer after {what}"
        );
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_two_names_finding_the_same_edge_are_declined_as_unsupported() {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let top_front = edge_between(&block, TOP, FRONT);
    let half = LENGTH / 2.0;
    let second_from = |from: f64| {
        joined_to(
            &joined_to(
                &Body::default(),
                [DVec2::ZERO, DVec2::new(half, BREADTH)],
                BREADTH,
            ),
            [DVec2::new(from, 0.0), DVec2::new(from + half, BREADTH)],
            BREADTH,
        )
    };
    let apart = second_from(half + 5.0);
    let bridged = [
        edge_between(&apart, TOP, FRONT),
        edge_between(&apart, TOP + 6, FRONT + 6),
    ];
    let abutting = second_from(half);
    for name in &bridged {
        assert_eq!(
            abutting.lost(name),
            None,
            "the names of both blocks' top front edges find the edge the join bridged"
        );
    }

    for (what, part, names) in [
        (
            "the top's front edge named twice",
            &block,
            [top_front.clone(), top_front.clone()],
        ),
        (
            "both blocks' top front edges, which a join bridged into one",
            &abutting,
            bridged.clone(),
        ),
    ] {
        let filleted = part.fillet_edges(&names, 2.0).err();
        let chamfered = part.chamfer_edges(&names, 2.0).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {what}"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {what}"
        );
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_several_edges_take_the_edges_their_names_find_again_after_a_change_of_size()
 {
    let value = 3.0;
    let drawn = block([LENGTH, BREADTH, HEIGHT]);
    let names: Vec<EdgeName> = long_edges().iter().map(|edge| edge.name(&drawn)).collect();
    let (breadth, height) = (14.0, 26.0);

    for (what, origin, [breadth, height]) in [
        ("as drawn", DVec3::ZERO, [BREADTH, HEIGHT]),
        ("resized", DVec3::ZERO, [breadth, height]),
        (
            "resized and moved",
            DVec3::new(100.0, -40.0, 7.0),
            [breadth, height],
        ),
    ] {
        let part = block_at(origin, [LENGTH, breadth, height]);
        let edges = long_edges_of(breadth, height).map(|edge| edge.moved(origin));
        let laid = part.faces_end()..part.faces_end() + edges.len();

        let filleted = part
            .fillet_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the fillet {what}: declined, {declined:?}"));
        let chamfered = part
            .chamfer_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the chamfer {what}: declined, {declined:?}"));

        let fillet = format!("the fillet {what}");
        taken_as_asked(filleted.asked, filleted.taken, value, &fillet);
        let mut area = 0.0;
        for (edge, number) in edges.iter().zip(laid.clone()) {
            let round = edge.round(value);
            is_the_round(
                &filleted.body,
                number,
                &round,
                &format!(
                    "{fillet}: face {number}, the round between faces {:?}",
                    edge.faces
                ),
            );
            area += round.area();
        }
        sound_drawn_within(
            &filleted.body,
            LENGTH * breadth * height - 4.0 * LENGTH * value * value * (1.0 - PI / 4.0),
            DRAWN * reach_of(&part) * area,
            &fillet,
        );

        let chamfer = format!("the chamfer {what}");
        taken_as_asked(chamfered.asked, chamfered.taken, value, &chamfer);
        for (edge, number) in edges.iter().zip(laid) {
            let (corners, facing) = edge.flat(value);
            let which = format!(
                "{chamfer}: face {number}, the flat between faces {:?}",
                edge.faces
            );
            face_facing(&chamfered.body, &[number], facing, &which);
            corners_are(&chamfered.body, number, &corners, &which);
        }
        let promised = LENGTH * breadth * height - 4.0 * LENGTH * value * value / 2.0;
        sound_drawn_within(&chamfered.body, promised, EXACT * promised, &chamfer);
    }
}

#[test]
fn a_fillet_and_a_chamfer_on_a_list_of_one_edge_give_the_body_and_the_value_the_step_of_that_edge_gives()
 {
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let ell = raised_with(
        &Body::default(),
        Contour::straight(ELL_OUTLINE.to_vec()),
        ELL_HEIGHT,
    );
    let rabbeted = cut_from(
        &block,
        [DVec2::new(-1.0, -1.0), DVec2::new(LENGTH + 1.0, 2.0)],
        HEIGHT - 2.0,
        3.0,
    );
    let top_front = edge_between(&block, TOP, FRONT);
    let inner = edge_between(&ell, ARM_BACK, STEM_RIGHT);

    for (what, part, edge, value) in [
        (
            "the block's top front edge, a value it fits",
            &block,
            &top_front,
            3.0,
        ),
        (
            "the block's top front edge, reduced",
            &block,
            &top_front,
            50.0,
        ),
        ("the L's inner corner, a value it fits", &ell, &inner, 3.0),
        ("the L's inner corner, reduced", &ell, &inner, 50.0),
        (
            "the block's top front edge, gone under a rabbet",
            &rabbeted,
            &top_front,
            1.0,
        ),
        ("a value that is no length", &block, &top_front, -1.0),
    ] {
        let list = [edge.clone()];

        assert_eq!(
            part.fillet_edges(&list, value),
            part.fillet(edge, value),
            "the fillet on {what}, asked {value}"
        );
        assert_eq!(
            part.chamfer_edges(&list, value),
            part.chamfer(edge, value),
            "the chamfer on {what}, asked {value}"
        );
    }
}

/// How deep the stair's tread is, from its nose to its foot, and how high
/// it stands.
const TREAD: f64 = 10.0;
const TREAD_HEIGHT: f64 = 10.0;

/// The numbers the stair's riser, its tread and the tread's nose answer to:
/// the riser is the taller block's front, two; the tread and the nose are the
/// lower block's top and front, seven and eight, past the taller block's six.
/// Their floors, their rights and their lefts merge; the lower block's back,
/// against the riser, is gone.
const RISER: usize = FRONT;
const TREAD_TOP: usize = TOP + 6;
const NOSE: usize = FRONT + 6;
const STAIR_RIGHT: [u32; 2] = [RIGHT as u32, RIGHT as u32 + 6];
const STAIR_LEFT: [u32; 2] = [LEFT as u32, LEFT as u32 + 6];

/// One step of a stair: a block [`LENGTH`] long from y = 10 to 20 and
/// [`HEIGHT`] high, and in front of it a block from y = 0 to 10, as long and
/// [`TREAD_HEIGHT`] high, joined to it. The lower block's top is the tread,
/// [`TREAD`] deep: its nose meets outwards, along the front, and its foot
/// meets inwards, against the riser. The two face each other across it.
fn stair() -> Body {
    let taller = joined_to(
        &Body::default(),
        [DVec2::new(0.0, TREAD), DVec2::new(LENGTH, 2.0 * TREAD)],
        HEIGHT,
    );
    joined_to(
        &taller,
        [DVec2::ZERO, DVec2::new(LENGTH, TREAD)],
        TREAD_HEIGHT,
    )
}

/// The sides a face is expected to meet, given as the numbers each answers
/// to, in the order [`sides_met`] lists them.
fn sides_by(faces: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut sorted = faces.to_vec();
    sorted.sort();
    sorted
}

#[test]
fn a_fillet_on_a_tread_s_nose_and_foot_asked_more_than_half_its_depth_takes_half_a_round_and_a_hollow_meeting_in_an_s()
 {
    let stair = stair();
    let names = [
        edge_between(&stair, TREAD_TOP, NOSE),
        edge_between(&stair, RISER, TREAD_TOP),
    ];
    let half = TREAD / 2.0;
    let laid = [stair.faces_end(), stair.faces_end() + 1];

    for (asked, taken) in [(4.9, 4.9), (MORE_THAN_HALF, half)] {
        let filleted = stair
            .fillet_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!("the fillet on the tread's nose and foot, asked {asked}");
        reduced(filleted.asked, filleted.taken, [asked, taken], &what);
        let body = &filleted.body;
        let round = LongEdge {
            faces: [TREAD_TOP, NOSE],
            start: DVec3::new(0.0, 0.0, TREAD_HEIGHT),
            facing: [DVec3::Z, -DVec3::Y],
        }
        .round(taken);
        let hollow = Hollow {
            start: DVec3::new(0.0, TREAD - taken, TREAD_HEIGHT + taken),
            along: DVec3::X,
            length: LENGTH,
            radius: taken,
            facing: [-DVec3::Y, DVec3::Z],
        };
        is_the_round(
            body,
            laid[0],
            &round,
            &format!("{what}: face {}, the nose rounded off at {taken}", laid[0]),
        );
        is_the_hollow(
            body,
            laid[1],
            &hollow,
            &format!("{what}: face {}, the foot filled in at {taken}", laid[1]),
        );
        let ends = [STAIR_RIGHT.to_vec(), STAIR_LEFT.to_vec()];
        if taken < half {
            corners_are(
                body,
                TREAD_TOP,
                &strip(
                    DVec3::new(0.0, taken, TREAD_HEIGHT),
                    DVec3::new(0.0, TREAD - taken, TREAD_HEIGHT),
                ),
                &format!("{what}: what is left of the tread, a strip between the two"),
            );
            assert!(
                body.edge_names()
                    .iter()
                    .all(|name| !lies_between(name, laid[0], laid[1])),
                "{what}: the round and the hollow do not meet, the tread between them"
            );
        } else {
            assert!(!body.has_face(TREAD_TOP), "{what}: the tread is gone");
            edge_between(body, laid[0], laid[1]);
            assert_eq!(
                sides_met(body, laid[0]),
                sides_by(&[&ends[..], &[vec![NOSE as u32], vec![laid[1] as u32]]].concat()),
                "{what}: the round meets the nose's front, the hollow along one edge where the \
                 two are tangent, and the stair's two ends, by one edge each"
            );
            assert_eq!(
                sides_met(body, laid[1]),
                sides_by(&[&ends[..], &[vec![RISER as u32], vec![laid[0] as u32]]].concat()),
                "{what}: the hollow meets the riser, the round along one edge where the two are \
                 tangent, and the stair's two ends, by one edge each"
            );
        }
        corners_are(
            body,
            NOSE,
            &strip(DVec3::ZERO, DVec3::new(0.0, 0.0, TREAD_HEIGHT - taken)),
            &format!("{what}: what is left of the nose's front, under the round"),
        );
        corners_are(
            body,
            RISER,
            &strip(
                DVec3::new(0.0, TREAD, TREAD_HEIGHT + taken),
                DVec3::new(0.0, TREAD, HEIGHT),
            ),
            &format!("{what}: what is left of the riser, above the hollow"),
        );
        sound_drawn_within(
            body,
            LENGTH * TREAD * (HEIGHT + TREAD_HEIGHT),
            DRAWN * reach_of(&stair) * (round.area() + hollow.area()),
            &format!("{what}: as much matter as the nose gives up, the foot takes in"),
        );
    }
}

#[test]
fn a_chamfer_on_a_tread_s_nose_and_foot_asked_half_its_depth_or_more_is_declined_as_unsupported() {
    let stair = stair();
    let names = [
        edge_between(&stair, TREAD_TOP, NOSE),
        edge_between(&stair, RISER, TREAD_TOP),
    ];

    for asked in [TREAD / 2.0, MORE_THAN_HALF] {
        let declined = stair.chamfer_edges(&names, asked).err();

        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "asked {asked}: the nose's flat and the foot's would lie in one plane, one slope \
             from the nose's front to the riser, which #791 builds"
        );
    }
}

/// The numbers the cut that makes the widening slot gives the faces of its
/// floor's far edge: its far wall where the slot is 10 wide, twelve, and the
/// step where it widens, eleven.
const NARROW_FAR_WALL: usize = 12;
const WIDENING_STEP: usize = 11;

/// Where along the block the widening slot widens, and where its far wall
/// stands beyond.
const WIDENS_AT: f64 = 20.0;
const WIDENED_WALL: f64 = 25.0;

/// The slotted block's slot, 10 wide from the left wall to x = 20 and 15
/// wide from there to the right wall, cut by one outline: its near wall, at
/// y = 10, runs the block's whole length, and its floor's near edge ends on
/// the block's left wall and its right; its far wall, at y = 20, stops at the
/// step where the slot widens, and its floor's far edge ends on the left wall
/// and on that step.
fn widening_slot() -> Body {
    let solid = block([LENGTH, SLOTTED_BREADTH, HEIGHT]);
    let [near, far] = SLOT_WALLS;
    let tool = solid
        .tool_raised(
            &profile(Contour::straight(vec![
                DVec2::new(-1.0, near),
                DVec2::new(LENGTH + 1.0, near),
                DVec2::new(LENGTH + 1.0, WIDENED_WALL),
                DVec2::new(WIDENS_AT, WIDENED_WALL),
                DVec2::new(WIDENS_AT, far),
                DVec2::new(-1.0, far),
            ])),
            Frame {
                origin: DVec3::Z * SLOT_FLOOR_HEIGHT,
                ..ground()
            },
            DVec3::Z * HEIGHT,
        )
        .expect("the tool raises");
    solid.difference(&tool).expect("the slot is cut")
}

#[test]
fn a_fillet_on_the_two_floor_edges_of_a_slot_that_widens_past_the_end_of_one_is_declined_as_unsupported()
 {
    let slot = widening_slot();
    let names = [
        edge_between(&slot, SLOT_FLOOR, SLOT_NEAR_WALL),
        edge_between(&slot, SLOT_FLOOR, NARROW_FAR_WALL),
    ];
    let far_ends = &names[1].ends;
    assert!(
        far_ends
            .iter()
            .any(|end| end.contains(&vec![WIDENING_STEP as u32])),
        "the far floor edge ends on the step where the slot widens: {far_ends:?}"
    );
    let half = (SLOT_WALLS[1] - SLOT_WALLS[0]) / 2.0;

    for asked in [half, MORE_THAN_HALF] {
        for name in &names {
            let alone = slot
                .fillet(name, asked)
                .unwrap_or_else(|declined| panic!("one edge alone, asked {asked}: {declined:?}"));
            taken_as_asked(alone.asked, alone.taken, asked, "one floor edge alone");
        }

        let declined = slot.fillet_edges(&names, asked).err();

        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "asked {asked}: the two fills' set-backs would meet across the floor, but the edges \
             end on different faces, and no one hollow runs from end to end"
        );
    }
}

#[test]
fn a_chamfer_on_the_two_floor_edges_of_a_slot_that_widens_past_the_end_of_one_takes_5_a_valley_along_its_narrow_part()
 {
    let slot = widening_slot();
    let names = [
        edge_between(&slot, SLOT_FLOOR, SLOT_NEAR_WALL),
        edge_between(&slot, SLOT_FLOOR, NARROW_FAR_WALL),
    ];
    let [near, far] = SLOT_WALLS;
    let half = (far - near) / 2.0;
    let (middle, up) = (near + half, SLOT_FLOOR_HEIGHT + half);
    let laid = [slot.faces_end(), slot.faces_end() + 1];
    let at = |x: f64, y: f64, z: f64| DVec3::new(x, y, z);

    for asked in [half, MORE_THAN_HALF] {
        let chamfered = slot
            .chamfer_edges(&names, asked)
            .unwrap_or_else(|declined| panic!("asked {asked}: declined, {declined:?}"));

        let what = format!("the chamfer on both floor edges of the widening slot, asked {asked}");
        reduced(chamfered.asked, chamfered.taken, [asked, half], &what);
        let body = &chamfered.body;
        let mut kept_and_laid = numbers(&slot);
        kept_and_laid.extend(laid);
        assert_eq!(
            numbers(body),
            kept_and_laid,
            "{what}: every face of the slotted block, and a flat for each edge"
        );
        let floor = SLOT_FLOOR_HEIGHT;
        for (number, facing, corners, which) in [
            (
                laid[0],
                DVec3::new(0.0, 1.0, 1.0),
                vec![
                    at(0.0, near, up),
                    at(LENGTH, near, up),
                    at(LENGTH, middle, floor),
                    at(WIDENS_AT, middle, floor),
                    at(0.0, middle, floor),
                ],
                "the near flat, the block's whole length, down to the valley and then to the \
                 floor beyond the widening",
            ),
            (
                laid[1],
                DVec3::new(0.0, -1.0, 1.0),
                vec![
                    at(0.0, far, up),
                    at(WIDENS_AT, far, up),
                    at(WIDENS_AT, middle, floor),
                    at(0.0, middle, floor),
                ],
                "the far flat, from the left wall to the step where the slot widens",
            ),
        ] {
            let which = format!("{what}: face {number}, {which}");
            face_facing(body, &[number], facing.normalize(), &which);
            corners_are(body, number, &corners, &which);
        }
        edge_between(body, laid[0], laid[1]);
        assert_eq!(
            sides_met(body, laid[0]),
            sides(&[RIGHT, LEFT, SLOT_FLOOR, SLOT_NEAR_WALL, laid[1]]),
            "{what}: the near flat meets its wall, the block's two walls, the far flat along the \
             valley and the floor beyond, by one edge each"
        );
        assert_eq!(
            sides_met(body, laid[1]),
            sides(&[LEFT, WIDENING_STEP, NARROW_FAR_WALL, laid[0]]),
            "{what}: the far flat meets its wall, the left wall, the step at its end and the near \
             flat along the valley, by one edge each"
        );
        corners_are(
            body,
            SLOT_FLOOR,
            &[
                at(WIDENS_AT, middle, floor),
                at(LENGTH, middle, floor),
                at(LENGTH, WIDENED_WALL, floor),
                at(WIDENS_AT, WIDENED_WALL, floor),
            ],
            &format!("{what}: what is left of the floor, beyond the widening"),
        );
        corners_are(
            body,
            WIDENING_STEP,
            &[
                at(WIDENS_AT, middle, floor),
                at(WIDENS_AT, WIDENED_WALL, floor),
                at(WIDENS_AT, WIDENED_WALL, HEIGHT),
                at(WIDENS_AT, far, HEIGHT),
                at(WIDENS_AT, far, up),
            ],
            &format!("{what}: the step, holding the far flat's end flush"),
        );
        let [near_wall, _] = slot_walls_from(half);
        for (wall, corners, which) in [
            (
                SLOT_NEAR_WALL,
                near_wall.to_vec(),
                "the near wall, the block's whole length",
            ),
            (
                NARROW_FAR_WALL,
                vec![
                    at(0.0, far, up),
                    at(WIDENS_AT, far, up),
                    at(WIDENS_AT, far, HEIGHT),
                    at(0.0, far, HEIGHT),
                ],
                "the far wall, up to the widening",
            ),
        ] {
            corners_are(
                body,
                wall,
                &corners,
                &format!("{what}: what is left of {which}, above its flat"),
            );
        }
        for (wall, x, beyond) in [(LEFT, 0.0, far), (RIGHT, LENGTH, WIDENED_WALL)] {
            let mut corners = vec![
                at(x, 0.0, 0.0),
                at(x, SLOTTED_BREADTH, 0.0),
                at(x, SLOTTED_BREADTH, HEIGHT),
                at(x, beyond, HEIGHT),
                at(x, near, up),
                at(x, middle, floor),
                at(x, near, HEIGHT),
                at(x, 0.0, HEIGHT),
            ];
            corners.push(if x == 0.0 {
                at(x, far, up)
            } else {
                at(x, WIDENED_WALL, floor)
            });
            corners_are(
                body,
                wall,
                &corners,
                &format!("{what}: the wall at x = {x}, the slot's section on it"),
            );
        }
        let widening = (HEIGHT - SLOT_FLOOR_HEIGHT)
            * (WIDENS_AT * (far - near) + (LENGTH - WIDENS_AT) * (WIDENED_WALL - near));
        let promised =
            LENGTH * SLOTTED_BREADTH * HEIGHT - widening + half * half / 2.0 * (LENGTH + WIDENS_AT);
        sound_drawn_within(body, promised, EXACT * promised, &what);
    }
}
