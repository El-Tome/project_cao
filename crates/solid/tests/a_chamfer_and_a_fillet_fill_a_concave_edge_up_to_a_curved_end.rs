//! A chamfer and a fillet fill a concave straight edge in up to a curved end
//! face. #778 fills an edge between two planar faces only where each of its
//! ends lies on a flat face; where the face at an end is curved — a cylinder,
//! a cone, a sphere or a torus — the fill is trimmed flush on that face's own
//! surface: the face stays one face, its outline taking in the fill's
//! section, and nothing of the fill stands past it. That holds whichever way
//! the round turns: bulging out of the matter, as a rib's rounded end, or a
//! round wall the edge meets at a few degrees, as a slot's round end.
//!
//! Closes #788.
//! - A chamfer and a fillet at the foot of a rib whose end is a round bulging
//!   out of the matter, a cylinder, end flush on that round. The round stays
//!   one face and takes in the fill's section, the body is closed, sound and
//!   of the right volume, and it has one more face —
//!   `a_chamfer_at_the_foot_of_a_rib_whose_end_is_a_round_bulging_out_is_trimmed_flush_on_the_round`,
//!   `a_fillet_at_the_foot_of_a_rib_whose_end_is_a_round_bulging_out_is_trimmed_flush_on_the_round`
//! - The same holds where the edge ends on a round wall it meets at a few
//!   degrees, such as a slot's round end —
//!   `a_chamfer_at_the_foot_of_a_slot_s_wall_is_trimmed_flush_on_the_round_end_it_meets_at_a_few_degrees`,
//!   `a_fillet_at_the_foot_of_a_slot_s_wall_is_trimmed_flush_on_the_round_end_it_meets_at_a_few_degrees`
//! - The same holds where the end face is a cone, a sphere and a torus —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_running_into_a_cone_are_trimmed_flush_on_it`,
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_running_into_a_ball_are_trimmed_flush_on_its_sphere`,
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_running_into_a_ring_are_trimmed_flush_on_its_torus`
//! - An end face the fill cannot be trimmed flush by is declined
//!   `Unsupported` with that reason, and no wrong body is built —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_round_end_turns_away_short_of_the_fill_s_reach_are_declined_as_unsupported`
//! - After a change of size, a move or a turn upstream, the step fills the
//!   same edge, found by its name, flush on the same curved end face —
//!   `a_chamfer_and_a_fillet_fill_the_concave_edge_their_name_finds_again_flush_on_the_same_round_after_a_change_of_size_a_move_and_a_turn`
//! - The exact STEP export of a rib filled up to its round end writes every
//!   face on its true surface —
//!   `a_rib_filled_up_to_its_round_end_written_to_step_writes_every_face_on_its_true_surface`
//! - #778's tests and the convex chamfer's and fillet's tests still pass — no
//!   test: `a_chamfer_and_a_fillet_fill_a_concave_edge.rs`,
//!   `a_chamfer_cuts_one_straight_edge.rs`,
//!   `a_fillet_rounds_one_straight_edge.rs`, `src/body/tests/chamfer.rs` and
//!   `src/body/tests/fillet.rs` are the answer, left as they were but for the
//!   one test of #778 that declined every end on a round, which this issue
//!   turns round and this file now speaks for: the round it ended on turns
//!   away from the rib's side short of the fill's reach, and is the fourth
//!   criterion's

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
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

/// How far off the round a ray may meet it: on a triangle, within the
/// drawing's tolerance of it, which is taken at the reach the body carries —
/// the largest of the operands that built it, the fill's tools among them,
/// rather than its own box. A hundredth holds that at this part's size with
/// room to spare, and is under a tenth of how far a fill trimmed by the
/// round's tangent plane instead would stand off it halfway across its
/// section.
const MET: f64 = 1e-2;

/// The number a raised part's top answers to.
const TOP: usize = 1;

/// The numbers the faces of a rib joined on the [`plate`] answer to: its
/// top, its front, its right side, its back and its left side. Its floor,
/// six, sits on the plate's top and the join takes it in.
const RIB_TOP: usize = 7;
const RIB_FRONT: usize = 8;
const RIB_RIGHT: usize = 9;
const RIB_BACK: usize = 10;
const RIB_LEFT: usize = 11;

/// Where the steps that built the [`rib_rounded_in_front`] stopped numbering
/// faces: the plate's six, and the rib's floor, top and four walls.
const STEPS_END: usize = 12;

/// Where a rib on the plate stands: from x = 18 to its right side at 22, its
/// front corners at y = 5 and its back at 22, from the plate's top at z = 4
/// to 14.
const RIB_SIDE: f64 = 22.0;
const RIB_ENDS: [f64; 2] = [5.0, 22.0];
const PLATE: f64 = 4.0;

/// The upright line the [`rib_rounded_in_front`]'s round front turns about,
/// read in plan, and the round's radius: its front corners stand 2 across
/// from the line and 7.875 in front of it, and 8.125 is the length of that.
const AXIS: DVec2 = DVec2::new(20.0, 12.875);
const RADIUS: f64 = 8.125;

/// How many steps the matter of a fill is summed over by Simpson's rule: what
/// it sums is smooth, and this many come within a hair of the body's
/// tolerance.
const STEPS: usize = 1 << 12;

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

/// An outline drawn on a frame raised along its normal by `height`, and
/// joined as `cao_part` builds a step: its floor numbered nought, its top
/// one, and its walls from two on, one for each run of the outline.
fn raised_on(frame: Frame, outline: Contour, height: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(&profile(outline), frame, frame.normal() * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// A plate 40 long, 30 deep and `thickness` thick, raised from the ground
/// and numbered as [`raised_on`] numbers it.
fn plate(thickness: f64) -> Body {
    raised_on(
        ground(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(40.0, 30.0)),
        thickness,
    )
}

/// The part with an outline sketched on the plate's top, `thickness` up,
/// raised 10 and joined as a part's next step is: its floor, which the join
/// takes in, and its top numbered past the part's faces, then its walls.
fn on_the_plate(part: &Body, thickness: f64, outline: Contour) -> Body {
    let frame = Frame {
        origin: DVec3::Z * thickness,
        ..ground()
    };
    let tool = part
        .tool_raised(&profile(outline), frame, DVec3::Z * 10.0)
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
}

/// The turn the [`rib_rounded_in_front`]'s round makes from one front corner
/// to the other, about [`AXIS`].
fn round_turn() -> f64 {
    2.0 * (RIB_SIDE - AXIS.x).atan2(AXIS.y - RIB_ENDS[0])
}

/// The plate with a rib on its top, as [`on_the_plate`] joins it: 4 wide
/// from x = 18 to its right side at 22, its back at y = 22, and its front a
/// round through its front corners (18, 5) and (22, 5) about the upright
/// line through [`AXIS`], of [`RADIUS`], bulging a quarter forward of them
/// at its middle. The round meets the rib's right side some 14 degrees off
/// square, and reaches 6.125 past that side's plane, twice what a fill of 3
/// reaches: a round that turns away from the side within the fill's reach
/// has no point ahead of the fill's far side to trim it by, and is the
/// fourth criterion's, not this one's.
fn rib_rounded_in_front() -> Body {
    let rib = Contour {
        corners: vec![
            DVec2::new(18.0, RIB_ENDS[0]),
            DVec2::new(RIB_SIDE, RIB_ENDS[0]),
            DVec2::new(RIB_SIDE, RIB_ENDS[1]),
            DVec2::new(18.0, RIB_ENDS[1]),
        ],
        runs: vec![
            Run::Round {
                center: AXIS,
                turn: round_turn(),
            },
            Run::Straight,
            Run::Straight,
            Run::Straight,
        ],
    };
    on_the_plate(&plate(PLATE), PLATE, rib)
}

/// The matter the [`rib_rounded_in_front`] holds: the plate's 40 × 30 × 4,
/// and the rib's 4 × 17 with the segment of its round beyond the chord
/// between its front corners, r² (θ − sin θ) / 2, raised 10.
fn rounded_rib_volume() -> f64 {
    let turn = round_turn();
    4800.0 + (4.0 * 17.0 + RADIUS * RADIUS * (turn - turn.sin()) / 2.0) * 10.0
}

/// The area of the [`rib_rounded_in_front`]'s round: its arc, r θ, raised 10.
fn round_area() -> f64 {
    RADIUS * round_turn() * 10.0
}

/// Where the rib's round stands at `x`, read in plan: the y of its point
/// there on the side of its axis the rib's front corners are.
fn on_the_round(x: f64) -> f64 {
    AXIS.y - (RADIUS * RADIUS - (x - AXIS.x).powi(2)).sqrt()
}

/// How far a point stands from the round's axis, read in plan.
fn off_the_axis(point: DVec3) -> f64 {
    point.truncate().distance(AXIS)
}

/// The height a chamfer of `value` at the foot of the rib's right side stands
/// over the plate's top, `out` from that side.
fn chamfer_height(value: f64, out: f64) -> f64 {
    value - out
}

/// The height a fillet of `value` at the foot of the rib's right side stands
/// over the plate's top, `out` from that side: under its round, whose axis
/// stands `value` out from the side and `value` over the plate.
fn fillet_height(value: f64, out: f64) -> f64 {
    value - (value * value - (value - out).powi(2)).max(0.0).sqrt()
}

/// The matter a fill of `value` at the foot of the rib's right side holds,
/// from the rib's back to the round: over every place of its section, `out`
/// from the side and under `height(out)`, the length from the round to the
/// back, which the round, upright, makes a matter of `out` alone. Read with
/// `out = value (1 − cos φ)`, under which a fillet's height, `value (1 −
/// sin φ)`, is as smooth as a chamfer's, `value cos φ`; the product has no
/// closed form for the fillet, so it is summed by Simpson's rule.
fn fill_matter(value: f64, height: impl Fn(f64) -> f64) -> f64 {
    let at = |phi: f64| {
        let out = value * (1.0 - f64::cos(phi));
        let long = RIB_ENDS[1] - on_the_round(RIB_SIDE + out);
        height(out) * long * value * phi.sin()
    };
    let step = PI / 2.0 / STEPS as f64;
    let inner: f64 = (1..STEPS)
        .map(|index| at(step * index as f64) * if index % 2 == 1 { 4.0 } else { 2.0 })
        .sum();
    (at(0.0) + inner + at(PI / 2.0)) * step / 3.0
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
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

/// The sides of every face at a corner, ascending, as a name records them.
fn sides_of(faces: &[usize]) -> Vec<Vec<u32>> {
    let mut sides: Vec<Vec<u32>> = faces.iter().map(|&face| vec![face as u32]).collect();
    sides.sort();
    sides
}

/// Holds that the faces numbered `first` and `second` meet along one edge
/// alone, between the two corners each of `ends` names by the faces at it.
/// A name records at each end the faces met there other than its own two,
/// and those are what is compared.
fn one_edge_between(body: &Body, [first, second]: [usize; 2], ends: [&[usize]; 2], what: &str) {
    let (one, other) = (vec![first as u32], vec![second as u32]);
    let meeting: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between.contains(&one) && name.between.contains(&other))
        .collect();
    assert_eq!(
        meeting.len(),
        1,
        "{what}: faces {first} and {second} meet along one edge: {meeting:?}"
    );
    let mut found: Vec<Vec<Vec<u32>>> = meeting[0]
        .ends
        .iter()
        .map(|end| {
            let mut sides = end.clone();
            sides.sort();
            sides
        })
        .collect();
    found.sort();
    let mut expected: Vec<Vec<Vec<u32>>> = ends
        .iter()
        .map(|faces| {
            let besides: Vec<usize> = faces
                .iter()
                .copied()
                .filter(|face| ![first, second].contains(face))
                .collect();
            sides_of(&besides)
        })
        .collect();
    expected.sort();
    assert_eq!(
        found, expected,
        "{what}: the edge between faces {first} and {second} runs between the corners where \
         these faces meet"
    );
}

/// Holds that no corner of the body stands where the faces `faces` alone
/// meet.
fn no_corner_where_only(body: &Body, faces: &[usize], what: &str) {
    let sides = sides_of(faces);
    let left: Vec<_> = body
        .vertex_names()
        .into_iter()
        .filter(|vertex| {
            let mut met = vertex.faces.clone();
            met.sort();
            met == sides
        })
        .collect();
    assert!(
        left.is_empty(),
        "{what}: a corner is left where faces {faces:?} alone meet: {left:?}"
    );
}

/// Holds that a flat face's corners are the points expected, each of them a
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

/// Holds that a flat face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    corners_are_or_on(body, face, expected, |_| false, what);
}

/// Holds that the fill's face, numbered `made`, answers to the number after
/// the steps' last one, every face of the part keeping its own, and that the
/// next step numbers its faces past it.
fn numbered_after_the_steps_last(part: &Body, body: &Body, made: usize, what: &str) {
    assert_eq!(
        made, STEPS_END,
        "{what}: the fill's face answers to {STEPS_END}, where the steps' numbers ended"
    );
    let mut kept_and_made = numbers(part);
    kept_and_made.insert(STEPS_END);
    assert_eq!(
        numbers(body),
        kept_and_made,
        "{what}: every number the part's faces answered to, and the fill's, {STEPS_END}"
    );
    assert_eq!(
        body.faces_end(),
        STEPS_END + 1,
        "{what}: the fill names one face, and the next step numbers its own past it"
    );
    assert_eq!(
        body.pieces(),
        part.pieces(),
        "{what}: nothing of the fill's making is numbered as a piece"
    );
}

/// Holds that a body with curved faces is exact and keeps the rules every
/// solid must: its surface closed, no face through another, the matter it
/// holds exactly what arithmetic promised, and the matter its triangles
/// enclose within `drawn` of it.
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

/// What the triangles of a body a step left on `part` may enclose apart from
/// its matter, its curved faces `area` in all: the drawing's tolerance at the
/// part's own reach, over them.
fn drawn_within(part: &Body, area: f64) -> f64 {
    DRAWN * reach_of(part) * area
}

/// Holds that a step was given the value it was asked.
fn taken_whole(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// Holds that a step left every face of the part it was given and made one
/// face more, and gives back the number that face answers to.
fn one_face_more(part: &Body, body: &Body, what: &str) -> usize {
    let (before, after) = (numbers(part), numbers(body));
    assert!(
        before.is_subset(&after),
        "{what}: a face went: {before:?} became {after:?}"
    );
    let made: Vec<usize> = after.difference(&before).copied().collect();
    assert_eq!(
        made.len(),
        1,
        "{what}: one face more: {before:?} became {after:?}"
    );
    made[0]
}

/// Holds what [`one_face_more`] holds, the face made flat.
fn one_flat_face_more(part: &Body, body: &Body, what: &str) -> usize {
    let made = one_face_more(part, body, what);
    assert!(
        body.is_flat(made),
        "{what}: the face the chamfer lays is flat"
    );
    made
}

/// Holds what [`one_face_more`] holds, the face made curved.
fn one_round_more(part: &Body, body: &Body, what: &str) -> usize {
    let made = one_face_more(part, body, what);
    assert!(
        !body.is_flat(made) && body.plane_of(made).is_none(),
        "{what}: the face the fillet lays is curved, and offers no plane to draw on"
    );
    made
}

/// The furthest the triangles of the face numbered `face` reach along X.
fn furthest_along_x(body: &Body, face: usize) -> f64 {
    body.triangles_of(face)
        .flatten()
        .map(|corner| corner.x)
        .fold(f64::NEG_INFINITY, f64::max)
}

/// Holds that the face numbered `face` is the rib's round: curved, and every
/// corner of its triangles on the round's cylinder.
fn drawn_on_the_round(body: &Body, face: usize, what: &str) {
    assert!(
        body.has_face(face) && !body.is_flat(face),
        "{what}: face {face} is a round"
    );
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            (off_the_axis(corner) - RADIUS).abs() < PLACE,
            "{what}: the round is drawn through {corner}, {} from its axis through {AXIS}, \
             where its radius is {RADIUS}",
            off_the_axis(corner)
        );
    }
}

/// Holds that the [`rib_rounded_in_front`] is what the steps are given: its
/// faces numbered up to [`STEPS_END`]; its front the round about [`AXIS`],
/// stopping at the rib's right side, meeting the plate's top along one edge
/// from the rib's left side to its right, and the right side along one from
/// the plate's top to the rib's; and its matter the plate's and the rib's.
fn the_rib_is_rounded_in_front(part: &Body) {
    let what = "the rib rounded in front, before any step";
    assert_eq!(
        part.faces_end(),
        STEPS_END,
        "{what}: its steps numbered their faces up to {STEPS_END}"
    );
    drawn_on_the_round(part, RIB_FRONT, what);
    let furthest = furthest_along_x(part, RIB_FRONT);
    assert!(
        (furthest - RIB_SIDE).abs() < PLACE,
        "{what}: the round reaches {furthest} along X, where the rib's right side stands at \
         {RIB_SIDE}"
    );
    one_edge_between(
        part,
        [TOP, RIB_FRONT],
        [&[TOP, RIB_FRONT, RIB_LEFT], &[TOP, RIB_FRONT, RIB_RIGHT]],
        &format!("{what}: the round's foot"),
    );
    one_edge_between(
        part,
        [RIB_FRONT, RIB_RIGHT],
        [
            &[TOP, RIB_FRONT, RIB_RIGHT],
            &[RIB_TOP, RIB_FRONT, RIB_RIGHT],
        ],
        &format!("{what}: the round's edge with the rib's right side"),
    );
    sound_drawn_within(
        part,
        rounded_rib_volume(),
        drawn_within(part, round_area()),
        what,
    );
}

/// Holds what a fill of `value` at the foot of the rib's right side, its
/// section `height(out)` high `out` from that side, leaves where it meets the
/// rib's round front: the fill, the face numbered `made`, meeting the plate's
/// top and the rib's right side along its sides, and the round and the rib's
/// back at its ends, by one edge each; the round, one face still, drawn on
/// its cylinder alone and reaching out to the fill's far side; every ray
/// aimed along the edge from in front of the rib, through a place of the
/// fill's section, meeting the round first and on its cylinder; and no
/// corner of the fill in front of the round's axis standing outside the
/// round. And, the fill having covered the foot of the rib's right side, the
/// edges either side of the corner the foot ended at made one, as a flat end
/// makes them: the plate's top meeting the round along one edge, on the
/// round's foot circle from the rib's left side to the fill's far side, no
/// corner left where the two alone meet; the right side meeting the round
/// along one edge, from the fill's top up to the rib's; and the plate's top
/// one face round the rib and the fill's footprint, joined at the rib's back
/// as #778 joins it.
fn flush_on_the_round(
    body: &Body,
    made: usize,
    value: f64,
    height: impl Fn(f64) -> f64,
    what: &str,
) {
    let mut sides = [TOP, RIB_FRONT, RIB_RIGHT, RIB_BACK].map(|side| vec![side as u32]);
    sides.sort();
    assert_eq!(
        sides_met(body, made),
        sides,
        "{what}: the fill meets the plate's top and the rib's right side along its sides, and \
         the round and the rib's back at its ends, by one edge each"
    );
    one_edge_between(
        body,
        [TOP, RIB_FRONT],
        [&[TOP, RIB_FRONT, RIB_LEFT], &[TOP, RIB_FRONT, made]],
        &format!("{what}: the round's foot, run on along the fill's footprint"),
    );
    no_corner_where_only(
        body,
        &[TOP, RIB_FRONT],
        &format!("{what}: the corner the foot of the rib's right side ended at"),
    );
    one_edge_between(
        body,
        [RIB_FRONT, RIB_RIGHT],
        [
            &[RIB_FRONT, RIB_RIGHT, made],
            &[RIB_TOP, RIB_FRONT, RIB_RIGHT],
        ],
        &format!("{what}: the round's edge with the rib's right side, above the fill"),
    );
    let [front, back] = RIB_ENDS;
    let (far, up, top) = (RIB_SIDE + value, PLATE + value, PLATE + 10.0);
    corners_are(
        body,
        RIB_RIGHT,
        &[
            DVec3::new(RIB_SIDE, front, up),
            DVec3::new(RIB_SIDE, front, top),
            DVec3::new(RIB_SIDE, back, up),
            DVec3::new(RIB_SIDE, back, top),
        ],
        &format!("{what}: what is left of the rib's right side"),
    );
    corners_are_or_on(
        body,
        TOP,
        &[
            DVec3::new(0.0, 0.0, PLATE),
            DVec3::new(40.0, 0.0, PLATE),
            DVec3::new(40.0, 30.0, PLATE),
            DVec3::new(0.0, 30.0, PLATE),
            DVec3::new(18.0, front, PLATE),
            DVec3::new(far, on_the_round(far), PLATE),
            DVec3::new(far, back, PLATE),
            DVec3::new(18.0, back, PLATE),
        ],
        |corner| {
            (off_the_axis(corner) - RADIUS).abs() < PLACE
                && (corner.z - PLATE).abs() < PLACE
                && (18.0 - PLACE..=far + PLACE).contains(&corner.x)
                && corner.y < AXIS.y
        },
        &format!(
            "{what}: the plate's top, one face round the rib and the fill's footprint, its edge \
             with the round on the round's foot circle"
        ),
    );
    drawn_on_the_round(body, RIB_FRONT, what);
    let furthest = furthest_along_x(body, RIB_FRONT);
    assert!(
        (furthest - far).abs() < PLACE,
        "{what}: the round reaches {furthest} along X, where it takes in the fill's section out \
         to {far}"
    );
    for share_out in [0.2, 0.5] {
        let out = value * share_out;
        for share_up in [0.25, 0.5, 0.75] {
            let from = DVec3::new(RIB_SIDE + out, -1.0, PLATE + height(out) * share_up);
            let hit = body.ray_hit(from, DVec3::Y).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} through the fill's section meets nothing")
            });
            let at = from + DVec3::Y * hit.distance;
            assert_eq!(
                hit.face,
                RIB_FRONT,
                "{what}: a ray from {from} through the fill's section meets face {} at {at}, \
                 before the round, which stands at y = {}",
                hit.face,
                on_the_round(from.x)
            );
            assert!(
                (off_the_axis(at) - RADIUS).abs() <= MET && at.y < AXIS.y,
                "{what}: a ray from {from} meets the round at {at}, off its cylinder, which \
                 stands at y = {} there",
                on_the_round(from.x)
            );
        }
    }
    for corner in body.triangles_of(made).flatten() {
        assert!(
            corner.y >= AXIS.y || off_the_axis(corner) <= RADIUS + PLACE,
            "{what}: the fill is drawn through {corner}, {} from the round's axis: past the \
             round, whose radius is {RADIUS}",
            off_the_axis(corner)
        );
    }
}

#[test]
fn a_chamfer_at_the_foot_of_a_rib_whose_end_is_a_round_bulging_out_is_trimmed_flush_on_the_round() {
    let value = 3.0;
    let part = rib_rounded_in_front();
    the_rib_is_rounded_in_front(&part);
    let foot = edge_between(&part, TOP, RIB_RIGHT);

    let chamfered = part.chamfer(&foot, value).unwrap_or_else(|declined| {
        panic!("the chamfer at the foot of the rib rounded in front: declined, {declined:?}")
    });

    let what = "the chamfer at the foot of the rib rounded in front";
    taken_whole(chamfered.asked, chamfered.taken, value, what);
    let body = &chamfered.body;
    let height = |out: f64| chamfer_height(value, out);
    sound_drawn_within(
        body,
        rounded_rib_volume() + fill_matter(value, height),
        drawn_within(&part, round_area() + value * value),
        what,
    );
    let chamfer = one_flat_face_more(&part, body, what);
    numbered_after_the_steps_last(&part, body, chamfer, what);
    flush_on_the_round(body, chamfer, value, height, what);
}

#[test]
fn a_fillet_at_the_foot_of_a_rib_whose_end_is_a_round_bulging_out_is_trimmed_flush_on_the_round() {
    let value = 3.0;
    let part = rib_rounded_in_front();
    the_rib_is_rounded_in_front(&part);
    let foot = edge_between(&part, TOP, RIB_RIGHT);

    let filleted = part.fillet(&foot, value).unwrap_or_else(|declined| {
        panic!("the fillet at the foot of the rib rounded in front: declined, {declined:?}")
    });

    let what = "the fillet at the foot of the rib rounded in front";
    taken_whole(filleted.asked, filleted.taken, value, what);
    let body = &filleted.body;
    let height = |out: f64| fillet_height(value, out);
    let long = RIB_ENDS[1] - RIB_ENDS[0];
    sound_drawn_within(
        body,
        rounded_rib_volume() + fill_matter(value, height),
        drawn_within(
            &part,
            round_area() + value * value + PI / 2.0 * value * long,
        ),
        what,
    );
    let fillet = one_round_more(&part, body, what);
    numbered_after_the_steps_last(&part, body, fillet, what);
    flush_on_the_round(body, fillet, value, height, what);
}
