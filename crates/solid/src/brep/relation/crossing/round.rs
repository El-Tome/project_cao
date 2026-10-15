//! Where a line crosses a sphere or a torus (#576, decision 4).
//!
//! A sphere is crossed at the roots of its quadratic, and touched once where
//! the line passes within the tolerance of it at its closest, as a wall is.
//! A torus is solved only where the line lies in a plane holding the axis:
//! there its section is two circles of the tube's radius, about `(h_c, ±R)`
//! in the plane's `(h, s)`, each the surface on its own side of the axis,
//! and the line crosses each as it crosses a wall. Any other line meets the
//! torus along the roots of a quartic no corner of the kernel's pairs stands
//! on: unsupported, as the pair it would lie on is.

use super::super::parallel;
use super::{Solved, Touches, touches};
use crate::brep::curve::Line;
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
