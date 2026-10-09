//! A cone as a listing gives it, read by its own formula: the half-line its
//! meridian runs along from the apex, turned about the axis.
//!
//! The listing's foot and ruling are read as they are, and nothing here is
//! measured from the apex but what stands behind it: the apex of a cone a
//! hair from a cylinder stands a hundred million reaches away, and a length
//! taken from it would carry its rounding.

use glam::{DVec2, DVec3};

use crate::brep::Cone;

/// One nappe of a cone: `l` along the ruling from the foot, growing away
/// from the apex, `θ` anticlockwise about the axis from `u`.
pub(super) struct Nappe {
    origin: DVec3,
    axis: DVec3,
    u: DVec3,
    v: DVec3,
    foot: DVec2,
    ruling: DVec2,
}

impl Nappe {
    pub(super) fn of(cone: &Cone) -> Nappe {
        let axis = cone.axis.normalize();
        let u = cone.u.normalize();
        Nappe {
            origin: cone.origin,
            axis,
            u,
            v: axis.cross(u),
            foot: cone.foot,
            ruling: cone.ruling.normalize(),
        }
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

    /// How far along the ruling a place stands, from the foot.
    pub(super) fn length(&self, place: DVec3) -> f64 {
        (self.meridian(place) - self.foot).dot(self.ruling)
    }

    /// How far along the ruling the apex stands, from the foot.
    pub(super) fn apex_length(&self) -> f64 {
        -self.foot.y / self.ruling.y
    }

    pub(super) fn apex(&self) -> DVec3 {
        self.origin + self.axis * (self.foot.x + self.apex_length() * self.ruling.x)
    }

    /// How far a place stands from the nappe: across the meridian line in
    /// front of the apex, from the apex behind it. The meridian half-plane
    /// the place stands in holds the nearest point, the other one only a
    /// farther one.
    pub(super) fn off(&self, place: DVec3) -> f64 {
        let from = self.meridian(place) - self.foot;
        let across = self.ruling.perp_dot(from);
        let past = from.dot(self.ruling) - self.apex_length();
        if past >= 0.0 {
            across.abs()
        } else {
            past.hypot(across)
        }
    }
}
