//! Two triangles in miniature, laid by hand at the very bound a rule turns
//! on, and what the one definition of a crossing reads of them.
//!
//! The changes a mutation run makes here that alter no answer, and why:
//! - `>` into `>=` on either height in `meets`' sign test: both corners stand
//!   further than the room from the plane there, so neither height is zero.
//! - `clear` into `false`: a face clear of the other's plane meets none of
//!   it, so `measured` finds nothing, and no corner of it rests on that plane,
//!   so `Lines::weigh` notes nothing; the test only spares the work.
//! - `+=` into `-=` on the area in `Facet::overlaps`: it negates the sum,
//!   rounding included, and only its size is read.
//! - `||` into `&&` in `Found::weigh`: the same filter as `clear` above.
//! - `Face::span` into a constant: every pair then comes into the sweep, and
//!   one whose spans along a direction lie further apart than the room shares
//!   no point to cross or lie along; which is weighed first changes no entry
//!   `Lines::passed` keeps, each being of one pair only.
//! - `||` into `&&` in `Folds::crossings`, either one: a triangle with two
//!   corners on one sample has no area, and `crossing` refuses it as a sliver.
//!
//! Closes #771.
//! - a cargo-mutants run on `brep/crossing.rs`, `brep/crossing/lines.rs`,
//!   `soundness/crossing.rs` and the crossing part of
//!   `brep/tessellation/folds.rs` reports no missed mutant, other than those
//!   named as equivalent above — no test: the run itself, reported on the pull
//!   request
//! - no production line changes — no test: read off `git diff --stat`, where
//!   `brep/crossing.rs` gains only the `#[cfg(test)]` declaring this file

use super::*;

fn at(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(x, y, z)
}

/// A face lying in z = 0, its right corner at the origin.
fn flat() -> Facet {
    Facet::of(&[DVec3::ZERO, at(4.0, 0.0, 0.0), at(0.0, 4.0, 0.0)])
}

#[test]
fn a_sliver_threaded_through_a_face_crosses_nothing_whichever_is_weighed_first() {
    let room = 0.01;
    let needle = Facet::of(&[
        at(0.0, 1.0, -2.0 * room),
        at(4.0, 1.0, 2.0 * room),
        at(3.0, 1.0, 1.5 * room),
    ]);
    assert!(
        sliver(&needle.corners, room),
        "its corners lie within the room of one line"
    );
    assert!(!crossing(&flat(), &needle, room));
    assert!(!crossing(&needle, &flat(), room));
}

#[test]
fn a_triangle_cutting_a_face_s_corner_off_a_hair_from_the_tip_crosses_it_whichever_is_weighed_first()
 {
    let room = 0.01;
    let cut = 1.2 * room * std::f64::consts::SQRT_2;
    let half = cut / 2.0;
    let cutting = Facet::of(&[
        at(half + 1.0, half - 1.0, -1.0),
        at(half - 1.0, half + 1.0, -1.0),
        at(half, half, 1.0),
    ]);
    assert!(
        crossing(&flat(), &cutting, room),
        "the corner stands more than the room beyond the cut, and the face's piece of the cut lies deep inside the other",
    );
    assert!(crossing(&cutting, &flat(), room));
}

#[test]
fn a_corner_sunk_into_a_face_by_exactly_the_room_rests_on_it() {
    let room = 2f64.powi(-10);
    let resting = Facet::of(&[at(1.0, 1.0, -room), at(2.0, 1.0, 1.0), at(1.0, 2.0, 1.0)]);
    assert!(!crossing(&flat(), &resting, room));
    assert!(!crossing(&resting, &flat(), room));
}

#[test]
fn an_edge_crossed_with_one_twice_as_long_along_the_same_line_spans_nothing() {
    let edge = at(0.1, 0.7, 1.3);
    assert_eq!(cross(edge, 2.0 * edge), DVec3::ZERO);
}

#[test]
fn a_segment_running_right_across_a_face_keeps_the_chord_more_than_the_room_inside_it() {
    let room = 1e-3;
    let piece = flat()
        .inside([at(-5.0, 1.0, 0.0), at(5.0, 1.0, 0.0)], room)
        .expect("the chord inside is three long");
    let ends = [
        at(room, 1.0, 0.0),
        at(3.0 - room * std::f64::consts::SQRT_2, 1.0, 0.0),
    ];
    for (got, wanted) in piece.iter().zip(ends) {
        assert!(
            got.distance(wanted) < 1e-12,
            "{piece:?} runs from {:?} to {:?}",
            ends[0],
            ends[1]
        );
    }
}

#[test]
fn a_piece_of_segment_inside_a_face_counts_only_when_longer_than_the_room() {
    let room = 0.25;
    let start = at(1.0, 1.0, 0.0);
    assert_eq!(flat().inside([start, at(1.25, 1.0, 0.0)], room), None);
    assert!(flat().inside([start, at(1.5, 1.0, 0.0)], room).is_some());
}
