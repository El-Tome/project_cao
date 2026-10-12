//! The triangles a body is drawn with and held to the rules by.
//!
//! Every edge is sampled once ([`sampling`]) and every face cut in its
//! surface's parameters from those very samples ([`outline`], [`sweep`]), so
//! two faces along an edge stand on the same points and the triangles close
//! by construction. No point is added inside a face: on a plane none is
//! needed, and on a cylinder every curve has a sample at every angle of the
//! grid, so that no triangle spans more than one step of it — but beside a
//! line where two walls touch, whose nearest steps [`contact`] withholds. So
//! too on a cone, whose rims share its grid, and whose apex stands for every
//! angle: a face reaching it is a fan from it. Each face is cut alone, so two
//! faces standing within the tolerance of each other can fold onto each other
//! across their edge; [`folds`] cuts them the other way once all are cut.

mod contact;
mod folds;
mod orientation;
mod outline;
mod sampling;
mod skins;
mod sweep;

use glam::DVec3;

use super::topology::{Body, FaceId};
use folds::Drawn;
use outline::Outline;
use sampling::Samples;

impl Body {
    /// Triangles standing within `tolerance` of the true surfaces, closed,
    /// facing out of the matter. A face that cannot be laid out or cut is
    /// left open rather than drawn wrong: the rules on the triangles see it.
    pub fn triangles(&self, tolerance: f64) -> Vec<[DVec3; 3]> {
        self.triangles_by_face(tolerance).triangles
    }

    /// The same triangles, and beside them the face each one belongs to. A
    /// face is never drawn alone: its edges are sampled with every face
    /// along them, so that the triangles of two faces close against each
    /// other.
    pub fn triangles_by_face(&self, tolerance: f64) -> Cut {
        let samples = Samples::of(self, tolerance);
        let mut drawn = Vec::new();
        let mut uncut = Vec::new();
        for id in self.face_ids() {
            let Some(outline) = Outline::of(self, &samples, id, tolerance) else {
                uncut.push(id);
                continue;
            };
            let Some(cut) = sweep::triangles(&outline.points, &outline.segments) else {
                uncut.push(id);
                continue;
            };
            drawn.push(Drawn {
                face: id,
                outline,
                triangles: cut,
            });
        }
        folds::unfolded(&mut drawn, &samples, self.scale().eps());
        let mut oriented = Vec::new();
        for Drawn {
            face: id,
            outline,
            triangles: cut,
        } in drawn
        {
            for corners in cut {
                let [a, b, c] = corners.map(|corner| outline.samples[corner]);
                if a == b || b == c || c == a {
                    continue;
                }
                oriented.push((
                    id,
                    if self.face(id).flipped {
                        [a, c, b]
                    } else {
                        [a, b, c]
                    },
                ));
            }
        }
        skins::cancelled(&mut oriented);
        let faces = oriented.iter().map(|(id, _)| *id).collect();
        let triangles = oriented
            .into_iter()
            .map(|(_, corners)| corners.map(|sample| samples.point(sample)))
            .collect();
        Cut {
            triangles,
            faces,
            uncut,
        }
    }
}

/// The triangles of a body, the face each belongs to, and the faces that
/// could not be laid out or cut, left open.
///
/// A face cut into nothing is not among them: one thinner than the
/// tolerance — a sliver a hair wide between a bore and a wall — has its
/// triangles collapse onto its edges, and the faces either side of it close
/// against each other across it.
pub struct Cut {
    pub triangles: Vec<[DVec3; 3]>,
    pub faces: Vec<FaceId>,
    pub uncut: Vec<FaceId>,
}

#[cfg(test)]
pub(crate) mod tests;
