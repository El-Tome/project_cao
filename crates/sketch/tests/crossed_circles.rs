use cao_sketch::{Region, Sketch, Snap, SnapSettings, WorkPlane};
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

/// The circle is sampled, so every area read off one is a polygon's and falls
/// a little short of the disc it stands for.
const SAMPLED: f64 = 0.01;

#[test]
fn a_run_straight_through_a_circle_leaves_two_half_discs() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, radius);
    let from = sketch.add_point(DVec2::new(-8.0, 0.0));
    let to = sketch.add_point(DVec2::new(8.0, 0.0));
    sketch.add_segment(from, to);

    let found = areas(&sketch);
    assert_eq!(found.len(), 2, "the run halves the disc: {found:?}");
    let half = std::f64::consts::PI * radius * radius / 2.0;
    assert!(
        found
            .iter()
            .all(|piece| (piece - half).abs() / half < SAMPLED),
        "each half is about {half}, got {found:?}",
    );
}

#[test]
fn two_overlapping_circles_leave_a_lens_between_two_crescents() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let near = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(near, radius);
    let far = sketch.add_point(DVec2::new(6.0, 0.0));
    sketch.add_circle(far, radius);

    let found = areas(&sketch);
    assert_eq!(found.len(), 3, "a lens and two crescents: {found:?}");

    let lens = radius * radius * (2.0 * (3.0f64 / radius).acos() - 0.96);
    let disc = std::f64::consts::PI * radius * radius;
    assert!(
        (found[0] - lens).abs() / lens < SAMPLED,
        "the lens is about {lens}, got {}",
        found[0],
    );
    assert!(
        found[1..]
            .iter()
            .all(|crescent| (crescent - (disc - lens)).abs() / (disc - lens) < SAMPLED),
        "and each crescent about {}, got {found:?}",
        disc - lens,
    );
}

#[test]
fn a_chord_between_two_points_of_a_circle_cuts_a_cap_off_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, radius);
    let from = sketch.add_point(DVec2::new(4.0, 3.0));
    let to = sketch.add_point(DVec2::new(-4.0, 3.0));
    sketch.add_segment(from, to);

    let found = areas(&sketch);
    assert_eq!(found.len(), 2, "the cap and what is left: {found:?}");

    let cap = radius * radius / 2.0 * (2.0 * (3.0f64 / radius).acos() - 0.96);
    let disc = std::f64::consts::PI * radius * radius;
    assert!(
        (found[0] - cap).abs() / cap < SAMPLED,
        "the cap is about {cap}, got {}",
        found[0],
    );
    assert!(
        (found[1] - (disc - cap)).abs() / (disc - cap) < SAMPLED,
        "and the rest about {}, got {}",
        disc - cap,
        found[1],
    );
}

#[test]
fn a_single_point_on_a_circle_leaves_it_whole() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, radius);
    sketch.add_point(DVec2::new(radius, 0.0));

    let found = areas(&sketch);
    assert_eq!(found.len(), 1, "nothing has cut it: {found:?}");
    let disc = std::f64::consts::PI * radius * radius;
    assert!(
        (found[0] - disc).abs() / disc < SAMPLED,
        "still the whole disc, about {disc}, got {}",
        found[0],
    );
}

#[test]
fn the_cursor_catches_the_place_a_run_enters_a_circle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let radius = 5.0;
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, radius);
    let from = sketch.add_point(DVec2::new(-8.0, 0.0));
    let to = sketch.add_point(DVec2::new(8.0, 0.0));
    sketch.add_segment(from, to);

    let magnets = SnapSettings {
        point_reach: 1.0,
        curve_reach: 1.0,
        grid_step: None,
        grid_reach: 0.0,
    };
    let (at, caught) = sketch.magnetise(DVec2::new(4.8, 0.2), &magnets);

    assert!(
        matches!(caught, Some(Snap::Crossing(_))),
        "the run enters the circle at (5, 0), and the cursor met {caught:?}",
    );
    assert!(
        at.distance(DVec2::new(radius, 0.0)) < 1e-9,
        "pulled onto (5, 0), got {at}",
    );
}
