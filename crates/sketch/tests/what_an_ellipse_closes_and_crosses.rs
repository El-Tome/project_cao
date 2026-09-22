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
//! - what must not break: a circle drawn inside an ellipse against its short
//!   axis leaves the three areas the drawing shows —
//!   `a_circle_drawn_inside_an_ellipse_against_its_short_axis_leaves_the_three_areas_it_shows`,
//!   read off a part drawn by hand; a circle touching an ellipse meets it once on each
//!   side, which cuts the ring between them in two —
//!   `a_circle_touching_an_ellipse_at_its_short_axis_leaves_three_areas`;
//!   a circle an
//!   ellipse overlaps is still cut into the
//!   areas the two make — `a_circle_an_ellipse_overlaps_is_cut_by_it_into_the_areas_they_make`;
//!   a trait drawn to one of the ellipse's own handles closes an area there —
//!   `a_trait_drawn_to_a_handle_of_the_ellipse_closes_an_area_there`; and one
//!   curve drawn twice, its axes given the other way round, crosses itself
//!   nowhere — `the_same_ellipse_drawn_twice_the_other_way_round_crosses_itself_nowhere`
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

#[test]
fn a_circle_an_ellipse_overlaps_is_cut_by_it_into_the_areas_they_make() {
    let (mut sketch, _) = an_ellipse();
    let centre = sketch.add_point(DVec2::new(80.0, 20.0));
    sketch.add_circle(centre, 20.0);

    let regions = sketch.regions();

    assert_eq!(
        regions.len(),
        3,
        "the lens, and what each curve keeps of its own: {:?}",
        regions.iter().map(spanned).collect::<Vec<f64>>(),
    );
}

#[test]
fn a_trait_drawn_to_a_handle_of_the_ellipse_closes_an_area_there() {
    let (mut sketch, _) = an_ellipse();
    let north = sketch
        .live_points()
        .find(|(_, place)| place.distance(DVec2::new(50.0, 40.0)) < 1e-9)
        .expect("the handle at the top of the second axis")
        .0;
    let on_the_curve = sketch.add_point(a_place_on_it());
    sketch.add_segment(north, on_the_curve);

    let regions = sketch.regions();

    assert_eq!(
        regions.len(),
        2,
        "the chord closes a piece off against the curve: {:?}",
        regions.iter().map(spanned).collect::<Vec<f64>>(),
    );
}

#[test]
fn the_same_ellipse_drawn_twice_the_other_way_round_crosses_itself_nowhere() {
    let (mut sketch, _) = an_ellipse();
    // The very same curve, its short axis given as the first one.
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 0.0));
    let north = sketch.add_point(DVec2::new(50.0, 40.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    sketch.add_ellipse(centre, [south, north], [west, east]);

    let crossings = sketch.crossings();

    assert!(
        crossings.is_empty(),
        "one curve drawn twice crosses itself nowhere, not {} times",
        crossings.len(),
    );
}

#[test]
fn a_circle_touching_an_ellipse_at_its_short_axis_leaves_three_areas() {
    let (mut sketch, _) = an_ellipse();
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    sketch.add_circle(centre, 20.0);

    let regions = sketch.regions();
    let spans: Vec<f64> = regions.iter().map(spanned).collect();
    assert_eq!(
        regions.len(),
        3,
        "the circle, and the two the ring is pinched into: {spans:?}",
    );
    let circle = std::f64::consts::PI * 20.0 * 20.0;
    let ring = std::f64::consts::PI * 30.0 * 20.0 - circle;
    let total: f64 = spans.iter().sum();
    assert!(
        (total - circle - ring).abs() < (circle + ring) * 0.01,
        "and they cover the ellipse between them: {spans:?}",
    );
}

#[test]
fn a_circle_drawn_inside_an_ellipse_against_its_short_axis_leaves_the_three_areas_it_shows() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    // Read off a part drawn by hand: the ellipse leans a little, and the
    // circle is a hair under the reach of the short axis it sits against.
    let middle = DVec2::new(32.499_999_765_805_16, 20.000_002_498_089_7);
    let along = DVec2::new(40.000_000_792_693_78, 3.749_991_544_566_136);
    let across = along.perp().normalize() * 15.354_612_009_812_739;
    let centre = sketch.add_point(middle);
    let west = sketch.add_point(middle - along);
    let east = sketch.add_point(middle + along);
    let south = sketch.add_point(middle - across);
    let north = sketch.add_point(middle + across);
    sketch.add_ellipse(centre, [west, east], [south, north]);
    let round = sketch.add_point(middle);
    sketch.add_circle(round, 15.354_612_005_382_88);

    let regions = sketch.regions();

    let spans: Vec<f64> = regions.iter().map(spanned).collect();
    assert_eq!(
        regions.len(),
        3,
        "the circle, and the two the ring is pinched into: {spans:?}",
    );
    let circle = std::f64::consts::PI * 15.354_612 * 15.354_612;
    let lobe = (std::f64::consts::PI * along.length() * 15.354_612 - circle) * 0.5;
    for (found, wanted) in spans.iter().zip([circle, lobe, lobe]) {
        assert!(
            (found - wanted).abs() < wanted * 0.02,
            "the disc and the two lobes are {circle:.0} and {lobe:.0}, not {spans:?}",
        );
    }
}
