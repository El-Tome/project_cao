use glam::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

/// A flat, convex-enough face of a solid, kept as its corners in order.
///
/// Polygons rather than triangles: the boolean operations cut faces against
/// planes, and a cut that lands on a triangle usually gives a quadrilateral.
/// Splitting it back into triangles at every step would multiply the count for
/// nothing — that is left to the very end, for the renderer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub corners: Vec<DVec3>,
}

impl Polygon {
    /// A face, or `None` when the corners given do not describe one.
    ///
    /// A sliver with no area is refused rather than kept: it has no direction
    /// to face, and the boolean operations sort faces by the plane they lie on.
    /// One face without a plane and the sorting never finishes — which ends the
    /// program on a blown stack rather than with an error.
    pub fn new(corners: Vec<DVec3>) -> Option<Self> {
        if corners.len() < 3 {
            return None;
        }
        let candidate = Self { corners };
        // Judged against the face's own size rather than in absolute units: a
        // thin wall is a real face at any scale, while a splinter left by a cut
        // has almost no area for the room it takes up. Splinters are what make
        // the partition below grow without end.
        let reach = candidate.perimeter();
        (reach > 1e-9 && candidate.area_vector().length() / (reach * reach) > 1e-7)
            .then_some(candidate)
    }

    fn perimeter(&self) -> f64 {
        (0..self.corners.len())
            .map(|index| {
                self.corners[index].distance(self.corners[(index + 1) % self.corners.len()])
            })
            .sum()
    }

    fn area_vector(&self) -> DVec3 {
        let mut doubled = DVec3::ZERO;
        for index in 0..self.corners.len() {
            let current = self.corners[index];
            let next = self.corners[(index + 1) % self.corners.len()];
            doubled += current.cross(next);
        }
        doubled * 0.5
    }

    /// The outward direction of the face, from the winding of its corners.
    pub fn normal(&self) -> DVec3 {
        // Newell's formula rather than one cross product: it uses every corner,
        // so a face whose first three corners happen to be nearly in line still
        // gets a usable direction.
        let mut normal = DVec3::ZERO;
        for index in 0..self.corners.len() {
            let current = self.corners[index];
            let next = self.corners[(index + 1) % self.corners.len()];
            normal += (current - next).cross(current + next);
        }
        normal.normalize_or(DVec3::Z)
    }

    pub fn plane_offset(&self) -> f64 {
        self.normal().dot(self.corners[0])
    }

    pub fn flipped(&self) -> Self {
        let mut corners = self.corners.clone();
        corners.reverse();
        Self { corners }
    }

    /// The face cut into triangles by a fan from its first corner. Faces here
    /// are convex or nearly so, which is what makes a fan enough.
    pub fn triangles(&self) -> impl Iterator<Item = [DVec3; 3]> + '_ {
        (1..self.corners.len().saturating_sub(1)).map(move |index| {
            [
                self.corners[0],
                self.corners[index],
                self.corners[index + 1],
            ]
        })
    }
}

/// A closed volume, as the faces of its surface.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Mesh {
    pub polygons: Vec<Polygon>,
}

impl Mesh {
    pub fn is_empty(&self) -> bool {
        self.polygons.is_empty()
    }

    pub fn triangles(&self) -> Vec<[DVec3; 3]> {
        self.polygons
            .iter()
            .flat_map(|polygon| polygon.triangles())
            .collect()
    }

    /// The face a ray meets first, if any.
    ///
    /// This is what lets a sketch be started on the part itself rather than
    /// only on the three planes of the origin: the face under the cursor is
    /// found the same way the cursor finds anything else in the view.
    pub fn ray_hit(&self, origin: DVec3, direction: DVec3) -> Option<FaceHit> {
        let mut nearest: Option<FaceHit> = None;
        for polygon in &self.polygons {
            for triangle in polygon.triangles() {
                let Some(distance) = ray_triangle(origin, direction, triangle) else {
                    continue;
                };
                if nearest.as_ref().is_none_or(|best| distance < best.distance) {
                    nearest = Some(FaceHit {
                        distance,
                        polygon: polygon.clone(),
                    });
                }
            }
        }
        nearest
    }

    pub fn bounds(&self) -> Option<(DVec3, DVec3)> {
        let first = *self.polygons.first()?.corners.first()?;
        Some(
            self.polygons
                .iter()
                .flat_map(|polygon| &polygon.corners)
                .fold((first, first), |(min, max), corner| {
                    (min.min(*corner), max.max(*corner))
                }),
        )
    }
}

/// A face of the part, and how far along the ray it was met.
#[derive(Clone, Debug)]
pub struct FaceHit {
    pub distance: f64,
    pub polygon: Polygon,
}

/// Where a ray crosses a triangle, as a distance along the ray.
///
/// Möller–Trumbore: it solves for the barycentric coordinates directly, so the
/// test that the crossing lies inside the triangle falls out of the same
/// arithmetic instead of needing a second step.
fn ray_triangle(origin: DVec3, direction: DVec3, [a, b, c]: [DVec3; 3]) -> Option<f64> {
    let (edge_1, edge_2) = (b - a, c - a);
    let across = direction.cross(edge_2);
    let determinant = edge_1.dot(across);
    if determinant.abs() < 1e-9 {
        return None;
    }

    let inverse = 1.0 / determinant;
    let to_corner = origin - a;
    let u = to_corner.dot(across) * inverse;
    if !(-1e-5..=1.0 + 1e-5).contains(&u) {
        return None;
    }

    let along = to_corner.cross(edge_1);
    let v = direction.dot(along) * inverse;
    if v < -1e-5 || u + v > 1.0 + 1e-5 {
        return None;
    }

    let distance = edge_2.dot(along) * inverse;
    (distance > 1e-5).then_some(distance)
}

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
pub(crate) mod tests {
    use super::*;

    /// The volume a closed surface encloses, from the signed volumes of the
    /// tetrahedra its triangles make with the origin. Negative means the
    /// surface is inside out, which is a bug worth catching.
    pub(crate) fn volume(mesh: &Mesh) -> f64 {
        mesh.triangles()
            .iter()
            .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
            .sum()
    }

    fn square(size: f64) -> Vec<DVec2> {
        vec![
            DVec2::ZERO,
            DVec2::new(size, 0.0),
            DVec2::new(size, size),
            DVec2::new(0.0, size),
        ]
    }

    fn fan(loop_points: &[DVec2]) -> Vec<[DVec2; 3]> {
        (1..loop_points.len() - 1)
            .map(|index| [loop_points[0], loop_points[index], loop_points[index + 1]])
            .collect()
    }

    pub(crate) fn box_of(size: f64, height: f64, at: DVec3) -> Mesh {
        let outline = square(size);
        prism(
            &outline,
            &[],
            &fan(&outline),
            |point| at + DVec3::new(point.x, point.y, 0.0),
            DVec3::Z * height,
        )
    }

    /// A sliver has no direction to face, and must not become a polygon: the
    /// boolean operations sort faces by their plane and would never finish.
    #[test]
    fn a_face_with_no_area_is_refused() {
        let flat = vec![DVec3::ZERO, DVec3::X, DVec3::X * 2.0];
        assert!(Polygon::new(flat).is_none(), "three points in a line");
        assert!(
            Polygon::new(vec![DVec3::ZERO, DVec3::X]).is_none(),
            "two points"
        );
        assert!(
            Polygon::new(vec![DVec3::ZERO; 4]).is_none(),
            "the same point four times"
        );
    }

    fn profile(min: DVec2, max: DVec2) -> Vec<DVec2> {
        vec![min, DVec2::new(max.x, min.y), max, DVec2::new(min.x, max.y)]
    }

    /// Pappus: sweeping an area right round gives its area times the distance
    /// travelled by its centre.
    #[test]
    fn a_full_turn_gives_pappus_volume() {
        let outline = profile(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
        let solid = revolution(
            &outline,
            &[],
            &fan(&outline),
            |point| DVec3::new(point.x, point.y, 0.0),
            DVec2::ZERO,
            DVec2::Y,
            std::f64::consts::TAU,
        )
        .expect("a profile on one side of the axis");

        let expected = std::f64::consts::TAU * 4.0 * 4.0;
        let made = volume(&solid);
        assert!(
            (made - expected).abs() / expected < 0.01,
            "{made} / {expected}"
        );
    }

    /// A part turn is capped at both ends, and holds the matching share.
    #[test]
    fn a_quarter_turn_holds_a_quarter_of_the_volume() {
        let outline = profile(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
        let solid = revolution(
            &outline,
            &[],
            &fan(&outline),
            |point| DVec3::new(point.x, point.y, 0.0),
            DVec2::ZERO,
            DVec2::Y,
            std::f64::consts::FRAC_PI_2,
        )
        .expect("a quarter turn");

        let expected = std::f64::consts::FRAC_PI_2 * 4.0 * 4.0;
        let made = volume(&solid);
        assert!(
            (made - expected).abs() / expected < 0.02,
            "{made} / {expected}"
        );
    }

    /// Turning the other way must not turn the solid inside out.
    #[test]
    fn turning_backwards_still_faces_outwards() {
        let outline = profile(DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
        let solid = revolution(
            &outline,
            &[],
            &fan(&outline),
            |point| DVec3::new(point.x, point.y, 0.0),
            DVec2::ZERO,
            DVec2::Y,
            -std::f64::consts::FRAC_PI_2,
        )
        .expect("a quarter turn the other way");
        assert!(volume(&solid) > 0.0, "{}", volume(&solid));
    }

    /// A profile lying across the axis would sweep through itself.
    #[test]
    fn a_profile_across_the_axis_is_refused() {
        let outline = profile(DVec2::new(-2.0, 0.0), DVec2::new(5.0, 2.0));
        assert!(
            revolution(
                &outline,
                &[],
                &fan(&outline),
                |point| DVec3::new(point.x, point.y, 0.0),
                DVec2::ZERO,
                DVec2::Y,
                std::f64::consts::TAU,
            )
            .is_none()
        );
    }

    #[test]
    fn a_ray_finds_the_face_it_meets_first() {
        let solid = box_of(10.0, 4.0, DVec3::ZERO);

        // Straight down onto the top of the box, from well above it.
        let hit = solid
            .ray_hit(DVec3::new(5.0, 5.0, 20.0), DVec3::NEG_Z)
            .expect("the top face");
        assert!((hit.distance - 16.0).abs() < 1e-3, "{}", hit.distance);
        assert!(hit.polygon.normal().dot(DVec3::Z) > 0.99, "it faces up");

        assert!(
            solid
                .ray_hit(DVec3::new(50.0, 50.0, 20.0), DVec3::NEG_Z)
                .is_none(),
            "beside the part"
        );
    }

    #[test]
    fn a_prism_holds_the_volume_of_its_face() {
        let solid = box_of(10.0, 4.0, DVec3::ZERO);
        assert!((volume(&solid) - 400.0).abs() < 1e-2, "{}", volume(&solid));
    }

    /// Extruding the other way must not turn the solid inside out.
    #[test]
    fn extruding_backwards_still_faces_outwards() {
        let outline = square(10.0);
        let solid = prism(
            &outline,
            &[],
            &fan(&outline),
            |point| DVec3::new(point.x, point.y, 0.0),
            DVec3::NEG_Z * 4.0,
        );
        assert!((volume(&solid) - 400.0).abs() < 1e-2, "{}", volume(&solid));
    }

    /// A tube: the walls of the hole belong to the surface, and the middle is
    /// not matter.
    #[test]
    fn a_hole_is_kept_hollow() {
        let outline = square(10.0);
        let hole = vec![
            DVec2::new(3.0, 3.0),
            DVec2::new(7.0, 3.0),
            DVec2::new(7.0, 7.0),
            DVec2::new(3.0, 7.0),
        ];
        // The face of the ring, cut by hand into four strips.
        let triangles = vec![
            [
                DVec2::new(0.0, 0.0),
                DVec2::new(10.0, 0.0),
                DVec2::new(7.0, 3.0),
            ],
            [
                DVec2::new(0.0, 0.0),
                DVec2::new(7.0, 3.0),
                DVec2::new(3.0, 3.0),
            ],
            [
                DVec2::new(10.0, 0.0),
                DVec2::new(10.0, 10.0),
                DVec2::new(7.0, 7.0),
            ],
            [
                DVec2::new(10.0, 0.0),
                DVec2::new(7.0, 7.0),
                DVec2::new(7.0, 3.0),
            ],
            [
                DVec2::new(10.0, 10.0),
                DVec2::new(0.0, 10.0),
                DVec2::new(3.0, 7.0),
            ],
            [
                DVec2::new(10.0, 10.0),
                DVec2::new(3.0, 7.0),
                DVec2::new(7.0, 7.0),
            ],
            [
                DVec2::new(0.0, 10.0),
                DVec2::new(0.0, 0.0),
                DVec2::new(3.0, 3.0),
            ],
            [
                DVec2::new(0.0, 10.0),
                DVec2::new(3.0, 3.0),
                DVec2::new(3.0, 7.0),
            ],
        ];

        let solid = prism(
            &outline,
            std::slice::from_ref(&hole),
            &triangles,
            |point| DVec3::new(point.x, point.y, 0.0),
            DVec3::Z * 2.0,
        );

        // (100 - 16) × 2
        assert!((volume(&solid) - 168.0).abs() < 1e-2, "{}", volume(&solid));
    }
}
