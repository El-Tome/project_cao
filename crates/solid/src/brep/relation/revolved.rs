//! Two surfaces of one axis, a cone and a plane square to the axis, a
//! cylinder or a cone of that axis, each read as a line of the meridian
//! half-plane — `(h, ρ)`, the height along the axis from the cone's origin
//! and the distance from the axis — and decided once, where the two lines
//! cross: a circle where they cross off the axis, the apex where they meet
//! on it, apart where they cross on the other nappe or not at all.
//!
//! Each surface is the part of its line at `ρ ≥ 0`: a ray leaving the axis
//! at a cone's apex or at a plane's foot. Two rays that do not cross off the
//! axis meet, if at all, where one starts.

use glam::{DVec2, DVec3};

use super::Relation;
use crate::brep::scale::Scale;
use crate::brep::surface::Cone;

/// What the cone meets, read in the cone's own meridian half-plane.
pub(super) enum Meridian {
    /// A plane square to the axis, at this height.
    Level(f64),
    /// A cylinder about the axis, of this radius.
    Wall(f64),
    /// Another cone about the axis: its meridian line's foot and ruling.
    Nappe { foot: DVec2, ruling: DVec2 },
}

impl Meridian {
    /// Another cone about the cone `frame`'s axis, read in `frame`'s
    /// meridian half-plane.
    pub(super) fn of(frame: &Cone, other: &Cone) -> Meridian {
        let turned = frame.axis.dot(other.axis).signum();
        let shift = (other.origin - frame.origin).dot(frame.axis);
        Meridian::Nappe {
            foot: DVec2::new(turned * other.foot.x + shift, other.foot.y),
            ruling: DVec2::new(turned * other.ruling.x, other.ruling.y),
        }
    }

    /// The line's normal and offset: the points `x` with `normal · x =
    /// offset`.
    fn line(&self) -> (DVec2, f64) {
        match *self {
            Meridian::Level(height) => (DVec2::X, height),
            Meridian::Wall(radius) => (DVec2::Y, radius),
            Meridian::Nappe { foot, ruling } => (ruling.perp(), ruling.perp().dot(foot)),
        }
    }

    /// The ray of the line the surface is, where it leaves the axis.
    fn ray(&self) -> Option<Ray> {
        match *self {
            Meridian::Level(height) => Some(Ray {
                start: DVec2::new(height, 0.0),
                direction: DVec2::Y,
            }),
            Meridian::Wall(_) => None,
            Meridian::Nappe { foot, ruling } => Some(Ray {
                start: DVec2::new(foot.x - ruling.x * foot.y / ruling.y, 0.0),
                direction: ruling,
            }),
        }
    }
}

/// A ray of the meridian half-plane leaving the axis.
struct Ray {
    start: DVec2,
    direction: DVec2,
}

impl Ray {
    fn of(cone: &Cone) -> Ray {
        Ray {
            start: DVec2::new(cone.height_at(cone.apex_at()), 0.0),
            direction: cone.ruling,
        }
    }

    fn distance(&self, point: DVec2) -> f64 {
        let from = point - self.start;
        (from - self.direction * from.dot(self.direction).max(0.0)).length()
    }
}

/// The relation of a cone and a surface of its axis. Lines parallel across
/// the box are two cones, one surface where their offsets are within the
/// tolerance, or a cone within the tolerance of a plane or a cylinder all
/// along, which no decision lays out. Where the lines cross, a plane keeps
/// its height and a cylinder its radius to the bit, so that the circle is
/// the one either shares with every other surface there.
pub(super) fn met(cone: &Cone, other: Meridian, scale: Scale) -> Relation {
    let eps = scale.eps();
    let across = cone.ruling.perp();
    let offset = across.dot(cone.foot);
    let (normal, level) = other.line();
    let sine = across.perp_dot(normal);
    if sine.abs() * 2.0 * scale.reach() <= eps {
        let Meridian::Nappe { .. } = other else {
            return Relation::Unsupported;
        };
        let agree = across.dot(normal) > 0.0;
        let facing = if agree { 1.0 } else { -1.0 };
        return if (offset - facing * level).abs() <= eps {
            Relation::Same { agree }
        } else {
            Relation::Apart
        };
    }
    let crossing = match other {
        Meridian::Level(height) => DVec2::new(height, cone.section(height)),
        Meridian::Wall(radius) => {
            DVec2::new((cone.ruling.x * radius - offset) / cone.ruling.y, radius)
        }
        Meridian::Nappe { .. } => DVec2::new(
            (offset * normal.y - level * across.y) / sine,
            (across.x * level - normal.x * offset) / sine,
        ),
    };
    if crossing.y > eps {
        return Relation::Circle(cone.circle(crossing.x, crossing.y));
    }
    let Some(theirs) = other.ray() else {
        return Relation::Apart;
    };
    let own = Ray::of(cone);
    let height = if crossing.y >= 0.0 {
        crossing.x
    } else {
        let (mine, yours) = (theirs.distance(own.start), own.distance(theirs.start));
        if mine.min(yours) > eps {
            return Relation::Apart;
        }
        if mine < yours {
            own.start.x
        } else {
            theirs.start.x
        }
    };
    let apex = cone.origin + cone.axis * height;
    if in_the_box(apex, scale) {
        Relation::Apex(apex)
    } else {
        Relation::Apart
    }
}

/// How far apart two cones of one axis stand: their axes across, and their
/// meridian lines' offsets read in the first's half-plane.
pub(in crate::brep) fn apart_by(one: &Cone, other: &Cone) -> f64 {
    let between = other.origin - one.origin;
    let across = (between - one.axis * one.axis.dot(between)).length();
    let normal = one.ruling.perp();
    let (theirs, level) = Meridian::of(one, other).line();
    let facing = normal.dot(theirs).signum();
    across + (normal.dot(one.foot) - facing * level).abs()
}

/// Whether a point stands within the box the scale is taken over.
pub(super) fn in_the_box(point: DVec3, scale: Scale) -> bool {
    point.abs().max_element() <= scale.reach() + scale.eps()
}
