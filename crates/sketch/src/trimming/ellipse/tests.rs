//! What sketch · trimming/ellipse.rs is held to.

use super::*;
use crate::plane::WorkPlane;

/// An ellipse about the origin, sixty wide and forty high, with the four
/// points its axes give it — east, north, west and south, in the order the
/// curve runs.
fn an_ellipse() -> (Sketch, EllipseId, [PointId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let west = sketch.add_point(DVec2::new(-30.0, 0.0));
    let east = sketch.add_point(DVec2::new(30.0, 0.0));
    let south = sketch.add_point(DVec2::new(0.0, -20.0));
    let north = sketch.add_point(DVec2::new(0.0, 20.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    (sketch, id, [east, north, west, south])
}

/// The places the drawn stretch of an ellipse runs through, for reading what a
/// cut left.
fn drawn_through(sketch: &Sketch, id: EllipseId) -> (DVec2, DVec2) {
    let places = sketch.ellipse_polyline(id);
    (places[0], places[places.len() - 1])
}

#[test]
fn a_click_between_two_points_takes_the_stretch_it_fell_in() {
    let (mut sketch, id, [east, north, ..]) = an_ellipse();

    let stretch = sketch.ellipse_stretch_at(id, DVec2::new(21.0, 14.0));

    assert_eq!(
        stretch,
        Some((east, north)),
        "the quarter the click fell in"
    );
    let trimmed = sketch.trim_ellipse(id, stretch).expect("a cut");
    assert_eq!(
        trimmed.pieces,
        vec![id],
        "the ellipse is what is left of it"
    );
    let (from, to) = drawn_through(&sketch, id);
    assert!(from.distance(DVec2::new(0.0, 20.0)) < 1e-6, "{from}");
    assert!(to.distance(DVec2::new(30.0, 0.0)) < 1e-6, "{to}");
}

#[test]
fn a_cut_leaves_the_three_quarters_the_click_did_not_fall_in() {
    let (mut sketch, id, _) = an_ellipse();
    let stretch = sketch.ellipse_stretch_at(id, DVec2::new(21.0, 14.0));

    sketch.trim_ellipse(id, stretch);

    let places = sketch.ellipse_polyline(id);
    let along: f64 = places
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum();
    let whole: f64 = sketch
        .ellipse_draft(id)
        .places()
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .sum();
    assert!(
        (along - whole * 0.75).abs() < whole * 0.02,
        "three quarters of {whole} is not {along}",
    );
}

#[test]
fn a_curve_with_nothing_on_it_goes_whole() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let west = sketch.add_point(DVec2::new(-30.0, 0.0));
    let east = sketch.add_point(DVec2::new(30.0, 0.0));
    let south = sketch.add_point(DVec2::new(0.0, -20.0));
    let north = sketch.add_point(DVec2::new(0.0, 20.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    for point in [west, east, south, north] {
        sketch.erase(Element::Point(point));
    }

    assert!(sketch.is_erased_ellipse(id), "its axes went with its ends");
}

#[test]
fn a_stretch_already_cut_is_cut_again_at_its_own_end() {
    let (mut sketch, id, [east, north, west, _]) = an_ellipse();
    sketch.trim_ellipse(id, Some((east, north)));

    // What is left runs from the north round to the east; the cut takes the
    // quarter it opens with.
    let again = sketch.ellipse_stretch_at(id, DVec2::new(-21.0, 14.0));

    assert_eq!(again, Some((north, west)));
    let trimmed = sketch.trim_ellipse(id, again).expect("a second cut");
    assert_eq!(trimmed.pieces, vec![id]);
    let (from, to) = drawn_through(&sketch, id);
    assert!(from.distance(DVec2::new(-30.0, 0.0)) < 1e-6, "{from}");
    assert!(to.distance(DVec2::new(30.0, 0.0)) < 1e-6, "{to}");
}

#[test]
fn a_cut_in_the_middle_of_a_stretch_leaves_two_pieces_of_the_one_curve() {
    let (mut sketch, id, [east, north, west, south]) = an_ellipse();
    // Left drawn: from the north, the long way round, to the east.
    sketch.trim_ellipse(id, Some((east, north)));
    let axes = sketch.ellipses()[id.0];

    let trimmed = sketch
        .trim_ellipse(id, Some((west, south)))
        .expect("a cut in the middle");

    assert_eq!(trimmed.pieces.len(), 2, "{trimmed:?}");
    let (second, other) = (trimmed.pieces[0], trimmed.pieces[1]);
    assert_eq!(second, id, "the ellipse is the first of the two");
    for piece in [second, other] {
        let held = sketch.ellipses()[piece.0];
        assert_eq!(
            (held.first, held.second, held.center),
            (axes.first, axes.second, axes.center),
            "both pieces stand on the very same axes",
        );
    }
    assert_eq!(sketch.ellipse_ends(second), Some((north, west)));
    assert_eq!(sketch.ellipse_ends(other), Some((south, east)));
}

#[test]
fn the_axes_go_with_the_last_piece_of_the_curve_and_not_the_first() {
    let (mut sketch, id, [east, north, west, south]) = an_ellipse();
    sketch.trim_ellipse(id, Some((east, north)));
    let trimmed = sketch
        .trim_ellipse(id, Some((west, south)))
        .expect("a cut in the middle");
    let (first, second) = (trimmed.pieces[0], trimmed.pieces[1]);
    let axes = sketch.ellipses()[first.0];

    sketch.erase(Element::Ellipse(first));

    assert!(!sketch.is_erased_ellipse(second), "the other piece stays");
    assert!(
        !sketch.is_erased_segment(axes.first) && !sketch.is_erased_segment(axes.second),
        "and the axes it stands on stay with it",
    );

    sketch.erase(Element::Ellipse(second));

    assert!(
        sketch.is_erased_segment(axes.first) && sketch.is_erased_segment(axes.second),
        "the axes go with the last piece",
    );
}
