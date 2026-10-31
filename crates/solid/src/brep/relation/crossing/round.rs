//! Where a line or a circle crosses a sphere or a torus (#576, decision 4).
//!
//! A sphere is crossed at the roots of its quadratic, and touched once where
//! the line passes within the tolerance of it at its closest, as a wall is.
//! A torus is solved only where the line lies in a plane holding the axis:
//! there its section is two circles of the tube's radius, about `(h_c, ±R)`
//! in the plane's `(h, s)`, each the surface on its own side of the axis,
//! and the line crosses each as it crosses a wall. Any other line meets the
//! torus along the roots of a quartic no corner of the kernel's pairs stands
//! on: unsupported, as the pair it would lie on is.
//!
//! A circle about the axis lies on the round where the round holds it, as a
//! latitude circle, and crosses it nowhere otherwise; one about a parallel
//! axis off it meets the round along no curve the kernel builds a corner on.
//! A circle in a plane holding the axis is a circle of that plane's signed
//! `(h, ρ)`, reaching across the axis, and crosses the round where it
//! crosses the round's own meridian circle on either side of the axis
//! (`relation/revolved/arc.rs`): the very crossings decision 2 gives the
//! pair of the round and the surface the circle lies on, a touch decided
//! there as that pair decides it. Any other circle is unsupported, as the
//! pair it lies on is.

use glam::{DVec2, DVec3};

use super::super::revolved::{Frame, Meridian, Met, met_in_the_half_plane};
use super::super::{parallel, square};
use super::{Solved, Touches, touches};
use crate::brep::curve::{Circle, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Sphere, Torus};

pub(super) fn line_and_sphere(
    line: &Line,
    sphere: &Sphere,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let from = line.origin - sphere.middle();
    let speed = line.direction.length_squared();
    let closest = -from.dot(line.direction) / speed;
    let gap = (from + line.direction * closest).length() - sphere.radius;
    crossed(line, closest, gap, sphere.radius, speed, scale, touching)
}

/// The crossings of a line passing `gap` outside a circle of `radius`,
/// closest at `closest`, with the touch decided as for a wall.
fn crossed(
    line: &Line,
    closest: f64,
    gap: f64,
    radius: f64,
    speed: f64,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    if gap > scale.eps() {
        return Solved::At(Vec::new());
    }
    let half = (-gap * (2.0 * radius + gap) / speed).max(0.0).sqrt();
    if touches(gap, half, touching.here(line.point(closest), scale), scale) {
        return Solved::At(vec![(closest, true)]);
    }
    Solved::At(vec![(closest - half, false), (closest + half, false)])
}

pub(super) fn line_and_torus(
    line: &Line,
    torus: &Torus,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let eps = scale.eps();
    let from = line.origin - torus.origin;
    let rise = line.direction.dot(torus.axis);
    let spread = line.direction - torus.axis * rise;
    let radial = from - torus.axis * from.dot(torus.axis);
    let across = if parallel(line.direction, torus.axis, scale) {
        radial.normalize_or_zero()
    } else {
        let normal = torus.axis.cross(line.direction).normalize();
        if from.dot(normal).abs() > eps {
            return Solved::Unsupported;
        }
        spread.normalize()
    };
    let (height, climb) = (from.dot(torus.axis), rise);
    let (aside, drift) = (radial.dot(across), line.direction.dot(across));
    let speed = climb * climb + drift * drift;
    let mut found: Vec<(f64, bool)> = Vec::new();
    for side in [1.0, -1.0] {
        let center = (torus.center.x, side * torus.center.y);
        let (dh, ds) = (height - center.0, aside - center.1);
        let closest = -(dh * climb + ds * drift) / speed;
        let gap = (dh + climb * closest).hypot(ds + drift * closest) - torus.radius;
        let Solved::At(roots) = crossed(line, closest, gap, torus.radius, speed, scale, touching)
        else {
            continue;
        };
        for (at, touch) in roots {
            let s = aside + drift * at;
            let own = s * side >= -eps;
            let twice = found.iter().any(|(other, _)| (other - at).abs() <= eps);
            if own && !twice {
                found.push((at, touch));
            }
        }
    }
    Solved::At(found)
}

pub(super) fn circle_and_sphere(
    circle: &Circle,
    sphere: &Sphere,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let eps = scale.eps();
    if sphere.holds(circle, eps) || sphere.meridian(circle, eps) {
        return Solved::Along;
    }
    let round = (DVec2::new(sphere.center, 0.0), sphere.radius);
    circle_and_round(circle, &Frame::of_sphere(sphere), round, scale, touching)
}

pub(super) fn circle_and_torus(
    circle: &Circle,
    torus: &Torus,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let eps = scale.eps();
    if torus.holds(circle, eps) || torus.meridian(circle, eps) {
        return Solved::Along;
    }
    let round = (torus.center, torus.radius);
    circle_and_round(circle, &Frame::of_torus(torus), round, scale, touching)
}

/// A circle the round does not hold, against the round whose meridian circle
/// is about `center` with `radius` in `frame`. In a plane holding the axis
/// the circle is read in that plane's signed `(h, ρ)`, positive towards its
/// own centre, and crossed with the round's meridian circle on that side and
/// with its reflection on the other, each crossing kept on the side whose
/// circle it was found on, a point of the axis once.
fn circle_and_round(
    circle: &Circle,
    frame: &Frame,
    (center, radius): (DVec2, f64),
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let eps = scale.eps();
    let from = circle.center - frame.origin;
    let h = from.dot(frame.axis);
    let radial = from - frame.axis * h;
    if parallel(circle.axis, frame.axis, scale) {
        return if radial.length() <= eps {
            Solved::At(Vec::new())
        } else {
            Solved::Unsupported
        };
    }
    if !square(circle.axis, frame.axis, scale) || from.dot(circle.axis).abs() > eps {
        return Solved::Unsupported;
    }
    let side = radial
        .try_normalize()
        .unwrap_or_else(|| circle.axis.cross(frame.axis).normalize());
    let own = DVec2::new(h, radial.length());
    let point = |at: DVec2| frame.origin + frame.axis * at.x + side * at.y;
    let mut found: Vec<(f64, bool)> = Vec::new();
    let mut points: Vec<DVec3> = Vec::new();
    for mirror in [1.0, -1.0] {
        let meridian = Meridian::Arc {
            center: DVec2::new(center.x, mirror * center.y),
            radius,
        };
        let (foot, along, gap, half) =
            match met_in_the_half_plane(own, circle.radius, &meridian, eps) {
                Met::Same => return Solved::Along,
                Met::Unsupported => return Solved::Unsupported,
                Met::Near {
                    foot,
                    along,
                    gap,
                    half,
                } => (foot, along, gap, half),
            };
        if gap > eps {
            continue;
        }
        let crossed = if touches(gap, half, touching.here(point(foot), scale), scale) {
            vec![(foot, true)]
        } else {
            vec![(foot - along * half, false), (foot + along * half, false)]
        };
        for (at, touch) in crossed {
            if at.y * mirror < -eps {
                continue;
            }
            let here = point(at);
            if points.iter().any(|known| known.distance(here) <= eps) {
                continue;
            }
            points.push(here);
            found.push((circle.parameter(here), touch));
        }
    }
    Solved::At(found)
}
