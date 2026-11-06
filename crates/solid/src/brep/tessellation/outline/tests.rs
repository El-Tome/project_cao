//! A face laid out in its surface's parameters, read without a body: loops
//! given as raw parameters, as a surface reads its samples.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::Outline;

fn empty() -> Outline {
    Outline {
        points: Vec::new(),
        samples: Vec::new(),
        segments: Vec::new(),
        at: Default::default(),
        walls: Vec::new(),
        strips: Vec::new(),
    }
}

/// The corners of a polygon in `(θ, h)`, each side cut into steps short
/// enough to unwrap, each angle read back into `(−π, π]` as a surface
/// reads it.
fn sampled(corners: &[[f64; 2]]) -> Vec<DVec2> {
    let steps = 64;
    (0..corners.len())
        .flat_map(|index| {
            let (from, to) = (
                DVec2::from(corners[index]),
                DVec2::from(corners[(index + 1) % corners.len()]),
            );
            (0..steps).map(move |step| from.lerp(to, step as f64 / steps as f64))
        })
        .map(|at| DVec2::new((at.x + PI).rem_euclid(TAU) - PI, at.y))
        .collect()
}

/// The area the triangles cover in the parameters.
fn covered(outline: &Outline, triangles: &[[usize; 3]]) -> f64 {
    triangles
        .iter()
        .map(|&[a, b, c]| {
            let [a, b, c] = [a, b, c].map(|corner| outline.points[corner]);
            (b - a).perp_dot(c - a).abs() / 2.0
        })
        .sum()
}

/// A band sheared round a cylinder over more than a turn, from `θ = −0.81`
/// to `7.09`, without winding: at each height its angles span 1.11, a turn
/// further on 2.77 higher. The hole's raw angles, `[−0.3, −0.1]` at height
/// 2.9, stand in the band only a turn on, `[5.98, 6.18]` (86751972230147).
#[test]
fn a_hole_in_the_far_turn_of_a_face_wider_than_a_turn_is_laid_where_the_face_holds_it() {
    let band = [[-0.81, 0.0], [0.3, 0.0], [7.09, 3.0], [5.98, 3.0]];
    let hole = [[-0.3, 2.85], [-0.3, 2.95], [-0.1, 2.95], [-0.1, 2.85]];
    let raws = vec![sampled(&band), sampled(&hole)];
    let laps: Vec<Vec<usize>> = vec![
        (0..raws[0].len()).collect(),
        (raws[0].len()..raws[0].len() + raws[1].len()).collect(),
    ];
    let mut outline = empty();
    outline
        .round_laid(&raws, &laps, 64, |_| true)
        .expect("a face that does not wind is laid as it is");
    let triangles = outline.cut().expect("the region is swept");
    let area = |corners: &[[f64; 2]]| {
        (0..corners.len())
            .map(|index| {
                DVec2::from(corners[index])
                    .perp_dot(DVec2::from(corners[(index + 1) % corners.len()]))
            })
            .sum::<f64>()
            .abs()
            / 2.0
    };
    let region = area(&band) - area(&hole);
    let swept = covered(&outline, &triangles);
    assert!(
        (swept - region).abs() <= 1e-9 * region,
        "the triangles cover {swept}, the face {region}"
    );
}
