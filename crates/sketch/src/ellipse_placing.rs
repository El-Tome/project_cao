//! Which ellipse the clicks gathered so far mean, once what was typed has had
//! its say.
//!
//! The sibling of [`crate::arc_placing`]: the drawing says what a run of
//! clicks adds up to, and the front-end only hands over the places they landed
//! on and the values typed at the cursor.

use glam::DVec2;

use crate::aim::LockedInput;
use crate::ellipsing::EllipseDraft;

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
pub fn ellipse_aimed(places: &[DVec2], cursor: DVec2, locked: LockedInput, scale: f64) -> DVec2 {
    let scale = scale.max(1e-9);
    match places {
        [centre] => {
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
        [centre, first_end] => {
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

/// The ellipse the places given and the cursor make, once the centre and the
/// end of the first axis are known.
pub fn ellipse_from(places: &[DVec2], cursor: DVec2) -> Option<EllipseDraft> {
    match places {
        [centre, first_end] => EllipseDraft::through(*centre, *first_end, cursor),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
