//! Which side of each other a sphere or a torus and a surface of its axis
//! stand on (#576), read off the rims their pair was decided along, in the
//! meridian half-plane of the axis. Along a rim the two only touch along,
//! the one whose meridian bends more towards the other's normal there stands
//! on that side of it: a shaft of a ball's radius stands outside the ball
//! along its equator and the ball inside the shaft, a bore touching a
//! groove's root stands on the groove's side. Between two rims the two cross
//! along, or beyond them, they stand one above the other all along each
//! stretch of the first's meridian, read at the stretch's middle, where they
//! stand furthest apart: a bore through a ball runs inside it between the
//! rims and outside beyond them. A stretch ends at the first's pole or on
//! the axis; one running on without end is read a tube's radius past its
//! rim, where the round has turned away from it.

use std::f64::consts::FRAC_PI_2;

use glam::{DVec2, DVec3};

use super::{measured, middle_of_the_lobe};
use crate::brep::relation::Rim;
use crate::brep::surface::{Sphere, Surface, Torus};

pub(super) fn across_the_rims(
    first: &Surface,
    second: &Surface,
    rims: &[Rim],
    place: DVec3,
    rounding: f64,
) -> Option<bool> {
    if let Some(touch) = rims.iter().find(|rim| rim.touching) {
        return bending_more(first, second, touch.circle.point(0.0));
    }
    let middle = middle_of_the_stretch(first, second, rims, place)?;
    measured(first, second, middle, rounding)
}

/// Whether a sphere or a torus is among the two.
pub(super) fn rounded(first: &Surface, second: &Surface) -> bool {
    matches!(
        (first, second),
        (Surface::Sphere(_) | Surface::Torus(_), _) | (_, Surface::Sphere(_) | Surface::Torus(_))
    )
}

/// Whether the second's meridian bends more towards the first's normal than
/// the first's own does, at `at`, where the two touch along a rim or at a
/// pole: a straight meridian bends not at all, a round's towards its
/// meridian circle's centre.
pub(super) fn bending_more(first: &Surface, second: &Surface, at: DVec3) -> Option<bool> {
    let normal = first.normal(first.parameters(at));
    let [own, theirs] = [first, second].map(|surface| bend(surface, at).dot(normal));
    Some(theirs > own)
}

/// The curvature vector of a surface's meridian at `at`: towards the
/// meridian circle's centre, one over its radius long, for a sphere or a
/// torus; nought for a straight meridian.
fn bend(surface: &Surface, at: DVec3) -> DVec3 {
    match surface {
        Surface::Sphere(sphere) => (sphere.middle() - at) / (sphere.radius * sphere.radius),
        Surface::Torus(torus) => (tube_centre(torus, at) - at) / (torus.radius * torus.radius),
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
            DVec3::ZERO
        }
    }
}

/// The centre of the torus's meridian circle through `at`.
fn tube_centre(torus: &Torus, at: DVec3) -> DVec3 {
    let theta = torus.parameters(at).x;
    torus.origin + torus.axis * torus.center.x + torus.radial(theta) * torus.center.y
}

/// The axis a pair with a round is read about, with the `u` every surface of
/// it shares.
struct Axle {
    origin: DVec3,
    axis: DVec3,
    u: DVec3,
}

impl Axle {
    fn of(surface: &Surface) -> Option<Axle> {
        match surface {
            Surface::Sphere(Sphere {
                origin, axis, u, ..
            })
            | Surface::Torus(Torus {
                origin, axis, u, ..
            }) => Some(Axle {
                origin: *origin,
                axis: *axis,
                u: *u,
            }),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
                None
            }
        }
    }

    /// The `(h, ρ)` of a point, and the direction it stands off the axis
    /// along.
    fn seen(&self, point: DVec3) -> (DVec2, DVec3) {
        let from = point - self.origin;
        let h = from.dot(self.axis);
        let radial = from - self.axis * h;
        (
            DVec2::new(h, radial.length()),
            radial.try_normalize().unwrap_or(self.u),
        )
    }

    fn point(&self, at: DVec2, outward: DVec3) -> DVec3 {
        self.origin + self.axis * at.x + outward * at.y
    }
}

/// A surface's meridian in the half-plane: a round's circle, with the angle
/// its poles stand at where it has any, or a straight line from where it
/// leaves the axis, if it does.
enum Meridian {
    Round {
        center: DVec2,
        radius: f64,
        poles: Option<f64>,
    },
    Straight {
        through: DVec2,
        along: DVec2,
        from: Option<f64>,
    },
}

impl Meridian {
    fn of(surface: &Surface, axle: &Axle) -> Option<Meridian> {
        let read = |origin: DVec3, axis: DVec3| {
            (
                axle.axis.dot(axis).signum(),
                (origin - axle.origin).dot(axle.axis),
            )
        };
        Some(match surface {
            Surface::Sphere(sphere) => {
                let (turned, shift) = read(sphere.origin, sphere.axis);
                Meridian::Round {
                    center: DVec2::new(turned * sphere.center + shift, 0.0),
                    radius: sphere.radius,
                    poles: Some(FRAC_PI_2),
                }
            }
            Surface::Torus(torus) => {
                let (turned, shift) = read(torus.origin, torus.axis);
                let (big, radius) = (torus.center.y, torus.radius);
                Meridian::Round {
                    center: DVec2::new(turned * torus.center.x + shift, big),
                    radius,
                    poles: (big.abs() <= radius).then(|| (-big / radius).acos()),
                }
            }
            Surface::Plane(plane) => {
                let level =
                    (plane.offset() - plane.normal.dot(axle.origin)) / plane.normal.dot(axle.axis);
                Meridian::Straight {
                    through: DVec2::new(level, 0.0),
                    along: DVec2::Y,
                    from: Some(0.0),
                }
            }
            Surface::Cylinder(cylinder) => Meridian::Straight {
                through: DVec2::new(0.0, cylinder.radius),
                along: DVec2::X,
                from: None,
            },
            Surface::Cone(cone) => {
                let (turned, shift) = read(cone.origin, cone.axis);
                let through = DVec2::new(turned * cone.foot.x + shift, cone.foot.y);
                Meridian::Straight {
                    through,
                    along: DVec2::new(turned * cone.ruling.x, cone.ruling.y),
                    from: Some(-cone.foot.y / cone.ruling.y),
                }
            }
            Surface::Elliptic(_) => return None,
        })
    }
}

/// The point of the first's meridian through `place` half way along the
/// stretch, between the rims or beyond them, that `place` stands on.
fn middle_of_the_stretch(
    first: &Surface,
    second: &Surface,
    rims: &[Rim],
    place: DVec3,
) -> Option<DVec3> {
    let axle = Axle::of(first).or_else(|| Axle::of(second))?;
    let (at, outward) = axle.seen(place);
    let crossed: Vec<DVec2> = rims
        .iter()
        .map(|rim| {
            DVec2::new(
                (rim.circle.center - axle.origin).dot(axle.axis),
                rim.circle.radius,
            )
        })
        .collect();
    let middle = match Meridian::of(first, &axle)? {
        Meridian::Round {
            center,
            radius,
            poles,
        } => {
            let angle = |point: DVec2| (point.x - center.x).atan2(point.y - center.y);
            let mut angles: Vec<f64> = crossed.iter().copied().map(angle).collect();
            angles.sort_by(f64::total_cmp);
            let own = angle(at);
            let half = match (poles, angles.as_slice()) {
                (None, [one, other]) => middle_of_the_lobe(own, [*one, *other]),
                (None, _) => return None,
                (Some(reach), _) => between(&angles, [-reach, reach], own, None)?,
            };
            center + DVec2::new(half.sin(), half.cos()) * radius
        }
        Meridian::Straight {
            through,
            along,
            from,
        } => {
            let span = match second {
                Surface::Sphere(sphere) => sphere.radius,
                Surface::Torus(torus) => torus.radius,
                Surface::Plane(_)
                | Surface::Cylinder(_)
                | Surface::Cone(_)
                | Surface::Elliptic(_) => {
                    return None;
                }
            };
            let mut along_it: Vec<f64> = crossed
                .iter()
                .map(|point| (*point - through).dot(along))
                .collect();
            along_it.sort_by(f64::total_cmp);
            let own = (at - through).dot(along);
            let half = between(
                &along_it,
                [from.unwrap_or(f64::NEG_INFINITY), f64::INFINITY],
                own,
                Some(span),
            )?;
            through + along * half
        }
    };
    Some(axle.point(middle, outward))
}

/// Half way along the stretch between two of the sorted `bounds`, or between
/// one of them and an end of `[low, high]`, that `own` stands on; an end
/// without bound is read `span` past the nearest bound.
fn between(bounds: &[f64], [low, high]: [f64; 2], own: f64, span: Option<f64>) -> Option<f64> {
    let (first, last) = (*bounds.first()?, *bounds.last()?);
    let own = own.clamp(low.min(first), high.max(last));
    if own <= first {
        return Some(if low.is_finite() {
            (low.min(first) + first) / 2.0
        } else {
            first - span?
        });
    }
    if own >= last {
        return Some(if high.is_finite() {
            (last + high.max(last)) / 2.0
        } else {
            last + span?
        });
    }
    bounds
        .windows(2)
        .find(|pair| pair[0] <= own && own <= pair[1])
        .map(|pair| (pair[0] + pair[1]) / 2.0)
}
