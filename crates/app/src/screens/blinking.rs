//! What a refusal names blinks where it stands — a value on the drawing, a
//! face of the part, a row of the variables — for a few seconds: long enough
//! to be found, short enough not to stay in the way.

const BLINKING_FOR: f64 = 3.0;
const ONE_FLASH: f64 = 0.25;

/// Whether what began blinking at `since` is lit at `now`, or dark — nothing
/// once the blinking is over.
pub(crate) fn lit(since: f64, now: f64) -> Option<bool> {
    let gone = now - since;
    (0.0..BLINKING_FOR)
        .contains(&gone)
        .then(|| ((gone / ONE_FLASH) as u64).is_multiple_of(2))
}

#[cfg(test)]
mod tests;
