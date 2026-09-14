use cao_sketch::{Region, Sketch, WorkPlane};
use glam::DVec2;

fn area(region: &Region) -> f64 {
    region
        .triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
        .sum()
}

fn areas(sketch: &Sketch) -> Vec<f64> {
    let mut found: Vec<f64> = sketch.regions().iter().map(area).collect();
    found.sort_by(f64::total_cmp);
    found
}

const TOLERANCE: f64 = 1e-9;

fn square_of(side: f64) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(side, 0.0));
    let c = sketch.add_point(DVec2::new(side, side));
    let d = sketch.add_point(DVec2::new(0.0, side));
    for (from, to) in [(a, b), (b, c), (c, d), (d, a)] {
        sketch.add_segment(from, to);
    }
    sketch
}

#[test]
fn a_run_from_one_side_to_the_opposite_one_halves_the_square() {
    let mut sketch = square_of(10.0);
    assert_eq!(sketch.regions().len(), 1, "the square starts as one region");

    let below = sketch.add_point(DVec2::new(5.0, 0.0));
    let above = sketch.add_point(DVec2::new(5.0, 10.0));
    sketch.add_segment(below, above);

    let found = areas(&sketch);
    assert_eq!(found.len(), 2, "the run splits the square in two");
    assert!(
        found.iter().all(|half| (half - 50.0).abs() < TOLERANCE),
        "each half is 50, got {found:?}",
    );
}

#[test]
fn a_chord_ending_on_a_curve_closes_the_piece_above_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let centre = sketch.add_point(DVec2::ZERO);
    let right = sketch.add_point(DVec2::new(radius, 0.0));
    let left = sketch.add_point(DVec2::new(-radius, 0.0));
    sketch.add_arc(centre, right, left);

    let from = sketch.add_point(DVec2::new(4.0, 3.0));
    let to = sketch.add_point(DVec2::new(-4.0, 3.0));
    sketch.add_segment(from, to);

    let found = areas(&sketch);
    assert_eq!(found.len(), 1, "the chord shuts the top of the bulge off");
    let expected = radius * radius / 2.0 * (2.0 * (3.0f64 / radius).acos() - 0.96);
    assert!(
        (found[0] - expected).abs() / expected < 0.01,
        "the piece above the chord is about {expected}, got {}",
        found[0],
    );
}

#[test]
fn a_whisker_leaving_a_side_and_going_nowhere_encloses_nothing_more() {
    let mut sketch = square_of(10.0);
    let foot = sketch.add_point(DVec2::new(5.0, 10.0));
    let tip = sketch.add_point(DVec2::new(5.0, 16.0));
    sketch.add_segment(foot, tip);

    let found = areas(&sketch);
    assert_eq!(found.len(), 1, "a dead end bounds no area: {found:?}");
    assert!(
        (found[0] - 100.0).abs() < TOLERANCE,
        "and the square is still the square, got {}",
        found[0],
    );
}
