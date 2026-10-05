//! A stop on a bug of the exact kernel's own, taken as a decline rather than
//! as the end of the application (#526's decision 3), and kept from the hook
//! that writes crashes down: a stop caught here is no crash, and a part
//! replayed on every edit would fill the crash log with ones that never
//! happened.

use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe, catch_unwind};
use std::sync::Once;

use crate::brep::Declined;

thread_local! {
    static CATCHING: Cell<bool> = const { Cell::new(false) };
}

static SILENCED: Once = Once::new();

/// What the kernel answers, a stop on a bug taken as a decline.
pub(super) fn caught<T>(work: impl FnOnce() -> Result<T, Declined>) -> Result<T, Declined> {
    quietly(work).unwrap_or(Err(Declined::Panicked))
}

/// What `work` answers, or nothing when it stopped on a bug.
pub(super) fn quietly<T>(work: impl FnOnce() -> T) -> Option<T> {
    SILENCED.call_once(silence_caught_panics);
    let outer = CATCHING.replace(true);
    let answer = catch_unwind(AssertUnwindSafe(work)).ok();
    CATCHING.set(outer);
    answer
}

/// Stands in front of the hook in place, and hands it every stop but the
/// ones caught here. The application puts its own hook in place as it
/// starts, before the kernel is ever called.
pub(super) fn silence_caught_panics() {
    let previous = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if !CATCHING.get() {
            previous(info);
        }
    }));
}
