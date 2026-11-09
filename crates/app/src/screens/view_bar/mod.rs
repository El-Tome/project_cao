//! The bar at the bottom right of the view that gathers the view's own tools:
//! « Recadrer », « Vue en coupe », « Matière devant » and the cursor's
//! coordinates.

mod state;
mod view;

pub use state::{Offered, ViewBar};
pub(crate) use view::offered_button;
pub use view::show;
