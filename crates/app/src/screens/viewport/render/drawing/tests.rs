//! What app · screens/viewport/render/drawing.rs is held to.
//!
//! Closes #286.
//! - the dimensions that will go with the stretch are drawn in the alert
//!   colour — `a_value_the_cut_would_take_is_drawn_in_the_alert_colour`
//! - their written value follows — no test: it is the same list read one file
//!   away, in `dimensions.rs`, and egui paints it as text on a window this
//!   test does not open
//!
//! The tint's share of #621, whose criteria live with the part's tests in
//! `crates/part/tests/a_fillet_rounds_a_corner_with_a_curve.rs`: a plate
//! rounded beside an arc is tinted over its rounded area —
//! `a_plate_rounded_beside_an_arc_is_tinted_along_its_rounded_outline`.
//!
//! Closes #321.
//! - the preview updates as soon as the panel holds enough to lay something,
//!   wherever the pointer is — over the panel, the canvas has no cursor —
//!   `a_copy_s_preview_shows_with_the_pointer_off_the_drawing`

use cao_part::PartDocument;
use cao_part::history::{Operation, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::{DimensionTarget, SegmentId, Stretch, WorkPlane};
use chrono::Utc;
use glam::Vec3;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::input::copying::{copy, enter_in_the_panel};

/// A vertex holds its position in `f32`, so a place read back off one has lost
/// the tail of its `f64`. Well under the width of anything drawn here.
const TOLERANCE: f64 = 1e-5;

const CORNER: DVec2 = DVec2::ZERO;
const OPPOSITE: DVec2 = DVec2::new(10.0, 20.0);

fn a_part_with_a_rectangle() -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(CORNER),
        opposite: PointRef::New(OPPOSITE),
        construction: false,
    });
    document
}

fn a_view() -> ViewScale {
    ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
        1.0,
        &ViewportConfig::default(),
        1.0,
    )
}

/// What one frame paints of the drawing, as lines and as surfaces.
fn painted(active: bool) -> (Vec<cao_render::Vertex>, Vec<cao_render::Vertex>) {
    let mut document = a_part_with_a_rectangle();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let sketch = document.sketches()[0].clone();

    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let shown = Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: None,
        active,
    };
    let (mut lines, mut surfaces) = (Vec::new(), Vec::new());
    push_sketch(
        &mut lines,
        &mut surfaces,
        &shown,
        &Theme::default(),
        a_view(),
        &context,
    );
    (lines, surfaces)
}

fn place(vertex: &cao_render::Vertex, sketch: &Sketch) -> DVec2 {
    sketch
        .plane
        .to_local(Vec3::from(vertex.position).as_dvec3())
}

#[test]
fn every_corner_of_what_was_drawn_is_painted_where_it_was_put() {
    let sketch = a_part_with_a_rectangle().sketches()[0].clone();
    let (lines, _) = painted(true);

    for corner in [
        CORNER,
        DVec2::new(OPPOSITE.x, CORNER.y),
        OPPOSITE,
        DVec2::new(CORNER.x, OPPOSITE.y),
    ] {
        assert!(
            lines
                .iter()
                .any(|vertex| place(vertex, &sketch).distance(corner) < TOLERANCE),
            "nothing is painted at {corner:?}, which is a corner of what was drawn",
        );
    }
}

#[test]
fn a_closed_contour_is_tinted_so_it_reads_as_a_face() {
    let (_, surfaces) = painted(true);

    assert!(
        !surfaces.is_empty(),
        "a closed contour is drawn as four separate lines and nothing else",
    );
    assert_eq!(
        surfaces.len() % 3,
        0,
        "an area is filled with something other than whole triangles",
    );
}

#[test]
fn a_drawing_set_aside_keeps_its_contour_and_loses_its_points() {
    let sketch = a_part_with_a_rectangle().sketches()[0].clone();
    let (active, active_fill) = painted(true);
    let (idle, idle_fill) = painted(false);

    assert_ne!(
        active[0].color, idle[0].color,
        "the drawing being worked on looks exactly like one set aside",
    );
    assert_ne!(
        active_fill[0].color, idle_fill[0].color,
        "the area of a drawing set aside is tinted like the active one",
    );
    assert!(
        active.len() > idle.len(),
        "a drawing set aside is painted everything the active one is",
    );
    for corner in [CORNER, OPPOSITE] {
        assert!(
            idle.iter()
                .any(|vertex| place(vertex, &sketch).distance(corner) < TOLERANCE),
            "a drawing set aside lost the corner at {corner:?} from its contour",
        );
    }
}

#[test]
fn a_shape_drawn_inside_another_takes_more_of_the_tint_than_the_one_around_it() {
    let mut document = a_part_with_a_rectangle();
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(2.0, 2.0)),
        opposite: PointRef::New(DVec2::new(8.0, 18.0)),
        construction: false,
    });
    let sketch = document.sketches()[0].clone();

    let mut surfaces = Vec::new();
    push_regions(&mut surfaces, &sketch, &Theme::default(), true);

    let depths: Vec<usize> = sketch.regions().iter().map(|region| region.depth).collect();
    assert!(
        depths.iter().any(|depth| *depth > 0),
        "a rectangle inside another gives no nested area at all: {depths:?}",
    );
    let shades: Vec<f32> = surfaces.iter().map(|vertex| vertex.color[3]).collect();
    assert!(
        shades.iter().any(|shade| *shade > shades[0]),
        "every area is tinted the same, so an outline and its pocket wash into one another",
    );
}

/// The rectangle with a length on one of its sides, and a cut that would take
/// that side away — the length said to go with it or not, and nothing else
/// told apart, so that what changes between the two is the value alone.
fn painted_with_the_length_going(said_to_go: bool) -> Vec<cao_render::Vertex> {
    let mut document = a_part_with_a_rectangle();
    let mut sketch = document.sketches()[0].clone();
    let side = SegmentId(0);
    sketch.set_dimension(DimensionTarget::Length(side), 10.0, false);
    let going = Going {
        stretch: Stretch::Straight {
            from: CORNER,
            to: OPPOSITE,
        },
        construction: false,
        rules: Vec::new(),
        values: match said_to_go {
            true => vec![DimensionTarget::Length(side)],
            false => Vec::new(),
        },
    };

    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let shown = Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: Some(&going),
        active: true,
    };
    let (mut lines, mut surfaces) = (Vec::new(), Vec::new());
    push_sketch(
        &mut lines,
        &mut surfaces,
        &shown,
        &Theme::default(),
        a_view(),
        &context,
    );
    lines
}

#[test]
fn a_value_the_cut_would_take_is_drawn_in_the_alert_colour() {
    let alert = tint(Theme::default().going);
    let in_alert = |lines: Vec<cao_render::Vertex>| {
        lines.iter().filter(|vertex| vertex.color == alert).count()
    };

    let going = in_alert(painted_with_the_length_going(true));
    let staying = in_alert(painted_with_the_length_going(false));

    assert!(
        going > staying,
        "the length about to go is drawn exactly like one that stays — the \
         stretch is red in both, so only the value can tell them apart",
    );
}

/// A plate whose corner at (10, 10) stands between a trait running east to
/// (50, 10) and an arc round (2, 10) of radius 8 rising a quarter turn to
/// (2, 18), closed by traits up to (50, 30) and across to the arc's end — and
/// the same plate with that corner rounded by five.
fn a_plate_beside_an_arc(rounded: bool) -> PartDocument {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(10.0, 10.0),
        on: Vec::new(),
    });
    let corner = cao_sketch::PointId(1);
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(corner),
        end: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(2.0, 10.0)),
        start: PointRef::Existing(corner),
        end: PointRef::New(DVec2::new(2.0, 18.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(2)),
        end: PointRef::New(DVec2::new(50.0, 30.0)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(5)),
        end: PointRef::Existing(cao_sketch::PointId(4)),
        construction: false,
    });
    if rounded {
        document.apply(Operation::Fillet {
            sketch: 0,
            corners: vec![cao_sketch::Corner::At(corner)],
            radius: 5.0.into(),
            construction: false,
        });
    }
    document
}

/// How much of the plane the frame tints, and the area the drawing reads.
fn tinted(mut document: PartDocument) -> (f64, f64) {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let sketch = document.sketches()[0].clone();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let shown = Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: None,
        active: true,
    };
    let (mut lines, mut surfaces) = (Vec::new(), Vec::new());
    push_sketch(
        &mut lines,
        &mut surfaces,
        &shown,
        &Theme::default(),
        a_view(),
        &context,
    );
    let tinted = surfaces
        .chunks(3)
        .map(|corners| {
            let [a, b, c] = [0, 1, 2].map(|rank| place(&corners[rank], &sketch));
            (b - a).perp_dot(c - a).abs() * 0.5
        })
        .sum();
    let read: f64 = sketch.regions().iter().map(|region| region.area()).sum();
    (tinted, read)
}

#[test]
fn a_plate_rounded_beside_an_arc_is_tinted_along_its_rounded_outline() {
    let (sharp, sharp_area) = tinted(a_plate_beside_an_arc(false));
    let (rounded, rounded_area) = tinted(a_plate_beside_an_arc(true));

    assert!(
        sharp_area - rounded_area > 2.0,
        "the fillet takes its sliver off the area: {sharp_area} against {rounded_area}"
    );
    assert!(
        (rounded - rounded_area).abs() < 0.2,
        "the rounded plate is tinted over its rounded area, {rounded_area}, not {rounded}"
    );
    assert!(
        (sharp - rounded) > 2.0,
        "less is tinted once the corner is rounded: {sharp} against {rounded}"
    );
}

#[test]
fn a_copy_s_preview_shows_with_the_pointer_off_the_drawing() {
    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(4.0, 5.0)),
        end: PointRef::New(DVec2::new(8.0, 5.0)),
        construction: false,
    });
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    crate::commands::run(
        cao_prefs::Command::ToolMirror,
        &mut context,
        &mut crate::screens::ribbon::Ribbon::default(),
        &mut crate::screens::viewport::ViewportState::default(),
    );
    let (snap, pixel) = (0.5, 0.05);
    copy(&mut context, 0, DVec2::new(6.0, 5.0), snap, pixel);
    enter_in_the_panel(&mut context);
    copy(&mut context, 0, DVec2::new(20.0, 0.0), snap, pixel);

    context.editor.cursor = None;
    let sketch = context.document.sketches()[0].clone();

    assert!(
        what_would_be_laid(&sketch, &context, a_view()).is_some(),
        "the trait held, mirrored across the axis the panel holds, is shown with \
         the pointer over the panel"
    );
}
