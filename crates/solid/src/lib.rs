//! The volumes: a polygon mesh, the extrusion of a sketch area into a prism,
//! and the boolean operations that add or take away matter. No rendering and
//! no interface, like `cao_sketch`.

mod body;
mod boolean;
mod clipping;
mod mesh;
#[cfg(any(test, feature = "test-support"))]
pub mod soundness;
mod sweep;

pub use body::{Body, FaceHit, FacePlane};
pub use mesh::{Mesh, Polygon};
pub use sweep::{Loop, prism, revolution};
