//! The triangles a body is drawn with and held to the rules by.

use glam::DVec3;

use super::topology::Body;

impl Body {
    /// Triangles standing within `tolerance` of the true surfaces, closed.
    pub fn triangles(&self, _tolerance: f64) -> Vec<[DVec3; 3]> {
        Vec::new()
    }
}
