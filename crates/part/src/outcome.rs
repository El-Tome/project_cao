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
    /// trait and of neither of its pieces.
    Cut { rules: usize, values: usize },
}

impl Outcome {
    /// Two cuts of one gesture, counted together: what the whole gesture cost
    /// is what the user is told, not what each corner of it cost.
    pub(crate) fn and(self, other: Self) -> Self {
        match (self, other) {
            (
                Self::Cut { rules, values },
                Self::Cut {
                    rules: more,
                    values: worth,
                },
            ) => Self::Cut {
                rules: rules + more,
                values: values + worth,
            },
            (_, last) => last,
        }
    }
}
