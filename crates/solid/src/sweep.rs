//! How a flat area of a drawing becomes matter: pushed straight along a
//! direction, or swept around an axis.

use glam::{DVec2, DVec3};

use crate::mesh::{Mesh, Polygon};

/// Turns a flat area into a prism: the face at the bottom, the same face moved
/// along `direction` at the top, and a wall for every edge between the two.
///
/// `outline` and `holes` are in the plane's own 2D coordinates; `to_world`
/// places them in space. A hole gets walls too — that is what makes the inside
/// of a tube a surface rather than an opening.
pub fn prism(
    outline: &[DVec2],
    holes: &[Vec<DVec2>],
    triangles: &[[DVec2; 3]],
    to_world: impl Fn(DVec2) -> DVec3,
    direction: DVec3,
) -> Mesh {
    let mut polygons = Vec::new();

    for triangle in triangles {
        let bottom: Vec<DVec3> = triangle.iter().map(|corner| to_world(*corner)).collect();
        let top: Vec<DVec3> = bottom.iter().map(|corner| *corner + direction).collect();

        // The two caps face opposite ways, so one of them is wound backwards.
        if let Some(polygon) = Polygon::new(bottom.clone()) {
            let outward = if polygon.normal().dot(direction) > 0.0 {
                polygon.flipped()
            } else {
                polygon
            };
            polygons.push(outward);
        }
        if let Some(polygon) = Polygon::new(top) {
            let outward = if polygon.normal().dot(direction) < 0.0 {
                polygon.flipped()
            } else {
                polygon
            };
            polygons.push(outward);
        }
    }

    // Which way the walls face depends both on how the loop turns and on which
    // side of the plane the matter is being pushed to.
    let origin = to_world(DVec2::ZERO);
    let normal = (to_world(DVec2::X) - origin)
        .cross(to_world(DVec2::Y) - origin)
        .normalize_or(DVec3::Z);
    let along = direction.dot(normal) > 0.0;

    let mut walls = |loop_points: &[DVec2], inward: bool| {
        for index in 0..loop_points.len() {
            let a = to_world(loop_points[index]);
            let b = to_world(loop_points[(index + 1) % loop_points.len()]);
            let corners = if inward {
                vec![b, a, a + direction, b + direction]
            } else {
                vec![a, b, b + direction, a + direction]
            };
            if let Some(polygon) = Polygon::new(corners) {
                polygons.push(polygon);
            }
        }
    };

    walls(outline, (signed_area(outline) > 0.0) != along);
    for hole in holes {
        // A hole's wall looks the other way: its matter is on the outside.
        walls(hole, (signed_area(hole) > 0.0) == along);
    }

    Mesh { polygons }
}

/// Turns a flat area into a solid of revolution: the face swept around an axis
/// lying in its own plane.
///
/// `axis_origin` and `axis_direction` are given in the plane's own 2D
/// coordinates, since that is where the user picks them — one of the sketch
/// axes, or a line they drew.
///
/// Returns `None` when the face straddles the axis: sweeping it would turn the
/// solid inside out through itself, and no amount of care afterwards recovers a
/// shape from that.
pub fn revolution(
    outline: &[DVec2],
    holes: &[Vec<DVec2>],
    triangles: &[[DVec2; 3]],
    to_world: impl Fn(DVec2) -> DVec3,
    axis_origin: DVec2,
    axis_direction: DVec2,
    turn: f64,
) -> Option<Mesh> {
    let along = axis_direction.normalize_or_zero();
    if along == DVec2::ZERO || turn.abs() < 1e-4 {
        return None;
    }

    // Everything must sit on one side of the axis. A profile crossing it would
    // sweep through itself.
    let side = |point: DVec2| along.perp_dot(point - axis_origin);
    let sides: Vec<f64> = outline
        .iter()
        .chain(holes.iter().flatten())
        .map(|point| side(*point))
        .collect();
    let furthest = sides.iter().fold(0.0f64, |far, each| far.max(each.abs()));
    if furthest < 1e-6 {
        return None;
    }
    let sign = sides
        .iter()
        .find(|distance| distance.abs() > furthest * 1e-3)
        .map(|distance| distance.signum())?;
    if sides
        .iter()
        .any(|distance| distance * sign < -furthest * 1e-3)
    {
        return None;
    }

    let origin = to_world(axis_origin);
    let axis = (to_world(axis_origin + along) - origin).normalize_or(DVec3::Z);
    let full = (turn.abs() - std::f64::consts::TAU).abs() < 1e-3;

    // Enough steps that the flats read as a curve, scaled to how far it turns.
    let steps = ((turn.abs() / std::f64::consts::TAU) * 64.0)
        .ceil()
        .max(3.0) as usize;
    let at = |point: DVec2, step: usize| {
        let angle = turn * step as f64 / steps as f64;
        let world = to_world(point) - origin;
        origin + glam::DQuat::from_axis_angle(axis, angle) * world
    };

    let mut polygons = Vec::new();

    // Walls, as triangles rather than quads: a quad swept around an axis is
    // bent, and the boolean operations sort faces by the plane they lie on.
    let mut wall = |loop_points: &[DVec2], flip: bool| {
        for index in 0..loop_points.len() {
            let (a, b) = (
                loop_points[index],
                loop_points[(index + 1) % loop_points.len()],
            );
            for step in 0..steps {
                let (a0, b0) = (at(a, step), at(b, step));
                let (a1, b1) = (at(a, step + 1), at(b, step + 1));
                let faces = if flip {
                    [[a0, b0, b1], [a0, b1, a1]]
                } else {
                    [[a0, b1, b0], [a0, a1, b1]]
                };
                polygons.extend(
                    faces
                        .into_iter()
                        .filter_map(|face| Polygon::new(face.to_vec())),
                );
            }
        }
    };

    // Which way the walls face depends on how the loop turns, which side of the
    // axis it sits on, and which way the sweep goes.
    let outward = (signed_area(outline) > 0.0) == (sign * turn > 0.0);
    wall(outline, outward);
    for hole in holes {
        wall(hole, (signed_area(hole) > 0.0) != (sign * turn > 0.0));
    }

    // A full turn closes on itself and needs no ends.
    if !full {
        for triangle in triangles {
            let start: Vec<DVec3> = triangle.iter().map(|point| at(*point, 0)).collect();
            let end: Vec<DVec3> = triangle.iter().map(|point| at(*point, steps)).collect();
            for (corners, closing) in [(start, false), (end, true)] {
                let Some(polygon) = Polygon::new(corners) else {
                    continue;
                };
                let outward = polygon
                    .normal()
                    .dot(axis.cross(polygon.corners[0] - origin));
                let facing = (outward > 0.0) == closing;
                polygons.push(if facing { polygon } else { polygon.flipped() });
            }
        }
    }

    Some(Mesh { polygons })
}

fn signed_area(loop_points: &[DVec2]) -> f64 {
    let mut total = 0.0;
    for index in 0..loop_points.len() {
        total += loop_points[index].perp_dot(loop_points[(index + 1) % loop_points.len()]);
    }
    total * 0.5
}

#[cfg(test)]
mod tests;
