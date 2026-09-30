//! The curve where two perpendicular cylinders meet: exact, as four arcs over
//! the height across both axes.

use glam::{DVec2, DVec3};

use super::curve::Meet;

impl Meet {
    pub fn point(&self, _t: f64) -> DVec3 {
        unimplemented!()
    }

    pub fn derivative(&self, _t: f64) -> DVec3 {
        unimplemented!()
    }

    pub fn parameter(&self, _point: DVec3) -> f64 {
        unimplemented!()
    }

    pub fn period(&self) -> Option<f64> {
        unimplemented!()
    }

    /// The point at `t` in the parameters `(θ, h)` of the first cylinder or of
    /// the second, with its first and second derivatives with respect to `t`.
    pub fn seen_on(&self, _on_first: bool, _t: f64) -> [DVec2; 3] {
        unimplemented!()
    }
}
