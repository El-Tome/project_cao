//! The part: what a `.caopart` records, and how it is replayed.
//!
//! Keep this crate free of any UI/windowing dependency so it can be reused
//! as-is by a future tablet or web shell.

mod document;
mod errors;
pub mod history;
mod state;

pub use document::{PartDocument, PartMetadata, SCHEMA_VERSION};
pub use errors::PartFileError;
pub use history::{ExtrusionMode, History, Operation, PointRef, RevolutionAxis};
pub use state::{DimensionOutcome, PartState};
