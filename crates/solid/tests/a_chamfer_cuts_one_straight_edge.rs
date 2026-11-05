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
//!   `a_chamfer_asked_more_than_the_top_can_give_takes_the_whole_top_and_reports_both_values`
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
use cao_solid::{Body, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it: every face
/// here is a plane, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// The numbers a raised block's faces answer to.
const TOP: usize = 1;
const FRONT: usize = 2;

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// A block of a length, a breadth and a height standing on the ground's
/// origin, raised and joined as `cao_part` builds a step: its floor numbered
/// nought, its top one, and its sides two to five from the front round to the
/// left.
fn block([length, breadth, height]: [f64; 3]) -> Body {
    let profile = Profile {
        exact: Some((
            Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
            Vec::new(),
        )),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let part = Body::default();
    let tool = part
        .tool_raised(&profile, ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the block is built")
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
fn sound(body: &Body, promised: f64) {
    assert!(body.is_exact(), "the exact kernel computed the body");
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()));
    assert_eq!(uncrossed(&triangles), Ok(()));
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );
    let drawn = enclosed(&triangles);
    assert!(
        (drawn - promised).abs() <= EXACT * promised,
        "its triangles enclose {drawn}, where arithmetic promised {promised}"
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

    assert!(
        (chamfered.asked - distance).abs() < PLACE && (chamfered.taken - distance).abs() < PLACE,
        "asked {} and took {}, where {distance} fits",
        chamfered.asked,
        chamfered.taken
    );
    let body = &chamfered.body;
    sound(
        body,
        length * breadth * height - length * distance * distance / 2.0,
    );
    let (before, after) = (numbers(&block), numbers(body));
    assert!(
        before.is_subset(&after),
        "a face went: {before:?} became {after:?}"
    );
    let made: Vec<usize> = after.difference(&before).copied().collect();
    assert_eq!(made.len(), 1, "one face more: {before:?} became {after:?}");
    let chamfer = made[0];
    assert!(body.is_flat(chamfer), "the face the chamfer lays is flat");
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
