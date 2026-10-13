//! The curve two square walls meet along, an elliptic one among them (#571),
//! as a listing gives it, read by its own formula: the height across both
//! axes its parameter stands for, and at that height each wall's chord
//! solved from the wall's own quadratic — its middle `−b/a` and its half
//! `√(b² − ac)/a`, the discriminant read through the half-widths so that it
//! keeps its digits at an end of the span — not the kernel's shear and
//! scale, so that a slip in one is not checked by the same slip. The
//! parameter keeps the kernel's convention, as a conic's does: the height
//! `m − l cos t`, each root changing sign through an end where it vanishes.

use glam::DVec3;

use crate::brep::{Meet, Wall};

/// Below this, relative to the size of the pair, an end of the span is
/// where both walls reach: the kernel's own reading, since which ends a
/// root vanishes at is the convention of the parameter, not a measure.
const ROUNDING: f64 = 256.0 * f64::EPSILON;

/// The place an elliptic meet reaches at its parameter `at`.
pub(super) fn point(meet: &Meet, at: f64) -> DVec3 {
    let third = axis(&meet.first);
    let along = axis(&meet.second);
    let one = (along - third * third.dot(along)).normalize();
    let two = third.cross(one);
    let between = origin(&meet.second) - origin(&meet.first);
    let (d, e) = (between.dot(two), between.dot(third));
    let [first, second] = [(&meet.first, one), (&meet.second, third)]
        .map(|(wall, across)| (reach(wall, two), chord(wall, across, two)));
    let (a, b) = (first.0, second.0);
    let size = 1.0 + origin(&meet.first).length() + origin(&meet.second).length();
    let rounding = ROUNDING * (size + extent(&meet.first) + extent(&meet.second));
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
    origin(&meet.first) + one * x + two * y + third * z
}

/// Whether a meet runs round an elliptic wall.
pub(super) fn elliptic(meet: &Meet) -> bool {
    matches!(meet.first, Wall::Elliptic(_)) || matches!(meet.second, Wall::Elliptic(_))
}

fn axis(wall: &Wall) -> DVec3 {
    match wall {
        Wall::Cylinder(cylinder) => cylinder.axis.normalize(),
        Wall::Elliptic(oval) => oval.axis.normalize(),
    }
}

fn origin(wall: &Wall) -> DVec3 {
    match wall {
        Wall::Cylinder(cylinder) => cylinder.origin,
        Wall::Elliptic(oval) => oval.origin,
    }
}

fn extent(wall: &Wall) -> f64 {
    match wall {
        Wall::Cylinder(cylinder) => cylinder.radius,
        Wall::Elliptic(oval) => oval.major,
    }
}

/// How far a wall reaches from its axis along `way`, square to it.
fn reach(wall: &Wall, way: DVec3) -> f64 {
    match wall {
        Wall::Cylinder(cylinder) => cylinder.radius,
        Wall::Elliptic(oval) => {
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
fn chord(wall: &Wall, across: DVec3, way: DVec3) -> [f64; 2] {
    match wall {
        Wall::Cylinder(_) => [0.0, 1.0],
        Wall::Elliptic(oval) => {
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
