//! The first row of the part screen: the way home, the part's name, where the
//! view stands, the message, and the count of the part's alerts, which opens
//! their list.

mod state;
mod view;

pub use state::{Held, TitleBar, settle};
pub use view::show;
