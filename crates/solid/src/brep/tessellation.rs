//! The triangles a body is drawn with and held to the rules by.
//!
//! Every edge is sampled once ([`sampling`]) and every face cut in its
//! surface's parameters from those very samples ([`outline`], [`sweep`]), so
//! two faces along an edge stand on the same points and the triangles close
//! by construction. No point is added inside a face: on a plane none is
//! needed, and on a cylinder every curve has a sample at every angle of the
//! grid, so that no triangle spans more than one step of it — but beside a
//! line where two walls touch, whose nearest steps [`contact`] withholds.

mod contact;
mod orientation;
mod outline;
mod sampling;
mod sweep;

use glam::DVec3;

use super::topology::Body;
use outline::Outline;
use sampling::Samples;

impl Body {
    /// Triangles standing within `tolerance` of the true surfaces, closed,
    /// facing out of the matter. A face that cannot be laid out or cut is
    /// left open rather than drawn wrong: the rules on the triangles see it.
    pub fn triangles(&self, tolerance: f64) -> Vec<[DVec3; 3]> {
        let samples = Samples::of(self, tolerance);
        let mut triangles = Vec::new();
        for id in self.face_ids() {
            let Some(outline) = Outline::of(self, &samples, id, tolerance) else {
                continue;
            };
            let Some(cut) = sweep::triangles(&outline.points, &outline.segments) else {
                continue;
            };
            for corners in cut {
                let [a, b, c] = corners.map(|corner| outline.samples[corner]);
                if a == b || b == c || c == a {
                    continue;
                }
                let [a, b, c] = [a, b, c].map(|sample| samples.point(sample));
                triangles.push(if self.face(id).flipped {
                    [a, c, b]
                } else {
                    [a, b, c]
                });
            }
        }
        triangles
    }
}

#[cfg(test)]
pub(crate) mod tests;
