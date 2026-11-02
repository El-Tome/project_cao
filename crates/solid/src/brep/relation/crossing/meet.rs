//! The curve two perpendicular walls meet along, against a surface through
//! the lines one of its walls makes with it (decision 4's triple read from
//! its most degenerate pair), the other wall a cylinder or an elliptic one.

use super::super::cylinders::cylinders;
use super::super::planar::plane_and_cylinder;
use super::super::{Relation, relation};
use super::cut::Cut;
use super::elliptic::line_and_elliptic;
use super::{Solved, Touches, line_and_cylinder};
use crate::brep::curve::{Curve, Line, Meet, Wall};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

/// The curve two walls meet along, against a surface one of the two cuts
/// along lines or touches along one — a wall parallel to it, a plane along
/// its axis: a triple is solved from its most degenerate pair, so the
/// crossings are where those lines pass through the other wall, each kept
/// where it stands on this component. A line the surface only touches gives
/// only touches. None where neither wall makes lines with the surface, which
/// is left to the scan.
pub(super) fn through_lines(
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
    let cut = match (surface, &own) {
        (Surface::Plane(plane), Wall::Cylinder(cylinder)) => Some(Cut { cylinder, plane }),
        (Surface::Plane(_), Wall::Elliptic(_))
        | (
            Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Elliptic(_)
            | Surface::Sphere(_)
            | Surface::Torus(_),
            _,
        ) => None,
    };
    let curve = Curve::Meet(*meet);
    let mut found = Vec::new();
    for (line, along_a_touch) in lines {
        let cut = cut.as_ref().filter(|_| !along_a_touch);
        let crossed = match other {
            Wall::Cylinder(other) => line_and_cylinder(&line, &other, scale, touching, cut),
            Wall::Elliptic(other) => line_and_elliptic(&line, &other, scale, touching),
        };
        let Solved::At(crossed) = crossed else {
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

/// How a wall of a meet and a surface meet, the pair decided as the
/// relation of any two surfaces is.
fn relation_with(own: &Wall, surface: &Surface, scale: Scale) -> Relation {
    match (own, surface) {
        (Wall::Cylinder(own), Surface::Plane(plane)) => plane_and_cylinder(plane, own, scale),
        (Wall::Cylinder(own), Surface::Cylinder(cylinder)) => cylinders(own, cylinder, scale),
        (
            Wall::Cylinder(_),
            Surface::Cone(_) | Surface::Elliptic(_) | Surface::Sphere(_) | Surface::Torus(_),
        ) => Relation::Unsupported,
        (Wall::Elliptic(own), Surface::Plane(_) | Surface::Cylinder(_) | Surface::Elliptic(_)) => {
            relation(&Surface::Elliptic(*own), surface, scale)
        }
        (Wall::Elliptic(_), Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_)) => {
            Relation::Unsupported
        }
    }
}
