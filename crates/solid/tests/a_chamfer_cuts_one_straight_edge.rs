//! A chamfer cuts one straight edge of an exact body off: given the name the
//! edge answers to (#582) and a distance, it sets the cut back by that
//! distance along each of the edge's two faces — the distance is the
//! set-back, as in every CAD tool, not the width of the face it lays — and a
//! flat face appears between them. A distance the two faces cannot give is
//! reduced to the largest they can, and the step says both. It is an
//! operation of the kernel on [`Body`], beside the join and the cut; the step
//! of the history that calls it comes with the application, in #695.
//!
//! Closes #696.
//! - A chamfer step given the name of a straight edge between two planar faces
//!   of a block, and a distance, leaves a closed body with one more planar
//!   face, set back by that distance along each of the two faces, and passes
//!   the soundness checks —
//!   `a_chamfer_on_a_block_s_top_front_edge_lays_one_flat_face_set_back_by_the_distance_along_the_top_and_the_front`;
//!   the one check the public body cannot be asked, that its listing stands
//!   on its geometry, from inside the crate by
//!   `every_body_a_chamfer_leaves_lists_what_it_is_made_of_as_its_geometry_stands`
//!   in `src/body/tests/chamfer.rs`, on this chamfer, one that takes a whole
//!   face, both of the trapezoid's slanted edges and the edge a join bridged
//! - The same holds on an edge whose two faces meet at an angle other than a
//!   right one —
//!   `a_chamfer_on_an_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_distance_along_each_face`
//! - A distance larger than the neighbouring faces can give is reduced to the
//!   largest that fits, and the step reports the value asked and the value
//!   taken —
//!   `a_chamfer_asked_more_than_the_top_can_give_takes_the_whole_top_and_reports_both_values`,
//!   the other face the limit on the same edge by
//!   `a_chamfer_asked_more_than_the_front_can_give_takes_the_whole_front_and_reports_both_values`,
//!   and a face's reach measured along it rather than square to the other by
//!   `a_chamfer_asked_more_than_a_slanted_wall_can_give_takes_the_wall_s_whole_width_along_it`
//! - After a change of size, a move or a turn upstream, the step cuts the same
//!   edge again, found by its name —
//!   `a_chamfer_cuts_the_edge_its_name_finds_again_after_a_change_of_size_a_move_and_a_turn`
//! - An edge its name no longer finds, gone or split, is a broken step whose
//!   reason says which —
//!   `a_chamfer_on_an_edge_a_cut_took_away_is_a_broken_step_saying_it_is_gone`,
//!   `a_chamfer_on_an_edge_a_cut_parted_in_two_is_a_broken_step_saying_it_is_split`
//! - On two blocks joined so that two edges become one, a chamfer on that edge
//!   cuts it whole — `a_chamfer_on_the_edge_a_join_bridged_from_two_cuts_it_whole`
//! - The face numbers of every existing part are unchanged —
//!   `a_chamfer_leaves_every_other_face_of_the_block_answering_to_the_number_it_had`;
//!   the parts with no chamfer keep theirs by `faces_keep_their_numbers.rs` and
//!   the #682 tests of `edges_and_vertices_keep_their_names.rs`
//! - The exact STEP export of a chamfered block writes the new face —
//!   `a_chamfered_block_written_to_step_writes_its_new_face`

mod step_reader;

use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Chamfered, Declined, EdgeName, Loop, Lost, step};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it: every face
/// here is a plane, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// The numbers a raised block's faces answer to, and a trapezoid's walls.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const SLANT: usize = 3;
const BACK: usize = 4;

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// Where a part is moved to, whole.
const ELSEWHERE: [f64; 3] = [100.0, -40.0, 7.0];

/// The ground's plane with its origin moved to `to`.
fn moved(to: [f64; 3]) -> Frame {
    Frame {
        origin: DVec3::from(to),
        ..ground()
    }
}

/// The ground's plane turned about the vertical by a quarter turn, `quarters`
/// times, its axes written out so that no rounding leans them.
fn quarter_turns(quarters: u32) -> Frame {
    let (u, v) = match quarters % 4 {
        0 => (DVec3::X, DVec3::Y),
        1 => (DVec3::Y, -DVec3::X),
        2 => (-DVec3::X, -DVec3::Y),
        _ => (-DVec3::Y, DVec3::X),
    };
    Frame {
        origin: DVec3::ZERO,
        u,
        v,
    }
}

/// The frame raised along its normal by `height`.
fn lifted(frame: Frame, height: f64) -> Frame {
    Frame {
        origin: frame.origin + frame.normal() * height,
        ..frame
    }
}

/// A point given in the part's own axes, along, across and up, where the
/// frame puts it.
fn placed(frame: Frame, [along, across, up]: [f64; 3]) -> DVec3 {
    frame.at(DVec2::new(along, across)) + frame.normal() * up
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

/// An outline drawn on the ground raised up by `height`, numbered as
/// [`raised_on`] numbers it.
fn raised(outline: Contour, height: f64) -> Body {
    raised_on(ground(), outline, height)
}

/// The part with an outline drawn on the ground, raised from `from` up by
/// `height`, joined to it as `cao_part` joins a step: the tool's faces
/// numbered past the part's.
fn joined_to(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    joined_on(part, lifted(ground(), from), outline, height)
}

/// The part with an outline drawn on a frame, raised along its normal by
/// `height`, joined to it as `cao_part` joins a step.
fn joined_on(part: &Body, frame: Frame, outline: Contour, height: f64) -> Body {
    let tool = part
        .tool_raised(&profile(outline), frame, frame.normal() * height)
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
}

/// The part with an outline drawn on the ground, raised from `from` up by
/// `height`, cut from it as `cao_part` cuts a step: the tool's faces numbered
/// past the part's.
fn cut_from(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    let tool = part
        .tool_raised(&profile(outline), lifted(ground(), from), DVec3::Z * height)
        .expect("the tool raises");
    part.difference(&tool).expect("the cut is built")
}

/// A block of a length, a breadth and a height standing on the ground's
/// origin: its floor numbered nought, its top one, and its sides two to five
/// from the front round to the left.
fn block([length, breadth, height]: [f64; 3]) -> Body {
    raised(
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        height,
    )
}

/// A right trapezoid raised by `height`: its front 40 long on the X axis, its
/// back 34 long at Y = 8, both from the Y axis, and between their right ends
/// a slanted wall 10 long, 6 back and 8 across. The slant meets the front at
/// an acute corner and the back at an obtuse one, whose cosines are 0.6 and
/// −0.6 and whose sines are both 0.8. Its walls are numbered from the front
/// round: the front two, the slant three, the back four and the left five.
fn trapezoid(height: f64) -> Body {
    raised(
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            DVec2::new(34.0, 8.0),
            DVec2::new(0.0, 8.0),
        ]),
        height,
    )
}

/// The two corners an upright edge of a part raised from the ground stands
/// between, at the point of its outline `at`.
fn upright(at: DVec2, height: f64) -> [DVec3; 2] {
    [at.extend(0.0), at.extend(height)]
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

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            expected.iter().any(|point| point.distance(*corner) < PLACE),
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
/// surface closed, no face through another, and the matter its triangles
/// enclose, as the matter it holds, what arithmetic promised.
fn sound(body: &Body, promised: f64, what: &str) {
    sound_drawn_within(body, promised, EXACT * promised, what);
}

/// Holds what [`sound`] holds of a body with curved faces, whose triangles
/// stand within the drawing's tolerance of them: the matter they enclose
/// within `drawn` of what arithmetic promised, the matter the body holds
/// exactly that.
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

/// Holds that a step left every face of the part it was given and made one
/// face more, flat, and gives back the number that face answers to.
fn one_flat_face_more(part: &Body, body: &Body, what: &str) -> usize {
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
    assert!(
        body.is_flat(made[0]),
        "{what}: the face the chamfer lays is flat"
    );
    made[0]
}

/// Holds that a chamfer was given the distance it was asked.
fn taken_whole(chamfered: &Chamfered, distance: f64, what: &str) {
    assert!(
        (chamfered.asked - distance).abs() < PLACE && (chamfered.taken - distance).abs() < PLACE,
        "{what}: asked {} and took {}, where {distance} fits",
        chamfered.asked,
        chamfered.taken
    );
}

/// Holds that a chamfer reports the distance it was asked, and the smaller
/// one the edge's two faces gave it.
fn reduced(chamfered: &Chamfered, asked: f64, taken: f64, what: &str) {
    assert!(
        (chamfered.asked - asked).abs() < PLACE,
        "{what}: reports {} asked, where {asked} was",
        chamfered.asked
    );
    assert!(
        (chamfered.taken - taken).abs() < PLACE,
        "{what}: took {}, where the faces give {taken}",
        chamfered.taken
    );
}

/// Holds that a step took the face numbered `consumed` away whole, left every
/// other face of the part it was given, and made one face more, flat; and
/// gives back the number that face answers to.
fn one_flat_face_for_another(part: &Body, body: &Body, consumed: usize, what: &str) -> usize {
    let (mut before, after) = (numbers(part), numbers(body));
    assert!(
        !after.contains(&consumed),
        "{what}: face {consumed} still answers: {after:?}"
    );
    before.remove(&consumed);
    assert!(
        before.is_subset(&after),
        "{what}: a face other than {consumed} went: {after:?}"
    );
    let made: Vec<usize> = after.difference(&before).copied().collect();
    assert_eq!(
        made.len(),
        1,
        "{what}: one face for face {consumed}: {after:?}"
    );
    assert!(
        body.is_flat(made[0]),
        "{what}: the face the chamfer lays is flat"
    );
    made[0]
}

/// Holds that a flat face faces the way `toward` points.
fn faces_toward(body: &Body, face: usize, toward: DVec3, what: &str) {
    let facing = body
        .plane_of(face)
        .expect("a flat face offers a plane")
        .normal;
    assert!(
        facing.distance(toward.normalize()) < PLACE,
        "{what}: the chamfer faces {facing}, where it faces {}",
        toward.normalize()
    );
}

#[test]
fn a_chamfer_on_a_block_s_top_front_edge_lays_one_flat_face_set_back_by_the_distance_along_the_top_and_the_front()
 {
    let [length, breadth, height] = [30.0, 10.0, 20.0];
    let distance = 3.0;
    let block = block([length, breadth, height]);
    let edge = edge_between(&block, TOP, FRONT);

    let chamfered = block.chamfer(&edge, distance).expect("the chamfer is cut");

    let what = "the top front edge";
    taken_whole(&chamfered, distance, what);
    let body = &chamfered.body;
    sound(
        body,
        length * breadth * height - length * distance * distance / 2.0,
        what,
    );
    let chamfer = one_flat_face_more(&block, body, what);
    let facing = body
        .plane_of(chamfer)
        .expect("a flat face offers a plane")
        .normal;
    let between = DVec3::new(0.0, -1.0, 1.0).normalize();
    assert!(
        facing.distance(between) < PLACE,
        "the chamfer faces {facing}, where it faces {between}, halfway between the top and the front"
    );
    let on_top = |along: f64| DVec3::new(along, distance, height);
    let on_front = |along: f64| DVec3::new(along, 0.0, height - distance);
    corners_are(
        body,
        chamfer,
        &[on_top(0.0), on_top(length), on_front(length), on_front(0.0)],
        "the chamfer",
    );
    corners_are(
        body,
        TOP,
        &[
            on_top(0.0),
            on_top(length),
            DVec3::new(length, breadth, height),
            DVec3::new(0.0, breadth, height),
        ],
        "what is left of the top",
    );
    corners_are(
        body,
        FRONT,
        &[
            DVec3::ZERO,
            DVec3::new(length, 0.0, 0.0),
            on_front(length),
            on_front(0.0),
        ],
        "what is left of the front",
    );
}

/// An upright edge of the trapezoid where two of its walls meet at a slant:
/// the walls, where a set-back of 2 along each ends, the far end of each wall
/// from the edge, and the way the chamfer faces, all in the ground's plane.
struct Slanted {
    what: &'static str,
    walls: [usize; 2],
    set_back: [DVec2; 2],
    far: [DVec2; 2],
    facing: DVec2,
}

#[test]
fn a_chamfer_on_an_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_distance_along_each_face() {
    let (height, distance) = (5.0, 2.0);
    let corners = [
        Slanted {
            what: "the acute edge between the front and the slant",
            walls: [FRONT, SLANT],
            set_back: [DVec2::new(38.0, 0.0), DVec2::new(38.8, 1.6)],
            far: [DVec2::ZERO, DVec2::new(34.0, 8.0)],
            facing: DVec2::new(2.0, -1.0),
        },
        Slanted {
            what: "the obtuse edge between the slant and the back",
            walls: [SLANT, BACK],
            set_back: [DVec2::new(35.2, 6.4), DVec2::new(32.0, 8.0)],
            far: [DVec2::new(40.0, 0.0), DVec2::new(0.0, 8.0)],
            facing: DVec2::new(1.0, 2.0),
        },
    ];

    for corner in corners {
        let what = corner.what;
        let part = trapezoid(height);
        let edge = edge_between(&part, corner.walls[0], corner.walls[1]);

        let chamfered = part
            .chamfer(&edge, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(&chamfered, distance, what);
        let body = &chamfered.body;
        let trapezoid = (40.0 + 34.0) / 2.0 * 8.0 * height;
        let corner_cut_off = distance * distance * 0.8 / 2.0 * height;
        sound(body, trapezoid - corner_cut_off, what);
        let chamfer = one_flat_face_more(&part, body, what);
        let facing = body
            .plane_of(chamfer)
            .expect("a flat face offers a plane")
            .normal;
        let between = corner.facing.normalize().extend(0.0);
        assert!(
            facing.distance(between) < PLACE,
            "{what}: the chamfer faces {facing}, where it faces {between}, halfway between its walls"
        );
        let [first, second] = corner.set_back.map(|at| upright(at, height));
        let [first_far, second_far] = corner.far.map(|at| upright(at, height));
        corners_are(body, chamfer, &[first, second].concat(), what);
        corners_are(
            body,
            corner.walls[0],
            &[first, first_far].concat(),
            &format!("what is left of the first wall at {what}"),
        );
        corners_are(
            body,
            corner.walls[1],
            &[second, second_far].concat(),
            &format!("what is left of the second wall at {what}"),
        );
    }
}

#[test]
fn a_chamfer_asked_more_than_the_top_can_give_takes_the_whole_top_and_reports_both_values() {
    let [length, breadth, height] = [30.0, 10.0, 20.0];
    let block = block([length, breadth, height]);
    let edge = edge_between(&block, TOP, FRONT);

    let chamfered = block
        .chamfer(&edge, 50.0)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "50 asked on the top front edge, the top 10 wide";
    reduced(&chamfered, 50.0, breadth, what);
    let body = &chamfered.body;
    sound(
        body,
        length * breadth * height - length * breadth * breadth / 2.0,
        what,
    );
    let chamfer = one_flat_face_for_another(&block, body, TOP, what);
    faces_toward(body, chamfer, DVec3::new(0.0, -1.0, 1.0), what);
    let on_back = |along: f64| DVec3::new(along, breadth, height);
    let on_front = |along: f64| DVec3::new(along, 0.0, height - breadth);
    corners_are(
        body,
        chamfer,
        &[
            on_back(0.0),
            on_back(length),
            on_front(length),
            on_front(0.0),
        ],
        "the chamfer, from the front to the top of the back",
    );
    corners_are(
        body,
        FRONT,
        &[
            DVec3::ZERO,
            DVec3::new(length, 0.0, 0.0),
            on_front(length),
            on_front(0.0),
        ],
        "what is left of the front",
    );
    corners_are(
        body,
        BACK,
        &[
            DVec3::new(0.0, breadth, 0.0),
            DVec3::new(length, breadth, 0.0),
            on_back(length),
            on_back(0.0),
        ],
        "the back, whole",
    );
}

#[test]
fn a_chamfer_asked_more_than_the_front_can_give_takes_the_whole_front_and_reports_both_values() {
    let [length, breadth, height] = [30.0, 20.0, 8.0];
    let block = block([length, breadth, height]);
    let edge = edge_between(&block, TOP, FRONT);

    let chamfered = block
        .chamfer(&edge, 50.0)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "50 asked on the top front edge, the front 8 high";
    reduced(&chamfered, 50.0, height, what);
    let body = &chamfered.body;
    sound(
        body,
        length * breadth * height - length * height * height / 2.0,
        what,
    );
    let chamfer = one_flat_face_for_another(&block, body, FRONT, what);
    faces_toward(body, chamfer, DVec3::new(0.0, -1.0, 1.0), what);
    let on_top = |along: f64| DVec3::new(along, height, height);
    let on_floor = |along: f64| DVec3::new(along, 0.0, 0.0);
    corners_are(
        body,
        chamfer,
        &[on_top(0.0), on_top(length), on_floor(length), on_floor(0.0)],
        "the chamfer, from the top to the front of the floor",
    );
    corners_are(
        body,
        TOP,
        &[
            on_top(0.0),
            on_top(length),
            DVec3::new(length, breadth, height),
            DVec3::new(0.0, breadth, height),
        ],
        "what is left of the top",
    );
    corners_are(
        body,
        FLOOR,
        &[
            on_floor(0.0),
            on_floor(length),
            DVec3::new(length, breadth, 0.0),
            DVec3::new(0.0, breadth, 0.0),
        ],
        "the floor, whole",
    );
}

#[test]
fn a_chamfer_asked_more_than_a_slanted_wall_can_give_takes_the_wall_s_whole_width_along_it() {
    let height = 5.0;
    let part = trapezoid(height);
    let edge = edge_between(&part, FRONT, SLANT);

    let chamfered = part
        .chamfer(&edge, 50.0)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "50 asked on the front slant edge, the slant 10 wide and 8 off the front";
    reduced(&chamfered, 50.0, 10.0, what);
    let body = &chamfered.body;
    let trapezoid = (40.0 + 34.0) / 2.0 * 8.0 * height;
    let corner_cut_off = 10.0 * 10.0 * 0.8 / 2.0 * height;
    sound(body, trapezoid - corner_cut_off, what);
    let chamfer = one_flat_face_for_another(&part, body, SLANT, what);
    faces_toward(body, chamfer, DVec3::new(2.0, -1.0, 0.0), what);
    let on_front = upright(DVec2::new(30.0, 0.0), height);
    let on_back = upright(DVec2::new(34.0, 8.0), height);
    corners_are(
        body,
        chamfer,
        &[on_front, on_back].concat(),
        "the chamfer, from the front to the end of the back",
    );
    corners_are(
        body,
        FRONT,
        &[upright(DVec2::ZERO, height), on_front].concat(),
        "what is left of the front",
    );
    corners_are(
        body,
        BACK,
        &[on_back, upright(DVec2::new(0.0, 8.0), height)].concat(),
        "the back, whole",
    );
}

/// An L lying on its frame: an arm `length` long and `depth` deep along the
/// frame's first axis, and at the arm's left end a stem `stem` wide reaching
/// back to `back`, raised by `height`. Nothing turns it onto itself, so a cut
/// on one of its edges can land on no twin. Its walls are numbered from the
/// front round: the front two, the arm's right end three, the arm's back
/// four, the stem's right side five, the stem's back six and the left seven.
#[derive(Clone, Copy)]
struct Ell {
    length: f64,
    depth: f64,
    stem: f64,
    back: f64,
    height: f64,
}

const ELL: Ell = Ell {
    length: 30.0,
    depth: 6.0,
    stem: 12.0,
    back: 14.0,
    height: 10.0,
};

const ELL_RESIZED: Ell = Ell {
    length: 40.0,
    depth: 8.0,
    stem: 16.0,
    back: 20.0,
    height: 12.0,
};

/// The numbers an L's arm's back, its stem's right side and its stem's back
/// answer to.
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;
const STEM_BACK: usize = 6;

impl Ell {
    fn on(&self, frame: Frame) -> Body {
        let Ell {
            length,
            depth,
            stem,
            back,
            height,
        } = *self;
        raised_on(
            frame,
            Contour::straight(vec![
                DVec2::ZERO,
                DVec2::new(length, 0.0),
                DVec2::new(length, depth),
                DVec2::new(stem, depth),
                DVec2::new(stem, back),
                DVec2::new(0.0, back),
            ]),
            height,
        )
    }

    fn volume(&self) -> f64 {
        (self.length * self.depth + self.stem * (self.back - self.depth)) * self.height
    }
}

#[test]
fn a_chamfer_cuts_the_edge_its_name_finds_again_after_a_change_of_size_a_move_and_a_turn() {
    let distance = 2.0;
    let name = edge_between(&ELL.on(ground()), TOP, STEM_BACK);
    let moved_and_turned = Frame {
        origin: DVec3::from(ELSEWHERE),
        ..quarter_turns(1)
    };

    for (what, ell, frame) in [
        ("as drawn", ELL, ground()),
        ("resized", ELL_RESIZED, ground()),
        ("moved", ELL, moved(ELSEWHERE)),
        ("turned a quarter", ELL, quarter_turns(1)),
        ("turned a half", ELL, quarter_turns(2)),
        ("turned three quarters", ELL, quarter_turns(3)),
        ("resized, moved and turned", ELL_RESIZED, moved_and_turned),
    ] {
        let part = ell.on(frame);

        let chamfered = part
            .chamfer(&name, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(&chamfered, distance, what);
        let body = &chamfered.body;
        let corner_cut_off = ell.stem * distance * distance / 2.0;
        sound(body, ell.volume() - corner_cut_off, what);
        let chamfer = one_flat_face_more(&part, body, what);
        let at = |point: [f64; 3]| placed(frame, point);
        faces_toward(body, chamfer, at([0.0, 1.0, 1.0]) - at([0.0; 3]), what);
        let Ell {
            stem, back, height, ..
        } = ell;
        corners_are(
            body,
            chamfer,
            &[
                at([0.0, back - distance, height]),
                at([stem, back - distance, height]),
                at([stem, back, height - distance]),
                at([0.0, back, height - distance]),
            ],
            &format!("the chamfer on the stem's back, {what}"),
        );
    }
}

#[test]
fn a_chamfer_on_an_edge_a_cut_took_away_is_a_broken_step_saying_it_is_gone() {
    let drawn = block([30.0, 10.0, 10.0]);
    let name = edge_between(&drawn, TOP, FRONT);
    let rabbeted = cut_from(
        &drawn,
        Contour::rectangle(DVec2::new(-1.0, -1.0), DVec2::new(31.0, 2.0)),
        8.0,
        3.0,
    );
    assert_eq!(
        rabbeted.lost(&name),
        Some(Lost::Gone),
        "a rabbet all along the edge leaves the top and the front meeting nowhere"
    );

    let declined = rabbeted.chamfer(&name, 1.0).err();

    assert_eq!(declined, Some(Declined::Lost(Lost::Gone)));
}

#[test]
fn a_chamfer_on_an_edge_a_cut_parted_in_two_is_a_broken_step_saying_it_is_split() {
    let drawn = block([30.0, 10.0, 10.0]);
    let name = edge_between(&drawn, TOP, FRONT);
    let notched = cut_from(
        &drawn,
        Contour::rectangle(DVec2::new(10.0, -1.0), DVec2::new(20.0, 3.0)),
        7.0,
        4.0,
    );
    assert_eq!(
        notched.lost(&name),
        Some(Lost::Split { now: 2 }),
        "a notch a third of the way along leaves the edge in two pieces"
    );

    let declined = notched.chamfer(&name, 1.0).err();

    assert_eq!(declined, Some(Declined::Lost(Lost::Split { now: 2 })));
}

#[test]
fn a_chamfer_on_the_edge_a_join_bridged_from_two_cuts_it_whole() {
    let [length, breadth, height] = [15.0, 10.0, 10.0];
    let distance = 2.0;
    let second_from = |from: f64| {
        joined_to(
            &block([length, breadth, height]),
            Contour::rectangle(DVec2::new(from, 0.0), DVec2::new(from + length, breadth)),
            0.0,
            height,
        )
    };
    let apart = second_from(length + 5.0);
    let (second_top, second_front) = (TOP + 6, FRONT + 6);
    let names = [
        ("the first block's", edge_between(&apart, TOP, FRONT)),
        (
            "the second block's",
            edge_between(&apart, second_top, second_front),
        ),
    ];
    let abutting = second_from(length);
    let merged = [
        vec![TOP as u32, second_top as u32],
        vec![FRONT as u32, second_front as u32],
    ];
    let bridged: Vec<EdgeName> = abutting
        .edge_names()
        .into_iter()
        .filter(|name| name.between == merged)
        .collect();
    assert_eq!(
        bridged.len(),
        1,
        "the tops and the fronts merge, and meet along one edge: {bridged:?}"
    );
    let whole = 2.0 * length;

    for (whose, name) in names {
        let what = format!("by {whose} top front edge");
        assert_eq!(
            abutting.lost(&name),
            None,
            "{what}: the name finds the edge the join bridged"
        );

        let chamfered = abutting
            .chamfer(&name, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(&chamfered, distance, &what);
        let body = &chamfered.body;
        sound(
            body,
            whole * breadth * height - whole * distance * distance / 2.0,
            &what,
        );
        let chamfer = one_flat_face_more(&abutting, body, &what);
        faces_toward(body, chamfer, DVec3::new(0.0, -1.0, 1.0), &what);
        corners_are(
            body,
            chamfer,
            &[
                DVec3::new(0.0, distance, height),
                DVec3::new(whole, distance, height),
                DVec3::new(whole, 0.0, height - distance),
                DVec3::new(0.0, 0.0, height - distance),
            ],
            &format!("the chamfer {what}, all along both blocks"),
        );
    }
}

#[test]
fn a_chamfer_leaves_every_other_face_of_the_block_answering_to_the_number_it_had() {
    let block = block([30.0, 10.0, 20.0]);
    let edge = edge_between(&block, TOP, FRONT);

    let chamfered = block.chamfer(&edge, 3.0).expect("the chamfer is cut");

    let body = &chamfered.body;
    let before = numbers(&block);
    let mut kept_and_made = before.clone();
    kept_and_made.insert(block.faces_end());
    assert_eq!(
        numbers(body),
        kept_and_made,
        "every number the block's faces answered to, and the chamfer's where the steps' numbers ended"
    );
    assert_eq!(
        body.faces_end(),
        block.faces_end() + 1,
        "the chamfer names one face, and the next step numbers its own past it"
    );
    for number in before {
        let was = block
            .plane_of(number)
            .expect("a block's face offers a plane");
        let is = body
            .plane_of(number)
            .unwrap_or_else(|| panic!("face {number} offers no plane"));
        assert!(
            is.normal.distance(was.normal) < PLACE,
            "face {number} faces {}, where it faced {}",
            is.normal,
            was.normal
        );
        for corner in &is.corners {
            let off = (*corner - was.corners[0]).dot(was.normal);
            assert!(
                off.abs() < PLACE,
                "face {number} has a corner at {corner}, {off} off the plane it lay on"
            );
        }
    }
    for untouched in [FLOOR, BACK] {
        let was = block
            .plane_of(untouched)
            .expect("a block's face offers a plane");
        corners_are(
            body,
            untouched,
            &was.corners,
            &format!("face {untouched}, which the chamfer does not reach"),
        );
    }
}

#[test]
fn a_chamfered_block_written_to_step_writes_its_new_face() {
    let [length, breadth, height] = [30.0, 10.0, 20.0];
    let distance = 3.0;
    let block = block([length, breadth, height]);
    let chamfered = block
        .chamfer(&edge_between(&block, TOP, FRONT), distance)
        .expect("the chamfer is cut");

    let text = step::of(&chamfered.body, 1.0).expect("an exact body is written");

    let solid = step_reader::read(&text)
        .unwrap_or_else(|fault| panic!("the file does not read back: {fault}"));
    assert_eq!(solid.faces, 7, "the block's six faces and the chamfer");
    assert!(
        solid.worst <= solid.uncertainty,
        "a loop stands {} off its face or its edge, the file's tolerance is {}",
        solid.worst,
        solid.uncertainty
    );
    let promised = length * breadth * height - length * distance * distance / 2.0;
    assert!(
        (solid.volume - promised).abs() <= solid.uncertainty * solid.area,
        "the file encloses {}, where arithmetic promised {promised}",
        solid.volume
    );
    let facing = DVec3::new(0.0, -1.0, 1.0).normalize();
    let through = [
        DVec3::new(0.0, distance, height),
        DVec3::new(length, 0.0, height - distance),
    ];
    assert!(
        solid.planes.iter().any(|&(origin, normal)| {
            normal.distance(facing) < PLACE
                && through
                    .iter()
                    .all(|point| (*point - origin).dot(normal).abs() <= solid.uncertainty)
        }),
        "no face of the file faces {facing} through {through:?}: {:?}",
        solid.planes
    );
}

#[test]
fn a_chamfer_on_the_inner_corner_of_an_l_where_its_faces_meet_inwards_is_declined_as_unsupported() {
    let ell = ELL.on(ground());
    let inner = edge_between(&ell, ARM_BACK, STEM_RIGHT);

    let declined = ell.chamfer(&inner, 1.0).err();

    assert_eq!(declined, Some(Declined::Unsupported));
}

#[test]
fn a_chamfer_on_the_round_rim_of_a_disc_is_declined_as_unsupported() {
    let disc = raised(
        Contour {
            corners: vec![DVec2::new(10.0, 0.0)],
            runs: vec![Run::Round {
                center: DVec2::ZERO,
                turn: TAU,
            }],
        },
        5.0,
    );
    let rim = edge_between(&disc, TOP, 2);

    let declined = disc.chamfer(&rim, 1.0).err();

    assert_eq!(declined, Some(Declined::Unsupported));
}

#[test]
fn a_chamfer_whose_cut_carried_along_its_edge_would_run_into_the_l_s_stem_is_declined_as_unsupported()
 {
    let ell = ELL.on(ground());
    let arm_back = edge_between(&ell, TOP, ARM_BACK);

    let declined = ell.chamfer(&arm_back, 1.0).err();

    assert_eq!(declined, Some(Declined::Unsupported));
}

/// The block a chamfer of 3 is asked on its top front edge, 30 long, 10 deep
/// and 20 high. Today the cut is a slab on the chamfer's plane, z − y = 17,
/// as long as the part's box along the edge and 3 more at each end, its sides
/// where y + z is 14 and 26, its top where z − y is 17 + 6√2: the shapes
/// below are drawn against it.
fn block_to_chamfer() -> Body {
    block([30.0, 10.0, 20.0])
}

/// Holds that a chamfer of 3 on the top front edge of a part built on
/// [`block_to_chamfer`] is declined as a cut the step cannot make.
fn declined_on_the_top_front_edge(part: &Body, what: &str) {
    let edge = edge_between(part, TOP, FRONT);

    let declined = part.chamfer(&edge, 3.0).err();

    assert_eq!(declined, Some(Declined::Unsupported), "{what}");
}

#[test]
fn a_chamfer_whose_cut_would_take_a_tab_beside_its_edge_away_whole_is_declined_as_unsupported() {
    let tabbed = joined_to(
        &block_to_chamfer(),
        Contour::rectangle(DVec2::new(-1.0, 0.2), DVec2::new(0.0, 1.0)),
        19.0,
        0.8,
    );

    declined_on_the_top_front_edge(
        &tabbed,
        "a tab on the left side, wholly above the chamfer's plane",
    );
}

#[test]
fn a_chamfer_whose_cut_would_take_away_a_lump_clear_of_its_edge_is_declined_as_unsupported() {
    let with_a_lump = joined_to(
        &block_to_chamfer(),
        Contour::rectangle(DVec2::new(10.0, -1.0), DVec2::new(12.0, -0.5)),
        20.5,
        0.5,
    );

    declined_on_the_top_front_edge(
        &with_a_lump,
        "a lump standing clear in front of the edge and above it",
    );
}

#[test]
fn a_chamfer_whose_cut_carried_along_its_edge_would_reach_a_second_block_in_line_is_declined_as_unsupported()
 {
    let first = block([15.0, 10.0, 10.0]);
    let in_line = joined_to(
        &first,
        Contour::rectangle(DVec2::new(20.0, 0.0), DVec2::new(35.0, 10.0)),
        0.0,
        10.0,
    );
    let edge = edge_between(&in_line, TOP, FRONT);

    let declined = in_line.chamfer(&edge, 2.0).err();

    assert_eq!(
        declined,
        Some(Declined::Unsupported),
        "the cut would chamfer the second block's top front edge as well"
    );
}

#[test]
fn a_chamfer_whose_cut_would_bite_into_a_rib_beside_its_edge_is_declined_as_unsupported() {
    let ribbed = joined_to(
        &block_to_chamfer(),
        Contour::rectangle(DVec2::new(-1.0, 0.2), DVec2::new(0.0, 3.5)),
        10.0,
        9.8,
    );

    declined_on_the_top_front_edge(
        &ribbed,
        "a rib on the left side the chamfer's plane crosses, every face of it kept in part",
    );
}

#[test]
fn a_chamfer_whose_cut_would_run_into_a_lump_above_its_face_is_declined_as_unsupported() {
    let with_a_lump = joined_to(
        &block_to_chamfer(),
        Contour::rectangle(DVec2::new(10.0, -6.0), DVec2::new(12.0, -2.5)),
        15.0,
        6.0,
    );

    declined_on_the_top_front_edge(
        &with_a_lump,
        "a lump clear of the chamfer's plane, crossing the cut's top and its side",
    );
}

/// A D standing on its flat: a half disc of radius 5 about the frame's
/// origin raised by 20, its straight front along the frame's first axis from
/// −5 to 5, its arc behind turning back 5 across. Its floor nought, its top
/// one, its front two and its arc three.
fn d_on(frame: Frame) -> Body {
    raised_on(
        frame,
        Contour {
            corners: vec![DVec2::new(-5.0, 0.0), DVec2::new(5.0, 0.0)],
            runs: vec![
                Run::Straight,
                Run::Round {
                    center: DVec2::ZERO,
                    turn: PI,
                },
            ],
        },
        20.0,
    )
}

/// The ground's plane turned about the vertical by `degrees`.
fn turned_by(degrees: f64) -> Frame {
    let (sin, cos) = degrees.to_radians().sin_cos();
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::new(cos, sin, 0.0),
        v: DVec3::new(-sin, cos, 0.0),
    }
}

/// The matter a chamfer set back by `distance` takes off the D's top front
/// edge: over the half disc where y ≤ s, a height of s − y, which is
/// 2s·∫√(25 − y²) − 2∫y√(25 − y²) from nought to s.
fn d_cut_off(distance: f64) -> f64 {
    let s = distance;
    let under_the_arc = s / 2.0 * (25.0 - s * s).sqrt() + 12.5 * (s / 5.0).asin();
    let moment = (125.0 - (25.0 - s * s).powf(1.5)) / 3.0;
    2.0 * s * under_the_arc - 2.0 * moment
}

/// What the triangles of a D may enclose apart from its matter: the
/// drawing's tolerance, two ten-thousandths of the part's reach of 20, times
/// the D's whole surface, 125π + 200, which is all a surface moved that far
/// can change.
fn d_drawn_within() -> f64 {
    2e-4 * 20.0 * (125.0 * PI + 200.0)
}

/// Holds that a chamfer on the D's top front edge, set back by `taken`,
/// left a sound body facing halfway between the top and the front.
fn d_chamfered(part: &Body, frame: Frame, chamfered: &Chamfered, what: &str) {
    let body = &chamfered.body;
    sound_drawn_within(
        body,
        250.0 * PI - d_cut_off(chamfered.taken),
        d_drawn_within(),
        what,
    );
    let made: Vec<usize> = numbers(body).difference(&numbers(part)).copied().collect();
    assert_eq!(made.len(), 1, "{what}: one face made: {made:?}");
    let at = |point: [f64; 3]| placed(frame, point);
    faces_toward(body, made[0], at([0.0, -1.0, 1.0]) - at([0.0; 3]), what);
}

#[test]
fn a_chamfer_on_the_flat_of_a_d_takes_the_distance_asked_where_the_arc_behind_gives_more() {
    let d = d_on(ground());
    let edge = edge_between(&d, TOP, FRONT);

    let chamfered = d
        .chamfer(&edge, 3.0)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "3 asked on a D whose top reaches 5 back";
    taken_whole(&chamfered, 3.0, what);
    d_chamfered(&d, ground(), &chamfered, what);
}

#[test]
fn a_chamfer_asked_more_than_a_d_s_top_can_give_takes_it_whole_to_where_its_arc_turns_back() {
    let d = d_on(ground());
    let edge = edge_between(&d, TOP, FRONT);

    let chamfered = d
        .chamfer(&edge, 50.0)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "50 asked on a D whose top reaches 5 back";
    reduced(&chamfered, 50.0, 5.0, what);
    d_chamfered(&d, ground(), &chamfered, what);
    assert!(
        !chamfered.body.has_face(TOP),
        "{what}: the top is taken whole"
    );
}

#[test]
fn a_d_turned_about_the_vertical_gives_its_chamfer_the_same_distance() {
    for degrees in [3.0, 10.0] {
        let frame = turned_by(degrees);
        let d = d_on(frame);
        let edge = edge_between(&d, TOP, FRONT);
        for (asked, taken) in [(3.0, 3.0), (50.0, 5.0)] {
            let what = format!("{asked} asked on a D turned by {degrees}°");

            let chamfered = d
                .chamfer(&edge, asked)
                .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

            reduced(&chamfered, asked, taken, &what);
            d_chamfered(&d, frame, &chamfered, &what);
        }
    }
}

#[test]
fn a_chamfer_on_a_straight_edge_running_on_tangentially_into_a_round_is_declined_as_unsupported() {
    let rounded = raised(
        Contour {
            corners: vec![
                DVec2::new(2.0, 0.0),
                DVec2::new(28.0, 0.0),
                DVec2::new(30.0, 2.0),
                DVec2::new(30.0, 8.0),
                DVec2::new(28.0, 10.0),
                DVec2::new(2.0, 10.0),
                DVec2::new(0.0, 8.0),
                DVec2::new(0.0, 2.0),
            ],
            runs: [(28.0, 2.0), (28.0, 8.0), (2.0, 8.0), (2.0, 2.0)]
                .into_iter()
                .flat_map(|(x, y)| {
                    [
                        Run::Straight,
                        Run::Round {
                            center: DVec2::new(x, y),
                            turn: PI / 2.0,
                        },
                    ]
                })
                .collect(),
        },
        20.0,
    );
    let edge = edge_between(&rounded, TOP, FRONT);

    let declined = rounded.chamfer(&edge, 1.0).err();

    assert_eq!(
        declined,
        Some(Declined::Unsupported),
        "the front runs on into the corner rounds at both ends: a tangent chain"
    );
}

#[test]
fn a_chamfer_distance_that_is_no_length_the_tolerance_holds_is_declined_saying_so() {
    let block = block_to_chamfer();
    let edge = edge_between(&block, TOP, FRONT);

    for distance in [f64::NAN, f64::NEG_INFINITY, -1.0, 0.0, 1e-8, 1e-12] {
        let declined = block.chamfer(&edge, distance).err();

        assert_eq!(declined, Some(Declined::Distance), "{distance} asked");
    }
}

#[test]
fn a_chamfer_asked_an_infinite_distance_takes_the_most_the_faces_give() {
    let block = block_to_chamfer();
    let edge = edge_between(&block, TOP, FRONT);

    let chamfered = block
        .chamfer(&edge, f64::INFINITY)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "an infinite distance asked on a top 10 wide";
    assert_eq!(chamfered.asked, f64::INFINITY, "{what}: the value asked");
    assert!(
        (chamfered.taken - 10.0).abs() < PLACE,
        "{what}: took {}, where the top gives 10",
        chamfered.taken
    );
    sound(&chamfered.body, 6000.0 - 30.0 * 10.0 * 10.0 / 2.0, what);
}

#[test]
fn a_chamfer_on_a_part_whose_top_two_slots_left_in_pieces_cuts_its_edge_and_keeps_every_piece() {
    let slot = |part: &Body, from: f64| {
        cut_from(
            part,
            Contour::rectangle(DVec2::new(-1.0, from), DVec2::new(31.0, from + 1.0)),
            15.0,
            6.0,
        )
    };
    let slotted = slot(&slot(&block_to_chamfer(), 4.0), 7.0);
    let pieces = slotted.pieces();
    assert_eq!(
        pieces.len(),
        2,
        "two slots across the top leave it in three pieces, two of them numbered apart: {pieces:?}"
    );
    assert_eq!(
        slotted.parted_from(pieces[0]),
        Some(TOP),
        "the first slot parts the top"
    );
    assert_eq!(
        slotted.parted_from(pieces[1]),
        Some(pieces[0]),
        "the second slot parts the piece the first left behind it"
    );
    let edge = edge_between(&slotted, TOP, FRONT);

    let chamfered = slotted
        .chamfer(&edge, 3.0)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "3 asked on the top front edge, the top's front piece 4 deep";
    taken_whole(&chamfered, 3.0, what);
    let body = &chamfered.body;
    sound(
        body,
        6000.0 - 2.0 * 30.0 * 5.0 - 30.0 * 3.0 * 3.0 / 2.0,
        what,
    );
    let mut kept_and_made = numbers(&slotted);
    kept_and_made.insert(slotted.faces_end());
    assert_eq!(
        numbers(body),
        kept_and_made,
        "{what}: every number and every piece's, and the chamfer's where the steps' numbers ended"
    );
    assert_eq!(body.pieces(), pieces, "{what}: the pieces");
    for piece in pieces {
        assert_eq!(
            body.parted_from(piece),
            slotted.parted_from(piece),
            "{what}: piece {piece} answers to the face it was parted from"
        );
    }
    let chamfer = slotted.faces_end();
    faces_toward(body, chamfer, DVec3::new(0.0, -1.0, 1.0), what);
    corners_are(
        body,
        chamfer,
        &[
            DVec3::new(0.0, 3.0, 20.0),
            DVec3::new(30.0, 3.0, 20.0),
            DVec3::new(30.0, 0.0, 17.0),
            DVec3::new(0.0, 0.0, 17.0),
        ],
        what,
    );
}

#[test]
fn a_chamfer_shorter_than_one_on_a_long_edge_cuts_it_from_end_to_end() {
    let block = block_to_chamfer();
    let edge = edge_between(&block, TOP, FRONT);

    let chamfered = block
        .chamfer(&edge, 0.5)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "0.5 asked on an edge 30 long";
    taken_whole(&chamfered, 0.5, what);
    let body = &chamfered.body;
    sound(body, 6000.0 - 30.0 * 0.5 * 0.5 / 2.0, what);
    let chamfer = one_flat_face_more(&block, body, what);
    corners_are(
        body,
        chamfer,
        &[
            DVec3::new(0.0, 0.5, 20.0),
            DVec3::new(30.0, 0.5, 20.0),
            DVec3::new(30.0, 0.0, 19.5),
            DVec3::new(0.0, 0.0, 19.5),
        ],
        "the chamfer, from one end of the edge to the other",
    );
}

#[test]
fn a_chamfer_on_an_edge_another_edge_carries_on_in_line_is_declined_as_unsupported() {
    let across = |at: f64| Frame {
        origin: DVec3::X * at,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let upright = joined_on(
        &Body::default(),
        across(0.0),
        Contour::rectangle(DVec2::ZERO, DVec2::new(10.0, 20.0)),
        15.0,
    );
    let carried_on = joined_on(
        &upright,
        across(15.0),
        Contour::straight(vec![
            DVec2::new(-5.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(10.0, 20.0),
            DVec2::new(0.0, 20.0),
        ]),
        15.0,
    );
    let (top, upright_front, slant) = (vec![4, 10], vec![5], vec![11]);
    let names = carried_on.edge_names();
    let named = |front: &Vec<u32>| {
        names
            .iter()
            .find(|name| name.between == [top.clone(), front.clone()])
            .cloned()
            .unwrap_or_else(|| panic!("no edge between {top:?} and {front:?}: {names:?}"))
    };
    let (edge, _carrying_it_on) = (named(&upright_front), named(&slant));

    let declined = carried_on.chamfer(&edge, 2.0).err();

    assert_eq!(
        declined,
        Some(Declined::Unsupported),
        "the edge between the top and the upright front runs on in line into the edge \
         between the top and the slant: the tangent chain rule declines it first, and the \
         cut carried along the line would bite the slant behind it"
    );
}

/// A hole of radius 2 bored through the block to chamfer, drawn as one
/// whole turn from `start`: one edge round its rim and no vertex on it, the
/// turn starting wherever the drawing put its corner.
fn bored_through(centre: DVec2, start: DVec2) -> Body {
    cut_from(
        &block_to_chamfer(),
        Contour {
            corners: vec![start],
            runs: vec![Run::Round {
                center: centre,
                turn: TAU,
            }],
        },
        -1.0,
        22.0,
    )
}

/// An elliptic hole bored through the block to chamfer, 2 across along X and
/// 1.5 along Y, drawn as one whole turn from the end of its first axis, at
/// `start` along X from its centre.
fn bored_through_an_ellipse(centre: DVec2, start: f64) -> Body {
    cut_from(
        &block_to_chamfer(),
        Contour {
            corners: vec![centre + DVec2::X * start],
            runs: vec![Run::Oval {
                center: centre,
                first: DVec2::X * start,
                second: 1.5,
                turn: TAU,
            }],
        },
        -1.0,
        22.0,
    )
}

/// What the triangles of the block bored through may enclose apart from its
/// matter: the drawing's tolerance, two ten-thousandths of the block's reach
/// of 30, times the wall of a round hole of radius 2 through its 20, which no
/// hole here outgrows; the flat faces are drawn on their planes.
fn bored_drawn_within() -> f64 {
    2e-4 * 30.0 * 2.0 * PI * 2.0 * 20.0
}

/// Holds that 50 asked on the edge between the top and the `side` of a part
/// bored through 3 from that edge takes the 3 the top gives up to the hole's
/// near side, and leaves a sound body facing halfway out between them.
fn stops_at_the_hole(part: &Body, side: usize, outward: DVec3, hole: f64, what: &str) {
    let edge = edge_between(part, TOP, side);

    let chamfered = part
        .chamfer(&edge, 50.0)
        .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

    reduced(&chamfered, 50.0, 3.0, what);
    let body = &chamfered.body;
    sound_drawn_within(
        body,
        6000.0 - hole * 20.0 - 10.0 * 3.0 * 3.0 / 2.0,
        bored_drawn_within(),
        what,
    );
    let chamfer = one_flat_face_more(part, body, what);
    faces_toward(body, chamfer, outward + DVec3::Z, what);
}

#[test]
fn a_chamfer_asked_more_than_the_top_gives_up_to_a_round_hole_takes_as_much_on_either_side_of_the_part()
 {
    let (left, right) = (5, 3);
    let round = PI * 2.0 * 2.0;
    for (what, centre, start, side, outward) in [
        (
            "the left edge, the hole drawn from its far side",
            DVec2::new(5.0, 5.0),
            DVec2::new(7.0, 5.0),
            left,
            -DVec3::X,
        ),
        (
            "the left edge, the hole drawn from its near side",
            DVec2::new(5.0, 5.0),
            DVec2::new(3.0, 5.0),
            left,
            -DVec3::X,
        ),
        (
            "the right edge, the hole drawn from its near side",
            DVec2::new(25.0, 5.0),
            DVec2::new(27.0, 5.0),
            right,
            DVec3::X,
        ),
        (
            "the right edge, the hole drawn from its back",
            DVec2::new(25.0, 5.0),
            DVec2::new(25.0, 7.0),
            right,
            DVec3::X,
        ),
    ] {
        stops_at_the_hole(&bored_through(centre, start), side, outward, round, what);
    }
}

#[test]
fn a_chamfer_asked_more_than_the_top_gives_up_to_an_elliptic_hole_takes_as_much_wherever_its_turn_starts()
 {
    let right = 3;
    let oval = PI * 2.0 * 1.5;
    for (what, start) in [
        ("the ellipse drawn from its near side", 2.0),
        ("the ellipse drawn from its far side", -2.0),
    ] {
        stops_at_the_hole(
            &bored_through_an_ellipse(DVec2::new(25.0, 5.0), start),
            right,
            DVec3::X,
            oval,
            &format!("the right edge, {what}"),
        );
    }
}

/// A slot 4 wide raised 20: a front 28 long on the X axis, a round end of
/// half a turn about `(28 + off, 2)` from the front's end, and a straight back
/// to the Y axis. Off by nought, the round leaves the front tangentially;
/// moved off, it leaves it at a corner of about `off / 2` radians, outwards
/// when `off` is positive and inwards when it is negative. Its top one, its
/// front two.
fn slot_off_tangent_by(off: f64) -> Body {
    let (end, centre) = (DVec2::new(28.0, 0.0), DVec2::new(28.0 + off, 2.0));
    raised(
        Contour {
            corners: vec![DVec2::ZERO, end, 2.0 * centre - end, DVec2::new(0.0, 4.0)],
            runs: vec![
                Run::Straight,
                Run::Round {
                    center: centre,
                    turn: PI,
                },
                Run::Straight,
                Run::Straight,
            ],
        },
        20.0,
    )
}

#[test]
fn a_chamfer_on_a_slot_s_front_whose_round_end_is_a_hair_off_tangent_is_declined_like_a_tangent_one()
 {
    for off in [0.0, 1e-10, -1e-10, 1e-8, -1e-8] {
        let slot = slot_off_tangent_by(off);
        let edge = edge_between(&slot, TOP, FRONT);

        let declined = slot.chamfer(&edge, 1.0).err();

        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "the round's centre {off} off tangent, well within what the sketch solves to: \
             the front runs on into the round, a tangent chain"
        );
    }
}

/// The slot's own matter: the quadrilateral from the front to the round's
/// chord, the half disc beyond the chord, raised 20.
fn slot_volume(off: f64) -> f64 {
    let radius_squared = 4.0 + off * off;
    ((56.0 + 2.0 * off) / 2.0 * 4.0 + PI * radius_squared / 2.0) * 20.0
}

/// The matter a chamfer of 1 on the slot's front takes, trimmed by the
/// slot's own walls: over every place of the slot's plan less than 1 back
/// from the front's line, a height of 1 − y. Up to 1 back, the slot runs from
/// the Y axis to its round, `x = c + √(r² − (y − 2)²)`; moved outwards, the
/// round also dips below the front's line, `2√(r² − (y − 2)²)` across. With
/// `t = y − 2`, `∫(1 − y)√(r² − t²)` is `G(t) = (r² − t²)^1.5 / 3 − F(t)`,
/// `F(t) = t/2·√(r² − t²) + r²/2·asin(t/r)`.
fn slot_cut_off(off: f64) -> f64 {
    let (centre, radius) = (28.0 + off, (4.0 + off * off).sqrt());
    let under = |t: f64| (radius * radius - t * t).max(0.0).sqrt();
    let g = |t: f64| {
        let f = t / 2.0 * under(t) + radius * radius / 2.0 * (t / radius).clamp(-1.0, 1.0).asin();
        under(t).powi(3) / 3.0 - f
    };
    let behind = centre / 2.0 + g(-1.0) - g(-2.0);
    let dipping = if off > 0.0 {
        2.0 * (g(-2.0) - g(-radius))
    } else {
        0.0
    };
    behind + dipping
}

#[test]
fn a_chamfer_on_a_slot_s_front_runs_on_over_a_round_meeting_it_at_a_few_degrees_trimmed_by_the_round_s_wall()
 {
    for off in [-0.1, 0.1] {
        let what = format!(
            "the round leaving the front's end {:.1} degrees off its line",
            (off / 2.0f64).atan().to_degrees()
        );
        let slot = slot_off_tangent_by(off);
        let edge = edge_between(&slot, TOP, FRONT);

        let chamfered = slot
            .chamfer(&edge, 1.0)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(&chamfered, 1.0, &what);
        let body = &chamfered.body;
        let cut_off = slot_cut_off(off);
        assert!(
            cut_off > 28.0 * 1.0 * 1.0 / 2.0,
            "{what}: the chamfer takes {cut_off}, more than the 14 of the edge's own prism"
        );
        sound_drawn_within(
            body,
            slot_volume(off) - cut_off,
            2e-4 * 31.0 * PI * 2.01 * 20.0,
            &what,
        );
        let chamfer = one_flat_face_more(&slot, body, &what);
        let (centre, radius) = (DVec2::new(28.0 + off, 2.0), (4.0 + off * off).sqrt());
        let plane = body.plane_of(chamfer).expect("a flat face offers a plane");
        for corner in &plane.corners {
            let on_the_round = (corner.truncate().distance(centre) - radius).abs() < PLACE;
            assert!(
                ((corner.z - corner.y) - 19.0).abs() < PLACE
                    && ((corner.z - 20.0).abs() < PLACE || corner.y.abs() < PLACE || on_the_round),
                "{what}: the chamfer has a corner at {corner}, off its plane or off the top, \
                 the front and the round that trim it"
            );
        }
    }
}

#[test]
fn a_chamfer_on_the_trapezoid_s_slanted_top_edge_is_trimmed_by_the_front_and_the_back_it_meets_at_its_ends()
 {
    let part = trapezoid(5.0);
    let edge = edge_between(&part, TOP, SLANT);

    let chamfered = part
        .chamfer(&edge, 1.0)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "1 asked on the top edge of the slant, which meets the back at 127 degrees";
    taken_whole(&chamfered, 1.0, what);
    let body = &chamfered.body;
    sound(
        body,
        (40.0 + 34.0) / 2.0 * 8.0 * 5.0 - 10.0 * 1.0 * 1.0 / 2.0,
        what,
    );
    let chamfer = one_flat_face_more(&part, body, what);
    faces_toward(body, chamfer, DVec3::new(0.8, 0.6, 1.0), what);
    corners_are(
        body,
        chamfer,
        &[
            DVec3::new(38.75, 0.0, 5.0),
            DVec3::new(32.75, 8.0, 5.0),
            DVec3::new(34.0, 8.0, 4.0),
            DVec3::new(40.0, 0.0, 4.0),
        ],
        "the chamfer, its line on the top stopping short of the front's end and running on \
         past the slant's end along the back",
    );
}

#[test]
fn a_chamfer_on_a_top_edge_of_a_hexagonal_prism_runs_on_past_both_its_ends_to_the_walls_beside_it()
{
    let half = 5.0 * 3.0f64.sqrt();
    let part = raised(
        Contour::straight(vec![
            DVec2::new(-5.0, -half),
            DVec2::new(5.0, -half),
            DVec2::new(10.0, 0.0),
            DVec2::new(5.0, half),
            DVec2::new(-5.0, half),
            DVec2::new(-10.0, 0.0),
        ]),
        5.0,
    );
    let edge = edge_between(&part, TOP, FRONT);

    let chamfered = part
        .chamfer(&edge, 1.0)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"));

    let what = "1 asked on the top edge of a hexagon's front 10 wide";
    taken_whole(&chamfered, 1.0, what);
    let body = &chamfered.body;
    let hexagon = 1.5 * 3.0f64.sqrt() * 10.0 * 10.0 * 5.0;
    let cut_off = 5.0 + 1.0 / (3.0 * 3.0f64.sqrt());
    sound(body, hexagon - cut_off, what);
    let chamfer = one_flat_face_more(&part, body, what);
    faces_toward(body, chamfer, DVec3::new(0.0, -1.0, 1.0), what);
    let past = 5.0 + 1.0 / 3.0f64.sqrt();
    corners_are(
        body,
        chamfer,
        &[
            DVec3::new(-past, 1.0 - half, 5.0),
            DVec3::new(past, 1.0 - half, 5.0),
            DVec3::new(5.0, -half, 4.0),
            DVec3::new(-5.0, -half, 4.0),
        ],
        "the chamfer, its line on the top longer than the edge, its line on the front the front's width",
    );
}
