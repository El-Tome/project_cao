//! A circle seen in the parameters of a sphere or a torus (#576): a latitude
//! circle as a stretch at constant `φ`, its angle read from the circle's own
//! reference as on a cone, never from points; a meridian as a stretch at
//! constant `θ`, read where the stretch stands farthest from the axis, since
//! at a pole the angle is rounding, its `φ` run on from its start the way the
//! circle turns, a whole circle a whole turn.

use glam::{DVec2, DVec3};

use super::chord;
use crate::brep::Declined;
use crate::brep::curve::{Circle, Curve};
use crate::brep::surface::{Sphere, Surface, Torus};
use crate::brep::trace::Trace;

/// The chart of a sphere or a torus: its frame, its meridian circle's
/// centre `(h_c, R)`, and how an angle along the meridian is windowed.
struct Chart {
    origin: DVec3,
    axis: DVec3,
    u: DVec3,
    v: DVec3,
    center: DVec2,
    seam: Option<f64>,
}

impl Chart {
    fn of(surface: &Surface) -> Option<Chart> {
        match surface {
            Surface::Sphere(Sphere {
                origin,
                axis,
                u,
                v,
                center,
                ..
            }) => Some(Chart {
                origin: *origin,
                axis: *axis,
                u: *u,
                v: *v,
                center: DVec2::new(*center, 0.0),
                seam: None,
            }),
            Surface::Torus(Torus {
                origin,
                axis,
                u,
                v,
                center,
                seam,
                ..
            }) => Some(Chart {
                origin: *origin,
                axis: *axis,
                u: *u,
                v: *v,
                center: *center,
                seam: Some(*seam),
            }),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
                None
            }
        }
    }

    /// The angle along the meridian of a point at `(h, ρ)`, in the window.
    fn phi(&self, meridian: DVec2) -> f64 {
        let from = meridian - self.center;
        let phi = from.x.atan2(from.y);
        match self.seam {
            Some(seam) => {
                let phi =
                    phi + std::f64::consts::TAU * ((seam - phi) / std::f64::consts::TAU).round();
                if phi <= seam - std::f64::consts::PI {
                    phi + std::f64::consts::TAU
                } else {
                    phi
                }
            }
            None => phi,
        }
    }

    fn meridian(&self, point: DVec3) -> DVec2 {
        let from = point - self.origin;
        let h = from.dot(self.axis);
        DVec2::new(h, (from - self.axis * h).length())
    }
}

/// The stretch of `circle` from `from` to `to` seen on `surface`, a sphere
/// or a torus: a latitude, a meridian, or the chord it stands within `eps`
/// of.
pub(super) fn traced(
    surface: &Surface,
    circle: &Circle,
    from: f64,
    to: f64,
    eps: f64,
) -> Result<Trace, Declined> {
    let Some(chart) = Chart::of(surface) else {
        return Err(Declined::Unsupported);
    };
    let latitude = match surface {
        Surface::Sphere(sphere) => sphere.holds(circle, eps),
        Surface::Torus(torus) => torus.holds(circle, eps),
        _ => false,
    };
    if latitude {
        let turn = circle.axis.dot(chart.axis).signum();
        let start = circle.u.dot(chart.v).atan2(circle.u.dot(chart.u));
        let height = (circle.center - chart.origin).dot(chart.axis);
        let phi = chart.phi(DVec2::new(height, circle.radius));
        return Ok(Trace::Segment {
            from: DVec2::new(start + turn * from, phi),
            to: DVec2::new(start + turn * to, phi),
        });
    }
    let meridian = match surface {
        Surface::Sphere(sphere) => sphere.meridian(circle, eps),
        Surface::Torus(torus) => torus.meridian(circle, eps),
        _ => false,
    };
    if meridian {
        let farthest = [from, (from + to) / 2.0, to]
            .into_iter()
            .map(|at| circle.point(at))
            .max_by(|one, other| chart.meridian(*one).y.total_cmp(&chart.meridian(*other).y))
            .expect("three places");
        let theta = surface.parameters(farthest).x;
        let (sin, cos) = theta.sin_cos();
        let radial = chart.u * cos + chart.v * sin;
        let turn = circle.axis.dot(radial.cross(chart.axis)).signum();
        let start = chart.phi(chart.meridian(circle.point(from)));
        return Ok(Trace::Segment {
            from: DVec2::new(theta, start),
            to: DVec2::new(theta, start + turn * (to - from)),
        });
    }
    chord(&Curve::Circle(*circle), surface, from, to, eps)
}
