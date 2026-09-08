//! The volumes: a polygon mesh, the extrusion of a sketch area into a prism,
//! and the boolean operations that add or take away matter. No rendering and
//! no interface, like `cao_sketch`.

mod boolean;
mod mesh;

pub use mesh::{FaceHit, Mesh, Polygon, prism, revolution};
