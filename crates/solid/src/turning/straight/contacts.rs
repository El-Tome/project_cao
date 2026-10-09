//! Whether laying a profile against its axis made it touch itself where
//! the drawing did not: a corner brought onto a run, two runs brought onto
//! one line, a run brought back onto the one before it, two runs brought
//! across each other; whether a slanted run meets the rest of the profile
//! anywhere but at its own corners; and whether the profile touches itself
//! at a corner on the axis between two runs that both leave it.
//!
//! Corners laid at one level share its value to the bit, and every run is
//! laid exactly square or parallel to the axis, or slanted as drawn: the laid
//! profile is compared exactly, with no tolerance, but for how near a corner
//! comes to a slant, which shares no level with it.

use glam::DVec2;

use super::Corner;

/// One run of a laid contour: the corners it goes between, by their number
/// in the profile.
pub(super) type Run = (usize, usize);

/// Whether the laid profile touches itself only where the profile as read
/// already did, to `eps`, and meets no slanted run but at its ends.
/// `contours` are the runs of each contour in order, each beginning where the
/// one before it ends.
pub(super) fn are_drawn(
    laid: &[DVec2],
    read: &[DVec2],
    contours: &[Vec<Run>],
    eps: f64,
    tolerance: f64,
) -> bool {
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
                && !slanted(laid, (from, to))
                && on(laid[corner], laid[from], laid[to])
                && apart(read[corner], read[from], read[to]) > eps
        })
    });
    !turns_back && !meet && !touches && !a_slant_is_met(laid, &runs, tolerance)
}

/// Whether a slanted run meets the rest of the laid profile anywhere but at
/// its ends: a corner within `tolerance` of it that is not laid on one of
/// them, or a run across it.
pub(super) fn a_slant_is_met(laid: &[DVec2], runs: &[Run], tolerance: f64) -> bool {
    runs.iter()
        .filter(|&&run| slanted(laid, run))
        .any(|&slant| {
            let (from, to) = (laid[slant.0], laid[slant.1]);
            runs.iter().any(|&other| {
                let corner = laid[other.0];
                cross(laid, slant, other)
                    || (corner != from && corner != to && near(corner, from, to, tolerance))
            })
        })
}

/// Whether a contour has a corner on the axis between two runs that both
/// leave it, where the body turned from it would touch itself at a point.
pub(super) fn pinched(contours: &[Vec<Corner>]) -> bool {
    contours.iter().any(|corners| {
        let count = corners.len();
        (0..count).any(|index| {
            let before = corners[(index + count - 1) % count].at;
            let after = corners[(index + 1) % count].at;
            corners[index].at.y == 0.0 && before.y > 0.0 && after.y > 0.0
        })
    })
}

/// Whether a laid run is neither square nor parallel to the axis.
pub(super) fn slanted(laid: &[DVec2], run: Run) -> bool {
    let (from, to) = (laid[run.0], laid[run.1]);
    from.x != to.x && from.y != to.y
}

/// Whether two runs lie on one line and share more than a point of it.
fn overlap(laid: &[DVec2], first: Run, second: Run) -> bool {
    let (a, b, c, d) = (laid[first.0], laid[first.1], laid[second.0], laid[second.1]);
    if slanted(laid, first) || slanted(laid, second) {
        let along = b - a;
        if along.perp_dot(c - a) != 0.0 || along.perp_dot(d - a) != 0.0 {
            return false;
        }
        let (start, end) = (along.dot(c - a), along.dot(d - a));
        return start.max(end).min(along.length_squared()) > start.min(end).max(0.0);
    }
    let shared = |axis: usize, across: usize| {
        a[axis] == b[axis]
            && c[axis] == d[axis]
            && a[axis] == c[axis]
            && a[across].max(b[across]).min(c[across].max(d[across]))
                > a[across].min(b[across]).max(c[across].min(d[across]))
    };
    shared(0, 1) || shared(1, 0)
}

/// Whether two runs cross inside both: a run square to the axis and one
/// parallel to it, or any two of which one slants, each with the other's
/// ends strictly on either side of its line.
fn cross(laid: &[DVec2], first: Run, second: Run) -> bool {
    if slanted(laid, first) || slanted(laid, second) {
        let (a, b, c, d) = (laid[first.0], laid[first.1], laid[second.0], laid[second.1]);
        let sides = |from: DVec2, to: DVec2, one: DVec2, other: DVec2| {
            (to - from).perp_dot(one - from) * (to - from).perp_dot(other - from)
        };
        return sides(a, b, c, d) < 0.0 && sides(c, d, a, b) < 0.0;
    }
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

/// Whether a point stands within `tolerance` of a slanted run as levels
/// are told apart, one coordinate at a time: along the axis from where the
/// run passes at the point's distance from it, or away from it where the run
/// passes at the point's place along it. A corner a run longer than the
/// tolerance from the slant's end, on a level of its own, is not near it.
pub(super) fn near(point: DVec2, from: DVec2, to: DVec2, tolerance: f64) -> bool {
    let gap = |axis: usize| {
        let across = 1 - axis;
        if point[across] < from[across].min(to[across])
            || point[across] > from[across].max(to[across])
        {
            return f64::INFINITY;
        }
        let share = (point[across] - from[across]) / (to[across] - from[across]);
        (point[axis] - (from[axis] + (to[axis] - from[axis]) * share)).abs()
    };
    gap(0).min(gap(1)) <= tolerance
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
