//! The canvas: camera navigation, the state a session of sketching keeps
//! between frames, and the frame loop that ties input to drawing together.
//!
//! What a click does with a tool lives in [`input`]; what ends up painted
//! lives in [`render`]. What the canvas knows is in [`state`], what it draws
//! is in [`view`].

mod cube_labels;
pub(crate) use cube_labels::repaint_if_the_atlas_grew;
mod finish;
mod input;
mod kept_sections;
mod visibility;
pub(crate) use input::copying::take_up as take_up_the_copy;
pub(crate) use input::{chamfer_to_swap, corner_picks_with, swap_held_chamfer, switch_held_cuts};
pub(crate) use input::{take as take_a_selection, take_everything};
mod matter;
mod navigation;
mod opening_view;
// Until the panel draws the step reader (#356): `expect` fails the day it does.
#[cfg_attr(not(test), expect(dead_code))]
mod reading;
mod render;
mod section;
mod state;
mod values;
mod view;

pub use state::DEFAULT_SKETCH_RADIUS;
pub use state::{ViewMode, ViewportState};
pub use view::{Shown, show};

pub(crate) use state::{
    HOLD_PIXELS, PICK_PIXELS, ViewScale, corner_origin, cube_rect, plane_half_size, to_ndc,
};
