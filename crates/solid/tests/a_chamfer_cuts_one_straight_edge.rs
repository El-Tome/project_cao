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
//!   `a_chamfer_on_a_block_s_top_front_edge_lays_one_flat_face_set_back_by_the_distance_along_the_top_and_the_front`
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

use std::collections::BTreeSet;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Chamfered, Declined, EdgeName, Loop, Lost};
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
    let drawn = enclosed(&triangles);
    assert!(
        (drawn - promised).abs() <= EXACT * promised,
        "{what}: its triangles enclose {drawn}, where arithmetic promised {promised}"
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

/// The number an L's stem's back answers to.
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
