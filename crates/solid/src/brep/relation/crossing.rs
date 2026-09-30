//! Where a curve crosses a surface: what the corners of a triple stand on.
//!
//! A line or a circle against a plane or a cylinder is solved in closed form,
//! exact but for rounding, a double root decided within the tolerance as one
//! touch. The curve two perpendicular cylinders meet along is solved
//! numerically, in `scan.rs`.

mod scan;

use glam::DVec3;

use super::cylinders::cylinders;
use super::planar::{plane_and_cylinder, planes};
use super::{Relation, parallel, square};
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};

/// A point where a curve meets a surface, at its parameter on the curve.
/// `tangent` when the curve touches the surface there without passing through.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crossing {
    pub parameter: f64,
    pub point: DVec3,
    pub tangent: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Crossings {
    /// Sorted along the curve.
    At(Vec<Crossing>),
    /// The curve lies on the surface, within the tolerance.
    Along,
    /// A circle against a cylinder at a skew angle.
    Unsupported,
}

/// Parameters on the curve, each with whether the curve only touches the
/// surface there.
enum Solved {
    At(Vec<(f64, bool)>),
    Along,
    Unsupported,
}

pub fn crossings(curve: &Curve, surface: &Surface, scale: Scale) -> Crossings {
    let solved = match (curve, surface) {
        (Curve::Line(line), Surface::Plane(plane)) => line_and_plane(line, plane, scale),
        (Curve::Line(line), Surface::Cylinder(cylinder)) => {
            line_and_cylinder(line, cylinder, scale)
        }
        (Curve::Circle(circle), surface) => circle_and_surface(circle, surface, scale),
        (Curve::Meet(meet), surface) => scan::meet_and_surface(meet, surface, scale),
    };
    match solved {
        Solved::At(found) => at(curve, found),
        Solved::Along => Crossings::Along,
        Solved::Unsupported => Crossings::Unsupported,
    }
}

/// A circle is its own plane cut by its own cylinder: what the surface makes
/// with the one or the other — decided as the relation of that pair decides
/// it — is lines, and those lines against the circle are the crossings. A
/// line the surface only touches gives only touches.
fn circle_and_surface(circle: &Circle, surface: &Surface, scale: Scale) -> Solved {
    let (own_plane, _) = Plane::through(circle.center, circle.axis);
    let own_cylinder = Cylinder::about(circle.center, circle.axis, circle.radius);
    let (relation, across) = match surface {
        Surface::Plane(plane) => (planes(&own_plane, plane, scale), true),
        Surface::Cylinder(cylinder) if parallel(circle.axis, cylinder.axis, scale) => {
            (cylinders(&own_cylinder, cylinder, scale), false)
        }
        Surface::Cylinder(cylinder) => (plane_and_cylinder(&own_plane, cylinder, scale), true),
    };
    let lines: Vec<(Line, bool)> = match relation {
        Relation::Same { .. } => return Solved::Along,
        Relation::Apart => Vec::new(),
        Relation::Line(line) => vec![(line, false)],
        Relation::Lines(lines) => lines.map(|line| (line, false)).to_vec(),
        Relation::Tangent(line) => vec![(line, true)],
        Relation::Circle(_) | Relation::Meet(_) | Relation::Unsupported => {
            return Solved::Unsupported;
        }
    };
    let mut found = Vec::new();
    for (line, touching) in lines {
        if across {
            found.extend(line_across_circle(&line, circle, touching, scale));
        } else {
            let height =
                (circle.center - line.origin).dot(circle.axis) / line.direction.dot(circle.axis);
            found.push((circle.parameter(line.point(height)), touching));
        }
    }
    Solved::At(found)
}

/// A line lying in the circle's plane passes the centre at its closest, and
/// touches the circle when that is within the tolerance of the radius.
fn line_across_circle(
    line: &Line,
    circle: &Circle,
    touching: bool,
    scale: Scale,
) -> Vec<(f64, bool)> {
    let closest = line.point(line.parameter(circle.center));
    let passing = (closest - circle.center).length();
    let gap = passing - circle.radius;
    if gap > scale.eps() {
        return Vec::new();
    }
    if gap >= -scale.eps() {
        return vec![(circle.parameter(closest), true)];
    }
    let half = ((circle.radius - passing) * (circle.radius + passing)).sqrt();
    [-half, half]
        .map(|along| (circle.parameter(closest + line.direction * along), touching))
        .to_vec()
}

/// The parameters found, as crossings on the curve, sorted.
fn at(curve: &Curve, found: Vec<(f64, bool)>) -> Crossings {
    let mut crossings: Vec<Crossing> = found
        .into_iter()
        .map(|(parameter, tangent)| Crossing {
            parameter,
            point: curve.point(parameter),
            tangent,
        })
        .collect();
    crossings.sort_by(|one, other| one.parameter.total_cmp(&other.parameter));
    Crossings::At(crossings)
}

/// Linear. Nothing when the line runs along the plane.
fn line_and_plane(line: &Line, plane: &Plane, scale: Scale) -> Solved {
    let away = plane.distance(line.origin);
    if square(line.direction, plane.normal, scale) {
        return beside(away, scale);
    }
    Solved::At(vec![(-away / line.direction.dot(plane.normal), false)])
}

/// A curve running beside a surface at a constant distance lies on it within
/// the tolerance, or never meets it.
fn beside(away: f64, scale: Scale) -> Solved {
    if away.abs() <= scale.eps() {
        Solved::Along
    } else {
        Solved::At(Vec::new())
    }
}

/// Quadratic, seen square to the axis: the line passes the axis at its
/// closest, and touches when that is within the tolerance of the radius.
fn line_and_cylinder(line: &Line, cylinder: &Cylinder, scale: Scale) -> Solved {
    let flat = |v: DVec3| v - cylinder.axis * cylinder.axis.dot(v);
    let (from, towards) = (flat(line.origin - cylinder.origin), flat(line.direction));
    if parallel(line.direction, cylinder.axis, scale) {
        return beside(from.length() - cylinder.radius, scale);
    }
    let speed = towards.length_squared();
    let closest = -from.dot(towards) / speed;
    let passing = (from + towards * closest).length();
    let gap = passing - cylinder.radius;
    if gap > scale.eps() {
        return Solved::At(Vec::new());
    }
    if gap >= -scale.eps() {
        return Solved::At(vec![(closest, true)]);
    }
    let half = ((cylinder.radius - passing) * (cylinder.radius + passing) / speed).sqrt();
    Solved::At(vec![(closest - half, false), (closest + half, false)])
}
