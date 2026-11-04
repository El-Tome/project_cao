//! Whether a turn was cut into fewer steps than its tolerance asks for, held
//! to the most a turn is cut into: the tolerance then no longer holds there.

use std::cell::Cell;

thread_local! {
    static CAPPED: Cell<bool> = const { Cell::new(false) };
}

/// What `work` answers, and whether some turn in it was capped.
pub(in crate::brep::tessellation) fn capped<T>(work: impl FnOnce() -> T) -> (T, bool) {
    let outer = CAPPED.replace(false);
    let answer = work();
    let inner = CAPPED.replace(outer);
    CAPPED.set(outer || inner);
    (answer, inner)
}

pub(super) fn noted() {
    CAPPED.set(true);
}
