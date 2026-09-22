//! An ellipse among the other curves: the areas it closes, the places it
//! crosses them, and what a division makes of those.
//!
//! Closes #413.
//! - a closed ellipse is an area — `a_lone_ellipse_encloses_one_area`; its
//!   wall extrudes as one curved face — no test here: that is the part layer's,
//!   held by `an_ellipse_extrudes_with_one_face_for_its_wall` in
//!   `crates/part/tests/an_ellipse_encloses_an_area.rs`
//! - a trait across an ellipse leaves two areas —
//!   `a_trait_across_an_ellipse_leaves_two_areas`
//! - snapping finds the crossings of an ellipse with a trait, a circle, an arc
//!   and another ellipse — `an_ellipse_crosses_a_trait`,
//!   `an_ellipse_crosses_a_circle`, `an_ellipse_crosses_an_arc`,
//!   `an_ellipse_crosses_another_ellipse`, and the cursor is pulled onto one —
//!   `the_cursor_is_pulled_onto_a_crossing_on_an_ellipse`
//! - a division through an ellipse is refused, as one through a circle is —
//!   `a_crossing_an_ellipse_runs_through_is_refused_rather_than_half_divided`.
//!   The issue said the point would be dropped there; the circle's rule won
//!   instead, since neither curve has ends and one point divides neither.

use cao_sketch::{Crossing, Sketch, Snap, SnapSettings, WorkPlane};
use glam::DVec2;

/// How much ground a region covers, read off the triangles it is cut into.
fn spanned(region: &cao_sketch::Region) -> f64 {
    region
        .face_triangles()
        .iter()
        .map(|[a, b, c]| ((*b - *a).perp_dot(*c - *a) * 0.5).abs())
        .sum()
}

/// An ellipse about (50, 20), sixty wide and forty high.
fn an_ellipse() -> (Sketch, [DVec2; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 0.0));
    let north = sketch.add_point(DVec2::new(50.0, 40.0));
    sketch.add_ellipse(centre, [west, east], [south, north]);
    (sketch, [DVec2::new(20.0, 20.0), DVec2::new(80.0, 20.0)])
}

#[test]
fn a_lone_ellipse_encloses_one_area() {
    let (sketch, _) = an_ellipse();

    let regions = sketch.regions();

    assert_eq!(regions.len(), 1, "{regions:?}");
    let area = spanned(&regions[0]);
    let wanted = std::f64::consts::PI * 30.0 * 20.0;
    assert!(
        (area - wanted).abs() < wanted * 0.01,
        "the area is {area}, where the ellipse encloses {wanted}",
    );
}

#[test]
fn a_trait_across_an_ellipse_leaves_two_areas() {
    let (mut sketch, _) = an_ellipse();
    let low = sketch.add_point(DVec2::new(50.0, -20.0));
    let high = sketch.add_point(DVec2::new(50.0, 60.0));
    sketch.add_segment(low, high);

    let regions = sketch.regions();

    assert_eq!(regions.len(), 2, "{regions:?}");
    let whole = std::f64::consts::PI * 30.0 * 20.0;
    for region in &regions {
        let area = spanned(region);
        assert!(
            (area - whole * 0.5).abs() < whole * 0.01,
            "each half is {area}, where half the ellipse is {}",
            whole * 0.5,
        );
    }
}

/// A place on the ellipse a sixth of a turn round, away from every handle.
fn a_place_on_it() -> DVec2 {
    DVec2::new(65.0, 20.0 + 20.0 * 3.0_f64.sqrt() / 2.0)
}

/// Whether a place stands on the ellipse of the fixture.
fn on_the_ellipse(place: &DVec2) -> bool {
    let west_to_east = (place.x - 50.0) / 30.0;
    let south_to_north = (place.y - 20.0) / 20.0;
    (west_to_east * west_to_east + south_to_north * south_to_north - 1.0).abs() < 1e-6
}

#[test]
fn an_ellipse_crosses_a_trait() {
    let (mut sketch, _) = an_ellipse();
    let low = sketch.add_point(DVec2::new(35.0, -20.0));
    let high = sketch.add_point(DVec2::new(35.0, 60.0));
    sketch.add_segment(low, high);

    let found = sketch.crossings();

    assert_eq!(
        found.iter().filter(|at| on_the_ellipse(at)).count(),
        2,
        "{found:?}"
    );
}

#[test]
fn an_ellipse_crosses_a_circle() {
    let (mut sketch, _) = an_ellipse();
    let centre = sketch.add_point(DVec2::new(80.0, 40.0));
    sketch.add_circle(centre, 22.0);

    let found = sketch.crossings();

    assert_eq!(
        found.iter().filter(|at| on_the_ellipse(at)).count(),
        2,
        "{found:?}"
    );
}

#[test]
fn an_ellipse_crosses_an_arc() {
    let (mut sketch, _) = an_ellipse();
    // A quarter of a circle about the ellipse's own centre, big enough to
    // reach past its short axis and not its long one, so it cuts it twice.
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let start = sketch.add_point(DVec2::new(75.0, 20.0));
    let end = sketch.add_point(DVec2::new(50.0, 45.0));
    sketch.add_arc(centre, start, end);

    let found = sketch.crossings();

    assert_eq!(
        found.iter().filter(|at| on_the_ellipse(at)).count(),
        1,
        "{found:?}"
    );
}

#[test]
fn an_ellipse_crosses_another_ellipse() {
    let (mut sketch, _) = an_ellipse();
    let centre = sketch.add_point(DVec2::new(50.0, 45.0));
    let west = sketch.add_point(DVec2::new(30.0, 45.0));
    let east = sketch.add_point(DVec2::new(70.0, 45.0));
    let south = sketch.add_point(DVec2::new(50.0, 25.0));
    let north = sketch.add_point(DVec2::new(50.0, 65.0));
    sketch.add_ellipse(centre, [west, east], [south, north]);

    let found = sketch.crossings();

    assert_eq!(
        found.iter().filter(|at| on_the_ellipse(at)).count(),
        2,
        "{found:?}"
    );
}

#[test]
fn the_cursor_is_pulled_onto_a_crossing_on_an_ellipse() {
    let (mut sketch, _) = an_ellipse();
    let low = sketch.add_point(DVec2::new(35.0, -20.0));
    let high = sketch.add_point(DVec2::new(35.0, 60.0));
    sketch.add_segment(low, high);
    let crossing = DVec2::new(35.0, 20.0 + 20.0 * (1.0 - 0.25_f64).sqrt());
    let settings = SnapSettings {
        point_reach: 2.0,
        curve_reach: 2.0,
        grid_step: None,
        grid_reach: 0.0,
    };

    let (at, caught) = sketch.magnetise(crossing + DVec2::new(0.4, 0.5), &settings);

    assert!(matches!(caught, Some(Snap::Crossing(_))), "{caught:?}");
    assert!(
        at.distance(crossing) < 1e-6,
        "{at} is not the crossing {crossing}"
    );
}

#[test]
fn a_crossing_an_ellipse_runs_through_is_refused_rather_than_half_divided() {
    let (mut sketch, _) = an_ellipse();
    let on = a_place_on_it();
    // Two traits crossing each other exactly where the ellipse runs.
    let (from, to) = (
        sketch.add_point(on + DVec2::new(-10.0, -10.0)),
        sketch.add_point(on + DVec2::new(10.0, 10.0)),
    );
    sketch.add_segment(from, to);
    let (across_from, across_to) = (
        sketch.add_point(on + DVec2::new(-10.0, 10.0)),
        sketch.add_point(on + DVec2::new(10.0, -10.0)),
    );
    sketch.add_segment(across_from, across_to);

    let named = sketch.crossing_at(on, 1.0);

    assert_eq!(
        named,
        Some(Crossing::Round),
        "the two traits cross where the ellipse runs, so the division is refused whole",
    );
}
