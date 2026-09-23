//! What app · screens/viewport/input/ellipses.rs is held to.
//!
//! Closes #412.
//! - three clicks lay an ellipse, its two construction axes and its centre, as
//!   one step of the history — `three_clicks_lay_the_ellipse_its_axes_and_its_centre_as_one_step`
//! - typed values at the cursor set both widths, and turn the curve to the
//!   angle typed — `the_values_typed_while_placing_become_dimensions_on_the_axes`
//!
//! Closes #195.
//! - placed from its two ends, three clicks lay half a curve as one step of
//!   the history, standing on the points clicked —
//!   `half_an_ellipse_is_one_step_standing_on_the_two_ends_clicked`
//! - the second axis is laid from the centre out to the rise, so the rise typed
//!   is what it measures — `the_rise_typed_is_what_the_second_axis_measures`
//!
//! Closes #314, for the ellipse.
//! - an angle typed for the first axis leaves a construction arm from the
//!   centre, and is read against it —
//!   `an_angle_typed_for_an_ellipse_leans_on_an_arm_from_its_centre`

use cao_part::PartDocument;
use cao_sketch::{DimensionTarget, EllipseMode, SketchAxis, WorkPlane};
use chrono::Utc;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, Tool};

/// Clicks the ellipse tool at each place in turn, with whatever each click
/// finds typed into the live fields, on a fresh sketch.
fn clicked(clicks: &[(DVec2, [Option<f64>; 2])]) -> PartDocument {
    clicked_in(EllipseMode::ByCentre, clicks)
}

fn clicked_in(mode: EllipseMode, clicks: &[(DVec2, [Option<f64>; 2])]) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    let mut editor = SketchEditor {
        tool: Tool::Ellipse,
        ellipse_mode: mode,
        ..SketchEditor::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    for (place, typed) in clicks {
        editor.live.field(0).locked = typed[0];
        editor.live.field(1).locked = typed[1];
        let mut context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        draw_ellipse(&mut context, 0, *place, 0.01, 0.1);
    }
    document
}

#[test]
fn three_clicks_lay_the_ellipse_its_axes_and_its_centre_as_one_step() {
    let mut document = clicked(&[
        (DVec2::new(50.0, 20.0), [None, None]),
        (DVec2::new(80.0, 20.0), [None, None]),
        (DVec2::new(47.0, 30.0), [None, None]),
    ]);

    let drawing = &document.sketches()[0];
    let (id, _) = drawing.live_ellipses().next().expect("an ellipse");
    let drawn = drawing.ellipse_draft(id);
    assert!(drawn.centre.distance(DVec2::new(50.0, 20.0)) < 1e-9);
    assert!(drawn.first.distance(DVec2::new(30.0, 0.0)) < 1e-9);
    assert!((drawn.second - 10.0).abs() < 1e-9);
    assert!(drawing.dimensions().is_empty(), "nothing was typed");

    document.undo();
    assert_eq!(document.sketches()[0].live_ellipses().count(), 0);
    assert_eq!(document.sketches()[0].live_segments().count(), 0);
}

#[test]
fn the_values_typed_while_placing_become_dimensions_on_the_axes() {
    let document = clicked(&[
        (DVec2::new(0.0, 0.0), [None, None]),
        (DVec2::new(10.0, 1.0), [Some(40.0), Some(30.0)]),
        (DVec2::new(-1.0, 3.0), [Some(12.0), None]),
    ]);

    let drawing = &document.sketches()[0];
    let (id, oval) = drawing.live_ellipses().next().expect("an ellipse");
    let value = |target| {
        drawing
            .dimension_of(target)
            .map(|dimension| dimension.value)
    };
    assert_eq!(value(DimensionTarget::Length(oval.first)), Some(40.0));
    assert_eq!(value(DimensionTarget::Length(oval.second)), Some(12.0));
    // The angle typed is no longer written against the axis: it is read against
    // a real arm, which
    // `an_angle_typed_for_an_ellipse_leans_on_an_arm_from_its_centre` holds.
    // What it does to the drawing is the same, and that is what follows.
    let drawn = drawing.ellipse_draft(id);
    assert!((drawn.first.length() - 20.0).abs() < 1e-4, "{drawn:?}");
    assert!(
        (drawn.first.to_angle().to_degrees() - 30.0).abs() < 1e-3,
        "{drawn:?}"
    );
    assert!((drawn.second - 6.0).abs() < 1e-4, "{drawn:?}");
}

#[test]
fn a_last_click_on_the_first_axis_draws_no_ellipse() {
    let document = clicked(&[
        (DVec2::new(0.0, 0.0), [None, None]),
        (DVec2::new(10.0, 0.0), [None, None]),
        (DVec2::new(-4.0, 0.0), [None, None]),
    ]);

    assert_eq!(document.sketches()[0].live_ellipses().count(), 0);
}

#[test]
fn an_end_worked_out_rather_than_clicked_is_not_welded_to_a_point_beside_it() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(-30.0, 0.001),
        on: Vec::new(),
    });
    let mut editor = SketchEditor {
        tool: Tool::Ellipse,
        ..SketchEditor::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    for place in [DVec2::ZERO, DVec2::new(30.0, 0.0), DVec2::new(0.0, 10.0)] {
        let mut context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        draw_ellipse(&mut context, 0, place, 0.5, 0.1);
    }

    let drawing = &document.sketches()[0];
    let (id, _) = drawing.live_ellipses().next().expect("an ellipse");
    let lone = cao_sketch::PointId(1);
    assert!(
        !drawing.ellipse_points(id).contains(&lone),
        "the far end of the first axis was never clicked, and took the point beside it",
    );
}

#[test]
fn half_an_ellipse_is_one_step_standing_on_the_two_ends_clicked() {
    let (left, right) = (DVec2::new(-30.0, 10.0), DVec2::new(30.0, 10.0));
    let mut document = clicked_in(
        EllipseMode::ByEnds,
        &[
            (left, [None, None]),
            (right, [None, None]),
            (DVec2::new(5.0, 30.0), [None, None]),
        ],
    );

    let drawing = &document.sketches()[0];
    let (id, ellipse) = drawing.live_ellipses().next().expect("an ellipse");
    assert_eq!(
        drawing.endpoints(ellipse.first),
        (left, right),
        "the two ends clicked are the first axis's own ends",
    );
    let (from, to) = drawing.ellipse_ends(id).expect("half of it is drawn");
    assert_eq!(
        (drawing.point(from), drawing.point(to)),
        (right, left),
        "and the stretch runs between those very points, the rise being above",
    );
    let laid = drawing.live_points().count();

    document.undo();

    assert_eq!(
        document.sketches()[0].live_ellipses().count(),
        0,
        "one undo takes the whole of it: it was laid as one step",
    );
    assert_eq!(
        laid, 5,
        "four points and the sketch origin: the centre is an end of the second \
         axis rather than a fifth point, and none was laid twice",
    );
}

#[test]
fn the_rise_typed_is_what_the_second_axis_measures() {
    let document = clicked_in(
        EllipseMode::ByEnds,
        &[
            (DVec2::new(-30.0, 10.0), [None, None]),
            (DVec2::new(30.0, 10.0), [Some(60.0), None]),
            (DVec2::new(5.0, 30.0), [Some(20.0), None]),
        ],
    );

    let drawing = &document.sketches()[0];
    let (id, ellipse) = drawing.live_ellipses().next().expect("an ellipse");
    let value = |target| {
        drawing
            .dimensions()
            .iter()
            .find(|held| held.target == target)
            .map(|held| held.value)
    };
    assert_eq!(
        value(DimensionTarget::Length(ellipse.first)),
        Some(60.0),
        "the gap typed between the two ends is what the first axis measures",
    );
    assert_eq!(
        value(DimensionTarget::Length(ellipse.second)),
        Some(20.0),
        "and the axis runs from the centre to the rise, so it measures the rise",
    );
    assert_eq!(
        drawing.segments()[ellipse.second.0].start,
        ellipse.center,
        "which is where it starts",
    );
    assert!((drawing.ellipse_draft(id).second - 20.0).abs() < 1e-6);
}

#[test]
fn an_angle_typed_for_an_ellipse_leans_on_an_arm_from_its_centre() {
    let mut document = clicked(&[
        (DVec2::new(0.0, 0.0), [None, None]),
        (DVec2::new(10.0, 1.0), [Some(40.0), Some(30.0)]),
        (DVec2::new(-1.0, 3.0), [Some(12.0), None]),
    ]);

    let drawing = &document.sketches()[0];
    let (_, oval) = drawing.live_ellipses().next().expect("an ellipse");
    let centre = drawing.point(oval.center);

    let arm = drawing
        .live_segments()
        .find(|(id, segment)| {
            let (from, to) = drawing.endpoints(*id);
            segment.construction
                && *id != oval.first
                && *id != oval.second
                && from.distance(centre) < 1e-9
                && (to.y - from.y).abs() < 1e-9
                && to.x > from.x
        })
        .map(|(id, _)| id);
    let Some(arm) = arm else {
        panic!("no arm springs east from the ellipse's centre for its angle to be read against");
    };

    assert!(
        drawing
            .dimension_of(DimensionTarget::AxisAngle {
                segment: oval.first,
                axis: SketchAxis::U,
            })
            .is_none(),
        "the ghost angle against the axis is still written beside the real arm",
    );

    let read = drawing
        .dimensions()
        .iter()
        .find(|dimension| match dimension.target {
            DimensionTarget::AngleBetween { first, second, .. } => {
                (first, second) == (arm, oval.first) || (first, second) == (oval.first, arm)
            }
            DimensionTarget::Angle { first, second } => {
                (first, second) == (arm, oval.first) || (first, second) == (oval.first, arm)
            }
            _ => false,
        })
        .expect("the arm carries no reading of the angle typed");
    assert!(
        (read.value - 30.0).abs() < 1e-3,
        "the reading says {} where 30 was typed",
        read.value,
    );

    document.undo();
    assert_eq!(
        document.sketches()[0].live_ellipses().count(),
        0,
        "the arm and its reading left the ellipse behind: the gesture is not one step",
    );
    assert_eq!(document.sketches()[0].live_segments().count(), 0);
}
