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
//!   `a_fillet_at_the_foot_of_a_rib_whose_end_is_a_round_bulging_out_is_trimmed_flush_on_the_round`;
//!   on a round bulging gently, of a large radius, its axis well past the
//!   part's back —
//!   `a_chamfer_at_the_foot_of_a_rib_whose_end_bulges_gently_in_a_round_of_large_radius_is_trimmed_flush_on_the_round`,
//!   `a_fillet_at_the_foot_of_a_rib_whose_end_bulges_gently_in_a_round_of_large_radius_is_trimmed_flush_on_the_round`;
//!   up a ramp, the foot rising along the round's axis as it runs —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_up_a_ramp_are_trimmed_flush_on_its_upright_round_end`;
//!   on a rib rounded at both ends, each trimmed flush —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_rounded_at_both_ends_are_trimmed_flush_on_each_round`;
//!   with #778's rule carried over, a value larger than the plate's top gives
//!   beside the rib reduced to the largest that fits, the fill flush on the
//!   round still —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_near_the_plate_s_side_asked_more_than_the_plate_gives_take_the_most_it_gives_and_stay_flush_on_the_round`;
//!   and the part left drawn as finely as its own box asks, the trim's
//!   tools reaching no further than the part does —
//!   `a_chamfer_and_a_fillet_up_to_a_round_end_leave_the_part_drawn_as_finely_as_its_own_box_asks`.
//!   No public accessor gives a body's scale. The drawing, which is cut at
//!   the drawing's tolerance times that scale's reach, is what a test can
//!   read of it
//! - The same holds where the edge ends on a round wall it meets at a few
//!   degrees, such as a slot's round end —
//!   `a_chamfer_at_the_foot_of_a_slot_s_wall_is_trimmed_flush_on_the_round_end_it_meets_at_a_few_degrees`,
//!   `a_fillet_at_the_foot_of_a_slot_s_wall_is_trimmed_flush_on_the_round_end_it_meets_at_a_few_degrees`;
//!   and where both of a slot's floor edges are filled in one step, its round
//!   end meeting both walls so, their fills meeting across the floor in a
//!   valley or one round bottom —
//!   `a_chamfer_on_both_floor_edges_of_a_slot_whose_round_end_meets_both_walls_at_a_few_degrees_meets_in_a_valley_flush_on_the_round`,
//!   `a_fillet_on_both_floor_edges_of_a_slot_whose_round_end_meets_both_walls_at_a_few_degrees_lays_one_round_bottom_flush_on_the_round`
//! - The same holds where the end face is a cone, a sphere and a torus —
//!   answered for the cylinder above and for the cone, a rib whose round end
//!   has draft:
//!   `a_chamfer_at_the_foot_of_a_rib_whose_round_end_has_draft_is_trimmed_flush_on_its_cone`,
//!   `a_fillet_at_the_foot_of_a_rib_whose_round_end_has_draft_is_trimmed_flush_on_its_cone`,
//!   and up a ramp —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_up_a_ramp_are_trimmed_flush_on_its_drafted_round_end`.
//!   The sphere and the torus left this issue for #805 (the human's decision):
//!   the kernel meets neither with a slanted plane or a cylinder off its axis,
//!   so a fill ending on one stays declined here, as the next criterion pins
//! - An end face the fill cannot be trimmed flush by is declined
//!   `Unsupported` with that reason, and no wrong body is built — a round the
//!   edge runs out of that turns away short of the fill's reach,
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_round_end_turns_away_short_of_the_fill_s_reach_are_declined_as_unsupported`;
//!   a round the edge runs into,
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_slot_s_wall_running_into_its_round_end_are_declined_as_unsupported`;
//!   a sphere, #805's, here a rib cut from a ball —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_end_is_a_sphere_are_declined_as_unsupported`;
//!   two edges filled in one step, the second of which runs into the round —
//!   `a_chamfer_and_a_fillet_on_both_floor_edges_of_a_slot_whose_back_wall_runs_into_its_round_end_are_declined_as_unsupported`;
//!   a drafted round end the edge runs into —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_slot_s_wall_running_into_its_drafted_round_end_are_declined_as_unsupported`
//!   (a drafted end it runs out of, #807's, cannot be cut with the slot's
//!   walls upright: its fixture says why); and a round the edge runs on into
//!   tangentially, #698's chain —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_slot_s_wall_running_on_tangentially_into_its_round_end_are_declined_as_unsupported`
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
//!
//! Closes #807.
//! - A chamfer and a fillet at the foot of a rib whose drafted round end
//!   stands near the plate's side leave the part drawn as finely as its own
//!   box asks —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_drafted_round_end_stands_near_the_plate_s_side_leave_the_part_drawn_as_finely_as_its_own_box_asks`.
//!   Since #876 the triangles are cut at the body's own size whatever reach
//!   it carries, so the test reads that reach where it still shows: what
//!   stands in front of a plane is cut a drawing tolerance of it past the
//!   plane. It is held to what the same fill up to an upright round carries,
//!   whose trim hugs the fill: beside the plate's side the fill's own prism
//!   stands its clearance past the side, and the box alone cannot be asked
//! - A chamfer and a fillet at the foot of a slot's wall whose drafted round
//!   end it meets at a few degrees are trimmed flush on the cone. The cone
//!   stays one face and takes in the fill's section, and the body is closed,
//!   sound and of the right volume, with one more face — answered for the
//!   chamfer,
//!   `a_chamfer_at_the_foot_of_a_slot_s_drafted_wall_is_trimmed_flush_on_the_drafted_round_end_it_meets_at_a_few_degrees`.
//!   The fillet is split to #907, and stays declined until then —
//!   `a_fillet_at_the_foot_of_a_slot_s_drafted_wall_meeting_its_drafted_round_end_at_a_few_degrees_is_declined_as_unsupported`.
//!   The slot's walls are drafted with its round, which a raise square to
//!   its sketch lays when the sketch is drawn across the slot
//! - #788's tests still pass — no test: they are the tests above in this
//!   file, left as they were, and running them is the answer
//!
//! The mutants of #807's trim no criterion's test reaches, one line each.
//! `leaving`'s growth of the radius along an edge: killed by the rib up a
//! steep ramp. `past_the_end`'s cylinder guard turned `true`: killed by the
//! pillar decline, which #908 has since turned round. `outside_cone`'s
//! `height` from `at + origin`: equivalent, `Cone::through` keeps
//! `origin · axis` nought.
//! `past_the_end`'s plane guard turned `true`: equivalent, #835's argument.
//! `leaving`'s `at > 0`, `at < 1` and radius `> 0` admitting equality:
//! unkilled, only an exit exactly at a prism end or the apex, and the prism
//! runs the body's box and a margin past it. Its `&&` turned `||`, and
//! `growth * at` turned `+` or `/`: unkilled, not reached, it takes an exit
//! past the prism's ends or beyond the apex. `past_the_end`'s cylinder `>`
//! turned `>=`: unkilled, only at exact tangency, #698's chain, still
//! declined further on.
//!
//! Closes #908.
//! - A chamfer and a fillet at the foot of a wall running into a round
//!   pillar, with the matter inside the cylinder, are trimmed flush on the
//!   pillar. The body is closed, sound and of the right volume —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_wall_running_into_a_round_pillar_are_trimmed_flush_on_the_pillar`.
//!   Flush as the rib's round end is: the pillar stays one face on its
//!   cylinder, cut back over the fill, its foot one edge with no corner left
//!   where only it and the plate's top meet, and nothing of the fill inside
//!   it
//! - The pinned decline that #807 adds for this case is turned around —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_wall_running_into_a_round_pillar_are_trimmed_flush_on_the_pillar`
//!   is that decline, renamed for what it now builds
//! - #788's and #807's other tests still pass, including the declines of an
//!   edge running into a cone (#807's comment) — no test: they are the tests
//!   above in this file, left as they were, and running them is the answer.
//!   A round the edge runs into whose matter lies outside it stays declined
//!   by them too: the slot's front running into its round end
//!
//! The mutants of #908's arm in `past_the_end` the upright pillar lets live,
//! one line each. `towards`'s `-` turned `+`: killed by the leaning pillar,
//! `a_chamfer_and_a_fillet_at_the_foot_of_a_wall_running_into_a_leaning_pillar_are_trimmed_flush_on_it`;
//! an upright pillar's axis is square to the edge's run, and the two agree.
//! The guard turned `true`, and its `&&` turned `||`: equivalent, they let
//! in a round the edge runs into with its matter outside, a slot's end or a
//! bore, whose fill the axis plane leaves standing in the air and
//! `cut_along`'s stray check declines `Unsupported` all the same. Its `<`
//! turned `<=`: equivalent, it lets in only an edge tangent to the round,
//! the flipped tangent slot or a wall tangent to a pillar, #698's chain,
//! declined further on.

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

/// How far a point `out` from the wall a fill stands against and `up` over
/// the face it lies on stands off the fill's surface, `height(out)` high,
/// measured square to it: its height off the surface over the secant of the
/// surface's slope there — as a ray cast down meets a chord of a sloping
/// round further off it than the chord stands.
fn off_the_surface(height: impl Fn(f64) -> f64, out: f64, up: f64) -> f64 {
    let step = 1e-6;
    let slope = (height(out + step) - height(out - step)) / (2.0 * step);
    (up - height(out)) / (1.0 + slope * slope).sqrt()
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
        let off = off_the_surface(&height, off_the_front, at.z - floor);
        assert!(
            hit.face == made && off.abs() <= MET,
            "{what}: a ray down from {from}, short of the round end, meets face {} at {at}, {off} \
             off the fill's surface",
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
    turned_about_the_round_s_axis_from(part, contour, 0.0)
}

/// What [`turned_about_the_round_s_axis`] turns, the plane it is drawn on
/// having its origin `height` up the axis: the surfaces it lays then reckon
/// heights from there.
fn turned_about_the_round_s_axis_from(part: &Body, contour: Contour, height: f64) -> Body {
    let sampled: &'static [DVec2] = Box::leak(contour.corners.clone().into_boxed_slice());
    let outline = Profile {
        exact: Some((contour, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: AXIS.extend(height),
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

/// The number the plate's right side answers to: the face beyond the plate's
/// top, where a fill takes the top's whole width beside a rib.
const PLATE_RIGHT: usize = 3;

/// A round end of a rib: the upright line it turns about, read in plan, on
/// the rib's middle, and its radius.
#[derive(Clone, Copy, Debug)]
struct Round {
    axis: DVec2,
    radius: f64,
}

impl Round {
    /// The round of `radius` through the two corners 4 apart along X either
    /// side of `middle` at `y`, its axis `toward` them from behind, +1, or
    /// from in front, −1.
    fn through(middle: f64, y: f64, radius: f64, toward: f64) -> Round {
        Round {
            axis: DVec2::new(middle, y + toward * (radius * radius - 4.0).sqrt()),
            radius,
        }
    }

    /// The turn it makes from one corner to the other.
    fn turn(self) -> f64 {
        2.0 * (2.0 / self.radius).asin()
    }

    /// The matter beyond the chord between its corners, in plan.
    fn segment(self) -> f64 {
        self.radius * self.radius * (self.turn() - self.turn().sin()) / 2.0
    }

    /// How far across from its axis it stands at `x`, read along Y.
    fn across(self, x: f64) -> f64 {
        (self.radius * self.radius - (x - self.axis.x).powi(2)).sqrt()
    }

    /// How far a point stands off its cylinder, out from its axis.
    fn off(self, point: DVec3) -> f64 {
        point.truncate().distance(self.axis) - self.radius
    }
}

/// How a rib ends behind: square, flat at y, or in a round through its back
/// corners bulging out behind them.
#[derive(Clone, Copy, Debug)]
enum Back {
    Flat(f64),
    Round(Round),
}

/// A rib 4 wide joined on the plate of [`plate`], 4 thick, as
/// [`on_the_plate`] joins it and numbers it: its left side at `left`, raised
/// `height`, its front a round through its front corners at y = 5, `front`,
/// and its back `back`.
#[derive(Clone, Copy, Debug)]
struct Rib {
    left: f64,
    height: f64,
    front: Round,
    back: Back,
}

/// Criterion 1's rib: from x = 18 to 22, its front the round about [`AXIS`]
/// of [`RADIUS`], its back flat at y = 22.
const CRITERION_1: Rib = Rib {
    left: 18.0,
    height: 10.0,
    front: Round {
        axis: AXIS,
        radius: RADIUS,
    },
    back: Back::Flat(22.0),
};

impl Rib {
    fn right(self) -> f64 {
        self.left + 4.0
    }

    /// Where its front stands at `x`, read in plan.
    fn front_at(self, x: f64) -> f64 {
        self.front.axis.y - self.front.across(x)
    }

    /// Where its back stands at `x`, read in plan.
    fn back_at(self, x: f64) -> f64 {
        match self.back {
            Back::Flat(y) => y,
            Back::Round(round) => round.axis.y + round.across(x),
        }
    }

    /// Its round ends, front first, each with the face it is and which way
    /// it faces out along Y.
    fn rounds(self) -> Vec<(Round, usize, f64)> {
        let mut rounds = vec![(self.front, RIB_FRONT, -1.0)];
        if let Back::Round(round) = self.back {
            rounds.push((round, RIB_BACK, 1.0));
        }
        rounds
    }

    fn on(self) -> Body {
        let (left, right, back) = (self.left, self.right(), self.back_at(self.left));
        let behind = match self.back {
            Back::Flat(_) => Run::Straight,
            Back::Round(round) => Run::Round {
                center: round.axis,
                turn: round.turn(),
            },
        };
        let outline = Contour {
            corners: vec![
                DVec2::new(left, RIB_ENDS[0]),
                DVec2::new(right, RIB_ENDS[0]),
                DVec2::new(right, back),
                DVec2::new(left, back),
            ],
            runs: vec![
                Run::Round {
                    center: self.front.axis,
                    turn: self.front.turn(),
                },
                Run::Straight,
                behind,
                Run::Straight,
            ],
        };
        let plate = plate(PLATE);
        let on_the_top = Frame {
            origin: DVec3::Z * PLATE,
            ..ground()
        };
        let rib = plate
            .tool_raised(&profile(outline), on_the_top, DVec3::Z * self.height)
            .expect("the rib raises");
        plate.union(&rib).expect("the rib is joined")
    }

    /// The matter the part holds: the plate's, and the rib's plan — the
    /// rectangle between its corners and the segment of each round end —
    /// raised by its height.
    fn volume(self) -> f64 {
        let back = match self.back {
            Back::Flat(_) => 0.0,
            Back::Round(round) => round.segment(),
        };
        let plan = 4.0 * (self.back_at(self.left) - RIB_ENDS[0]) + self.front.segment() + back;
        4800.0 + plan * self.height
    }

    /// The area of its round ends.
    fn rounds_area(self) -> f64 {
        self.rounds()
            .iter()
            .map(|(round, ..)| round.radius * round.turn() * self.height)
            .sum()
    }

    /// How long a fill at the foot of its right side runs, `out` from it:
    /// from its front to its back.
    fn along(self, out: f64) -> f64 {
        self.back_at(self.right() + out) - self.front_at(self.right() + out)
    }
}

/// Holds that a rib is what the steps are given: its faces numbered up to
/// [`STEPS_END`]; each round end curved, drawn on its cylinder, stopping at
/// the rib's right side, meeting the plate's top along one edge from the
/// rib's left side to its right and the right side along one from the
/// plate's top to the rib's; and its matter the plate's and the rib's.
fn the_rib_stands_as_drawn(part: &Body, rib: Rib, what: &str) {
    assert_eq!(
        part.faces_end(),
        STEPS_END,
        "{what}: its steps numbered their faces up to {STEPS_END}"
    );
    for (round, face, _) in rib.rounds() {
        drawn_on_the_cylinder(part, face, (round.axis, round.radius), what);
        let furthest = furthest_along_x(part, face);
        assert!(
            (furthest - rib.right()).abs() < PLACE,
            "{what}: the round {face} reaches {furthest} along X, where the rib's right side \
             stands at {}",
            rib.right()
        );
        one_edge_between(
            part,
            [TOP, face],
            [&[TOP, face, RIB_LEFT], &[TOP, face, RIB_RIGHT]],
            &format!("{what}: the round {face}'s foot"),
        );
        one_edge_between(
            part,
            [face, RIB_RIGHT],
            [&[TOP, face, RIB_RIGHT], &[RIB_TOP, face, RIB_RIGHT]],
            &format!("{what}: the round {face}'s edge with the rib's right side"),
        );
    }
    sound_drawn_within(
        part,
        rib.volume(),
        drawn_within(part, rib.rounds_area()),
        what,
    );
}

/// How many edges lie between the faces numbered `first` and `second`.
fn edges_between(body: &Body, first: usize, second: usize) -> usize {
    let (one, other) = (vec![first as u32], vec![second as u32]);
    body.edge_names()
        .into_iter()
        .filter(|name| name.between.contains(&one) && name.between.contains(&other))
        .count()
}

/// Holds what a fill of `taken` at the foot of a rib's right side, its
/// section `height(out)` high `out` from that side, leaves where it meets
/// the rib's ends, as [`flush_on_the_round`] holds it of criterion 1's: the
/// fill, the face numbered `made`, meeting the rib's right side and the face
/// it lies on, `beneath` — the plate's top, or its right side where the fill
/// took the top's whole width — along its sides, and the rib's front and
/// back at its ends, by one edge each; at each end, the plate's top meeting
/// it along one edge — from the rib's left side to the fill where the fill
/// lies on the top — no corner left where the two alone meet, and the right
/// side meeting it along one edge from the fill's top up to the rib's; what
/// is left of the right side; the plate's top one face round the rib and
/// the fill's footprint, its edges with the rounds on their foot circles; and
/// at each round end, one face still, drawn on its cylinder alone and
/// reaching out to the fill's far side, every ray aimed along the edge from
/// outside the rib through a place of the fill's section meeting it first
/// and on it, and no corner of the fill beyond its axis standing outside it.
fn flush_on_the_rib_s_ends(
    body: &Body,
    (rib, made): (Rib, usize),
    taken: f64,
    height: impl Fn(f64) -> f64,
    beneath: usize,
    what: &str,
) {
    let mut sides = [beneath, RIB_FRONT, RIB_RIGHT, RIB_BACK].map(|side| vec![side as u32]);
    sides.sort();
    assert_eq!(
        sides_met(body, made),
        sides,
        "{what}: the fill meets the face it lies on and the rib's right side along its sides, \
         and the rib's front and back at its ends, by one edge each"
    );
    let (right, far, up, top) = (
        rib.right(),
        rib.right() + taken,
        PLATE + taken,
        PLATE + rib.height,
    );
    for end in [RIB_FRONT, RIB_BACK] {
        if beneath == TOP {
            one_edge_between(
                body,
                [TOP, end],
                [&[TOP, end, RIB_LEFT], &[TOP, end, made]],
                &format!("{what}: the foot of face {end}, run on along the fill's footprint"),
            );
        } else {
            assert_eq!(
                edges_between(body, TOP, end),
                1,
                "{what}: the plate's top meets face {end} along one edge"
            );
        }
        no_corner_where_only(
            body,
            &[TOP, end],
            &format!("{what}: the corner the foot of the rib's right side ended at, at {end}"),
        );
    }
    for (_, face, _) in rib.rounds() {
        one_edge_between(
            body,
            [face, RIB_RIGHT],
            [&[face, RIB_RIGHT, made], &[RIB_TOP, face, RIB_RIGHT]],
            &format!("{what}: the round {face}'s edge with the rib's right side, above the fill"),
        );
    }
    let (front, back) = (RIB_ENDS[0], rib.back_at(rib.left));
    corners_are(
        body,
        RIB_RIGHT,
        &[
            DVec3::new(right, front, up),
            DVec3::new(right, front, top),
            DVec3::new(right, back, up),
            DVec3::new(right, back, top),
        ],
        &format!("{what}: what is left of the rib's right side"),
    );
    let rounds = rib.rounds();
    corners_are_or_on(
        body,
        TOP,
        &[
            DVec3::new(0.0, 0.0, PLATE),
            DVec3::new(40.0, 0.0, PLATE),
            DVec3::new(40.0, 30.0, PLATE),
            DVec3::new(0.0, 30.0, PLATE),
            DVec3::new(rib.left, front, PLATE),
            DVec3::new(far, rib.front_at(far), PLATE),
            DVec3::new(far, rib.back_at(far), PLATE),
            DVec3::new(rib.left, back, PLATE),
        ],
        |corner| {
            rounds
                .iter()
                .any(|(round, ..)| round.off(corner).abs() < PLACE)
                && (corner.z - PLATE).abs() < PLACE
                && (rib.left - PLACE..=far + PLACE).contains(&corner.x)
        },
        &format!(
            "{what}: the plate's top, one face round the rib and the fill's footprint, its edges \
             with the rounds on their foot circles"
        ),
    );
    for (round, face, outward) in rounds {
        let whose = format!("{what}: the round {face}");
        drawn_on_the_cylinder(body, face, (round.axis, round.radius), &whose);
        let furthest = furthest_along_x(body, face);
        assert!(
            (furthest - far).abs() < PLACE,
            "{whose} reaches {furthest} along X, where it takes in the fill's section out to {far}"
        );
        let outside = if outward < 0.0 { -1.0 } else { 31.0 };
        for share_out in [0.2, 0.5] {
            let out = taken * share_out;
            for share_up in [0.25, 0.5, 0.75] {
                let from = DVec3::new(right + out, outside, PLATE + height(out) * share_up);
                let hit = body.ray_hit(from, -DVec3::Y * outward).unwrap_or_else(|| {
                    panic!("{whose}: a ray from {from} through the fill's section meets nothing")
                });
                let at = from - DVec3::Y * outward * hit.distance;
                assert!(
                    hit.face == face && round.off(at).abs() <= MET,
                    "{whose}: a ray from {from} through the fill's section meets face {} at {at}, \
                     {} off the round",
                    hit.face,
                    round.off(at)
                );
            }
        }
        for corner in body.triangles_of(made).flatten() {
            assert!(
                (corner.y - round.axis.y) * outward <= 0.0 || round.off(corner) <= PLACE,
                "{whose}: the fill is drawn through {corner}, {} outside the round: past it",
                round.off(corner)
            );
        }
    }
}

/// Which step a fill is laid by.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Fill {
    Chamfer,
    Fillet,
}

/// Holds that `fill`, asked `asked` at the foot of `rib`'s right side, takes
/// `taken` — reporting both — and leaves a closed, sound body of the part's
/// matter and the fill's, one face more numbered after the steps' last, the
/// fill flat or curved as its step lays it, and flush on the rib's ends as
/// [`flush_on_the_rib_s_ends`] holds, lying on `beneath`.
fn filled_flush_on_the_rib_s_ends(
    (rib, fill): (Rib, Fill),
    [asked, taken]: [f64; 2],
    beneath: usize,
    what: &str,
) {
    let part = rib.on();
    the_rib_stands_as_drawn(&part, rib, &format!("{what}, before the step"));
    let foot = edge_between(&part, TOP, RIB_RIGHT);

    let laid = match fill {
        Fill::Chamfer => part
            .chamfer(&foot, asked)
            .map(|chamfered| (chamfered.body, chamfered.asked, chamfered.taken)),
        Fill::Fillet => part
            .fillet(&foot, asked)
            .map(|filleted| (filleted.body, filleted.asked, filleted.taken)),
    };
    let (body, reported_asked, reported_taken) =
        laid.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    reduced(reported_asked, reported_taken, [asked, taken], what);
    let height = |out: f64| match fill {
        Fill::Chamfer => chamfer_height(taken, out),
        Fill::Fillet => fillet_height(taken, out),
    };
    let laid_area = match fill {
        Fill::Chamfer => 0.0,
        Fill::Fillet => PI / 2.0 * taken * rib.along(0.0).max(rib.along(taken)),
    };
    sound_drawn_within(
        &body,
        rib.volume() + fill_matter(taken, height, |out, _| rib.along(out)),
        drawn_within(&part, rib.rounds_area() + taken * taken + laid_area),
        what,
    );
    let made = match fill {
        Fill::Chamfer => one_flat_face_more(&part, &body, what),
        Fill::Fillet => one_round_more(&part, &body, what),
    };
    numbered_after_the_steps_last(&part, &body, made, what);
    flush_on_the_rib_s_ends(&body, (rib, made), taken, height, beneath, what);
    if beneath == PLATE_RIGHT {
        corners_are(
            &body,
            PLATE_RIGHT,
            &[
                DVec3::new(40.0, 0.0, 0.0),
                DVec3::new(40.0, 30.0, 0.0),
                DVec3::new(40.0, 30.0, PLATE),
                DVec3::new(40.0, rib.back_at(40.0), PLATE),
                DVec3::new(40.0, rib.front_at(40.0), PLATE),
                DVec3::new(40.0, 0.0, PLATE),
            ],
            &format!("{what}: the plate's right side, the fill's foot along its top"),
        );
    }
}

/// Holds that a step reports the value it was asked, `asked`, and the one it
/// took, `taken`, as `[was_asked, gave]` say.
fn reduced(asked: f64, taken: f64, [was_asked, gave]: [f64; 2], what: &str) {
    assert!(
        (asked - was_asked).abs() < PLACE,
        "{what}: reports {asked} asked, where {was_asked} was"
    );
    assert!(
        (taken - gave).abs() < PLACE,
        "{what}: took {taken}, where the faces give {gave}"
    );
}

/// A rib whose front bulges gently: a round of 50 through its front corners,
/// its axis past the plate's back, standing a twenty-fifth forward of them at
/// its middle and meeting the rib's sides some 2.3 degrees off square.
fn gently_bulging() -> Rib {
    Rib {
        front: Round::through(20.0, RIB_ENDS[0], 50.0, 1.0),
        ..CRITERION_1
    }
}

#[test]
fn a_chamfer_at_the_foot_of_a_rib_whose_end_bulges_gently_in_a_round_of_large_radius_is_trimmed_flush_on_the_round()
 {
    filled_flush_on_the_rib_s_ends(
        (gently_bulging(), Fill::Chamfer),
        [3.0, 3.0],
        TOP,
        "the chamfer at the foot of a rib bulging gently in a round of 50",
    );
}

#[test]
fn a_fillet_at_the_foot_of_a_rib_whose_end_bulges_gently_in_a_round_of_large_radius_is_trimmed_flush_on_the_round()
 {
    filled_flush_on_the_rib_s_ends(
        (gently_bulging(), Fill::Fillet),
        [3.0, 3.0],
        TOP,
        "the fillet at the foot of a rib bulging gently in a round of 50",
    );
}

/// Criterion 1's rib moved to the plate's right, from x = 34 to 38, its
/// round moved with it: the plate's top beyond its right side 2 wide.
fn near_the_plate_s_side() -> Rib {
    Rib {
        left: 34.0,
        front: Round::through(36.0, RIB_ENDS[0], RADIUS, 1.0),
        ..CRITERION_1
    }
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_near_the_plate_s_side_asked_more_than_the_plate_gives_take_the_most_it_gives_and_stay_flush_on_the_round()
 {
    for (fill, what) in [
        (
            Fill::Chamfer,
            "the chamfer of 3 at the foot of a rib 2 short of the plate's side",
        ),
        (
            Fill::Fillet,
            "the fillet of 3 at the foot of a rib 2 short of the plate's side",
        ),
    ] {
        filled_flush_on_the_rib_s_ends(
            (near_the_plate_s_side(), fill),
            [3.0, 2.0],
            PLATE_RIGHT,
            what,
        );
    }
}

/// Criterion 1's rib with its back rounded as its front is: a round of
/// [`RADIUS`] through its back corners at y = 22, bulging a quarter behind
/// them.
fn rounded_at_both_ends() -> Rib {
    Rib {
        back: Back::Round(Round::through(20.0, 22.0, RADIUS, -1.0)),
        ..CRITERION_1
    }
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_rounded_at_both_ends_are_trimmed_flush_on_each_round()
 {
    for (fill, what) in [
        (
            Fill::Chamfer,
            "the chamfer at the foot of a rib rounded at both ends",
        ),
        (
            Fill::Fillet,
            "the fillet at the foot of a rib rounded at both ends",
        ),
    ] {
        filled_flush_on_the_rib_s_ends((rounded_at_both_ends(), fill), [3.0, 3.0], TOP, what);
    }
}

/// The furthest the midpoint of any side of any triangle of the face
/// numbered `face`, whose corners lie on its surface, stands off it, `off`
/// saying how far a point stands off it: the sag of its chords.
fn furthest_chord(body: &Body, face: usize, off: impl Fn(DVec3) -> f64) -> f64 {
    body.triangles_of(face)
        .flat_map(|[a, b, c]| [(a + b) / 2.0, (b + c) / 2.0, (c + a) / 2.0])
        .map(|middle| off(middle).abs())
        .fold(0.0, f64::max)
}

/// Criterion 1's rib moved to x = 32 to 36, its round moved with it: a fill
/// of 3 at the foot of its right side reaches 39, inside the plate's box.
fn moved_toward_the_plate_s_side() -> Rib {
    Rib {
        left: 32.0,
        front: Round::through(34.0, RIB_ENDS[0], RADIUS, 1.0),
        ..CRITERION_1
    }
}

#[test]
fn a_chamfer_and_a_fillet_up_to_a_round_end_leave_the_part_drawn_as_finely_as_its_own_box_asks() {
    let value = 3.0;
    for (rib, where_it_stands) in [
        (CRITERION_1, "where criterion 1 stands it"),
        (
            moved_toward_the_plate_s_side(),
            "moved toward the plate's side",
        ),
        (
            near_the_plate_s_side(),
            "beside the plate's side, the fill taking the top's whole width there",
        ),
    ] {
        let part = rib.on();
        let what = format!("the rib {where_it_stands}");
        the_rib_stands_as_drawn(&part, rib, &format!("{what}, before the step"));
        let fine = DRAWN * reach_of(&part) * (1.0 + 1e-9);
        let on_the_round = |point: DVec3| rib.front.off(point);
        let sag = furthest_chord(&part, RIB_FRONT, on_the_round);
        assert!(
            sag <= fine,
            "{what}, before the step: its round's chords stand {sag} off it, where its box asks \
             {fine}"
        );
        let foot = edge_between(&part, TOP, RIB_RIGHT);

        let chamfered = part
            .chamfer(&foot, value)
            .unwrap_or_else(|declined| panic!("{what}: the chamfer declined, {declined:?}"));
        let filleted = part
            .fillet(&foot, value)
            .unwrap_or_else(|declined| panic!("{what}: the fillet declined, {declined:?}"));

        let radius = filleted.taken;
        let hollow = |point: DVec3| {
            DVec2::new(point.x, point.z).distance(DVec2::new(rib.right() + radius, PLATE + radius))
                - radius
        };
        let made = part.faces_end();
        for (step, body, laid) in [
            ("the chamfer", &chamfered.body, None),
            ("the fillet", &filleted.body, Some(made)),
        ] {
            let sag = furthest_chord(body, RIB_FRONT, on_the_round);
            assert!(
                sag <= fine,
                "{what}, {step}: the round's chords stand {sag} off it, where the part's box asks \
                 {fine} — the step drew the part coarser than its own size does"
            );
            if let Some(laid) = laid {
                let sag = furthest_chord(body, laid, hollow);
                assert!(
                    sag <= fine,
                    "{what}, {step}: the fillet's chords stand {sag} off it, where the part's box \
                     asks {fine} — the step drew the part coarser than its own size does"
                );
            }
        }
    }
}

/// The plate with a slot whose round end meets both its walls off tangent,
/// turning in toward the slot: the slot of [`slot_in_the_plate`], its round
/// end the arc about [`slot_axis`] from the front's end at (28, 10) to the
/// back's at (28, 14), some 174 degrees, leaving each wall at a corner of
/// some 2.9 degrees. Past either floor edge's end the wall's line runs out
/// of the round's cylinder.
fn slot_turning_in_at_both_walls() -> Body {
    let slot = Contour {
        corners: vec![
            DVec2::new(-5.0, SLOT_FRONT_Y),
            DVec2::new(SLOT_FRONT_END, SLOT_FRONT_Y),
            DVec2::new(SLOT_FRONT_END, SLOT_FRONT_Y + 4.0),
            DVec2::new(-5.0, SLOT_FRONT_Y + 4.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: slot_axis(),
                turn: 2.0 * 2f64.atan2(SLOT_FRONT_END - slot_axis().x),
            },
            Run::Straight,
            Run::Straight,
        ],
    };
    cut_from(&plate(SLOT_PLATE), slot, SLOT_FLOOR_Z, SLOT_PLATE)
}

/// The matter the [`slot_turning_in_at_both_walls`] holds: the plate's,
/// less the slot's, 6 deep — 28 by 4 to the round's chord, and the segment
/// beyond it.
fn slot_turning_in_volume() -> f64 {
    let turn = 2.0 * 2f64.atan2(SLOT_FRONT_END - slot_axis().x);
    let segment = slot_radius().powi(2) * (turn - turn.sin()) / 2.0;
    40.0 * 30.0 * SLOT_PLATE - (SLOT_FRONT_END * 4.0 + segment) * (SLOT_PLATE - SLOT_FLOOR_Z)
}

/// The names of the slot's two floor edges, along its front and along its
/// back.
fn slot_floor_edges(part: &Body) -> [EdgeName; 2] {
    [
        edge_between(part, SLOT_FLOOR, SLOT_FRONT),
        edge_between(part, SLOT_FLOOR, SLOT_BACK),
    ]
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

/// Holds that the [`slot_turning_in_at_both_walls`] is what the steps are
/// given: its faces numbered up to [`STEPS_END`], its round end on its
/// cylinder, each floor edge one edge from the plate's left side to the
/// round, the wall's line running out of the round's cylinder past each, and
/// its matter the plate's less the slot's.
fn the_slot_turns_in_at_both_walls(part: &Body, what: &str) {
    assert_eq!(part.faces_end(), STEPS_END, "{what}: its steps' numbers");
    drawn_on_the_cylinder(part, SLOT_END, (slot_axis(), slot_radius()), what);
    for (wall, y) in [(SLOT_FRONT, SLOT_FRONT_Y), (SLOT_BACK, SLOT_FRONT_Y + 4.0)] {
        one_edge_between(
            part,
            [SLOT_FLOOR, wall],
            [
                &[SLOT_FLOOR, wall, PLATE_LEFT],
                &[SLOT_FLOOR, wall, SLOT_END],
            ],
            &format!("{what}: the foot of wall {wall}"),
        );
        let past = DVec2::new(SLOT_FRONT_END + 0.01, y);
        assert!(
            past.distance(slot_axis()) > slot_radius(),
            "{what}: just past the foot of wall {wall}, at {past}, its line is out of the round"
        );
    }
    sound_drawn_within(
        part,
        slot_turning_in_volume(),
        drawn_within(part, slot_end_area()),
        what,
    );
}

/// The height the fill across the slot's floor stands over it at `y`: the
/// fill of `height` from the front's foot up to the floor's middle, and the
/// one from the back's beyond it.
fn across_the_floor(y: f64, height: impl Fn(f64) -> f64) -> f64 {
    let middle = SLOT_FRONT_Y + 2.0;
    if y <= middle {
        height((y - SLOT_FRONT_Y).max(0.0))
    } else {
        height((SLOT_FRONT_Y + 4.0 - y).max(0.0))
    }
}

/// Holds what the two fills across the floor of the
/// [`slot_turning_in_at_both_walls`], `laid` the faces they make, each of
/// section `height(out)` `out` from its wall, leave where they meet the
/// round end: the round end, one face still, drawn on its cylinder alone,
/// no corner of it under the fills' surface and corners where the fills
/// reach its walls; each wall cut short above its fill, and meeting the
/// round end along one edge from there up, no corner left where the two
/// alone meet; the plate's left side flush on the fills' ends, `on_the_ends`
/// saying which of its corners lie on them; every ray down onto the fills a
/// tenth short of the round meeting them; and nothing of them past the
/// round.
fn met_across_the_floor_flush_on_the_round(
    body: &Body,
    laid: &[usize],
    height: impl Fn(f64) -> f64 + Copy,
    on_the_ends: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let (front, back, floor, half) = (SLOT_FRONT_Y, SLOT_FRONT_Y + 4.0, SLOT_FLOOR_Z, 2.0);
    let up = floor + half;
    drawn_on_the_cylinder(body, SLOT_END, (slot_axis(), slot_radius()), what);
    let round: Vec<DVec3> = body.triangles_of(SLOT_END).flatten().collect();
    for corner in &round {
        let under = floor + across_the_floor(corner.y, height);
        assert!(
            corner.z >= under - PLACE,
            "{what}: the round end is drawn through {corner}, under the fills' surface at \
             {under}: it still stands where the fills cover it"
        );
    }
    for y in [front, back] {
        let corner = DVec3::new(SLOT_FRONT_END, y, up);
        assert!(
            round.iter().any(|drawn| drawn.distance(corner) < PLACE),
            "{what}: the round end has no corner at {corner}, where a fill reaches its wall"
        );
    }
    for (wall, y) in [(SLOT_FRONT, front), (SLOT_BACK, back)] {
        corners_are(
            body,
            wall,
            &[
                DVec3::new(0.0, y, up),
                DVec3::new(SLOT_FRONT_END, y, up),
                DVec3::new(SLOT_FRONT_END, y, SLOT_PLATE),
                DVec3::new(0.0, y, SLOT_PLATE),
            ],
            &format!("{what}: what is left of wall {wall}"),
        );
        assert_eq!(
            edges_between(body, wall, SLOT_END),
            1,
            "{what}: wall {wall} meets the round end along one edge, from its fill up"
        );
        no_corner_where_only(
            body,
            &[wall, SLOT_END],
            &format!("{what}: the corner the foot of wall {wall} ended at"),
        );
    }
    assert!(
        !body.has_face(SLOT_FLOOR),
        "{what}: the slot's floor is gone, under the fills"
    );
    corners_are_or_on(
        body,
        PLATE_LEFT,
        &[
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 30.0, 0.0),
            DVec3::new(0.0, 30.0, SLOT_PLATE),
            DVec3::new(0.0, back, SLOT_PLATE),
            DVec3::new(0.0, back, up),
            DVec3::new(0.0, front, up),
            DVec3::new(0.0, front, SLOT_PLATE),
            DVec3::new(0.0, 0.0, SLOT_PLATE),
        ],
        on_the_ends,
        &format!("{what}: the plate's left side, its notch's foot filled flush"),
    );
    for y in [front + 0.4, front + 1.0, back - 1.0, back - 0.4] {
        let from = DVec3::new(on_the_slot_s_end(y) - SHORT_OF_THE_END, y, SLOT_PLATE - 0.5);
        let hit = body.ray_hit(from, -DVec3::Z).unwrap_or_else(|| {
            panic!("{what}: a ray down from {from}, short of the round end, meets nothing")
        });
        let at = from - DVec3::Z * hit.distance;
        let from_the_wall = (y - front).min(back - y);
        let off = off_the_surface(height, from_the_wall, at.z - floor);
        assert!(
            laid.contains(&hit.face) && off.abs() <= MET,
            "{what}: a ray down from {from}, short of the round end, meets face {} at {at}, {off} \
             off the fills' surface",
            hit.face
        );
    }
    for &face in laid {
        for corner in body.triangles_of(face).flatten() {
            let off = corner.truncate().distance(slot_axis());
            assert!(
                corner.x <= slot_axis().x || off <= slot_radius() + PLACE,
                "{what}: a fill is drawn through {corner}, {off} from the round's axis: past the \
                 round end"
            );
        }
    }
}

#[test]
fn a_chamfer_on_both_floor_edges_of_a_slot_whose_round_end_meets_both_walls_at_a_few_degrees_meets_in_a_valley_flush_on_the_round()
 {
    let half = 2.0;
    let part = slot_turning_in_at_both_walls();
    let what = "the chamfer on both floor edges of the slot turning in at both walls";
    the_slot_turns_in_at_both_walls(&part, &format!("{what}, before the step"));

    let chamfered = part
        .chamfer_edges(&slot_floor_edges(&part), half)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    reduced(chamfered.asked, chamfered.taken, [half, half], what);
    let body = &chamfered.body;
    let height = |out: f64| chamfer_height(half, out);
    sound_drawn_within(
        body,
        slot_turning_in_volume() + 2.0 * fill_matter(half, height, |out, _| along_the_slot(out)),
        drawn_within(&part, slot_end_area() + 2.0 * half * half),
        what,
    );
    let laid = [STEPS_END, STEPS_END + 1];
    let mut kept_and_laid = numbers(&part);
    kept_and_laid.remove(&SLOT_FLOOR);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the part but the slot's floor, and a flat for each edge, numbered \
         after the steps' last"
    );
    assert_eq!(
        body.faces_end(),
        STEPS_END + 2,
        "{what}: the steps' numbers"
    );
    let valley = SLOT_FRONT_Y + half;
    let deepest = DVec3::new(on_the_slot_s_end(valley), valley, SLOT_FLOOR_Z);
    for (flat, wall, other, y) in [
        (laid[0], SLOT_FRONT, laid[1], SLOT_FRONT_Y),
        (laid[1], SLOT_BACK, laid[0], SLOT_FRONT_Y + 4.0),
    ] {
        assert!(body.is_flat(flat), "{what}: face {flat} is flat");
        let mut sides = [PLATE_LEFT, wall, SLOT_END, other].map(|side| vec![side as u32]);
        sides.sort();
        assert_eq!(
            sides_met(body, flat),
            sides,
            "{what}: the flat from wall {wall} meets its wall, the plate's left side, the round \
             end and the other flat along the valley, by one edge each"
        );
        corners_are_or_on(
            body,
            flat,
            &[
                DVec3::new(0.0, y, SLOT_FLOOR_Z + half),
                DVec3::new(0.0, valley, SLOT_FLOOR_Z),
                DVec3::new(SLOT_FRONT_END, y, SLOT_FLOOR_Z + half),
                deepest,
            ],
            |corner| (corner.truncate().distance(slot_axis()) - slot_radius()).abs() < PLACE,
            &format!("{what}: the flat from wall {wall}, its end on the round end"),
        );
    }
    met_across_the_floor_flush_on_the_round(
        body,
        &laid,
        height,
        |corner| corner.x.abs() < PLACE && (corner.y - valley).abs() < PLACE,
        what,
    );
    assert!(
        body.triangles_of(SLOT_END)
            .flatten()
            .any(|corner| corner.distance(deepest) < PLACE),
        "{what}: the round end has a corner at {deepest}, where the valley ends on it"
    );
}

#[test]
fn a_fillet_on_both_floor_edges_of_a_slot_whose_round_end_meets_both_walls_at_a_few_degrees_lays_one_round_bottom_flush_on_the_round()
 {
    let half = 2.0;
    let part = slot_turning_in_at_both_walls();
    let what = "the fillet on both floor edges of the slot turning in at both walls";
    the_slot_turns_in_at_both_walls(&part, &format!("{what}, before the step"));

    let filleted = part
        .fillet_edges(&slot_floor_edges(&part), half)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    reduced(filleted.asked, filleted.taken, [half, half], what);
    let body = &filleted.body;
    let height = |out: f64| fillet_height(half, out);
    sound_drawn_within(
        body,
        slot_turning_in_volume() + 2.0 * fill_matter(half, height, |out, _| along_the_slot(out)),
        drawn_within(
            &part,
            slot_end_area() + 2.0 * half * half + PI * half * along_the_slot(half),
        ),
        what,
    );
    let laid = [STEPS_END, STEPS_END + 1];
    let mut kept_and_laid = numbers(&part);
    kept_and_laid.remove(&SLOT_FLOOR);
    kept_and_laid.extend(laid);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the part but the slot's floor, and the round bottom answering to \
         {laid:?}"
    );
    assert_eq!(
        body.faces_end(),
        STEPS_END + 2,
        "{what}: the steps' numbers"
    );
    let drawn: Vec<Vec<[DVec3; 3]>> = laid
        .iter()
        .map(|&number| body.triangles_of(number).collect())
        .collect();
    assert!(
        !drawn[0].is_empty() && drawn[0] == drawn[1] && !body.is_flat(laid[0]),
        "{what}: faces {laid:?} are one curved face, the round bottom"
    );
    let mut sides = [PLATE_LEFT, SLOT_FRONT, SLOT_END, SLOT_BACK].map(|side| vec![side as u32]);
    sides.sort();
    assert_eq!(
        sides_met_by(body, &laid),
        sides,
        "{what}: the round bottom meets the slot's two walls along its sides, and the plate's \
         left side and the round end at its ends, by one edge each"
    );
    let axis = DVec2::new(SLOT_FRONT_Y + half, SLOT_FLOOR_Z + half);
    met_across_the_floor_flush_on_the_round(
        body,
        &laid,
        height,
        |corner| {
            corner.x.abs() < PLACE
                && (DVec2::new(corner.y, corner.z).distance(axis) - half).abs() < PLACE
        },
        what,
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_both_floor_edges_of_a_slot_whose_back_wall_runs_into_its_round_end_are_declined_as_unsupported()
 {
    let part = slot_in_the_plate();
    the_slot_is_cut_in_the_plate(&part);
    let back_end = DVec2::new(SLOT_FRONT_END + 2.0 * SLOT_OFF, SLOT_FRONT_Y + 4.0);
    let past = back_end + DVec2::X * 0.01;
    assert!(
        past.distance(slot_axis()) < slot_radius(),
        "the slot turning in at its front alone: just past the back's foot, at {past}, its line \
         runs into the round"
    );
    let names = slot_floor_edges(&part);

    let declined = [
        ("the chamfer", part.chamfer_edges(&names, 2.0).err()),
        ("the fillet", part.fillet_edges(&names, 2.0).err()),
    ];

    for (step, declined) in declined {
        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "{step} of 2 on both floor edges of a slot whose back runs into its round end"
        );
    }
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_slot_s_wall_running_on_tangentially_into_its_round_end_are_declined_as_unsupported()
 {
    let part = slot_off_tangent_by(0.0);
    let what = "the slot whose round end is tangent to its walls, before any step";
    let (axis, radius) = (slot_axis_off_by(0.0), slot_radius_off_by(0.0));
    drawn_on_the_cylinder(&part, SLOT_END, (axis, radius), what);
    assert!(
        (axis.x - SLOT_FRONT_END).abs() < PLACE && (radius - 2.0).abs() < PLACE,
        "{what}: the round's axis stands square to the front's end, half the slot's width back, \
         so that it leaves the front tangentially"
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
        slot_volume_off_by(0.0),
        drawn_within(&part, PI * radius * (SLOT_PLATE - SLOT_FLOOR_Z)),
        what,
    );

    both_declined(
        &part,
        [SLOT_FLOOR, SLOT_FRONT],
        1.0,
        "at the foot of a slot's front that runs on tangentially into its round end, #698's chain",
    );
}

/// The number the [`on_the_ramp`] plate's sloping top answers to: its fourth
/// wall, raised from the third run of its section.
const RAMP: usize = 4;

/// How the [`on_the_ramp`] plate's top rises along Y, for every unit along:
/// some 11.3 degrees.
const RISE: f64 = 0.2;

/// Where a rib on the [`on_the_ramp`] plate stands: its back upright at
/// y = 19 and its top level at z = 15.
const RAMP_RIB_BACK: f64 = 19.0;
const RAMP_RIB_TOP: f64 = 15.0;

/// The height of the [`on_the_ramp`] plate's top over the ground at `y`.
fn ramp_at(y: f64) -> f64 {
    2.0 + RISE * y
}

/// The way along the foot of the rib's right side, up the ramp, and the way
/// square to the ramp, out of it.
fn up_the_ramp() -> DVec3 {
    DVec3::new(0.0, 1.0, RISE).normalize()
}

fn off_the_ramp() -> DVec3 {
    DVec3::new(0.0, -RISE, 1.0).normalize()
}

/// The radius of the round end of the rib on the ramp with `draft` at height
/// `z`: criterion 1's round where the ramp meets it at the rib's front
/// corners, at y = 5, drawn in by `draft` for every unit up — none for an
/// upright cylinder.
fn ramp_round_radius(draft: f64, z: f64) -> f64 {
    RADIUS - draft * (z - ramp_at(RIB_ENDS[0]))
}

/// How far a point stands off the round end of the rib on the ramp with
/// `draft`, out from its axis across its height.
fn off_the_ramp_round(draft: f64, point: DVec3) -> f64 {
    off_the_axis(point) - ramp_round_radius(draft, point.z)
}

/// A plate 40 long whose top rises along Y as a ramp, from 2 over the ground
/// at its front to 8 at its back, 30 behind — raised along X from its
/// section, so that its faces number: the section at x = 0 nought, at
/// x = 40 one, then its floor two, its back three, the ramp four and its
/// front five — and on it a rib standing upright: 4 wide from x = 18 to 22,
/// from below the ramp up to z = 15, its back upright at y = 19, its front
/// cut by a ring turned whole about the upright line through [`AXIS`], whose
/// inner wall meets the ramp at the rib's front corners where criterion 1's
/// round meets the plate, and draws in by `draft` for every unit up: an
/// upright cylinder at no draft, a cone otherwise. The foot of the rib's
/// right side runs up the ramp, oblique to the round's axis, and runs out of
/// the round at its front end, where the matter lies inside it. The ring is
/// drawn on a plane whose origin stands where the ramp meets the axis's line
/// at the height of the rib's front corners, not on the ground, so that
/// nothing reckoned from the round's origin up its axis comes out at nought.
fn on_the_ramp(draft: f64) -> Body {
    let section = Frame {
        origin: DVec3::ZERO,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let plate = raised_on(
        section,
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, ramp_at(30.0)),
            DVec2::new(0.0, ramp_at(0.0)),
        ]),
        40.0,
    );
    let below_the_ramp = Frame {
        origin: DVec3::Z,
        ..ground()
    };
    let rib = plate
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(18.0, 4.0),
                DVec2::new(RIB_SIDE, RAMP_RIB_BACK),
            )),
            below_the_ramp,
            DVec3::Z * (RAMP_RIB_TOP - 1.0),
        )
        .expect("the rib raises");
    let (low, high, from) = (0.0, RAMP_RIB_TOP + 1.0, ramp_at(RIB_ENDS[0]));
    let ring = turned_about_the_round_s_axis_from(
        &rib,
        Contour::straight(vec![
            DVec2::new(ramp_round_radius(draft, high), high - from),
            DVec2::new(ramp_round_radius(draft, low), low - from),
            DVec2::new(12.0, low - from),
            DVec2::new(12.0, high - from),
        ]),
        from,
    );
    let rounded = rib.difference(&ring).expect("the round end is cut");
    plate.union(&rounded).expect("the rib is joined")
}

/// Where the line through `from` running up the ramp, along
/// [`up_the_ramp`], meets the round end of the rib on the ramp with `draft`
/// at its front: how far along that line, from `from`. The nearer root of
/// the quadratic the line's distance from the axis squared less the round's
/// radius squared makes along it.
fn into_the_ramp_round(draft: f64, from: DVec3) -> f64 {
    let way = up_the_ramp();
    let off = (from - AXIS.extend(0.0)).truncate();
    let across = way.truncate();
    let (radius, shrink) = (ramp_round_radius(draft, from.z), draft * way.z);
    let square = across.length_squared() - shrink * shrink;
    let half = off.dot(across) + radius * shrink;
    let rest = off.length_squared() - radius * radius;
    (-half - (half * half - square * rest).sqrt()) / square
}

/// The point of the fill's section `out` along the ramp from the rib's right
/// side and `up` off the ramp, on the line through the foot's point at
/// y = 0.
fn on_the_ramp_section(out: f64, up: f64) -> DVec3 {
    DVec3::new(RIB_SIDE + out, 0.0, ramp_at(0.0)) + off_the_ramp() * up
}

/// How long a fill at the foot of the right side of the rib on the ramp with
/// `draft` runs, `out` from that side and `up` off the ramp, measured up
/// the ramp: from where that line meets the round to where it meets the
/// rib's upright back.
fn along_the_ramp_rib(draft: f64, out: f64, up: f64) -> f64 {
    let from = on_the_ramp_section(out, up);
    let to_the_back = (RAMP_RIB_BACK - from.y) / up_the_ramp().y;
    to_the_back - into_the_ramp_round(draft, from)
}

/// Where the round end of the rib on the ramp with `draft` stands at `x` and
/// height `z`, read in plan: the y of its point there in front of its axis.
fn on_the_ramp_round(draft: f64, x: f64, z: f64) -> f64 {
    AXIS.y - (ramp_round_radius(draft, z).powi(2) - (x - AXIS.x).powi(2)).sqrt()
}

/// The matter the [`on_the_ramp`] part with `draft` holds: the plate's, 40
/// times its section; and the rib's above the ramp, column by column across
/// its plan — from the ramp up to the rib's top behind where the round stands
/// at that top, and up to the round in front of it, down to where the round
/// meets the ramp — summed by Simpson's rule.
fn on_the_ramp_volume(draft: f64) -> f64 {
    let plate = 40.0 * (ramp_at(0.0) + ramp_at(30.0)) / 2.0 * 30.0;
    let column = |x: f64| {
        let behind = on_the_ramp_round(draft, x, RAMP_RIB_TOP);
        let to_the_top = (RAMP_RIB_TOP - 2.0) * (RAMP_RIB_BACK - behind)
            - RISE * (RAMP_RIB_BACK.powi(2) - behind.powi(2)) / 2.0;
        if draft == 0.0 {
            return to_the_top;
        }
        let under = |y: f64| {
            let off = DVec2::new(x, y).distance(AXIS);
            ramp_at(RIB_ENDS[0]) + (RADIUS - off) / draft
        };
        let (mut front, mut back) = (AXIS.y - RADIUS - 2.0, behind);
        for _ in 0..200 {
            let middle = (front + back) / 2.0;
            if under(middle) > ramp_at(middle) {
                back = middle;
            } else {
                front = middle;
            }
        }
        to_the_top + simpson(|y| under(y) - ramp_at(y), [back, behind], STEPS)
    };
    plate + simpson(column, [18.0, RIB_SIDE], STEPS)
}

/// The area of the [`on_the_ramp`] rib's round end, rounded up: an arc no
/// longer than 4.2 between the rib's sides, up a slant no taller than 13 and
/// a hair.
fn ramp_round_area(draft: f64) -> f64 {
    4.2 * 13.0 * (1.0 + draft * draft).sqrt()
}

/// Holds that the [`on_the_ramp`] part with `draft` is what the steps are
/// given: its faces numbered up to [`CONE_STEPS_END`], the round end its
/// ring's inner wall, drawn on itself and stopping at the rib's right side;
/// the foot of the right side one edge, between the round and the rib's
/// back, rising along the ramp; and its matter the plate's and the rib's.
fn the_rib_stands_on_the_ramp(part: &Body, draft: f64, what: &str) {
    assert_eq!(
        part.faces_end(),
        CONE_STEPS_END,
        "{what}: its steps numbered their faces up to {CONE_STEPS_END}"
    );
    assert!(
        part.has_face(CONE_FRONT) && !part.is_flat(CONE_FRONT) && part.is_flat(RAMP),
        "{what}: the round end, face {CONE_FRONT}, is curved, and the ramp, face {RAMP}, flat"
    );
    for corner in part.triangles_of(CONE_FRONT).flatten() {
        assert!(
            off_the_ramp_round(draft, corner).abs() < PLACE,
            "{what}: the round end is drawn through {corner}, {} off it",
            off_the_ramp_round(draft, corner)
        );
    }
    let furthest = furthest_along_x(part, CONE_FRONT);
    assert!(
        (furthest - RIB_SIDE).abs() < PLACE,
        "{what}: the round end reaches {furthest} along X, where the rib's right side stands at \
         {RIB_SIDE}"
    );
    one_edge_between(
        part,
        [RAMP, RIB_RIGHT],
        [&[RAMP, RIB_RIGHT, CONE_FRONT], &[RAMP, RIB_RIGHT, RIB_BACK]],
        &format!("{what}: the foot of the rib's right side"),
    );
    let ends = [
        DVec3::new(RIB_SIDE, RIB_ENDS[0], ramp_at(RIB_ENDS[0])),
        DVec3::new(RIB_SIDE, RAMP_RIB_BACK, ramp_at(RAMP_RIB_BACK)),
    ];
    let side = part
        .plane_of(RIB_RIGHT)
        .expect("the rib's right side is flat");
    for end in ends {
        assert!(
            side.corners
                .iter()
                .any(|corner| corner.distance(end) < PLACE),
            "{what}: the foot of the rib's right side ends at {end}, up the ramp: {:?}",
            side.corners
        );
    }
    sound_drawn_within(
        part,
        on_the_ramp_volume(draft),
        drawn_within(part, ramp_round_area(draft)),
        what,
    );
}

/// Holds that `fill` of 3 at the foot of the right side of the rib on the
/// ramp with `draft` builds and is trimmed flush on the round end, as
/// criterion 1 holds it: the value taken whole; a closed, sound body of the
/// part's matter and the fill's; one face more, numbered after the steps'
/// last; the fill meeting the ramp and the rib's right side along its sides,
/// and the round and the rib's back at its ends, by one edge each; the ramp
/// meeting the round along one edge, from the rib's left side to the fill,
/// no corner left where the two alone meet; the right side meeting the round
/// along one edge from the fill's top up; the round one face still, drawn on
/// itself alone and reaching out to the fill's far side; every ray aimed
/// along Y from in front of the rib through a place of the fill's section,
/// where that place's line up the ramp meets the round, meeting the round
/// first and on it; and no corner of the fill in front of the round's axis
/// standing outside it.
fn filled_up_the_ramp(draft: f64, fill: Fill, what: &str) {
    let value = 3.0;
    let part = on_the_ramp(draft);
    the_rib_stands_on_the_ramp(&part, draft, &format!("{what}, before the step"));
    let foot = edge_between(&part, RAMP, RIB_RIGHT);

    let laid = match fill {
        Fill::Chamfer => part
            .chamfer(&foot, value)
            .map(|chamfered| (chamfered.body, chamfered.asked, chamfered.taken)),
        Fill::Fillet => part
            .fillet(&foot, value)
            .map(|filleted| (filleted.body, filleted.asked, filleted.taken)),
    };
    let (body, asked, taken) =
        laid.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    taken_whole(asked, taken, value, what);
    let height = |out: f64| match fill {
        Fill::Chamfer => chamfer_height(value, out),
        Fill::Fillet => fillet_height(value, out),
    };
    let laid_area = match fill {
        Fill::Chamfer => 0.0,
        Fill::Fillet => PI / 2.0 * value * along_the_ramp_rib(draft, 0.0, value),
    };
    sound_drawn_within(
        &body,
        on_the_ramp_volume(draft)
            + fill_matter(value, height, |out, up| along_the_ramp_rib(draft, out, up)),
        drawn_within(&part, ramp_round_area(draft) + value * value + laid_area),
        what,
    );
    let made = match fill {
        Fill::Chamfer => one_flat_face_more(&part, &body, what),
        Fill::Fillet => one_round_more(&part, &body, what),
    };
    numbered_after_the_steps_last(&part, &body, made, what);
    let mut sides = [RAMP, RIB_RIGHT, RIB_BACK, CONE_FRONT].map(|side| vec![side as u32]);
    sides.sort();
    assert_eq!(
        sides_met(&body, made),
        sides,
        "{what}: the fill meets the ramp and the rib's right side along its sides, and the round \
         and the rib's back at its ends, by one edge each"
    );
    one_edge_between(
        &body,
        [RAMP, CONE_FRONT],
        [&[RAMP, CONE_FRONT, RIB_LEFT], &[RAMP, CONE_FRONT, made]],
        &format!("{what}: the round's foot on the ramp, run on along the fill's footprint"),
    );
    no_corner_where_only(
        &body,
        &[RAMP, CONE_FRONT],
        &format!("{what}: the corner the foot of the rib's right side ended at"),
    );
    one_edge_between(
        &body,
        [CONE_FRONT, RIB_RIGHT],
        [
            &[CONE_FRONT, RIB_RIGHT, made],
            &[RIB_TOP, CONE_FRONT, RIB_RIGHT],
        ],
        &format!("{what}: the round's edge with the rib's right side, above the fill"),
    );
    for corner in body.triangles_of(CONE_FRONT).flatten() {
        assert!(
            off_the_ramp_round(draft, corner).abs() < PLACE,
            "{what}: the round end is drawn through {corner}, {} off it",
            off_the_ramp_round(draft, corner)
        );
    }
    let far = RIB_SIDE + value;
    let furthest = furthest_along_x(&body, CONE_FRONT);
    assert!(
        (furthest - far).abs() < PLACE,
        "{what}: the round reaches {furthest} along X, where it takes in the fill's section out \
         to {far}"
    );
    for share_out in [0.2, 0.5] {
        let out = value * share_out;
        for share_up in [0.25, 0.5, 0.75] {
            let place = on_the_ramp_section(out, height(out) * share_up);
            let on_the_round = place + up_the_ramp() * into_the_ramp_round(draft, place);
            let from = DVec3::new(on_the_round.x, -1.0, on_the_round.z);
            let hit = body.ray_hit(from, DVec3::Y).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} through the fill's section meets nothing")
            });
            let at = from + DVec3::Y * hit.distance;
            assert!(
                hit.face == CONE_FRONT
                    && off_the_ramp_round(draft, at).abs() <= MET
                    && at.distance(on_the_round) <= MET / up_the_ramp().y,
                "{what}: a ray from {from} through the fill's section meets face {} at {at}, \
                 where the fill ends on the round at {on_the_round}",
                hit.face
            );
        }
    }
    for corner in body.triangles_of(made).flatten() {
        assert!(
            corner.y >= AXIS.y || off_the_ramp_round(draft, corner) <= PLACE,
            "{what}: the fill is drawn through {corner}, {} outside the round: past it",
            off_the_ramp_round(draft, corner)
        );
    }
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_up_a_ramp_are_trimmed_flush_on_its_upright_round_end()
 {
    for (fill, what) in [
        (
            Fill::Chamfer,
            "the chamfer at the foot of a rib up a ramp, its end an upright round",
        ),
        (
            Fill::Fillet,
            "the fillet at the foot of a rib up a ramp, its end an upright round",
        ),
    ] {
        filled_up_the_ramp(0.0, fill, what);
    }
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_up_a_ramp_are_trimmed_flush_on_its_drafted_round_end()
 {
    for (fill, what) in [
        (
            Fill::Chamfer,
            "the chamfer at the foot of a rib up a ramp, its end a drafted round",
        ),
        (
            Fill::Fillet,
            "the fillet at the foot of a rib up a ramp, its end a drafted round",
        ),
    ] {
        filled_up_the_ramp(DRAFT, fill, what);
    }
}

/// How far the round end of the [`slot_drafted_at_its_end`] widens for every
/// unit up, a pocket's draft of some 5.7 degrees.
const SLOT_DRAFT: f64 = 0.1;

/// The radius of the [`slot_drafted_at_its_end`]'s round end at height `z`:
/// criterion 2's at the slot's floor, widening by [`SLOT_DRAFT`] above it.
fn slot_drafted_radius(z: f64) -> f64 {
    slot_radius() + SLOT_DRAFT * (z - SLOT_FLOOR_Z)
}

/// Criterion 2's slot with its round end drafted, as a pocket's is: the slot
/// cut straight up to its round's axis, and its end cut by a cone turned
/// whole about the upright line through [`slot_axis`], criterion 2's round
/// at the floor and widening by [`SLOT_DRAFT`] above it. Turned whole, the
/// cone's circle dips below the front's line at the floor between its two
/// crossings of it, at x = 27.8 and 28: the foot of the slot's front ends at
/// the first, and runs on past it into the cone, where the slot's air lies.
/// The matter lies outside the cone. The second cut's floor lies on the
/// first's, and the slot's floor answers to both their numbers.
///
/// A drafted end the foot runs out of, as #807 is to fill, cannot be cut so
/// with the slot's walls upright: turned part way round from the front's
/// end, the cone leaves a sliver of a flat face standing on that end at the
/// floor, and the foot ends where four faces meet. It wants the slot's walls
/// drafted with the round, as the [`slot_drafted_walls_and_end`] has them.
fn slot_drafted_at_its_end() -> Body {
    let straight = Contour::rectangle(
        DVec2::new(-5.0, SLOT_FRONT_Y),
        DVec2::new(slot_axis().x, SLOT_FRONT_Y + 4.0),
    );
    let slot = cut_from(&plate(SLOT_PLATE), straight, SLOT_FLOOR_Z, SLOT_PLATE);
    let corners = vec![
        DVec2::new(0.0, SLOT_PLATE + 1.0),
        DVec2::new(0.0, SLOT_FLOOR_Z),
        DVec2::new(slot_drafted_radius(SLOT_FLOOR_Z), SLOT_FLOOR_Z),
        DVec2::new(slot_drafted_radius(SLOT_PLATE + 1.0), SLOT_PLATE + 1.0),
    ];
    let sampled: &'static [DVec2] = Box::leak(corners.clone().into_boxed_slice());
    let outline = Profile {
        exact: Some((Contour::straight(corners), Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: slot_axis().extend(0.0),
        u: DVec3::X,
        v: DVec3::Z,
    };
    let upright = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    let end = slot
        .tool_turned(&outline, frame, &Turn::of(upright, TAU, 1e-4, &outline))
        .expect("the drafted end turns");
    slot.difference(&end).expect("the drafted end is cut")
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_slot_s_wall_running_into_its_drafted_round_end_are_declined_as_unsupported()
 {
    let part = slot_drafted_at_its_end();
    let what = "the slot drafted at its round end, before any step";
    let drafted: Vec<usize> = numbers(&part)
        .into_iter()
        .filter(|&face| {
            !part.is_flat(face)
                && part.triangles_of(face).flatten().all(|corner| {
                    (corner.truncate().distance(slot_axis()) - slot_drafted_radius(corner.z)).abs()
                        < PLACE
                })
        })
        .collect();
    assert_eq!(
        drafted.len(),
        1,
        "{what}: one face lies on the drafted cone, widening up from criterion 2's round"
    );
    let end = DVec3::new(SLOT_FRONT_END + 2.0 * SLOT_OFF, SLOT_FRONT_Y, SLOT_FLOOR_Z);
    assert!(
        part.triangles_of(drafted[0])
            .flatten()
            .any(|corner| corner.distance(end) < PLACE),
        "{what}: the cone meets the foot of the slot's front at {end}, where its circle first \
         crosses the front's line"
    );
    let past = end.truncate() + DVec2::X * 0.01;
    assert!(
        past.distance(slot_axis()) < slot_drafted_radius(SLOT_FLOOR_Z),
        "{what}: just past the foot's end, at {past}, the front's line is inside the cone"
    );
    let cone = vec![drafted[0] as u32];
    let feet: Vec<EdgeName> = part
        .edge_names()
        .into_iter()
        .filter(|name| {
            name.between.contains(&vec![SLOT_FRONT as u32])
                && name
                    .between
                    .iter()
                    .any(|side| side.contains(&(SLOT_FLOOR as u32)))
        })
        .collect();
    let [name] = &feet[..] else {
        panic!("{what}: one edge at the foot of the slot's front: {feet:?}");
    };
    assert!(
        name.ends.iter().any(|end| end.contains(&cone)),
        "{what}: the foot of the slot's front ends on the cone, face {}: {name:?}",
        drafted[0]
    );

    let declined = [
        ("the chamfer", part.chamfer(name, 1.0).err()),
        ("the fillet", part.fillet(name, 1.0).err()),
    ];

    for (step, declined) in declined {
        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "{step} of 1 at the foot of a slot's front running into its drafted round end"
        );
    }
}

/// The reach the tolerances of `body` are taken at: the larger of its
/// operands', which every later step of the part is decided at. Read off
/// what of it stands in front of a plane through the plate, which is cut a
/// drawing tolerance of that reach past the plane: since #876 the triangles
/// are cut at the body's own size, and no longer show it.
fn reach_carried(body: &Body) -> f64 {
    let through_the_plate = 1.0;
    let (low, _) = body
        .in_front(DVec3::Z, through_the_plate)
        .bounds()
        .expect("the part stands in front of a plane through its plate");
    (low.z - through_the_plate) / DRAWN
}

/// An outline drawn on the upright plane through the upright line through
/// `axis`, read in plan, that runs out from the line along `start`, out
/// along it and up along Z, turned `angle` about the line.
fn turned_about(part: &Body, axis: DVec2, start: DVec2, angle: f64, contour: Contour) -> Body {
    let sampled: &'static [DVec2] = Box::leak(contour.corners.clone().into_boxed_slice());
    let outline = Profile {
        exact: Some((contour, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = Frame {
        origin: axis.extend(0.0),
        u: start.normalize().extend(0.0),
        v: DVec3::Z,
    };
    let upright = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    part.tool_turned(&outline, frame, &Turn::of(upright, angle, 1e-4, &outline))
        .expect("the outline turns")
}

/// The plate with a rib on its top whose round end has draft, as the
/// [`rib_drafted_in_front`]'s has, its round `front` at the plate's top: a
/// rib 4 wide either side of the round's axis, from y = 4 to
/// [`DRAFTED_BACK`], raised 10, its front cut by a piece of a ring turned
/// about that axis whose inner wall is the cone drawing in by [`DRAFT`]
/// above the plate's top. The piece is turned no further round than the
/// rib needs, so that it stands inside the plate's box and the part carries
/// the box's reach alone. Its front answers to [`CONE_FRONT`].
fn rib_drafted_on(front: Round) -> Body {
    let cone = |z: f64| front.radius - DRAFT * (z - PLATE);
    let plate = plate(PLATE);
    let on_the_top = Frame {
        origin: DVec3::Z * PLATE,
        ..ground()
    };
    let rib = plate
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(front.axis.x - 2.0, 4.0),
                DVec2::new(front.axis.x + 2.0, DRAFTED_BACK),
            )),
            on_the_top,
            DVec3::Z * 10.0,
        )
        .expect("the rib raises");
    let outer = DVec2::new(2.0, front.axis.y - 4.0).length() + 0.4;
    let half = 1.4 * (2.0 / cone(15.0)).asin();
    let ring = turned_about(
        &rib,
        front.axis,
        DVec2::new(-half.sin(), -half.cos()),
        2.0 * half,
        Contour::straight(vec![
            DVec2::new(cone(15.0), 15.0),
            DVec2::new(cone(3.0), 3.0),
            DVec2::new(outer, 3.0),
            DVec2::new(outer, 15.0),
        ]),
    );
    let drafted = rib.difference(&ring).expect("the draft is cut");
    plate.union(&drafted).expect("the rib is joined")
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_drafted_round_end_stands_near_the_plate_s_side_leave_the_part_drawn_as_finely_as_its_own_box_asks()
 {
    let value = 3.0;
    for (rib, where_it_stands) in [
        (CRITERION_1, "where criterion 1 stands it"),
        (
            moved_toward_the_plate_s_side(),
            "moved toward the plate's side",
        ),
        (
            near_the_plate_s_side(),
            "beside the plate's side, the fill taking the top's whole width there",
        ),
        (
            gently_bulging(),
            "bulging gently in a round of 50, its axis well past the part's back",
        ),
    ] {
        let what = format!("the rib drafted {where_it_stands}");
        let (part, upright) = (rib_drafted_on(rib.front), rib.on());
        let off_its_cone = |point: DVec3| {
            point.truncate().distance(rib.front.axis)
                - (rib.front.radius - DRAFT * (point.z - PLATE))
        };
        assert!(
            part.has_face(CONE_FRONT)
                && !part.is_flat(CONE_FRONT)
                && part
                    .triangles_of(CONE_FRONT)
                    .flatten()
                    .all(|corner| off_its_cone(corner).abs() < PLACE),
            "{what}, before the step: its front, face {CONE_FRONT}, is the drafted round's cone"
        );
        let own = reach_of(&part);
        let carried = reach_carried(&part);
        assert!(
            (carried - own).abs() <= PLACE,
            "{what}, before the step: it carries a reach of {carried}, where its box reaches {own}"
        );
        let foot = edge_between(&part, TOP, RIB_RIGHT);
        let upright_foot = edge_between(&upright, TOP, RIB_RIGHT);

        let laid = [
            (
                "the chamfer",
                part.chamfer(&foot, value).map(|chamfered| chamfered.body),
                upright
                    .chamfer(&upright_foot, value)
                    .map(|chamfered| chamfered.body),
            ),
            (
                "the fillet",
                part.fillet(&foot, value).map(|filleted| filleted.body),
                upright
                    .fillet(&upright_foot, value)
                    .map(|filleted| filleted.body),
            ),
        ];

        for (step, drafted, square) in laid {
            let drafted =
                drafted.unwrap_or_else(|declined| panic!("{what}: {step} declined, {declined:?}"));
            let square = square.unwrap_or_else(|declined| {
                panic!("{what}: {step} up to the upright round declined, {declined:?}")
            });
            let (carried, hugging) = (reach_carried(&drafted), reach_carried(&square));
            assert!(
                carried <= hugging + PLACE,
                "{what}, {step}: the part carries a reach of {carried}, where its box reaches \
                 {own} and the same fill up to the upright round, trimmed by a solid standing no \
                 further out than the fill, carries {hugging} — every later step would be \
                 decided and drawn coarser than the part's size asks"
            );
        }
    }
}

/// The numbers the faces of the [`slot_drafted_walls_and_end`] answer to:
/// its floor, which its straight cut and its round end both lay, answering
/// to both their numbers; the back and front walls the straight cut leaves,
/// the second and fourth runs of its section; the cone its round end
/// leaves; and where its steps stopped numbering. The plate's left side,
/// which the slot runs out through, is [`PLATE_LEFT`].
const DRAFTED_SLOT_FLOOR: [u32; 2] = [8, 18];
const DRAFTED_SLOT_BACK: usize = 9;
const DRAFTED_SLOT_FRONT: usize = 11;
const DRAFTED_SLOT_END: usize = 19;
const DRAFTED_SLOT_STEPS_END: usize = 24;

/// How many steps a fill's matter across the [`slot_drafted_walls_and_end`]
/// is summed over by Simpson's rule, each way: the length it runs bends
/// sharply where its wall meets the cone at a few degrees.
const ACROSS_STEPS: usize = 1 << 10;

/// How far each wall of the [`slot_drafted_walls_and_end`] stands from the
/// slot's middle at height `z`: 2 at the floor, in the ratio to its round
/// end's radius the floor gives at every height, so that the slot's plan at
/// any height is its floor's, widened about the round's axis.
fn drafted_slot_half_width(z: f64) -> f64 {
    2.0 * slot_drafted_radius(z) / slot_radius()
}

/// How far the [`slot_drafted_walls_and_end`]'s walls lean out for every
/// unit up: a draft of some 5.7 degrees, a hair less than its round's.
fn drafted_slot_wall_lean() -> f64 {
    2.0 * SLOT_DRAFT / slot_radius()
}

/// Where the [`slot_drafted_walls_and_end`]'s round end stands at `y` and
/// height `z`: the x of its point there past its axis.
fn on_the_drafted_slot_s_end(y: f64, z: f64) -> f64 {
    slot_axis().x + (slot_drafted_radius(z).powi(2) - (y - slot_axis().y).powi(2)).sqrt()
}

/// How far a point stands off the [`slot_drafted_walls_and_end`]'s cone, out
/// from its axis.
fn off_the_drafted_slot_s_end(point: DVec3) -> f64 {
    point.truncate().distance(slot_axis()) - slot_drafted_radius(point.z)
}

/// Where the [`slot_drafted_walls_and_end`]'s front meets its round end at
/// height `z`: on the cone's ruling through the front's end at the floor,
/// the line every height's plan widens it along.
fn on_the_drafted_slot_s_ruling(z: f64) -> DVec3 {
    let end = DVec2::new(SLOT_FRONT_END, SLOT_FRONT_Y);
    (slot_axis() + (end - slot_axis()) * slot_drafted_radius(z) / slot_radius()).extend(z)
}

/// Criterion 2's slot with its walls and its round end drafted, as a moulded
/// pocket's are: the slot of [`slot_in_the_plate`] at its floor, and every
/// height above it that floor's plan widened about [`slot_axis`] as its
/// round end widens by [`SLOT_DRAFT`]. Its straight part is cut first: its
/// section across the slot, the floor and the two walls leaning out by
/// [`drafted_slot_wall_lean`], drawn on a plane square to the slot beyond
/// the plate's left side and raised along it, then cut short on the
/// upright plane through the round's axis and the front's end at the
/// floor, which every height's plan widens along. Its round end is the cone
/// turned half a turn about the round's axis from that plane. The front
/// meets the cone along the ruling through its end, turning in toward the
/// slot at some 2.9 degrees at every height, so that past the foot's end the
/// front's line runs out of the cone into the matter, where a fill there
/// stands.
fn slot_drafted_walls_and_end() -> Body {
    let (middle, floor, top) = (slot_axis().y, SLOT_FLOOR_Z, SLOT_PLATE + 1.0);
    let plate = plate(SLOT_PLATE);
    let across = Frame {
        origin: DVec3::X * -5.0,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let section = Contour::straight(vec![
        DVec2::new(middle - drafted_slot_half_width(floor), floor),
        DVec2::new(middle + drafted_slot_half_width(floor), floor),
        DVec2::new(middle + drafted_slot_half_width(top), top),
        DVec2::new(middle - drafted_slot_half_width(top), top),
    ]);
    let along = plate
        .tool_raised(&profile(section), across, DVec3::X * 35.0)
        .expect("the slot's straight part raises");
    let to_the_end = DVec2::new(SLOT_FRONT_END, SLOT_FRONT_Y) - slot_axis();
    let through_the_axis = Frame {
        origin: slot_axis().extend(0.0),
        u: to_the_end.normalize().extend(0.0),
        v: DVec3::Z,
    };
    let past = plate
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(-4.0, 3.0),
                DVec2::new(4.0, 12.0),
            )),
            through_the_axis,
            -through_the_axis.normal() * 3.0,
        )
        .expect("the block past the round's axis raises");
    let straight = along
        .difference(&past)
        .expect("the straight part is cut short");
    let round = turned_about(
        &plate,
        slot_axis(),
        to_the_end,
        PI,
        Contour::straight(vec![
            DVec2::new(0.0, floor),
            DVec2::new(slot_drafted_radius(floor), floor),
            DVec2::new(slot_drafted_radius(top), top),
            DVec2::new(0.0, top),
        ]),
    );
    plate
        .difference(&straight)
        .expect("the straight part is cut")
        .difference(&round)
        .expect("the round end is cut")
}

/// The matter the [`slot_drafted_walls_and_end`] holds: the plate's
/// 40 × 30 × 10, less the slot's — at each height between its floor and the
/// plate's top, the strip between its walls from the plate's left side to
/// the plane through the round's axis, which crosses it about its middle,
/// and the half disc beyond that plane.
fn drafted_slot_volume() -> f64 {
    let plan = |z: f64| {
        2.0 * drafted_slot_half_width(z) * slot_axis().x + PI * slot_drafted_radius(z).powi(2) / 2.0
    };
    40.0 * 30.0 * SLOT_PLATE - simpson(plan, [SLOT_FLOOR_Z, SLOT_PLATE], UP_STEPS)
}

/// The area of the [`slot_drafted_walls_and_end`]'s cone: half a turn of it,
/// along its slant from the floor to the plate's top.
fn drafted_slot_end_area() -> f64 {
    let mean = (slot_drafted_radius(SLOT_FLOOR_Z) + slot_drafted_radius(SLOT_PLATE)) / 2.0;
    PI * mean * (SLOT_PLATE - SLOT_FLOOR_Z) * (1.0 + SLOT_DRAFT * SLOT_DRAFT).sqrt()
}

/// What `at` sums to over the triangle `corners`, by Simpson's rule each way
/// over `steps` steps.
fn over_triangle(at: impl Fn(DVec2) -> f64, [a, b, c]: [DVec2; 3], steps: usize) -> f64 {
    let (ab, ac) = (b - a, c - a);
    let spread = ab.perp_dot(ac).abs();
    let along = |out: f64| {
        simpson(
            |share: f64| at(a + (ab * (1.0 - share) + ac * share) * out),
            [0.0, 1.0],
            steps,
        ) * out
            * spread
    };
    simpson(along, [0.0, 1.0], steps)
}

/// What `at` sums to over the sector of `radius` about `centre` between the
/// angles `turns`, by Simpson's rule each way over `steps` steps.
fn over_sector(
    at: impl Fn(DVec2) -> f64,
    (centre, radius): (DVec2, f64),
    turns: [f64; 2],
    steps: usize,
) -> f64 {
    let round = |turn: f64| {
        simpson(
            |out: f64| at(centre + DVec2::from_angle(turn) * out) * out,
            [0.0, radius],
            steps,
        )
    };
    simpson(round, turns, steps)
}

/// The section of a fill of `value` at the foot of the
/// [`slot_drafted_walls_and_end`]'s front, read across the slot along Y and
/// up Z: in the corner between the floor and the front leaning out over it,
/// a flat set back by the value along each, or a round of the value tangent
/// to both.
#[derive(Clone, Copy, Debug)]
struct DraftedSection {
    fill: Fill,
    value: f64,
}

impl DraftedSection {
    fn foot() -> DVec2 {
        DVec2::new(SLOT_FRONT_Y, SLOT_FLOOR_Z)
    }

    fn up_the_wall() -> DVec2 {
        DVec2::new(-drafted_slot_wall_lean(), 1.0).normalize()
    }

    /// The way the front faces, out of the matter into the slot.
    fn off_the_wall() -> DVec2 {
        DVec2::new(1.0, drafted_slot_wall_lean()).normalize()
    }

    /// The angle between the floor and the front, through the air.
    fn angle() -> f64 {
        DVec2::X.dot(Self::up_the_wall()).acos()
    }

    /// How far the fill reaches along the floor and up the front from the
    /// foot.
    fn set_back(self) -> f64 {
        match self.fill {
            Fill::Chamfer => self.value,
            Fill::Fillet => self.value / (Self::angle() / 2.0).tan(),
        }
    }

    fn on_the_floor(self) -> DVec2 {
        Self::foot() + DVec2::X * self.set_back()
    }

    fn on_the_wall(self) -> DVec2 {
        Self::foot() + Self::up_the_wall() * self.set_back()
    }

    fn centre(self) -> DVec2 {
        self.on_the_floor() + DVec2::Y * self.value
    }

    /// How far a point stands off the fill's surface, out of the fill.
    fn off(self, at: DVec2) -> f64 {
        match self.fill {
            Fill::Chamfer => {
                let (from, to) = (self.on_the_floor(), self.on_the_wall());
                let out = (to - from).perp().normalize();
                let out = if out.dot(Self::foot() - from) > 0.0 {
                    -out
                } else {
                    out
                };
                (at - from).dot(out)
            }
            Fill::Fillet => self.value - at.distance(self.centre()),
        }
    }

    /// How deep a point lies inside the fill's section: over the floor, off
    /// the front, on the foot's side of the line between the fill's sides,
    /// and under its surface, the least of the four.
    fn depth(self, at: DVec2) -> f64 {
        let (floor, wall) = (self.on_the_floor(), self.on_the_wall());
        let back = (wall - floor).perp().normalize();
        let back = if back.dot(Self::foot() - floor) < 0.0 {
            -back
        } else {
            back
        };
        (at.y - SLOT_FLOOR_Z)
            .min((at - Self::foot()).dot(Self::off_the_wall()))
            .min((at - floor).dot(back))
            .min(-self.off(at))
    }

    /// The matter it holds: over every place of it, the length it runs from
    /// the plate's left side to the round end.
    fn matter(self) -> f64 {
        let long = |at: DVec2| on_the_drafted_slot_s_end(at.x, at.y);
        let (foot, floor, wall) = (Self::foot(), self.on_the_floor(), self.on_the_wall());
        match self.fill {
            Fill::Chamfer => over_triangle(long, [foot, floor, wall], ACROSS_STEPS),
            Fill::Fillet => {
                let centre = self.centre();
                let down = (floor - centre).to_angle();
                over_triangle(long, [foot, floor, centre], ACROSS_STEPS)
                    + over_triangle(long, [foot, centre, wall], ACROSS_STEPS)
                    - over_sector(
                        long,
                        (centre, self.value),
                        [down - (PI - Self::angle()), down],
                        ACROSS_STEPS,
                    )
            }
        }
    }
}

/// Holds that the faces whose sides are `between` meet along one edge
/// alone, between the corners where the faces `ends` name, besides its own
/// two, meet it.
fn one_edge_of(body: &Body, between: [Vec<u32>; 2], ends: [Vec<Vec<u32>>; 2], what: &str) {
    let meeting: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| between.iter().all(|side| name.between.contains(side)))
        .collect();
    assert_eq!(
        meeting.len(),
        1,
        "{what}: faces {between:?} meet along one edge: {meeting:?}"
    );
    let sorted = |mut sides: Vec<Vec<u32>>| {
        sides.sort();
        sides
    };
    let mut found: Vec<Vec<Vec<u32>>> = meeting[0].ends.iter().cloned().map(sorted).collect();
    found.sort();
    let mut expected: Vec<Vec<Vec<u32>>> = ends.into_iter().map(sorted).collect();
    expected.sort();
    assert_eq!(
        found, expected,
        "{what}: the edge between faces {between:?} runs between the corners where these faces \
         meet it"
    );
}

/// Holds that no corner of the body stands where the faces whose sides are
/// `sides` alone meet.
fn no_corner_of(body: &Body, mut sides: Vec<Vec<u32>>, what: &str) {
    sides.sort();
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
        "{what}: a corner is left where faces {sides:?} alone meet: {left:?}"
    );
}

/// Holds that the face numbered [`DRAFTED_SLOT_END`] is the drafted slot's
/// cone: curved, and every corner of its triangles on the cone.
fn drawn_on_the_drafted_slot_s_cone(body: &Body, what: &str) {
    assert!(
        body.has_face(DRAFTED_SLOT_END) && !body.is_flat(DRAFTED_SLOT_END),
        "{what}: face {DRAFTED_SLOT_END} is the round end"
    );
    let corners: Vec<DVec3> = body.triangles_of(DRAFTED_SLOT_END).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round end is drawn");
    for corner in corners {
        assert!(
            off_the_drafted_slot_s_end(corner).abs() < PLACE,
            "{what}: the round end is drawn through {corner}, {} off its cone",
            off_the_drafted_slot_s_end(corner)
        );
    }
}

fn drafted_slot_side(face: usize) -> Vec<u32> {
    vec![face as u32]
}

/// Holds that the [`slot_drafted_walls_and_end`] is what the steps are
/// given: its faces numbered up to [`DRAFTED_SLOT_STEPS_END`]; its floor,
/// walls and the plate's left side flat; its round end on its cone; the
/// front leaning out from its foot to the plate's top and meeting the cone
/// along the ruling through its end; the foot of the front one edge, from
/// the plate's left side to the cone; and its matter the plate's less the
/// slot's.
fn the_slot_is_drafted_walls_and_end(part: &Body) {
    let what = "the slot drafted at its walls and its round end, before any step";
    assert_eq!(
        part.faces_end(),
        DRAFTED_SLOT_STEPS_END,
        "{what}: its steps numbered their faces up to {DRAFTED_SLOT_STEPS_END}"
    );
    for flat in [
        PLATE_LEFT,
        DRAFTED_SLOT_FLOOR[0] as usize,
        DRAFTED_SLOT_FLOOR[1] as usize,
        DRAFTED_SLOT_BACK,
        DRAFTED_SLOT_FRONT,
    ] {
        assert!(
            part.has_face(flat) && part.is_flat(flat),
            "{what}: face {flat} is flat"
        );
    }
    drawn_on_the_drafted_slot_s_cone(part, what);
    let lip = SLOT_FRONT_Y - drafted_slot_wall_lean() * (SLOT_PLATE - SLOT_FLOOR_Z);
    corners_are(
        part,
        DRAFTED_SLOT_FRONT,
        &[
            DVec3::new(0.0, SLOT_FRONT_Y, SLOT_FLOOR_Z),
            on_the_drafted_slot_s_ruling(SLOT_FLOOR_Z),
            on_the_drafted_slot_s_ruling(SLOT_PLATE),
            DVec3::new(0.0, lip, SLOT_PLATE),
        ],
        &format!("{what}: the front, leaning out to {lip} at the plate's top"),
    );
    one_edge_of(
        part,
        [
            DRAFTED_SLOT_FLOOR.to_vec(),
            drafted_slot_side(DRAFTED_SLOT_FRONT),
        ],
        [
            vec![drafted_slot_side(PLATE_LEFT)],
            vec![drafted_slot_side(DRAFTED_SLOT_END)],
        ],
        &format!("{what}: the foot of the front"),
    );
    sound_drawn_within(
        part,
        drafted_slot_volume(),
        drawn_within(part, drafted_slot_end_area()),
        what,
    );
}

/// Holds what `fill` of 1 at the foot of the [`slot_drafted_walls_and_end`]'s
/// front leaves: the value taken whole; a closed, sound body of the part's
/// matter and the fill's, with one face more, numbered after the steps'
/// last, flat or curved as its step lays it; the fill meeting the floor and
/// the front along its sides, and the plate's left side and the cone at its
/// ends, by one edge each; the cone one face still, drawn on itself alone,
/// with corners where the fill's sides reach it and none inside the fill's
/// section; the cone's foot and its edge with the front run short to the
/// fill, and no corner left where the foot of the front ended; every ray
/// down onto the fill a little short of the cone meeting it on its surface;
/// and nothing of the fill standing past the cone.
fn filled_flush_on_the_drafted_slot_s_end(fill: Fill, what: &str) {
    let value = 1.0;
    let part = slot_drafted_walls_and_end();
    the_slot_is_drafted_walls_and_end(&part);
    let foot = part
        .edge_names()
        .into_iter()
        .find(|name| {
            name.between.contains(&DRAFTED_SLOT_FLOOR.to_vec())
                && name
                    .between
                    .contains(&drafted_slot_side(DRAFTED_SLOT_FRONT))
        })
        .expect("the foot of the drafted slot's front is an edge");

    let laid = match fill {
        Fill::Chamfer => part
            .chamfer(&foot, value)
            .map(|chamfered| (chamfered.body, chamfered.asked, chamfered.taken)),
        Fill::Fillet => part
            .fillet(&foot, value)
            .map(|filleted| (filleted.body, filleted.asked, filleted.taken)),
    };
    let (body, asked, taken) =
        laid.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    taken_whole(asked, taken, value, what);
    let section = DraftedSection { fill, value };
    let laid_area = match fill {
        Fill::Chamfer => value * value,
        Fill::Fillet => value * value + (PI - DraftedSection::angle()) * value * SLOT_FRONT_END,
    };
    sound_drawn_within(
        &body,
        drafted_slot_volume() + section.matter(),
        drawn_within(&part, drafted_slot_end_area() + laid_area),
        what,
    );
    let made = match fill {
        Fill::Chamfer => one_flat_face_more(&part, &body, what),
        Fill::Fillet => one_round_more(&part, &body, what),
    };
    numbered_after_the_steps_last(&part, &body, made, what);
    let (floor, front, end, fill_side) = (
        DRAFTED_SLOT_FLOOR.to_vec(),
        drafted_slot_side(DRAFTED_SLOT_FRONT),
        drafted_slot_side(DRAFTED_SLOT_END),
        drafted_slot_side(made),
    );
    let mut sides = vec![
        drafted_slot_side(PLATE_LEFT),
        floor.clone(),
        front.clone(),
        end.clone(),
    ];
    sides.sort();
    assert_eq!(
        sides_met(&body, made),
        sides,
        "{what}: the fill meets the floor and the front along its sides, and the plate's left \
         side and the cone at its ends, by one edge each"
    );

    drawn_on_the_drafted_slot_s_cone(&body, what);
    let cone: Vec<DVec3> = body.triangles_of(DRAFTED_SLOT_END).flatten().collect();
    let [on_the_floor, on_the_wall] = [section.on_the_floor(), section.on_the_wall()]
        .map(|at| DVec3::new(on_the_drafted_slot_s_end(at.x, at.y), at.x, at.y));
    for (corner, side) in [
        (on_the_floor, "on the floor"),
        (on_the_wall, "up the front"),
    ] {
        assert!(
            cone.iter().any(|drawn| drawn.distance(corner) < PLACE),
            "{what}: the cone has no corner at {corner}, where the fill's side {side} reaches it"
        );
    }
    for corner in &cone {
        let depth = section.depth(DVec2::new(corner.y, corner.z));
        assert!(
            depth <= PLACE,
            "{what}: the cone is drawn through {corner}, {depth} inside the fill's section: it \
             still stands where the fill covers it"
        );
    }
    one_edge_of(
        &body,
        [floor.clone(), end.clone()],
        [
            vec![fill_side.clone()],
            vec![drafted_slot_side(DRAFTED_SLOT_BACK)],
        ],
        &format!("{what}: the cone's foot, run short to the fill"),
    );
    one_edge_of(
        &body,
        [front.clone(), end.clone()],
        [vec![fill_side], vec![drafted_slot_side(TOP)]],
        &format!("{what}: the cone's edge with the front, run short to the fill"),
    );
    for faces in [
        vec![floor.clone(), front.clone(), end.clone()],
        vec![floor, end.clone()],
        vec![front, end],
    ] {
        no_corner_of(
            &body,
            faces,
            &format!("{what}: the corner the foot of the front ended at"),
        );
    }
    for share in [0.2, 0.5] {
        let y = SLOT_FRONT_Y + section.set_back() * share;
        let from = DVec3::new(
            on_the_drafted_slot_s_end(y, SLOT_FLOOR_Z) - SHORT_OF_THE_END,
            y,
            SLOT_PLATE - 0.5,
        );
        let hit = body.ray_hit(from, -DVec3::Z).unwrap_or_else(|| {
            panic!("{what}: a ray down from {from}, short of the cone, meets nothing")
        });
        let at = from - DVec3::Z * hit.distance;
        let off = section.off(DVec2::new(at.y, at.z));
        assert!(
            hit.face == made && off.abs() <= MET,
            "{what}: a ray down from {from}, short of the cone, meets face {} at {at}, {off} off \
             the fill's surface",
            hit.face
        );
    }
    for corner in body.triangles_of(made).flatten() {
        let off = off_the_drafted_slot_s_end(corner);
        assert!(
            corner.x <= slot_axis().x || off <= PLACE,
            "{what}: the fill is drawn through {corner}, {off} outside the cone: past the round \
             end"
        );
    }
}

#[test]
fn a_chamfer_at_the_foot_of_a_slot_s_drafted_wall_is_trimmed_flush_on_the_drafted_round_end_it_meets_at_a_few_degrees()
 {
    filled_flush_on_the_drafted_slot_s_end(
        Fill::Chamfer,
        "the chamfer at the foot of the drafted slot's front",
    );
}

/// The fillet on the edge the chamfer above fills: its prism is trimmed on
/// the cone, and joining the trimmed fill to the body declines. #907 lifts
/// it, and turns this test round into the chamfer's, holding
/// [`filled_flush_on_the_drafted_slot_s_end`] with [`Fill::Fillet`].
#[test]
fn a_fillet_at_the_foot_of_a_slot_s_drafted_wall_meeting_its_drafted_round_end_at_a_few_degrees_is_declined_as_unsupported()
 {
    let part = slot_drafted_walls_and_end();
    the_slot_is_drafted_walls_and_end(&part);
    let foot = part
        .edge_names()
        .into_iter()
        .find(|name| {
            name.between.contains(&DRAFTED_SLOT_FLOOR.to_vec())
                && name
                    .between
                    .contains(&drafted_slot_side(DRAFTED_SLOT_FRONT))
        })
        .expect("the foot of the drafted slot's front is an edge");

    let declined = part.fillet(&foot, 1.0).err();

    assert_eq!(
        declined,
        Some(Declined::Unsupported),
        "the fillet of 1 at the foot of the drafted slot's front, meeting its drafted round end \
         at a few degrees"
    );
    the_slot_is_drafted_walls_and_end(&part);
}

/// How fast the [`rib_up_a_steep_ramp`]'s ramp rises along Y, and how far its
/// round end draws in for every unit up: some 27 and 17 degrees, steep
/// enough that each long edge of a fill at the foot of the rib's right side
/// rises along the cone's axis by a share of its run the cone's radius
/// follows, and where it leaves the cone is read from both.
const STEEP_RISE: f64 = 0.5;
const STEEP_DRAFT: f64 = 0.3;

/// Where the steps that built the [`rib_up_a_steep_ramp`] stopped numbering
/// faces: past the [`on_the_ramp`] part's, by the two sides of the piece of
/// a ring its front is cut by.
const STEEP_STEPS_END: usize = CONE_STEPS_END + 2;

/// The height of the [`rib_up_a_steep_ramp`]'s ramp at `y`.
fn steep_ramp_at(y: f64) -> f64 {
    2.0 + STEEP_RISE * y
}

/// Where the top of the [`rib_up_a_steep_ramp`] stands: 8 over the ramp at
/// its back.
fn steep_rib_top() -> f64 {
    steep_ramp_at(RAMP_RIB_BACK) + 8.0
}

/// The radius of the [`rib_up_a_steep_ramp`]'s round end at height `z`:
/// criterion 1's round where the ramp meets it at the rib's front corners,
/// drawn in by [`STEEP_DRAFT`] for every unit up.
fn steep_round_radius(z: f64) -> f64 {
    RADIUS - STEEP_DRAFT * (z - steep_ramp_at(RIB_ENDS[0]))
}

/// How far a point stands off the [`rib_up_a_steep_ramp`]'s cone, out from
/// its axis.
fn off_the_steep_round(point: DVec3) -> f64 {
    off_the_axis(point) - steep_round_radius(point.z)
}

/// A plate 40 long whose top rises along Y as a ramp of [`STEEP_RISE`],
/// raised along X from its section and numbered as the [`on_the_ramp`]
/// plate is, and on it a rib 4 wide from x = 18 to 22, from below the ramp
/// up to [`steep_rib_top`], its back upright at y = 19, its front cut by a
/// piece of a ring turned about the upright line through [`AXIS`], whose
/// inner wall is the cone of [`steep_round_radius`] — turned no further
/// round than the rib's front needs, so that the steep draft leaves its
/// back alone. Its faces number as the [`on_the_ramp`] part's do.
fn rib_up_a_steep_ramp() -> Body {
    let plate = raised_on(
        Frame {
            origin: DVec3::ZERO,
            u: DVec3::Y,
            v: DVec3::Z,
        },
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, steep_ramp_at(30.0)),
            DVec2::new(0.0, steep_ramp_at(0.0)),
        ]),
        40.0,
    );
    let below_the_ramp = Frame {
        origin: DVec3::Z,
        ..ground()
    };
    let rib = plate
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(18.0, 4.0),
                DVec2::new(RIB_SIDE, RAMP_RIB_BACK),
            )),
            below_the_ramp,
            DVec3::Z * (steep_rib_top() - 1.0),
        )
        .expect("the rib raises");
    let (low, high) = (0.5, steep_rib_top() + 1.0);
    let outer = DVec2::new(2.0, AXIS.y - 4.0)
        .length()
        .max(steep_round_radius(low))
        + 0.5;
    let half = 1.4 * (2.0 / steep_round_radius(high)).asin();
    let ring = turned_about(
        &rib,
        AXIS,
        DVec2::new(-half.sin(), -half.cos()),
        2.0 * half,
        Contour::straight(vec![
            DVec2::new(steep_round_radius(high), high),
            DVec2::new(steep_round_radius(low), low),
            DVec2::new(outer, low),
            DVec2::new(outer, high),
        ]),
    );
    let rounded = rib.difference(&ring).expect("the round end is cut");
    plate.union(&rounded).expect("the rib is joined")
}

/// The way along the foot of the [`rib_up_a_steep_ramp`]'s right side, up
/// the ramp, and the way square to the ramp, out of it.
fn up_the_steep_ramp() -> DVec3 {
    DVec3::new(0.0, 1.0, STEEP_RISE).normalize()
}

fn off_the_steep_ramp() -> DVec3 {
    DVec3::new(0.0, -STEEP_RISE, 1.0).normalize()
}

/// The point of a fill's section `out` along the ramp from the rib's right
/// side and `up` off the ramp, on the line through the foot's point at
/// y = 0.
fn on_the_steep_ramp_section(out: f64, up: f64) -> DVec3 {
    DVec3::new(RIB_SIDE + out, 0.0, steep_ramp_at(0.0)) + off_the_steep_ramp() * up
}

/// Where the line through `from` running up the ramp meets the
/// [`rib_up_a_steep_ramp`]'s cone in front: how far along that line, the
/// nearer root of the quadratic its distance from the axis squared less the
/// cone's radius squared makes along it.
fn into_the_steep_round(from: DVec3) -> f64 {
    let way = up_the_steep_ramp();
    let off = (from - AXIS.extend(0.0)).truncate();
    let across = way.truncate();
    let (radius, shrink) = (steep_round_radius(from.z), STEEP_DRAFT * way.z);
    let square = across.length_squared() - shrink * shrink;
    let half = off.dot(across) + radius * shrink;
    let rest = off.length_squared() - radius * radius;
    (-half - (half * half - square * rest).sqrt()) / square
}

/// How long a fill at the foot of the [`rib_up_a_steep_ramp`]'s right side
/// runs, `out` from that side and `up` off the ramp, measured up the ramp:
/// from the cone to the rib's upright back.
fn along_the_steep_ramp_rib(out: f64, up: f64) -> f64 {
    let from = on_the_steep_ramp_section(out, up);
    (RAMP_RIB_BACK - from.y) / up_the_steep_ramp().y - into_the_steep_round(from)
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_up_a_steep_ramp_are_trimmed_flush_on_its_steeply_drafted_round_end()
 {
    let value = 3.0;
    let part = rib_up_a_steep_ramp();
    let before = "the rib up a steep ramp, before the step";
    assert_eq!(
        part.faces_end(),
        STEEP_STEPS_END,
        "{before}: its steps numbered their faces up to {STEEP_STEPS_END}"
    );
    assert!(
        part.has_face(CONE_FRONT)
            && !part.is_flat(CONE_FRONT)
            && part
                .triangles_of(CONE_FRONT)
                .flatten()
                .all(|corner| off_the_steep_round(corner).abs() < PLACE),
        "{before}: its front, face {CONE_FRONT}, is the steeply drafted cone"
    );
    let foot = edge_between(&part, RAMP, RIB_RIGHT);

    for fill in [Fill::Chamfer, Fill::Fillet] {
        let what =
            format!("the {fill:?} at the foot of a rib up a steep ramp, its end steeply drafted");
        let laid = match fill {
            Fill::Chamfer => part
                .chamfer(&foot, value)
                .map(|chamfered| (chamfered.body, chamfered.asked, chamfered.taken)),
            Fill::Fillet => part
                .fillet(&foot, value)
                .map(|filleted| (filleted.body, filleted.asked, filleted.taken)),
        };
        let (body, asked, taken) =
            laid.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(asked, taken, value, &what);
        let height = |out: f64| match fill {
            Fill::Chamfer => chamfer_height(value, out),
            Fill::Fillet => fillet_height(value, out),
        };
        let laid_area = match fill {
            Fill::Chamfer => 0.0,
            Fill::Fillet => PI / 2.0 * value * along_the_steep_ramp_rib(0.0, value),
        };
        let cone_area = 4.3
            * (steep_rib_top() - steep_ramp_at(RIB_ENDS[0]))
            * (1.0 + STEEP_DRAFT * STEEP_DRAFT).sqrt();
        sound_drawn_within(
            &body,
            part.volume() + fill_matter(value, height, along_the_steep_ramp_rib),
            drawn_within(&part, cone_area + value * value + laid_area),
            &what,
        );
        let made = match fill {
            Fill::Chamfer => one_flat_face_more(&part, &body, &what),
            Fill::Fillet => one_round_more(&part, &body, &what),
        };
        numbered_after_the_steps_last(&part, &body, made, &what);
        let mut sides = [RAMP, RIB_RIGHT, RIB_BACK, CONE_FRONT].map(|side| vec![side as u32]);
        sides.sort();
        assert_eq!(
            sides_met(&body, made),
            sides,
            "{what}: the fill meets the ramp and the rib's right side along its sides, and the \
             cone and the rib's back at its ends, by one edge each"
        );
        for corner in body.triangles_of(CONE_FRONT).flatten() {
            assert!(
                off_the_steep_round(corner).abs() < PLACE,
                "{what}: the cone is drawn through {corner}, {} off it",
                off_the_steep_round(corner)
            );
        }
        for share_out in [0.2, 0.5] {
            let out = value * share_out;
            for share_up in [0.25, 0.5, 0.75] {
                let place = on_the_steep_ramp_section(out, height(out) * share_up);
                let on_the_round = place + up_the_steep_ramp() * into_the_steep_round(place);
                let from = DVec3::new(on_the_round.x, -1.0, on_the_round.z);
                let hit = body.ray_hit(from, DVec3::Y).unwrap_or_else(|| {
                    panic!("{what}: a ray from {from} through the fill's section meets nothing")
                });
                let at = from + DVec3::Y * hit.distance;
                assert!(
                    hit.face == CONE_FRONT
                        && off_the_steep_round(at).abs() <= MET
                        && at.distance(on_the_round) <= MET / up_the_steep_ramp().y,
                    "{what}: a ray from {from} through the fill's section meets face {} at {at}, \
                     where the fill ends on the cone at {on_the_round}",
                    hit.face
                );
            }
        }
        for corner in body.triangles_of(made).flatten() {
            assert!(
                corner.y >= AXIS.y || off_the_steep_round(corner) <= PLACE,
                "{what}: the fill is drawn through {corner}, {} outside the cone: past it",
                off_the_steep_round(corner)
            );
        }
    }
}

/// The number the front of the [`wall_into_a_pillar`]'s wall answers to:
/// the first run of its outline, past the plate's six and the wall's floor
/// and top.
const WALL_FRONT: usize = 8;

/// The plate of [`plate`], 4 thick, with a wall on its top from x = 5 to its
/// end at 25, from y = 12 to 14, 8 high.
fn wall_on_the_plate() -> Body {
    let plate = plate(PLATE);
    let on_the_top = Frame {
        origin: DVec3::Z * PLATE,
        ..ground()
    };
    let wall = plate
        .tool_raised(
            &profile(Contour::rectangle(
                DVec2::new(5.0, 12.0),
                DVec2::new(25.0, 14.0),
            )),
            on_the_top,
            DVec3::Z * 8.0,
        )
        .expect("the wall raises");
    plate.union(&wall).expect("the wall is joined")
}

/// A pillar's section: a disc of 3 about `centre`, in two half rounds.
fn disc_about(centre: DVec2) -> Contour {
    Contour {
        corners: vec![centre + DVec2::X * 3.0, centre - DVec2::X * 3.0],
        runs: vec![
            Run::Round {
                center: centre,
                turn: PI,
            },
            Run::Round {
                center: centre,
                turn: PI,
            },
        ],
    }
}

/// The [`wall_on_the_plate`] and a round pillar of 3 about the upright line
/// through the wall's end, (25, 13), 10 high: the foot of the wall's front
/// runs into the pillar, whose matter lies inside its cylinder.
fn wall_into_a_pillar() -> Body {
    let walled = wall_on_the_plate();
    let on_the_top = Frame {
        origin: DVec3::Z * PLATE,
        ..ground()
    };
    let pillar = walled
        .tool_raised(
            &profile(disc_about(DVec2::new(25.0, 13.0))),
            on_the_top,
            DVec3::Z * 10.0,
        )
        .expect("the pillar raises");
    walled.union(&pillar).expect("the pillar is joined")
}

/// The matter the [`wall_into_a_pillar`] holds: the plate's 40 × 30 × 4, the
/// pillar's disc raised 10, and the wall's 20 × 2 raised 8 less what of it
/// the pillar holds, the segment of the disc between y = 12 and 14.
fn wall_into_a_pillar_volume() -> f64 {
    let in_the_pillar = 8.0f64.sqrt() + 9.0 * (1.0f64 / 3.0).asin();
    4800.0 + PI * 9.0 * 10.0 + (40.0 - in_the_pillar) * 8.0
}

/// The numbers the [`wall_into_a_pillar`]'s faces past its wall's front
/// answer to: the wall's top and its back and left end; the pillar's round,
/// one face answering to both halves of its outline; and where the steps
/// that built it stopped numbering, past the pillar's floor, top and two
/// halves.
const WALL_TOP: usize = 7;
const WALL_BACK: usize = 10;
const WALL_LEFT: usize = 11;
const PILLAR_ROUND: usize = 15;
const PILLAR_SIDE: [u32; 2] = [14, 15];
const PILLAR_STEPS_END: usize = 16;

/// The upright line the [`wall_into_a_pillar`]'s pillar stands about, read
/// in plan, and its radius.
const PILLAR: (DVec2, f64) = (DVec2::new(25.0, 13.0), 3.0);

/// Where the [`wall_into_a_pillar`]'s pillar stands at `y`, read in plan:
/// the x of its point there on the wall's side of its axis.
fn on_the_pillar(y: f64) -> f64 {
    let (axis, radius) = PILLAR;
    axis.x - (radius * radius - (y - axis.y).powi(2)).sqrt()
}

/// How far a point stands from the pillar's axis, read in plan.
fn off_the_pillar_s_axis(point: DVec3) -> f64 {
    point.truncate().distance(PILLAR.0)
}

/// How long a fill at the foot of the wall's front runs, `out` from it: from
/// the wall's left end to the pillar.
fn along_the_wall(out: f64) -> f64 {
    on_the_pillar(12.0 - out) - 5.0
}

/// The matter a chamfer of `value` at the foot of the wall's front holds,
/// in closed form, the pillar leaning `lean` toward the wall's run: over its
/// section, `value − out` high, the length [`along_the_wall_under`] gives,
/// 20 less the root of 9 − t² at t = 1 + out from the pillar's axis over the
/// cosine of the lean, and its tangent times the height `up`. With
/// a = 1 + `value`, ∫₁ᵃ (a − t) √(9 − t²) dt is a [F] − [G], F the area
/// under the circle and G = −(9 − t²)^{3/2} / 3; `up` sums to `value`³ / 6.
fn chamfer_into_the_pillar(value: f64, lean: f64) -> f64 {
    let under = |t: f64| (t * (9.0 - t * t).sqrt() + 9.0 * (t / 3.0).asin()) / 2.0;
    let moment = |t: f64| -(9.0 - t * t).powf(1.5) / 3.0;
    let a = 1.0 + value;
    let root = a * (under(a) - under(1.0)) - (moment(a) - moment(1.0));
    10.0 * value * value - root / lean.cos() + lean.tan() * value.powi(3) / 6.0
}

fn side(face: usize) -> Vec<u32> {
    vec![face as u32]
}

/// Holds that the [`wall_into_a_pillar`] is what the steps are given: its
/// faces numbered up to [`PILLAR_STEPS_END`]; the pillar's front half on its
/// cylinder; the foot of the wall's front one edge, from the wall's left end
/// to the pillar; and its matter the plate's, the pillar's and the wall's.
fn the_wall_runs_into_a_pillar(part: &Body) {
    let what = "the wall running into a round pillar, before any step";
    assert_eq!(
        part.faces_end(),
        PILLAR_STEPS_END,
        "{what}: its steps numbered their faces up to {PILLAR_STEPS_END}"
    );
    drawn_on_the_cylinder(part, PILLAR_ROUND, PILLAR, what);
    one_edge_of(
        part,
        [side(TOP), side(WALL_FRONT)],
        [vec![side(WALL_LEFT)], vec![PILLAR_SIDE.to_vec()]],
        &format!("{what}: the foot of the wall's front"),
    );
    sound_drawn_within(
        part,
        wall_into_a_pillar_volume(),
        drawn_within(part, 2.0 * PI * 3.0 * 10.0),
        what,
    );
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_wall_running_into_a_round_pillar_are_trimmed_flush_on_the_pillar()
 {
    let value = 1.0;
    let part = wall_into_a_pillar();
    the_wall_runs_into_a_pillar(&part);
    let foot = edge_between(&part, TOP, WALL_FRONT);

    for fill in [Fill::Chamfer, Fill::Fillet] {
        let what = format!("the {fill:?} at the foot of a wall running into a round pillar");
        let laid = match fill {
            Fill::Chamfer => part
                .chamfer(&foot, value)
                .map(|chamfered| (chamfered.body, chamfered.asked, chamfered.taken)),
            Fill::Fillet => part
                .fillet(&foot, value)
                .map(|filleted| (filleted.body, filleted.asked, filleted.taken)),
        };
        let (body, asked, taken) =
            laid.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(asked, taken, value, &what);
        let height = |out: f64| match fill {
            Fill::Chamfer => chamfer_height(value, out),
            Fill::Fillet => fillet_height(value, out),
        };
        let (matter, laid_area) = match fill {
            Fill::Chamfer => (chamfer_into_the_pillar(value, 0.0), 0.0),
            Fill::Fillet => (
                fill_matter(value, height, |out, _| along_the_wall(out)),
                PI / 2.0 * value * along_the_wall(0.0),
            ),
        };
        sound_drawn_within(
            &body,
            wall_into_a_pillar_volume() + matter,
            drawn_within(&part, 2.0 * PI * 3.0 * 10.0 + value * value + laid_area),
            &what,
        );
        let made = match fill {
            Fill::Chamfer => one_flat_face_more(&part, &body, &what),
            Fill::Fillet => one_round_more(&part, &body, &what),
        };
        numbered_after_the_steps_last(&part, &body, made, &what);

        let mut sides = vec![
            side(TOP),
            side(WALL_FRONT),
            side(WALL_LEFT),
            PILLAR_SIDE.to_vec(),
        ];
        sides.sort();
        assert_eq!(
            sides_met(&body, made),
            sides,
            "{what}: the fill meets the plate's top and the wall's front along its sides, and \
             the wall's left end and the pillar at its ends, by one edge each"
        );
        let (front, up) = (12.0 - value, PLATE + value);
        let (near, far) = (on_the_pillar(12.0), on_the_pillar(front));
        no_corner_of(
            &body,
            vec![side(TOP), PILLAR_SIDE.to_vec()],
            &format!("{what}: the pillar's foot, where the fill's side crosses its line"),
        );
        one_edge_of(
            &body,
            [side(TOP), PILLAR_SIDE.to_vec()],
            [vec![side(WALL_BACK)], vec![side(made)]],
            &format!("{what}: the pillar's foot, run short to the fill"),
        );
        one_edge_of(
            &body,
            [side(WALL_FRONT), PILLAR_SIDE.to_vec()],
            [vec![side(made)], vec![side(WALL_TOP)]],
            &format!("{what}: the pillar's edge with the wall's front, run short to the fill"),
        );
        no_corner_of(
            &body,
            vec![side(TOP), side(WALL_FRONT), PILLAR_SIDE.to_vec()],
            &format!("{what}: the corner the foot of the wall's front ended at"),
        );
        corners_are(
            &body,
            WALL_FRONT,
            &[
                DVec3::new(5.0, 12.0, up),
                DVec3::new(near, 12.0, up),
                DVec3::new(near, 12.0, PLATE + 8.0),
                DVec3::new(5.0, 12.0, PLATE + 8.0),
            ],
            &format!("{what}: what is left of the wall's front"),
        );
        drawn_on_the_cylinder(&body, PILLAR_ROUND, PILLAR, &what);
        let round: Vec<DVec3> = body.triangles_of(PILLAR_ROUND).flatten().collect();
        for corner in &round {
            let off_the_wall = 12.0 - corner.y;
            let under = if corner.x < PILLAR.0.x && (0.0..value).contains(&off_the_wall) {
                PLATE + height(off_the_wall)
            } else {
                PLATE
            };
            assert!(
                corner.z >= under - PLACE,
                "{what}: the pillar is drawn through {corner}, under the fill's surface at \
                 {under}: it still stands where the fill covers it"
            );
        }
        for corner in [DVec3::new(near, 12.0, up), DVec3::new(far, front, PLATE)] {
            assert!(
                round.iter().any(|drawn| drawn.distance(corner) < PLACE),
                "{what}: the pillar has no corner at {corner}, where a side of the fill reaches it"
            );
        }
        let furthest = furthest_along_x(&body, made);
        assert!(
            (furthest - far).abs() < PLACE,
            "{what}: the fill runs on to {furthest}, where its side on the plate meets the \
             pillar at {far}"
        );
        for share_out in [0.2, 0.5] {
            let out = value * share_out;
            let y = 12.0 - out;
            let from = DVec3::new(on_the_pillar(y) - SHORT_OF_THE_END, y, PLATE + 10.0);
            let hit = body.ray_hit(from, -DVec3::Z).unwrap_or_else(|| {
                panic!("{what}: a ray down from {from}, short of the pillar, meets nothing")
            });
            let at = from - DVec3::Z * hit.distance;
            let off = off_the_surface(height, out, at.z - PLATE);
            assert!(
                hit.face == made && off.abs() <= MET,
                "{what}: a ray down from {from}, short of the pillar, meets face {} at {at}, \
                 {off} off the fill's surface",
                hit.face
            );
        }
        for corner in body.triangles_of(made).flatten() {
            assert!(
                off_the_pillar_s_axis(corner) >= PILLAR.1 - PLACE,
                "{what}: the fill is drawn through {corner}, {} from the pillar's axis: inside \
                 the pillar, whose radius is {}",
                off_the_pillar_s_axis(corner),
                PILLAR.1
            );
        }
    }
}

/// How far the [`wall_into_a_leaning_pillar`]'s pillar leans from upright,
/// toward the wall's run: some 17 degrees.
const LEAN: f64 = 0.3;

/// The direction the [`wall_into_a_leaning_pillar`]'s pillar stands along.
fn leaning() -> DVec3 {
    DVec3::new(LEAN.sin(), 0.0, LEAN.cos())
}

/// Where the leaning pillar's axis crosses the plate's top: under the
/// wall's end, as the upright pillar's does.
const LEAN_FOOT: DVec3 = DVec3::new(25.0, 13.0, PLATE);

/// The [`wall_on_the_plate`] and a round pillar of 3 leaning [`LEAN`] toward
/// the wall's run, its axis through (25, 13) on the plate's top, raised 14
/// along it from 3 under the top: its floor inside the plate, its top cap
/// over the wall's, and the wall's end inside it. The foot of the wall's
/// front runs into the pillar, its matter inside its cylinder, and the
/// cylinder's axis is not square to the edge: the plane through the axis
/// the fill is cut at is square to the edge's run off the axis, not to the
/// run itself. The cylinder's origin, the point of its axis nearest the
/// world's, lies some eleven along it under the plate.
fn wall_into_a_leaning_pillar() -> Body {
    let walled = wall_on_the_plate();
    let (lean_sin, lean_cos) = LEAN.sin_cos();
    let frame = Frame {
        origin: LEAN_FOOT - leaning() * 3.0,
        u: DVec3::new(lean_cos, 0.0, -lean_sin),
        v: DVec3::Y,
    };
    let pillar = walled
        .tool_raised(&profile(disc_about(DVec2::ZERO)), frame, leaning() * 14.0)
        .expect("the leaning pillar raises");
    walled.union(&pillar).expect("the leaning pillar is joined")
}

/// The matter the [`wall_into_a_leaning_pillar`] holds: the plate's
/// 40 × 30 × 4; the pillar's above the plate's top, which crosses its axis
/// where the cap stands 11 along it, π 9 × 11; and the wall's 20 × 2 raised
/// 8 less what of it the pillar holds. At `up` over the plate the pillar's
/// section is an ellipse about x = 25 + `up` tan, 3 / cos across along X,
/// and between y = 12 and 14 it holds of the wall, short of x = 25, the
/// disc's segment over the cosine less 2 `up` tan.
fn leaning_pillar_volume() -> f64 {
    let in_the_pillar = 8.0f64.sqrt() + 9.0 * (1.0f64 / 3.0).asin();
    let held = (8.0 * in_the_pillar - 64.0 * LEAN.sin()) / LEAN.cos();
    4800.0 + PI * 9.0 * 11.0 + 320.0 - held
}

/// How far a point stands from the leaning pillar's axis.
fn off_the_leaning_axis(point: DVec3) -> f64 {
    let from = point - LEAN_FOOT;
    (from - leaning() * from.dot(leaning())).length()
}

/// How long a fill at the foot of the wall's front runs, `out` from it and
/// `up` over the plate, the pillar leaning `lean`: from the wall's left end
/// to where the pillar's cylinder stands there, its axis `up` tan along X,
/// and the disc's root over the cosine short of that.
fn along_the_wall_under(lean: f64, out: f64, up: f64) -> f64 {
    let root = (9.0 - (1.0 + out).powi(2)).sqrt();
    20.0 + (lean.sin() * up - root) / lean.cos()
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_wall_running_into_a_leaning_pillar_are_trimmed_flush_on_it()
 {
    let value = 1.0;
    let part = wall_into_a_leaning_pillar();
    let before = "the wall running into a leaning pillar, before any step";
    let round_area = 2.0 * PI * 3.0 * 14.0;
    sound_drawn_within(
        &part,
        leaning_pillar_volume(),
        drawn_within(&part, round_area),
        before,
    );
    for corner in part.triangles_of(PILLAR_ROUND).flatten() {
        assert!(
            (off_the_leaning_axis(corner) - 3.0).abs() < PLACE,
            "{before}: the pillar is drawn through {corner}, {} from its leaning axis",
            off_the_leaning_axis(corner)
        );
    }
    one_edge_of(
        &part,
        [side(TOP), side(WALL_FRONT)],
        [vec![side(WALL_LEFT)], vec![PILLAR_SIDE.to_vec()]],
        &format!("{before}: the foot of the wall's front"),
    );
    let foot = edge_between(&part, TOP, WALL_FRONT);

    for fill in [Fill::Chamfer, Fill::Fillet] {
        let what = format!("the {fill:?} at the foot of a wall running into a leaning pillar");
        let laid = match fill {
            Fill::Chamfer => part
                .chamfer(&foot, value)
                .map(|chamfered| (chamfered.body, chamfered.asked, chamfered.taken)),
            Fill::Fillet => part
                .fillet(&foot, value)
                .map(|filleted| (filleted.body, filleted.asked, filleted.taken)),
        };
        let (body, asked, taken) =
            laid.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(asked, taken, value, &what);
        let height = |out: f64| match fill {
            Fill::Chamfer => chamfer_height(value, out),
            Fill::Fillet => fillet_height(value, out),
        };
        let (matter, laid_area) = match fill {
            Fill::Chamfer => (chamfer_into_the_pillar(value, LEAN), 0.0),
            Fill::Fillet => (
                fill_matter(value, height, |out, up| along_the_wall_under(LEAN, out, up)),
                PI / 2.0 * value * along_the_wall_under(LEAN, 0.0, value),
            ),
        };
        sound_drawn_within(
            &body,
            leaning_pillar_volume() + matter,
            drawn_within(&part, round_area + value * value + laid_area),
            &what,
        );
        let made = match fill {
            Fill::Chamfer => one_flat_face_more(&part, &body, &what),
            Fill::Fillet => one_round_more(&part, &body, &what),
        };
        numbered_after_the_steps_last(&part, &body, made, &what);
        let mut sides = vec![
            side(TOP),
            side(WALL_FRONT),
            side(WALL_LEFT),
            PILLAR_SIDE.to_vec(),
        ];
        sides.sort();
        assert_eq!(
            sides_met(&body, made),
            sides,
            "{what}: the fill meets the plate's top and the wall's front along its sides, and \
             the wall's left end and the pillar at its ends, by one edge each"
        );
        no_corner_of(
            &body,
            vec![side(TOP), PILLAR_SIDE.to_vec()],
            &format!("{what}: the pillar's foot, where the fill's side crosses its line"),
        );
        for corner in body.triangles_of(PILLAR_ROUND).flatten() {
            assert!(
                (off_the_leaning_axis(corner) - 3.0).abs() < PLACE,
                "{what}: the pillar is drawn through {corner}, {} from its leaning axis",
                off_the_leaning_axis(corner)
            );
        }
        for corner in body.triangles_of(made).flatten() {
            assert!(
                off_the_leaning_axis(corner) >= 3.0 - PLACE,
                "{what}: the fill is drawn through {corner}, {} from the pillar's leaning axis: \
                 inside the pillar, whose radius is 3",
                off_the_leaning_axis(corner)
            );
        }
    }
}
