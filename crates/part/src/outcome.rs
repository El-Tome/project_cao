//! What an operation has to say for itself.
//!
//! Most say nothing: they do what was asked and the drawing shows it. The ones
//! here changed something the drawing cannot show on its own — a scale it now
//! holds, a value it could not honour, a rule a cut took away — and the
//! interface is the only place that can say so.

use crate::dimensioning::DimensionOutcome;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    /// What a typed value did.
    Dimension(DimensionOutcome),
    /// What a cut left behind, counted: rules and values that spoke of the
    /// trait and of neither of its pieces, and the corners of the gesture it
    /// could not take at all.
    Cut {
        rules: usize,
        values: usize,
        /// Corners the gesture named that were not there to cut. One gesture
        /// names several, and a corner refused does not stop the others — so
        /// it is counted and said, rather than silently dropped.
        refused: usize,
    },
    /// A rule the drawing could not hold, whatever gave way: the drawing is as
    /// it was, and the rule was not laid.
    RuleRefused,
}
