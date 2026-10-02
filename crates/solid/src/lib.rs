//! The volumes: a polygon mesh, the extrusion of a sketch area into a prism,
//! and the boolean operations that add or take away matter. No rendering and
//! no interface, like `cao_sketch`.
//!
//! What leaves the crate is a [`Body`] and what it answers, its faces named by
//! their numbers. The mesh and its polygons stay inside: whatever computes the
//! matter next has only the body to keep (#499).

mod body;
mod boolean;
mod clipping;
mod mesh;
#[cfg(any(test, feature = "test-support"))]
pub mod soundness;
mod sweep;

pub use body::{Body, FaceHit, FacePlane};
pub use sweep::Loop;
