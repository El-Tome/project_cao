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

/// How far apart two drafts may stand, against the size of the curve, and
/// still describe the same one.
const SAME_CURVE: f64 = 1e-9;

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

    /// A place in the ellipse's own measure, where the curve itself is the
    /// circle of radius one about the origin.
    pub(crate) fn squashed(&self, place: DVec2) -> DVec2 {
        let out = place - self.centre;
        let along = self.first.length();
        DVec2::new(
            out.dot(self.first) / (along * along),
            out.dot(self.second_axis()) / (self.second * self.second),
        )
    }

    /// How far off the curve a place stands, in that same measure: nought on
    /// it, negative inside, positive outside.
    pub(crate) fn off_by(&self, place: DVec2) -> f64 {
        self.squashed(place).length() - 1.0
    }

    /// The turn at which the ellipse passes through a place standing on it.
    ///
    /// Exact and cheap where [`Self::turn_nearest`] hunts: squashed, the curve
    /// is the unit circle, and the turn is the angle of the place. It answers
    /// for a place off the curve as the place squashed down onto it, which is
    /// not the nearest place — that is what `turn_nearest` is for.
    pub(crate) fn turn_on(&self, place: DVec2) -> f64 {
        self.squashed(place)
            .to_angle()
            .rem_euclid(std::f64::consts::TAU)
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

    /// Whether two drafts describe one and the same curve.
    ///
    /// The same ellipse has four descriptions — either axis first, either way
    /// round — and the tool hands back whichever one was drawn. Comparing the
    /// numbers as they stand would call two of them different curves, and
    /// hunting one against the other then reads rounding noise as a crossing
    /// at every step of the walk.
    pub(crate) fn is_the_curve(&self, other: &Self) -> bool {
        let scale = self.first.length().max(self.second);
        let close = |near: f64, far: f64| (near - far).abs() <= scale * SAME_CURVE;
        if self.centre.distance(other.centre) > scale * SAME_CURVE {
            return false;
        }
        let (along, across) = (self.first.length(), self.second);
        let (their_along, their_across) = (other.first.length(), other.second);
        let lined_up = |one: DVec2, two: DVec2| {
            one.perp_dot(two).abs() <= one.length() * two.length() * SAME_CURVE
        };
        let axes_alike = close(along, their_along) && close(across, their_across);
        let axes_swapped = close(along, their_across) && close(across, their_along);
        // A round one is the same curve whichever way its axes are laid.
        (axes_alike && close(along, across))
            || (axes_alike && lined_up(self.first, other.first))
            || (axes_swapped && lined_up(self.first, other.second_axis()))
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
