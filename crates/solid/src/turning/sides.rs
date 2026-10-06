//! Which sides of an area cut along its axis a turn needs. Turned whole, a
//! side whose mirror lies within another side sweeps nothing that side does
//! not sweep already, and joining the two is the flats' slowest work.

use glam::DVec2;

use super::Turn;
use crate::profile::Profile;

/// How near a straight side may come to the other side's outline and still
/// count as within it, as a share of their furthest point from the axis:
/// rounding, nothing a drawing means.
const ROUNDING: f64 = 1e-9;

impl Turn {
    /// The sides worth turning, by their index among `sides`, in order: every
    /// side when the turn is partial, and turned whole, every side but those
    /// another side's turn holds already. Of two sides mirroring each other,
    /// the first is kept.
    ///
    /// A side with a hole is always turned: the other side may fill it.
    pub fn sides_needed(&self, sides: &[Profile]) -> Vec<usize> {
        if !self.is_whole() {
            return (0..sides.len()).collect();
        }
        let mut kept: Vec<usize> = Vec::new();
        for (index, side) in sides.iter().enumerate() {
            if kept.iter().any(|&other| self.holds(&sides[other], side)) {
                continue;
            }
            kept.retain(|&other| !self.holds(side, &sides[other]));
            kept.push(index);
        }
        kept.sort_unstable();
        kept
    }

    /// Whether `outer` turned whole sweeps all `inner` does: drawn as the
    /// distance from the axis against the way along it, `inner` lies within
    /// `outer`. A side sampled from a curve is the flats', drawn within their
    /// band of the axis; a straight one is held within rounding.
    fn holds(&self, outer: &Profile, inner: &Profile) -> bool {
        let solid = |side: &Profile| {
            side.sampled_holes.is_empty()
                && side
                    .exact
                    .as_ref()
                    .is_none_or(|(_, holes)| holes.is_empty())
                && side.sampled.points.len() >= 3
        };
        if !solid(outer) || !solid(inner) {
            return false;
        }
        let outer_drawn = self.drawn(outer);
        let inner_drawn = self.drawn(inner);
        let curved = |side: &Profile| side.sampled.curves.iter().any(|curve| curve.is_some());
        let room = if curved(outer) || curved(inner) {
            self.on_the_axis
        } else {
            let reach = outer_drawn
                .iter()
                .chain(&inner_drawn)
                .map(|point| point.x)
                .fold(0.0, f64::max);
            self.resolution.max(ROUNDING * reach)
        };

        let within = |point: DVec2| {
            encloses(&outer_drawn, point) || from_outline(&outer_drawn, point) <= room
        };
        let crossing = segments(&inner_drawn)
            .any(|(a, b)| segments(&outer_drawn).any(|(c, d)| cross(a, b, c, d, room)));
        segments(&inner_drawn).all(|(a, b)| within(a) && within((a + b) / 2.0)) && !crossing
    }

    /// The side's outline as the turn draws it: each point's distance from
    /// the axis, and how far along the axis it stands.
    fn drawn(&self, side: &Profile) -> Vec<DVec2> {
        let along = self.axis.direction.normalize_or_zero();
        side.sampled
            .points
            .iter()
            .map(|point| {
                DVec2::new(
                    self.axis.side(*point).abs(),
                    along.dot(*point - self.axis.origin),
                )
            })
            .collect()
    }
}

fn segments(points: &[DVec2]) -> impl Iterator<Item = (DVec2, DVec2)> + '_ {
    points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(a, b)| (*a, *b))
}

fn encloses(outline: &[DVec2], point: DVec2) -> bool {
    segments(outline)
        .filter(|(a, b)| (a.y > point.y) != (b.y > point.y))
        .filter(|(a, b)| point.x < a.x + (point.y - a.y) / (b.y - a.y) * (b.x - a.x))
        .count()
        % 2
        == 1
}

fn from_outline(outline: &[DVec2], point: DVec2) -> f64 {
    segments(outline)
        .map(|(a, b)| {
            let run = b - a;
            let along = (point - a).dot(run) / run.length_squared().max(f64::MIN_POSITIVE);
            point.distance(a + run * along.clamp(0.0, 1.0))
        })
        .fold(f64::INFINITY, f64::min)
}

/// Whether the segments cross each other through both, each end of either
/// further than `room` from the other's line.
fn cross(a: DVec2, b: DVec2, c: DVec2, d: DVec2, room: f64) -> bool {
    let off = |from: DVec2, to: DVec2, point: DVec2| {
        let run = to - from;
        run.perp_dot(point - from) / run.length().max(f64::MIN_POSITIVE)
    };
    let apart = |first: f64, second: f64| {
        (first > room && second < -room) || (first < -room && second > room)
    };
    apart(off(c, d, a), off(c, d, b)) && apart(off(a, b, c), off(a, b, d))
}

#[cfg(test)]
mod tests;
