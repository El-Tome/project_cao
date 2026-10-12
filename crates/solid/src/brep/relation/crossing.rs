//! Where a curve crosses a surface: what the corners of a triple stand on.
//!
//! A line or a circle against a plane or a cylinder is solved in closed form,
//! exact but for rounding. A double root is one touch where the curve passes
//! outside the surface within the tolerance, where its two roots stand
//! within the tolerance of each other, or where a surface it lies on was
//! decided to touch the other there ([`Touches`], decision 4): a line a hair
//! inside a wall otherwise crosses it twice, its two roots many tolerances
//! apart, and a corner between them would stand that far off every other
//! curve through either. The curve two perpendicular cylinders
//! meet along is solved through the lines the surface makes with one of its
//! two cylinders where it makes any, and numerically otherwise, in `scan.rs`.
//! A line a plane cuts a cylinder along, crossed with a circle or a cylinder
//! square to it, is measured from the radii (`cut.rs`). A line or a circle
//! against a cone is solved in `cone.rs`.

mod cone;
mod cut;
mod scan;

use glam::DVec3;

use super::cylinders::cylinders;
use super::planar::{plane_and_cylinder, planes};
use super::{Relation, parallel, square};
use crate::brep::curve::{Circle, Curve, Line, Meet};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use cut::Cut;

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
    /// A circle against a cylinder at a skew angle, or against a cone
    /// unless about an axis parallel to the cone's or in a plane holding
    /// it; the curve two cylinders meet along against a cone.
    Unsupported,
}

/// Parameters on the curve, each with whether the curve only touches the
/// surface there.
enum Solved {
    At(Vec<(f64, bool)>),
    Along,
    Unsupported,
}

/// What the surfaces a curve lies on were decided to make with a surface it
/// crosses (decision 2): whether one of them touches it along a line — a
/// plane touching a wall, two parallel walls touching — and the points where
/// one of them touches it at a point, a node or the contact of two
/// perpendicular cylinders. A double root within the tolerance of the
/// surface, inside it or out, is a touch where it stands on such a line or
/// within the tolerance of such a point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Touches {
    pub along: bool,
    pub at: Vec<DVec3>,
}

impl Touches {
    fn here(&self, point: DVec3, scale: Scale) -> bool {
        self.along || self.at.iter().any(|at| at.distance(point) <= scale.eps())
    }
}

/// Where a curve lying on no surface decided to touch `surface` crosses it.
pub fn crossings(curve: &Curve, surface: &Surface, scale: Scale) -> Crossings {
    crossings_given(curve, surface, scale, &Touches::default())
}

/// Where a curve crosses `surface`, given what the surfaces it lies on were
/// decided to make with it.
pub fn crossings_given(
    curve: &Curve,
    surface: &Surface,
    scale: Scale,
    touching: &Touches,
) -> Crossings {
    let solved = match (curve, surface) {
        (Curve::Line(line), Surface::Cone(cone)) => {
            cone::line_and_cone(line, cone, scale, touching)
        }
        (Curve::Circle(circle), Surface::Cone(cone)) => {
            cone::circle_and_cone(circle, cone, scale, touching)
        }
        (Curve::Meet(_), Surface::Cone(_)) => Solved::Unsupported,
        (Curve::Line(line), Surface::Plane(plane)) => line_and_plane(line, plane, scale),
        (Curve::Line(line), Surface::Cylinder(cylinder)) => {
            line_and_cylinder(line, cylinder, scale, touching, None)
        }
        (Curve::Circle(circle), surface) => circle_and_surface(circle, surface, scale, touching),
        (Curve::Meet(meet), surface) => meet_through_lines(meet, surface, scale, touching)
            .unwrap_or_else(|| scan::meet_and_surface(meet, surface, scale)),
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
fn circle_and_surface(
    circle: &Circle,
    surface: &Surface,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let (own_plane, _) = Plane::through(circle.center, circle.axis);
    let own_cylinder = Cylinder::about(circle.center, circle.axis, circle.radius);
    let (relation, across, cut) = match surface {
        Surface::Plane(plane) => (planes(&own_plane, plane, scale), true, None),
        Surface::Cylinder(cylinder) if parallel(circle.axis, cylinder.axis, scale) => {
            (cylinders(&own_cylinder, cylinder, scale), false, None)
        }
        Surface::Cylinder(cylinder) => (
            plane_and_cylinder(&own_plane, cylinder, scale),
            true,
            Some(Cut {
                cylinder,
                plane: &own_plane,
            }),
        ),
        Surface::Cone(_) => return Solved::Unsupported,
    };
    circle_through(circle, relation, (across, cut), scale, touching)
}

/// What the circle's own plane, `across`, or its own cylinder makes with a
/// surface, against the circle: a line in its plane crosses it where it
/// passes through it, a line along its cylinder at its height. Lines the
/// plane cuts a cylinder along are read as that `cut`.
fn circle_through(
    circle: &Circle,
    relation: Relation,
    (across, cut): (bool, Option<Cut>),
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let lines: Vec<(Line, bool, Option<&Cut>)> = match &relation {
        Relation::Same { .. } => return Solved::Along,
        Relation::Apart => Vec::new(),
        Relation::Line(line) => vec![(*line, false, None)],
        Relation::Lines(lines) => lines.map(|line| (line, false, cut.as_ref())).to_vec(),
        Relation::Tangent(line) => vec![(*line, true, None)],
        Relation::Circle(_)
        | Relation::Meet(_)
        | Relation::Rulings { .. }
        | Relation::Apex(_)
        | Relation::Unsupported => {
            return Solved::Unsupported;
        }
    };
    let mut found = Vec::new();
    for (line, along_a_touch, cut) in lines {
        if across {
            found.extend(line_across_circle(
                &line,
                circle,
                (along_a_touch, cut),
                touching,
                scale,
            ));
        } else {
            let height =
                (circle.center - line.origin).dot(circle.axis) / line.direction.dot(circle.axis);
            found.push((circle.parameter(line.point(height)), along_a_touch));
        }
    }
    Solved::At(found)
}

/// The curve two cylinders meet along, against a surface one of the two
/// cuts along lines or touches along one — a cylinder parallel to it, a
/// plane along its axis: a triple is solved from its most degenerate pair,
/// so the crossings are where those lines pass through the other cylinder,
/// each kept where it stands on this component. A line the surface only
/// touches gives only touches. None where neither cylinder makes lines with
/// the surface, which is left to the scan.
fn meet_through_lines(
    meet: &Meet,
    surface: &Surface,
    scale: Scale,
    touching: &Touches,
) -> Option<Solved> {
    let (lines, own, other) = [(meet.first, meet.second), (meet.second, meet.first)]
        .into_iter()
        .find_map(|(own, other)| {
            let lines: Vec<(Line, bool)> = match relation_with(&own, surface, scale) {
                Relation::Lines(lines) => lines.map(|line| (line, false)).to_vec(),
                Relation::Tangent(line) => vec![(line, true)],
                _ => return None,
            };
            Some((lines, own, other))
        })?;
    let cut = match surface {
        Surface::Plane(plane) => Some(Cut {
            cylinder: &own,
            plane,
        }),
        Surface::Cylinder(_) | Surface::Cone(_) => None,
    };
    let curve = Curve::Meet(*meet);
    let mut found = Vec::new();
    for (line, along_a_touch) in lines {
        let cut = cut.as_ref().filter(|_| !along_a_touch);
        let Solved::At(crossed) = line_and_cylinder(&line, &other, scale, touching, cut) else {
            continue;
        };
        for (at, touch) in crossed {
            let point = line.point(at);
            let parameter = meet.parameter(point);
            if curve.point(parameter).distance(point) <= scale.eps() {
                found.push((parameter, along_a_touch || touch));
            }
        }
    }
    Some(Solved::At(found))
}

/// How a cylinder of a meet and a surface meet, the pair decided as the
/// relation of any two surfaces is.
fn relation_with(own: &Cylinder, surface: &Surface, scale: Scale) -> Relation {
    match surface {
        Surface::Plane(plane) => plane_and_cylinder(plane, own, scale),
        Surface::Cylinder(cylinder) => cylinders(own, cylinder, scale),
        Surface::Cone(_) => Relation::Unsupported,
    }
}

/// A line lying in the circle's plane passes the centre at its closest, and
/// touches the circle there when it passes within the tolerance of the
/// radius, as `touches` decides. A line a plane cuts a cylinder along is
/// measured as that `cut`.
fn line_across_circle(
    line: &Line,
    circle: &Circle,
    (along_a_touch, cut): (bool, Option<&Cut>),
    touching: &Touches,
    scale: Scale,
) -> Vec<(f64, bool)> {
    let closest = line.point(line.parameter(circle.center));
    let gap = cut.map_or_else(
        || (closest - circle.center).length() - circle.radius,
        |cut| cut.gap(line, circle.center, circle.axis, circle.radius),
    );
    if gap > scale.eps() {
        return Vec::new();
    }
    let half = (-gap * (2.0 * circle.radius + gap)).max(0.0).sqrt();
    if touches(gap, half, touching.here(closest, scale), scale) {
        return vec![(circle.parameter(closest), true)];
    }
    [-half, half]
        .map(|along| {
            (
                circle.parameter(closest + line.direction * along),
                along_a_touch,
            )
        })
        .to_vec()
}

/// Whether a curve passing `gap` from a surface, outside it or within it,
/// its two roots `half` either side of where it passes closest, only
/// touches it there: within the tolerance outside, its roots within the
/// tolerance of each other inside, or anywhere within the tolerance where
/// it passes closest on a touch decided there. A cap a hair from the node of
/// two cylinders touching inside cuts one along a ruling a femtometre inside
/// the other, which it crosses a fraction of a micron either side of the
/// node, where the curve the two meet along passes; the ruling through the
/// node only touches the other there, whatever rounding left of the touch.
fn touches(gap: f64, half: f64, touching: bool, scale: Scale) -> bool {
    let eps = scale.eps();
    gap.abs() <= eps && (touching || gap >= 0.0 || half <= eps)
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
/// closest, and touches when that is within the tolerance of the radius. A
/// line a plane cuts a cylinder square to this one along is measured as that
/// `cut`.
fn line_and_cylinder(
    line: &Line,
    cylinder: &Cylinder,
    scale: Scale,
    touching: &Touches,
    cut: Option<&Cut>,
) -> Solved {
    let flat = |v: DVec3| v - cylinder.axis * cylinder.axis.dot(v);
    let (from, towards) = (flat(line.origin - cylinder.origin), flat(line.direction));
    if parallel(line.direction, cylinder.axis, scale) {
        return beside(from.length() - cylinder.radius, scale);
    }
    let speed = towards.length_squared();
    let closest = -from.dot(towards) / speed;
    let gap = cut.map_or_else(
        || (from + towards * closest).length() - cylinder.radius,
        |cut| cut.gap(line, cylinder.origin, cylinder.axis, cylinder.radius),
    );
    if gap > scale.eps() {
        return Solved::At(Vec::new());
    }
    let half = (-gap * (2.0 * cylinder.radius + gap) / speed)
        .max(0.0)
        .sqrt();
    if touches(gap, half, touching.here(line.point(closest), scale), scale) {
        return Solved::At(vec![(closest, true)]);
    }
    Solved::At(vec![(closest - half, false), (closest + half, false)])
}
