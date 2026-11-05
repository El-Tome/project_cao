//! The curve a cone meets another wall along, evaluated on one lap: the
//! angle and the length along the ruling at a parameter, and their speeds.

use glam::{DVec2, DVec3};

use super::{ACROSS_A_NODE, Lap, NEAR, Ruled, angle_between, solved};

impl Ruled {
    /// The angle at `t` on lap `lap`, its derivative, whether `t` runs the
    /// half of the span where `sin t ≥ 0`, and how far the angle stands
    /// from either end of the span: `2h sin²(t/2)` and `2h cos²(t/2)`, free
    /// of the cancellation `m − h cos t` leaves near an end.
    fn angle(&self, lap: &Lap, t: f64) -> (f64, f64, bool, [f64; 2]) {
        match lap.span {
            Some([from, to]) => {
                let half = (to - from) / 2.0;
                let (s, c) = (t / 2.0).sin_cos();
                let (low, high) = (2.0 * half * s * s, 2.0 * half * c * c);
                let theta = if low <= high { from + low } else { to - high };
                (theta, 2.0 * half * s * c, s * c >= 0.0, [low, high])
            }
            None => (lap.start + t, 1.0, true, [f64::INFINITY; 2]),
        }
    }

    /// `√D` signed at `t` on lap `lap`, and where an end of the span stands
    /// nearer than `NEAR`, `θ'/√D` too: both read off the end's Taylor term,
    /// whose ratio holds no nought over nought.
    fn root_at(&self, lap: &Lap, t: f64) -> (f64, f64, f64, Option<f64>) {
        let (theta, theta1, positive, [low, high]) = self.angle(lap, t);
        let sign = self.sign(lap, theta, positive);
        let Some([from, to]) = lap.span else {
            return (theta, theta1, self.signed_root(theta, sign), None);
        };
        if low.min(high) >= NEAR {
            return (theta, theta1, self.signed_root(theta, sign), None);
        }
        let (zero, off, share) = if low <= high {
            (from, low, (t / 2.0).cos())
        } else {
            (to, high, (t / 2.0).sin())
        };
        let half = (to - from) / 2.0;
        let slope = self.discriminant(zero)[1].abs();
        let parity = if positive { sign } else { -sign };
        let ratio = 2.0 * half * share.abs() / (parity * (2.0 * half * slope).sqrt());
        (theta, theta1, sign * (slope * off).sqrt(), Some(ratio))
    }

    /// The angle, length and their derivatives in `t` on lap `lap`.
    pub fn local(&self, lap: &Lap, t: f64) -> [DVec2; 2] {
        let (theta, theta1, root, ratio) = self.root_at(lap, t);
        let terms = self.terms(theta);
        let l = solved(&terms, root);
        let pull = terms.a[1] * l * l + terms.b[1] * l + terms.c[1];
        let by_a_node = self
            .doubles
            .iter()
            .any(|double| angle_between(theta, *double) < ACROSS_A_NODE);
        let l1 = match ratio {
            Some(ratio) => -pull * ratio,
            None if root != 0.0 && !by_a_node => -pull * theta1 / root,
            None => {
                let step = 1e-7;
                let around = |t: f64| {
                    let (theta, _, root, _) = self.root_at(lap, t);
                    solved(&self.terms(theta), root)
                };
                (around(t + step) - around(t - step)) / (2.0 * step)
            }
        };
        [DVec2::new(theta, l), DVec2::new(theta1, l1)]
    }

    pub fn point_of(&self, at: DVec2) -> DVec3 {
        self.cone.point(at)
    }

    /// The point at `(θ, l)` moved along `(θ', l')`.
    pub fn speed_of(&self, [at, rate]: [DVec2; 2]) -> DVec3 {
        let (sin, cos) = at.x.sin_cos();
        let turned = self.cone.v * cos - self.cone.u * sin;
        let [d, _] = self.ruling(at.x);
        turned * self.cone.radius_at(at.y) * rate.x + d * rate.y
    }
}
