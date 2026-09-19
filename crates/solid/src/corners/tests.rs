//! What counts as a corner of a solid, and what names it.
//!
//! Closes #359.
//! - a corner is named by the faces meeting at it, and no two corners of a
//!   box share a name — `every_corner_of_a_box_carries_a_name_of_its_own`
//! - a flat face cut into pieces meets a wall along a seam, and a seam is no
//!   corner — `a_seam_a_cut_left_across_a_flat_face_is_no_corner`
//! - the rim where a curved wall meets a cap is a curve, not a run of corners
//!   — `the_rim_of_a_round_wall_is_no_run_of_corners`

use crate::mesh::Polygon;

use super::*;

/// A box, as the six faces it is made of.
fn a_box(width: f64) -> Mesh {
    let (x, y, z) = (width, 20.0, 4.0);
    let at = |a: f64, b: f64, c: f64| DVec3::new(a, b, c);
    let faces = [
        (
            vec![
                at(0.0, 0.0, 0.0),
                at(x, 0.0, 0.0),
                at(x, y, 0.0),
                at(0.0, y, 0.0),
            ],
            0,
        ),
        (
            vec![at(0.0, 0.0, z), at(0.0, y, z), at(x, y, z), at(x, 0.0, z)],
            1,
        ),
        (
            vec![
                at(0.0, 0.0, 0.0),
                at(0.0, 0.0, z),
                at(x, 0.0, z),
                at(x, 0.0, 0.0),
            ],
            2,
        ),
        (
            vec![at(x, 0.0, 0.0), at(x, 0.0, z), at(x, y, z), at(x, y, 0.0)],
            3,
        ),
        (
            vec![at(x, y, 0.0), at(x, y, z), at(0.0, y, z), at(0.0, y, 0.0)],
            4,
        ),
        (
            vec![
                at(0.0, y, 0.0),
                at(0.0, y, z),
                at(0.0, 0.0, z),
                at(0.0, 0.0, 0.0),
            ],
            5,
        ),
    ];
    Mesh {
        polygons: faces
            .into_iter()
            .map(|(corners, face)| Polygon::new(corners).expect("a face").on_face(face))
            .collect(),
    }
}

fn names(mesh: &Mesh) -> BTreeSet<Vec<usize>> {
    mesh.corners()
        .into_iter()
        .map(|corner| corner.faces)
        .collect()
}

#[test]
fn every_corner_of_a_box_carries_a_name_of_its_own() {
    let found = a_box(10.0).corners();

    assert_eq!(found.len(), 8);
    assert_eq!(
        names(&a_box(10.0)).len(),
        8,
        "eight corners, eight names: nothing has to be told apart by where it \
         happens to be",
    );
    for corner in &found {
        assert_eq!(corner.faces.len(), 3, "{corner:?}");
    }
}

#[test]
fn a_name_holds_when_the_part_changes_size() {
    assert_eq!(
        names(&a_box(10.0)),
        names(&a_box(30.0)),
        "the faces are the same faces however far apart they stand",
    );
}

#[test]
fn a_seam_a_cut_left_across_a_flat_face_is_no_corner() {
    // A wall, and two pieces of one flat floor meeting it — which is what a
    // boolean leaves behind when it cuts a face in two.
    let at = |a: f64, b: f64, c: f64| DVec3::new(a, b, c);
    let mesh = Mesh {
        polygons: vec![
            Polygon::new(vec![
                at(0.0, 0.0, 0.0),
                at(5.0, 0.0, 0.0),
                at(5.0, 0.0, 4.0),
                at(0.0, 0.0, 4.0),
            ])
            .expect("a wall")
            .on_face(0),
            Polygon::new(vec![
                at(0.0, 0.0, 0.0),
                at(0.0, 3.0, 0.0),
                at(2.0, 3.0, 0.0),
                at(2.0, 0.0, 0.0),
            ])
            .expect("half a floor")
            .on_face(1),
            Polygon::new(vec![
                at(2.0, 0.0, 0.0),
                at(2.0, 3.0, 0.0),
                at(5.0, 3.0, 0.0),
                at(5.0, 0.0, 0.0),
            ])
            .expect("the other half")
            .on_face(2),
        ],
    };

    let found = mesh.corners();

    assert!(
        !found.iter().any(|corner| corner.at == at(2.0, 0.0, 0.0)),
        "three faces meet there and the surface does not turn: the whole \
         seam would answer to one name — {found:?}",
    );
}

#[test]
fn the_rim_of_a_round_wall_is_no_run_of_corners() {
    // Two flats of a sampled wall and the cap above them, meeting the way
    // they do at every sample along a rim.
    let at = |a: f64, b: f64, c: f64| DVec3::new(a, b, c);
    let mesh = Mesh {
        polygons: vec![
            Polygon::new(vec![
                at(0.0, 0.0, 0.0),
                at(1.0, 0.0, 0.0),
                at(1.0, 0.0, 4.0),
                at(0.0, 0.0, 4.0),
            ])
            .expect("one flat of the wall")
            .on_face(0),
            Polygon::new(vec![
                at(1.0, 0.0, 0.0),
                at(2.0, 0.6, 0.0),
                at(2.0, 0.6, 4.0),
                at(1.0, 0.0, 4.0),
            ])
            .expect("the next flat, turned")
            .on_face(0),
            Polygon::new(vec![
                at(0.0, 0.0, 4.0),
                at(1.0, 0.0, 4.0),
                at(2.0, 0.6, 4.0),
                at(1.0, 2.0, 4.0),
            ])
            .expect("the cap")
            .on_face(1),
        ],
    };

    let found = mesh.corners();

    assert!(
        !found.iter().any(|corner| corner.at == at(1.0, 0.0, 4.0)),
        "the wall is one face however many flats it was sampled into, so two \
         faces meet on that rim and not three — {found:?}",
    );
}
