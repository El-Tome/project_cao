//! What the installation remembers between two parts: themes, shortcuts, the
//! toolbar layout, profiles, recent files.
//!
//! Nothing here knows the geometry, or the part. `Command` lives here because
//! it is the vocabulary of intent — what the toolbar and the keyboard bind to a
//! gesture — and not of what a `.caopart` records.

mod adapters;
pub mod command;
pub mod config;
pub mod locations;
pub mod ports;
pub mod profiles;
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
pub use locations::{Locations, default_projects_dir};
pub use ports::{FileError, Files};
pub use profiles::Profiles;
pub use recents::{MAX_RECENTS, RecentEntry, RecentList};
pub use settings::{DEFAULT_PROFILE, PROFILE_EXTENSION, Profile, Settings};
pub use shortcuts::{Chord, Key, Shortcuts};
pub use storage::StorageError;
pub use theme::{Background, Rgba, Stop, Theme};
pub use toolbar::{Edge, Item, Path, ToolbarLayout};
