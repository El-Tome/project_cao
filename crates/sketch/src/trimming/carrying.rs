//! What a cut carries over to the pieces it leaves, and what it cannot.

use crate::constraints::{Constraint, Dimension, DimensionTarget};
use crate::sketch::SegmentId;

/// What was standing before the trait went and is not standing after: what
/// spoke of it.
pub(super) fn gone<T: Copy + PartialEq>(before: &[T], after: &[T]) -> Vec<T> {
    before
        .iter()
        .copied()
        .filter(|held| !after.contains(held))
        .collect()
}

pub(super) fn targets(values: &[Dimension]) -> Vec<DimensionTarget> {
    values.iter().map(|value| value.target).collect()
}

/// The same rule, said of a piece of the trait it named.
///
/// Only what a rule says about *direction* survives a cut: the pieces lie on
/// the line the trait lay on, so they stand to everything else exactly as it
/// did. A rule about its length speaks of a trait that is no longer there.
pub(super) fn about_direction(
    rule: Constraint,
    cut: SegmentId,
    piece: SegmentId,
) -> Option<Constraint> {
    let moved = |id: SegmentId| match id == cut {
        true => piece,
        false => id,
    };
    match rule {
        Constraint::Perpendicular { first, second } if first == cut || second == cut => {
            Some(Constraint::Perpendicular {
                first: moved(first),
                second: moved(second),
            })
        }
        Constraint::Parallel { first, second } if first == cut || second == cut => {
            Some(Constraint::Parallel {
                first: moved(first),
                second: moved(second),
            })
        }
        Constraint::Collinear { first, second } if first == cut || second == cut => {
            Some(Constraint::Collinear {
                first: moved(first),
                second: moved(second),
            })
        }
        Constraint::AxisCollinear { segment, axis } if segment == cut => {
            Some(Constraint::AxisCollinear {
                segment: piece,
                axis,
            })
        }
        _ => None,
    }
}

/// What a value measured against the trait still measures, once the trait is a
/// piece of itself.
///
/// An angle against an axis is read off the direction, which both pieces
/// inherit. An angle at a corner belongs to whichever piece still reaches that
/// corner. A length measures a trait that is shorter than what was typed, and
/// says nothing about either piece.
pub(super) fn still_measured(
    value: DimensionTarget,
    cut: SegmentId,
    piece: SegmentId,
    reaches_the_corner: bool,
) -> Option<DimensionTarget> {
    let moved = |id: SegmentId| match id == cut {
        true => piece,
        false => id,
    };
    match value {
        DimensionTarget::AxisAngle { segment, axis } if segment == cut => {
            Some(DimensionTarget::AxisAngle {
                segment: piece,
                axis,
            })
        }
        DimensionTarget::Angle { first, second }
            if (first == cut || second == cut) && reaches_the_corner =>
        {
            Some(DimensionTarget::Angle {
                first: moved(first),
                second: moved(second),
            })
        }
        _ => None,
    }
}
