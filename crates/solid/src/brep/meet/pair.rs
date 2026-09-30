//! Two perpendicular cylinders read in the frame of the first: the span the
//! curve they meet along covers, where each root vanishes, and the roots
//! themselves with their derivatives.

use std::f64::consts::TAU;

use glam::DVec3;

use crate::brep::surface::Cylinder;

/// Below this, relative to the size of the pair, a gap is rounding and not a
/// decision. The decision within the kernel's tolerance is taken once, by
/// `Meeting::of`, which moves the second cylinder onto what it decided: what
/// is left here is the rounding of that move.
const ROUNDING: f64 = 256.0 * f64::EPSILON;

/// How two perpendicular cylinders meet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Configuration {
    Apart,
    /// A single point, where they touch from outside.
    Contact,
    OneLoop,
    /// One cylinder passes right through the other.
    TwoLoops,
    /// One cylinder inside the other, touching it along a side: two loops
    /// through a node, run as one curve.
    FigureOfEight,
    /// Equal radii and axes meeting: two ellipses crossing at two nodes.
    TwoEllipses,
}

pub(super) struct Pair {
    pub origin: DVec3,
    pub axes: [DVec3; 3],
    pub a: f64,
    pub b: f64,
    pub d: f64,
    pub e: f64,
    pub low: f64,
    pub high: f64,
    /// How far each root is from vanishing at the low end and at the high
    /// end: nought exactly where it does.
    pub x_gaps: [f64; 2],
    pub z_gaps: [f64; 2],
    pub rounding: f64,
}

impl Pair {
    pub fn of(first: &Cylinder, second: &Cylinder) -> Pair {
        let third = first.axis;
        let one = (second.axis - third * third.dot(second.axis)).normalize();
        let two = third.cross(one);
        let between = second.origin - first.origin;
        let (a, b) = (first.radius, second.radius);
        let (d, e) = (between.dot(two), between.dot(third));
        let rounding = ROUNDING * (1.0 + first.origin.length() + second.origin.length() + a + b);

        let (mut low, mut high) = ((-a).max(d - b), a.min(d + b));
        let mut x_gaps = [low + a, a - high];
        let mut z_gaps = [low - (d - b), (d + b) - high];
        if x_gaps[0].max(z_gaps[0]) <= rounding {
            low = -a;
            x_gaps[0] = 0.0;
            z_gaps[0] = 0.0;
        }
        if x_gaps[1].max(z_gaps[1]) <= rounding {
            high = a;
            x_gaps[1] = 0.0;
            z_gaps[1] = 0.0;
        }
        Pair {
            origin: first.origin,
            axes: [one, two, third],
            a,
            b,
            d,
            e,
            low,
            high,
            x_gaps,
            z_gaps,
            rounding,
        }
    }

    pub fn x_vanishes(&self) -> [bool; 2] {
        self.x_gaps.map(|gap| gap == 0.0)
    }

    pub fn z_vanishes(&self) -> [bool; 2] {
        self.z_gaps.map(|gap| gap == 0.0)
    }

    pub fn configuration(&self) -> Configuration {
        let span = self.high - self.low;
        if span < -self.rounding {
            return Configuration::Apart;
        }
        if span <= self.rounding {
            return Configuration::Contact;
        }
        let (x, z) = (self.x_vanishes(), self.z_vanishes());
        match [x[0] && z[0], x[1] && z[1]] {
            [true, true] => Configuration::TwoEllipses,
            [true, false] | [false, true] => Configuration::FigureOfEight,
            _ if x == [true, true] || z == [true, true] => Configuration::TwoLoops,
            _ => Configuration::OneLoop,
        }
    }

    pub fn components(&self) -> u8 {
        match self.configuration() {
            Configuration::Apart | Configuration::Contact => 0,
            Configuration::OneLoop | Configuration::FigureOfEight => 1,
            Configuration::TwoLoops | Configuration::TwoEllipses => 2,
        }
    }

    /// A root vanishing at one end only changes sign there and comes back
    /// after two turns of the half-angle.
    pub fn period(&self) -> f64 {
        let once = |ends: [bool; 2]| ends[0] != ends[1];
        if once(self.x_vanishes()) || once(self.z_vanishes()) {
            2.0 * TAU
        } else {
            TAU
        }
    }

    /// The signs of `x` and of `z − e` on a component. The second component
    /// takes the other side of the root that never vanishes, or of `z − e`.
    pub fn signs(&self, component: u8) -> [f64; 2] {
        match component {
            0 => [1.0, 1.0],
            _ if self.x_vanishes() == [false, false] => [-1.0, 1.0],
            _ => [1.0, -1.0],
        }
    }

    /// `(x, y, z)` at `t`, with its first and second derivatives.
    pub fn local(&self, signs: [f64; 2], t: f64) -> [DVec3; 3] {
        let half = (self.high - self.low) / 2.0;
        let middle = (self.high + self.low) / 2.0;
        let angles = Angles::at(t);
        let y = [
            middle - half * angles.cos,
            half * angles.sin,
            half * angles.cos,
        ];
        let x = root(self.x_gaps, half, &angles);
        let z = root(self.z_gaps, half, &angles);
        let [sx, sz] = signs;
        [
            DVec3::new(sx * x[0], y[0], self.e + sz * z[0]),
            DVec3::new(sx * x[1], y[1], sz * z[1]),
            DVec3::new(sx * x[2], y[2], sz * z[2]),
        ]
    }

    pub fn world(&self, local: DVec3) -> DVec3 {
        self.origin + self.direction(local)
    }

    pub fn direction(&self, local: DVec3) -> DVec3 {
        self.axes[0] * local.x + self.axes[1] * local.y + self.axes[2] * local.z
    }

    pub fn local_of(&self, point: DVec3) -> DVec3 {
        let from = point - self.origin;
        DVec3::new(
            from.dot(self.axes[0]),
            from.dot(self.axes[1]),
            from.dot(self.axes[2]),
        )
    }
}

struct Angles {
    sin: f64,
    cos: f64,
    half_sin: f64,
    half_cos: f64,
}

impl Angles {
    fn at(t: f64) -> Angles {
        let (sin, cos) = t.sin_cos();
        let (half_sin, half_cos) = (t / 2.0).sin_cos();
        Angles {
            sin,
            cos,
            half_sin,
            half_cos,
        }
    }
}

/// A root `√((g_hi + hi − y)(g_lo + y − lo))`, signed so that it runs on
/// through an end where its gap vanishes, with its two derivatives.
fn root(gaps: [f64; 2], half: f64, angles: &Angles) -> [f64; 3] {
    let k = (2.0 * half).sqrt();
    let Angles {
        sin,
        cos,
        half_sin: s,
        half_cos: c,
    } = *angles;
    let towards_high = if gaps[1] == 0.0 {
        [k * c, -k * s / 2.0, -k * c / 4.0]
    } else {
        smooth(gaps[1] + 2.0 * half * c * c, -half * sin, -half * cos)
    };
    let towards_low = if gaps[0] == 0.0 {
        [k * s, k * c / 2.0, -k * s / 4.0]
    } else {
        smooth(gaps[0] + 2.0 * half * s * s, half * sin, half * cos)
    };
    let [p, p1, p2] = towards_high;
    let [q, q1, q2] = towards_low;
    [p * q, p1 * q + p * q1, p2 * q + 2.0 * p1 * q1 + p * q2]
}

/// The square root of a positive function, from its value and derivatives.
fn smooth(value: f64, first: f64, second: f64) -> [f64; 3] {
    let root = value.sqrt();
    let slope = first / (2.0 * root);
    [root, slope, (second - 2.0 * slope * slope) / (2.0 * root)]
}
