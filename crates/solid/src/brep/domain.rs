//! A face's domain in its surface's parameters: its edges seen there as
//! traces, and where a point of the surface stands against it.

mod traced;

use super::Declined;
use super::topology::{Body, EdgeId, FaceId, SurfaceId};
use super::trace::Trace;
pub(super) use traced::traced;

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
}

#[cfg(test)]
mod tests;
