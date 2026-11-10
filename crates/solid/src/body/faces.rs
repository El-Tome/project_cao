//! The body face by face: a ray that goes on through the faces it is told to
//! leave out, and a tool numbered as the part it is joined to or cut from
//! would number its faces.

use glam::DVec3;

use super::{Body, FaceHit, Flats, Matter};
use crate::brep::Declined;
use crate::mesh::Mesh;

impl Body {
    /// The face a ray meets first among those `kept` lets through, the ray
    /// going on through the others as if they were not there. A face that
    /// answers to several numbers is met when one of them is let through.
    pub fn ray_hit_among(
        &self,
        origin: DVec3,
        direction: DVec3,
        kept: &dyn Fn(usize) -> bool,
    ) -> Option<FaceHit> {
        match &self.matter {
            Matter::Exact(exact) => exact.ray_hit(origin, direction, kept),
            Matter::Flats(flats) => {
                let hit = flats.mesh.ray_hit_among(origin, direction, kept)?;
                Some(FaceHit {
                    distance: hit.distance,
                    face: hit.polygon.face,
                    normal: hit.polygon.normal(),
                })
            }
        }
    }

    /// This tool with its faces numbered as joining it to `part` or cutting
    /// it from `part` would number them, and nothing of `part`: a tool shown
    /// apart from the part then answers to the numbers its faces have there.
    pub fn numbered_after(&self, part: &Body) -> Result<Body, Declined> {
        let emptied = match &part.matter {
            Matter::Exact(exact) => Matter::Exact(Box::new(exact.emptied())),
            Matter::Flats(flats) => Matter::Flats(Flats {
                mesh: Mesh::default(),
                next: flats.next,
            }),
        };
        Body { matter: emptied }.union(self)
    }
}
