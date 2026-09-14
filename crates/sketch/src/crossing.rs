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
    let radius = arc.centre.distance(arc.start);
    along_segment_at_circle(a1, a2, arc.centre, radius)
        .into_iter()
        .filter_map(|along| round_arc(arc, a1.lerp(a2, along)).map(|round| (along, round)))
        .collect()
}

/// Where a straight run crosses a whole circle: how far along `a1..a2`, and
/// how far round the turn measured from the circle's own zero.
///
/// A circle has no ends, so unlike an arc it rejects neither of the two.
pub(crate) fn where_segment_crosses_circle(
    a1: DVec2,
    a2: DVec2,
    centre: DVec2,
    radius: f64,
) -> Vec<(f64, f64)> {
    along_segment_at_circle(a1, a2, centre, radius)
        .into_iter()
        .map(|along| (along, turn_at(centre, a1.lerp(a2, along))))
        .collect()
}

/// Where two curves cross, each as the pair of fractions of their own sweeps.
///
/// Two circles meet twice, and either meeting can fall outside what one of the
/// two arcs actually runs over. Curves sharing a centre never cross, however
/// their radii compare.
pub(crate) fn where_arcs_cross(first: ArcDraft, second: ArcDraft) -> Vec<(f64, f64)> {
    let (near, far) = (
        first.centre.distance(first.start),
        second.centre.distance(second.start),
    );
    where_circles_meet(first.centre, near, second.centre, far)
        .into_iter()
        .filter_map(|place| Some((round_arc(first, place)?, round_arc(second, place)?)))
        .collect()
}

/// Where a curve crosses a whole circle: how far round the arc's own sweep,
/// and how far round the circle's turn.
pub(crate) fn where_arc_crosses_circle(
    arc: ArcDraft,
    centre: DVec2,
    radius: f64,
) -> Vec<(f64, f64)> {
    let reach = arc.centre.distance(arc.start);
    where_circles_meet(arc.centre, reach, centre, radius)
        .into_iter()
        .filter_map(|place| Some((round_arc(arc, place)?, turn_at(centre, place))))
        .collect()
}

/// Where two whole circles cross, each as a fraction of its own turn.
pub(crate) fn where_circles_cross(
    near: DVec2,
    near_radius: f64,
    far: DVec2,
    far_radius: f64,
) -> Vec<(f64, f64)> {
    where_circles_meet(near, near_radius, far, far_radius)
        .into_iter()
        .map(|place| (turn_at(near, place), turn_at(far, place)))
        .collect()
}

/// How far round the turn a place stands, as a fraction of a whole one,
/// measured from the direction the maths calls zero.
pub(crate) fn turn_at(centre: DVec2, place: DVec2) -> f64 {
    (place - centre)
        .to_angle()
        .rem_euclid(std::f64::consts::TAU)
        / std::f64::consts::TAU
}

/// How far along the run it meets the circle, nearest end first, leaving out
/// the two ends themselves.
fn along_segment_at_circle(a1: DVec2, a2: DVec2, centre: DVec2, radius: f64) -> Vec<f64> {
    let (along, reach) = (a2 - a1, a1 - centre);
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
    let mut found: Vec<f64> = [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)]
        .into_iter()
        .filter(|along| *along > 0.0 && *along < 1.0)
        .collect();
    found.sort_by(f64::total_cmp);
    found
}

/// The two places two whole circles meet, or nothing when they miss, nest or
/// share a centre.
fn where_circles_meet(near: DVec2, near_radius: f64, far: DVec2, far_radius: f64) -> Vec<DVec2> {
    let apart = far - near;
    let span = apart.length();
    if span <= 0.0 || span >= near_radius + far_radius || span <= (near_radius - far_radius).abs() {
        return Vec::new();
    }
    let reach = (near_radius * near_radius - far_radius * far_radius + span * span) / (2.0 * span);
    let rise = (near_radius * near_radius - reach * reach).max(0.0).sqrt();
    let along = apart / span;
    let middle = near + along * reach;
    let sideways = DVec2::new(-along.y, along.x) * rise;
    vec![middle + sideways, middle - sideways]
}

/// How far round its own sweep the arc stands at that place, or `None` when
/// the place is on the rest of the circle — the part the arc does not run
/// over — or on one of its two ends.
pub(crate) fn round_arc(arc: ArcDraft, place: DVec2) -> Option<f64> {
    let sweep = sweep_of(arc);
    let from = (arc.start - arc.centre).to_angle();
    let turned = ((place - arc.centre).to_angle() - from).rem_euclid(std::f64::consts::TAU);
    (turned > 0.0 && turned < sweep).then(|| turned / sweep)
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
    fn a_straight_run_through_a_whole_circle_keeps_both_of_its_meetings() {
        let mut found = where_segment_crosses_circle(
            DVec2::new(-10.0, 3.0),
            DVec2::new(10.0, 3.0),
            DVec2::ZERO,
            5.0,
        );
        assert_eq!(found.len(), 2, "a chord of the circle, got {found:?}");

        let turn = std::f64::consts::TAU;
        for (expected_along, expected_round) in [
            (0.3, (3.0f64).atan2(-4.0) / turn),
            (0.7, (3.0f64).atan2(4.0) / turn),
        ] {
            let (along, round) = found.remove(0);
            assert!(
                (along - expected_along).abs() < TOLERANCE,
                "{expected_along} of the way along the run, got {along}",
            );
            assert!(
                (round - expected_round).abs() < TOLERANCE,
                "and {expected_round} of the way round the circle, got {round}",
            );
        }
    }

    #[test]
    fn two_whole_circles_meet_twice_wherever_they_overlap() {
        let found = where_circles_cross(DVec2::ZERO, 5.0, DVec2::new(6.0, 0.0), 5.0);
        assert_eq!(
            found.len(),
            2,
            "the two circles meet at (3, 4) and (3, -4): {found:?}"
        );

        let turn = std::f64::consts::TAU;
        let (round_near, round_far) = found[0];
        assert!(
            (round_near - (4.0f64).atan2(3.0) / turn).abs() < TOLERANCE,
            "at (3, 4) the first has turned {round_near}",
        );
        assert!(
            (round_far - (4.0f64).atan2(-3.0) / turn).abs() < TOLERANCE,
            "and the second {round_far}",
        );
    }

    #[test]
    fn two_circles_that_miss_each_other_never_meet() {
        assert!(where_circles_cross(DVec2::ZERO, 1.0, DVec2::new(9.0, 0.0), 2.0).is_empty());
        assert!(
            where_circles_cross(DVec2::ZERO, 5.0, DVec2::ZERO, 2.0).is_empty(),
            "one inside the other, sharing a centre",
        );
    }

    #[test]
    fn a_curve_crossing_a_whole_circle_reports_only_the_meetings_it_runs_over() {
        let found = where_arc_crosses_circle(upper_half_circle(5.0), DVec2::new(6.0, 0.0), 5.0);
        assert_eq!(
            found.len(),
            1,
            "the circles meet at (3, 4) and (3, -4), and the arc runs over the first only: {found:?}",
        );

        let (round_arc, round_circle) = found[0];
        let expected = (4.0f64).atan2(3.0) / std::f64::consts::PI;
        assert!(
            (round_arc - expected).abs() < TOLERANCE,
            "{expected} of the way round the arc, got {round_arc}",
        );
        let expected = (4.0f64).atan2(-3.0) / std::f64::consts::TAU;
        assert!(
            (round_circle - expected).abs() < TOLERANCE,
            "and {expected} of the way round the circle, got {round_circle}",
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
