#[cfg(test)]
use crate::constraints::DimensionTarget;
#[cfg(test)]
use crate::plane::WorkPlane;

use super::*;

fn two_traits_crossing() -> (Sketch, [SegmentId; 2], DVec2) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(0.0, 0.0));
    let east = sketch.add_point(DVec2::new(10.0, 0.0));
    let south = sketch.add_point(DVec2::new(5.0, -5.0));
    let north = sketch.add_point(DVec2::new(5.0, 5.0));
    let across = sketch.add_segment(west, east);
    let up = sketch.add_segment(south, north);
    (sketch, [across, up], DVec2::new(5.0, 0.0))
}

#[test]
fn splitting_a_crossing_leaves_four_pieces_meeting_at_one_point() {
    let (mut sketch, [across, up], crossing) = two_traits_crossing();

    let split = sketch
        .split(&[across, up], crossing)
        .expect("a crossing that can be split");

    assert!(
        sketch.is_erased_segment(across) && sketch.is_erased_segment(up),
        "both traits the click fell on are gone, replaced by their pieces"
    );
    assert_eq!(split.pieces.len(), 4, "two pieces out of each trait");
    for piece in &split.pieces {
        let piece = sketch.segments()[piece.0];
        assert!(
            piece.start == split.point || piece.end == split.point,
            "every piece stops at the point dropped at the crossing"
        );
    }
}

#[test]
fn a_division_that_cannot_be_made_leaves_the_drawing_as_it_was() {
    let (mut sketch, [across, up], _) = two_traits_crossing();
    let before = sketch.clone();

    let refused = sketch.split(&[across, up], DVec2::new(5.0, 3.0));

    assert_eq!(
        refused, None,
        "the place named is on one trait, not on both"
    );
    assert_eq!(
        sketch.points().len(),
        before.points().len(),
        "a refused division drops no point"
    );
    assert!(
        !sketch.is_erased_segment(across) && !sketch.is_erased_segment(up),
        "a refused division cuts neither trait"
    );
}

#[test]
fn a_division_that_fails_on_its_second_trait_undoes_the_first() {
    let (mut sketch, [across, _], crossing) = two_traits_crossing();
    let before = sketch.clone();

    let refused = sketch.split(&[across, across], crossing);

    assert_eq!(refused, None, "the trait was already cut by the first pass");
    assert!(
        !sketch.is_erased_segment(across),
        "a drawing divided halfway is worse than one not divided at all"
    );
    assert_eq!(sketch.points().len(), before.points().len());
    assert_eq!(sketch.live_segments().count(), 2);
}

#[test]
fn a_crossing_a_point_already_stands_on_is_not_divided_again() {
    let (mut sketch, [across, up], crossing) = two_traits_crossing();
    sketch.add_point(crossing);
    let points = sketch.points().len();

    let refused = sketch.split(&[across, up], crossing);

    assert_eq!(refused, None, "there is nothing left to drop there");
    assert_eq!(
        sketch.points().len(),
        points,
        "a second point on the same place would be one the user cannot tell from the first"
    );
}

#[test]
fn a_length_measured_over_the_whole_trait_does_not_survive_its_division() {
    let (mut sketch, [across, up], crossing) = two_traits_crossing();
    sketch.set_dimension(DimensionTarget::Length(across), 10.0, false);

    sketch
        .split(&[across, up], crossing)
        .expect("a crossing that can be split");

    let measured: Vec<DimensionTarget> = sketch
        .dimensions()
        .iter()
        .map(|value| value.target)
        .collect();
    assert!(
        !measured.contains(&DimensionTarget::Length(across)),
        "the trait it measured is gone, and it measured neither piece: {measured:?}"
    );
}

#[test]
fn the_traits_running_through_a_crossing_are_the_ones_a_division_cuts() {
    let (sketch, [across, up], crossing) = two_traits_crossing();

    let found = sketch.crossing_at(crossing + DVec2::new(0.2, 0.1), 1.0);

    assert_eq!(
        found,
        Some(Crossing::Traits {
            at: crossing,
            segments: vec![across, up],
        }),
        "a click near the crossing names the place and both traits through it"
    );
}

#[test]
fn a_click_nowhere_near_a_crossing_names_none() {
    let (sketch, _, crossing) = two_traits_crossing();

    assert_eq!(
        sketch.crossing_at(crossing + DVec2::new(3.0, 3.0), 1.0),
        None
    );
}

#[test]
fn a_crossing_a_curve_runs_through_is_refused_rather_than_half_divided() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(0.0, 0.0));
    sketch.add_circle(centre, 5.0);
    let west = sketch.add_point(DVec2::new(-10.0, 0.0));
    let east = sketch.add_point(DVec2::new(10.0, 0.0));
    sketch.add_segment(west, east);

    let found = sketch.crossing_at(DVec2::new(5.0, 0.0), 1.0);

    assert_eq!(
        found,
        Some(Crossing::Curved),
        "the trait could be cut there, but the circle it crosses could not"
    );
}

#[test]
fn two_traits_crossing_where_a_curve_also_runs_are_left_alone_together() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(0.0, 0.0));
    sketch.add_circle(centre, 5.0);
    let west = sketch.add_point(DVec2::new(-10.0, 0.0));
    let east = sketch.add_point(DVec2::new(10.0, 0.0));
    sketch.add_segment(west, east);
    let below = sketch.add_point(DVec2::new(5.0, -3.0));
    let above = sketch.add_point(DVec2::new(5.0, 3.0));
    sketch.add_segment(below, above);

    let found = sketch.crossing_at(DVec2::new(5.0, 0.0), 0.5);

    assert_eq!(
        found,
        Some(Crossing::Curved),
        "cutting the two traits and leaving the circle whole is the half-division nobody asked for"
    );
}

#[test]
fn a_drawing_far_from_the_origin_is_divisible_too() {
    let far = 1.0e7;
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(far - 5.0, 0.0));
    let east = sketch.add_point(DVec2::new(far + 5.0, 0.0));
    let across = sketch.add_segment(west, east);
    let below = sketch.add_point(DVec2::new(far, -5.0));
    let above = sketch.add_point(DVec2::new(far, 5.0));
    let up = sketch.add_segment(below, above);

    let found = sketch.crossing_at(DVec2::new(far, 0.0), 1.0);

    assert_eq!(
        found,
        Some(Crossing::Traits {
            at: DVec2::new(far, 0.0),
            segments: vec![across, up],
        }),
    );
}

#[test]
fn a_division_says_what_it_cost() {
    let (mut sketch, [across, up], crossing) = two_traits_crossing();
    sketch.set_dimension(DimensionTarget::Length(across), 10.0, false);
    sketch.set_dimension(DimensionTarget::Length(up), 10.0, false);

    let split = sketch
        .split(&[across, up], crossing)
        .expect("a crossing that can be split");

    assert_eq!(
        split.values_dropped, 2,
        "one length measured over each of the two traits divided"
    );
}
