//! Where a circle crosses a surface: its own plane cut by its own cylinder,
//! what the surface makes with the one or the other, against the circle.

use crate::brep::curve::{Circle, Line};
use crate::brep::relation::cylinders::cylinders;
use crate::brep::relation::elliptic::plane_and_elliptic;
use crate::brep::relation::planar::{plane_and_cylinder, planes};
use crate::brep::relation::walls::round_and_elliptic;
use crate::brep::relation::{Relation, parallel, square};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};

use super::cut::Cut;
use super::{Solved, Touches, touches};

/// A circle is its own plane cut by its own cylinder: what the surface makes
/// with the one or the other — decided as the relation of that pair decides
/// it — is lines, and those lines against the circle are the crossings. A
/// line the surface only touches gives only touches.
pub(super) fn circle_and_surface(
    circle: &Circle,
    surface: &Surface,
    scale: Scale,
    touching: &Touches,
) -> Solved {
    let (own_plane, _) = Plane::through(circle.center, circle.axis);
    let own_cylinder = Cylinder::about(circle.center, circle.axis, circle.radius);
    let (relation, across, cut) = match surface {
        Surface::Elliptic(wall) if square(circle.axis, wall.axis, scale) => {
            (plane_and_elliptic(&own_plane, wall, scale), true, None)
        }
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
        Surface::Elliptic(wall) if parallel(circle.axis, wall.axis, scale) => {
            (round_and_elliptic(&own_cylinder, wall, scale), false, None)
        }
        Surface::Sphere(sphere) => {
            return held(sphere.holds(circle, scale.eps()) || sphere.meridian(circle, scale.eps()));
        }
        Surface::Torus(torus) => {
            return held(torus.holds(circle, scale.eps()) || torus.meridian(circle, scale.eps()));
        }
        Surface::Cone(_) | Surface::Elliptic(_) => return Solved::Unsupported,
    };
    circle_through(circle, relation, (across, cut), scale, touching)
}

/// A circle a sphere or a torus holds, as a latitude or a meridian, lies
/// along it; any other crosses it nowhere the kernel builds a corner at: a
/// meridian circle against a coaxial surface is that pair's own relation.
fn held(on: bool) -> Solved {
    if on {
        Solved::Along
    } else {
        Solved::At(Vec::new())
    }
}

/// What the circle's own plane, `across`, or its own cylinder makes with a
/// surface, against the circle: a line in its plane crosses it where it
/// passes through it, a line along its cylinder at its height. Lines the
/// plane cuts a cylinder along are read as that `cut`.
pub(super) fn circle_through(
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
        Relation::Tangent(line) | Relation::Ruling { line, .. } => vec![(*line, true, None)],
        Relation::Parallel(rulings) => rulings
            .iter()
            .map(|ruled| (ruled.line, ruled.touching, None))
            .collect(),
        Relation::Circle(_)
        | Relation::Meet(_)
        | Relation::Rulings { .. }
        | Relation::Apex(_)
        | Relation::Conic(_)
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
