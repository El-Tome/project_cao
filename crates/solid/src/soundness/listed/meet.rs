//! The curve two walls meet along, an elliptic one among them (#571), at
//! any angle (#577), as a listing gives it, read by its own formula: the
//! height across both axes its parameter stands for, and at that height
//! each wall's chord solved from the wall's own quadratic — its middle
//! `−b/a` and its half `√(b² − ac)/a`, the discriminant read through the
//! half-widths so that it keeps its digits at an end of the span — not the
//! kernel's shear and scale, so that a slip in one is not checked by the
//! same slip. At a slant, the second wall's chord runs across its own axis,
//! and the point is found along the first's axis where the second's
//! coordinate across its chords is met. The parameter keeps the kernel's
//! convention, as a conic's does: the height `m − l cos t`, each root
//! changing sign through an end where it vanishes.

use glam::DVec3;

use crate::brep::{Cylinder, EllipticCylinder, Meet, Scale, Wall};

/// Below this, relative to the size of the pair, an end of the span is
/// where both walls reach: the kernel's own reading, since which ends a
/// root vanishes at is the convention of the parameter, not a measure.
const ROUNDING: f64 = 256.0 * f64::EPSILON;

/// The place an elliptic meet reaches at its parameter `at`.
/// Not a number where a wall of the pair is a cone, which the formula does
/// not read.
pub(super) fn point(meet: &Meet, at: f64) -> DVec3 {
    let (Some(one), Some(other)) = (Round::of(&meet.first), Round::of(&meet.second)) else {
        return DVec3::NAN;
    };
    let third = axis(&one);
    let along = axis(&other);
    let square = 2.0 * third.dot(along).abs() <= Scale::RELATIVE;
    let side = if square {
        (along - third * third.dot(along)).normalize()
    } else {
        normal(third, along).cross(third).normalize()
    };
    let two = third.cross(side);
    let across = if square {
        third
    } else {
        along.cross(two).normalize()
    };
    let between = origin(&other) - origin(&one);
    let (d, e) = (between.dot(two), between.dot(across));
    let [first, second] = [(&one, side), (&other, across)]
        .map(|(wall, across)| (reach(wall, two), chord(wall, across, two)));
    let (a, b) = (first.0, second.0);
    let size = 1.0 + origin(&one).length() + origin(&other).length();
    let rounding = ROUNDING * (size + extent(&one) + extent(&other));
    let (mut low, mut high) = ((-a).max(d - b), a.min(d + b));
    let mut x_gaps = [low + a, a - high];
    let mut z_gaps = [low - (d - b), (d + b) - high];
    if x_gaps[0].max(z_gaps[0]) <= rounding {
        (low, x_gaps[0], z_gaps[0]) = (-a, 0.0, 0.0);
    }
    if x_gaps[1].max(z_gaps[1]) <= rounding {
        (high, x_gaps[1], z_gaps[1]) = (a, 0.0, 0.0);
    }
    let half = (high - low) / 2.0;
    let y = (high + low) / 2.0 - half * at.cos();
    let (sin, cos) = (at / 2.0).sin_cos();
    let root = |gaps: [f64; 2]| {
        let towards = |gap: f64, share: f64| {
            if gap == 0.0 {
                (2.0 * half).sqrt() * share
            } else {
                (gap + 2.0 * half * share * share).sqrt()
            }
        };
        towards(gaps[1], cos) * towards(gaps[0], sin)
    };
    let signs = match meet.component {
        0 => [1.0, 1.0],
        _ if x_gaps.iter().all(|gap| *gap != 0.0) => [-1.0, 1.0],
        _ => [1.0, -1.0],
    };
    let ([x_middle, x_half], [z_middle, z_half]) = (first.1, second.1);
    let x = x_middle * y + signs[0] * x_half * root(x_gaps);
    let z = e + z_middle * (y - d) + signs[1] * z_half * root(z_gaps);
    if square {
        return origin(&one) + side * x + two * y + third * z;
    }
    let foot = side * x + two * y;
    origin(&one) + foot + third * ((z - foot.dot(across)) / third.dot(across))
}

/// The normal of two directions, each component's two products kept apart
/// to their rounding, so that two nearly parallel axes keep its digits.
fn normal(one: DVec3, other: DVec3) -> DVec3 {
    let minor = |a: f64, b: f64, c: f64, d: f64| {
        let kept = c * d;
        a.mul_add(b, -kept) - c.mul_add(d, -kept)
    };
    DVec3::new(
        minor(one.y, other.z, one.z, other.y),
        minor(one.z, other.x, one.x, other.z),
        minor(one.x, other.y, one.y, other.x),
    )
}

/// Whether a meet runs round an elliptic wall and no cone: a cone's curve
/// is read by the kernel's own formula (#577).
pub(super) fn elliptic(meet: &Meet) -> bool {
    let has = |wall: fn(&Wall) -> bool| wall(&meet.first) || wall(&meet.second);
    has(|wall| matches!(wall, Wall::Elliptic(_))) && !has(|wall| matches!(wall, Wall::Cone(_)))
}

/// A wall the formula reads: a cylinder or an elliptic wall.
enum Round<'a> {
    Cylinder(&'a Cylinder),
    Elliptic(&'a EllipticCylinder),
}

impl Round<'_> {
    fn of(wall: &Wall) -> Option<Round<'_>> {
        match wall {
            Wall::Cylinder(cylinder) => Some(Round::Cylinder(cylinder)),
            Wall::Elliptic(oval) => Some(Round::Elliptic(oval)),
            Wall::Cone(_) => None,
        }
    }
}

fn axis(wall: &Round) -> DVec3 {
    match wall {
        Round::Cylinder(cylinder) => cylinder.axis.normalize(),
        Round::Elliptic(oval) => oval.axis.normalize(),
    }
}

fn origin(wall: &Round) -> DVec3 {
    match wall {
        Round::Cylinder(cylinder) => cylinder.origin,
        Round::Elliptic(oval) => oval.origin,
    }
}

fn extent(wall: &Round) -> f64 {
    match wall {
        Round::Cylinder(cylinder) => cylinder.radius,
        Round::Elliptic(oval) => oval.major,
    }
}

/// How far a wall reaches from its axis along `way`, square to it.
fn reach(wall: &Round, way: DVec3) -> f64 {
    match wall {
        Round::Cylinder(cylinder) => cylinder.radius,
        Round::Elliptic(oval) => {
            let [u, v] = [oval.u.normalize(), oval.v.normalize()];
            (oval.major * way.dot(u)).hypot(oval.minor * way.dot(v))
        }
    }
}

/// The chord a wall's section cuts at the height `h` along `way`, read
/// along `across`: its middle per unit of height, and what a unit of the
/// root `√((w − h)(w + h))` is worth along it. The section is
/// `a c² + 2 b h c + c' h² = 1` in `(c, h)`, so the middle is `−b h / a`,
/// and the discriminant is `(w² − h²)/(A² B²)`, `w = A B √a`.
fn chord(wall: &Round, across: DVec3, way: DVec3) -> [f64; 2] {
    match wall {
        Round::Cylinder(_) => [0.0, 1.0],
        Round::Elliptic(oval) => {
            let [u, v] = [oval.u.normalize(), oval.v.normalize()];
            let (major, minor) = (oval.major, oval.minor);
            let [cu, cv] = [across.dot(u) / major, across.dot(v) / minor];
            let [hu, hv] = [way.dot(u) / major, way.dot(v) / minor];
            let a = cu * cu + cv * cv;
            let b = cu * hu + cv * hv;
            [-b / a, 1.0 / (major * minor * a)]
        }
    }
}
