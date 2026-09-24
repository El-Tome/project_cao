//! The part: what a `.caopart` records, and how it is replayed.
//!
//! Keep this crate free of any UI/windowing dependency so it can be reused
//! as-is by a future tablet or web shell.

mod adapters;
pub mod anchoring;
mod broken;
mod compaction;
mod copying;
mod curves;
mod cutting;
mod descent;
pub(crate) mod dimensioning;
mod document;
#[cfg(any(test, feature = "test-support"))]
mod drawn_to_order;
mod errors;
mod extrusion;
pub mod feature;
mod file_name;
pub mod formula;
pub mod history;
pub mod library;
mod outcome;
mod picture;
pub mod ports;
mod resizing;
mod state;
mod straight;
mod superseded;
pub mod variables;

#[cfg(any(test, feature = "test-support"))]
pub use adapters::InMemoryFiles;
pub use broken::Broken;
pub use compaction::compact;
pub use dimensioning::DimensionOutcome;
pub use document::{PartDocument, PartMetadata, Refused, SCHEMA_VERSION, Use};
#[cfg(any(test, feature = "test-support"))]
pub use drawn_to_order::Recipe;
pub use errors::PartFileError;
pub use formula::{Formula, Unreadable};
pub use history::{ExtrusionMode, History, Operation, PointRef, RevolutionAxis};
pub use outcome::Outcome;
pub use picture::Picture;
pub use ports::{Entry, FileError, Files, Folders};
pub use state::PartState;
pub use variables::{NameProblem, Unusable, Variable, VariableChange, VariableId, Variables};
