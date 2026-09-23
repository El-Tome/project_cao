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

/// Which side of its first axis half a curve is drawn on, which is the side
/// the rise fell on.
///
/// Named rather than handed out as a number, so that the one place the
/// convention lives is here: a caller that had to read a sign or an index back
/// into a side would be free to read it the other way round, and a preview
/// showing the opposite half of what the click lays is green all the way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rise {
    /// The way the second axis itself points, a quarter turn counter-clockwise
    /// from the first.
    Along,
    /// The other way.
    Against,
}

/// Which side of the first axis a placement by two ends draws, the rise being
/// where the third click fell.
pub fn rise_of(drawn: EllipseDraft, rise: DVec2) -> Rise {
    match (rise - drawn.centre).dot(drawn.second_axis()) < 0.0 {
        true => Rise::Against,
        false => Rise::Along,
    }
}

impl Rise {
    /// The two ends of the first axis in the order the drawn stretch runs
    /// between them, as ranks of `[centre - first, centre + first]`.
    ///
    /// A stretch always runs the way a turn grows, and a turn of nothing is
    /// the end at `centre + first`. So the half on the second axis's own side
    /// opens there, and the other half opens at the far end.
    pub fn between(self) -> [usize; 2] {
        match self {
            Self::Along => [1, 0],
            Self::Against => [0, 1],
        }
    }

    /// The way out from the centre to where the curve reaches, which is where
    /// the second axis stops when only half a curve is drawn.
    pub fn reach(self, drawn: EllipseDraft) -> DVec2 {
        match self {
            Self::Along => drawn.second_axis(),
            Self::Against => -drawn.second_axis(),
        }
    }
}

#[cfg(test)]
mod tests;
