//! The curve two perpendicular cylinders meet along, seen in the parameters
//! `(θ, h)` of either of them, `θ` unwrapped along the component so that it
//! never jumps by a turn.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::pair::Pair;
use crate::brep::curve::Meet;
use crate::brep::surface::Cylinder;

/// How a component goes round a cylinder's axis: facing one way, never
/// reaching the opposite side, or winding round it with a known angle.
enum Around {
    Facing(DVec2),
    Winding(f64),
}

impl Meet {
    /// The point at `t` in the parameters `(θ, h)` of the first cylinder or of
    /// the second, with its first and second derivatives with respect to `t`.
    pub fn seen_on(&self, on_first: bool, t: f64) -> [DVec2; 3] {
        let pair = self.pair();
        let signs = pair.signs(self.component);
        let [point, speed, bend] = pair.local(signs, t);
        if on_first {
            let across = [point, speed, bend].map(|v| DVec2::new(v.x, v.y));
            let around = around_first(&pair, signs[0], t);
            let [angle, turn, sway] = angles(across, around, start(&self.first, pair.axes[0]));
            [
                DVec2::new(angle, point.z),
                DVec2::new(turn, speed.z),
                DVec2::new(sway, bend.z),
            ]
        } else {
            let mut across = [point, speed, bend].map(|v| DVec2::new(v.y, v.z));
            across[0] -= DVec2::new(pair.d, pair.e);
            let around = around_second(&pair, signs[1], t);
            let [angle, turn, sway] = angles(across, around, start(&self.second, pair.axes[1]));
            let axis = self.second.axis;
            let height = (pair.world(point) - self.second.origin).dot(axis);
            [
                DVec2::new(angle, height),
                DVec2::new(turn, pair.direction(speed).dot(axis)),
                DVec2::new(sway, pair.direction(bend).dot(axis)),
            ]
        }
    }

    /// Every parameter in the first period where the component stands at the
    /// angle `theta` of the first cylinder or of the second, sorted.
    ///
    /// Exact but for rounding: the angle fixes a point of the cylinder's
    /// circle, so a height across both axes, which the half-angle of the
    /// parameter is read off; the side of the root that does not vanish there
    /// tells which arcs pass. Where the component turns back at that angle it
    /// is found once.
    pub fn at_angle(&self, on_first: bool, theta: f64) -> Vec<f64> {
        let pair = self.pair();
        if self.component >= pair.components() {
            return Vec::new();
        }
        let signs = pair.signs(self.component);
        let period = pair.period();
        let (radius, gaps, from) = if on_first {
            (pair.a, pair.x_gaps, start(&self.first, pair.axes[0]))
        } else {
            (pair.b, pair.z_gaps, start(&self.second, pair.axes[1]))
        };
        let (sin, cos) = (theta - from).sin_cos();
        let quarter = (theta - from) / 2.0 - FRAC_PI_2 / 2.0;
        let (below, above, side) = if on_first {
            let (rising, falling) = (quarter.sin(), quarter.cos());
            (
                2.0 * radius * rising * rising - gaps[1],
                2.0 * radius * falling * falling - gaps[0],
                cos,
            )
        } else {
            let (half_sin, half_cos) = ((theta - from) / 2.0).sin_cos();
            (
                2.0 * radius * half_sin * half_sin - gaps[1],
                2.0 * radius * half_cos * half_cos - gaps[0],
                sin,
            )
        };
        if below < -pair.rounding || above < -pair.rounding {
            return Vec::new();
        }
        let half = above.max(0.0).sqrt().atan2(below.max(0.0).sqrt());
        let on_its_end = side.abs() * radius <= pair.rounding;
        let mut found: Vec<f64> = [
            2.0 * half,
            TAU - 2.0 * half,
            TAU + 2.0 * half,
            2.0 * TAU - 2.0 * half,
        ]
        .into_iter()
        .map(|t| t.rem_euclid(period))
        .map(|t| if t < period { t } else { 0.0 })
        .filter(|&t| {
            let point = pair.local(signs, t)[0];
            let root = if on_first { point.x } else { point.z - pair.e };
            on_its_end || root * side > 0.0
        })
        .collect();
        found.sort_by(f64::total_cmp);
        found.dedup();
        found
    }

    /// Every parameter in the first period where the component turns back in
    /// the angle of the first cylinder or of the second, sorted: the ends of
    /// its span where that cylinder's own root does not vanish, since the
    /// point stands still on that cylinder's circle there. Along an arc the
    /// height across both axes is monotone and the root keeps its sign, so
    /// the angle is monotone too.
    pub fn turns(&self, on_first: bool) -> Vec<f64> {
        let pair = self.pair();
        if self.component >= pair.components() {
            return Vec::new();
        }
        let period = pair.period();
        let vanishes = if on_first {
            pair.x_vanishes()
        } else {
            pair.z_vanishes()
        };
        let mut found: Vec<f64> = [0.0, PI]
            .into_iter()
            .zip(vanishes)
            .filter(|&(_, vanishes)| !vanishes)
            .flat_map(|(end, _)| [end, end + TAU])
            .filter(|&t| t < period)
            .collect();
        found.sort_by(f64::total_cmp);
        found
    }
}

/// The first cylinder is `x² + y² = a²`: the component winds round it when
/// `x` vanishes at both ends of the span, where `(x, y) = a (±sin t, −cos t)`.
fn around_first(pair: &Pair, sign: f64, t: f64) -> Around {
    match pair.x_vanishes() {
        [true, true] => Around::Winding(sign * t - FRAC_PI_2),
        [false, true] => Around::Facing(DVec2::Y),
        [true, false] => Around::Facing(DVec2::NEG_Y),
        [false, false] => Around::Facing(DVec2::new(sign, 0.0)),
    }
}

/// The second is `(y − d)² + (z − e)² = b²`: the component winds round it
/// when `z − e` vanishes at both ends, where `(y − d, z − e) = b (−cos t, ±sin t)`.
fn around_second(pair: &Pair, sign: f64, t: f64) -> Around {
    match pair.z_vanishes() {
        [true, true] => Around::Winding(sign * (PI - t)),
        [false, true] => Around::Facing(DVec2::X),
        [true, false] => Around::Facing(DVec2::NEG_X),
        [false, false] => Around::Facing(DVec2::new(0.0, sign)),
    }
}

/// Where a cylinder's angle stands for the direction of the frame its circle
/// is read in.
fn start(cylinder: &Cylinder, direction: DVec3) -> f64 {
    direction.dot(cylinder.v).atan2(direction.dot(cylinder.u))
}

/// The angle of a point going round the origin of its plane, with its first
/// and second derivatives, from the point's own.
fn angles(across: [DVec2; 3], around: Around, start: f64) -> [f64; 3] {
    let [point, speed, bend] = across;
    let angle = match around {
        Around::Facing(towards) => {
            towards.y.atan2(towards.x) + towards.perp_dot(point).atan2(towards.dot(point))
        }
        Around::Winding(reference) => {
            let raw = point.y.atan2(point.x);
            raw + TAU * ((reference - raw) / TAU).round()
        }
    };
    let square = point.length_squared();
    let turn = point.perp_dot(speed) / square;
    let sway = point.perp_dot(bend) / square - 2.0 * turn * point.dot(speed) / square;
    [start + angle, turn, sway]
}
