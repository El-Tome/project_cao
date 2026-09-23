//! Which ellipse the clicks gathered so far mean, once what was typed has had
//! its say.
//!
//! The sibling of [`crate::arc_placing`]: the drawing says what a run of
//! clicks adds up to, and the front-end only hands over the places they landed
//! on and the values typed at the cursor.

use glam::DVec2;

use crate::aim::LockedInput;
use crate::ellipsing::EllipseDraft;

/// How an ellipse is being drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EllipseMode {
    /// The centre, the end of the first axis, then how far the second reaches.
    #[default]
    ByCentre,
    /// The two ends of the first axis, then how far the curve rises from them.
    ///
    /// What this lays is half a curve: the ellipse those three say, with the
    /// half on the far side of the first axis taken away.
    ByEnds,
}

impl EllipseMode {
    /// How many places it needs before the ellipse is settled.
    pub fn wants(self) -> usize {
        match self {
            Self::ByCentre | Self::ByEnds => 3,
        }
    }
}

/// How many places the ellipse tool needs: its centre, the end of its first
/// axis, and how far the second reaches.
pub const ELLIPSE_PLACES: usize = 3;

/// Where the next click of the ellipse tool lands, the cursor bent by what was
/// typed.
///
/// With the centre alone given, the first axis is being drawn: a width typed
/// fixes how far it reaches across the whole curve, an angle typed which way
/// it runs from the plane's first axis. With the first axis given, the second
/// is: a width typed fixes how far it reaches, on whichever side of the first
/// the cursor stands. Widths come in millimetres, `scale` being how many a
/// unit of the drawing is worth.
pub fn ellipse_aimed(
    mode: EllipseMode,
    places: &[DVec2],
    cursor: DVec2,
    locked: LockedInput,
    scale: f64,
) -> DVec2 {
    let scale = scale.max(1e-9);
    match (mode, places) {
        // Placed from its two ends, the first length typed is the whole axis
        // rather than half of it: it is the gap between the two clicks, which
        // is what one has a figure for. The rise typed at the third click is
        // the half-axis, for the same reason.
        (EllipseMode::ByEnds, [from]) => {
            let span = cursor - *from;
            let reach = match locked.first {
                Some(width) => width / scale,
                None => span.length(),
            };
            let direction = match locked.second {
                Some(degrees) => DVec2::from_angle(degrees.to_radians()),
                None => match span.try_normalize() {
                    Some(direction) => direction,
                    None => return cursor,
                },
            };
            *from + direction * reach
        }
        (EllipseMode::ByEnds, [from, to]) => {
            let middle = from.midpoint(*to);
            let Some(across) = (*to - middle).perp().try_normalize() else {
                return cursor;
            };
            let rise = (cursor - middle).dot(across);
            let rise = match locked.first {
                Some(typed) => (typed / scale).copysign(rise),
                None => rise,
            };
            middle + across * rise
        }
        (EllipseMode::ByCentre, [centre]) => {
            let span = cursor - *centre;
            let reach = match locked.first {
                Some(width) => width * 0.5 / scale,
                None => span.length(),
            };
            let direction = match locked.second {
                Some(degrees) => DVec2::from_angle(degrees.to_radians()),
                None => match span.try_normalize() {
                    Some(direction) => direction,
                    None => return cursor,
                },
            };
            *centre + direction * reach
        }
        (EllipseMode::ByCentre, [centre, first_end]) => {
            let Some(across) = (*first_end - *centre).perp().try_normalize() else {
                return cursor;
            };
            let reach = (cursor - *centre).dot(across);
            let reach = match locked.first {
                Some(width) => (width * 0.5 / scale).copysign(reach),
                None => reach,
            };
            *centre + across * reach
        }
        _ => cursor,
    }
}

/// The ellipse the places given and the cursor make, once enough of them are
/// known: the centre and one end of the first axis, or both of its ends.
pub fn ellipse_from(mode: EllipseMode, places: &[DVec2], cursor: DVec2) -> Option<EllipseDraft> {
    match (mode, places) {
        (EllipseMode::ByCentre, [centre, first_end]) => {
            EllipseDraft::through(*centre, *first_end, cursor)
        }
        (EllipseMode::ByEnds, [from, to]) => EllipseDraft::through(from.midpoint(*to), *to, cursor),
        _ => None,
    }
}

/// Which half of the curve a placement by its two ends draws, as the two ends
/// of the first axis in the order the stretch runs between them: `[0, 1]` from
/// that axis's start round to its end, `[1, 0]` the other way.
///
/// The curve bulges to the side the rise fell on, and a stretch always runs
/// the way a turn grows — which from the axis's end passes the side the second
/// axis itself points to. So the side is what decides the order.
pub fn half_between(drawn: EllipseDraft, rise: DVec2) -> [usize; 2] {
    match (rise - drawn.centre).dot(drawn.second_axis()) < 0.0 {
        true => [0, 1],
        false => [1, 0],
    }
}

#[cfg(test)]
mod tests;
