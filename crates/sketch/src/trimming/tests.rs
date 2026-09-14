#[cfg(test)]
use crate::plane::WorkPlane;

use super::*;

const NEAR: f64 = 1e-6;

#[test]
fn the_points_sitting_on_a_trait_come_in_order_from_its_start() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 1.0));
    let end = sketch.add_point(DVec2::new(10.0, 1.0));
    let segment = sketch.add_segment(start, end);

    let far = sketch.add_point(DVec2::new(7.0, 1.0));
    let near = sketch.add_point(DVec2::new(3.0, 1.0));
    sketch.add_point(DVec2::new(5.0, 4.0));
    sketch.add_point(DVec2::new(14.0, 1.0));

    let sitting: Vec<PointId> = sketch
        .sitting_along(segment, NEAR)
        .into_iter()
        .map(|(_, id)| id)
        .collect();
    assert_eq!(sitting, vec![start, near, far, end]);
}

fn a_trait_with_two_points_on_it() -> (Sketch, SegmentId, [PointId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 1.0));
    let end = sketch.add_point(DVec2::new(10.0, 1.0));
    let segment = sketch.add_segment(start, end);
    let near = sketch.add_point(DVec2::new(3.0, 1.0));
    let far = sketch.add_point(DVec2::new(7.0, 1.0));
    (sketch, segment, [start, near, far, end])
}

#[test]
fn a_click_names_the_stretch_it_fell_in() {
    let (sketch, segment, [start, near, far, end]) = a_trait_with_two_points_on_it();

    for (at, expected) in [
        (DVec2::new(1.0, 1.0), (start, near)),
        (DVec2::new(5.0, 1.2), (near, far)),
        (DVec2::new(9.0, 1.0), (far, end)),
    ] {
        assert_eq!(
            sketch.stretch_at(segment, at, NEAR),
            Some(expected),
            "at {at}"
        );
    }
}

#[test]
fn trimming_a_stretch_leaves_the_rest_of_the_trait_standing() {
    let (mut sketch, segment, [start, near, far, end]) = a_trait_with_two_points_on_it();

    let kept = sketch
        .trim(segment, near, far)
        .expect("a cut that can be made");

    assert!(
        sketch.is_erased_segment(segment),
        "the trait it was cut from is gone"
    );
    assert_eq!(
        kept.len(),
        2,
        "an end on either side of the stretch taken out"
    );
    let ends: Vec<(PointId, PointId)> = kept
        .iter()
        .map(|piece| {
            let piece = sketch.segments()[piece.0];
            (piece.start, piece.end)
        })
        .collect();
    assert_eq!(ends, vec![(start, near), (far, end)]);
}

#[test]
fn trimming_a_trait_nothing_sits_on_takes_the_whole_trait() {
    let (mut sketch, segment, [start, _, _, end]) = a_trait_with_two_points_on_it();

    assert_eq!(sketch.trim(segment, start, end), Some(Vec::new()));
    assert!(sketch.is_erased_segment(segment));
}

#[test]
fn naming_one_point_twice_cuts_the_trait_there_and_takes_nothing() {
    let (mut sketch, segment, [start, near, _, end]) = a_trait_with_two_points_on_it();

    let pieces = sketch
        .trim(segment, near, near)
        .expect("a cut that can be made");

    let ends: Vec<(PointId, PointId)> = pieces
        .iter()
        .map(|piece| {
            let piece = sketch.segments()[piece.0];
            (piece.start, piece.end)
        })
        .collect();
    assert_eq!(ends, vec![(start, near), (near, end)]);
}

#[test]
fn a_trait_that_only_helps_build_the_drawing_still_only_helps_once_trimmed() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 1.0));
    let end = sketch.add_point(DVec2::new(10.0, 1.0));
    let segment = sketch.add_construction_segment(start, end);
    let middle = sketch.add_point(DVec2::new(5.0, 1.0));

    let pieces = sketch
        .trim(segment, middle, end)
        .expect("a cut that can be made");

    assert_eq!(pieces.len(), 1);
    assert!(sketch.segments()[pieces[0].0].construction);
}

#[test]
fn a_point_held_on_a_trait_is_held_on_the_piece_it_lands_on() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 1.0));
    let end = sketch.add_point(DVec2::new(10.0, 1.0));
    let segment = sketch.add_segment(start, end);
    let held = sketch.add_point(DVec2::new(2.0, 1.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: held,
        segment,
    });
    let from = sketch.add_point(DVec2::new(5.0, 1.0));
    let to = sketch.add_point(DVec2::new(8.0, 1.0));

    let pieces = sketch
        .trim(segment, from, to)
        .expect("a cut that can be made");

    assert!(
        sketch.constraints().contains(&Constraint::OnSegment {
            point: held,
            segment: pieces[0],
        }),
        "held on {:?}, and the rules standing are {:?}",
        pieces[0],
        sketch.constraints(),
    );
}

#[test]
fn a_trait_is_never_cut_at_a_point_the_drawing_does_not_have() {
    let (mut sketch, segment, [_, _, _, end]) = a_trait_with_two_points_on_it();

    assert_eq!(sketch.trim(segment, PointId(99), end), None);
    assert!(
        !sketch.is_erased_segment(segment),
        "a cut that cannot be made leaves the trait alone",
    );
}

#[test]
fn a_point_past_the_end_never_makes_the_trait_longer() {
    let (mut sketch, segment, [_, _, _, end]) = a_trait_with_two_points_on_it();
    let beyond = sketch.add_point(DVec2::new(-20.0, 1.0));

    assert_eq!(sketch.trim(segment, beyond, end), None);
    assert!(!sketch.is_erased_segment(segment));
}

#[test]
fn a_piece_of_no_length_is_never_left_behind() {
    let (mut sketch, segment, [start, near, _, end]) = a_trait_with_two_points_on_it();
    let twin = sketch.add_point(sketch.point(start));

    let pieces = sketch
        .trim(segment, twin, near)
        .expect("a cut that can be made");

    let ends: Vec<(PointId, PointId)> = pieces
        .iter()
        .map(|piece| {
            let piece = sketch.segments()[piece.0];
            (piece.start, piece.end)
        })
        .collect();
    assert_eq!(ends, vec![(near, end)]);
}

#[test]
fn cutting_a_trait_at_one_of_its_own_ends_hands_nothing_back() {
    let (mut sketch, segment, [start, _, _, end]) = a_trait_with_two_points_on_it();

    for at in [start, end] {
        assert_eq!(sketch.trim(segment, at, at), None, "cut at {at:?}");
    }
    assert!(!sketch.is_erased_segment(segment));
}

#[test]
fn a_trait_cut_where_a_circle_brushes_it_still_stands_on_that_place() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(2.0, 4.0));
    let end = sketch.add_point(DVec2::new(12.0, 4.0));
    let segment = sketch.add_segment(start, end);
    let centre = sketch.add_point(DVec2::new(7.0, 7.0));
    let circle = sketch.add_circle(centre, 3.0);
    sketch.add_tangency(circle, segment);
    sketch.resolve(1.0);
    let contact = sketch
        .constraints()
        .iter()
        .find_map(|rule| match rule {
            Constraint::Tangent {
                at: Some(point), ..
            } => Some(*point),
            _ => None,
        })
        .expect("a tangency keeps a point where the two touch");

    let pieces = sketch
        .trim(segment, start, contact)
        .expect("a cut that can be made");

    assert!(!sketch.is_erased_point(contact), "the piece stands on it");
    assert_eq!(sketch.segments()[pieces[0].0].start, contact);
    assert!(
        sketch
            .stretch_at(pieces[0], DVec2::new(10.0, 4.0), 1e-3)
            .is_some(),
        "and the piece can be cut again",
    );
}
