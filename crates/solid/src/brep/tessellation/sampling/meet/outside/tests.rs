use std::f64::consts::{FRAC_PI_2, PI};

use glam::DVec3;

use super::kept_outside;
use crate::brep::curve::Meet;
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::tessellation::contact::Wall;
use crate::brep::tessellation::tests::fixtures;
use crate::brep::topology::{Body, EdgeId};

const TOLERANCE: f64 = 0.02;

const PROBES: usize = 64;

/// The meet of the stock and a hole of radius 5 bored across it along Y,
/// from its top on the stock's wall at (0, 20, 10) a quarter round the hole
/// down to (-5, 19.4, 5), between two vertices; and a wall along Z of
/// `radius`, inside the stock and touching its wall along the line through
/// the top.
fn a_quarter_meet_and_a_wall_touching_the_stock_inside_at_its_top(
    radius: f64,
) -> (Body, EdgeId, Meet, Vec<Wall>) {
    let mut build = fixtures::Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, fixtures::STOCK_RADIUS);
    let bore = build.cylinder(DVec3::Z * 5.0, DVec3::Y, fixtures::HOLE_RADIUS);
    let meet = build
        .meeting(stock, bore, fixtures::STOCK_RADIUS)
        .components[0];
    let (from, to) = (FRAC_PI_2, PI);
    let top = meet.point(from);
    let ends = [build.vertex(top), build.vertex(meet.point(to))];
    let edge = build.meet_edge(meet, Some(ends), from, to);
    let across = DVec3::new(top.x, top.y, 0.0);
    let center = across * (1.0 - radius / across.length());
    let wall = build.cylinder(center, DVec3::Z, radius);
    let body = build.finish(fixtures::STOCK_RADIUS);
    let Surface::Cylinder(cylinder) = *body.surface(wall) else {
        unreachable!("the wall is round")
    };
    (body, edge, meet, vec![(wall, cylinder)])
}

/// The edge's points from its first end to its last, `between` in the
/// middle.
fn run_of(body: &Body, edge: EdgeId, between: &[DVec3]) -> Vec<DVec3> {
    let ends = body.edge(edge).ends.expect("between two vertices");
    let mut run = vec![body.vertex(ends[0]).point];
    run.extend_from_slice(between);
    run.push(body.vertex(ends[1]).point);
    run
}

/// How deep the chord between `start` and `end` passes inside `wall` at its
/// deepest, probed at [`PROBES`] places along it.
fn deepest_inside(wall: &Cylinder, start: DVec3, end: DVec3) -> f64 {
    (1..PROBES)
        .map(|part| -wall.distance(start.lerp(end, part as f64 / PROBES as f64)))
        .fold(0.0, f64::max)
}

#[test]
fn a_meet_s_chord_passing_inside_a_wall_its_end_stands_on_where_the_curve_stays_outside_is_cut_until_it_stays_outside()
 {
    let (body, edge, meet, walls) =
        a_quarter_meet_and_a_wall_touching_the_stock_inside_at_its_top(10.0);
    let eps = body.scale().eps();
    let wall = &walls[0].1;
    let before = run_of(&body, edge, &[]);
    assert!(
        deepest_inside(wall, before[0], before[1]) > 1e3 * eps,
        "the fixture's chord stays outside the wall"
    );

    let kept = kept_outside(&body, edge, &meet, &walls, TOLERANCE, Vec::new());

    assert!(!kept.is_empty(), "the chord is left whole");
    let run = run_of(&body, edge, &kept);
    let mut last = FRAC_PI_2;
    for point in &kept {
        let t = meet.parameter(*point);
        assert!(
            (meet.point(t) - *point).length() < 1e-9,
            "{point} is off the curve"
        );
        assert!(last < t && t < PI, "{t} out of order after {last}");
        last = t;
    }
    for pair in run.windows(2) {
        let depth = deepest_inside(wall, pair[0], pair[1]);
        assert!(
            depth <= eps,
            "the chord from {} to {} passes {depth} inside the wall",
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn a_meet_s_chords_ending_within_the_tolerance_of_a_wall_a_hair_inside_its_own_are_left_alone() {
    let hair = 1e-3;
    let (body, edge, meet, walls) = a_quarter_meet_and_a_wall_touching_the_stock_inside_at_its_top(
        fixtures::STOCK_RADIUS - hair,
    );
    let eps = body.scale().eps();
    let wall = &walls[0].1;
    let points: Vec<DVec3> = (1..4)
        .map(|step| meet.point(FRAC_PI_2 + FRAC_PI_2 * step as f64 / 4.0))
        .collect();
    let run = run_of(&body, edge, &points);
    for point in &run {
        assert!(
            wall.distance(*point).abs() < TOLERANCE,
            "{point} stands further than the tolerance from the wall"
        );
    }
    assert!(
        run.windows(2)
            .all(|pair| deepest_inside(wall, pair[0], pair[1]) > 1e3 * eps),
        "a chord of the fixture stays outside the wall"
    );
    assert!(
        run.iter().any(|point| wall.distance(*point) > 1e2 * eps),
        "the fixture's curve lies on the wall"
    );

    let kept = kept_outside(&body, edge, &meet, &walls, TOLERANCE, points.clone());

    assert_eq!(kept, points);
}

#[test]
fn a_meet_s_own_samples_stand_bit_for_bit_among_those_the_wall_adds() {
    let (body, edge, meet, walls) =
        a_quarter_meet_and_a_wall_touching_the_stock_inside_at_its_top(10.0);
    let points: Vec<DVec3> = [0.25, 0.5, 0.75]
        .map(|share| meet.point(FRAC_PI_2 + FRAC_PI_2 * share) + DVec3::X * 1e-10)
        .to_vec();

    let kept = kept_outside(&body, edge, &meet, &walls, TOLERANCE, points.clone());

    assert!(kept.len() > points.len(), "the wall adds no sample");
    let mut found = kept.iter();
    for point in &points {
        assert!(
            found.any(|kept| kept.to_array() == point.to_array()),
            "{point} is not kept as it was, in its place"
        );
    }
}
