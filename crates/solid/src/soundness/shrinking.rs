//! Taking a failing case down to the smallest one that still fails.

/// Shrinks `case` for as long as one of the cases `smaller` offers still
/// fails, and hands back the last one that did.
///
/// Greedy: the first smaller case that still fails is taken, and the search
/// starts again from it. That does not find the smallest case there is, but it
/// finds one nothing offered can shrink further, which is what a human needs
/// to read — two faces and a cut rather than seven solids and four steps.
///
/// `still_fails` answers whether a case breaks the rule the original broke,
/// not merely whether it breaks something: a case shrunk into a different
/// failure is a different bug, and the report would describe neither.
/// `keep_going` bounds the search, so a campaign stays inside its deadline.
pub fn shrink<C: Clone>(
    case: C,
    smaller: impl Fn(&C) -> Vec<C>,
    mut still_fails: impl FnMut(&C) -> bool,
    mut keep_going: impl FnMut() -> bool,
) -> C {
    let mut smallest = case;
    'search: while keep_going() {
        for candidate in smaller(&smallest) {
            if !keep_going() {
                break 'search;
            }
            if still_fails(&candidate) {
                smallest = candidate;
                continue 'search;
            }
        }
        break;
    }
    smallest
}

#[cfg(test)]
mod tests;
