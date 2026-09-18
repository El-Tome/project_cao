use glam::DVec3;
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

#[cfg(test)]
pub(crate) mod tests {
    use glam::DVec2;

    use super::*;
    use crate::sweep::prism;

    /// The volume a closed surface encloses, from the signed volumes of the
    /// tetrahedra its triangles make with the origin. Negative means the
    /// surface is inside out, which is a bug worth catching.
    pub(crate) fn volume(mesh: &Mesh) -> f64 {
        mesh.triangles()
            .iter()
            .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
            .sum()
    }

    pub(crate) fn square(size: f64) -> Vec<DVec2> {
        vec![
            DVec2::ZERO,
            DVec2::new(size, 0.0),
            DVec2::new(size, size),
            DVec2::new(0.0, size),
        ]
    }

    pub(crate) fn fan(loop_points: &[DVec2]) -> Vec<[DVec2; 3]> {
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
}
