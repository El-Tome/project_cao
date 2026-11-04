//! What sketch · corner_cut/picking.rs is held to.

use glam::DVec2;

use crate::WorkPlane;
use crate::annotation::AnnotationMetrics;
use crate::corner_cut::{CornerCutId, CutShape, Grip};
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
fn a_press_on_the_curve_of_a_fillet_drags_its_corner_from_afar() {
    let (sketch, corner, _) = a_rounded_right_angle();

    assert_eq!(
        sketch.point_to_drag_at(on_the_curve(), 0.5),
        Some(Grip {
            point: corner,
            from_afar: true
        })
    );
}

#[test]
fn a_press_on_a_point_of_a_fillet_drags_its_corner_from_afar() {
    let (sketch, corner, _) = a_rounded_right_angle();

    for place in [
        DVec2::new(20.0, 20.0),
        DVec2::new(20.0, 10.0),
        DVec2::new(10.0, 20.0),
    ] {
        assert_eq!(
            sketch.point_to_drag_at(place, 0.5),
            Some(Grip {
                point: corner,
                from_afar: true
            }),
            "at {place}"
        );
    }
}

#[test]
fn a_press_on_the_corner_itself_drags_it_from_under_the_hand() {
    let (sketch, corner, _) = a_rounded_right_angle();

    assert_eq!(
        sketch.point_to_drag_at(DVec2::new(10.0, 10.0), 0.5),
        Some(Grip {
            point: corner,
            from_afar: false
        })
    );
}

#[test]
fn a_press_on_a_point_hung_on_a_fillet_drags_its_corner_from_afar_too() {
    let (mut sketch, corner, _) = a_rounded_right_angle();
    let hung = sketch.add_point(DVec2::new(20.0, 20.0));
    sketch.add_constraint(crate::Constraint::AtCutPoint {
        point: hung,
        cut: CornerCutId(0),
        at: crate::CutPoint::Centre,
    });

    assert_eq!(
        sketch.point_to_drag_at(DVec2::new(20.0, 20.0), 0.5),
        Some(Grip {
            point: corner,
            from_afar: true
        })
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

/// A trait from (10, 10) to (50, 10) and an arc round (2, 10) of radius 8
/// from that corner up to (2, 18), the corner rounded by five: the fillet runs
/// round (14, 15) from (14, 10) on the trait to where it touches the arc.
fn a_trait_and_an_arc_rounded() -> (Sketch, PointId, crate::arc::ArcId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let far = sketch.add_point(DVec2::new(50.0, 10.0));
    let along = sketch.add_segment(corner, far);
    let centre = sketch.add_point(DVec2::new(2.0, 10.0));
    let top = sketch.add_point(DVec2::new(2.0, 18.0));
    let arc = sketch.add_arc(centre, corner, top);
    sketch
        .cut_corner(along, arc, CutShape::Fillet { radius: 5.0 }, false, 0)
        .expect("the trait and the arc take a fillet");
    (sketch, corner, arc)
}

/// Where the fillet beside the arc touches it.
const ON_THE_ARC: DVec2 = DVec2::new(2.0 + 96.0 / 13.0, 10.0 + 40.0 / 13.0);

#[test]
fn a_press_on_a_fillet_beside_an_arc_drags_its_corner() {
    let (sketch, corner, _) = a_trait_and_an_arc_rounded();
    let centre = DVec2::new(14.0, 15.0);
    let halfway = ((DVec2::new(14.0, 10.0) - centre).normalize()
        + (ON_THE_ARC - centre).normalize())
    .normalize();

    for place in [
        centre + halfway * 5.0,
        centre,
        DVec2::new(14.0, 10.0),
        ON_THE_ARC,
    ] {
        assert_eq!(
            sketch.point_to_drag_at(place, 0.5),
            Some(Grip {
                point: corner,
                from_afar: true
            }),
            "a press at {place} on the fillet takes its corner"
        );
    }
}

#[test]
fn a_press_on_an_arc_beside_where_its_fillet_meets_it_takes_the_arc() {
    let (sketch, _, arc) = a_trait_and_an_arc_rounded();
    let past_the_fillet = DVec2::new(2.0, 10.0) + DVec2::from_angle(25_f64.to_radians()) * 8.05;

    let picked = sketch.pick(past_the_fillet, 0.5, METRICS);

    assert_eq!(
        picked,
        Some(Selection::Element(crate::element::Element::Arc(arc))),
        "a press on the arc a third of a unit past where the fillet meets it takes the arc, \
         as one on a trait past its fillet takes the trait"
    );
}

#[test]
fn a_press_on_an_arc_of_ellipse_beside_where_its_fillet_meets_it_takes_the_ellipse() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(46.0, 34.0));
    let up = sketch.add_point(DVec2::new(46.0, 54.0));
    let rising = sketch.add_segment(corner, up);
    let [centre, back, front, low, high] = [
        (40.0, 30.0),
        (30.0, 30.0),
        (50.0, 30.0),
        (40.0, 25.0),
        (40.0, 35.0),
    ]
    .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let ellipse = sketch.add_ellipse(centre, [back, front], [low, high]);
    sketch.draw_the_stretch(ellipse, corner, back);
    sketch
        .cut_corner(rising, ellipse, CutShape::Fillet { radius: 3.0 }, false, 0)
        .expect("the trait and the arc of ellipse take a fillet");
    let turn = 77_f64.to_radians();
    let past_the_fillet = DVec2::new(40.0 + 10.0 * turn.cos(), 30.0 + 5.0 * turn.sin());

    let picked = sketch.pick(past_the_fillet, 0.5, METRICS);

    assert_eq!(
        picked,
        Some(Selection::Element(crate::element::Element::Ellipse(
            ellipse
        ))),
        "a press on the arc of ellipse a third of a unit past where the fillet meets it takes \
         the ellipse, as one on a trait past its fillet takes the trait"
    );
}
