//! A profile raised along its plane's normal into a body.

mod piece;
mod walls;

use std::f64::consts::TAU;

use glam::DVec3;

use super::Declined;
use super::curve::Circle;
use super::scale::Scale;
use super::topology::{Body, turning_points};
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
        let lifted = if backwards {
            Frame {
                v: -frame.v,
                ..frame
            }
        } else {
            frame
        };
        let lift = lifted.normal() * height.abs() / normal.length_squared();
        let scale = Scale::of(reach(outline, holes, frame, lift));
        let eps = scale.eps();
        if lift.length() <= eps {
            return Err(Declined::Travel);
        }
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
        Ok(walls::raise(lifted, lift, &contours, eps))
    }
}

/// The largest coordinate the body will reach, at either end: its corners and
/// the points where an arc turns back in a coordinate. Read exactly, so that
/// the profile is decided at the tolerance of the body it makes.
fn reach(outline: &Contour, holes: &[Contour], frame: Frame, lift: DVec3) -> f64 {
    let mut points = Vec::new();
    for contour in std::iter::once(outline).chain(holes) {
        for (corner, run) in contour.corners.iter().zip(&contour.runs) {
            points.push(frame.at(*corner));
            if let Run::Round { center, turn } = *run
                && turn.abs() < TAU
            {
                let away = *corner - center;
                let circle = Circle {
                    center: frame.at(center),
                    axis: frame.normal(),
                    radius: away.length(),
                    u: frame.u,
                    v: frame.v,
                };
                let start = away.y.atan2(away.x);
                let (from, to) = (start.min(start + turn), start.max(start + turn));
                points.extend(turning_points(&circle, from, to));
            }
        }
    }
    points
        .iter()
        .flat_map(|point| [*point, *point + lift])
        .map(|point| point.abs().max_element())
        .fold(0.0, f64::max)
}

#[cfg(test)]
mod tests;
