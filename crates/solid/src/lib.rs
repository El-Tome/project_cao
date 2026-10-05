//! The volumes: a sketch area raised into a prism or turned about an axis, and
//! the boolean operations that add or take away matter, computed by the exact
//! kernel on planes and cylinders or by flat pieces where it cannot. No
//! rendering and no interface, like `cao_sketch`.
//!
//! What leaves the crate is a [`Body`] and what it answers, its faces named by
//! their numbers, and the [`Declined`] reason when the exact kernel gives no
//! answer. The kernels stay inside: the rest of the workspace has only the
//! body to keep (#499, #526).

mod body;
mod boolean;
pub mod brep;
mod clipping;
mod mesh;
pub mod profile;
#[cfg(any(test, feature = "test-support"))]
pub mod soundness;
mod sweep;
pub mod turning;

pub use body::{Body, FaceHit, FacePlane};
pub use brep::Declined;
pub use sweep::Loop;
