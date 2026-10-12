//! The slivers laying leaves where a run brings a slanted run's end back
//! along it: the run back stays within the tolerance of the slant the whole
//! way, since it starts on it, so what lies between the two is thinner than
//! the tolerance and is not there.

use std::ops::Range;

use glam::DVec2;

use super::contacts::{near, slanted};
use super::{next, previous};

/// Each slanted run whose end the run beyond it brings back to within
/// `tolerance` of it, cut short where that run ends: its end laid on that
/// corner, so the run back is laid to no length. Read from either end of the
/// slant, and once, in the order of the corners.
pub(super) fn cut_short(laid: &mut [DVec2], rings: &[Range<usize>], tolerance: f64) {
    for ring in rings {
        for end in ring.clone() {
            let (before, after) = (previous(ring, end), next(ring, end));
            for (far, back) in [(before, after), (after, before)] {
                let (from, to, corner) = (laid[far], laid[end], laid[back]);
                if corner != from
                    && corner != to
                    && slanted(laid, (far, end))
                    && near(corner, from, to, tolerance)
                {
                    laid[end] = corner;
                    break;
                }
            }
        }
    }
}
