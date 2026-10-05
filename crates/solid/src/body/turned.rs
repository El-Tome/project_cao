//! A profile turned by the flats, for the body to fall back on when the
//! exact kernel does not turn it.

use glam::DVec2;

use crate::mesh::Mesh;
use crate::profile::{Frame, Profile};
use crate::sweep::{self, Loop};
use crate::turning::Turn;

/// The profile turned by the flats, every point the drawing can only have
/// meant on the axis laid on it first (#487, #488); `None` when they make
/// nothing of it.
pub(super) fn turned_flats(profile: &Profile, frame: Frame, turn: &Turn) -> Option<Mesh> {
    let snapped = Snapped::of(profile, turn);
    let holes: Vec<Loop<'_>> = snapped
        .holes
        .iter()
        .zip(&profile.sampled_holes)
        .map(|(points, hole)| Loop {
            points,
            curves: hole.curves,
        })
        .collect();
    sweep::revolution(
        Loop {
            points: &snapped.outline,
            curves: profile.sampled.curves,
        },
        &holes,
        &snapped.triangles,
        |point| frame.at(point),
        turn.axis.origin,
        turn.axis.direction,
        turn.angle,
    )
}

/// The points a profile was sampled into, each laid on the axis when it
/// stands within the turn's band of it.
struct Snapped {
    outline: Vec<DVec2>,
    holes: Vec<Vec<DVec2>>,
    triangles: Vec<[DVec2; 3]>,
}

impl Snapped {
    fn of(profile: &Profile, turn: &Turn) -> Snapped {
        let laid = |points: &[DVec2]| points.iter().map(|&point| turn.on_axis(point)).collect();
        Snapped {
            outline: laid(profile.sampled.points),
            holes: profile
                .sampled_holes
                .iter()
                .map(|hole| laid(hole.points))
                .collect(),
            triangles: profile
                .triangles
                .iter()
                .map(|triangle| triangle.map(|point| turn.on_axis(point)))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests;
