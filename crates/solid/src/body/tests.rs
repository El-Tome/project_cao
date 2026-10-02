//! What a body answers about the matter it holds, without showing what it is
//! made of.

use glam::{DVec2, DVec3};

use super::*;

/// A block raised from the origin, its top one flat face stored as two
/// triangles. The prism numbers its faces bottom, top, then the walls.
fn block(width: f64, depth: f64, height: f64) -> Body {
    let corners = [
        DVec2::ZERO,
        DVec2::new(width, 0.0),
        DVec2::new(width, depth),
        DVec2::new(0.0, depth),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    Body::prism(
        Loop::straight(&corners),
        &[],
        &triangles,
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec3::Z * height,
    )
}

const TOP: usize = 1;

#[test]
fn a_block_encloses_its_width_times_its_depth_times_its_height() {
    let volume = block(3.0, 4.0, 5.0).volume();

    assert!((volume - 60.0).abs() < 1e-9, "{volume}");
}

#[test]
fn a_ray_names_the_face_it_meets_and_which_way_it_faces() {
    let hit = block(10.0, 10.0, 4.0)
        .ray_hit(DVec3::new(5.0, 2.0, 20.0), DVec3::NEG_Z)
        .expect("the top of the block");

    assert_eq!(hit.face, TOP);
    assert!((hit.distance - 16.0).abs() < 1e-9, "{}", hit.distance);
    assert!(hit.normal.dot(DVec3::Z) > 0.999, "{}", hit.normal);
}

#[test]
fn a_face_is_drawn_with_the_triangles_it_was_raised_from() {
    let body = block(10.0, 6.0, 4.0);

    let pieces: Vec<[DVec3; 3]> = body.pieces_of(TOP).collect();

    assert_eq!(pieces.len(), 2, "{pieces:?}");
    assert!(
        pieces.iter().flatten().all(|corner| corner.z == 4.0),
        "every piece of the top lies on it: {pieces:?}",
    );
    assert_eq!(body.pieces_of(body.faces_end()).count(), 0);
}

#[test]
fn a_flat_face_offers_its_plane_with_every_corner_of_every_piece() {
    let body = block(10.0, 6.0, 4.0);

    let plane = body.plane_of(TOP).expect("the block has a top");

    assert!(body.is_flat(TOP));
    assert!(plane.normal.dot(DVec3::Z) > 0.999, "{}", plane.normal);
    assert_eq!(plane.corners.len(), 6, "two triangles: {:?}", plane.corners);
    assert!(
        plane.corners.contains(&DVec3::new(0.0, 0.0, 4.0))
            && plane.corners.contains(&DVec3::new(10.0, 6.0, 4.0)),
        "{:?}",
        plane.corners,
    );
    assert_eq!(body.plane_of(body.faces_end()), None);
}
