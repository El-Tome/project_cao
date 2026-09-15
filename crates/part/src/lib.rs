//! The part: what a `.caopart` records, and how it is replayed.
//!
//! Keep this crate free of any UI/windowing dependency so it can be reused
//! as-is by a future tablet or web shell.

mod adapters;
pub mod compaction;
mod copying;
pub(crate) mod dimensioning;
mod document;
mod errors;
mod extrusion;
pub mod feature;
mod file_name;
pub mod history;
pub mod library;
mod outcome;
mod picture;
pub mod ports;
mod state;

#[cfg(any(test, feature = "test-support"))]
pub use adapters::InMemoryFiles;
pub use compaction::compact;
pub use dimensioning::DimensionOutcome;
pub use document::{PartDocument, PartMetadata, SCHEMA_VERSION};
pub use errors::PartFileError;
pub use history::{ExtrusionMode, History, Operation, PointRef, RevolutionAxis};
pub use outcome::Outcome;
pub use picture::Picture;
pub use ports::{Entry, FileError, Files, Folders};
pub use state::PartState;
