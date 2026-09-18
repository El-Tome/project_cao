//! Which arc the clicks gathered so far mean.
//!
//! The sibling of [`crate::circling`] for pieces of a circle: the drawing says
//! what a run of clicks adds up to, and the front-end only has to hand over the
//! places they landed on.

use glam::DVec2;

use crate::arcing::ArcDraft;
use crate::construct::circumcentre;

/// How an arc is being drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ArcMode {
    /// The centre, then where the curve starts, then how far round it runs.
    #[default]
    ByCenter,
    /// The two ends, then a point the curve is bent through.
    ByEnds,
}

impl ArcMode {
    /// How many places it needs before the arc is settled.
    pub fn wants(self) -> usize {
        match self {
            Self::ByCenter | Self::ByEnds => 3,
        }
    }
}

/// The arc the clicks so far and the cursor make, if they make one.
///
/// The cursor gives a direction, never a distance: the far end is brought back
/// onto the circle the near one already fixed, so what is drawn is a piece of a
/// circle at every moment rather than only once it is recorded.
pub fn arc_from(mode: ArcMode, places: &[DVec2], cursor: DVec2) -> Option<ArcDraft> {
    match mode {
        ArcMode::ByCenter => {
            let (centre, start) = (*places.first()?, *places.get(1)?);
            let radius = centre.distance(start);
            let reach = cursor - centre;
            if radius < 1e-9 || reach.length() < 1e-9 {
                return None;
            }
            let end = centre + DVec2::from_angle(reach.to_angle()) * radius;
            (end.distance(start) > 1e-6).then_some(ArcDraft { centre, start, end })
        }
        ArcMode::ByEnds => {
            let (a, b) = (*places.first()?, *places.get(1)?);
            let centre = circumcentre(a, b, cursor)?;
            // The walk always turns counter-clockwise, so the two ends are
            // handed over in whichever order makes that turn pass through
            // the point the curve was bent to, rather than the long way round.
            let angle_of = |point: DVec2| (point - centre).to_angle();
            let (from, through, to) = (angle_of(a), angle_of(cursor), angle_of(b));
            let (start, end) = match (through - from).rem_euclid(std::f64::consts::TAU)
                < (to - from).rem_euclid(std::f64::consts::TAU)
            {
                true => (a, b),
                false => (b, a),
            };
            Some(ArcDraft { centre, start, end })
        }
    }
}

/// The cursor, once a value typed into the live field has had its say: a
/// radius or a distance while the second place is being picked, and while the
/// third is, an angle in degrees for `ByCenter` or the curve's own radius for
/// `ByEnds`. Left alone, the cursor decides everything, the way it always has.
/// `scale` is how many millimetres a unit of the drawing is worth, since a
/// typed radius or distance arrives in millimetres.
pub fn aimed(
    mode: ArcMode,
    places: &[DVec2],
    cursor: DVec2,
    locked: Option<f64>,
    scale: f64,
) -> DVec2 {
    let Some(locked) = locked else {
        return cursor;
    };
    match (mode, places.len()) {
        (ArcMode::ByCenter, 2) => {
            let (centre, start) = (places[0], places[1]);
            let radius = centre.distance(start);
            if radius < 1e-9 {
                return cursor;
            }
            let base = (start - centre).to_angle();
            centre + DVec2::from_angle(base + locked.to_radians()) * radius
        }
        (ArcMode::ByEnds, 2) => {
            bent_through(places[0], places[1], cursor, locked / scale.max(1e-9)).unwrap_or(cursor)
        }
        (ArcMode::ByCenter, 1) | (ArcMode::ByEnds, 1) => {
            let radius = locked / scale.max(1e-9);
            match radius > 1e-9 {
                true => places[0] + (cursor - places[0]).normalize_or(DVec2::X) * radius,
                false => cursor,
            }
        }
        _ => cursor,
    }
}

/// The leg a typed angle opens from, as the two places it runs between: the
/// centre, and the end already picked, which is the direction [`aimed`]
/// measures those degrees against. `None` wherever nothing is being read as an
/// angle, so that a canvas asking what to draw is told rather than guessing.
pub fn angle_reference(mode: ArcMode, places: &[DVec2]) -> Option<(DVec2, DVec2)> {
    match (mode, places.len()) {
        (ArcMode::ByCenter, 2) => Some((places[0], places[1])),
        _ => None,
    }
}

/// The place a curve of that radius, hung on those two ends, has to be bent
/// through. The cursor keeps everything the radius does not say: which side of
/// the chord the curve bulges to, and — by standing further off the chord than
/// half of it is long — whether it takes the short way round or the long one.
///
/// `None` when no such curve exists: a radius shorter than half the chord
/// reaches neither end, and a cursor on the chord itself names no side.
fn bent_through(a: DVec2, b: DVec2, cursor: DVec2, radius: f64) -> Option<DVec2> {
    let chord = b - a;
    let half = chord.length() / 2.0;
    let middle = (a + b) / 2.0;
    let across = chord.perp().normalize_or_zero();
    let side = (cursor - middle).dot(across);
    if half < 1e-9 || radius < half || side.abs() < 1e-9 {
        return None;
    }
    let reach = (radius * radius - half * half).sqrt();
    let rise = match side.abs() > half {
        true => radius + reach,
        false => radius - reach,
    };
    Some(middle + across * side.signum() * rise)
}

#[cfg(test)]
mod tests;
