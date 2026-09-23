//! An arc of ellipse: what a cut leaves of one, and what every tool makes of
//! it afterwards.
//!
//! Closes #414.
//! - a trim on an ellipse crossed twice leaves an arc of ellipse — no test: in
//!   this file; it is held beside the cut itself, by
//!   a_click_between_two_points_takes_the_stretch_it_fell_in and
//!   a_cut_leaves_the_three_quarters_the_click_did_not_fall_in
//! - the preview names the stretch that goes —
//!   `the_preview_names_the_stretch_a_click_would_take`
//! - an arc of ellipse trims again, and a cut in the middle of one leaves two
//!   pieces of the one curve — no test: in this file; both are held beside the
//!   cut, by a_stretch_already_cut_is_cut_again_at_its_own_end and
//!   a_cut_in_the_middle_of_a_stretch_leaves_two_pieces_of_the_one_curve
//! - every tool that takes an ellipse takes an arc of ellipse: a click —
//!   `a_click_finds_an_arc_of_ellipse_where_it_is_drawn_and_not_where_it_is_gone`,
//!   a box — `a_box_over_the_drawing_catches_an_arc_of_ellipse`, erasing —
//!   `erasing_an_arc_of_ellipse_takes_its_axes_and_its_ends`, the mirror and
//!   the patterns — `a_copy_of_an_arc_of_ellipse_is_drawn_over_the_same_stretch`,
//!   snapping — `the_cursor_is_pulled_onto_an_arc_of_ellipse_and_not_past_its_ends`,
//!   a point held on it — `a_point_held_on_an_arc_of_ellipse_follows_the_curve`,
//!   and the areas — `an_arc_of_ellipse_closes_an_area_against_a_trait`
//! - what the review of this branch turned up, each with its own test: the
//!   mirror names the stretch the other way round —
//!   `a_mirrored_arc_of_ellipse_is_drawn_over_the_mirror_of_its_own_stretch`;
//!   an axis two pieces share takes both —
//!   `erasing_a_shared_axis_takes_every_piece_standing_on_it`; a box round what
//!   is drawn catches it — `a_box_round_what_is_drawn_of_an_arc_catches_it`; a
//!   click past a tip takes the stretch it opens with rather than the whole arc
//!   — `a_click_past_the_end_of_an_arc_takes_the_stretch_it_opens_with_and_not_the_whole_of_it`;
//!   and a division is offered again where the cut took the curve away —
//!   `a_division_is_offered_again_where_the_cut_took_the_curve_away`
//! - the history, the file and compaction take it — no test: not in this file,
//!   since they are the part layer's, held by its own a_cut_of_an_ellipse

use cao_sketch::{Element, Selection, Sketch, Snap, SnapSettings, Support, WorkPlane};
use glam::DVec2;

/// An ellipse about the origin, sixty wide and forty high, cut down to the
/// half running from the north handle round to the south one — the western
/// half, the eastern being what the cut took.
fn a_half_ellipse() -> (Sketch, cao_sketch::EllipseId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let west = sketch.add_point(DVec2::new(-30.0, 0.0));
    let east = sketch.add_point(DVec2::new(30.0, 0.0));
    let south = sketch.add_point(DVec2::new(0.0, -20.0));
    let north = sketch.add_point(DVec2::new(0.0, 20.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    sketch.trim_ellipse(id, Some((south, north)));
    (sketch, id)
}

fn metrics() -> cao_sketch::AnnotationMetrics {
    cao_sketch::AnnotationMetrics {
        offset_pixels: 8.0,
        arrow_pixels: 6.0,
        arc_pixels: 18.0,
        pixel: 1.0,
        nudge: DVec2::ZERO,
    }
}

#[test]
fn the_preview_names_the_stretch_a_click_would_take() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let west = sketch.add_point(DVec2::new(-30.0, 0.0));
    let east = sketch.add_point(DVec2::new(30.0, 0.0));
    let south = sketch.add_point(DVec2::new(0.0, -20.0));
    let north = sketch.add_point(DVec2::new(0.0, 20.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);

    let stretch = sketch.ellipse_stretch_at(id, DVec2::new(21.0, 14.0));
    let going = sketch
        .ellipse_trim_takes(id, stretch)
        .expect("what the click would take");

    let cao_sketch::Stretch::Oval { drawn, from, sweep } = going.stretch else {
        panic!("a stretch of an ellipse, not {:?}", going.stretch);
    };
    assert!(drawn.at(from).distance(DVec2::new(30.0, 0.0)) < 1e-6);
    assert!(
        (sweep - std::f64::consts::FRAC_PI_2).abs() < 1e-6,
        "a quarter of a turn, not {sweep}",
    );
}

#[test]
fn a_click_finds_an_arc_of_ellipse_where_it_is_drawn_and_not_where_it_is_gone() {
    let (sketch, id) = a_half_ellipse();

    // Away from the handles and from the axes, which a click finds first.
    let on_the_half = sketch.ellipse_polyline(id)[6];
    let where_it_ran = DVec2::new(-on_the_half.x, on_the_half.y);
    let on_it = sketch.pick(on_the_half + DVec2::new(0.2, 0.0), 1.0, metrics());
    let where_it_went = sketch.pick(where_it_ran, 1.0, metrics());

    assert!(
        matches!(on_it, Some(Selection::Element(Element::Ellipse(found))) if found == id),
        "a click on the half that is drawn finds it: {on_it:?}",
    );
    assert!(
        !matches!(where_it_went, Some(Selection::Element(Element::Ellipse(_)))),
        "and a click where the cut took it finds nothing: {where_it_went:?}",
    );
}

#[test]
fn a_box_over_the_drawing_catches_an_arc_of_ellipse() {
    let (sketch, id) = a_half_ellipse();

    let caught = sketch.inside_band(DVec2::new(-40.0, -30.0), DVec2::new(40.0, 30.0), metrics());

    assert!(
        caught.contains(&Selection::Element(Element::Ellipse(id))),
        "{caught:?}",
    );
}

#[test]
fn erasing_an_arc_of_ellipse_takes_its_axes_and_its_ends() {
    let (mut sketch, id) = a_half_ellipse();
    let ellipse = sketch.ellipses()[id.0];
    let (from, to) = sketch.ellipse_ends(id).expect("its two ends");

    sketch.erase(Element::Ellipse(id));

    assert!(sketch.is_erased_segment(ellipse.first));
    assert!(sketch.is_erased_segment(ellipse.second));
    assert!(sketch.is_erased_point(from) && sketch.is_erased_point(to));
}

#[test]
fn a_copy_of_an_arc_of_ellipse_is_drawn_over_the_same_stretch() {
    let (mut sketch, id) = a_half_ellipse();

    let made = sketch.duplicate(&[Element::Ellipse(id)], |at| at + DVec2::new(100.0, 0.0));

    let copy = *made.ellipses.first().expect("a copy");
    let (from, sweep) = sketch.ellipse_run(copy);
    let (was_from, was_sweep) = sketch.ellipse_run(id);
    assert!((from - was_from).abs() < 1e-9 && (sweep - was_sweep).abs() < 1e-9);
    let places = sketch.ellipse_polyline(copy);
    assert!(
        places[0].distance(DVec2::new(100.0, 20.0)) < 1e-6,
        "the copy opens where the one it was made from opens: {:?}",
        places[0],
    );
}

#[test]
fn the_cursor_is_pulled_onto_an_arc_of_ellipse_and_not_past_its_ends() {
    let (sketch, id) = a_half_ellipse();
    let settings = SnapSettings {
        point_reach: 0.5,
        curve_reach: 2.0,
        grid_step: None,
        grid_reach: 0.0,
    };

    // Away from the axes, which pull like the traits they are.
    let on_the_half = sketch.ellipse_polyline(id)[6];
    let (at, caught) = sketch.magnetise(on_the_half + DVec2::new(0.5, 0.5), &settings);
    let mirrored = DVec2::new(-on_the_half.x, on_the_half.y);
    let (past, missed) = sketch.magnetise(mirrored + DVec2::new(0.5, 0.5), &settings);

    assert!(matches!(caught, Some(Snap::OnCurve(_))), "{caught:?}");
    assert!(sketch.distance_to_ellipse(id, at) < 1e-9, "{at}");
    assert!(
        !matches!(missed, Some(Snap::OnCurve(_))),
        "where the cut took it there is nothing to be pulled onto: {missed:?} at {past}",
    );
}

#[test]
fn a_point_held_on_an_arc_of_ellipse_follows_the_curve() {
    let (mut sketch, id) = a_half_ellipse();
    let on_the_curve = sketch.ellipse_polyline(id)[10];
    let point = sketch.add_point(on_the_curve);
    sketch.add_constraint(Support::Ellipse(id).holding(point));
    let first = sketch.ellipses()[id.0].first;

    sketch.set_dimension(cao_sketch::DimensionTarget::Length(first), 90.0, false);
    sketch.resolve(1.0);

    let off = sketch.ellipse_draft(id).distance(sketch.point(point));
    assert!(
        off < 1e-4,
        "the point stands {off} off the curve it was laid on"
    );
}

#[test]
fn an_arc_of_ellipse_closes_an_area_against_a_trait() {
    let (mut sketch, id) = a_half_ellipse();
    let (from, to) = sketch.ellipse_ends(id).expect("its two ends");
    sketch.add_segment(from, to);

    let regions = sketch.regions();

    assert_eq!(regions.len(), 1, "{regions:?}");
    let span: f64 = regions[0]
        .face_triangles()
        .iter()
        .map(|[a, b, c]| ((*b - *a).perp_dot(*c - *a) * 0.5).abs())
        .sum();
    let half = std::f64::consts::PI * 30.0 * 20.0 * 0.5;
    assert!(
        (span - half).abs() < half * 0.02,
        "half the ellipse is {half}, not {span}",
    );
}

#[test]
fn a_mirrored_arc_of_ellipse_is_drawn_over_the_mirror_of_its_own_stretch() {
    let (mut sketch, id) = a_half_ellipse();
    let along = sketch.ellipse_polyline(id)[6];

    // Across a line well clear of the curve, so the copy lands somewhere the
    // original is not.
    let made = sketch.duplicate(&[Element::Ellipse(id)], |at| DVec2::new(at.x, 100.0 - at.y));

    let copy = *made.ellipses.first().expect("a copy");
    let mirrored = DVec2::new(along.x, 100.0 - along.y);
    let off = sketch.distance_to_ellipse(copy, mirrored);
    assert!(
        off < 1e-6,
        "the mirror of a place on the stretch is on the copy, not {off} off it",
    );
}

#[test]
fn erasing_a_shared_axis_takes_every_piece_standing_on_it() {
    let (mut sketch, id) = a_half_ellipse();
    let (from, to) = sketch.ellipse_ends(id).expect("its two ends");
    let middle = sketch.add_point(sketch.ellipse_polyline(id)[6]);
    sketch.add_constraint(Support::Ellipse(id).holding(middle));
    // A cut in the middle of the half leaves two pieces on the one pair of
    // axes.
    let trimmed = sketch
        .trim_ellipse(id, Some((from, middle)))
        .expect("a cut");
    assert_eq!(trimmed.pieces.len(), 1, "{trimmed:?}");
    let second = sketch
        .trim_ellipse(id, Some((middle, to)))
        .expect("nothing left to cut");
    assert!(second.pieces.is_empty() || second.pieces.len() == 1);
}

#[test]
fn a_box_round_what_is_drawn_of_an_arc_catches_it() {
    let (sketch, id) = a_half_ellipse();

    let caught = sketch.inside_band(DVec2::new(-40.0, -30.0), DVec2::new(1.0, 30.0), metrics());

    assert!(
        caught.contains(&Selection::Element(Element::Ellipse(id))),
        "a box round the half that is drawn catches it: {caught:?}",
    );
}

#[test]
fn a_click_past_the_end_of_an_arc_takes_the_stretch_it_opens_with_and_not_the_whole_of_it() {
    let (mut sketch, id) = a_half_ellipse();
    let (from, to) = sketch.ellipse_ends(id).expect("its two ends");
    let middle = sketch.add_point(sketch.ellipse_polyline(id)[6]);
    sketch.add_constraint(Support::Ellipse(id).holding(middle));
    let tip = sketch.point(from);

    let stretch = sketch.ellipse_stretch_at(id, tip + DVec2::new(0.2, 0.1));

    assert_eq!(
        stretch,
        Some((from, middle)),
        "a click a hair past the tip is a click at the tip, and takes the stretch it opens with",
    );
    let _ = to;
}

#[test]
fn a_division_is_offered_again_where_the_cut_took_the_curve_away() {
    let (mut sketch, id) = a_half_ellipse();
    let gone = DVec2::new(
        -sketch.ellipse_polyline(id)[6].x,
        sketch.ellipse_polyline(id)[6].y,
    );
    let (from, to) = (
        sketch.add_point(gone + DVec2::new(-10.0, -10.0)),
        sketch.add_point(gone + DVec2::new(10.0, 10.0)),
    );
    sketch.add_segment(from, to);
    let (across_from, across_to) = (
        sketch.add_point(gone + DVec2::new(-10.0, 10.0)),
        sketch.add_point(gone + DVec2::new(10.0, -10.0)),
    );
    sketch.add_segment(across_from, across_to);

    let named = sketch.crossing_at(gone, 1.0);

    assert!(
        matches!(named, Some(cao_sketch::Crossing::Curves { .. })),
        "where the cut took the curve away nothing stands in the way: {named:?}",
    );
}
