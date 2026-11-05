//! Two walls read in the frame of the first: the span the curve they meet
//! along covers, where each root vanishes, and the roots themselves with
//! their derivatives.

use std::f64::consts::TAU;

use glam::DVec3;

use super::section::{Section, read};
use crate::brep::curve::Wall;
use crate::brep::scale::Scale;

/// Below this, relative to the size of the pair, a gap is rounding and not a
/// decision. The decision within the kernel's tolerance is taken once, by
/// `Meeting::of`, which moves the second wall onto what it decided: what
/// is left here is the rounding of that move.
const ROUNDING: f64 = 256.0 * f64::EPSILON;

/// How two walls meet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Configuration {
    Apart,
    /// A single point, where they touch from outside.
    Contact,
    OneLoop,
    /// One wall passes right through the other.
    TwoLoops,
    /// One wall inside the other, touching it along a side: two loops
    /// through a node, run as one curve.
    FigureOfEight,
    /// Equal half-widths and axes meeting: two ellipses crossing at two
    /// nodes.
    TwoEllipses,
}

pub(super) struct Pair {
    pub origin: DVec3,
    pub axes: [DVec3; 3],
    /// The half-widths of the two walls along the height: each radius, or
    /// how far an elliptic wall reaches along the second axis.
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
    pub sections: [Section; 2],
    /// The direction the second wall's section is read across the chords
    /// along: the first's axis for walls square to each other.
    pub across: DVec3,
    pub slant: Option<Slant>,
}

/// How the second wall's axis leans off square to the first's: the cosine
/// and the sine of the angle between them. The third local coordinate is
/// then the second's across the chords, `ζ = z sin − x cos`, and the height
/// along the first's axis `z = (ζ + x cos) / sin`: each ruling of the first
/// wall meets the second where its section does, a shear and a scale of the
/// square pair's curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Slant {
    pub cos: f64,
    pub sin: f64,
}

impl Pair {
    pub fn of(first: &Wall, second: &Wall) -> Pair {
        let third = first.axis();
        let along = second.axis();
        let cos = third.dot(along);
        let (one, slant) = if 2.0 * cos.abs() <= Scale::RELATIVE {
            ((along - third * cos).normalize(), None)
        } else {
            let square = crossed(third, along);
            let sin = square.length();
            (square.cross(third).normalize(), Some(Slant { cos, sin }))
        };
        let two = third.cross(one);
        let across = slant.map_or(third, |Slant { cos, sin }| third * sin - one * cos);
        let between = second.origin() - first.origin();
        let (a, first_section, first_size) = read(first, one, two);
        let (b, second_section, second_size) = read(second, across, two);
        let (d, e) = (between.dot(two), between.dot(across));
        let rounding = ROUNDING
            * (1.0 + first.origin().length() + second.origin().length() + first_size + second_size);

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
            origin: first.origin(),
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
            sections: [first_section, second_section],
            across,
            slant,
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
        let (y, [x, z]) = self.roots(signs, t);
        if self.sections == [Section::Round; 2] {
            return [
                DVec3::new(x[0], y[0], self.e + z[0]),
                DVec3::new(x[1], y[1], z[1]),
                DVec3::new(x[2], y[2], z[2]),
            ];
        }
        let x = self.sections[0].across(y, x);
        let from_the_axis = [y[0] - self.d, y[1], y[2]];
        let z = match self.sections[1] {
            Section::Round => [self.e + z[0], z[1], z[2]],
            oval => {
                let [z, z1, z2] = oval.across(from_the_axis, z);
                [self.e + z, z1, z2]
            }
        };
        [
            DVec3::new(x[0], y[0], z[0]),
            DVec3::new(x[1], y[1], z[1]),
            DVec3::new(x[2], y[2], z[2]),
        ]
    }

    /// The height across both axes at `t` and the two signed roots, each
    /// with its first and second derivatives: on a circular wall the root
    /// is its coordinate across the height, `x` or `z − e`; on an elliptic
    /// one, the coordinate across the circle of its half-width that the
    /// ellipse is sheared and scaled off.
    pub fn roots(&self, signs: [f64; 2], t: f64) -> ([f64; 3], [[f64; 3]; 2]) {
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
        (y, [x.map(|value| sx * value), z.map(|value| sz * value)])
    }

    /// How far the curve reaches across the chords of the first wall, or
    /// the second, from the coordinate's own origin, where its root reaches
    /// `room` at most over heights from the wall's axis `[low, high]`: the
    /// root either way on a circle, the middles of the chords moved off it
    /// on an ellipse.
    pub fn reach_across(&self, wall: usize, room: f64, [low, high]: [f64; 2]) -> [f64; 2] {
        match self.sections[wall] {
            Section::Round => [-room, room],
            Section::Oval { shear, scale } => {
                let [one, other] = [shear * low, shear * high];
                let spread = scale * room + self.rounding;
                [one.min(other) - spread, one.max(other) + spread]
            }
        }
    }

    /// The point of the curve at the height `y`, where both roots vanish.
    pub fn at_end(&self, y: f64) -> DVec3 {
        let x = match self.sections[0] {
            Section::Round => 0.0,
            Section::Oval { shear, .. } => shear * y,
        };
        let z = match self.sections[1] {
            Section::Round => self.e,
            Section::Oval { shear, .. } => self.e + shear * (y - self.d),
        };
        self.world(DVec3::new(x, y, z))
    }

    pub fn world(&self, local: DVec3) -> DVec3 {
        self.origin + self.direction(local)
    }

    pub fn direction(&self, local: DVec3) -> DVec3 {
        self.axes[0] * local.x + self.axes[1] * local.y + self.axes[2] * self.rise(local)
    }

    /// How far a local point, or a local derivative, stands along the first
    /// wall's axis.
    pub fn rise(&self, local: DVec3) -> f64 {
        match self.slant {
            None => local.z,
            Some(Slant { cos, sin }) => (local.z + local.x * cos) / sin,
        }
    }

    pub fn local_of(&self, point: DVec3) -> DVec3 {
        let from = point - self.origin;
        DVec3::new(
            from.dot(self.axes[0]),
            from.dot(self.axes[1]),
            from.dot(self.across),
        )
    }
}

/// `one × other` to the last bits however nearly parallel the two: plainly,
/// axes 1e-5 apart lost five digits, which the height then magnified again.
fn crossed(one: DVec3, other: DVec3) -> DVec3 {
    let apart = |a: f64, b: f64, c: f64, d: f64| {
        let product = c * d;
        let rounding = (-c).mul_add(d, product);
        a.mul_add(b, -product) + rounding
    };
    DVec3::new(
        apart(one.y, other.z, one.z, other.y),
        apart(one.z, other.x, one.x, other.z),
        apart(one.x, other.y, one.y, other.x),
    )
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
