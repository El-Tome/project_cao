//! The first row of the part screen: the way home, the part's name, where the
//! view stands, and the message.

mod state;
mod view;

pub use state::{Covering, TitleBar, settle};
pub use view::show;
