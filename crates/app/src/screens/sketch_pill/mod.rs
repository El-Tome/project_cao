//! The pill a sketch shows at the top of the view while it is open: its name,
//! where it is drawn, what the solver leaves it to set, and the way to finish
//! it.

mod state;
mod view;

pub use state::{SketchPill, laid_on};
pub use view::show;
