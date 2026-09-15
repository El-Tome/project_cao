use glam::DVec2;

use super::*;
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
fn a_chamfer_leaves_no_point_standing_where_the_corner_was() {
    let (mut sketch, along, up, pivot) = a_right_angle();

    sketch
        .chamfer(along, up, Chamfer::Equal(3.0))
        .expect("a corner that can be cut");

    assert!(
        sketch.is_erased_point(pivot),
        "the corner is cut off, and a point left hanging where it was belongs to nothing"
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
