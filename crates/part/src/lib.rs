//! The part: what a `.caopart` records, and how it is replayed.
//!
//! Keep this crate free of any UI/windowing dependency so it can be reused
//! as-is by a future tablet or web shell.

mod adapters;
mod document;
mod errors;
mod extrusion;
pub mod feature;
mod file_name;
pub mod history;
pub mod ports;
mod state;

#[cfg(any(test, feature = "test-support"))]
pub use adapters::InMemoryFiles;
pub use document::{PartDocument, PartMetadata, SCHEMA_VERSION};
pub use errors::PartFileError;
pub use history::{ExtrusionMode, History, Operation, PointRef, RevolutionAxis};
pub use ports::{FileError, Files};
pub use state::{DimensionOutcome, PartState};
