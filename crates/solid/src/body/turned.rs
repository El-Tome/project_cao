//! A profile turned by the flats, for the body to fall back on when the
//! exact kernel does not turn it.

use crate::mesh::Mesh;
use crate::profile::{Frame, Profile};
use crate::sweep;
use crate::turning::Turn;

/// The profile turned by the flats, or `None` when they make nothing of it.
pub(super) fn turned_flats(profile: &Profile, frame: Frame, turn: &Turn) -> Option<Mesh> {
    sweep::revolution(
        profile.sampled,
        &profile.sampled_holes,
        profile.triangles,
        |point| frame.at(point),
        turn.axis.origin,
        turn.axis.direction,
        turn.angle,
    )
}
