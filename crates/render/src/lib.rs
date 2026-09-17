//! GPU rendering of the 3D viewport: world axes, adaptive grid and the
//! orientation cube. Depends on `wgpu` and `glam` only — no UI framework — so
//! the same renderer can back a future tablet/web front-end.

pub mod camera;
pub mod cube;
pub mod geometry;
pub mod grid;
pub mod offscreen;
mod renderer;

pub use camera::{CubeFace, CubeZone, OrbitCamera, ViewTransition};
pub use geometry::{
    AxisStyle, BackgroundShape, Vertex, push_axes, push_background, push_plane_outline,
    push_plane_quad, push_solid, srgb,
};
pub use grid::{GridStyle, adaptive_step, push_grid};
pub use offscreen::{Size, draw};
pub use renderer::{SceneFrame, SceneRenderer, ViewportRect};
