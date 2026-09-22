//! What a cut carries over to the pieces it leaves, and what it cannot.

use crate::constraints::{Constraint, Dimension, DimensionTarget, Toward};
use crate::sketch::{PointId, SegmentId};

/// One of the pieces a cut left, and what of the trait's own rules it can
/// still answer for.
pub(super) struct Piece {
    pub(super) id: SegmentId,
    /// Where it starts and stops along the trait it came from, as fractions of
    /// that trait.
    pub(super) spans: (f64, f64),
    /// Whether it still reaches the point an angle on the trait was read at.
    pub(super) reaches_the_corner: bool,
}

impl Piece {
    /// Whether a place on the trait fell on this piece.
    pub(super) fn holds(&self, place: f64) -> bool {
        (self.spans.0..=self.spans.1).contains(&place)
    }

    /// Whether this piece runs on from a place on the trait the way an arm
    /// heads: it carries the stretch just past that place, towards the end or
    /// back towards the start.
    ///
    /// The place is where two lines cross, found by one sum, and the piece's
    /// bounds come from another; at a T's foot, or where a division cut, the
    /// two should agree and need not to the last bit. A T's foot comes out a
    /// few parts in 10¹⁶ *before* the stem's start about one time in five, and
    /// read exactly that drops the angle off the only piece that carries it.
    /// So the place is taken a hair along the way the arm heads, where the
    /// stretch it measures truly lies.
    pub(super) fn runs_on_from(&self, place: f64, toward: Toward) -> bool {
        const PAST: f64 = 1e-9;
        let past = match toward {
            Toward::End => place + PAST,
            Toward::Start => place - PAST,
        };
        self.spans.0 < past && past < self.spans.1
    }
}

/// What a trait carried before it was cut: everything standing that spoke of
/// it, and where along it the ones fastened to a place sat.
///
/// A tangency is held at the point where the circle touches; a distance to the
/// line, at the foot of the point it measures from. Both are places on the
/// line, and each follows the piece its place fell on.
pub(super) struct Carried {
    pub(super) rules: Vec<Constraint>,
    pub(super) values: Vec<Dimension>,
    /// The points a rule holds on the trait, and where along it they sit.
    pub(super) held: Vec<(f64, PointId)>,
    pub(super) fastened: Vec<(Constraint, f64)>,
    pub(super) measured_at: Vec<(DimensionTarget, f64)>,
}

impl Carried {
    pub(super) fn place_of(&self, rule: Constraint) -> Option<f64> {
        place_in(&self.fastened, rule)
    }

    pub(super) fn place_measured(&self, value: DimensionTarget) -> Option<f64> {
        place_in(&self.measured_at, value)
    }
}

fn place_in<T: Copy + PartialEq>(fastened: &[(T, f64)], of: T) -> Option<f64> {
    fastened
        .iter()
        .find(|(held, _)| *held == of)
        .map(|(_, place)| *place)
}

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
/// What a rule says about *direction* survives whole: the pieces lie on the
/// line the trait lay on, so they stand to everything else exactly as it did.
/// What is fastened to a place on that line — a tangency, held at the point
/// where the circle touches — follows the piece that place fell on. A rule
/// about length speaks of a trait that is no longer there.
pub(super) fn still_holds(
    rule: Constraint,
    cut: SegmentId,
    piece: &Piece,
    place: Option<f64>,
) -> Option<Constraint> {
    let moved = |id: SegmentId| match id == cut {
        true => piece.id,
        false => id,
    };
    let fell_on_the_piece = place.is_some_and(|place| piece.holds(place));
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
                segment: piece.id,
                axis,
            })
        }
        Constraint::Tangent {
            circle,
            segment,
            at,
        } if segment == cut && fell_on_the_piece => Some(Constraint::Tangent {
            circle,
            segment: piece.id,
            at,
        }),
        _ => None,
    }
}

/// What a value measured against the trait still measures, once the trait is a
/// piece of itself.
///
/// An angle against an axis is read off the direction, which both pieces
/// inherit. An angle at a corner belongs to whichever piece still reaches that
/// corner, and a distance to the line to whichever piece the foot of it fell
/// on — put on both, it would say the same thing twice. A length measures a
/// trait that is shorter than what was typed, and says nothing about either
/// piece.
pub(super) fn still_measured(
    value: DimensionTarget,
    cut: SegmentId,
    piece: &Piece,
    place: Option<f64>,
) -> Option<DimensionTarget> {
    let moved = |id: SegmentId| match id == cut {
        true => piece.id,
        false => id,
    };
    match value {
        DimensionTarget::AxisAngle { segment, axis } if segment == cut => {
            Some(DimensionTarget::AxisAngle {
                segment: piece.id,
                axis,
            })
        }
        DimensionTarget::Angle { first, second }
            if (first == cut || second == cut) && piece.reaches_the_corner =>
        {
            Some(DimensionTarget::Angle {
                first: moved(first),
                second: moved(second),
            })
        }
        DimensionTarget::PointToSegment { point, segment }
            if segment == cut && place.is_some_and(|place| piece.holds(place)) =>
        {
            Some(DimensionTarget::PointToSegment {
                point,
                segment: piece.id,
            })
        }
        // An angle between two traits that share no end is read where they
        // meet, and its arm runs one way from there: it follows the piece that
        // carries on from that place in that direction. A place a cut took
        // away leaves no piece to meet the other trait, and the angle goes.
        DimensionTarget::AngleBetween {
            first,
            first_toward,
            second,
            second_toward,
        } if first == cut || second == cut => {
            let toward = match first == cut {
                true => first_toward,
                false => second_toward,
            };
            place
                .is_some_and(|meet| piece.runs_on_from(meet, toward))
                .then_some(DimensionTarget::AngleBetween {
                    first: moved(first),
                    first_toward,
                    second: moved(second),
                    second_toward,
                })
        }
        _ => None,
    }
}
