use glam::{DVec2, DVec3};

use super::{DEPTH, Drawn, Folds, opens, through};
use crate::brep::tessellation::outline::Outline;
use crate::brep::tessellation::sampling::Samples;
use crate::brep::tessellation::tests::fixtures;

const CLEAR: f64 = 1e-10;

fn at(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(x, y, z)
}

#[test]
fn two_triangles_threaded_through_each_other_pass_through() {
    let flat = [at(0.0, 0.0, 0.0), at(2.0, 0.0, 0.0), at(0.0, 2.0, 0.0)];
    let standing = [at(0.5, 0.5, -1.0), at(0.5, 0.5, 1.0), at(-1.0, 0.5, 0.0)];
    assert!(through(flat, standing, CLEAR));
    assert!(through(standing, flat, CLEAR));
}

#[test]
fn two_triangles_leaving_a_shared_corner_either_side_of_their_planes_line_do_not() {
    let one = [at(0.0, 0.0, 0.0), at(1.0, 0.0, -1.0), at(1.0, 0.0, 1.0)];
    let other = [at(0.0, 0.0, 0.0), at(-1.0, -1.0, 0.0), at(-1.0, 1.0, 0.0)];
    assert!(!through(one, other, CLEAR));
}

#[test]
fn a_triangle_resting_a_corner_on_another_within_the_clearance_does_not_pass_through_it() {
    let flat = [at(0.0, 0.0, 0.0), at(2.0, 0.0, 0.0), at(0.0, 2.0, 0.0)];
    let resting = [at(0.5, 0.5, -1e-12), at(0.5, 0.5, 1.0), at(1.0, 0.2, 1.0)];
    assert!(!through(flat, resting, CLEAR));
}

#[test]
fn a_chord_sinking_two_billionths_under_a_corner_beside_it_passes_through_the_face_there() {
    let ceiling = [
        at(1.75000001, 3.5, 9.50000001),
        at(1.9413417161825448, 4.461939766255643, 9.50000001),
        at(1.941341716182545, 3.5380602337443565, 9.50000001),
    ];
    let chord = [
        at(2.0, 3.5380602337443565, 9.691341716182546),
        at(1.75, 3.5, 9.5),
        at(2.0, 3.5, 9.5),
    ];
    assert!(through(chord, ceiling, CLEAR));
}

#[test]
fn three_samples_of_a_rim_collinear_in_a_cone_s_parameters_but_for_rounding_make_no_triangle() {
    let rim = [
        DVec2::new(3.4781918664744134, 42.42640687119284),
        DVec2::new(3.5903916041026207, 42.426406871192825),
        DVec2::new(3.702591341730827, 42.42640687119283),
    ];
    assert!(!opens(rim[0], rim[1], rim[2]));
}

#[test]
fn a_sliver_a_few_billionths_of_a_turn_wide_still_opens() {
    let sliver = [
        DVec2::new(2.4980915514631756, -2.0),
        DVec2::new(2.4980915471965086, 0.5),
        DVec2::new(2.498091544796509, 0.5),
    ];
    assert!(opens(sliver[0], sliver[1], sliver[2]));
}

#[test]
fn a_triangle_a_hair_high_and_a_wide_one_lying_on_it_across_their_edge_are_folded_from_either() {
    let reach = 12.0;
    let tilt = 2e-9;
    let mut build = fixtures::Build::new();
    let [a, b, hair, far] = [
        at(0.0, 0.0, 0.0),
        at(reach, 0.0, 0.0),
        at(reach / 2.0, 3e-5, tilt),
        at(reach / 2.0, reach, 0.0),
    ];
    let [a, b, hair_corner, far_corner] = [a, b, hair, far].map(|point| build.vertex(point));
    let outward = (at(reach, 0.0, 0.0))
        .cross(at(reach / 2.0, 3e-5, tilt))
        .normalize();
    let faces = [
        build.polygon(&[a, b, hair_corner], outward),
        build.polygon(&[a, b, far_corner], DVec3::Z),
    ];
    let body = build.finish(reach);
    let eps = body.scale().eps();
    let tolerance = 1e-3 * reach;
    let mut samples = Samples::of(&body, tolerance);
    let mut drawn: Vec<Drawn> = faces
        .iter()
        .map(|face| {
            let outline = Outline::of(&body, &mut samples, *face, tolerance)
                .expect("a flat face is laid out");
            assert_eq!(outline.samples.len(), 3, "a triangle's outline");
            Drawn {
                face: *face,
                outline,
                triangles: vec![[0, 1, 2]],
                surface: *body.surface(body.face(*face).surface),
            }
        })
        .collect();
    let folds = Folds::of(&mut drawn, &samples, eps, tolerance);

    for (face, other) in [(0, 1), (1, 0)] {
        assert_eq!(
            folds.lying((face, 0)),
            Some((other, 0)),
            "the fold is found from the {} triangle, within {eps}",
            ["hair high", "wide"][face],
        );
    }
}

#[test]
fn a_diagonal_is_turned_though_a_thin_triangle_of_another_face_stands_two_thirds_of_a_tolerance_above_what_it_makes()
 {
    let reach = 12.0;
    let mut build = fixtures::Build::new();
    let [start, end, corner, opposite] = [
        at(0.0, 0.0, 0.0),
        at(reach, 0.0, 0.0),
        at(reach, 8.0, 0.0),
        at(0.0, 8.0, 0.0),
    ]
    .map(|point| build.vertex(point));
    let quad = build.polygon(&[start, end, corner, opposite], DVec3::Z);
    let eps = 1e-9 * reach;
    let hair = at(reach / 2.0, 3e-5, 0.64 * eps);
    let hair_corner = build.vertex(hair);
    let outward = at(reach, 0.0, 0.0).cross(hair).normalize();
    let thin = build.polygon(&[start, end, hair_corner], outward);
    let body = build.finish(reach);
    assert_eq!(
        body.scale().eps(),
        eps,
        "the tolerance the hair is set against"
    );
    let tolerance = 1e-3 * reach;
    let mut samples = Samples::of(&body, tolerance);
    let mut drawn: Vec<Drawn> = [quad, thin]
        .iter()
        .map(|face| Drawn {
            face: *face,
            outline: Outline::of(&body, &mut samples, *face, tolerance)
                .expect("a flat face is laid out"),
            triangles: Vec::new(),
            surface: *body.surface(body.face(*face).surface),
        })
        .collect();
    let index_of = |drawn: &Drawn, point: DVec3| {
        (0..drawn.outline.samples.len())
            .find(|corner| samples.point(drawn.outline.samples[*corner]) == point)
            .expect("a corner of the face is in its outline")
    };
    let turning_left = |drawn: &Drawn, points: [DVec3; 3]| {
        let [a, b, c] = points.map(|point| index_of(drawn, point));
        let [pa, pb, pc] = [a, b, c].map(|corner| drawn.outline.points[corner]);
        if (pb - pa).perp_dot(pc - pa) > 0.0 {
            [a, b, c]
        } else {
            [a, c, b]
        }
    };
    let [v, c, u, d] = [
        at(0.0, 0.0, 0.0),
        at(reach, 0.0, 0.0),
        at(reach, 8.0, 0.0),
        at(0.0, 8.0, 0.0),
    ];
    drawn[0].triangles = vec![
        turning_left(&drawn[0], [u, v, c]),
        turning_left(&drawn[0], [u, v, d]),
    ];
    drawn[1].triangles = vec![turning_left(&drawn[1], [v, c, hair])];
    let across = [index_of(&drawn[0], c), index_of(&drawn[0], d)];
    let samples = &samples;
    let mut folds = Folds::of(&mut drawn, samples, eps, tolerance);

    assert!(
        folds.unfold((0, 0), DEPTH).is_some(),
        "the thin triangle lies a hair above the one the flip makes, and crosses it nowhere",
    );
    assert!(
        folds.drawn[0]
            .triangles
            .iter()
            .all(|triangle| across.iter().all(|corner| triangle.contains(corner))),
        "both triangles of the quad are on the other diagonal: {:?}",
        folds.drawn[0].triangles,
    );
}
