//! What a cut of an arc carries over to the pieces it leaves, and what it
//! cannot. The curve's answer to `carrying.rs`, which says the same for a
//! straight trait: the two sides share no rule, since nothing that names an
//! arc names a trait.

use crate::arc::ArcId;
use crate::constraints::{Constraint, Dimension, DimensionTarget};

/// One of the arcs a cut left, and the stretch of the old sweep it covers.
pub(super) struct Piece {
    pub(super) id: ArcId,
    pub(super) spans: (f64, f64),
    /// Whether this is the piece the reach is read on. A cut takes nothing off
    /// the radius, so both pieces still stand that far from the centre — but
    /// only one carries the number, and a rule holds the other to it. Written
    /// twice, the drawing would let the user drive apart two halves of what was
    /// one circle.
    pub(super) carries_the_reach: bool,
}

impl Piece {
    pub(super) fn holds(&self, place: f64) -> bool {
        (self.spans.0..=self.spans.1).contains(&place)
    }
}

/// What an arc carried before it was cut: everything standing that spoke of
/// it, and where round it the ones fastened to a place sat.
pub(super) struct Carried {
    pub(super) rules: Vec<Constraint>,
    pub(super) values: Vec<Dimension>,
    /// A tangency is held at the point where the line touches, which is a place
    /// on the curve and follows the piece that place fell on.
    pub(super) fastened: Vec<(Constraint, f64)>,
}

impl Carried {
    pub(super) fn place_of(&self, rule: Constraint) -> Option<f64> {
        self.fastened
            .iter()
            .find(|(held, _)| *held == rule)
            .map(|(_, place)| *place)
    }
}

/// The same rule, said of a piece of the arc it named.
///
/// What a rule says about the *reach* survives whole: a cut takes nothing off
/// the radius, so each piece stands to the rest of the drawing exactly as the
/// arc did. A tangency is fastened to the point where the line touches, and
/// follows the piece that point fell on.
pub(super) fn still_holds(
    rule: Constraint,
    cut: ArcId,
    piece: &Piece,
    place: Option<f64>,
) -> Option<Constraint> {
    let moved = |id: ArcId| match id == cut {
        true => piece.id,
        false => id,
    };
    match rule {
        Constraint::EqualRadiusArc { first, second } if first == cut || second == cut => {
            Some(Constraint::EqualRadiusArc {
                first: moved(first),
                second: moved(second),
            })
        }
        Constraint::ArcTangent { arc, segment, at }
            if arc == cut && place.is_some_and(|place| piece.holds(place)) =>
        {
            Some(Constraint::ArcTangent {
                arc: piece.id,
                segment,
                at,
            })
        }
        // A point held on the curve follows the piece it sits on, as the
        // contact of a tangency does. Sitting on the stretch that went, it is
        // held by nothing: what held it is not drawn any more.
        Constraint::OnArc { point, arc }
            if arc == cut && place.is_some_and(|place| piece.holds(place)) =>
        {
            Some(Constraint::OnArc {
                point,
                arc: piece.id,
            })
        }
        _ => None,
    }
}

/// What a value read on the arc still reads, once the arc is a piece of itself.
///
/// A radius is the same reach on either piece, and is read on the one that
/// carries it — the other is held to it. A sweep says how far round the whole
/// curve went, which is further than either piece goes.
pub(super) fn still_measured(
    value: DimensionTarget,
    cut: ArcId,
    piece: &Piece,
) -> Option<DimensionTarget> {
    match value {
        DimensionTarget::ArcRadius(arc) if arc == cut && piece.carries_the_reach => {
            Some(DimensionTarget::ArcRadius(piece.id))
        }
        _ => None,
    }
}
