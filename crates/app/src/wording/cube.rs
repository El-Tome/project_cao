use cao_render::CubeFace;

use crate::lang::Catalogue;

/// The only place a face of the orientation cube is turned into a name.
///
/// `cao_render` knows the face as a direction; which side of the part a user
/// calls that is the interface's decision, and it turns on which way round the
/// part is modelled.
pub fn face(lang: &Catalogue, face: CubeFace) -> String {
    lang.t(match face {
        CubeFace::PlusX => "cube.right",
        CubeFace::MinusX => "cube.left",
        CubeFace::PlusY => "cube.back",
        CubeFace::MinusY => "cube.front",
        CubeFace::PlusZ => "cube.top",
        CubeFace::MinusZ => "cube.bottom",
    })
}

#[cfg(test)]
mod tests {
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
}
