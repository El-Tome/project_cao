//! The cut of a ring torus's chart (#576, #623): the window one turn wide
//! along the meridian a ring's faces are read in is cut at the antipode of
//! its seam, and a face reaching across the cut is closed along what it
//! reaches of it, so that a ray up the meridian from anywhere in the face
//! crosses an odd number of times in all.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use crate::brep::trace::Trace;

/// The laps of the chart's cut a face of a ring is closed along: one for
/// each stretch of the cut its meridians cross it at ([`wraps`]), or the
/// whole cut for a face crossed by none that holds it ([`holds_the_cut`]).
/// Nothing on a sphere, whose chart has no cut, nor where the meridians
/// crossing the cut do not start and stop the face in turn.
pub(super) fn closing(loops: &[Vec<Trace>], seam: Option<f64>, flipped: bool) -> Vec<Vec<Trace>> {
    let Some(seam) = seam else {
        return Vec::new();
    };
    match wraps(loops, seam, flipped) {
        Some(laps) if !laps.is_empty() => laps,
        Some(_) if holds_the_cut(loops, flipped) => vec![cut(seam, TAU, 0.0)],
        Some(_) | None => Vec::new(),
    }
}

/// The lap of the chart's two cuts that closes a face of a ring between the
/// angles `left` and `right` about the axis: the window's top, at `seam + π`
/// along the meridian, run from `right` to `left`, and its bottom, at
/// `seam − π`, run back, so that a ray up the meridian from anywhere in the
/// window between them crosses the top once. A face with no loop is the
/// whole window.
pub(super) fn cut(seam: f64, right: f64, left: f64) -> Vec<Trace> {
    vec![
        Trace::Segment {
            from: DVec2::new(right, seam + PI),
            to: DVec2::new(left, seam + PI),
        },
        Trace::Segment {
            from: DVec2::new(left, seam - PI),
            to: DVec2::new(right, seam - PI),
        },
    ]
}

/// The laps of the chart's cut that close a face of a torus whose loops wind
/// round the tube — a ring turned part of the way, bounded by two whole
/// meridians, or by a meridian, a rim and another meridian — one for each
/// stretch of the cut within the face. Only a meridian crosses the cut, and
/// the face reaches the cut on the side of it the loop keeps it on: the
/// side where the angle about the axis falls of a loop running the angle
/// along the meridian up, the other side of one running it down, and the
/// other way round on a `flipped` face. A meridian crosses the cut where it
/// runs through it, or once all along when it is a whole circle: one ending
/// on the cut, at a rim the cut lies along, does not. Each stretch runs from
/// where the face starts along the cut to where it stops, the angle about
/// the axis growing. None where the crossings do not start and stop the
/// face in turn, and no stretch where no loop crosses the cut.
fn wraps(loops: &[Vec<Trace>], seam: f64, flipped: bool) -> Option<Vec<Vec<Trace>>> {
    let top = seam + PI;
    let mut crossings: Vec<(f64, bool)> = loops
        .iter()
        .flatten()
        .filter_map(|trace| match *trace {
            Trace::Segment { from, to } if from.x == to.x && from.y != to.y => {
                let (low, high) = (from.y.min(to.y), from.y.max(to.y));
                let inside =
                    (((high - top) / TAU).ceil() - ((low - top) / TAU).floor() - 1.0).max(0.0);
                let whole = high - low >= TAU - WHOLE;
                let times = if inside == 0.0 && whole { 1.0 } else { inside };
                let starts = (to.y < from.y) != flipped;
                Some(std::iter::repeat_n(
                    (from.x.rem_euclid(TAU), starts),
                    times as usize,
                ))
            }
            _ => None,
        })
        .flatten()
        .collect();
    crossings.sort_by(|one, other| one.0.total_cmp(&other.0));
    let count = crossings.len();
    if count == 0 {
        return Some(Vec::new());
    }
    if count % 2 == 1 {
        return None;
    }
    let first = crossings.iter().position(|(_, starts)| *starts)?;
    let mut laps = Vec::with_capacity(count / 2);
    for pair in 0..count / 2 {
        let (left, starts) = crossings[(first + 2 * pair) % count];
        let (right, stops) = crossings[(first + 2 * pair + 1) % count];
        if !starts || stops {
            return None;
        }
        laps.push(cut(seam, right, right - (right - left).rem_euclid(TAU)));
    }
    Some(laps)
}

/// Whether a face of a ring no meridian of which crosses the cut holds the
/// whole cut of the chart, as a face with no loop does: it lies above the
/// highest latitude circle bounding it, which keeps it on its left — above
/// where it runs the angle about the axis up, below on a `flipped` face,
/// whose loops run the other way round — and above a rim it touches along
/// there, which runs both ways. A whole ring with holes cut in its tube, and
/// a band the cut runs through (#623).
fn holds_the_cut(loops: &[Vec<Trace>], flipped: bool) -> bool {
    let latitudes: Vec<(f64, bool)> = loops
        .iter()
        .flatten()
        .filter_map(|trace| match *trace {
            Trace::Segment { from, to } if from.y == to.y && from.x != to.x => {
                Some((from.y, to.x > from.x))
            }
            _ => None,
        })
        .collect();
    let highest = latitudes
        .iter()
        .map(|(phi, _)| *phi)
        .fold(f64::NEG_INFINITY, f64::max);
    latitudes
        .iter()
        .any(|&(phi, up)| phi == highest && up != flipped)
}

/// How far short of a whole turn along the meridian a trace may fall and
/// still be a whole circle, read at the scale of the angle's rounding.
const WHOLE: f64 = 1e-9;
