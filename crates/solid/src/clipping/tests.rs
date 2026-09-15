use glam::DVec3;

use super::*;

const TOLERANCE: f64 = 1e-9;

/// A box with its corners at the two places given, as six faces.
fn box_between(low: DVec3, high: DVec3) -> Mesh {
    let corner = |x: f64, y: f64, z: f64| DVec3::new(x, y, z);
    let (l, h) = (low, high);
    let faces = [
        vec![
            corner(l.x, l.y, l.z),
            corner(h.x, l.y, l.z),
            corner(h.x, h.y, l.z),
            corner(l.x, h.y, l.z),
        ],
        vec![
            corner(l.x, l.y, h.z),
            corner(l.x, h.y, h.z),
            corner(h.x, h.y, h.z),
            corner(h.x, l.y, h.z),
        ],
        vec![
            corner(l.x, l.y, l.z),
            corner(l.x, l.y, h.z),
            corner(h.x, l.y, h.z),
            corner(h.x, l.y, l.z),
        ],
        vec![
            corner(l.x, h.y, l.z),
            corner(h.x, h.y, l.z),
            corner(h.x, h.y, h.z),
            corner(l.x, h.y, h.z),
        ],
        vec![
            corner(l.x, l.y, l.z),
            corner(l.x, h.y, l.z),
            corner(l.x, h.y, h.z),
            corner(l.x, l.y, h.z),
        ],
        vec![
            corner(h.x, l.y, l.z),
            corner(h.x, l.y, h.z),
            corner(h.x, h.y, h.z),
            corner(h.x, h.y, l.z),
        ],
    ];
    Mesh {
        polygons: faces.into_iter().filter_map(Polygon::new).collect(),
    }
}

fn corners(mesh: &Mesh) -> Vec<DVec3> {
    mesh.polygons
        .iter()
        .flat_map(|face| face.corners.iter().copied())
        .collect()
}

#[test]
fn a_body_straddling_the_plane_keeps_nothing_of_the_side_in_front() {
    let body = box_between(DVec3::new(-1.0, -1.0, -1.0), DVec3::new(1.0, 1.0, 1.0));

    let left = body.behind(DVec3::Z, 0.0);

    assert!(
        corners(&left).iter().all(|at| at.z <= TOLERANCE),
        "nothing of the near side is handed on: {:?}",
        corners(&left)
            .iter()
            .filter(|at| at.z > TOLERANCE)
            .collect::<Vec<_>>()
    );
    assert!(
        corners(&left)
            .iter()
            .any(|at| (at.z + 1.0).abs() <= TOLERANCE),
        "and the far side is still there"
    );
}

#[test]
fn a_body_entirely_behind_the_plane_is_handed_on_untouched() {
    let body = box_between(DVec3::new(-1.0, -1.0, -3.0), DVec3::new(1.0, 1.0, -2.0));

    let left = body.behind(DVec3::Z, 0.0);

    assert_eq!(left.polygons, body.polygons, "nothing stood in the way");
}

#[test]
fn a_body_entirely_in_front_of_the_plane_leaves_nothing_at_all() {
    let body = box_between(DVec3::new(-1.0, -1.0, 2.0), DVec3::new(1.0, 1.0, 3.0));

    assert!(body.behind(DVec3::Z, 0.0).polygons.is_empty());
}

#[test]
fn the_face_a_sketch_is_started_on_stays_when_the_matter_above_it_goes() {
    let body = box_between(DVec3::new(-1.0, -1.0, -1.0), DVec3::new(1.0, 1.0, 0.0));

    let left = body.behind(DVec3::Z, 0.0);

    let on_the_plane = left
        .polygons
        .iter()
        .filter(|face| face.corners.iter().all(|at| at.z.abs() <= TOLERANCE))
        .count();
    assert_eq!(
        on_the_plane, 1,
        "the face the drawing sits on is not matter standing in front of it"
    );
}

#[test]
fn the_plane_can_stand_anywhere_and_face_any_way() {
    let body = box_between(DVec3::new(-2.0, -2.0, -2.0), DVec3::new(2.0, 2.0, 2.0));

    let left = body.behind(DVec3::NEG_X, 1.0);

    assert!(
        corners(&left).iter().all(|at| at.x >= -1.0 - TOLERANCE),
        "kept is the side the normal points away from, wherever the plane sits"
    );
    assert!(
        corners(&left)
            .iter()
            .any(|at| (at.x - 2.0).abs() <= TOLERANCE)
    );
}
