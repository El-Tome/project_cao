//! The second operand moved away from an axis rather than along a line
//! (#626): a wall a hair from tangent to a round of its axis is no
//! translation from touching it, so its radius grows or shrinks by the
//! hair, and every point of what moves with it goes square to the axis by
//! as much. A cylinder, a torus or a cone of the axis stays one, its radius,
//! its ring or its meridian line moved; a plane square to the axis or
//! holding it slides along itself; a circle of the axis keeps its centre and
//! its angles, and a line along the axis, or a circle of a plane holding it,
//! goes with the side of the axis it stands on. Anything else — a sphere,
//! whose poles stand on the axis, a point on the axis, a surface of another
//! axis, a line across it — is no such move, and the move is not made.

use glam::{DVec2, DVec3};

use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Surface, Torus};

/// Every point moved square to the line through `origin` along `axis`,
/// away from it by `by`, or towards it where `by` is negative.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::brep::canonical) struct Across {
    pub origin: DVec3,
    pub axis: DVec3,
    pub by: f64,
}

/// Under this share of the tolerance, a surface or a point stands on the
/// axis, or square to it, but for rounding.
const ROUNDING: f64 = 1e-6;

impl Across {
    /// The point moved, none where it stands on the axis or the move would
    /// take it there or across.
    pub fn point(&self, point: DVec3, scale: Scale) -> Option<DVec3> {
        let away = self.away(point, scale)?;
        let from = point - self.origin;
        let across = (from - self.axis * self.axis.dot(from)).length();
        (across + self.by > scale.eps()).then(|| point + away * self.by)
    }

    /// Whether the move leaves `surface` where it is: a plane square to the
    /// axis, or holding it.
    pub fn slides(&self, surface: &Surface, scale: Scale) -> bool {
        let Surface::Plane(plane) = surface else {
            return false;
        };
        let room = ROUNDING * scale.eps();
        let leaning = plane.normal.cross(self.axis).length() * scale.reach();
        let tilted = plane.normal.dot(self.axis).abs() * scale.reach();
        leaning <= room || tilted <= room && plane.distance(self.origin).abs() <= room
    }

    /// The surface moved, canonical; none for one this move does not keep
    /// what it is.
    pub fn surface(&self, surface: &Surface, scale: Scale) -> Option<Surface> {
        let lift = DVec2::new(0.0, self.by);
        match surface {
            Surface::Cylinder(cylinder) if self.about(cylinder.origin, cylinder.axis, scale) => {
                let radius = cylinder.radius + self.by;
                (radius > 0.0).then(|| {
                    Surface::Cylinder(Cylinder::about(cylinder.origin, cylinder.axis, radius))
                })
            }
            Surface::Torus(torus) if self.about(torus.origin, torus.axis, scale) => {
                let ring = torus.center + lift;
                (ring.y > 0.0).then(|| {
                    Surface::Torus(Torus::through(
                        torus.origin,
                        torus.axis,
                        ring,
                        torus.radius,
                        torus.middle() + lift,
                    ))
                })
            }
            Surface::Cone(cone) if self.about(cone.origin, cone.axis, scale) => {
                let foot = cone.foot + lift;
                Some(Surface::Cone(Cone::through(
                    cone.origin,
                    cone.axis,
                    [foot, foot + cone.ruling],
                )))
            }
            Surface::Plane(_)
            | Surface::Cylinder(_)
            | Surface::Torus(_)
            | Surface::Cone(_)
            | Surface::Sphere(_)
            | Surface::Elliptic(_) => None,
        }
    }

    /// The curve moved; none for one this move does not keep a line or a
    /// circle.
    pub fn curve(&self, curve: &Curve, scale: Scale) -> Option<Curve> {
        match *curve {
            Curve::Circle(circle) if self.about(circle.center, circle.axis, scale) => {
                let radius = circle.radius + self.by;
                (radius > 0.0).then_some(Curve::Circle(Circle { radius, ..circle }))
            }
            Curve::Circle(circle) if self.holds(circle.center, circle.axis, scale) => {
                Some(Curve::Circle(Circle {
                    center: self.point(circle.center, scale)?,
                    ..circle
                }))
            }
            Curve::Line(line)
                if line.direction.cross(self.axis).length() * scale.reach()
                    <= ROUNDING * scale.eps() =>
            {
                Some(Curve::Line(Line::through(
                    self.point(line.origin, scale)?,
                    line.direction,
                )))
            }
            Curve::Line(_) | Curve::Circle(_) | Curve::Conic(_) | Curve::Meet(_) => None,
        }
    }

    /// The way from the axis to `point`, of unit length; none on the axis.
    fn away(&self, point: DVec3, scale: Scale) -> Option<DVec3> {
        let from = point - self.origin;
        let across = from - self.axis * self.axis.dot(from);
        (across.length() > scale.eps()).then(|| across.normalize())
    }

    /// Whether the line through `origin` along `axis` is the move's axis.
    fn about(&self, origin: DVec3, axis: DVec3, scale: Scale) -> bool {
        let room = ROUNDING * scale.eps();
        let from = origin - self.origin;
        axis.cross(self.axis).length() * scale.reach() <= room
            && (from - self.axis * self.axis.dot(from)).length() <= room
    }

    /// Whether the plane through `point` square to `normal` holds the axis,
    /// `point` standing off it.
    fn holds(&self, point: DVec3, normal: DVec3, scale: Scale) -> bool {
        let room = ROUNDING * scale.eps();
        normal.dot(self.axis).abs() * scale.reach() <= room
            && normal.dot(point - self.origin).abs() <= room
            && self.away(point, scale).is_some()
    }
}
