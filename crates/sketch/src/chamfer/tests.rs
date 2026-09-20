use glam::DVec2;

use super::*;
use crate::constraints::{Constraint, DimensionTarget};
use crate::plane::WorkPlane;
use crate::sketch::PointId;

const TOLERANCE: f64 = 1e-9;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);

/// A right angle standing clear of the sketch's own origin, one side running
/// east and the other north, ten units each.
fn a_right_angle() -> (Sketch, SegmentId, SegmentId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(CORNER);
    let east = sketch.add_point(CORNER + DVec2::new(10.0, 0.0));
    let north = sketch.add_point(CORNER + DVec2::new(0.0, 10.0));
    let along = sketch.add_segment(pivot, east);
    let up = sketch.add_segment(pivot, north);
    (sketch, along, up, pivot)
}

fn cut_ends(sketch: &Sketch, chamfered: &Chamfered) -> (DVec2, DVec2) {
    sketch.endpoints(chamfered.cut)
}

#[test]
fn an_equal_chamfer_takes_the_same_length_off_both_sides_of_the_corner() {
    let (mut sketch, along, up, _pivot) = a_right_angle();

    let chamfered = sketch
        .chamfer(along, up, Chamfer::Equal(3.0))
        .expect("a corner that can be cut");

    let (from, to) = cut_ends(&sketch, &chamfered);
    let near = |place: DVec2, wanted: DVec2| place.distance(wanted) <= TOLERANCE;
    assert!(
        (near(from, CORNER + DVec2::new(3.0, 0.0)) && near(to, CORNER + DVec2::new(0.0, 3.0)))
            || (near(from, CORNER + DVec2::new(0.0, 3.0))
                && near(to, CORNER + DVec2::new(3.0, 0.0))),
        "the cut runs between the two points three units out along each side, got {from:?} to {to:?}"
    );
    assert_eq!(chamfered.pieces.len(), 2, "what is left of each side");
}

#[test]
fn a_chamfer_leaves_the_sides_stopping_where_the_cut_starts() {
    let (mut sketch, along, up, _pivot) = a_right_angle();

    let chamfered = sketch
        .chamfer(along, up, Chamfer::Equal(3.0))
        .expect("a corner that can be cut");

    assert!(
        sketch.is_erased_segment(along) && sketch.is_erased_segment(up),
        "both sides are gone, replaced by what is left of them"
    );
    let mut reaches: Vec<f64> = chamfered
        .pieces
        .iter()
        .map(|piece| {
            let (from, to) = sketch.endpoints(*piece);
            from.distance(to)
        })
        .collect();
    reaches.sort_by(f64::total_cmp);
    for reach in reaches {
        assert!(
            (reach - 7.0).abs() <= TOLERANCE,
            "each side kept its far seven units, got {reach}"
        );
    }
}

#[test]
fn a_chamfer_of_two_distances_takes_what_each_side_was_given() {
    let (mut sketch, along, up, _pivot) = a_right_angle();

    let chamfered = sketch
        .chamfer(
            along,
            up,
            Chamfer::Sided {
                first: 2.0,
                second: 6.0,
            },
        )
        .expect("a corner that can be cut");

    let (from, to) = cut_ends(&sketch, &chamfered);
    let ends = [from, to];
    assert!(
        ends.iter()
            .any(|end| end.distance(CORNER + DVec2::new(2.0, 0.0)) <= TOLERANCE),
        "two units out along the side named first, got {ends:?}"
    );
    assert!(
        ends.iter()
            .any(|end| end.distance(CORNER + DVec2::new(0.0, 6.0)) <= TOLERANCE),
        "six units out along the other, got {ends:?}"
    );
}

#[test]
fn a_chamfer_given_an_angle_leaves_the_cut_at_that_angle_with_its_first_side() {
    let (mut sketch, along, up, _pivot) = a_right_angle();

    let chamfered = sketch
        .chamfer(
            along,
            up,
            Chamfer::Angled {
                along: 3.0,
                degrees: 60.0,
            },
        )
        .expect("a corner that can be cut");

    let (from, to) = cut_ends(&sketch, &chamfered);
    let (at, opposite) = match from.distance(CORNER + DVec2::new(3.0, 0.0)) <= TOLERANCE {
        true => (from, to),
        false => (to, from),
    };
    let towards_corner = (CORNER - at).normalize();
    let across = (opposite - at).normalize();
    let opening = towards_corner.angle_to(across).abs().to_degrees();
    assert!(
        (opening - 60.0).abs() <= 1e-6,
        "the cut leaves the side it was measured along at sixty degrees, got {opening}"
    );
}

#[test]
fn a_chamfer_reaching_past_the_end_of_a_side_is_refused() {
    let (mut sketch, along, up, _pivot) = a_right_angle();
    let before = sketch.clone();

    let refused = sketch.chamfer(along, up, Chamfer::Equal(12.0));

    assert_eq!(refused, None, "there are only ten units of side to take");
    assert!(
        !sketch.is_erased_segment(along) && !sketch.is_erased_segment(up),
        "a refused chamfer cuts neither side"
    );
    assert_eq!(sketch.points().len(), before.points().len());
}

#[test]
fn two_traits_that_share_no_corner_cannot_be_chamfered() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(0.0, 5.0));
    let d = sketch.add_point(DVec2::new(10.0, 5.0));
    let lower = sketch.add_segment(a, b);
    let upper = sketch.add_segment(c, d);

    assert_eq!(sketch.chamfer(lower, upper, Chamfer::Equal(1.0)), None);
}

#[test]
fn a_chamfer_of_nothing_at_all_is_refused() {
    let (mut sketch, along, up, _pivot) = a_right_angle();

    assert_eq!(sketch.chamfer(along, up, Chamfer::Equal(0.0)), None);
}

#[test]
fn a_chamfer_leaves_the_corner_behind_held_on_the_lines_of_both_sides() {
    let (mut sketch, along, up, pivot) = a_right_angle();

    let chamfered = sketch
        .chamfer(along, up, Chamfer::Equal(3.0))
        .expect("a corner that can be cut");

    assert!(
        !sketch.is_erased_point(pivot),
        "the corner point stands where the corner was, for the values to be measured from"
    );
    assert_eq!(chamfered.corner, pivot);
    let held: Vec<SegmentId> = sketch
        .constraints()
        .iter()
        .filter_map(|rule| match rule {
            Constraint::OnSegment { point, segment } if *point == pivot => Some(*segment),
            _ => None,
        })
        .collect();
    assert_eq!(
        held.len(),
        2,
        "one hold per side, so the corner stays at their crossing, got {held:?}"
    );
    assert!(
        held.iter()
            .all(|segment| chamfered.pieces.contains(segment)),
        "each hold names what is left of a side, got {held:?} of {:?}",
        chamfered.pieces
    );
}

#[test]
fn a_chamfer_lays_back_in_construction_the_stretch_it_took_off_each_side() {
    let (mut sketch, along, up, _pivot) = a_right_angle();

    let chamfered = sketch
        .chamfer(along, up, Chamfer::Equal(3.0))
        .expect("a corner that can be cut");

    for stretch in chamfered.stretches {
        assert!(
            sketch.segments()[stretch.0].construction,
            "it borders nothing, it only holds what the corner was worth"
        );
        let (from, to) = sketch.endpoints(stretch);
        assert!(
            from.distance(CORNER) <= TOLERANCE,
            "every stretch runs from the corner, got {from:?} to {to:?}"
        );
        assert!((sketch.segment_length(stretch) - 3.0).abs() <= TOLERANCE);
    }
}

#[test]
fn a_chamfer_keeps_the_angle_the_two_sides_stood_at() {
    let (mut sketch, along, up, _pivot) = a_right_angle();
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: along,
            second: up,
        }
        .normalised(),
        90.0,
        false,
    );

    let chamfered = sketch
        .chamfer(along, up, Chamfer::Equal(3.0))
        .expect("a corner that can be cut");

    let carried = sketch
        .dimension_of(
            DimensionTarget::Angle {
                first: chamfered.stretches[0],
                second: chamfered.stretches[1],
            }
            .normalised(),
        )
        .expect("the angle the corner stood at, now read between the stretches");
    assert!(
        (carried.value - 90.0).abs() <= TOLERANCE,
        "the cut took the corner, not what it was worth, got {}",
        carried.value
    );
}

#[test]
fn a_chamfer_rehangs_on_the_corner_the_length_each_side_was_given() {
    let (mut sketch, along, up, pivot) = a_right_angle();
    sketch.set_dimension(DimensionTarget::Length(along), 10.0, false);
    sketch.set_dimension(DimensionTarget::Length(up), 10.0, false);
    let far_east = sketch.segments()[along.0].end;

    sketch
        .chamfer(along, up, Chamfer::Equal(3.0))
        .expect("a corner that can be cut");

    let carried = sketch
        .dimension_of(
            DimensionTarget::Distance {
                from: pivot,
                to: far_east,
            }
            .normalised(),
        )
        .expect("the length the side was given, measured from the corner now");
    assert!(
        (carried.value - 10.0).abs() <= TOLERANCE,
        "the side is still ten long from the corner out, even though the cut \
         took three of it — got {}",
        carried.value
    );
    assert_eq!(
        sketch.dimensions().len(),
        2,
        "one per side, and neither was lost to the cut"
    );
}

#[test]
fn a_corner_a_third_trait_runs_into_keeps_its_point() {
    let (mut sketch, along, up, pivot) = a_right_angle();
    let away = sketch.add_point(CORNER + DVec2::new(-10.0, -10.0));
    let third = sketch.add_segment(pivot, away);

    sketch
        .chamfer(along, up, Chamfer::Equal(3.0))
        .expect("a corner that can be cut");

    assert!(
        !sketch.is_erased_point(pivot) && !sketch.is_erased_segment(third),
        "the third trait still runs into that point, so the point stays"
    );
}
