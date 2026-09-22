//! The curve an ellipse is, once its centre and its two axes are known: where a
//! turn round it lands, how far a place stands from it, the box it fits in, and
//! the run of straight steps it is drawn as.

use glam::DVec2;

/// An ellipse as the places it stands on: the one a tool is part-way through
/// drawing, and the one already in the sketch once its points are looked up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EllipseDraft {
    pub centre: DVec2,
    /// From the centre to one end of the first axis: its direction and its
    /// half-length together.
    pub first: DVec2,
    /// How far the second axis reaches either side of the centre, square to
    /// the first.
    pub second: f64,
}

/// Into how many straight steps a whole ellipse is cut. More than a circle is,
/// since a long ellipse bends hardest at the ends of its long axis.
pub(crate) const FULL_ELLIPSE_STEPS: usize = 96;

/// A reach below which an axis is taken for nothing at all.
const NO_REACH: f64 = 1e-9;

impl EllipseDraft {
    /// The ellipse a centre, the end of its first axis and a place the second
    /// reaches make: that place counts only for how far it stands square to
    /// the first axis.
    pub fn through(centre: DVec2, first_end: DVec2, second_reach: DVec2) -> Option<Self> {
        let first = first_end - centre;
        if first.length() < NO_REACH {
            return None;
        }
        let second = (second_reach - centre).dot(first.perp().normalize()).abs();
        (second >= NO_REACH).then_some(Self {
            centre,
            first,
            second,
        })
    }

    /// From the centre to one end of the second axis, a quarter turn
    /// counter-clockwise from the first.
    pub fn second_axis(&self) -> DVec2 {
        self.first.perp().normalize() * self.second
    }

    /// Where a turn round the ellipse lands, a turn of nothing being the end of
    /// the first axis and a quarter turn the end of the second.
    pub fn at(&self, turn: f64) -> DVec2 {
        self.centre + self.first * turn.cos() + self.second_axis() * turn.sin()
    }

    /// The turn that lands closest to a place.
    ///
    /// Sampled first, then refined: the nearest place is where the line to it
    /// stands square to the curve, and a long ellipse has up to four such
    /// places, of which only the sampling can tell the right one.
    pub fn turn_nearest(&self, place: DVec2) -> f64 {
        const SAMPLES: usize = 64;
        let mut turn = (0..SAMPLES)
            .map(|step| std::f64::consts::TAU * step as f64 / SAMPLES as f64)
            .min_by(|a, b| {
                self.at(*a)
                    .distance_squared(place)
                    .total_cmp(&self.at(*b).distance_squared(place))
            })
            .unwrap_or(0.0);
        let second = self.second_axis();
        for _ in 0..16 {
            let (sin, cos) = turn.sin_cos();
            let off = self.at(turn) - place;
            let along = -self.first * sin + second * cos;
            let bend = -self.first * cos - second * sin;
            let slope = off.dot(along);
            let change = along.length_squared() + off.dot(bend);
            if change.abs() < 1e-18 {
                break;
            }
            let step = (slope / change).clamp(-0.5, 0.5);
            turn -= step;
            if step.abs() < 1e-14 {
                break;
            }
        }
        turn.rem_euclid(std::f64::consts::TAU)
    }

    /// The place on the curve closest to a place.
    pub fn nearest(&self, place: DVec2) -> DVec2 {
        self.at(self.turn_nearest(place))
    }

    /// How far a place stands from the curve.
    pub fn distance(&self, place: DVec2) -> f64 {
        self.nearest(place).distance(place)
    }

    /// The smallest box the curve fits in, as its two opposite corners.
    pub fn bounds(&self) -> (DVec2, DVec2) {
        let second = self.second_axis();
        let reach = DVec2::new(self.first.x.hypot(second.x), self.first.y.hypot(second.y));
        (self.centre - reach, self.centre + reach)
    }

    /// The whole curve as a closed run of places, the first repeated at the
    /// end.
    pub fn places(&self) -> Vec<DVec2> {
        (0..=FULL_ELLIPSE_STEPS)
            .map(|step| self.at(std::f64::consts::TAU * step as f64 / FULL_ELLIPSE_STEPS as f64))
            .collect()
    }
}

#[cfg(test)]
mod tests;
