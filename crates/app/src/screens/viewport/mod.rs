//! The canvas: camera navigation, the state a session of sketching keeps
//! between frames, and the frame loop that ties input to drawing together.
//!
//! What a click does with a tool lives in [`input`]; what ends up painted
//! lives in [`render`]. What the canvas knows is in [`state`], what it draws
//! is in [`view`].

mod cube_labels;
mod finish;
mod input;
pub(crate) use input::corner_picks_with;
mod matter;
mod navigation;
mod render;
mod state;
mod values;
mod view;

pub use input::DEFAULT_SKETCH_RADIUS;
pub use state::{ViewMode, ViewportState};
pub use view::show;

pub(crate) use render::a_field_at_the_cursor_holds_the_keyboard;
pub(crate) use state::{PICK_PIXELS, ViewScale, corner_origin, plane_half_size, to_ndc};
