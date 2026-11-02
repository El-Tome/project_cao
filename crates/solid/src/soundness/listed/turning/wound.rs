//! A face of a ring winding round the tube (#623): between loops that each
//! wind round it — two whole meridians, or a meridian and a notch a coaxial
//! turn cut into the other side.
//!
//! Two such loops part the torus into two strips, each bounded, and only
//! the way the loops run says which of them the face is: a face listed
//! backwards reads as the other strip, listed right. So the area such loops
//! sweep tells nothing, and is not read; what can be read is that they wind
//! round the tube as many times one way as the other, and the rays, the
//! triangles and the volume tell the rest.

use std::f64::consts::{PI, TAU};

use super::CHORDS;
use crate::brep::Listing;
use crate::soundness::listed::lying::point;
use crate::soundness::listed::round::Tube;

/// How many times a loop winds round the tube, the angle along the
/// meridian it turns read along chords of its edges.
pub(super) fn tube_turns(tube: &Tube, listing: &Listing, uses: &[(usize, bool)]) -> f64 {
    let turned: f64 = uses
        .iter()
        .map(|&(edge, forward)| {
            let edge = &listing.edges[edge];
            let phi = |piece: usize| {
                let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
                tube.phi(point(&edge.curve, at))
            };
            let turned: f64 = (0..CHORDS)
                .map(|piece| (phi(piece + 1) - phi(piece) + PI).rem_euclid(TAU) - PI)
                .sum();
            if forward { turned } else { -turned }
        })
        .sum();
    (turned / TAU).round()
}

/// What a loop of a ring sweeps beyond what its edges read in the window of
/// the chart alone (#623): a loop running through the cut, a notch across
/// it, reads the latitude circles past the cut a whole turn of the meridian
/// off, and each of them sweeps the ring's radius times that turn times the
/// angle it runs about the axis. Read along the loop from its start, the
/// angle along the meridian run on as the loop runs; nothing for a loop that
/// never reaches the cut.
pub(super) fn past_the_cut(tube: &Tube, listing: &Listing, uses: &[(usize, bool)]) -> f64 {
    let ring = tube.rise(0.0, TAU) / TAU;
    let places: Vec<glam::DVec3> = uses
        .iter()
        .flat_map(|&(edge, forward)| {
            let edge = &listing.edges[edge];
            let mut places: Vec<glam::DVec3> = (0..=CHORDS)
                .map(|piece| {
                    let at = edge.from + (edge.to - edge.from) * piece as f64 / CHORDS as f64;
                    point(&edge.curve, at)
                })
                .collect();
            if !forward {
                places.reverse();
            }
            places
        })
        .collect();
    let mut swept = 0.0;
    let mut off = 0.0;
    for pair in places.windows(2) {
        let (from, to) = (tube.phi(pair[0]), tube.phi(pair[1]));
        let turn = (tube.angle(pair[1]) - tube.angle(pair[0]) + PI).rem_euclid(TAU) - PI;
        swept -= ring * off * turn;
        off += (to - from + PI).rem_euclid(TAU) - PI - (to - from);
    }
    swept
}
