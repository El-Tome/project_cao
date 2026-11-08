//! Two cylinders: parallel ones cross along lines or touch along one, and at
//! any other angle they meet along the curve of `meet.rs`, built on the
//! walls as they stand unless `Meeting::of` reads them apart or touching at
//! a point.

use super::{Relation, parallel, sorted};
use crate::brep::curve::{Line, Wall};
use crate::brep::meet::{Configuration, Meeting, goes_first};
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;

pub(super) fn cylinders(one: &Cylinder, other: &Cylinder, scale: Scale) -> Relation {
    if parallel(one.axis, other.axis, scale) {
        return match goes_first(one, other) {
            std::cmp::Ordering::Greater => side_by_side(other, one, scale),
            _ => side_by_side(one, other, scale),
        };
    }
    let (first, second) = (Wall::Cylinder(*one), Wall::Cylinder(*other));
    let snapped = Meeting::of(&first, &second, scale);
    let meeting = if matches!(
        snapped.configuration,
        Configuration::Apart | Configuration::Contact
    ) {
        snapped
    } else {
        Meeting::as_they_stand(&first, &second)
    };
    if meeting.configuration == Configuration::Apart {
        Relation::Apart
    } else {
        Relation::Meet(meeting)
    }
}

/// Parallel, the larger first. Seen square to the axes they are two circles:
/// one when their centres and radii are within the tolerance, touching when
/// the distance of the centres is within it of the sum or the difference of
/// the radii — the line then drawn through the point where exact circles
/// would touch, halfway between the two where they miss by a hair.
fn side_by_side(large: &Cylinder, small: &Cylinder, scale: Scale) -> Relation {
    let eps = scale.eps();
    let between = small.origin - large.origin;
    let across = between - large.axis * large.axis.dot(between);
    let distance = across.length();
    let (outer, inner) = (large.radius, small.radius);
    if distance <= eps && outer - inner <= eps {
        return Relation::Same { agree: true };
    }
    let outside = distance - (outer + inner);
    let inside = (outer - inner) - distance;
    if outside > eps || inside > eps {
        return Relation::Apart;
    }
    let towards = across / distance;
    if outside >= -eps {
        let touch = large.origin + towards * (distance * outer / (outer + inner));
        return Relation::Tangent(Line::through(touch, large.axis));
    }
    if inside >= -eps {
        let touch = large.origin + towards * ((outer + distance + inner) / 2.0);
        return Relation::Tangent(Line::through(touch, large.axis));
    }
    let along = (distance * distance + (outer - inner) * (outer + inner)) / (2.0 * distance);
    let half = ((outer - along) * (outer + along)).sqrt();
    let foot = large.origin + towards * along;
    let aside = large.axis.cross(towards) * half;
    Relation::Lines(sorted([
        Line::through(foot - aside, large.axis),
        Line::through(foot + aside, large.axis),
    ]))
}
