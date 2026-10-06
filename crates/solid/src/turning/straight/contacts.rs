//! Whether laying a profile square to its axis made it touch itself where
//! the drawing did not: a corner brought onto a run, two runs brought onto
//! one line, a run brought back onto the one before it, two runs brought
//! across each other.
//!
//! Corners laid at one level share its value to the bit, and every run is
//! laid exactly square or parallel to the axis: the laid profile is compared
//! exactly, with no tolerance.

use glam::DVec2;

/// One run of a laid contour: the corners it goes between, by their number
/// in the profile.
pub(super) type Run = (usize, usize);

/// Whether the laid profile touches itself only where the profile as read
/// already did, to `eps`. `contours` are the runs of each contour in order,
/// each beginning where the one before it ends.
pub(super) fn are_drawn(laid: &[DVec2], read: &[DVec2], contours: &[Vec<Run>], eps: f64) -> bool {
    let runs: Vec<Run> = contours.iter().flatten().copied().collect();
    let turns_back = contours.iter().any(|contour| {
        (0..contour.len()).any(|index| {
            let (from, to) = contour[index];
            let (_, beyond) = contour[(index + 1) % contour.len()];
            let (before, after) = (laid[to] - laid[from], laid[beyond] - laid[to]);
            before.perp_dot(after) == 0.0 && before.dot(after) < 0.0
        })
    });
    let meet = runs.iter().enumerate().any(|(index, &first)| {
        runs[index + 1..]
            .iter()
            .any(|&second| overlap(laid, first, second) || cross(laid, first, second))
    });
    let touches = runs.iter().any(|&(corner, _)| {
        runs.iter().any(|&(from, to)| {
            corner != from
                && corner != to
                && on(laid[corner], laid[from], laid[to])
                && apart(read[corner], read[from], read[to]) > eps
        })
    });
    !turns_back && !meet && !touches
}

/// Whether two runs lie on one line and share more than a point of it.
fn overlap(laid: &[DVec2], first: Run, second: Run) -> bool {
    let (a, b, c, d) = (laid[first.0], laid[first.1], laid[second.0], laid[second.1]);
    let shared = |axis: usize, across: usize| {
        a[axis] == b[axis]
            && c[axis] == d[axis]
            && a[axis] == c[axis]
            && a[across].max(b[across]).min(c[across].max(d[across]))
                > a[across].min(b[across]).max(c[across].min(d[across]))
    };
    shared(0, 1) || shared(1, 0)
}

/// Whether a run square to the axis and one parallel to it cross inside both.
fn cross(laid: &[DVec2], first: Run, second: Run) -> bool {
    let inside = |value: f64, from: f64, to: f64| from.min(to) < value && value < from.max(to);
    let square_across = |square: Run, parallel: Run| {
        let (a, b, c, d) = (
            laid[square.0],
            laid[square.1],
            laid[parallel.0],
            laid[parallel.1],
        );
        a.x == b.x && c.y == d.y && inside(a.x, c.x, d.x) && inside(c.y, a.y, b.y)
    };
    square_across(first, second) || square_across(second, first)
}

/// Whether a point lies on a run laid square or parallel to the axis, its
/// ends included: on such a run, inside the box it spans.
fn on(point: DVec2, from: DVec2, to: DVec2) -> bool {
    let within = |value: f64, from: f64, to: f64| from.min(to) <= value && value <= from.max(to);
    within(point.x, from.x, to.x) && within(point.y, from.y, to.y)
}

/// How far a point stands from a segment.
fn apart(point: DVec2, from: DVec2, to: DVec2) -> f64 {
    let run = to - from;
    let along = if run.length_squared() == 0.0 {
        0.0
    } else {
        ((point - from).dot(run) / run.length_squared()).clamp(0.0, 1.0)
    };
    point.distance(from + run * along)
}
