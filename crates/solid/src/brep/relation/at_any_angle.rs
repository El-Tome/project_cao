//! Two walls not parallel, an elliptic one among them (#571, #577): they
//! meet along the curve `meet.rs` builds, decided once by `Meeting::of` —
//! moved onto a touch or a node within the tolerance — or stand apart. Two
//! cylinders are `cylinders.rs`'s; parallel walls are not read here.

use super::{Relation, parallel};
use crate::brep::curve::Wall;
use crate::brep::meet::{Configuration, Meeting};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

/// The relation of two walls not parallel, one of them elliptic; none for
/// any other pair.
pub(super) fn walls(one: &Surface, other: &Surface, scale: Scale) -> Option<Relation> {
    let (first, second) = (Wall::of(one)?, Wall::of(other)?);
    if matches!((first, second), (Wall::Cylinder(_), Wall::Cylinder(_)))
        || parallel(first.axis(), second.axis(), scale)
    {
        return None;
    }
    let meeting = Meeting::as_they_stand(&first, &second);
    Some(if meeting.configuration == Configuration::Apart {
        Relation::Apart
    } else {
        Relation::Meet(meeting)
    })
}
