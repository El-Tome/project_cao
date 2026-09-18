//! What a mesh answers about its faces and what a ray meets.

use glam::DVec2;

use super::*;
use crate::sweep::{Loop, prism};

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
        Loop::straight(&outline),
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
