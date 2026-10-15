//! A face's domain in its surface's parameters: its edges seen there as
//! traces, and where a point of the surface stands against it.

mod crossing;
mod distance;
pub(super) mod floor;
mod inside;
mod traced;

use glam::{DVec2, DVec3};

use super::Declined;
use super::surface::Surface;
use super::topology::{Body, EdgeId, Face, FaceId, SurfaceId};
use super::trace::Trace;
use distance::Unrolling;
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
            self.scale.eps(),
        )
    }

    /// A face's loops seen in its surface's parameters, each trace run the
    /// way its loop runs. On a cylinder a trace may start a whole turn away
    /// from where the one before it ended.
    pub fn traces(&self, face: FaceId) -> Result<Vec<Vec<Trace>>, Declined> {
        self.traces_of(self.face(face))
    }

    /// The same, of a face not yet in the body.
    pub(in crate::brep) fn traces_of(&self, face: &Face) -> Result<Vec<Vec<Trace>>, Declined> {
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

    /// A face's loops as [`Body::traces`] gives them, closed along the floors
    /// of its surface where they reach a pole or go round one ([`floor`]): a
    /// domain to count parity in, never to measure a boundary on.
    pub(in crate::brep) fn closed_traces(&self, face: FaceId) -> Result<Vec<Vec<Trace>>, Declined> {
        Ok(self.closed(face, self.traces(face)?))
    }

    fn closed(&self, face: FaceId, loops: Vec<Vec<Trace>>) -> Vec<Vec<Trace>> {
        let surface = self.surface(self.face(face).surface);
        match surface {
            Surface::Cone(cone) => floor::closed(cone, loops, self.scale.eps()),
            Surface::Sphere(_) | Surface::Torus(_) => {
                floor::closed_round(&floor::poles_of(surface, self.scale.eps()), loops)
            }
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Elliptic(_) => loops,
        }
    }

    /// Where `at`, in the parameters of the face's surface, stands against
    /// the face: on its boundary within `eps` of it — of its edges, or of a
    /// pole it holds within it, a vertex of it — and otherwise inside or
    /// outside as the number of loops a ray up the second parameter crosses
    /// is odd or even, its domain closed along the floors.
    pub fn locate(&self, face: FaceId, at: DVec2, eps: f64) -> Result<Location, Declined> {
        let loops = self.traces(face)?;
        let unrolling = self.unrolled_at(face);
        let place = self.surface(self.face(face).surface).point(at);
        let near = loops.iter().flatten().any(|trace| {
            !self.far_from(trace, place, eps) && distance::distance(trace, at, unrolling) <= eps
        }) || self
            .poles_held(face)
            .iter()
            .any(|pole| pole.distance(place) <= eps);
        if near {
            return Ok(Location::Boundary);
        }
        let above = crossing::heights(&self.closed(face, loops), at.x, unrolling.periodic())
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
    /// face and well away from its boundary, its apex included; none for a
    /// face with no inside.
    pub fn point_inside(&self, face: FaceId) -> Result<Option<DVec2>, Declined> {
        Ok(inside::point_inside(
            &self.closed_traces(face)?,
            self.unrolled_at(face),
        ))
    }

    /// Whether `place` stands further than `eps` from the curve two cylinders
    /// meet along, or the conic, that a trace follows, told from the box
    /// round it before the distance is sought along it: on a surface a
    /// distance is never shorter than straight across, and the box holds
    /// every point of the curve. Other traces are measured in closed form at
    /// once.
    fn far_from(&self, trace: &Trace, place: DVec3, eps: f64) -> bool {
        let [low, high] = match *trace {
            Trace::Graph { meet, from, to, .. } => meet.bounds(from, to),
            Trace::Conic {
                conic, from, to, ..
            } => conic.bounds(from, to),
            Trace::Segment { .. } | Trace::Round { .. } => return false,
        };
        let room = eps + self.scale.eps();
        place.cmplt(low - room).any() || place.cmpgt(high + room).any()
    }

    /// How a face's parameters unroll into lengths.
    fn unrolled_at(&self, face: FaceId) -> Unrolling {
        match self.surface(self.face(face).surface) {
            Surface::Cylinder(cylinder) => Unrolling::Round(cylinder.radius),
            Surface::Cone(cone) => Unrolling::Cone(*cone),
            Surface::Elliptic(wall) => Unrolling::Elliptic(*wall),
            Surface::Sphere(sphere) => Unrolling::Sphere(*sphere),
            Surface::Torus(torus) => Unrolling::Torus(*torus),
            Surface::Plane(_) => Unrolling::Flat,
        }
    }
}

#[cfg(test)]
mod tests;
