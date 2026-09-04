//! Domain types and persistence shared by every CAO front-end (desktop app today,
//! future tablet/web shells later). Keep this crate free of any UI/windowing
//! dependency so it can be reused as-is by those future front-ends.

pub mod config;
mod document;
mod recents;
mod storage;

pub use config::ViewportConfig;
pub use document::PartDocument;
pub use recents::{MAX_RECENTS, RecentEntry, RecentList};
pub use storage::{StorageError, default_projects_dir};
