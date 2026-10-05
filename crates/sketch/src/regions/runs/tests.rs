//! What sketch · regions/runs.rs is held to: an outline hands a kernel each of
//! its runs as what it is — a straight step, a piece of a circle about its
//! centre turned so far, or a piece of an ellipse — rather than the steps it
//! was sampled into. Part of #526.

use std::f64::consts::TAU;

use super::*;
use crate::plane::WorkPlane;
use crate::regions::Region;
use crate::sketch::Sketch;

/// Far under anything the drawing is worth, and far over what summing a few
/// hundred sampled turns loses.
const TOLERANCE: f64 = 1e-9;

fn only_area(sketch: &Sketch) -> Region {
    let mut regions = sketch.regions();
    assert_eq!(regions.len(), 1, "the drawing was meant to close one area");
    regions.remove(0)
}

#[test]
fn a_circle_reads_as_one_whole_turn_about_its_centre() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(3.0, -4.0));
    sketch.add_circle(centre, 10.0);

    let runs = only_area(&sketch).outline.runs();

    assert_eq!(
        runs.len(),
        1,
        "a circle has no corner to cut it at: {runs:?}"
    );
    let (corner, leg) = runs[0];
    let Leg::Round { centre, turned } = leg else {
        panic!("a circle is a round run, got {leg:?}");
    };
    assert!(
        centre.distance(DVec2::new(3.0, -4.0)) < TOLERANCE,
        "{centre}"
    );
    assert!(
        (corner.distance(centre) - 10.0).abs() < TOLERANCE,
        "the one corner sits on the circle, got {corner}",
    );
    assert!(
        (turned.abs() - TAU).abs() < TOLERANCE,
        "the whole way round, got {turned}",
    );
}

/// The surface the runs enclose, signed as they are walked, by Green's
/// theorem run by run — read off the runs alone, so a run with the wrong
/// centre, the wrong turn or the wrong sign shows in it.
fn enclosed(runs: &[(DVec2, Leg)]) -> f64 {
    let twice: f64 = (0..runs.len())
        .map(|index| {
            let (from, leg) = runs[index];
            let to = runs[(index + 1) % runs.len()].0;
            match leg {
                Leg::Straight => from.perp_dot(to),
                Leg::Round { centre, turned } => {
                    let radius = from.distance(centre);
                    centre.perp_dot(to - from) + radius * radius * turned
                }
                Leg::Oval => panic!("an oval run encloses nothing this reads"),
            }
        })
        .sum();
    twice / 2.0
}

/// Every round run, turned about its centre from its corner, lands on the
/// corner the next run leaves.
fn assert_each_round_run_ends_where_the_next_begins(runs: &[(DVec2, Leg)]) {
    for (index, (from, leg)) in runs.iter().enumerate() {
        let Leg::Round { centre, turned } = *leg else {
            continue;
        };
        let next = runs[(index + 1) % runs.len()].0;
        let landed = centre + DVec2::from_angle(turned).rotate(*from - centre);
        assert!(
            landed.distance(next) < 1e-7,
            "run {index} turned {turned} from {from} about {centre} lands on \
             {landed}, and the next run leaves {next}",
        );
    }
}

fn points(sketch: &mut Sketch, places: &[(f64, f64)]) -> Vec<crate::PointId> {
    places
        .iter()
        .map(|(x, y)| sketch.add_point(DVec2::new(*x, *y)))
        .collect()
}

#[test]
fn a_rounded_rectangle_reads_four_quarter_turns_between_four_straight_runs() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let rim = points(
        &mut sketch,
        &[
            (5.0, 0.0),
            (35.0, 0.0),
            (40.0, 5.0),
            (40.0, 15.0),
            (35.0, 20.0),
            (5.0, 20.0),
            (0.0, 15.0),
            (0.0, 5.0),
        ],
    );
    let centres = points(
        &mut sketch,
        &[(35.0, 5.0), (35.0, 15.0), (5.0, 15.0), (5.0, 5.0)],
    );
    for side in 0..4 {
        sketch.add_segment(rim[2 * side], rim[2 * side + 1]);
        sketch.add_arc(centres[side], rim[2 * side + 1], rim[(2 * side + 2) % 8]);
    }

    let runs = only_area(&sketch).outline.runs();

    assert_eq!(runs.len(), 8, "four sides and four corners: {runs:?}");
    let straight = runs.iter().filter(|(_, leg)| *leg == Leg::Straight).count();
    assert_eq!(straight, 4, "{runs:?}");
    for (index, (_, leg)) in runs.iter().enumerate() {
        if let Leg::Round { turned, .. } = leg {
            assert!(
                (turned.abs() - TAU / 4.0).abs() < TOLERANCE,
                "each corner turns a quarter, got {turned}",
            );
            assert_eq!(
                runs[(index + 1) % 8].1,
                Leg::Straight,
                "a corner sits between two sides: {runs:?}",
            );
        }
    }
    assert_each_round_run_ends_where_the_next_begins(&runs);
    let expected = 40.0 * 20.0 - 4.0 * (25.0 - TAU / 4.0 * 25.0 / 2.0);
    assert!(
        (enclosed(&runs).abs() - expected).abs() < 1e-7,
        "the runs alone enclose {}, the arithmetic says {expected}",
        enclosed(&runs),
    );
}

/// The one round run of an outline, and the surface all its runs enclose,
/// signed the way it is walked.
fn the_round_run_and_the_surface(region: &Region) -> (f64, f64) {
    let runs = region.outline.runs();
    assert_each_round_run_ends_where_the_next_begins(&runs);
    let turned: Vec<f64> = runs
        .iter()
        .filter_map(|(_, leg)| match leg {
            Leg::Round { turned, .. } => Some(*turned),
            _ => None,
        })
        .collect();
    assert_eq!(turned.len(), 1, "one arc in this outline: {runs:?}");
    (turned[0], enclosed(&runs))
}

#[test]
fn an_arc_walked_backwards_turns_the_other_way() {
    let upper_half = |sketch: &mut Sketch| {
        let ends = points(sketch, &[(10.0, 0.0), (-10.0, 0.0)]);
        let centre = sketch.add_point(DVec2::ZERO);
        sketch.add_arc(centre, ends[0], ends[1]);
        ends
    };

    let mut dome = Sketch::new(WorkPlane::XY);
    let ends = upper_half(&mut dome);
    dome.add_segment(ends[1], ends[0]);
    let (turned, surface) = the_round_run_and_the_surface(&only_area(&dome));
    assert!(
        (turned.abs() - TAU / 2.0).abs() < TOLERANCE && turned * surface > 0.0,
        "closed by its chord, the arc bulges out of the area and turns the way \
         the loop is walked: turned {turned}, surface {surface}",
    );
    assert!(
        (surface.abs() - TAU / 4.0 * 100.0).abs() < 1e-7,
        "half a disc of ten, got {surface}",
    );

    let mut bitten = Sketch::new(WorkPlane::XY);
    let ends = upper_half(&mut bitten);
    let top = points(&mut bitten, &[(10.0, 20.0), (-10.0, 20.0)]);
    bitten.add_segment(ends[0], top[0]);
    bitten.add_segment(top[0], top[1]);
    bitten.add_segment(top[1], ends[1]);
    let (turned, surface) = the_round_run_and_the_surface(&only_area(&bitten));
    assert!(
        (turned.abs() - TAU / 2.0).abs() < TOLERANCE && turned * surface < 0.0,
        "the same arc bitten into a plate is walked against the way it was \
         drawn, and turns against the loop: turned {turned}, surface {surface}",
    );
    assert!(
        (surface.abs() - (400.0 - TAU / 4.0 * 100.0)).abs() < 1e-7,
        "a plate twenty square less half a disc of ten, got {surface}",
    );
}

#[test]
fn an_ellipse_reads_as_an_oval_run() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let first = points(&mut sketch, &[(20.0, 0.0), (-20.0, 0.0)]);
    let second = points(&mut sketch, &[(0.0, 8.0), (0.0, -8.0)]);
    sketch.add_ellipse(centre, [first[0], first[1]], [second[0], second[1]]);

    let runs = only_area(&sketch).outline.runs();

    assert_eq!(runs.len(), 1, "a whole ellipse has one corner: {runs:?}");
    assert_eq!(runs[0].1, Leg::Oval, "{runs:?}");
}

#[test]
fn a_fillet_reads_as_a_round_run_between_the_two_straight_runs_it_joins() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = points(
        &mut sketch,
        &[(0.0, 0.0), (40.0, 0.0), (40.0, 20.0), (0.0, 20.0)],
    );
    let sides: Vec<_> = (0..4)
        .map(|side| sketch.add_segment(corners[side], corners[(side + 1) % 4]))
        .collect();
    sketch
        .fillet(sides[0], sides[1], 5.0)
        .expect("a fillet of five fits a corner of a forty by twenty");

    let runs = only_area(&sketch).outline.runs();

    assert_eq!(runs.len(), 5, "four sides and one rounded corner: {runs:?}");
    let round = runs
        .iter()
        .position(|(_, leg)| matches!(leg, Leg::Round { .. }))
        .expect("the rounded corner is a round run");
    let Leg::Round { centre, turned } = runs[round].1 else {
        unreachable!()
    };
    assert!(
        centre.distance(DVec2::new(35.0, 5.0)) < 1e-7,
        "the fillet stands five in from both sides, got {centre}",
    );
    assert!(
        (turned.abs() - TAU / 4.0).abs() < TOLERANCE,
        "a square corner rounded turns a quarter, got {turned}",
    );
    for beside in [round + 4, round + 1] {
        assert_eq!(runs[beside % 5].1, Leg::Straight, "{runs:?}");
    }
    assert_each_round_run_ends_where_the_next_begins(&runs);
    let expected = 800.0 - (25.0 - TAU / 4.0 * 25.0 / 2.0);
    assert!(
        (enclosed(&runs).abs() - expected).abs() < 1e-7,
        "the runs alone enclose {}, the arithmetic says {expected}",
        enclosed(&runs),
    );
}

#[test]
fn a_circle_trimmed_down_to_its_chord_reads_the_long_way_round() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let circle = sketch.add_circle(centre, 10.0);
    let chord = points(&mut sketch, &[(6.0, 8.0), (-6.0, 8.0)]);
    sketch.add_segment(chord[0], chord[1]);
    sketch
        .trim_circle(circle, Some((chord[0], chord[1])))
        .expect("the stretch over the chord can be cut away");

    let (turned, surface) = the_round_run_and_the_surface(&only_area(&sketch));

    let cap = 2.0 * 6.0_f64.atan2(8.0);
    assert!(
        (turned.abs() - (TAU - cap)).abs() < TOLERANCE,
        "what is left of the circle turns the long way round, {}, got {turned}",
        TAU - cap,
    );
    let expected = TAU / 2.0 * 100.0 - 100.0 / 2.0 * (cap - cap.sin());
    assert!(
        (surface.abs() - expected).abs() < 1e-7,
        "a disc of ten less the cap over the chord, {expected}, got {surface}",
    );
}

#[test]
fn a_hole_hands_over_its_own_runs() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = points(
        &mut sketch,
        &[(0.0, 0.0), (40.0, 0.0), (40.0, 30.0), (0.0, 30.0)],
    );
    for side in 0..4 {
        sketch.add_segment(corners[side], corners[(side + 1) % 4]);
    }
    let middle = sketch.add_point(DVec2::new(20.0, 15.0));
    sketch.add_circle(middle, 5.0);

    let regions = sketch.regions();
    let plate = regions
        .iter()
        .find(|region| !region.holes.is_empty())
        .expect("the rectangle is hollow of the circle");

    let outside = plate.outline.runs();
    assert_eq!(outside.len(), 4, "{outside:?}");
    assert!(outside.iter().all(|(_, leg)| *leg == Leg::Straight));
    assert_eq!(plate.holes.len(), 1);
    let hole = plate.holes[0].runs();
    assert_eq!(hole.len(), 1, "a round hole has one corner: {hole:?}");
    let Leg::Round { centre, turned } = hole[0].1 else {
        panic!("the hole is a round run, got {hole:?}");
    };
    assert!(centre.distance(DVec2::new(20.0, 15.0)) < TOLERANCE);
    assert!((turned.abs() - TAU).abs() < TOLERANCE, "{turned}");
    let matter = enclosed(&outside).abs() - enclosed(&hole).abs();
    assert!(
        (matter - (1200.0 - TAU / 2.0 * 25.0)).abs() < 1e-7,
        "forty by thirty less a disc of five, got {matter}",
    );
}
