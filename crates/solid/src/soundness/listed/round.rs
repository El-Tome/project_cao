//! A sphere or a torus as a listing gives it, read by its own formula: the
//! circle its meridian runs along, about `(h_c, R)` in the half-plane and
//! of radius `r`, turned about the axis (#576).

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use crate::brep::{Circle, Sphere, Torus};

/// The tube of a torus, or of a sphere, whose ring radius is nought.
pub(super) struct Tube {
    origin: DVec3,
    axis: DVec3,
    u: DVec3,
    v: DVec3,
    center: DVec2,
    radius: f64,
    seam: f64,
}

impl Tube {
    pub(super) fn of_sphere(sphere: &Sphere) -> Tube {
        let axis = sphere.axis.normalize();
        let u = sphere.u.normalize();
        Tube {
            origin: sphere.origin,
            axis,
            u,
            v: axis.cross(u),
            center: DVec2::new(sphere.center, 0.0),
            radius: sphere.radius,
            seam: 0.0,
        }
    }

    pub(super) fn of_torus(torus: &Torus) -> Tube {
        let axis = torus.axis.normalize();
        let u = torus.u.normalize();
        Tube {
            origin: torus.origin,
            axis,
            u,
            v: axis.cross(u),
            center: torus.center,
            radius: torus.radius,
            seam: torus.seam,
        }
    }

    /// The tube's radius.
    pub(super) fn tube(&self) -> f64 {
        self.radius
    }

    /// A point of the axis, and its direction.
    pub(super) fn frame(&self) -> (DVec3, DVec3) {
        (self.origin, self.axis)
    }

    /// A place's height along the axis and its distance from it.
    fn meridian(&self, place: DVec3) -> DVec2 {
        let from = place - self.origin;
        let h = from.dot(self.axis);
        DVec2::new(h, (from - self.axis * h).length())
    }

    /// How far from the axis a place stands.
    pub(super) fn radius(&self, place: DVec3) -> f64 {
        self.meridian(place).y
    }

    /// The angle about the axis a place stands at.
    pub(super) fn angle(&self, place: DVec3) -> f64 {
        let from = place - self.origin;
        from.dot(self.v).atan2(from.dot(self.u))
    }

    /// The angle along the meridian circle a place stands at, within half a
    /// turn of the angle the chart is centred on.
    pub(super) fn phi(&self, place: DVec3) -> f64 {
        let from = self.meridian(place) - self.center;
        let phi = from.x.atan2(from.y);
        let phi = phi + TAU * ((self.seam - phi) / TAU).round();
        if phi <= self.seam - PI {
            phi + TAU
        } else {
            phi
        }
    }

    /// How far a place stands from the meridian circle, in its half-plane.
    pub(super) fn off(&self, place: DVec3) -> f64 {
        ((self.meridian(place) - self.center).length() - self.radius).abs()
    }

    /// Whether a circle is a meridian circle: in a plane holding the axis,
    /// about the meridian circle's centre within `room`, of its radius.
    pub(super) fn holds_meridian(&self, circle: &Circle, room: f64) -> bool {
        let center = self.meridian(circle.center);
        circle.axis.dot(self.axis).abs() <= SQUARE
            && (circle.center - self.origin).dot(circle.axis).abs() <= room
            && (center.x - self.center.x).abs() <= room
            && (center.y - self.center.y.abs()).abs() <= room
            && (circle.radius - self.radius).abs() <= room
    }
}

impl Tube {
    /// Where the poles stand: the two ends of a sphere, where the meridian
    /// circle of a spindle torus meets the axis, none on a ring.
    pub(super) fn poles(&self) -> Vec<DVec3> {
        let (big, r) = (self.center.y, self.radius);
        if big.abs() > r {
            return Vec::new();
        }
        let reach = (-big / r).acos();
        [-reach, reach]
            .into_iter()
            .map(|phi| self.origin + self.axis * (self.center.x + r * phi.sin()))
            .collect()
    }
}

/// How far from square to the axis, in the sine of the angle between the
/// two, a circle's axis may stand and the circle still be a meridian.
const SQUARE: f64 = 1e-9;
