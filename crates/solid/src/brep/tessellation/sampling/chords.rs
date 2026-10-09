//! Where a stretch of a curve given by its parameter is cut between two of
//! its samples, so that every chord stands within the tolerance of it.

use glam::DVec3;

/// How many times a stretch is halved at most: past this, a chord is shorter
/// than the rounding of the parameter along a curve of any reach.
const HALVINGS: u32 = 40;

/// How many equal parts a stretch is cut into to measure its chord from the
/// curve: the places between them are where the curve is measured.
const PROBES: usize = 8;

/// The parameters strictly between `from` and `to` a stretch of the curve
/// `point` is cut at, in order, pushed onto `places`, so that no chord
/// between two of them stands further than `tolerance` from the curve.
pub(super) fn halved(
    point: &dyn Fn(f64) -> DVec3,
    from: f64,
    to: f64,
    tolerance: f64,
    places: &mut Vec<f64>,
) {
    halve(point, from, to, tolerance, HALVINGS, places);
}

fn halve(
    point: &dyn Fn(f64) -> DVec3,
    from: f64,
    to: f64,
    tolerance: f64,
    left: u32,
    places: &mut Vec<f64>,
) {
    if left == 0 || straight(point, from, to, tolerance) {
        return;
    }
    let middle = (from + to) / 2.0;
    halve(point, from, middle, tolerance, left - 1, places);
    places.push(middle);
    halve(point, middle, to, tolerance, left - 1, places);
}

/// Whether the chord of a stretch stands within `tolerance` of the curve,
/// measured on the curve itself at places spread along the stretch.
fn straight(point: &dyn Fn(f64) -> DVec3, from: f64, to: f64, tolerance: f64) -> bool {
    let (start, end) = (point(from), point(to));
    (1..PROBES).all(|part| {
        let at = from + (to - from) * part as f64 / PROBES as f64;
        from_chord(point(at), start, end) <= tolerance
    })
}

/// How far `point` stands from the segment between `start` and `end`.
fn from_chord(point: DVec3, start: DVec3, end: DVec3) -> f64 {
    let along = end - start;
    let length = along.length_squared();
    let share = if length > 0.0 {
        ((point - start).dot(along) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (point - (start + along * share)).length()
}
