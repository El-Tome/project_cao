//! Domain types and persistence shared by every CAO front-end (desktop app today,
//! future tablet/web shells later). Keep this crate free of any UI/windowing
//! dependency so it can be reused as-is by those future front-ends.

mod document;
mod recents;
mod storage;

pub use document::PartDocument;
pub use recents::{RecentEntry, RecentList, MAX_RECENTS};
pub use storage::{default_projects_dir, StorageError};
