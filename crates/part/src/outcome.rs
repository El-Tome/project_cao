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
        /// Corners the value did not fit. One gesture names several, and a
        /// corner too tight for the radius asked does not stop the others —
        /// so it is counted and said, rather than silently dropped.
        refused: usize,
    },
}

impl Outcome {
    /// Two cuts of one gesture, counted together: what the whole gesture cost
    /// is what the user is told, not what each corner of it cost.
    pub(crate) fn and(self, other: Self) -> Self {
        match (self, other) {
            (
                Self::Cut {
                    rules,
                    values,
                    refused,
                },
                Self::Cut {
                    rules: more,
                    values: worth,
                    refused: turned_away,
                },
            ) => Self::Cut {
                rules: rules + more,
                values: values + worth,
                refused: refused + turned_away,
            },
            (_, last) => last,
        }
    }
}
