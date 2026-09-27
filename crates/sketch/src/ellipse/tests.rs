//! What sketch · ellipse.rs is held to.
//!
//! Closes #412.
//! - a length on an axis drives the ellipse —
//!   `a_length_typed_on_an_axis_drives_the_ellipse_and_keeps_it_whole`
//! - dragging an axis end turns or stretches that axis and keeps the other
//!   square to it and centred — `an_axis_end_dragged_round_turns_the_other_axis_with_it`,
//!   and the centre stays where it was — `an_axis_end_dragged_alone_leaves_the_centre_where_it_was`,
//!   even when the values given refuse the drag —
//!   `an_axis_end_dragged_against_the_values_given_leaves_the_centre_where_it_was`.
//!   Since #422 a drag of the end stretches its axis along itself, and turns
//!   the ellipse only when taken twice as far off the axis as along it: the
//!   second test says so now, and the first holds what a turn of the axes
//!   still has to keep
//! - dragging the curve scales the ellipse about its centre —
//!   `the_curve_dragged_out_scales_the_ellipse_about_its_centre_and_keeps_its_shape`
//! - erasing the ellipse takes its axes —
//!   `erasing_an_ellipse_takes_its_axes_and_the_points_nothing_else_stands_on`;
//!   erasing an axis takes the ellipse — `erasing_one_axis_takes_the_ellipse_with_it`
//! - snapping onto the curve takes an ellipse — `a_cursor_near_the_curve_is_pulled_onto_it`
//! - a point dropped on the curve stays on it —
//!   `a_point_held_on_the_curve_follows_it_when_an_axis_is_lengthened`
//! - click, box, erase, mirror and patterns take an ellipse — no test: in this file; the
//!   four sweeps walk `one_of_every_kind`, which lays an ellipse, in picking,
//!   banding, duplicating and element tests; the mirror and both patterns go
//!   through `duplicate`

use glam::DVec2;

use super::*;
use crate::constraints::{DimensionTarget, SketchAxis};
use crate::holding::Support;
use crate::plane::WorkPlane;
use crate::resizing::Curved;
use crate::snap::{Snap, SnapSettings};

/// An ellipse centred at (50, 20), 60 wide along x and 20 high.
fn wide_ellipse() -> (Sketch, EllipseId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 10.0));
    let north = sketch.add_point(DVec2::new(50.0, 30.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    (sketch, id)
}

#[test]
fn an_ellipse_is_laid_with_both_its_axes_as_construction_traits() {
    let (sketch, id) = wide_ellipse();
    let ellipse = sketch.ellipses()[id.0];

    for axis in [ellipse.first, ellipse.second] {
        assert!(sketch.segments()[axis.0].construction);
        assert_eq!(sketch.ellipse_of_axis(axis), Some(id));
    }
    let drawn = sketch.ellipse_draft(id);
    assert_eq!(drawn.first.length(), 30.0);
    assert_eq!(drawn.second, 10.0);
}

#[test]
fn a_length_typed_on_an_axis_drives_the_ellipse_and_keeps_it_whole() {
    let (mut sketch, id) = wide_ellipse();
    let second = sketch.ellipses()[id.0].second;

    sketch.set_dimension(DimensionTarget::Length(second), 40.0, false);
    sketch.solve(1.0);

    let drawn = sketch.ellipse_draft(id);
    assert!((drawn.second - 20.0).abs() < 1e-4, "{drawn:?}");
    assert!(
        (drawn.first.length() - 30.0).abs() < 1e-4,
        "the other axis keeps its length: {drawn:?}"
    );
    let [centre, west, east, south, north] = sketch.ellipse_points(id).map(|p| sketch.point(p));
    assert!(centre.distance((west + east) * 0.5) < 1e-4);
    assert!(centre.distance((south + north) * 0.5) < 1e-4);
    assert!(
        (east - west)
            .normalize()
            .dot((north - south).normalize())
            .abs()
            < 1e-4
    );
}

#[test]
fn an_axis_end_dragged_round_turns_the_other_axis_with_it() {
    let (mut sketch, id) = wide_ellipse();
    let [centre, _, east, ..] = sketch.ellipse_points(id);

    let stays = sketch.point(centre);
    sketch.settle_around_all(&[(east, DVec2::new(50.0, 50.0)), (centre, stays)], 1.0);

    let [centre, west, east, south, north] = sketch.ellipse_points(id).map(|p| sketch.point(p));
    assert!(
        centre.distance(DVec2::new(50.0, 20.0)) < 1e-4,
        "the centre stays: {centre}"
    );
    assert!(
        west.distance(DVec2::new(50.0, -10.0)) < 1e-3,
        "the far end follows round: {west}"
    );
    assert!(
        (east - west)
            .normalize()
            .dot((north - south).normalize())
            .abs()
            < 1e-4,
        "the axes stay square",
    );
    assert!(centre.distance((south + north) * 0.5) < 1e-4);
}

#[test]
fn erasing_an_ellipse_takes_its_axes_and_the_points_nothing_else_stands_on() {
    let (mut sketch, id) = wide_ellipse();
    let [centre, west, east, ..] = sketch.ellipse_points(id);
    let beyond = sketch.add_point(DVec2::new(100.0, 20.0));
    let leading_away = sketch.add_segment(east, beyond);
    let ellipse = sketch.ellipses()[id.0];

    sketch.erase(Element::Ellipse(id));

    assert!(sketch.is_erased_segment(ellipse.first));
    assert!(sketch.is_erased_segment(ellipse.second));
    assert!(sketch.is_erased_point(centre));
    assert!(sketch.is_erased_point(west));
    assert!(
        !sketch.is_erased_point(east) && !sketch.is_erased_segment(leading_away),
        "a trait drawn from an axis end keeps its end",
    );
}

#[test]
fn erasing_one_axis_takes_the_ellipse_with_it() {
    let (mut sketch, id) = wide_ellipse();
    let ellipse = sketch.ellipses()[id.0];

    sketch.erase(Element::Segment(ellipse.second));

    assert!(sketch.is_erased_ellipse(id));
    assert!(sketch.is_erased_segment(ellipse.first));
}

#[test]
fn a_dimension_on_an_axis_goes_with_the_ellipse() {
    let (mut sketch, id) = wide_ellipse();
    let first = sketch.ellipses()[id.0].first;
    sketch.set_dimension(DimensionTarget::Length(first), 60.0, false);

    sketch.erase(Element::Ellipse(id));

    assert!(sketch.dimensions().is_empty());
}

#[test]
fn the_curve_is_picked_near_it_and_not_from_its_middle() {
    let (sketch, id) = wide_ellipse();

    assert_eq!(
        sketch.nearest_ellipse(DVec2::new(50.0, 30.5), 1.0),
        Some(id)
    );
    assert_eq!(sketch.nearest_ellipse(DVec2::new(50.0, 21.0), 1.0), None);
}

#[test]
fn a_place_on_the_curve_lands_on_the_ellipse() {
    let (sketch, id) = wide_ellipse();

    let supports = sketch.supports_at(DVec2::new(50.0, 30.0));

    assert!(supports.contains(&Support::Ellipse(id)), "{supports:?}");
}

#[test]
fn a_point_held_on_the_curve_follows_it_when_an_axis_is_lengthened() {
    let (mut sketch, id) = wide_ellipse();
    let drawn = sketch.ellipse_draft(id);
    let point = sketch.add_point(drawn.at(0.7));
    sketch.add_constraint(Support::Ellipse(id).holding(point));
    let first = sketch.ellipses()[id.0].first;

    sketch.set_dimension(DimensionTarget::Length(first), 90.0, false);
    sketch.resolve(1.0);

    let drawn = sketch.ellipse_draft(id);
    assert!((drawn.first.length() - 45.0).abs() < 1e-4, "{drawn:?}");
    let off = drawn.distance(sketch.point(point));
    assert!(off < 1e-4, "the point stands {off} off the curve");
}

#[test]
fn a_point_held_on_the_curve_slides_round_it() {
    let (mut sketch, id) = wide_ellipse();
    let point = sketch.add_point(DVec2::new(80.0, 20.0 + 1e-12));
    sketch.add_constraint(Support::Ellipse(id).holding(point));

    let slid = sketch.slide(point, DVec2::new(50.0, 60.0));

    assert!(slid.distance(DVec2::new(50.0, 30.0)) < 1e-6, "{slid}");
}

#[test]
fn a_cursor_near_the_curve_is_pulled_onto_it() {
    let (sketch, _) = wide_ellipse();
    let settings = SnapSettings {
        point_reach: 1.0,
        curve_reach: 1.0,
        grid_step: None,
        grid_reach: 0.0,
    };
    let (id, _) = sketch.live_ellipses().next().expect("the ellipse");

    let (at, caught) = sketch.magnetise(DVec2::new(40.0, 30.0), &settings);

    assert!(matches!(caught, Some(Snap::OnCurve(_))), "{caught:?}");
    assert!(sketch.ellipse_draft(id).distance(at) < 1e-9, "{at}");
}

#[test]
fn a_press_on_the_curve_takes_hold_of_the_ellipse() {
    let (sketch, id) = wide_ellipse();

    assert_eq!(
        sketch.curve_at(DVec2::new(50.0, 30.2), 0.5),
        Some(Curved::Ellipse(id))
    );
}

#[test]
fn the_curve_dragged_out_scales_the_ellipse_about_its_centre_and_keeps_its_shape() {
    let (mut sketch, id) = wide_ellipse();
    let grabbed = Curved::Ellipse(id);

    let reach = sketch.reach_through(grabbed, DVec2::new(50.0, 35.0));
    assert!((reach - 45.0).abs() < 1e-9, "half as big again: {reach}");
    sketch.resize(grabbed, reach, 1.0);

    let drawn = sketch.ellipse_draft(id);
    assert!(drawn.centre.distance(DVec2::new(50.0, 20.0)) < 1e-6);
    assert!((drawn.first.length() - 45.0).abs() < 1e-6, "{drawn:?}");
    assert!((drawn.second - 15.0).abs() < 1e-6, "{drawn:?}");
}

#[test]
fn an_axis_offers_no_stretch_to_trim() {
    let (mut sketch, id) = wide_ellipse();
    let first = sketch.ellipses()[id.0].first;
    let across = sketch.add_point(DVec2::new(35.0, 0.0));
    let over = sketch.add_point(DVec2::new(35.0, 40.0));
    sketch.add_segment(across, over);

    assert_eq!(sketch.stretch_at(first, DVec2::new(25.0, 20.0)), None);
}

#[test]
fn a_corner_an_axis_makes_with_a_trait_is_not_cut() {
    let (mut sketch, id) = wide_ellipse();
    let [_, _, east, ..] = sketch.ellipse_points(id);
    let first = sketch.ellipses()[id.0].first;
    let up = sketch.add_point(DVec2::new(80.0, 60.0));
    let side = sketch.add_segment(east, up);

    assert!(!sketch.fillet_fits(first, side, 2.0));
    assert!(!sketch.chamfer_fits(first, side, crate::chamfer::Chamfer::Equal(2.0)));
}

#[test]
fn a_centre_merged_onto_another_point_carries_the_ellipse_with_it() {
    let (mut sketch, id) = wide_ellipse();
    let [centre, ..] = sketch.ellipse_points(id);

    sketch.merge_points(Sketch::ORIGIN, centre);

    assert_eq!(sketch.ellipses()[id.0].center, Sketch::ORIGIN);
    assert!(!sketch.is_erased_ellipse(id));
}

#[test]
fn an_axis_end_dragged_alone_leaves_the_centre_where_it_was() {
    let (mut sketch, id) = wide_ellipse();
    let [_, _, east, ..] = sketch.ellipse_points(id);

    sketch.settle_around(east, DVec2::new(90.0, 25.0), 1.0);

    let [centre, west, east, south, north] = sketch.ellipse_points(id).map(|p| sketch.point(p));
    assert!(
        centre.distance(DVec2::new(50.0, 20.0)) < 1e-4,
        "the centre moved to {centre}"
    );
    assert!(
        east.distance(DVec2::new(90.0, 20.0)) < 1e-4,
        "the end lengthens its axis along itself, and does not turn it: {east}"
    );
    assert!(west.distance(DVec2::new(10.0, 20.0)) < 1e-4, "{west}");
    assert!(south.distance(DVec2::new(50.0, 10.0)) < 1e-4, "{south}");
    assert!(north.distance(DVec2::new(50.0, 30.0)) < 1e-4, "{north}");
}

#[test]
fn an_axis_end_dragged_against_the_values_given_leaves_the_centre_where_it_was() {
    let (mut sketch, id) = wide_ellipse();
    let [centre, _, east, ..] = sketch.ellipse_points(id);
    let first = sketch.ellipses()[id.0].first;
    // A length and an angle leave the axis free to travel and nothing else, so
    // the cursor is asking for something the drawing has already refused.
    sketch.set_dimension(DimensionTarget::Length(first), 60.0, false);
    sketch.set_dimension(
        DimensionTarget::AxisAngle {
            segment: first,
            axis: SketchAxis::U,
        },
        30.0,
        false,
    );
    sketch.resolve(1.0);
    let stood = sketch.point(centre);

    sketch.settle_around(east, DVec2::new(90.0, 60.0), 1.0);

    let moved = sketch.point(centre).distance(stood);
    assert!(moved < 1e-4, "the centre slid {moved} towards the cursor");
}
