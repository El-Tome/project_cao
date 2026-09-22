//! What app · screens/viewport/render/ellipse.rs is held to.

use cao_part::PartDocument;
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::{LockedInput, ToolState, WorkPlane};
use chrono::Utc;
use glam::Vec3;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;

/// What one frame of the canvas paints, and what the live fields read, for an
/// ellipse with these places already picked and the cursor here.
fn a_frame(places: Vec<DVec2>, cursor: DVec2) -> (Vec<DVec2>, Option<[f64; 2]>) {
    let mut document = PartDocument::new("part", Utc::now());
    let mut editor = SketchEditor {
        tool_state: ToolState::Ellipse {
            places,
            first_typed: LockedInput::default(),
        },
        ..SketchEditor::default()
    };
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let millimetres = document.scale();
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
        1.0,
        &ViewportConfig::default(),
        millimetres,
    );

    let sketch = Sketch::new(WorkPlane::XY);
    let mut out = Vec::new();
    push_preview(&mut out, &context, &sketch, cursor, [1.0; 4], scale);
    let painted = out
        .iter()
        .map(|vertex| {
            sketch
                .plane
                .to_local(Vec3::from(vertex.position).as_dvec3())
        })
        .collect();
    let fields = live_fields(&context, cursor).map(|(_, values)| values);
    (painted, fields)
}

#[test]
fn over_the_centre_alone_the_first_axis_is_shown_across_the_whole_curve() {
    let (painted, fields) = a_frame(vec![DVec2::ZERO], DVec2::new(30.0, 0.0));

    for end in [DVec2::new(-30.0, 0.0), DVec2::new(30.0, 0.0)] {
        assert!(
            painted.iter().any(|place| place.distance(end) < 1e-6),
            "nothing reaches {end}",
        );
    }
    let [width, angle] = fields.expect("the fields of the first axis");
    assert!(
        (width - 60.0).abs() < 1e-9,
        "the width across the curve: {width}"
    );
    assert!(angle.abs() < 1e-9);
}

#[test]
fn with_the_first_axis_given_the_curve_is_shown_through_the_cursor() {
    let (painted, fields) = a_frame(
        vec![DVec2::ZERO, DVec2::new(30.0, 0.0)],
        DVec2::new(0.0, 10.0),
    );

    let drawn = cao_sketch::EllipseDraft {
        centre: DVec2::ZERO,
        first: DVec2::new(30.0, 0.0),
        second: 10.0,
    };
    let on_the_curve = painted
        .iter()
        .filter(|place| drawn.distance(**place) < 1e-6)
        .count();
    assert!(
        on_the_curve > 90,
        "the curve is drawn: {on_the_curve} places on it"
    );
    let [width, _] = fields.expect("the field of the second axis");
    assert!((width - 20.0).abs() < 1e-9, "{width}");
}
