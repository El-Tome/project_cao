//! What app · screens/viewport/render/drawing.rs is held to.

use cao_part::PartDocument;
use cao_part::history::{Operation, PointRef};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::WorkPlane;
use chrono::Utc;
use glam::Vec3;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

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
