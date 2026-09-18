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
mod tests;
