//! A face's domain in its surface's parameters: its edges seen there as
//! traces, and where a point of the surface stands against it.

mod crossing;
mod distance;
mod inside;
mod traced;

use glam::DVec2;

use super::Declined;
use super::surface::Surface;
use super::topology::{Body, EdgeId, FaceId, SurfaceId};
use super::trace::Trace;
pub(super) use traced::traced;

/// Where a point of a surface stands against a face lying on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Location {
    Inside,
    Outside,
    /// Within the tolerance asked of the face's boundary, measured on the
    /// surface.
    Boundary,
}

impl Body {
    /// An edge seen in the parameters of a surface it lies on, run the
    /// edge's own way.
    pub fn trace(&self, edge: EdgeId, surface: SurfaceId) -> Result<Trace, Declined> {
        let stretch = self.edge(edge);
        traced(
            self.curve(stretch.curve),
            self.surface(surface),
            stretch.from,
            stretch.to,
        )
    }

    /// A face's loops seen in its surface's parameters, each trace run the
    /// way its loop runs. On a cylinder a trace may start a whole turn away
    /// from where the one before it ended.
    pub fn traces(&self, face: FaceId) -> Result<Vec<Vec<Trace>>, Declined> {
        let face = self.face(face);
        face.loops
            .iter()
            .map(|lap| {
                lap.iter()
                    .map(|coedge| {
                        let trace = self.trace(coedge.edge, face.surface)?;
                        Ok(if coedge.forward {
                            trace
                        } else {
                            trace.reversed()
                        })
                    })
                    .collect()
            })
            .collect()
    }

    /// Where `at`, in the parameters of the face's surface, stands against
    /// the face: on its boundary within `eps` of it, and otherwise inside or
    /// outside as the number of loops a ray up the second parameter crosses
    /// is odd or even.
    pub fn locate(&self, face: FaceId, at: DVec2, eps: f64) -> Result<Location, Declined> {
        let loops = self.traces(face)?;
        let radius = self.unrolled_at(face);
        let near = loops
            .iter()
            .flatten()
            .any(|trace| distance::distance(trace, at, radius) <= eps);
        if near {
            return Ok(Location::Boundary);
        }
        let above = crossing::heights(&loops, at.x, radius.is_some())
            .into_iter()
            .filter(|height| *height > at.y)
            .count();
        Ok(if above % 2 == 1 {
            Location::Inside
        } else {
            Location::Outside
        })
    }

    /// A point of the face's surface, in its parameters, strictly inside the
    /// face and well away from its boundary; none for a face with no inside.
    pub fn point_inside(&self, face: FaceId) -> Result<Option<DVec2>, Declined> {
        Ok(inside::point_inside(
            &self.traces(face)?,
            self.unrolled_at(face),
        ))
    }

    /// The radius a face's parameters unroll at: its cylinder's, and none on
    /// a plane, whose parameters are lengths already.
    fn unrolled_at(&self, face: FaceId) -> Option<f64> {
        match self.surface(self.face(face).surface) {
            Surface::Cylinder(cylinder) => Some(cylinder.radius),
            Surface::Plane(_) => None,
        }
    }
}

#[cfg(test)]
mod tests;
