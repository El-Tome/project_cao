//! How a profile is read before it is turned about an axis lying in its
//! plane: whether the turn is whole, on which side of the axis the profile
//! lies, and how close to the axis counts as on it. Decided here once, for
//! both kernels, the part and the harness alike (#533).

mod sides;
mod straight;

use std::f64::consts::TAU;

use glam::DVec2;

use crate::profile::Profile;

pub use straight::{Corner, Straight};

/// How close to its axis a point of a profile counts as on it, as a share of
/// the area's furthest point from the axis: the flats' band, wide enough for
/// the drift a solver leaves on a corner drawn on the axis (#487, #488).
pub const ON_THE_AXIS: f64 = 1e-3;

/// A turn shorter than this, in radians, makes nothing and names nothing.
pub const SHORTEST: f64 = 1e-4;

/// How close to a whole turn, in radians, a turn is taken as whole.
pub const WHOLE: f64 = 1e-3;

/// An area whose furthest point from its axis is closer than this lies on
/// the axis altogether, whatever its size, and turns into nothing.
pub(crate) const THINNEST: f64 = 1e-6;

/// Whether a turn of `angle` radians, either way, closes on itself.
pub fn is_whole(angle: f64) -> bool {
    (angle.abs() - TAU).abs() < WHOLE
}

/// A line of a profile's plane, in the profile's own coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Axis {
    pub origin: DVec2,
    pub direction: DVec2,
}

impl Axis {
    /// How far a point stands from the axis, positive on the left of its
    /// direction; nought for every point when the axis has no direction.
    pub fn side(&self, point: DVec2) -> f64 {
        self.direction
            .normalize_or_zero()
            .perp_dot(point - self.origin)
    }
}

/// A turn of a profile about an axis, and the tolerances it is read at.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Turn {
    pub axis: Axis,
    /// Signed radians, at most a whole turn either way.
    pub angle: f64,
    /// How far apart the drawing holds two points that should be one; nought
    /// where nothing was drawn.
    pub resolution: f64,
    /// How close to the axis a point counts as on it. Read off the whole
    /// area, and kept for every piece the area is cut into.
    pub on_the_axis: f64,
}

/// Where a profile lies against its axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lie {
    /// On one side, `+1.0` or `-1.0` as [`Axis::side`] signs it, its points
    /// within the band taken as on the axis.
    Side(f64),
    /// On both sides, past the band: turned whole it would sweep through
    /// itself, so each side is turned apart.
    Across,
    /// Too thin, or turned too short, to make anything.
    Nothing,
}

impl Turn {
    /// The turn of `area` by `angle` radians about `axis`, its band read off
    /// the area's furthest point from the axis.
    pub fn of(axis: Axis, angle: f64, resolution: f64, area: &Profile) -> Turn {
        Turn {
            axis,
            angle,
            resolution,
            on_the_axis: ON_THE_AXIS * furthest(axis, area),
        }
    }

    pub fn is_whole(&self) -> bool {
        is_whole(self.angle)
    }

    /// Where `area` lies against the axis, read on every point it was sampled
    /// into and every corner it was drawn with.
    pub fn lie(&self, area: &Profile) -> Lie {
        if self.angle.abs() < SHORTEST || furthest(self.axis, area) < THINNEST {
            return Lie::Nothing;
        }
        let mut beyond = points(area)
            .map(|point| self.axis.side(point))
            .filter(|side| side.abs() > self.on_the_axis)
            .map(f64::signum);
        let Some(sign) = beyond.next() else {
            return Lie::Nothing;
        };
        if beyond.all(|each| each == sign) {
            Lie::Side(sign)
        } else {
            Lie::Across
        }
    }

    /// The point laid on the axis when it stands within the band, where the
    /// drawing can only have meant it; left where it is beyond.
    pub fn on_axis(&self, point: DVec2) -> DVec2 {
        if self.axis.side(point).abs() > self.on_the_axis {
            return point;
        }
        let along = self.axis.direction.normalize_or_zero();
        self.axis.origin + along * along.dot(point - self.axis.origin)
    }
}

fn furthest(axis: Axis, area: &Profile) -> f64 {
    points(area)
        .map(|point| axis.side(point).abs())
        .fold(0.0, f64::max)
}

/// Every point the area was sampled into, its holes' too, and every corner
/// it was drawn with: a profile handed exactly, with nothing sampled, is
/// still read.
fn points<'a>(area: &'a Profile) -> impl Iterator<Item = DVec2> + 'a {
    let sampled = std::iter::once(area.sampled)
        .chain(area.sampled_holes.iter().copied())
        .flat_map(|each| each.points.iter().copied());
    let exact = area
        .exact
        .iter()
        .flat_map(|(outline, holes)| std::iter::once(outline).chain(holes))
        .flat_map(|contour| contour.corners.iter().copied());
    sampled.chain(exact)
}

#[cfg(test)]
mod tests;
