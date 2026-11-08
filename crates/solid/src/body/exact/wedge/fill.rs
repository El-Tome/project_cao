//! The join a fill makes (#778): the prism a concave edge is filled with
//! joined to the body, its ends merged into the faces at the edge's ends.

use crate::body::exact::{Exact, PIECES};
use crate::brep::Declined;

impl Exact {
    /// The body joined with the prism `tool` fills a corner with, every face
    /// of the prism that merged into one of the body's — its ends, flush with
    /// the faces at the edge's ends — answering to the body's numbers alone,
    /// so that a cap or a wall the fill reaches stays the face it was. The
    /// prism's other faces keep their numbers, past the steps' count, for
    /// [`Exact::cut_along`] to find.
    pub(super) fn filled_by(&self, tool: &Exact) -> Result<Exact, Declined> {
        let Exact {
            mut brep,
            next,
            pieces,
            roots,
            drawn,
            ..
        } = self.joined(tool)?;
        let brought = |number: &u32| (next..PIECES).contains(number);
        let faces: Vec<_> = brep.face_ids().collect();
        for face in faces {
            let numbers = brep.numbers(face).to_vec();
            let Some(&own) = numbers.iter().find(|number| !brought(number)) else {
                continue;
            };
            for &number in numbers.iter().filter(|number| brought(number)) {
                brep.rename(&[face], number, own);
            }
        }
        let filled = Exact::of(brep, next, pieces, roots);
        if let Some(drawn) = drawn.into_inner() {
            let _ = filled.drawn.set(drawn);
        }
        Ok(filled)
    }
}
