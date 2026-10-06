//! Where a line or a circle crosses a cone (#536, decision 4).
//!
//! A line or a circle square to the axis crosses the cone where it crosses
//! the cylinder of the cone's section at its height, which is the very
//! cylinder a plane square to the axis cuts the cone along: the corner is
//! the one the circle they meet along gives. Through the apex, a line
//! crosses once, or only touches where it stays outside on both sides. Any
//! other line crosses at the roots of the cone's quadric kept on its own
//! nappe, and touches it once where it passes within the tolerance of it at
//! its closest, as decided for a cylinder. A circle in a plane holding the
//! axis crosses the two rulings that plane cuts; any other circle is
//! unsupported, as the pair of surfaces it lies on is.

use glam::DVec3;

use super::super::cylinders::cylinders;
use super::super::revolved::in_the_box;
use super::super::{Relation, parallel, relation, square};
use super::{Solved, Touches, circle_through, line_and_cylinder, touches};
use crate::brep::curve::{Circle, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, Plane, Surface};

/// Along a ruling, square to the axis, through the apex, or anywhere else,
/// in that order: the first that holds decides.
pub(super) fn line_and_cone(line: &Line, cone: &Cone, scale: Scale, touching: &Touches) -> Solved {
    let eps = scale.eps();
    if cone.rules(line, scale) {
        return Solved::Along;
    }
    let apex = cone.apex();
    let through = in_the_box(apex, scale) && passing(line, apex) <= eps;
    if cone.ruling.x != 0.0 && square(line.direction, cone.axis, scale) {
        let height = (nearest_the_axis(line, cone) - cone.origin).dot(cone.axis);
        let radius = cone.section(height);
        if radius > eps {
            let section = Cylinder::about(cone.origin, cone.axis, radius);
            return line_and_cylinder(line, &section, scale, touching);
        }
        if radius < -eps || !through {
            return Solved::At(Vec::new());
        }
        return Solved::At(vec![(line.parameter(apex), true)]);
    }
    if through {
        return Solved::At(vec![(
            line.parameter(apex),
            outside_both_ways(line, cone, scale),
        )]);
    }
    let roots = cone.roots(line.origin, line.direction);
    if let Some((closest, gap)) = closest(line, cone) {
        if gap > eps {
            return Solved::At(Vec::new());
        }
        let half = match roots {
            [Some(one), Some(other)] => (other - one) / 2.0,
            _ => 0.0,
        };
        let here = touching.here(line.point(closest), scale);
        if gap >= -eps && (roots[0].is_none() || touches(gap, half, here, scale)) {
            return Solved::At(vec![(closest, true)]);
        }
    }
    Solved::At(roots.into_iter().flatten().map(|at| (at, false)).collect())
}

/// How far a line passes from a point.
fn passing(line: &Line, point: DVec3) -> f64 {
    line.point(line.parameter(point)).distance(point)
}

/// The line's point nearest the axis.
fn nearest_the_axis(line: &Line, cone: &Cone) -> DVec3 {
    let flat = |v: DVec3| v - cone.axis * cone.axis.dot(v);
    let (from, towards) = (flat(line.origin - cone.origin), flat(line.direction));
    let speed = towards.length_squared();
    if speed == 0.0 {
        return line.origin;
    }
    line.point(-from.dot(towards) / speed)
}

/// Whether a line through the apex stays outside the nappe on both sides
/// of it across the box: how far it stands off the cone grows on either side
/// at a rate that is the line's spread from the axis against its rise along
/// it, and never falls more than the tolerance below nought over the box.
fn outside_both_ways(line: &Line, cone: &Cone, scale: Scale) -> bool {
    let rise = line.direction.dot(cone.axis);
    let spread = (line.direction - cone.axis * rise).length();
    let away = cone.ruling.x.abs() * spread;
    let towards = (cone.ruling.y * rise).abs();
    (away - towards) * 2.0 * scale.reach() >= -scale.eps()
}

/// Where the line passes closest to the nappe, and how far outside it it
/// stands there, negative within it: where `G(t) = |w_h|·|q(t)| −
/// sgn(w_h)·(c + w_ρ·h(t))`, which is the distance to the cone in front of
/// its apex and convex, is least. None where `G` only falls or only rises
/// along the line, which then crosses the nappe once at most.
fn closest(line: &Line, cone: &Cone) -> Option<(f64, f64)> {
    let from = line.origin - cone.origin;
    let rise = line.direction.dot(cone.axis);
    let spread = line.direction - cone.axis * rise;
    let speed = spread.length();
    let [w_h, w_rho] = cone.ruling.to_array();
    let (away, towards) = (w_h.abs() * speed, (w_rho * rise).abs());
    if away <= towards {
        return None;
    }
    let level = |at: f64| {
        let point = from + line.direction * at;
        let h = point.dot(cone.axis);
        let radial = (point - cone.axis * h).length();
        w_h.abs() * radial - w_h.signum() * (cone.ruling.perp().dot(cone.foot) + w_rho * h)
    };
    let radial = from - cone.axis * from.dot(cone.axis);
    let square_to = -radial.dot(spread) / (speed * speed);
    let off = (radial + spread * square_to).length();
    let at = square_to
        + w_h.signum() * w_rho * rise * off
            / (speed * ((away - towards) * (away + towards)).sqrt());
    Some((at, level(at)))
}

/// A circle about an axis parallel to the cone's crosses it as it crosses
/// the cylinder of the cone's section at its height, lies along it where the
/// cone holds it, and touches the apex where it passes through it. One in a
/// plane holding the axis crosses the rulings that plane cuts, on the cone's
/// own nappe; at the apex, where it crosses both, it is a line through the
/// apex along its tangent there that decides whether it only touches.
pub(super) fn circle_and_cone(
    circle: &Circle,
    cone: &Cone,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let eps = scale.eps();
    if cone.ruling.x != 0.0 && parallel(circle.axis, cone.axis, scale) {
        if cone.holds(circle, eps) {
            return Solved::Along;
        }
        let radius = cone.section((circle.center - cone.origin).dot(cone.axis));
        if radius > eps {
            let own = Cylinder::about(circle.center, circle.axis, circle.radius);
            let section = Cylinder::about(cone.origin, cone.axis, radius);
            return circle_through(
                circle,
                cylinders(&own, &section, scale),
                false,
                scale,
                touching,
            );
        }
        let apex = cone.apex();
        let at = circle.parameter(apex);
        if radius < -eps || circle.point(at).distance(apex) > eps {
            return Solved::At(Vec::new());
        }
        return Solved::At(vec![(at, true)]);
    }
    let (own_plane, _) = Plane::through(circle.center, circle.axis);
    let Relation::Rulings { lines, .. } =
        relation(&Surface::Plane(own_plane), &Surface::Cone(*cone), scale)
    else {
        return Solved::Unsupported;
    };
    let apex = cone.apex();
    match circle_through(circle, Relation::Lines(lines), true, scale, touching) {
        Solved::At(found) => {
            let mut kept: Vec<(f64, bool)> = Vec::new();
            for (at, touch) in found {
                let point = circle.point(at);
                let twice = kept
                    .iter()
                    .any(|(other, _)| circle.point(*other).distance(point) <= eps);
                if cone.distance(point).abs() > eps || twice {
                    continue;
                }
                if point.distance(apex) <= eps {
                    let tangent = Line::through(point, circle.derivative(at));
                    kept.push((at, outside_both_ways(&tangent, cone, scale)));
                } else {
                    kept.push((at, touch));
                }
            }
            Solved::At(kept)
        }
        other => other,
    }
}
