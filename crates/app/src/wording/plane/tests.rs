//! What app · wording/plane.rs is held to.

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
