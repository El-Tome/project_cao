/// What happened when a length was applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthOutcome {
    /// The segment now has exactly the requested length, and whatever hung off
    /// its far end moved rigidly with it.
    Exact,
    /// The far end could not move freely because the geometry loops back to the
    /// fixed end. Only the far end moved, so the shapes around it are distorted.
    BestEffort,
    /// The segment has no direction to stretch along.
    Degenerate,
}
