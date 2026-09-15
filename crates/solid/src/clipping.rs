use glam::DVec3;

use crate::boolean::{Plane, Split};
use crate::mesh::{Mesh, Polygon};

impl Mesh {
    /// What is left of the solid behind a plane: everything on the far side of
    /// it, with the faces it crosses cut off where they meet it.
    ///
    /// The surface is left open where the plane went through. This is a way of
    /// looking rather than a way of taking matter away, and nothing is put back
    /// to close it.
    pub fn behind(&self, normal: DVec3, offset: f64) -> Mesh {
        let plane = Plane { normal, offset };
        let mut kept: Vec<Polygon> = Vec::new();
        for polygon in &self.polygons {
            let mut cut = Split::default();
            plane.split(polygon, &mut cut);
            kept.extend(cut.back);
            // A face lying in the plane itself is neither in front nor behind,
            // and it is the very face a sketch started on a face is drawn on.
            kept.extend(cut.coplanar_back);
            kept.extend(cut.coplanar_front);
        }
        Mesh { polygons: kept }
    }
}

#[cfg(test)]
mod tests;
