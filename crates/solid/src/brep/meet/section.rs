//! How a wall's section is read across a pair: a circle as its own root, an
//! ellipse as the root of the circle of its half-width, sheared and scaled
//! (#571).

use glam::DVec3;

use crate::brep::curve::Wall;
use crate::brep::surface::EllipticCylinder;

/// How a wall's section is read across the pair: a circle as its own root,
/// an ellipse as the root of the circle of its half-width, sheared along the
/// diameter conjugate to the chords it cuts and scaled across them (#571).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Section {
    Round,
    Oval {
        /// How far the middle of a chord moves per unit of height.
        shear: f64,
        /// What a unit of the root is worth across the chord.
        scale: f64,
    },
}

impl Section {
    /// A section's coordinate across the chords, its value and its first two
    /// derivatives, from the height's and the signed root's: on a circle
    /// the root itself, bit for bit.
    pub(super) fn across(self, height: [f64; 3], root: [f64; 3]) -> [f64; 3] {
        match self {
            Section::Round => root,
            Section::Oval { shear, scale } => {
                [0, 1, 2].map(|order| shear * height[order] + scale * root[order])
            }
        }
    }
}

/// A wall read across the pair, its coordinate across the chords along
/// `across` and the height along `along`: its half-width along the height,
/// how its section is read, and its size — a radius, or a major half-axis.
pub(super) fn read(wall: &Wall, across: DVec3, along: DVec3) -> (f64, Section, f64) {
    match wall {
        Wall::Cylinder(cylinder) => (cylinder.radius, Section::Round, cylinder.radius),
        Wall::Cone(cone) => (cone.foot.y.abs(), Section::Round, cone.foot.y.abs()),
        Wall::Elliptic(oval) => {
            let (half, section) = sheared(oval, across, along);
            (half, section, oval.major)
        }
    }
}

/// An elliptic wall's section in the coordinates `(c, h)` along `across` and
/// `along`, square to each other and to its axis. Its quadratic form there,
/// `p₁₁ c² + 2 p₁₂ c h + p₂₂ h² = 1`, of determinant `1/(A²B²)`, solved for
/// `c` at the height `h`: `c = κ h ± α √((w − h)(w + h))`, with `κ = −p₁₂/p₁₁`,
/// `α = 1/(A B p₁₁)` and the half-width `w = A B √p₁₁`, how far the wall
/// reaches along `along`.
fn sheared(oval: &EllipticCylinder, across: DVec3, along: DVec3) -> (f64, Section) {
    let (major, minor) = (oval.major, oval.minor);
    let [cu, cv] = [across.dot(oval.u) / major, across.dot(oval.v) / minor];
    let [hu, hv] = [along.dot(oval.u) / major, along.dot(oval.v) / minor];
    let p11 = cu * cu + cv * cv;
    let p12 = cu * hu + cv * hv;
    let half = oval.support(along);
    let section = Section::Oval {
        shear: -p12 / p11,
        scale: 1.0 / (major * minor * p11),
    };
    (half, section)
}

/// The eccentric angle at which an elliptic wall reaches furthest along
/// `along`, square to its axis.
pub(super) fn reached(oval: &EllipticCylinder, along: DVec3) -> f64 {
    (oval.minor * along.dot(oval.v)).atan2(oval.major * along.dot(oval.u))
}
