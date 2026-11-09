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
//!   answered for the cylinder above and for the cone, a rib whose round end
//!   has draft:
//!   `a_chamfer_at_the_foot_of_a_rib_whose_round_end_has_draft_is_trimmed_flush_on_its_cone`,
//!   `a_fillet_at_the_foot_of_a_rib_whose_round_end_has_draft_is_trimmed_flush_on_its_cone`.
//!   The sphere and the torus left this issue for #805 (the human's decision):
//!   the kernel meets neither with a slanted plane or a cylinder off its axis,
//!   so a fill ending on one stays declined here, as the next criterion pins
//! - An end face the fill cannot be trimmed flush by is declined
//!   `Unsupported` with that reason, and no wrong body is built — a round the
//!   edge runs out of that turns away short of the fill's reach,
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_round_end_turns_away_short_of_the_fill_s_reach_are_declined_as_unsupported`;
//!   a round the edge runs into,
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_slot_s_wall_running_into_its_round_end_are_declined_as_unsupported`;
//!   and a sphere, #805's, here a rib cut from a ball —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_end_is_a_sphere_are_declined_as_unsupported`
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

mod step_reader;

use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Declined, EdgeName, Loop, step};
use glam::{DQuat, DVec2, DVec3};

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

/// Where the steps that built the [`rib_rounded_in_front`] and the
/// [`slot_in_the_plate`] stopped numbering faces: the plate's six, and the
/// floor, the top and the four walls of the rib joined on it or of the slot
/// cut from it.
const STEPS_END: usize = 12;

/// The numbers the faces of the [`slot_in_the_plate`] answer to: the plate's
/// left side, which the slot runs out through; and the cut's floor, which is
/// the slot's, its front wall, its round end and its back wall. The cut's top,
/// seven, and its left end, eleven, stand past the plate and leave nothing.
const PLATE_LEFT: usize = 5;
const SLOT_FLOOR: usize = 6;
const SLOT_FRONT: usize = 8;
const SLOT_END: usize = 9;
const SLOT_BACK: usize = 10;

/// Where the [`slot_in_the_plate`] stands: its front wall at y = 10, 4 in
/// front of its back, ending 28 along; its floor at z = 4, in a plate 10
/// thick.
const SLOT_FRONT_Y: f64 = 10.0;
const SLOT_FRONT_END: f64 = 28.0;
const SLOT_FLOOR_Z: f64 = 4.0;
const SLOT_PLATE: f64 = 10.0;

/// How far the [`slot_in_the_plate`]'s round end stands off tangent to its
/// front: its axis a tenth back along the front from where a tangent round's
/// would stand, so that the round leaves the front's end turning in toward
/// the slot by some 2.9 degrees.
const SLOT_OFF: f64 = -0.1;

/// How far short of the slot's round end, along the slot, a ray is aimed
/// down onto a fill there: the round meets that line slantwise, so a tenth
/// along it stands four to six hundredths off the round, clear of the sag
/// of the round's triangles toward the slot, and past where the front ends
/// at either place a ray is aimed.
const SHORT_OF_THE_END: f64 = 0.1;

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

/// How many steps the matter of a fill is summed over by Simpson's rule, out
/// from its wall and up its height: what it sums is smooth, and this many
/// come within a hair of the body's tolerance.
const STEPS: usize = 1 << 12;
const UP_STEPS: usize = 1 << 8;

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

/// How long a fill at the foot of the rib's right side runs, `out` from that
/// side: from the round to the rib's back.
fn along_the_rib(out: f64) -> f64 {
    RIB_ENDS[1] - on_the_round(RIB_SIDE + out)
}

/// The height a chamfer of `value` in a corner stands over the face it lies
/// on, `out` from the wall it stands against: the rib's right side over the
/// plate's top, the slot's front over its floor.
fn chamfer_height(value: f64, out: f64) -> f64 {
    value - out
}

/// The height a fillet of `value` in a corner stands over the face it lies
/// on, `out` from the wall it stands against: under its round, whose axis
/// stands `value` out from the wall and `value` over the face.
fn fillet_height(value: f64, out: f64) -> f64 {
    value - (value * value - (value - out).powi(2)).max(0.0).sqrt()
}

/// What `at` sums to from `from` to `to`, by Simpson's rule over `steps`
/// steps.
fn simpson(at: impl Fn(f64) -> f64, [from, to]: [f64; 2], steps: usize) -> f64 {
    let step = (to - from) / steps as f64;
    let inner: f64 = (1..steps)
        .map(|index| at(from + step * index as f64) * if index % 2 == 1 { 4.0 } else { 2.0 })
        .sum();
    (at(from) + inner + at(to)) * step / 3.0
}

/// The matter a fill of `value` holds: over every place of its section,
/// `out` from the wall it stands against and `up` over the face it lies on,
/// under `height(out)`, the length it runs from one end to the other,
/// `long(out, up)` — a matter of `out` alone where the round it ends on is
/// an upright cylinder, of both where it is a cone. Read with `out = value
/// (1 − cos φ)`, under which a fillet's height, `value (1 − sin φ)`, is as
/// smooth as a chamfer's, `value cos φ`; the product has no closed form for
/// the fillet, so it is summed by Simpson's rule, up each height and then
/// out across the section.
fn fill_matter(value: f64, height: impl Fn(f64) -> f64, long: impl Fn(f64, f64) -> f64) -> f64 {
    let across = |phi: f64| {
        let out = value * (1.0 - f64::cos(phi));
        let up = simpson(|up| long(out, up), [0.0, height(out)], UP_STEPS);
        up * value * phi.sin()
    };
    simpson(across, [0.0, PI / 2.0], STEPS)
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
    let steps_end = part.faces_end();
    assert_eq!(
        made, steps_end,
        "{what}: the fill's face answers to {steps_end}, where the steps' numbers ended"
    );
    let mut kept_and_made = numbers(part);
    kept_and_made.insert(steps_end);
    assert_eq!(
        numbers(body),
        kept_and_made,
        "{what}: every number the part's faces answered to, and the fill's, {steps_end}"
    );
    assert_eq!(
        body.faces_end(),
        steps_end + 1,
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

/// Holds that the face numbered `face` is a round: curved, and every corner
/// of its triangles on the upright cylinder about `axis`, read in plan, of
/// `radius`.
fn drawn_on_the_cylinder(body: &Body, face: usize, (axis, radius): (DVec2, f64), what: &str) {
    assert!(
        body.has_face(face) && !body.is_flat(face),
        "{what}: face {face} is a round"
    );
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        let off = corner.truncate().distance(axis);
        assert!(
            (off - radius).abs() < PLACE,
            "{what}: the round is drawn through {corner}, {off} from its axis through {axis}, \
             where its radius is {radius}"
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
    drawn_on_the_cylinder(part, RIB_FRONT, (AXIS, RADIUS), what);
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
    drawn_on_the_cylinder(body, RIB_FRONT, (AXIS, RADIUS), what);
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
        rounded_rib_volume() + fill_matter(value, height, |out, _| along_the_rib(out)),
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
        rounded_rib_volume() + fill_matter(value, height, |out, _| along_the_rib(out)),
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

/// The part with an outline drawn on the ground, raised from `from` up by
/// `height`, cut from it as `cao_part` cuts a step: the tool's faces numbered
/// past the part's.
fn cut_from(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    let frame = Frame {
        origin: DVec3::Z * from,
        ..ground()
    };
    let tool = part
        .tool_raised(&profile(outline), frame, DVec3::Z * height)
        .expect("the tool raises");
    part.difference(&tool).expect("the cut is built")
}

/// The upright line the [`slot_in_the_plate`]'s round end turns about, read
/// in plan, and the round's radius, from that line to the front's end.
fn slot_axis() -> DVec2 {
    slot_axis_off_by(SLOT_OFF)
}

fn slot_radius() -> f64 {
    slot_radius_off_by(SLOT_OFF)
}

/// The upright line the round end of a slot off tangent by `off` turns
/// about, and its radius: the axis `off` along the front from where a
/// tangent round's would stand.
fn slot_axis_off_by(off: f64) -> DVec2 {
    DVec2::new(SLOT_FRONT_END + off, SLOT_FRONT_Y + 2.0)
}

fn slot_radius_off_by(off: f64) -> f64 {
    (4.0 + off * off).sqrt()
}

/// A plate 40 long, 30 deep and 10 thick with a slot cut into its top as a
/// part's next step cuts it: 4 wide, its floor 6 under the plate's top, it
/// runs out through the plate's left side and ends, 28 along its front, in a
/// round end of half a turn about [`slot_axis`] — the convex fillet's slot
/// off tangent by −0.1, cut as a pocket. The round leaves the front's end at
/// a corner of some 2.9 degrees, not tangentially, which would be #698's
/// chain, and turning in toward the slot, so that it runs on across the foot
/// of the front, where a fill there stands. The matter lies outside its
/// cylinder.
fn slot_in_the_plate() -> Body {
    slot_off_tangent_by(SLOT_OFF)
}

/// The [`slot_in_the_plate`] with its round end's axis moved `off` along the
/// front from where a tangent round's would stand.
fn slot_off_tangent_by(off: f64) -> Body {
    let (end, centre) = (
        DVec2::new(SLOT_FRONT_END, SLOT_FRONT_Y),
        slot_axis_off_by(off),
    );
    let slot = Contour {
        corners: vec![
            DVec2::new(-5.0, SLOT_FRONT_Y),
            end,
            2.0 * centre - end,
            DVec2::new(-5.0, SLOT_FRONT_Y + 4.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: centre,
                turn: PI,
            },
            Run::Straight,
            Run::Straight,
        ],
    };
    cut_from(&plate(SLOT_PLATE), slot, SLOT_FLOOR_Z, SLOT_PLATE)
}

/// The matter the [`slot_in_the_plate`] holds: the plate's 40 × 30 × 10, less
/// the slot's, 6 deep — the quadrilateral from the plate's left side to the
/// round's chord, and the half disc beyond the chord.
fn slot_part_volume() -> f64 {
    slot_volume_off_by(SLOT_OFF)
}

/// The matter a slot off tangent by `off` leaves the plate, as
/// [`slot_part_volume`] reckons it.
fn slot_volume_off_by(off: f64) -> f64 {
    let back_end = SLOT_FRONT_END + 2.0 * off;
    let plan = (SLOT_FRONT_END + back_end) / 2.0 * 4.0 + PI * slot_radius_off_by(off).powi(2) / 2.0;
    40.0 * 30.0 * SLOT_PLATE - plan * (SLOT_PLATE - SLOT_FLOOR_Z)
}

/// The area of the [`slot_in_the_plate`]'s round end: half a turn of it, 6
/// high.
fn slot_end_area() -> f64 {
    PI * slot_radius() * (SLOT_PLATE - SLOT_FLOOR_Z)
}

/// Where the slot's round end stands at `y`, read in plan: the x of its point
/// there past its axis.
fn on_the_slot_s_end(y: f64) -> f64 {
    slot_axis().x + (slot_radius().powi(2) - (y - slot_axis().y).powi(2)).sqrt()
}

/// How long a fill at the foot of the slot's front runs, `out` from it: from
/// the plate's left side to the slot's round end.
fn along_the_slot(out: f64) -> f64 {
    on_the_slot_s_end(SLOT_FRONT_Y + out)
}

/// Holds that the [`slot_in_the_plate`] is what the steps are given: its
/// faces numbered up to [`STEPS_END`]; its round end on the cylinder about
/// [`slot_axis`]; the foot of its front one edge, from the plate's left side
/// to the round; the round's foot one edge, from the front to the back, and
/// its edge with the front one, from the floor to the plate's top; the
/// plate's left side notched by the slot; and its matter the plate's less the
/// slot's.
fn the_slot_is_cut_in_the_plate(part: &Body) {
    let what = "the slot cut in the plate, before any step";
    assert_eq!(
        part.faces_end(),
        STEPS_END,
        "{what}: its steps numbered their faces up to {STEPS_END}"
    );
    for flat in [PLATE_LEFT, SLOT_FLOOR, SLOT_FRONT, SLOT_BACK] {
        assert!(
            part.has_face(flat) && part.is_flat(flat),
            "{what}: face {flat} is flat"
        );
    }
    drawn_on_the_cylinder(part, SLOT_END, (slot_axis(), slot_radius()), what);
    one_edge_between(
        part,
        [SLOT_FLOOR, SLOT_FRONT],
        [
            &[SLOT_FLOOR, SLOT_FRONT, PLATE_LEFT],
            &[SLOT_FLOOR, SLOT_FRONT, SLOT_END],
        ],
        &format!("{what}: the foot of the slot's front"),
    );
    one_edge_between(
        part,
        [SLOT_FLOOR, SLOT_END],
        [
            &[SLOT_FLOOR, SLOT_END, SLOT_FRONT],
            &[SLOT_FLOOR, SLOT_END, SLOT_BACK],
        ],
        &format!("{what}: the round's foot"),
    );
    one_edge_between(
        part,
        [SLOT_FRONT, SLOT_END],
        [
            &[SLOT_FRONT, SLOT_END, SLOT_FLOOR],
            &[SLOT_FRONT, SLOT_END, TOP],
        ],
        &format!("{what}: the round's edge with the front"),
    );
    let (front, back, floor) = (SLOT_FRONT_Y, SLOT_FRONT_Y + 4.0, SLOT_FLOOR_Z);
    corners_are(
        part,
        PLATE_LEFT,
        &[
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 30.0, 0.0),
            DVec3::new(0.0, 30.0, SLOT_PLATE),
            DVec3::new(0.0, back, SLOT_PLATE),
            DVec3::new(0.0, back, floor),
            DVec3::new(0.0, front, floor),
            DVec3::new(0.0, front, SLOT_PLATE),
            DVec3::new(0.0, 0.0, SLOT_PLATE),
        ],
        &format!("{what}: the plate's left side, notched by the slot"),
    );
    sound_drawn_within(
        part,
        slot_part_volume(),
        drawn_within(part, slot_end_area()),
        what,
    );
}

/// Holds what a fill of `value` at the foot of the slot's front, its section
/// `height(out)` high `out` from the front, leaves: the fill, the face
/// numbered `made`, meeting the floor and the front along its sides, and the
/// plate's left side and the slot's round end at its ends, by one edge each;
/// the left side flush on the fill's end, as #778 trims it, its notch taking
/// in the fill's section — its corners on the fill, `on_the_fill` says,
/// besides; the front cut short over the fill; the round end, one face still,
/// drawn on its cylinder alone, no corner of it under the fill's surface and
/// its corners where the fill's sides reach it; the fill running on to the
/// round end, every ray down onto it a little short of the round meeting it, and
/// nothing of it standing past the round, beyond its cylinder. And where the
/// foot of the front ended, at the round, nothing left: the fill covers that
/// corner, so the round's foot and its edge with the front, run short to the
/// fill, are one edge each, and no corner stands there.
fn flush_on_the_slot_s_end(
    body: &Body,
    made: usize,
    value: f64,
    height: impl Fn(f64) -> f64,
    on_the_fill: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let mut sides = [PLATE_LEFT, SLOT_FLOOR, SLOT_FRONT, SLOT_END].map(|side| vec![side as u32]);
    sides.sort();
    assert_eq!(
        sides_met(body, made),
        sides,
        "{what}: the fill meets the floor and the front along its sides, and the plate's left \
         side and the round end at its ends, by one edge each"
    );
    let (front, back, floor) = (SLOT_FRONT_Y, SLOT_FRONT_Y + 4.0, SLOT_FLOOR_Z);
    let (out, up) = (front + value, floor + value);
    let far = on_the_slot_s_end(out);
    one_edge_between(
        body,
        [SLOT_FLOOR, SLOT_END],
        [
            &[SLOT_FLOOR, SLOT_END, made],
            &[SLOT_FLOOR, SLOT_END, SLOT_BACK],
        ],
        &format!("{what}: the round's foot, run short to the fill"),
    );
    one_edge_between(
        body,
        [SLOT_FRONT, SLOT_END],
        [&[SLOT_FRONT, SLOT_END, made], &[SLOT_FRONT, SLOT_END, TOP]],
        &format!("{what}: the round's edge with the front, run short to the fill"),
    );
    for faces in [
        &[SLOT_FLOOR, SLOT_FRONT, SLOT_END][..],
        &[SLOT_FLOOR, SLOT_END],
        &[SLOT_FRONT, SLOT_END],
    ] {
        no_corner_where_only(
            body,
            faces,
            &format!("{what}: the corner the foot of the front ended at"),
        );
    }
    corners_are_or_on(
        body,
        PLATE_LEFT,
        &[
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 30.0, 0.0),
            DVec3::new(0.0, 30.0, SLOT_PLATE),
            DVec3::new(0.0, back, SLOT_PLATE),
            DVec3::new(0.0, back, floor),
            DVec3::new(0.0, out, floor),
            DVec3::new(0.0, front, up),
            DVec3::new(0.0, front, SLOT_PLATE),
            DVec3::new(0.0, 0.0, SLOT_PLATE),
        ],
        &on_the_fill,
        &format!("{what}: the plate's left side, its notch taking in the fill's section, flush"),
    );
    corners_are(
        body,
        SLOT_FRONT,
        &[
            DVec3::new(0.0, front, up),
            DVec3::new(SLOT_FRONT_END, front, up),
            DVec3::new(SLOT_FRONT_END, front, SLOT_PLATE),
            DVec3::new(0.0, front, SLOT_PLATE),
        ],
        &format!("{what}: what is left of the slot's front"),
    );
    corners_are_or_on(
        body,
        SLOT_FLOOR,
        &[
            DVec3::new(0.0, out, floor),
            DVec3::new(far, out, floor),
            DVec3::new(SLOT_FRONT_END + 2.0 * SLOT_OFF, back, floor),
            DVec3::new(0.0, back, floor),
        ],
        |corner| {
            (corner.truncate().distance(slot_axis()) - slot_radius()).abs() < PLACE
                && (corner.z - floor).abs() < PLACE
                && corner.y >= out - PLACE
        },
        &format!(
            "{what}: the slot's floor, one face beside the fill's footprint, its edge with the \
             round on the round's foot circle"
        ),
    );
    let axis_and_radius = (slot_axis(), slot_radius());
    drawn_on_the_cylinder(body, SLOT_END, axis_and_radius, what);
    let round: Vec<DVec3> = body.triangles_of(SLOT_END).flatten().collect();
    for corner in &round {
        let off_the_front = corner.y - front;
        let under = if off_the_front < value {
            floor + height(off_the_front.max(0.0))
        } else {
            floor
        };
        assert!(
            corner.z >= under - PLACE,
            "{what}: the round end is drawn through {corner}, under the fill's surface at {under}: \
             it still stands where the fill covers it"
        );
    }
    for corner in [
        DVec3::new(SLOT_FRONT_END, front, up),
        DVec3::new(far, out, floor),
    ] {
        assert!(
            round.iter().any(|drawn| drawn.distance(corner) < PLACE),
            "{what}: the round end has no corner at {corner}, where a side of the fill reaches it"
        );
    }
    let furthest = furthest_along_x(body, made);
    assert!(
        (furthest - far).abs() < PLACE,
        "{what}: the fill runs on to {furthest}, where its side on the floor meets the round end \
         at {far}"
    );
    for share_out in [0.2, 0.5] {
        let off_the_front = value * share_out;
        let y = front + off_the_front;
        let from = DVec3::new(on_the_slot_s_end(y) - SHORT_OF_THE_END, y, SLOT_PLATE - 0.5);
        let hit = body.ray_hit(from, -DVec3::Z).unwrap_or_else(|| {
            panic!("{what}: a ray down from {from}, short of the round end, meets nothing")
        });
        let at = from - DVec3::Z * hit.distance;
        let surface = floor + height(off_the_front);
        assert!(
            hit.face == made && (at.z - surface).abs() <= MET,
            "{what}: a ray down from {from}, short of the round end, meets face {} at {at}, \
             where the fill's surface stands at {surface}",
            hit.face
        );
    }
    for corner in body.triangles_of(made).flatten() {
        let off = corner.truncate().distance(slot_axis());
        assert!(
            corner.x <= slot_axis().x || off <= slot_radius() + PLACE,
            "{what}: the fill is drawn through {corner}, {off} from the round's axis: past the \
             round end, whose radius is {}",
            slot_radius()
        );
    }
}

#[test]
fn a_chamfer_at_the_foot_of_a_slot_s_wall_is_trimmed_flush_on_the_round_end_it_meets_at_a_few_degrees()
 {
    let value = 1.0;
    let part = slot_in_the_plate();
    the_slot_is_cut_in_the_plate(&part);
    let foot = edge_between(&part, SLOT_FLOOR, SLOT_FRONT);

    let chamfered = part.chamfer(&foot, value).unwrap_or_else(|declined| {
        panic!("the chamfer at the foot of the slot's front: declined, {declined:?}")
    });

    let what = "the chamfer at the foot of the slot's front";
    taken_whole(chamfered.asked, chamfered.taken, value, what);
    let body = &chamfered.body;
    let height = |out: f64| chamfer_height(value, out);
    sound_drawn_within(
        body,
        slot_part_volume() + fill_matter(value, height, |out, _| along_the_slot(out)),
        drawn_within(&part, slot_end_area() + value * value),
        what,
    );
    let chamfer = one_flat_face_more(&part, body, what);
    numbered_after_the_steps_last(&part, body, chamfer, what);
    flush_on_the_slot_s_end(body, chamfer, value, height, |_| false, what);
}

#[test]
fn a_fillet_at_the_foot_of_a_slot_s_wall_is_trimmed_flush_on_the_round_end_it_meets_at_a_few_degrees()
 {
    let value = 1.0;
    let part = slot_in_the_plate();
    the_slot_is_cut_in_the_plate(&part);
    let foot = edge_between(&part, SLOT_FLOOR, SLOT_FRONT);

    let filleted = part.fillet(&foot, value).unwrap_or_else(|declined| {
        panic!("the fillet at the foot of the slot's front: declined, {declined:?}")
    });

    let what = "the fillet at the foot of the slot's front";
    taken_whole(filleted.asked, filleted.taken, value, what);
    let body = &filleted.body;
    let height = |out: f64| fillet_height(value, out);
    let long = along_the_slot(value);
    sound_drawn_within(
        body,
        slot_part_volume() + fill_matter(value, height, |out, _| along_the_slot(out)),
        drawn_within(
            &part,
            slot_end_area() + value * value + PI / 2.0 * value * long,
        ),
        what,
    );
    let fillet = one_round_more(&part, body, what);
    numbered_after_the_steps_last(&part, body, fillet, what);
    let axis = DVec2::new(SLOT_FRONT_Y + value, SLOT_FLOOR_Z + value);
    flush_on_the_slot_s_end(
        body,
        fillet,
        value,
        height,
        |corner| {
            corner.x.abs() < PLACE
                && (DVec2::new(corner.y, corner.z).distance(axis) - value).abs() < PLACE
        },
        what,
    );
}

/// The numbers the [`rib_drafted_in_front`]'s faces past the rib's walls
/// answer to: its front, the cone the ring's inner wall leaves, numbered
/// first of the ring's four; and where the steps that built it stopped
/// numbering, past the ring's. The rib's own front, eight, is cut away whole,
/// and the ring leaves nothing else.
const CONE_FRONT: usize = 12;
const CONE_STEPS_END: usize = 16;

/// How far the [`rib_drafted_in_front`]'s round front draws in toward its
/// axis for every unit up: a draft of some 5.7 degrees.
const DRAFT: f64 = 0.1;

/// Where the [`rib_drafted_in_front`]'s back stands: near enough its round's
/// axis that a ring turned whole about it leaves the back's corners alone,
/// some 0.7 inside the cone at the rib's top.
const DRAFTED_BACK: f64 = 19.0;

/// The radius of the [`rib_drafted_in_front`]'s cone at height `z`: the
/// round of criterion 1 at the plate's top, drawn in by the draft above it.
fn cone_radius(z: f64) -> f64 {
    RADIUS - DRAFT * (z - PLATE)
}

/// Where the drafted rib's cone stands at `x` and height `z`: the y of its
/// point there on the side of its axis the rib's front corners are.
fn on_the_cone(x: f64, z: f64) -> f64 {
    AXIS.y - (cone_radius(z).powi(2) - (x - AXIS.x).powi(2)).sqrt()
}

/// How far a point stands off the drafted rib's cone, out from its axis
/// across its height.
fn off_the_cone(point: DVec3) -> f64 {
    off_the_axis(point) - cone_radius(point.z)
}

/// An outline drawn on the plane through the upright line through [`AXIS`]
/// that runs along X, out along X and up along Z, turned whole about that
/// line, as `cao_part` turns a step: its faces, one for each run, numbered
/// past `part`'s when the boolean that takes it numbers them.
fn turned_about_the_round_s_axis(part: &Body, contour: Contour) -> Body {
    let sampled: &'static [DVec2] = Box::leak(contour.corners.clone().into_boxed_slice());
    let outline = Profile {
        exact: Some((contour, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: AXIS.extend(0.0),
        u: DVec3::X,
        v: DVec3::Z,
    };
    let upright = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    part.tool_turned(&outline, frame, &Turn::of(upright, TAU, 1e-4, &outline))
        .expect("the ring turns")
}

/// The plate with a rib on its top whose round end has draft: a rib 4 wide
/// from x = 18 to its right side at 22, from y = 4 to its back at 19, raised
/// 10, its front then cut by a ring turned whole about the upright line
/// through [`AXIS`], whose inner wall is a cone of [`cone_radius`] — the
/// round of criterion 1 at the plate's top, drawing in by [`DRAFT`] above it
/// — before the rib is joined to the plate. Its front is then that cone: it
/// meets the plate's top along the foot circle of criterion 1, through the
/// rib's front corners (18, 5) and (22, 5), bulges a quarter forward of them,
/// and stands further back the higher it goes. The edge at the foot of the
/// rib's right side runs out of the cone at its front end, where the matter
/// lies inside it.
fn rib_drafted_in_front() -> Body {
    let plate = plate(PLATE);
    let on_the_top = Frame {
        origin: DVec3::Z * PLATE,
        ..ground()
    };
    let rib = plate
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(18.0, 4.0),
                DVec2::new(RIB_SIDE, DRAFTED_BACK),
            )),
            on_the_top,
            DVec3::Z * 10.0,
        )
        .expect("the rib raises");
    let ring = turned_about_the_round_s_axis(
        &rib,
        Contour::straight(vec![
            DVec2::new(cone_radius(15.0), 15.0),
            DVec2::new(cone_radius(3.0), 3.0),
            DVec2::new(12.0, 3.0),
            DVec2::new(12.0, 15.0),
        ]),
    );
    let drafted = rib.difference(&ring).expect("the draft is cut");
    plate.union(&drafted).expect("the rib is joined")
}

/// The matter the [`rib_drafted_in_front`] holds: the plate's 40 × 30 × 4,
/// and across the rib's 4 and up its 10, the length from its cone to its
/// back, summed by Simpson's rule.
fn drafted_rib_volume() -> f64 {
    let across = |z: f64| {
        simpson(
            |x| DRAFTED_BACK - on_the_cone(x, z),
            [18.0, RIB_SIDE],
            STEPS,
        )
    };
    4800.0 + simpson(across, [PLATE, PLATE + 10.0], STEPS)
}

/// The most the [`rib_drafted_in_front`]'s cone can hold: an arc no longer
/// than 4.2 between the rib's sides at any height, along a slant of 10 and a
/// hair more.
fn cone_area() -> f64 {
    4.2 * 10.0 * (1.0 + DRAFT * DRAFT).sqrt()
}

/// How long a fill at the foot of the drafted rib's right side runs, `out`
/// from that side and `up` over the plate's top: from the cone to the back.
fn along_the_drafted_rib(out: f64, up: f64) -> f64 {
    DRAFTED_BACK - on_the_cone(RIB_SIDE + out, PLATE + up)
}

/// Holds that the face numbered `face` is the drafted rib's cone: curved,
/// and every corner of its triangles on the cone.
fn drawn_on_the_cone(body: &Body, face: usize, what: &str) {
    assert!(
        body.has_face(face) && !body.is_flat(face),
        "{what}: face {face} is a round"
    );
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the cone is drawn");
    for corner in corners {
        assert!(
            off_the_cone(corner).abs() < PLACE,
            "{what}: the cone is drawn through {corner}, {} off it",
            off_the_cone(corner)
        );
    }
}

/// Holds that the [`rib_drafted_in_front`] is what the steps are given: its
/// faces numbered up to [`CONE_STEPS_END`], the plate's, the rib's top, right
/// side, back and left side, and the cone; the cone curved, drawn on itself,
/// stopping at the rib's right side and standing further back at the rib's
/// top than at its foot; meeting the plate's top along one edge from the
/// rib's left side to its right, and the right side along one from the
/// plate's top to the rib's; and its matter the plate's and the rib's.
fn the_rib_is_drafted_in_front(part: &Body) {
    let what = "the rib drafted in front, before any step";
    assert_eq!(
        part.faces_end(),
        CONE_STEPS_END,
        "{what}: its steps numbered their faces up to {CONE_STEPS_END}"
    );
    let faces: BTreeSet<usize> = (0..6)
        .chain([RIB_TOP, RIB_RIGHT, RIB_BACK, RIB_LEFT, CONE_FRONT])
        .collect();
    assert_eq!(numbers(part), faces, "{what}: the faces it has");
    drawn_on_the_cone(part, CONE_FRONT, what);
    let furthest = furthest_along_x(part, CONE_FRONT);
    assert!(
        (furthest - RIB_SIDE).abs() < PLACE,
        "{what}: the cone reaches {furthest} along X, where the rib's right side stands at \
         {RIB_SIDE}"
    );
    let drafted = DVec3::new(RIB_SIDE, on_the_cone(RIB_SIDE, PLATE + 10.0), PLATE + 10.0);
    assert!(
        body_corner_at(part, CONE_FRONT, drafted),
        "{what}: the cone meets the rib's right side at its top at {drafted}, further back \
         than at its foot"
    );
    one_edge_between(
        part,
        [TOP, CONE_FRONT],
        [&[TOP, CONE_FRONT, RIB_LEFT], &[TOP, CONE_FRONT, RIB_RIGHT]],
        &format!("{what}: the cone's foot"),
    );
    one_edge_between(
        part,
        [CONE_FRONT, RIB_RIGHT],
        [
            &[TOP, CONE_FRONT, RIB_RIGHT],
            &[RIB_TOP, CONE_FRONT, RIB_RIGHT],
        ],
        &format!("{what}: the cone's edge with the rib's right side"),
    );
    sound_drawn_within(
        part,
        drafted_rib_volume(),
        drawn_within(part, cone_area()),
        what,
    );
}

/// Whether some corner of the triangles of the face numbered `face` stands
/// at `point`.
fn body_corner_at(body: &Body, face: usize, point: DVec3) -> bool {
    body.triangles_of(face)
        .flatten()
        .any(|corner| corner.distance(point) < PLACE)
}

/// Holds what a fill of `value` at the foot of the drafted rib's right side,
/// its section `height(out)` high `out` from that side, leaves where it meets
/// the cone, as [`flush_on_the_round`] holds it of criterion 1's cylinder:
/// the fill, the face numbered `made`, meeting the plate's top and the rib's
/// right side along its sides, and the cone and the rib's back at its ends,
/// by one edge each; the plate's top meeting the cone along one edge, on the
/// foot circle from the rib's left side to the fill's far side, no corner
/// left where the two alone meet; the right side meeting the cone along one
/// edge, from the fill's top up to the rib's, and its corners there and at
/// the back; the plate's top one face round the rib and the fill's
/// footprint; the cone, one face still, drawn on itself alone and reaching
/// out to the fill's far side; every ray aimed along the edge from in front
/// of the rib, through a place of the fill's section, meeting the cone first
/// and on it; and no corner of the fill in front of the cone's axis standing
/// outside the cone.
fn flush_on_the_cone(
    body: &Body,
    made: usize,
    value: f64,
    height: impl Fn(f64) -> f64,
    what: &str,
) {
    let mut sides = [TOP, RIB_RIGHT, RIB_BACK, CONE_FRONT].map(|side| vec![side as u32]);
    sides.sort();
    assert_eq!(
        sides_met(body, made),
        sides,
        "{what}: the fill meets the plate's top and the rib's right side along its sides, and \
         the cone and the rib's back at its ends, by one edge each"
    );
    one_edge_between(
        body,
        [TOP, CONE_FRONT],
        [&[TOP, CONE_FRONT, RIB_LEFT], &[TOP, CONE_FRONT, made]],
        &format!("{what}: the cone's foot, run on along the fill's footprint"),
    );
    no_corner_where_only(
        body,
        &[TOP, CONE_FRONT],
        &format!("{what}: the corner the foot of the rib's right side ended at"),
    );
    one_edge_between(
        body,
        [CONE_FRONT, RIB_RIGHT],
        [
            &[CONE_FRONT, RIB_RIGHT, made],
            &[RIB_TOP, CONE_FRONT, RIB_RIGHT],
        ],
        &format!("{what}: the cone's edge with the rib's right side, above the fill"),
    );
    let (far, up, top) = (RIB_SIDE + value, PLATE + value, PLATE + 10.0);
    corners_are_or_on(
        body,
        RIB_RIGHT,
        &[
            DVec3::new(RIB_SIDE, on_the_cone(RIB_SIDE, up), up),
            DVec3::new(RIB_SIDE, on_the_cone(RIB_SIDE, top), top),
            DVec3::new(RIB_SIDE, DRAFTED_BACK, up),
            DVec3::new(RIB_SIDE, DRAFTED_BACK, top),
        ],
        |corner| {
            (corner.x - RIB_SIDE).abs() < PLACE
                && off_the_cone(corner).abs() < PLACE
                && (up - PLACE..=top + PLACE).contains(&corner.z)
        },
        &format!("{what}: what is left of the rib's right side, its front on the cone"),
    );
    corners_are_or_on(
        body,
        TOP,
        &[
            DVec3::new(0.0, 0.0, PLATE),
            DVec3::new(40.0, 0.0, PLATE),
            DVec3::new(40.0, 30.0, PLATE),
            DVec3::new(0.0, 30.0, PLATE),
            DVec3::new(18.0, RIB_ENDS[0], PLATE),
            DVec3::new(far, on_the_cone(far, PLATE), PLATE),
            DVec3::new(far, DRAFTED_BACK, PLATE),
            DVec3::new(18.0, DRAFTED_BACK, PLATE),
        ],
        |corner| {
            off_the_cone(corner).abs() < PLACE
                && (corner.z - PLATE).abs() < PLACE
                && (18.0 - PLACE..=far + PLACE).contains(&corner.x)
                && corner.y < AXIS.y
        },
        &format!(
            "{what}: the plate's top, one face round the rib and the fill's footprint, its edge \
             with the cone on the cone's foot circle"
        ),
    );
    drawn_on_the_cone(body, CONE_FRONT, what);
    let furthest = furthest_along_x(body, CONE_FRONT);
    assert!(
        (furthest - far).abs() < PLACE,
        "{what}: the cone reaches {furthest} along X, where it takes in the fill's section out \
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
                CONE_FRONT,
                "{what}: a ray from {from} through the fill's section meets face {} at {at}, \
                 before the cone, which stands at y = {}",
                hit.face,
                on_the_cone(from.x, from.z)
            );
            assert!(
                off_the_cone(at).abs() <= MET && at.y < AXIS.y,
                "{what}: a ray from {from} meets the cone at {at}, {} off it, which stands at \
                 y = {} there",
                off_the_cone(at),
                on_the_cone(from.x, from.z)
            );
        }
    }
    for corner in body.triangles_of(made).flatten() {
        assert!(
            corner.y >= AXIS.y || off_the_cone(corner) <= PLACE,
            "{what}: the fill is drawn through {corner}, {} outside the cone: past it",
            off_the_cone(corner)
        );
    }
}

#[test]
fn a_chamfer_at_the_foot_of_a_rib_whose_round_end_has_draft_is_trimmed_flush_on_its_cone() {
    let value = 3.0;
    let part = rib_drafted_in_front();
    the_rib_is_drafted_in_front(&part);
    let foot = edge_between(&part, TOP, RIB_RIGHT);

    let chamfered = part.chamfer(&foot, value).unwrap_or_else(|declined| {
        panic!("the chamfer at the foot of the rib drafted in front: declined, {declined:?}")
    });

    let what = "the chamfer at the foot of the rib drafted in front";
    taken_whole(chamfered.asked, chamfered.taken, value, what);
    let body = &chamfered.body;
    let height = |out: f64| chamfer_height(value, out);
    sound_drawn_within(
        body,
        drafted_rib_volume() + fill_matter(value, height, along_the_drafted_rib),
        drawn_within(&part, cone_area() + value * value),
        what,
    );
    let chamfer = one_flat_face_more(&part, body, what);
    numbered_after_the_steps_last(&part, body, chamfer, what);
    flush_on_the_cone(body, chamfer, value, height, what);
}

#[test]
fn a_fillet_at_the_foot_of_a_rib_whose_round_end_has_draft_is_trimmed_flush_on_its_cone() {
    let value = 3.0;
    let part = rib_drafted_in_front();
    the_rib_is_drafted_in_front(&part);
    let foot = edge_between(&part, TOP, RIB_RIGHT);

    let filleted = part.fillet(&foot, value).unwrap_or_else(|declined| {
        panic!("the fillet at the foot of the rib drafted in front: declined, {declined:?}")
    });

    let what = "the fillet at the foot of the rib drafted in front";
    taken_whole(filleted.asked, filleted.taken, value, what);
    let body = &filleted.body;
    let height = |out: f64| fillet_height(value, out);
    let long = DRAFTED_BACK - RIB_ENDS[0];
    sound_drawn_within(
        body,
        drafted_rib_volume() + fill_matter(value, height, along_the_drafted_rib),
        drawn_within(&part, cone_area() + value * value + PI / 2.0 * value * long),
        what,
    );
    let fillet = one_round_more(&part, body, what);
    numbered_after_the_steps_last(&part, body, fillet, what);
    flush_on_the_cone(body, fillet, value, height, what);
}

/// Holds that a chamfer and a fillet of `value` on the edge between the
/// faces numbered `first` and `second` are both declined as a fill the
/// kernel cannot build, and so build no body.
fn both_declined(part: &Body, [first, second]: [usize; 2], value: f64, what: &str) {
    let name = edge_between(part, first, second);

    let declined = [
        ("the chamfer", part.chamfer(&name, value).err()),
        ("the fillet", part.fillet(&name, value).err()),
    ];

    for (step, declined) in declined {
        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "{step} of {value} {what}"
        );
    }
}

/// The round of #778's decline: about (20, 6), through the rib's front
/// corners, its radius √5.
const TURNING_AWAY: (DVec2, f64) = (DVec2::new(20.0, 6.0), 2.236_067_977_499_79);

/// The plate with the rib of #778's round decline on its top: the rib of
/// [`rib_rounded_in_front`], its front a round about [`TURNING_AWAY`]'s axis
/// through its front corners, bulging forward and leaving its right side
/// some 27 degrees off the side's line. Its cylinder reaches only √5 − 2,
/// some 0.236, past the side's plane, where a fill of 3 reaches 3: nothing of
/// it stands ahead of the fill's far side to trim the fill by.
fn rib_whose_round_turns_away() -> Body {
    let rib = Contour {
        corners: vec![
            DVec2::new(18.0, RIB_ENDS[0]),
            DVec2::new(RIB_SIDE, RIB_ENDS[0]),
            DVec2::new(RIB_SIDE, RIB_ENDS[1]),
            DVec2::new(18.0, RIB_ENDS[1]),
        ],
        runs: vec![
            Run::Round {
                center: TURNING_AWAY.0,
                turn: 2.0 * 2f64.atan(),
            },
            Run::Straight,
            Run::Straight,
            Run::Straight,
        ],
    };
    on_the_plate(&plate(PLATE), PLATE, rib)
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_round_end_turns_away_short_of_the_fill_s_reach_are_declined_as_unsupported()
 {
    let part = rib_whose_round_turns_away();
    let what = "the rib whose round turns away, before any step";
    drawn_on_the_cylinder(&part, RIB_FRONT, TURNING_AWAY, what);
    let (axis, radius) = TURNING_AWAY;
    let reach = axis.x + radius;
    assert!(
        reach - RIB_SIDE < 0.25,
        "{what}: the round's cylinder reaches {reach}, a fifth past the rib's right side"
    );
    one_edge_between(
        &part,
        [TOP, RIB_RIGHT],
        [&[TOP, RIB_RIGHT, RIB_FRONT], &[TOP, RIB_RIGHT, RIB_BACK]],
        &format!("{what}: the foot of the rib's right side"),
    );
    let turn = 2.0 * 2f64.atan();
    let segment = radius * radius * (turn - turn.sin()) / 2.0;
    sound_drawn_within(
        &part,
        4800.0 + (4.0 * 17.0 + segment) * 10.0,
        drawn_within(&part, radius * turn * 10.0),
        what,
    );

    both_declined(
        &part,
        [TOP, RIB_RIGHT],
        3.0,
        "at the foot of a rib whose round end turns away from its side within the fill's reach",
    );
}

/// How far the slot whose foot runs into its round end has that round off
/// tangent: its axis a tenth on along the front, so that the round leaves
/// the front's end turning out, dipping a fortieth of a hundredth below the
/// front's line before it comes back — the convex fillet's declined slot,
/// cut as a pocket. Past its end the foot of the front runs into the round's
/// cylinder, where the slot's air lies.
const RUNNING_INTO: f64 = 0.1;

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_slot_s_wall_running_into_its_round_end_are_declined_as_unsupported()
 {
    let part = slot_off_tangent_by(RUNNING_INTO);
    let what = "the slot whose round end dips below its front, before any step";
    let (axis, radius) = (
        slot_axis_off_by(RUNNING_INTO),
        slot_radius_off_by(RUNNING_INTO),
    );
    drawn_on_the_cylinder(&part, SLOT_END, (axis, radius), what);
    let past = DVec2::new(SLOT_FRONT_END + 0.01, SLOT_FRONT_Y);
    assert!(
        past.distance(axis) < radius,
        "{what}: just past the foot's end, at {past}, the line of the front is inside the round"
    );
    one_edge_between(
        &part,
        [SLOT_FLOOR, SLOT_FRONT],
        [
            &[SLOT_FLOOR, SLOT_FRONT, PLATE_LEFT],
            &[SLOT_FLOOR, SLOT_FRONT, SLOT_END],
        ],
        &format!("{what}: the foot of the slot's front"),
    );
    sound_drawn_within(
        &part,
        slot_volume_off_by(RUNNING_INTO),
        drawn_within(&part, PI * radius * (SLOT_PLATE - SLOT_FLOOR_Z)),
        what,
    );

    both_declined(
        &part,
        [SLOT_FLOOR, SLOT_FRONT],
        1.0,
        "at the foot of a slot's front that runs into its round end",
    );
}

/// The numbers the faces of the [`rib_cut_from_a_ball`] answer to: its right
/// side, its sphere and its left side, numbered after the run along the axis
/// it was turned about, which makes no face. Its two ends, lying on the
/// plate's top, are taken in by the join.
const BALL_RIGHT: usize = 7;
const BALL: usize = 8;

/// Where the [`rib_cut_from_a_ball`]'s sphere has its centre, on the plate's
/// top, and its radius.
const BALL_CENTRE: DVec3 = DVec3::new(20.0, 15.0, PLATE);
const BALL_RADIUS: f64 = 8.0;

/// The plate with a rib on its top cut from a ball: the slice of the ball of
/// [`BALL_RADIUS`] about [`BALL_CENTRE`] between x = 18 and its right side at
/// 22, above the plate, turned half a turn about the line along X through the
/// centre as a part's next step turns. Its sides are square to that line and
/// its floor holds it, the only planes the kernel meets a sphere with; a rib
/// with flat sides parallel to a sphere's axis and off it cannot be built.
/// The edge at the foot of its right side runs out of the sphere at both
/// ends, where the matter lies inside it — the case #805 is to fill, and
/// whose fill's chamfer, slanted to that line, the kernel cannot meet the
/// sphere with today.
fn rib_cut_from_a_ball() -> Body {
    let plate = plate(PLATE);
    let reach = (BALL_RADIUS * BALL_RADIUS - 4.0).sqrt();
    let slice = Contour {
        corners: vec![
            DVec2::new(18.0, 0.0),
            DVec2::new(RIB_SIDE, 0.0),
            DVec2::new(RIB_SIDE, reach),
            DVec2::new(18.0, reach),
        ],
        runs: vec![
            Run::Straight,
            Run::Straight,
            Run::Round {
                center: DVec2::new(BALL_CENTRE.x, 0.0),
                turn: PI - 2.0 * reach.atan2(RIB_SIDE - BALL_CENTRE.x),
            },
            Run::Straight,
        ],
    };
    let sampled: &'static [DVec2] = Box::leak(slice.corners.clone().into_boxed_slice());
    let outline = Profile {
        exact: Some((slice, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let on_the_top = Frame {
        origin: DVec3::new(0.0, BALL_CENTRE.y, PLATE),
        u: DVec3::X,
        v: DVec3::Y,
    };
    let along_x = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::X,
    };
    let rib = plate
        .tool_turned(&outline, on_the_top, &Turn::of(along_x, PI, 1e-4, &outline))
        .expect("the slice turns");
    plate.union(&rib).expect("the rib is joined")
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_end_is_a_sphere_are_declined_as_unsupported() {
    let part = rib_cut_from_a_ball();
    let what = "the rib cut from a ball, before any step";
    assert!(
        part.has_face(BALL) && !part.is_flat(BALL) && part.is_flat(BALL_RIGHT),
        "{what}: face {BALL} is a round and face {BALL_RIGHT} flat"
    );
    for corner in part.triangles_of(BALL).flatten() {
        let off = corner.distance(BALL_CENTRE) - BALL_RADIUS;
        assert!(
            off.abs() < PLACE,
            "{what}: the rib's sphere is drawn through {corner}, {off} off it"
        );
    }
    one_edge_between(
        &part,
        [TOP, BALL_RIGHT],
        [&[TOP, BALL_RIGHT, BALL], &[TOP, BALL_RIGHT, BALL]],
        &format!("{what}: the foot of the rib's right side, the sphere at both its ends"),
    );
    let half_turned = PI / 2.0 * (4.0 * BALL_RADIUS * BALL_RADIUS - 16.0 / 3.0);
    sound_drawn_within(
        &part,
        4800.0 + half_turned,
        drawn_within(&part, 2.0 * PI * BALL_RADIUS * 4.0),
        what,
    );

    both_declined(
        &part,
        [TOP, BALL_RIGHT],
        2.0,
        "at the foot of a rib whose ends are a sphere, which #805 is to fill",
    );
}

/// The ground's plane with its origin moved to `to`.
fn moved(to: [f64; 3]) -> Frame {
    Frame {
        origin: DVec3::from(to),
        ..ground()
    }
}

/// Where a part is moved to, whole.
const ELSEWHERE: [f64; 3] = [100.0, -40.0, 7.0];

/// The ground's plane turned about the vertical by `degrees`.
fn turned_about_the_vertical(degrees: f64) -> Frame {
    let turn = DQuat::from_rotation_z(degrees.to_radians());
    Frame {
        origin: DVec3::ZERO,
        u: turn * DVec3::X,
        v: turn * DVec3::Y,
    }
}

/// The ground's plane tipped by 40 degrees about an axis leaning off all
/// three, its origin moved to `to`: a part drawn on it has no face and no
/// edge square to an axis or along one.
fn tipped(to: [f64; 3]) -> Frame {
    let turn = DQuat::from_axis_angle(DVec3::new(1.0, 2.0, 2.0).normalize(), 40f64.to_radians());
    Frame {
        origin: DVec3::from(to),
        u: turn * DVec3::X,
        v: turn * DVec3::Y,
    }
}

/// A point of the world read in the axes of a part drawn on `frame`: along,
/// across and up.
fn in_the_part_s_axes(frame: Frame, point: DVec3) -> DVec3 {
    let off = point - frame.origin;
    DVec3::new(off.dot(frame.u), off.dot(frame.v), off.dot(frame.normal()))
}

/// The size of a part like the [`rib_rounded_in_front`]: how thick its plate
/// is, where its rib's back stands and how high the rib is raised. Its round
/// is drawn where it was.
#[derive(Clone, Copy)]
struct RibSize {
    plate: f64,
    back: f64,
    height: f64,
}

const AS_DRAWN: RibSize = RibSize {
    plate: PLATE,
    back: 22.0,
    height: 10.0,
};

const RESIZED: RibSize = RibSize {
    plate: 6.0,
    back: 25.0,
    height: 12.0,
};

impl RibSize {
    /// The part, its plate and its rib drawn on `frame`, numbered as
    /// [`rib_rounded_in_front`] numbers them: on the ground, as drawn, it is
    /// that part.
    fn on(self, frame: Frame) -> Body {
        let plate = raised_on(
            frame,
            Contour::rectangle(DVec2::ZERO, DVec2::new(40.0, 30.0)),
            self.plate,
        );
        let on_the_top = Frame {
            origin: frame.origin + frame.normal() * self.plate,
            ..frame
        };
        let rib = Contour {
            corners: vec![
                DVec2::new(18.0, RIB_ENDS[0]),
                DVec2::new(RIB_SIDE, RIB_ENDS[0]),
                DVec2::new(RIB_SIDE, self.back),
                DVec2::new(18.0, self.back),
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
        let tool = plate
            .tool_raised(&profile(rib), on_the_top, frame.normal() * self.height)
            .expect("the rib raises");
        plate.union(&tool).expect("the rib is joined")
    }

    /// The matter the part holds, as [`rounded_rib_volume`] reckons it.
    fn volume(self) -> f64 {
        let turn = round_turn();
        let segment = RADIUS * RADIUS * (turn - turn.sin()) / 2.0;
        1200.0 * self.plate + (4.0 * (self.back - RIB_ENDS[0]) + segment) * self.height
    }

    /// How long a fill at the foot of the rib's right side runs, `out` from
    /// it: from the round to the back.
    fn along(self, out: f64) -> f64 {
        self.back - on_the_round(RIB_SIDE + out)
    }
}

/// Holds what a fill of `value` at the foot of the rib's right side of a
/// part drawn on `frame` leaves of the round, read in the part's own axes:
/// the fill, the face numbered `made`, meeting the plate's top, the rib's
/// right side, the round and the rib's back by one edge each; the plate's
/// top meeting the round along one edge from the rib's left side to the
/// fill, no corner left where the two alone meet; the round drawn on its
/// cylinder and reaching out to the fill's far side; and no corner of the
/// fill in front of the round's axis outside the round.
fn flush_on_the_same_round(body: &Body, frame: Frame, made: usize, value: f64, what: &str) {
    let mut sides = [TOP, RIB_FRONT, RIB_RIGHT, RIB_BACK].map(|side| vec![side as u32]);
    sides.sort();
    assert_eq!(
        sides_met(body, made),
        sides,
        "{what}: the fill meets the plate's top, the rib's right side, the round and the rib's \
         back, by one edge each"
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
    assert!(
        body.has_face(RIB_FRONT) && !body.is_flat(RIB_FRONT),
        "{what}: face {RIB_FRONT} is the round still"
    );
    let round: Vec<DVec3> = body
        .triangles_of(RIB_FRONT)
        .flatten()
        .map(|corner| in_the_part_s_axes(frame, corner))
        .collect();
    for corner in &round {
        assert!(
            (off_the_axis(*corner) - RADIUS).abs() < PLACE,
            "{what}: the round is drawn through {corner}, in the part's axes, off its cylinder"
        );
    }
    let furthest = round
        .iter()
        .map(|corner| corner.x)
        .fold(f64::NEG_INFINITY, f64::max);
    let far = RIB_SIDE + value;
    assert!(
        (furthest - far).abs() < PLACE,
        "{what}: the round reaches {furthest} along the part, where it takes in the fill's \
         section out to {far}"
    );
    for corner in body.triangles_of(made).flatten() {
        let corner = in_the_part_s_axes(frame, corner);
        assert!(
            corner.y >= AXIS.y || off_the_axis(corner) <= RADIUS + PLACE,
            "{what}: the fill is drawn through {corner}, in the part's axes, past the round"
        );
    }
}

#[test]
fn a_chamfer_and_a_fillet_fill_the_concave_edge_their_name_finds_again_flush_on_the_same_round_after_a_change_of_size_a_move_and_a_turn()
 {
    let value = 3.0;
    let name = edge_between(&AS_DRAWN.on(ground()), TOP, RIB_RIGHT);

    for (drawn, size, frame) in [
        ("as drawn", AS_DRAWN, ground()),
        ("resized", RESIZED, ground()),
        ("moved", AS_DRAWN, moved(ELSEWHERE)),
        (
            "turned 30 degrees about the vertical",
            AS_DRAWN,
            turned_about_the_vertical(30.0),
        ),
        ("tipped off every axis", AS_DRAWN, tipped([0.0; 3])),
        ("resized, moved and tipped", RESIZED, tipped(ELSEWHERE)),
    ] {
        let part = size.on(frame);
        let round = RADIUS * round_turn() * size.height;
        let long = size.along(0.0);

        let chamfered = part
            .chamfer(&name, value)
            .unwrap_or_else(|declined| panic!("the chamfer, {drawn}: declined, {declined:?}"));

        let what = &format!("the chamfer, {drawn}");
        taken_whole(chamfered.asked, chamfered.taken, value, what);
        let body = &chamfered.body;
        let height = |out: f64| chamfer_height(value, out);
        sound_drawn_within(
            body,
            size.volume() + fill_matter(value, height, |out, _| size.along(out)),
            drawn_within(&part, round + value * value),
            what,
        );
        let chamfer = one_flat_face_more(&part, body, what);
        numbered_after_the_steps_last(&part, body, chamfer, what);
        flush_on_the_same_round(body, frame, chamfer, value, what);

        let filleted = part
            .fillet(&name, value)
            .unwrap_or_else(|declined| panic!("the fillet, {drawn}: declined, {declined:?}"));

        let what = &format!("the fillet, {drawn}");
        taken_whole(filleted.asked, filleted.taken, value, what);
        let body = &filleted.body;
        let height = |out: f64| fillet_height(value, out);
        sound_drawn_within(
            body,
            size.volume() + fill_matter(value, height, |out, _| size.along(out)),
            drawn_within(&part, round + value * value + PI / 2.0 * value * long),
            what,
        );
        let fillet = one_round_more(&part, body, what);
        numbered_after_the_steps_last(&part, body, fillet, what);
        flush_on_the_same_round(body, frame, fillet, value, what);
    }
}

/// How long the edges of the [`rib_rounded_in_front`] filled at its foot are
/// in all, rounded up: the plate's twelve, near 300; the rib's top and foot
/// outlines, some 42 each, and its four uprights; the fill's sides and ends.
/// The file's tolerance over a strip that long is what its area may be off
/// by.
const FILLED_RIB_EDGES: f64 = 520.0;

/// The area of the [`rib_rounded_in_front`] filled at the foot of its right
/// side by `value`, its section `height(out)` high, holding `section` and
/// laying a face of `laid`: the plate's floor and walls; its top less the
/// rib's plan and the fill's footprint, which runs from the round to the
/// back; the rib's top, its left side, its right side above the fill, and
/// its back with the fill's section; and the round, grown by the fill's
/// section laid on its cylinder, which stretches each step out from the
/// rib's side by r over √(r² − x²), x that step's reach from the axis.
fn filled_rib_area(value: f64, height: impl Fn(f64) -> f64, section: f64, laid: f64) -> f64 {
    let turn = round_turn();
    let plan = 4.0 * 17.0 + RADIUS * RADIUS * (turn - turn.sin()) / 2.0;
    let footprint = simpson(along_the_rib, [0.0, value], STEPS);
    let plate = 1200.0 + 2.0 * (40.0 + 30.0) * PLATE + 1200.0 - plan - footprint;
    let rib = plan + 17.0 * 10.0 + 17.0 * (10.0 - value) + 4.0 * 10.0 + section;
    let stretched = fill_matter(value, height, |out, _| {
        RADIUS / (RADIUS * RADIUS - (RIB_SIDE + out - AXIS.x).powi(2)).sqrt()
    });
    plate + rib + round_area() + stretched + laid
}

/// Holds that a STEP file read back is one exact solid of the body's faces,
/// all its loops on their surfaces — every edge's curve on the surfaces of
/// both faces it bounds — within the file's tolerance, no wider than the
/// kernel's; enclosing the matter arithmetic promised and the body holds, and
/// covering the area arithmetic promised, within the file's tolerance over
/// the part's edges, `edges` long in all.
fn written_whole(
    solid: &step_reader::Solid,
    body: &Body,
    [volume, area, edges]: [f64; 3],
    what: &str,
) {
    assert_eq!(
        solid.faces,
        numbers(body).len(),
        "{what}: the file holds the faces the body does"
    );
    assert!(
        solid.uncertainty <= kernel_tolerance(body) * (1.0 + 1e-12),
        "{what}: the file declares points one under {}, the kernel only under {}",
        solid.uncertainty,
        kernel_tolerance(body)
    );
    assert!(
        solid.worst <= solid.uncertainty,
        "{what}: a loop stands {} off its face or its edge, the file's tolerance is {}",
        solid.worst,
        solid.uncertainty
    );
    for (whose, volume) in [
        ("arithmetic promised", volume),
        ("the body holds", body.volume()),
    ] {
        assert!(
            (solid.volume - volume).abs() <= solid.uncertainty * solid.area,
            "{what}: the file encloses {}, where {whose} {volume}",
            solid.volume
        );
    }
    assert!(
        (solid.area - area).abs() <= solid.uncertainty * edges,
        "{what}: the file's faces cover {}, where arithmetic promised {area}",
        solid.area
    );
}

/// The kernel's tolerance for a body, in the file's millimetres at one to
/// the unit: the hair it takes two surfaces one within, at the body's reach.
fn kernel_tolerance(body: &Body) -> f64 {
    Scale::HAIR * Scale::of(reach_of(body)).eps()
}

/// Holds that the file writes the rib's round on its cylinder, about the
/// upright line through [`AXIS`] and of [`RADIUS`].
fn round_written_on_its_cylinder(solid: &step_reader::Solid, what: &str) -> usize {
    let found = solid.cylinders.iter().position(|&(origin, axis, radius)| {
        axis.cross(DVec3::Z).length() < PLACE
            && (origin.truncate().distance(AXIS)) <= solid.uncertainty
            && (radius - RADIUS).abs() <= solid.uncertainty
    });
    found.unwrap_or_else(|| {
        panic!(
            "{what}: no cylinder of radius {RADIUS} about the upright line through {AXIS} under \
             the round: {:?}",
            solid.cylinders
        )
    })
}

#[test]
fn a_rib_filled_up_to_its_round_end_written_to_step_writes_every_face_on_its_true_surface() {
    let value = 3.0;
    let part = rib_rounded_in_front();
    let foot = edge_between(&part, TOP, RIB_RIGHT);

    let chamfered = part.chamfer(&foot, value).expect("the chamfer is laid");
    let text = step::of(&chamfered.body, 1.0).expect("an exact body is written");

    let what = "the rib chamfered up to its round";
    let solid = step_reader::read(&text)
        .unwrap_or_else(|fault| panic!("{what}: the file does not read back: {fault}"));
    assert_eq!(
        solid.faces, 12,
        "{what}: the plate's six faces, the rib's top, its round, right side, back and left \
         side, and the chamfer"
    );
    let height = |out: f64| chamfer_height(value, out);
    let footprint = simpson(along_the_rib, [0.0, value], STEPS);
    written_whole(
        &solid,
        &chamfered.body,
        [
            rounded_rib_volume() + fill_matter(value, height, |out, _| along_the_rib(out)),
            filled_rib_area(value, height, value * value / 2.0, 2f64.sqrt() * footprint),
            FILLED_RIB_EDGES,
        ],
        what,
    );
    round_written_on_its_cylinder(&solid, what);
    assert_eq!(
        solid.cylinders.len(),
        1,
        "{what}: the round alone is written on a cylinder: {:?}",
        solid.cylinders
    );
    let facing = DVec3::new(1.0, 0.0, 1.0).normalize();
    let through = [
        DVec3::new(RIB_SIDE + value, RIB_ENDS[1], PLATE),
        DVec3::new(RIB_SIDE, RIB_ENDS[1], PLATE + value),
    ];
    assert!(
        solid.planes.iter().any(|&(origin, normal)| {
            normal.distance(facing) < PLACE
                && through
                    .iter()
                    .all(|point| (*point - origin).dot(normal).abs() <= solid.uncertainty)
        }),
        "{what}: no face of the file faces {facing} through {through:?}: {:?}",
        solid.planes
    );
    assert!(
        text.contains("ELLIPSE("),
        "{what}: the chamfer's edge on the round, where a slanted plane meets an upright \
         cylinder, is written as its ellipse"
    );

    let filleted = part.fillet(&foot, value).expect("the fillet is laid");
    let text = step::of(&filleted.body, 1.0).expect("an exact body is written");

    let what = "the rib filleted up to its round";
    let solid = step_reader::read(&text)
        .unwrap_or_else(|fault| panic!("{what}: the file does not read back: {fault}"));
    assert_eq!(
        solid.faces, 12,
        "{what}: the plate's six faces, the rib's top, its round, right side, back and left \
         side, and the fillet"
    );
    let height = |out: f64| fillet_height(value, out);
    let laid = simpson(
        |turn| along_the_rib(value * (1.0 - turn.sin())) * value,
        [0.0, PI / 2.0],
        STEPS,
    );
    written_whole(
        &solid,
        &filleted.body,
        [
            rounded_rib_volume() + fill_matter(value, height, |out, _| along_the_rib(out)),
            filled_rib_area(value, height, value * value * (1.0 - PI / 4.0), laid),
            FILLED_RIB_EDGES,
        ],
        what,
    );
    let round = round_written_on_its_cylinder(&solid, what);
    let fillet: Vec<_> = solid
        .cylinders
        .iter()
        .enumerate()
        .filter(|&(index, _)| index != round)
        .map(|(_, cylinder)| *cylinder)
        .collect();
    let [(origin, axis, written)] = fillet[..] else {
        panic!(
            "{what}: one cylinder besides the round's, under the fillet: {:?}",
            solid.cylinders
        );
    };
    assert!(
        (written - value).abs() <= solid.uncertainty,
        "{what}: the fillet is written on a cylinder of radius {written}, where it is {value}"
    );
    assert!(
        axis.cross(DVec3::Y).length() < PLACE,
        "{what}: the fillet's cylinder runs along {axis}, where the foot ran along Y"
    );
    let off = origin - DVec3::new(RIB_SIDE + value, 0.0, PLATE + value);
    assert!(
        (off - DVec3::Y * off.y).length() <= solid.uncertainty,
        "{what}: the fillet's cylinder stands about the line through {origin}, where its axis \
         is the radius out from the plate's top and from the rib's right side"
    );
    let [meeting] = &solid.along_splines[..] else {
        panic!(
            "{what}: one curve written as a B-spline, where the fillet meets the round: {}",
            solid.along_splines.len()
        );
    };
    let tolerance = kernel_tolerance(&filleted.body);
    for point in meeting {
        let off_the_round = (off_the_axis(*point) - RADIUS).abs();
        let off_the_fillet = (DVec2::new(point.x, point.z)
            .distance(DVec2::new(RIB_SIDE + value, PLATE + value))
            - value)
            .abs();
        assert!(
            off_the_round <= tolerance && off_the_fillet <= tolerance,
            "{what}: the fillet's edge on the round is written through {point}, {off_the_round} \
             off the round and {off_the_fillet} off the fillet, the kernel's tolerance is \
             {tolerance}"
        );
    }
}
