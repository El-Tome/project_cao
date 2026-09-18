//! How a flat area of a drawing becomes matter: pushed straight along a
//! direction, or swept around an axis.

use glam::{DVec2, DVec3};

use crate::mesh::{Mesh, Polygon};

/// One closed loop a solid is raised from, and which of its segments came from
/// the same curve.
///
/// Segment `index` runs from `points[index]` to the point after it, and
/// `curves[index]` names the curve it was sampled from — `None` for a trait
/// drawn straight. Without it, the wall raised from a circle would come out as
/// as many faces as the circle was sampled into.
#[derive(Clone, Copy, Debug)]
pub struct Loop<'a> {
    pub points: &'a [DVec2],
    pub curves: &'a [Option<usize>],
}

impl<'a> Loop<'a> {
    /// A loop with no curve in it: every segment is a trait drawn straight.
    pub fn straight(points: &'a [DVec2]) -> Self {
        Self {
            points,
            curves: &[],
        }
    }

    /// The curve a segment was sampled from. A loop given fewer marks than it
    /// has segments reads as drawn straight rather than refusing to build.
    fn curve(&self, index: usize) -> Option<usize> {
        self.curves.get(index).copied().flatten()
    }
}

/// Hands out a number per stretch of surface, and remembers which one a curve
/// already has so that every wall sampled from it lands on the same face.
#[derive(Default)]
struct Faces {
    next: usize,
    of_curve: Vec<(usize, usize)>,
}

impl Faces {
    fn fresh(&mut self) -> usize {
        self.next += 1;
        self.next - 1
    }

    /// The face of a segment: its own when it was drawn straight, the one its
    /// curve already holds otherwise.
    fn of(&mut self, curve: Option<usize>) -> usize {
        let Some(curve) = curve else {
            return self.fresh();
        };
        if let Some((_, face)) = self.of_curve.iter().find(|(known, _)| *known == curve) {
            return *face;
        }
        let face = self.fresh();
        self.of_curve.push((curve, face));
        face
    }

    /// Curves are numbered within one loop, so the next loop starts clean.
    fn next_loop(&mut self) {
        self.of_curve.clear();
    }
}

/// Turns a flat area into a prism: the face at the bottom, the same face moved
/// along `direction` at the top, and a wall for every edge between the two.
///
/// `outline` and `holes` are in the plane's own 2D coordinates; `to_world`
/// places them in space. A hole gets walls too — that is what makes the inside
/// of a tube a surface rather than an opening.
pub fn prism(
    outline: Loop<'_>,
    holes: &[Loop<'_>],
    triangles: &[[DVec2; 3]],
    to_world: impl Fn(DVec2) -> DVec3,
    direction: DVec3,
) -> Mesh {
    let mut polygons = Vec::new();
    let mut faces = Faces::default();
    // Both caps are one face each however many triangles the area was cut
    // into, which is what a sketch started on one of them needs.
    let (below, above) = (faces.fresh(), faces.fresh());

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
            polygons.push(outward.on_face(below));
        }
        if let Some(polygon) = Polygon::new(top) {
            let outward = if polygon.normal().dot(direction) < 0.0 {
                polygon.flipped()
            } else {
                polygon
            };
            polygons.push(outward.on_face(above));
        }
    }

    // Which way the walls face depends both on how the loop turns and on which
    // side of the plane the matter is being pushed to.
    let origin = to_world(DVec2::ZERO);
    let normal = (to_world(DVec2::X) - origin)
        .cross(to_world(DVec2::Y) - origin)
        .normalize_or(DVec3::Z);
    let along = direction.dot(normal) > 0.0;

    let mut walls = |side: Loop<'_>, inward: bool, faces: &mut Faces| {
        faces.next_loop();
        let points = side.points;
        for index in 0..points.len() {
            let a = to_world(points[index]);
            let b = to_world(points[(index + 1) % points.len()]);
            let corners = if inward {
                vec![b, a, a + direction, b + direction]
            } else {
                vec![a, b, b + direction, a + direction]
            };
            let face = faces.of(side.curve(index));
            if let Some(polygon) = Polygon::new(corners) {
                polygons.push(polygon.on_face(face));
            }
        }
    };

    walls(
        outline,
        (signed_area(outline.points) > 0.0) != along,
        &mut faces,
    );
    for hole in holes {
        // A hole's wall looks the other way: its matter is on the outside.
        walls(*hole, (signed_area(hole.points) > 0.0) == along, &mut faces);
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
    outline: Loop<'_>,
    holes: &[Loop<'_>],
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
        .points
        .iter()
        .chain(holes.iter().flat_map(|hole| hole.points))
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
    let mut faces = Faces::default();

    // Walls, as triangles rather than quads: a quad swept around an axis is
    // bent, and the boolean operations sort faces by the plane they lie on.
    //
    // Every step of one sweep is the same stretch of surface: what a straight
    // trait sweeps is a cone, and what a curve sweeps is rounder still.
    let mut wall = |side: Loop<'_>, flip: bool, faces: &mut Faces| {
        faces.next_loop();
        let points = side.points;
        for index in 0..points.len() {
            let (a, b) = (points[index], points[(index + 1) % points.len()]);
            let swept = faces.of(side.curve(index));
            for step in 0..steps {
                let (a0, b0) = (at(a, step), at(b, step));
                let (a1, b1) = (at(a, step + 1), at(b, step + 1));
                let corners = if flip {
                    [[a0, b0, b1], [a0, b1, a1]]
                } else {
                    [[a0, b1, b0], [a0, a1, b1]]
                };
                polygons.extend(
                    corners
                        .into_iter()
                        .filter_map(|piece| Polygon::new(piece.to_vec()))
                        .map(|polygon| polygon.on_face(swept)),
                );
            }
        }
    };

    // Which way the walls face depends on how the loop turns, which side of the
    // axis it sits on, and which way the sweep goes.
    let outward = (signed_area(outline.points) > 0.0) == (sign * turn > 0.0);
    wall(outline, outward, &mut faces);
    for hole in holes {
        wall(
            *hole,
            (signed_area(hole.points) > 0.0) != (sign * turn > 0.0),
            &mut faces,
        );
    }

    // A full turn closes on itself and needs no ends.
    if !full {
        let (opening, closing) = (faces.fresh(), faces.fresh());
        for triangle in triangles {
            let start: Vec<DVec3> = triangle.iter().map(|point| at(*point, 0)).collect();
            let end: Vec<DVec3> = triangle.iter().map(|point| at(*point, steps)).collect();
            for (corners, at_the_end) in [(start, false), (end, true)] {
                let Some(polygon) = Polygon::new(corners) else {
                    continue;
                };
                let outward = polygon
                    .normal()
                    .dot(axis.cross(polygon.corners[0] - origin));
                let facing = (outward > 0.0) == at_the_end;
                let end = if at_the_end { closing } else { opening };
                polygons.push(if facing { polygon } else { polygon.flipped() }.on_face(end));
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
