//! What app · wording/plane.rs is held to.
//!
//! Closes #733 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`.
//! - found in review: a slanted plane through the origin reads « incliné » on
//!   a sketch's row — `a_plane_through_the_origin_square_to_no_axis_reads_slanted_in_short`

use std::collections::BTreeSet;

use super::*;

const EVERY_KIND: [PlaneKind; 5] = [
    PlaneKind::OffOrigin,
    PlaneKind::OriginXY,
    PlaneKind::OriginXZ,
    PlaneKind::OriginYZ,
    PlaneKind::OriginSlanted,
];

fn said(kind: PlaneKind) -> String {
    label(&Catalogue::french(), kind)
}

#[test]
fn each_plane_the_sketch_recognises_reads_differently() {
    let read: BTreeSet<String> = EVERY_KIND.iter().map(|kind| said(*kind)).collect();

    assert_eq!(
        read.len(),
        EVERY_KIND.len(),
        "two planes read the same: {read:?}"
    );
}

#[test]
fn a_plane_through_the_origin_is_named_after_its_axes_and_a_face_is_not() {
    assert_eq!(said(PlaneKind::OriginXY), "Plan XY");
    assert_eq!(said(PlaneKind::OriginXZ), "Plan XZ");
    assert_eq!(said(PlaneKind::OriginYZ), "Plan YZ");
    assert_eq!(said(PlaneKind::OriginSlanted), "Plan d'esquisse");
    assert_eq!(said(PlaneKind::OffOrigin), "Face de la pièce");
}

#[test]
fn a_plane_through_the_origin_square_to_no_axis_reads_slanted_in_short() {
    assert_eq!(
        short(&Catalogue::french(), PlaneKind::OriginSlanted),
        "incliné",
        "a slanted plane through the origin does not read « incliné » on a row of the tree",
    );
}
