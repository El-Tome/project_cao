use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// A flat, convex-enough face of a solid, kept as its corners in order.
///
/// Polygons rather than triangles: the boolean operations cut faces against
/// planes, and a cut that lands on a triangle usually gives a quadrilateral.
/// Splitting it back into triangles at every step would multiply the count for
/// nothing — that is left to the very end, for the renderer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub corners: Vec<Vec3>,
}

impl Polygon {
    pub fn new(corners: Vec<Vec3>) -> Option<Self> {
        (corners.len() >= 3).then_some(Self { corners })
    }

    /// The outward direction of the face, from the winding of its corners.
    pub fn normal(&self) -> Vec3 {
        // Newell's formula rather than one cross product: it uses every corner,
        // so a face whose first three corners happen to be nearly in line still
        // gets a usable direction.
        let mut normal = Vec3::ZERO;
        for index in 0..self.corners.len() {
            let current = self.corners[index];
            let next = self.corners[(index + 1) % self.corners.len()];
            normal += (current - next).cross(current + next);
        }
        normal.normalize_or(Vec3::Z)
    }

    pub fn plane_offset(&self) -> f32 {
        self.normal().dot(self.corners[0])
    }

    pub fn flipped(&self) -> Self {
        let mut corners = self.corners.clone();
        corners.reverse();
        Self { corners }
    }

    /// The face cut into triangles by a fan from its first corner. Faces here
    /// are convex or nearly so, which is what makes a fan enough.
    pub fn triangles(&self) -> impl Iterator<Item = [Vec3; 3]> + '_ {
        (1..self.corners.len().saturating_sub(1))
            .map(move |index| [self.corners[0], self.corners[index], self.corners[index + 1]])
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

    pub fn triangles(&self) -> Vec<[Vec3; 3]> {
        self.polygons
            .iter()
            .flat_map(|polygon| polygon.triangles())
            .collect()
    }

    pub fn bounds(&self) -> Option<(Vec3, Vec3)> {
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

/// Turns a flat area into a prism: the face at the bottom, the same face moved
/// along `direction` at the top, and a wall for every edge between the two.
///
/// `outline` and `holes` are in the plane's own 2D coordinates; `to_world`
/// places them in space. A hole gets walls too — that is what makes the inside
/// of a tube a surface rather than an opening.
pub fn prism(
    outline: &[Vec2],
    holes: &[Vec<Vec2>],
    triangles: &[[Vec2; 3]],
    to_world: impl Fn(Vec2) -> Vec3,
    direction: Vec3,
) -> Mesh {
    let mut polygons = Vec::new();

    for triangle in triangles {
        let bottom: Vec<Vec3> = triangle.iter().map(|corner| to_world(*corner)).collect();
        let top: Vec<Vec3> = bottom.iter().map(|corner| *corner + direction).collect();

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
    let origin = to_world(Vec2::ZERO);
    let normal = (to_world(Vec2::X) - origin)
        .cross(to_world(Vec2::Y) - origin)
        .normalize_or(Vec3::Z);
    let along = direction.dot(normal) > 0.0;

    let mut walls = |loop_points: &[Vec2], inward: bool| {
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

fn signed_area(loop_points: &[Vec2]) -> f32 {
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
    pub(crate) fn volume(mesh: &Mesh) -> f32 {
        mesh.triangles()
            .iter()
            .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
            .sum()
    }

    fn square(size: f32) -> Vec<Vec2> {
        vec![
            Vec2::ZERO,
            Vec2::new(size, 0.0),
            Vec2::new(size, size),
            Vec2::new(0.0, size),
        ]
    }

    fn fan(loop_points: &[Vec2]) -> Vec<[Vec2; 3]> {
        (1..loop_points.len() - 1)
            .map(|index| [loop_points[0], loop_points[index], loop_points[index + 1]])
            .collect()
    }

    pub(crate) fn box_of(size: f32, height: f32, at: Vec3) -> Mesh {
        let outline = square(size);
        prism(
            &outline,
            &[],
            &fan(&outline),
            |point| at + Vec3::new(point.x, point.y, 0.0),
            Vec3::Z * height,
        )
    }

    #[test]
    fn a_prism_holds_the_volume_of_its_face() {
        let solid = box_of(10.0, 4.0, Vec3::ZERO);
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
            |point| Vec3::new(point.x, point.y, 0.0),
            Vec3::NEG_Z * 4.0,
        );
        assert!((volume(&solid) - 400.0).abs() < 1e-2, "{}", volume(&solid));
    }

    /// A tube: the walls of the hole belong to the surface, and the middle is
    /// not matter.
    #[test]
    fn a_hole_is_kept_hollow() {
        let outline = square(10.0);
        let hole = vec![
            Vec2::new(3.0, 3.0),
            Vec2::new(7.0, 3.0),
            Vec2::new(7.0, 7.0),
            Vec2::new(3.0, 7.0),
        ];
        // The face of the ring, cut by hand into four strips.
        let triangles = vec![
            [Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0), Vec2::new(7.0, 3.0)],
            [Vec2::new(0.0, 0.0), Vec2::new(7.0, 3.0), Vec2::new(3.0, 3.0)],
            [Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0), Vec2::new(7.0, 7.0)],
            [Vec2::new(10.0, 0.0), Vec2::new(7.0, 7.0), Vec2::new(7.0, 3.0)],
            [
                Vec2::new(10.0, 10.0),
                Vec2::new(0.0, 10.0),
                Vec2::new(3.0, 7.0),
            ],
            [
                Vec2::new(10.0, 10.0),
                Vec2::new(3.0, 7.0),
                Vec2::new(7.0, 7.0),
            ],
            [Vec2::new(0.0, 10.0), Vec2::new(0.0, 0.0), Vec2::new(3.0, 3.0)],
            [Vec2::new(0.0, 10.0), Vec2::new(3.0, 3.0), Vec2::new(3.0, 7.0)],
        ];

        let solid = prism(
            &outline,
            std::slice::from_ref(&hole),
            &triangles,
            |point| Vec3::new(point.x, point.y, 0.0),
            Vec3::Z * 2.0,
        );

        // (100 - 16) × 2
        assert!((volume(&solid) - 168.0).abs() < 1e-2, "{}", volume(&solid));
    }
}
