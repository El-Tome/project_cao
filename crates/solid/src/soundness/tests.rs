//! The pieces every rule is tried on: solids built by hand, sound or broken on
//! purpose, as triangles.
//!
//! Closes #448.
//! - replaying twice gives the same result, bit for bit, caught out by an
//!   answer broken on purpose — `a_corner_one_bit_away_is_not_the_same_answer`,
//!   `an_answer_with_a_triangle_more_is_not_the_same_answer`

use glam::DVec3;

use super::*;

/// The six faces of a box between two corners, as twelve triangles facing
/// out.
pub(crate) fn cube(low: DVec3, high: DVec3) -> Vec<Triangle> {
    let corner = |x: bool, y: bool, z: bool| {
        DVec3::new(
            if x { high.x } else { low.x },
            if y { high.y } else { low.y },
            if z { high.z } else { low.z },
        )
    };
    let faces = [
        [(0, 0, 0), (0, 1, 0), (1, 1, 0), (1, 0, 0)],
        [(0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)],
        [(0, 0, 0), (1, 0, 0), (1, 0, 1), (0, 0, 1)],
        [(0, 1, 0), (0, 1, 1), (1, 1, 1), (1, 1, 0)],
        [(0, 0, 0), (0, 0, 1), (0, 1, 1), (0, 1, 0)],
        [(1, 0, 0), (1, 1, 0), (1, 1, 1), (1, 0, 1)],
    ];
    faces
        .iter()
        .flat_map(|face| {
            let [a, b, c, d] = face.map(|(x, y, z)| corner(x == 1, y == 1, z == 1));
            [[a, b, c], [a, c, d]]
        })
        .collect()
}

pub(crate) fn unit_cube() -> Vec<Triangle> {
    cube(DVec3::ZERO, DVec3::ONE)
}

#[test]
fn the_box_every_rule_is_tried_on_faces_out() {
    let volume: f64 = unit_cube()
        .iter()
        .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
        .sum();
    assert!((volume - 1.0).abs() < 1e-12, "{volume}");
}

#[test]
fn the_same_triangles_twice_are_repeatable() {
    assert_eq!(repeatable(&unit_cube(), &unit_cube()), Ok(()));
}

#[test]
fn a_corner_one_bit_away_is_not_the_same_answer() {
    let mut nudged = unit_cube();
    nudged[5][1].x = f64::from_bits(nudged[5][1].x.to_bits() + 1);
    assert_eq!(
        repeatable(&unit_cube(), &nudged),
        Err(Flaw::Unrepeatable { at: 5 })
    );
}

#[test]
fn an_answer_with_a_triangle_more_is_not_the_same_answer() {
    let mut longer = unit_cube();
    longer.push(longer[0]);
    assert_eq!(
        repeatable(&unit_cube(), &longer),
        Err(Flaw::Unrepeatable { at: 12 })
    );
}

#[test]
fn a_solid_reaches_as_far_as_its_furthest_corner_and_never_under_one() {
    assert_eq!(reach(&cube(DVec3::splat(-30.0), DVec3::splat(2.0))), 30.0);
    assert_eq!(reach(&cube(DVec3::ZERO, DVec3::splat(0.25))), 1.0);
}
