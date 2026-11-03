//! What sketch · corner_cut/picking.rs is held to.

use glam::DVec2;

use crate::WorkPlane;
use crate::annotation::AnnotationMetrics;
use crate::corner_cut::{CornerCutId, CutShape};
use crate::picking::Selection;
use crate::sketch::{PointId, SegmentId, Sketch};

const METRICS: AnnotationMetrics = AnnotationMetrics {
    offset_pixels: 30.0,
    arrow_pixels: 8.0,
    arc_pixels: 30.0,
    pixel: 0.1,
    nudge: DVec2::ZERO,
};

/// A right angle at (10, 10) rounded to a radius of ten: the curve runs from
/// (20, 10) to (10, 20) about (20, 20).
fn a_rounded_right_angle() -> (Sketch, PointId, [SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let right = sketch.add_point(DVec2::new(110.0, 10.0));
    let up = sketch.add_point(DVec2::new(10.0, 110.0));
    let sides = [
        sketch.add_segment(corner, right),
        sketch.add_segment(corner, up),
    ];
    sketch
        .cut_corner(
            sides[0],
            sides[1],
            CutShape::Fillet { radius: 10.0 },
            false,
            0,
        )
        .expect("a right angle takes a fillet");
    (sketch, corner, sides)
}

/// Halfway along the curve.
fn on_the_curve() -> DVec2 {
    DVec2::new(20.0, 20.0) - DVec2::new(1.0, 1.0).normalize() * 10.0
}

#[test]
fn a_press_on_the_curve_of_a_fillet_takes_the_fillet() {
    let (sketch, _, _) = a_rounded_right_angle();

    let picked = sketch.pick(on_the_curve(), 0.5, METRICS);

    assert_eq!(picked, Some(Selection::Cut(CornerCutId(0))));
}

#[test]
fn a_press_on_the_curve_of_a_fillet_drags_its_corner() {
    let (sketch, corner, _) = a_rounded_right_angle();

    assert_eq!(sketch.point_to_drag_at(on_the_curve(), 0.5), Some(corner));
}

#[test]
fn a_press_on_a_point_of_a_fillet_drags_its_corner() {
    let (sketch, corner, _) = a_rounded_right_angle();

    for place in [
        DVec2::new(20.0, 20.0),
        DVec2::new(20.0, 10.0),
        DVec2::new(10.0, 20.0),
    ] {
        assert_eq!(
            sketch.point_to_drag_at(place, 0.5),
            Some(corner),
            "at {place}"
        );
    }
}

#[test]
fn a_press_on_a_point_hung_on_a_fillet_drags_its_corner_too() {
    let (mut sketch, corner, _) = a_rounded_right_angle();
    let hung = sketch.add_point(DVec2::new(20.0, 20.0));
    sketch.add_constraint(crate::Constraint::AtCutPoint {
        point: hung,
        cut: CornerCutId(0),
        at: crate::CutPoint::Centre,
    });

    assert_eq!(
        sketch.point_to_drag_at(DVec2::new(20.0, 20.0), 0.5),
        Some(corner)
    );
}

#[test]
fn a_fillet_taken_carries_its_corner() {
    let (sketch, corner, _) = a_rounded_right_angle();

    assert_eq!(
        sketch.points_of(&[Selection::Cut(CornerCutId(0))]),
        vec![corner]
    );
}

#[test]
fn a_press_on_the_side_beyond_the_curve_still_takes_the_side() {
    let (sketch, _, [right, _]) = a_rounded_right_angle();

    let picked = sketch.pick(DVec2::new(60.0, 10.0), 0.5, METRICS);

    assert_eq!(
        picked,
        Some(Selection::Element(crate::element::Element::Segment(right)))
    );
}

fn reaching(reach: f64) -> crate::snap::SnapSettings {
    crate::snap::SnapSettings {
        point_reach: reach,
        curve_reach: reach,
        grid_step: None,
        grid_reach: 0.0,
    }
}

#[test]
fn the_centre_of_a_fillet_is_a_place_to_start_from() {
    let (sketch, _, _) = a_rounded_right_angle();

    let (at, snap) = sketch.magnetise(DVec2::new(20.2, 19.9), &reaching(0.5));

    assert_eq!(
        snap,
        Some(crate::snap::Snap::Centre(DVec2::new(20.0, 20.0)))
    );
    assert!(at.distance(DVec2::new(20.0, 20.0)) < 1e-12);
}

#[test]
fn the_ends_of_a_fillet_are_places_to_start_from() {
    let (sketch, _, _) = a_rounded_right_angle();

    let (at, _) = sketch.magnetise(DVec2::new(20.2, 10.1), &reaching(0.5));

    assert!(at.distance(DVec2::new(20.0, 10.0)) < 1e-9, "{at}");
}

#[test]
fn the_middle_of_a_fillet_is_a_place_to_start_from() {
    let (sketch, _, _) = a_rounded_right_angle();

    let (at, snap) = sketch.magnetise(on_the_curve() + DVec2::new(0.1, 0.0), &reaching(0.5));

    assert_eq!(snap, Some(crate::snap::Snap::Midpoint(at)));
    assert!(at.distance(on_the_curve()) < 1e-9, "{at}");
}

#[test]
fn a_press_on_a_side_beside_where_the_curve_meets_it_takes_the_side() {
    let (sketch, _, [right, _]) = a_rounded_right_angle();

    let picked = sketch.pick(DVec2::new(20.3, 10.05), 0.5, METRICS);

    assert_eq!(
        picked,
        Some(Selection::Element(crate::element::Element::Segment(right)))
    );
}
