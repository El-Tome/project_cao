//! A loop of a face of a sphere or a spindle passing through both poles
//! (#628): a lune between two meridians, or what a coaxial turn of another
//! extent leaves of one.
//!
//! Each pass through a pole is closed along the pole's floor, from the
//! angle about the axis the loop comes in at to the angle it leaves at, the
//! low pole's floor running the angle up and the high pole's down — the way
//! that keeps the face on the loop's left, as the domain and the triangles
//! close it — and the other way round on a face listed inward, whose loops
//! run the other way. Two meridians part the surface into two lunes, both
//! bounded, and either may be the face: only the way the loop runs says
//! which, so the area tells nothing of a face listed backwards there, which
//! is left to the rays, the triangles and the volume.

use std::f64::consts::TAU;

use glam::DVec3;

use crate::brep::Listing;
use crate::soundness::listed::lying::point;
use crate::soundness::listed::round::Tube;

/// How far along an edge from its end at a pole the angle about the axis it
/// comes in or leaves at is read, as a fraction of the edge.
const NEAR: f64 = 1e-3;

/// What closing a loop through both poles along their floors sweeps, read
/// from `about`, and the angle about the axis the floors turn; none where
/// the loop does not pass through both.
pub(super) fn through_both(
    tube: &Tube,
    listing: &Listing,
    uses: &[(usize, bool)],
    (about, room): (DVec3, f64),
    outward: bool,
) -> Option<(f64, f64)> {
    let poles = tube.poles();
    let &[low, high] = &poles[..] else {
        return None;
    };
    let ends = |&(edge, forward): &(usize, bool)| {
        let listed = &listing.edges[edge];
        let (start, end) = if forward {
            (listed.from, listed.to)
        } else {
            (listed.to, listed.from)
        };
        (start, end, &listed.curve)
    };
    let from = tube.phi(about);
    let (mut area, mut turned) = (0.0, 0.0);
    let mut passed = [false, false];
    for (one, next) in uses.iter().zip(uses.iter().cycle().skip(1)) {
        let (start, end, curve) = ends(one);
        let reached = point(curve, end);
        let Some(rank) = [low, high]
            .iter()
            .position(|pole| reached.distance(*pole) <= room)
        else {
            continue;
        };
        let (onward, beyond, next_curve) = ends(next);
        let arriving = tube.angle(point(curve, end + (start - end) * NEAR));
        let leaving = tube.angle(point(next_curve, onward + (beyond - onward) * NEAR));
        let swing = if (rank == 0) == outward {
            (leaving - arriving).rem_euclid(TAU)
        } else {
            -(arriving - leaving).rem_euclid(TAU)
        };
        area -= tube.rise(from, tube.phi([low, high][rank])) * swing;
        turned += swing;
        passed[rank] = true;
    }
    (passed == [true, true]).then_some((area, turned))
}
