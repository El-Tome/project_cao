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
//!
//! A sphere or a torus (#576) is a circle of the same half-plane, about
//! `(h_c, R)` and of radius `r`, kept where `ρ ≥ 0`; a round against a line
//! or another circle is decided in `arc.rs`, in the frame of the first
//! surface, which `Frame` carries so that every circle built is the one a
//! cylinder of the axis has.

mod arc;

use glam::{DVec2, DVec3};

use super::Relation;
use crate::brep::curve::Circle;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Plane, Sphere, Surface, Torus};
pub(super) use arc::met as met_round;

/// What the cone meets, read in the cone's own meridian half-plane.
pub(super) enum Meridian {
    /// A plane square to the axis, at this height.
    Level(f64),
    /// A cylinder about the axis, of this radius.
    Wall(f64),
    /// Another cone about the axis: its meridian line's foot and ruling.
    Nappe { foot: DVec2, ruling: DVec2 },
    /// A sphere or a torus about the axis: its meridian circle's centre,
    /// `(h_c, R)`, and radius (#576). A sphere has `R = 0`.
    Arc { center: DVec2, radius: f64 },
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

    /// A surface about `frame`'s axis, read in its meridian half-plane with
    /// the same `turned` and `shift` a cone is read with: a cylinder its
    /// radius, a cone its line, a sphere or a torus its circle; none for a
    /// plane, read as a level by its caller, or an elliptic wall (#576).
    pub(super) fn of_surface(frame: &Frame, surface: &Surface) -> Option<Meridian> {
        let read = |origin: DVec3, axis: DVec3| {
            (
                frame.axis.dot(axis).signum(),
                (origin - frame.origin).dot(frame.axis),
            )
        };
        match surface {
            Surface::Cylinder(cylinder) => Some(Meridian::Wall(cylinder.radius)),
            Surface::Cone(cone) => {
                let (turned, shift) = read(cone.origin, cone.axis);
                Some(Meridian::Nappe {
                    foot: DVec2::new(turned * cone.foot.x + shift, cone.foot.y),
                    ruling: DVec2::new(turned * cone.ruling.x, cone.ruling.y),
                })
            }
            Surface::Sphere(sphere) => {
                let (turned, shift) = read(sphere.origin, sphere.axis);
                Some(Meridian::Arc {
                    center: DVec2::new(turned * sphere.center + shift, 0.0),
                    radius: sphere.radius,
                })
            }
            Surface::Torus(torus) => {
                let (turned, shift) = read(torus.origin, torus.axis);
                Some(Meridian::Arc {
                    center: DVec2::new(turned * torus.center.x + shift, torus.center.y),
                    radius: torus.radius,
                })
            }
            Surface::Plane(_) | Surface::Elliptic(_) => None,
        }
    }

    /// The line's normal and offset: the points `x` with `normal · x =
    /// offset`. An arc is no line: `met` crosses it in `arc.rs` before it
    /// reads one.
    fn line(&self) -> (DVec2, f64) {
        match *self {
            Meridian::Level(height) => (DVec2::X, height),
            Meridian::Wall(radius) => (DVec2::Y, radius),
            Meridian::Nappe { foot, ruling } => (ruling.perp(), ruling.perp().dot(foot)),
            Meridian::Arc { .. } => unreachable!("an arc of the meridian is met as a circle"),
        }
    }

    /// The ray of the line the surface is, where it leaves the axis.
    fn ray(&self) -> Option<Ray> {
        match *self {
            Meridian::Level(height) => Some(Ray {
                start: DVec2::new(height, 0.0),
                direction: DVec2::Y,
            }),
            Meridian::Wall(_) | Meridian::Arc { .. } => None,
            Meridian::Nappe { foot, ruling } => Some(Ray {
                start: DVec2::new(foot.x - ruling.x * foot.y / ruling.y, 0.0),
                direction: ruling,
            }),
        }
    }
}

/// The axis a surface of revolution is read about, with the `origin`, `u`
/// and `v` every surface of that axis shares: the frame every circle of the
/// meridian half-plane is built in, so that it is the very circle a cylinder
/// of the axis has there (#576).
#[derive(Clone, Copy, Debug)]
pub(super) struct Frame {
    pub origin: DVec3,
    pub axis: DVec3,
    pub u: DVec3,
    pub v: DVec3,
}

impl Frame {
    pub(super) fn of_cone(cone: &Cone) -> Frame {
        Frame {
            origin: cone.origin,
            axis: cone.axis,
            u: cone.u,
            v: cone.v,
        }
    }

    pub(super) fn of_sphere(sphere: &Sphere) -> Frame {
        Frame {
            origin: sphere.origin,
            axis: sphere.axis,
            u: sphere.u,
            v: sphere.v,
        }
    }

    pub(super) fn of_torus(torus: &Torus) -> Frame {
        Frame {
            origin: torus.origin,
            axis: torus.axis,
            u: torus.u,
            v: torus.v,
        }
    }

    /// The point of the axis at height `h`.
    pub(super) fn point(&self, h: f64) -> DVec3 {
        self.origin + self.axis * h
    }

    /// The circle about the axis at height `h` and of `radius`, as
    /// `Cone::circle` builds it.
    pub(super) fn circle(&self, h: f64, radius: f64) -> Circle {
        Circle {
            center: self.point(h),
            axis: self.axis,
            radius,
            u: self.u,
            v: self.v,
        }
    }

    /// The height a plane parallel to the axis's normal plane stands at, as
    /// `conical::with_plane` reads it.
    pub(super) fn level(&self, plane: &Plane) -> f64 {
        (plane.offset() - plane.normal.dot(self.origin)) / plane.normal.dot(self.axis)
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
/// the one either shares with every other surface there. A sphere or a
/// torus about the axis is crossed as the circle it is, in the cone's frame
/// (`arc.rs`).
pub(super) fn met(cone: &Cone, other: Meridian, scale: Scale) -> Relation {
    if let Meridian::Arc { center, radius } = other {
        let nappe = Meridian::Nappe {
            foot: cone.foot,
            ruling: cone.ruling,
        };
        return arc::met(&Frame::of_cone(cone), center, radius, nappe, scale);
    }
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
        Meridian::Arc { .. } => unreachable!("an arc of the meridian is met as a circle"),
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
