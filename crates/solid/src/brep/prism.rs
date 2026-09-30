//! A profile raised along its plane's normal into a body.

mod piece;
mod walls;

use glam::DVec3;

use super::Declined;
use super::scale::Scale;
use super::topology::Body;
use crate::profile::{Contour, Frame, Run};
use piece::Piece;

/// How far off its plane's normal a travel may lean, relative to its length,
/// and still be taken as square to the plane.
const SQUARE: f64 = 1e-12;

impl Body {
    /// A profile pushed along `travel`, which must stand square to its plane.
    ///
    /// The outline is turned anticlockwise and the holes clockwise about the
    /// travel whatever way they were drawn, so that a profile raised backwards
    /// is the same solid as the one raised forwards from the far end.
    pub fn raised(
        outline: &Contour,
        holes: &[Contour],
        frame: Frame,
        travel: DVec3,
    ) -> Result<Body, Declined> {
        if !frame.origin.is_finite() {
            return Err(Declined::Profile);
        }
        let normal = frame.normal();
        let height = travel.dot(normal);
        let lean = travel.cross(normal).length();
        if !(height.is_finite()
            && height != 0.0
            && lean <= SQUARE * travel.length() * normal.length())
        {
            return Err(Declined::Travel);
        }
        let backwards = height < 0.0;
        let scale = Scale::of(reach(outline, holes, frame, travel));
        let eps = scale.eps();
        let read = |contour: &Contour, anticlockwise: bool| -> Result<Vec<Piece>, Declined> {
            let mut pieces = piece::pieces(contour, eps)?;
            if backwards {
                pieces = pieces.iter().map(Piece::mirrored).collect();
            }
            if piece::signed_area(&pieces).abs() <= eps * scale.reach() {
                return Err(Declined::Profile);
            }
            Ok(piece::turned(pieces, anticlockwise))
        };
        let mut contours = vec![read(outline, true)?];
        for hole in holes {
            contours.push(read(hole, false)?);
        }
        let lifted = if backwards {
            Frame {
                v: -frame.v,
                ..frame
            }
        } else {
            frame
        };
        let lift = lifted.normal() * height.abs() / normal.length_squared();
        Ok(walls::raise(lifted, lift, &contours, eps))
    }
}

/// The largest coordinate the profile's corners reach at either end, and the
/// boxes of the circles its arcs run on: the reach the profile is read at.
fn reach(outline: &Contour, holes: &[Contour], frame: Frame, travel: DVec3) -> f64 {
    let mut reach: f64 = 0.0;
    for contour in std::iter::once(outline).chain(holes) {
        for (corner, run) in contour.corners.iter().zip(&contour.runs) {
            let (center, radius) = match *run {
                Run::Straight => (*corner, 0.0),
                Run::Round { center, .. } => (center, corner.distance(center)),
            };
            let (corner, center) = (frame.at(*corner), frame.at(center));
            for shift in [DVec3::ZERO, travel] {
                reach = reach
                    .max((corner + shift).abs().max_element())
                    .max((center + shift).abs().max_element() + radius);
            }
        }
    }
    reach
}

#[cfg(test)]
mod tests;
