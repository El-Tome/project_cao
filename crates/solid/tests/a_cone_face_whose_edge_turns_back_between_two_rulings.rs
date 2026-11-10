//! A cone face whose edge turns back in θ between two grid rulings is cut with
//! no triangle upside down in space (#820, part of #755).
//!
//! The cut of such a face (`Outline::columned`) fans from a corner of a grid
//! column on a rim to two samples of the meet near its turn — the turn
//! itself, and the next grid angle a hair away. The pair runs almost along a
//! ruling, so the fan's triangle is a few thousandths of a degree wide in the
//! chart, and lies folded over in space: its normal opposite to every
//! triangle around it. `folds::crossed` reads only other faces, so a face
//! folded onto itself passes the checks; #811's guard removes the spurious
//! sample that kept the fold beside a neighbour sharing its edge, and the six
//! seeds below then fail `Uncrossed` through the body.
//!
//! "Upside down" is read here without the surface: two triangles of one face
//! sharing a corner never face more than a right angle apart. At the body's
//! own tolerance the faces of these cases hold every such pair within 59°;
//! the fold stands at 180°.
//!
//! Closes #820. #811's guard on `Ruled::at_angle` is not on this branch — it
//! is set aside on `wip/811-guard-set-aside` until this lands — so what holds
//! with it is measured with the guard applied, not asserted.
//! - 562's shrunk case is pinned: no triangle of its cone face stands upside
//!   down against its neighbours, on `main`'s sampling and with #811's guard —
//!   reworded: pinned on `main`'s sampling, which this branch carries; with the
//!   guard, measured and recorded on the pull request —
//!   `seed_352000000000562_a_cone_joined_to_a_cylinder_of_radius_1_leaning_80_degrees_folds_no_triangle_of_a_face_onto_its_neighbours`
//! - The six seeds above hold through the body with #811's guard applied — no
//!   test: the guard is not on this branch; measured with it applied in a
//!   scratch worktree and recorded on the pull request, and their pins
//!   through the body land with #811. On `main`'s sampling each of the five
//!   others carries the same fold on its cone face, held here:
//!   `seed_352000000005478_a_cone_joined_to_an_elliptic_wall_folds_no_triangle_of_a_face_onto_its_neighbours`,
//!   `seed_352000000006255_a_cone_joined_to_a_cylinder_of_radius_30_leaning_84_degrees_folds_no_triangle_of_a_face_onto_its_neighbours`,
//!   `seed_352000000007343_a_cone_joined_to_a_cylinder_then_cut_by_a_turned_disc_folds_no_triangle_of_a_face_onto_its_neighbours`,
//!   `seed_354000000010173_a_cone_joined_to_an_elliptic_wall_folds_no_triangle_of_a_face_onto_its_neighbours`,
//!   `seed_354000000013382_a_cone_joined_to_an_elliptic_wall_leaning_90_degrees_folds_no_triangle_of_a_face_onto_its_neighbours`
//! - No draw regresses against `main`, seed by seed, and the measurement is
//!   recorded in `docs/exact-kernel-failures.md` — no test: a measurement and
//!   a document; the walls sets on both kernels and the turned set are run on
//!   `main` and on the branch and compared seed by seed

#[allow(dead_code, unused_imports)]
mod random_solids;

use std::collections::HashMap;

use glam::DVec3;
use random_solids::{Application, Axis, Case, Kernel, Leaf, Outline, Plane, Section, Step};

/// Builds a case through the application's body and holds every face of it,
/// as the body draws it, unfolded: no two triangles of one face sharing a
/// corner face more than a right angle apart.
fn no_face_folds_onto_itself(case: &Case) {
    let leaves: Vec<&Leaf> = case.leaves().collect();
    let mut body = Application
        .raised(leaves[0])
        .expect("the first leaf is raised");
    for (step, leaf) in case.steps.iter().zip(&leaves[1..]) {
        let tool = Application.raised(leaf).expect("the tool is raised");
        body = Application
            .combined(&body, &tool, step.mode)
            .expect("the step is built");
    }
    let faces: Vec<usize> = (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&face| body.has_face(face))
        .collect();
    assert!(!faces.is_empty(), "the body holds no face\n{case}");
    let mut folds = Vec::new();
    for face in faces {
        let triangles: Vec<[DVec3; 3]> = body.triangles_of(face).collect();
        assert!(
            !triangles.is_empty(),
            "face {face} is drawn with no triangle\n{case}"
        );
        folds.extend(folds_of(face, &triangles));
    }
    assert!(
        folds.is_empty(),
        "triangles of one face facing more than a right angle apart across a shared corner:\n{}\n{case}",
        folds.join("\n")
    );
}

fn folds_of(face: usize, triangles: &[[DVec3; 3]]) -> Vec<String> {
    let normals: Vec<DVec3> = triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).cross(*c - *a).normalize_or_zero())
        .collect();
    let mut around: HashMap<[u64; 3], Vec<usize>> = HashMap::new();
    for (index, triangle) in triangles.iter().enumerate() {
        for corner in triangle {
            around
                .entry(corner.to_array().map(f64::to_bits))
                .or_default()
                .push(index);
        }
    }
    let mut folds = Vec::new();
    for sharing in around.values() {
        for (place, &first) in sharing.iter().enumerate() {
            for &second in &sharing[place + 1..] {
                let cosine = normals[first].dot(normals[second]);
                if first != second && cosine < 0.0 {
                    folds.push(format!(
                        "face {face}: {:?} and {:?} at cos {cosine:.6}",
                        triangles[first], triangles[second]
                    ));
                }
            }
        }
    }
    folds.sort();
    folds.dedup();
    folds
}

#[test]
fn seed_352000000000562_a_cone_joined_to_a_cylinder_of_radius_1_leaning_80_degrees_folds_no_triangle_of_a_face_onto_its_neighbours()
 {
    no_face_folds_onto_itself(&Case::new(
        Leaf::turned(
            Plane::xz(-2.0),
            Axis::second(0.0),
            Section::bands(-1.0, &[[8.0, 0.0, 0.5]]).sloping_to(&[[0.0, 3.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::tilted([12.0, -2.0, 1.0], [0.0, -80.0, 0.0]),
            Outline::circle([0.0, 0.8826347875402311], 1.0),
            29.0,
        ))],
    ));
}

#[test]
fn seed_352000000005478_a_cone_joined_to_an_elliptic_wall_folds_no_triangle_of_a_face_onto_its_neighbours()
 {
    no_face_folds_onto_itself(&Case::new(
        Leaf::turned(
            Plane::yz(0.5),
            Axis::second(0.0),
            Section::bands(-1.0, &[[4.5, 0.0, 2.0]]).sloping_to(&[[0.0, 5.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::tilted(
                [0.5, -21.340822079655823, 1.2499998987316945],
                [-90.0, 0.0, 0.0],
            ),
            Outline::ellipse(
                [-2.880343162536262, -0.0],
                [0.38822856765378044, -1.4488887394336025],
                0.5,
            ),
            39.0,
        ))],
    ));
}

#[test]
fn seed_352000000006255_a_cone_joined_to_a_cylinder_of_radius_30_leaning_84_degrees_folds_no_triangle_of_a_face_onto_its_neighbours()
 {
    no_face_folds_onto_itself(&Case::new(
        Leaf::turned(
            Plane::yz(30.0),
            Axis::first(0.0),
            Section::bands(-30.0, &[[240.0, 0.0, 150.0]]).sloping_to(&[[0.0, 15.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::tilted([627.0, 152.0, -0.0], [90.0, -84.02558168442748, 0.0]),
            Outline::circle([0.0, 2.0], 30.0),
            1200.0,
        ))],
    ));
}

#[test]
fn seed_352000000007343_a_cone_joined_to_a_cylinder_then_cut_by_a_turned_disc_folds_no_triangle_of_a_face_onto_its_neighbours()
 {
    no_face_folds_onto_itself(&Case::new(
        Leaf::turned(
            Plane::yz(7.5),
            Axis::second(0.0),
            Section::bands(0.0, &[[35.0, 0.0, 5.0]]).sloping_to(&[[0.0, 20.0]]),
            360.0,
        ),
        vec![
            Step::add(Leaf::prism(
                Plane::tilted(
                    [7.5, 79.02067347825036, 17.499999970671244],
                    [90.0, 0.0, 0.0],
                ),
                Outline::circle([-6.0, -0.0], 10.0),
                168.0,
            )),
            Step::cut(Leaf::turned(
                Plane::xy(17.0),
                Axis::second(0.0),
                Section::bands(-66.0, &[[168.0, 0.0, 8.0]]),
                360.0,
            )),
        ],
    ));
}

#[test]
fn seed_354000000010173_a_cone_joined_to_an_elliptic_wall_folds_no_triangle_of_a_face_onto_its_neighbours()
 {
    no_face_folds_onto_itself(&Case::new(
        Leaf::turned(
            Plane::xy(2.0),
            Axis::second(-1.0),
            Section::bands(-2.0, &[[6.0, 0.0, 2.0]]).sloping_to(&[[0.0, 5.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::tilted([-1.0, -19.0, 2.0], [-90.0, 0.0, 0.0]),
            Outline::ellipse([-3.999999999999999, -0.0], [0.0, -2.0], 0.5),
            40.0,
        ))],
    ));
}

#[test]
fn seed_354000000013382_a_cone_joined_to_an_elliptic_wall_leaning_90_degrees_folds_no_triangle_of_a_face_onto_its_neighbours()
 {
    no_face_folds_onto_itself(&Case::new(
        Leaf::turned(
            Plane::xz(2.0),
            Axis::first(1.0),
            Section::bands(-4.0, &[[4.5, 0.0, 0.5]]).sloping_to(&[[0.0, 5.0]]),
            360.0,
        ),
        vec![Step::add(Leaf::prism(
            Plane::tilted([18.0, 2.0, 1.0], [0.0, -90.0, 0.0]),
            Outline::ellipse(
                [-0.0, -2.864932891026669],
                [0.12940952255126015, -0.4829629131445342],
                2.0,
            ),
            39.0,
        ))],
    ));
}
