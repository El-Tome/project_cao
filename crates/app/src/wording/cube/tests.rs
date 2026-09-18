//! What app · wording/cube.rs is held to.

use std::collections::BTreeSet;

use super::*;

fn said(which: CubeFace) -> String {
    face(&Catalogue::french(), which)
}

#[test]
fn each_face_of_the_cube_reads_differently() {
    let read: BTreeSet<String> = CubeFace::ALL.iter().map(|face| said(*face)).collect();

    assert_eq!(
        read.len(),
        CubeFace::ALL.len(),
        "two faces read the same: {read:?}"
    );
}

#[test]
fn a_face_is_named_from_where_the_part_is_looked_at() {
    assert_eq!(said(CubeFace::MinusY), "FACE");
    assert_eq!(said(CubeFace::PlusY), "ARRIÈRE");
    assert_eq!(said(CubeFace::PlusX), "DROITE");
    assert_eq!(said(CubeFace::MinusX), "GAUCHE");
    assert_eq!(said(CubeFace::PlusZ), "DESSUS");
    assert_eq!(said(CubeFace::MinusZ), "DESSOUS");
}
