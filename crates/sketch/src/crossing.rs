//! Where two of the drawing's curves meet away from any point it holds.
//!
//! Each answer is given as how far along the pieces stand rather than as a
//! place, because that is what cutting them apart needs: a place would have to
//! be turned back into a fraction before anything could be split at it.

use glam::DVec2;

use crate::arcing::{ArcDraft, sweep_of};

/// How far along each of the two segments they cross, or `None` when they do
/// not: the pair of fractions of `a1..a2` and of `b1..b2`, both strictly
/// between nought and one.
///
/// Touching at an end is not crossing. A vertex two edges already share is
/// where the walk can turn anyway, and splitting there would only hand it the
/// same graph with an extra name in it.
pub(crate) fn where_segments_cross(
    a1: DVec2,
    a2: DVec2,
    b1: DVec2,
    b2: DVec2,
) -> Option<(f64, f64)> {
    let (along, across) = (a2 - a1, b2 - b1);
    let turn = along.perp_dot(across);
    if turn.abs() < along.length() * across.length() * PARALLEL {
        return None;
    }
    let gap = b1 - a1;
    let (t, u) = (gap.perp_dot(across) / turn, gap.perp_dot(along) / turn);
    let inside = |fraction: f64| fraction > 0.0 && fraction < 1.0;
    (inside(t) && inside(u)).then_some((t, u))
}

/// Where a straight run and a curve cross, each as a pair of fractions: how
/// far along `a1..a2`, and how far round the arc's own sweep.
///
/// A line meets a circle twice, so there can be two of them; a crossing that
/// lands on the part of the circle the arc does not run over is not one.
pub(crate) fn where_segment_crosses_arc(a1: DVec2, a2: DVec2, arc: ArcDraft) -> Vec<(f64, f64)> {
    let (along, reach) = (a2 - a1, a1 - arc.centre);
    let radius = arc.centre.distance(arc.start);
    let (a, b, c) = (
        along.length_squared(),
        2.0 * reach.dot(along),
        reach.length_squared() - radius * radius,
    );
    let discriminant = b * b - 4.0 * a * c;
    if discriminant <= 0.0 {
        return Vec::new();
    }
    let root = discriminant.sqrt();
    let mut found: Vec<(f64, f64)> = [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)]
        .into_iter()
        .filter(|t| *t > 0.0 && *t < 1.0)
        .filter_map(|t| round_arc(arc, a1 + along * t).map(|round| (t, round)))
        .collect();
    found.sort_by(|left, right| left.0.total_cmp(&right.0));
    found
}

/// How far round its own sweep the arc stands at that place, or `None` when
/// the place is on the rest of the circle — the part the arc does not run
/// over — or on one of its two ends.
fn round_arc(arc: ArcDraft, place: DVec2) -> Option<f64> {
    let sweep = sweep_of(arc);
    let from = (arc.start - arc.centre).to_angle();
    let turned = ((place - arc.centre).to_angle() - from).rem_euclid(std::f64::consts::TAU);
    (turned > 0.0 && turned < sweep).then(|| turned / sweep)
}

/// Where two curves cross, each as the pair of fractions of their own sweeps.
///
/// Two circles meet twice, and either meeting can fall outside what one of the
/// two arcs actually runs over. Curves sharing a centre never cross, however
/// their radii compare.
pub(crate) fn where_arcs_cross(first: ArcDraft, second: ArcDraft) -> Vec<(f64, f64)> {
    let apart = second.centre - first.centre;
    let span = apart.length();
    let (near, far) = (
        first.centre.distance(first.start),
        second.centre.distance(second.start),
    );
    if span <= 0.0 || span >= near + far || span <= (near - far).abs() {
        return Vec::new();
    }
    let reach = (near * near - far * far + span * span) / (2.0 * span);
    let rise = (near * near - reach * reach).max(0.0).sqrt();
    let along = apart / span;
    let middle = first.centre + along * reach;
    let sideways = DVec2::new(-along.y, along.x) * rise;

    [middle + sideways, middle - sideways]
        .into_iter()
        .filter_map(|place| Some((round_arc(first, place)?, round_arc(second, place)?)))
        .collect()
}

/// Below this much of a turn between two directions they are taken as
/// parallel: a sine, so the same figure whatever the drawing is measured in.
const PARALLEL: f64 = 1e-12;

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-9;

    #[test]
    fn two_segments_that_cross_meet_part_way_along_each_of_them() {
        let found = where_segments_cross(
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(2.0, -1.0),
            DVec2::new(2.0, 3.0),
        );
        let Some((along_a, along_b)) = found else {
            panic!("the two segments cross at (2, 0) and nothing was reported");
        };
        assert!(
            (along_a - 0.2).abs() < TOLERANCE,
            "a fifth of the way along the first, got {along_a}",
        );
        assert!(
            (along_b - 0.25).abs() < TOLERANCE,
            "a quarter of the way along the second, got {along_b}",
        );
    }

    #[test]
    fn a_segment_that_only_ends_on_another_is_touching_it_not_crossing_it() {
        assert_eq!(
            where_segments_cross(
                DVec2::new(0.0, 0.0),
                DVec2::new(10.0, 0.0),
                DVec2::new(4.0, 0.0),
                DVec2::new(4.0, 6.0),
            ),
            None,
        );
    }

    #[test]
    fn two_segments_along_the_same_line_never_cross() {
        assert_eq!(
            where_segments_cross(
                DVec2::new(0.0, 0.0),
                DVec2::new(10.0, 0.0),
                DVec2::new(3.0, 0.0),
                DVec2::new(13.0, 0.0),
            ),
            None,
        );
    }

    fn upper_half_circle(radius: f64) -> ArcDraft {
        ArcDraft {
            centre: DVec2::ZERO,
            start: DVec2::new(radius, 0.0),
            end: DVec2::new(-radius, 0.0),
        }
    }

    #[test]
    fn a_straight_run_through_a_curve_cuts_it_twice() {
        let found = where_segment_crosses_arc(
            DVec2::new(-10.0, 3.0),
            DVec2::new(10.0, 3.0),
            upper_half_circle(5.0),
        );
        assert_eq!(found.len(), 2, "a chord of the bulge, got {found:?}");

        let (along_a, round_arc) = found[0];
        assert!(
            (along_a - 0.3).abs() < TOLERANCE,
            "the first meeting is three tenths along the run, got {along_a}",
        );
        let expected = (std::f64::consts::PI - 3.0f64.atan2(4.0)) / std::f64::consts::PI;
        assert!(
            (round_arc - expected).abs() < TOLERANCE,
            "and {expected} of the way round the curve, got {round_arc}",
        );

        assert!(
            (found[1].0 - 0.7).abs() < TOLERANCE,
            "the second is seven tenths along the run, got {}",
            found[1].0,
        );
    }

    #[test]
    fn a_chord_passing_below_a_bulge_never_meets_it() {
        assert!(
            where_segment_crosses_arc(
                DVec2::new(-10.0, -3.0),
                DVec2::new(10.0, -3.0),
                upper_half_circle(5.0),
            )
            .is_empty(),
            "the run cuts the circle where the arc does not go",
        );
    }

    #[test]
    fn two_curves_meet_only_where_both_of_them_run() {
        let mut shifted = upper_half_circle(5.0);
        shifted.centre = DVec2::new(6.0, 0.0);
        shifted.start = DVec2::new(11.0, 0.0);
        shifted.end = DVec2::new(1.0, 0.0);

        let found = where_arcs_cross(upper_half_circle(5.0), shifted);
        assert_eq!(
            found.len(),
            1,
            "the circles meet at (3, 4) and (3, -4), and only the first is on both arcs: {found:?}",
        );

        let (round_first, round_second) = found[0];
        let expected = 4.0f64.atan2(3.0) / std::f64::consts::PI;
        assert!(
            (round_first - expected).abs() < TOLERANCE,
            "{expected} of the way round the first, got {round_first}",
        );
        let expected = 4.0f64.atan2(-3.0) / std::f64::consts::PI;
        assert!(
            (round_second - expected).abs() < TOLERANCE,
            "{expected} of the way round the second, got {round_second}",
        );
    }
}
