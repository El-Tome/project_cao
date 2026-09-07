//! Domain types and persistence shared by every CAO front-end (desktop app today,
//! future tablet/web shells later). Keep this crate free of any UI/windowing
//! dependency so it can be reused as-is by those future front-ends.

pub mod command;
pub mod config;
mod document;
pub mod history;
mod recents;
pub mod settings;
pub mod shortcuts;
mod state;
mod storage;
pub mod theme;
pub mod toolbar;

pub use command::Command;
pub use config::ViewportConfig;
pub use document::{PartDocument, PartMetadata, SCHEMA_VERSION};
pub use history::{ExtrusionMode, History, Operation, PointRef, RevolutionAxis};
pub use recents::{MAX_RECENTS, RecentEntry, RecentList};
pub use settings::{PROFILE_EXTENSION, Profile, Profiles, Settings};
pub use shortcuts::{Chord, Key, Shortcuts};
pub use state::{DimensionOutcome, PartState};
pub use storage::{StorageError, crash_log_path, default_projects_dir, record_panics};
pub use theme::{Background, Rgba, Stop, Theme};
pub use toolbar::{Edge, ToolbarLayout};
