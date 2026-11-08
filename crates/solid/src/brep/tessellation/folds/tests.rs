//! The folds in miniature: a few triangles laid by hand, and what the folds
//! read of them.
//!
//! Closes #768.
//! - `passed` in the check and the folds' clear tier read one function of
//!   `brep/crossing.rs`, and `folds/along.rs` is gone — no test: structural,
//!   checked by review
//! - for each of the three gaps, a test in miniature beside the folds shows
//!   it caught — faces that end on the line:
//!   `four_faces_ending_on_one_line_where_two_skins_cross_pass_through_each_other_seen_from_each`;
//!   a line inside a face, with matter above and below:
//!   `two_faces_ending_on_a_line_inside_a_third_one_above_it_one_below_pass_through_it_seen_from_each`;
//!   stretches:
//!   `a_face_with_a_corner_partway_along_another_s_edge_passes_through_it_there_seen_from_each_triangle`
//! - no draw regresses against `main`, seed by seed, on the three sets, both
//!   kernels — no test: a campaign compared seed by seed, on the pull request

use glam::{DVec2, DVec3};

use super::{At, DEPTH, Drawn, Folds, opens, through};
use crate::brep::tessellation::outline::Outline;
use crate::brep::tessellation::sampling::Samples;
use crate::brep::tessellation::tests::fixtures;
use crate::soundness::{Flaw, Triangle, uncrossed};

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

fn two_faces_bent_round_one_edge_seen_from(wing: usize) -> bool {
    let reach = 12.0;
    let half = reach / 2.0;
    let around = |degrees: f64| {
        let (sin, cos) = degrees.to_radians().sin_cos();
        at(half * cos, half * sin, half)
    };
    let mut build = fixtures::Build::new();
    let [bottom, top] = [at(0.0, 0.0, 0.0), at(0.0, 0.0, reach)];
    let ends = [bottom, top].map(|point| build.vertex(point));
    let faces = [0.0, 5.0].map(|degrees| {
        let wing = around(degrees);
        let corner = build.vertex(wing);
        build.polygon(&[ends[0], ends[1], corner], top.cross(wing).normalize())
    });
    let body = build.finish(reach);
    let eps = body.scale().eps();
    let tolerance = 1e-3 * reach;
    let mut samples = Samples::of(&body, tolerance);
    let outlines = faces.map(|face| {
        Outline::of(&body, &mut samples, face, tolerance).expect("a flat face is laid out")
    });
    let mut drawn: Vec<Drawn> = faces
        .into_iter()
        .zip(outlines)
        .zip([170.0, 175.0])
        .map(|((face, mut outline), degrees)| {
            let index_of = |outline: &Outline, point: DVec3| {
                (0..outline.samples.len())
                    .find(|corner| samples.point(outline.samples[*corner]) == point)
                    .expect("a corner of the triangle is in its outline")
            };
            assert_eq!(outline.samples.len(), 3, "a triangle's outline");
            let [low, high] = [bottom, top].map(|point| index_of(&outline, point));
            let near = 3 - low - high;
            outline.samples.push(samples.added(around(degrees)));
            outline.points.push(DVec2::ZERO);
            Drawn {
                face,
                outline,
                triangles: vec![[low, high, near], [high, low, 3]],
                surface: *body.surface(body.face(face).surface),
            }
        })
        .collect();
    let folds = Folds::of(&mut drawn, &samples, eps, tolerance);
    folds.crossed_along((0, wing))
}

#[test]
fn a_face_bent_from_0_to_170_degrees_round_an_edge_and_one_bent_from_5_to_175_pass_through_each_other_seen_from_the_wing_at_0()
 {
    assert!(
        two_faces_bent_round_one_edge_seen_from(0),
        "the other face's wings, at 5 and 175 degrees, lie on one side of this wing's plane, yet on either side of the wing at 170",
    );
}

#[test]
fn a_face_bent_from_0_to_170_degrees_round_an_edge_and_one_bent_from_5_to_175_pass_through_each_other_seen_from_the_wing_at_170()
 {
    assert!(
        two_faces_bent_round_one_edge_seen_from(1),
        "the other face's wings, at 5 and 175 degrees, lie either side of this wing's plane",
    );
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

/// A flat face round `outline`, cut into `triangles`, each turning
/// counterclockwise seen from the side the face looks to.
type ByHand<'a> = (&'a [DVec3], &'a [Triangle]);

/// Faces laid by hand within `reach`, every point one sample whichever faces
/// have it, as the folds read them.
fn laid(faces: &[ByHand], reach: f64) -> (Vec<Drawn>, Samples, f64, f64) {
    let mut build = fixtures::Build::new();
    let mut vertices = Vec::new();
    let mut ids = Vec::new();
    for (outline, triangles) in faces {
        let mut corners = Vec::new();
        for point in outline.iter() {
            let vertex = match vertices.iter().find(|(at, _)| at == point) {
                Some((_, vertex)) => *vertex,
                None => {
                    let vertex = build.vertex(*point);
                    vertices.push((*point, vertex));
                    vertex
                }
            };
            corners.push(vertex);
        }
        let [a, b, c] = triangles[0];
        ids.push(build.polygon(&corners, (b - a).cross(c - a).normalize()));
    }
    let body = build.finish(reach);
    let (eps, tolerance) = (body.scale().eps(), 1e-3 * reach);
    let mut samples = Samples::of(&body, tolerance);
    let mut outlines = Vec::new();
    for face in &ids {
        outlines.push(
            Outline::of(&body, &mut samples, *face, tolerance).expect("a flat face is laid out"),
        );
    }
    let mut known: Vec<(DVec3, usize)> = outlines
        .iter()
        .flat_map(|outline| outline.samples.iter().map(|id| (samples.point(*id), *id)))
        .collect();
    let mut drawn = Vec::new();
    for ((face, mut outline), (_, triangles)) in ids.into_iter().zip(outlines).zip(faces) {
        let mut cut = Vec::new();
        for corners in triangles.iter() {
            cut.push(corners.map(|point| {
                let id = match known.iter().find(|(at, _)| *at == point) {
                    Some((_, id)) => *id,
                    None => {
                        let id = samples.added(point);
                        known.push((point, id));
                        id
                    }
                };
                match outline.samples.iter().position(|each| *each == id) {
                    Some(corner) => corner,
                    None => {
                        outline.samples.push(id);
                        outline.points.push(DVec2::ZERO);
                        outline.samples.len() - 1
                    }
                }
            }));
        }
        drawn.push(Drawn {
            face,
            outline,
            triangles: cut,
            surface: *body.surface(body.face(face).surface),
        });
    }
    (drawn, samples, eps, tolerance)
}

#[test]
fn a_triangle_threaded_through_a_triangle_of_another_face_is_found_crossing_it_from_each() {
    let flat = [at(0.0, 0.0, 0.0), at(12.0, 0.0, 0.0), at(0.0, 12.0, 0.0)];
    let standing = [at(3.0, 3.0, -6.0), at(3.0, 3.0, 6.0), at(-6.0, 3.0, 0.0)];
    let faces: [ByHand; 2] = [(&flat, &[flat]), (&standing, &[standing])];
    let (mut drawn, samples, eps, tolerance) = laid(&faces, 12.0);
    let folds = Folds::of(&mut drawn, &samples, eps, tolerance);
    assert_eq!(folds.crossing((0, 0)), Some((1, 0)));
    assert_eq!(folds.crossing((1, 0)), Some((0, 0)));
}

#[test]
fn a_pass_over_a_triangle_lying_on_one_of_another_face_meets_the_fold_from_each_and_takes_no_flip()
{
    let [a, b] = [at(0.0, 0.0, 0.0), at(12.0, 0.0, 0.0)];
    let hair = [a, b, at(6.0, 3e-5, 2e-9)];
    let wide = [a, b, at(6.0, 12.0, 0.0)];
    let faces: [ByHand; 2] = [(&hair, &[hair]), (&wide, &[wide])];
    let (mut drawn, samples, eps, tolerance) = laid(&faces, 12.0);
    let mut folds = Folds::of(&mut drawn, &samples, eps, tolerance);
    assert_eq!(
        folds.pass(),
        (2, false),
        "each triangle is a face of its own, with no diagonal to turn",
    );
}

#[test]
fn a_triangle_threaded_through_a_triangle_of_another_face_counts_one_crossing() {
    let flat = [at(0.0, 0.0, 0.0), at(12.0, 0.0, 0.0), at(0.0, 12.0, 0.0)];
    let standing = [at(3.0, 3.0, -6.0), at(3.0, 3.0, 6.0), at(-6.0, 3.0, 0.0)];
    let faces: [ByHand; 2] = [(&flat, &[flat]), (&standing, &[standing])];
    let (mut drawn, samples, eps, tolerance) = laid(&faces, 12.0);
    let folds = Folds::of(&mut drawn, &samples, eps, tolerance);
    assert_eq!(
        folds.crossings(0, folds.drawn[0].triangles[0], usize::MAX),
        1
    );
}

#[test]
fn a_pass_over_a_thin_triangle_lying_on_a_quad_turns_the_quad_s_diagonal() {
    let reach = 12.0;
    let [v, c, u, d] = [
        at(0.0, 0.0, 0.0),
        at(reach, 0.0, 0.0),
        at(reach, 8.0, 0.0),
        at(0.0, 8.0, 0.0),
    ];
    let hair = at(reach / 2.0, 3e-5, 0.64e-9 * reach);
    let thin = [v, c, hair];
    let faces: [ByHand; 2] = [(&[v, c, u, d], &[[u, v, c], [u, d, v]]), (&thin, &[thin])];
    let (mut drawn, samples, eps, tolerance) = laid(&faces, reach);
    let mut folds = Folds::of(&mut drawn, &samples, eps, tolerance);
    let (_, taken) = folds.pass();
    assert!(
        taken,
        "the thin triangle lies on one the quad's other diagonal does not make"
    );
}

#[test]
fn two_faces_meeting_along_an_edge_pass_through_nothing_along_it() {
    let [p, q] = [at(0.0, 0.0, 0.0), at(12.0, 0.0, 0.0)];
    let flat = [p, q, at(6.0, 6.0, 0.0)];
    let wall = [q, p, at(6.0, 0.0, -6.0)];
    let faces: [ByHand; 2] = [(&flat, &[flat]), (&wall, &[wall])];
    let (mut drawn, samples, eps, tolerance) = laid(&faces, 12.0);
    let folds = Folds::of(&mut drawn, &samples, eps, tolerance);
    assert!(!folds.crossed_along((0, 0)));
    assert!(!folds.crossed_along((1, 0)));
}

#[test]
fn two_faces_laid_back_to_back_are_a_skin_no_line_is_read_through() {
    let [p, q, r, s] = [
        at(0.0, 0.0, 0.0),
        at(12.0, 0.0, 0.0),
        at(6.0, 6.0, 0.0),
        at(6.0, -6.0, 0.0),
    ];
    let faces: [ByHand; 2] = [
        (&[p, s, q, r], &[[p, q, r], [q, p, s]]),
        (&[p, r, q, s], &[[q, p, r], [p, q, s]]),
    ];
    let (mut drawn, samples, eps, tolerance) = laid(&faces, 12.0);
    let folds = Folds::of(&mut drawn, &samples, eps, tolerance);
    assert!(
        !folds.crossed_along((0, 0)),
        "its twin on the same three samples makes it a skin of no thickness, left out with it",
    );
}

/// The triangles of faces laid by hand the folds do not see passing through
/// another face along a line, where the rules see the faces pass through each
/// other and no two triangles cross or lie on each other.
fn missed_along(faces: &[ByHand], reach: f64) -> Vec<At> {
    let (mut drawn, samples, eps, tolerance) = laid(faces, reach);
    let triangles: Vec<Triangle> = drawn
        .iter()
        .flat_map(|each| {
            let point = |corner: usize| samples.point(each.outline.samples[corner]);
            each.triangles.iter().map(move |corners| corners.map(point))
        })
        .collect();
    assert!(
        matches!(uncrossed(&triangles), Err(Flaw::Crossing { .. })),
        "the rules see the faces pass through each other: {triangles:?}",
    );
    let every: Vec<At> = (0..drawn.len())
        .flat_map(|face| (0..drawn[face].triangles.len()).map(move |each| (face, each)))
        .collect();
    let folds = Folds::of(&mut drawn, &samples, eps, tolerance);
    for at in &every {
        assert_eq!(
            (folds.crossing(*at), folds.lying(*at)),
            (None, None),
            "the triangle {at:?} crosses nothing and lies on nothing",
        );
    }
    every
        .into_iter()
        .filter(|at| !folds.crossed_along(*at))
        .collect()
}

#[test]
fn four_faces_ending_on_one_line_where_two_skins_cross_pass_through_each_other_seen_from_each() {
    let [p, q, left, right, up, down] = [
        at(6.0, 0.0, 0.0),
        at(6.0, 12.0, 0.0),
        at(0.0, 6.0, 0.0),
        at(12.0, 6.0, 0.0),
        at(6.0, 6.0, 6.0),
        at(6.0, 6.0, -6.0),
    ];
    let faces = [[p, q, left], [q, p, right], [p, q, up], [q, p, down]];
    let laid: Vec<ByHand> = faces
        .iter()
        .map(|face| (face.as_slice(), std::slice::from_ref(face)))
        .collect();
    assert_eq!(
        missed_along(&laid, 12.0),
        Vec::<At>::new(),
        "no face has two wings on the line, yet going round it the floor's halves and the wall's run the same way as their neighbours",
    );
}

#[test]
fn two_faces_ending_on_a_line_inside_a_third_one_above_it_one_below_pass_through_it_seen_from_each()
{
    let flat = [at(0.0, 0.0, 0.0), at(12.0, 0.0, 0.0), at(0.0, 12.0, 0.0)];
    let [start, end] = [at(2.0, 3.0, 0.0), at(6.0, 3.0, 0.0)];
    let rising = [start, end, at(4.0, 6.0, 4.0)];
    let sinking = [end, start, at(4.0, 1.0, -4.0)];
    let faces = [flat, rising, sinking];
    let laid: Vec<ByHand> = faces
        .iter()
        .map(|face| (face.as_slice(), std::slice::from_ref(face)))
        .collect();
    assert_eq!(
        missed_along(&laid, 12.0),
        Vec::<At>::new(),
        "the line lies inside the flat face, one face going on from it above and another below",
    );
}

#[test]
fn a_face_with_a_corner_partway_along_another_s_edge_passes_through_it_there_seen_from_each_triangle()
 {
    let [p, q, partway, left, right, up, down] = [
        at(6.0, 0.0, 0.0),
        at(6.0, 12.0, 0.0),
        at(6.0, 4.0, 0.0),
        at(0.0, 6.0, 0.0),
        at(12.0, 6.0, 0.0),
        at(6.0, 6.0, 6.0),
        at(6.0, 6.0, -6.0),
    ];
    let floor = [
        [p, partway, left],
        [partway, q, left],
        [partway, p, right],
        [q, partway, right],
    ];
    let wall = [[p, q, up], [q, p, down]];
    let laid: [ByHand; 2] = [(&[left, p, right, q], &floor), (&[p, down, q, up], &wall)];
    assert_eq!(
        missed_along(&laid, 12.0),
        Vec::<At>::new(),
        "the floor's edges along the line end at a corner partway along the wall's, and the two pass through each other along both stretches",
    );
}
