//! What the installation remembers between two parts: themes, shortcuts, the
//! toolbar layout, profiles, recent files.
//!
//! Nothing here knows the geometry, or the part. `Command` lives here because
//! it is the vocabulary of intent — what the toolbar and the keyboard bind to a
//! gesture — and not of what a `.caopart` records.

mod adapters;
pub mod command;
pub mod config;
pub mod ports;
mod recents;
pub mod settings;
pub mod shortcuts;
mod storage;
pub mod theme;
pub mod toolbar;

#[cfg(any(test, feature = "test-support"))]
pub use adapters::InMemoryFiles;
pub use command::{Command, CommandFamily};
pub use config::ViewportConfig;
pub use ports::{FileError, Files};
pub use recents::{MAX_RECENTS, RecentEntry, RecentList};
pub use settings::{PROFILE_EXTENSION, Profile, Profiles, Settings};
pub use shortcuts::{Chord, Key, Shortcuts};
pub use storage::{StorageError, crash_log_path, default_projects_dir, record_panics};
pub use theme::{Background, Rgba, Stop, Theme};
pub use toolbar::{Edge, Item, Path, ToolbarLayout};
