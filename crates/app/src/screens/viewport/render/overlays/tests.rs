//! What app · screens/viewport/render/overlays.rs is held to.
//!
//! Closes #286.
//! - the rule marks that will go are written in the alert colour, even the one
//!   under the cursor — `a_rule_the_cut_would_take_is_marked_in_the_alert_colour`

use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::{SegmentId, Stretch};
use glam::DVec2;

use super::*;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// Everything one frame painted, with no window and no GPU: egui is asked to
/// run a single pass and hand back the shapes it would have sent out.
fn shapes(paint: impl Fn(&egui::Ui)) -> Vec<egui::Shape> {
    let context = egui::Context::default();
    let mut output = egui::FullOutput::default();
    // The first pass has no size for the area yet and lays nothing down; the
    // second paints against what the first worked out.
    for _ in 0..2 {
        output.textures_delta.clear();
        context.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .show(&context, |ui| paint(ui));
        output = context.end_pass();
    }
    output.textures_delta.clear();
    let mut flat = Vec::new();
    for clipped in output.shapes {
        flatten(clipped.shape, &mut flat);
    }
    flat
}

/// egui hands a layer back as one shape holding the rest, so what a painter
/// added is a leaf of that tree rather than an entry of the list.
fn flatten(shape: egui::Shape, out: &mut Vec<egui::Shape>) {
    match shape {
        egui::Shape::Vec(shapes) => {
            for shape in shapes {
                flatten(shape, out);
            }
        }
        shape => out.push(shape),
    }
}

/// Every piece of text a frame painted.
fn words(shapes: &[egui::Shape]) -> Vec<String> {
    shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_string()),
            _ => None,
        })
        .collect()
}

fn a_view() -> (ViewportState, ViewScale) {
    let state = ViewportState::default();
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        1.0,
    );
    (state, scale)
}

#[test]
fn the_scale_bar_says_how_long_one_grid_step_is() {
    let (state, scale) = a_view();
    let expected = state.config.unit.format(scale.step_millimeters);

    let painted = shapes(|ui| paint_ruler(ui, &state, SCREEN, scale));

    assert!(
        words(&painted).contains(&expected),
        "the scale bar is drawn without saying what it measures; it said {:?}",
        words(&painted),
    );
}

#[test]
fn the_scale_bar_is_exactly_one_grid_step_long() {
    let (state, scale) = a_view();
    let step = scale.step_in_points(1.0);

    let painted = shapes(|ui| paint_ruler(ui, &state, SCREEN, scale));
    let widest = painted
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::LineSegment { points, .. } => Some((points[1].x - points[0].x).abs()),
            _ => None,
        })
        .fold(0.0_f32, f32::max);

    assert!(
        (widest - step).abs() < 0.5,
        "the bar is {widest} points long where a grid step is {step}",
    );
}

#[test]
fn the_labels_of_the_orientation_cube_name_the_faces_that_can_be_seen() {
    let (state, _) = a_view();
    let lang = Catalogue::french();

    let painted = shapes(|ui| paint_face_labels(ui, &state, SCREEN, &lang));
    let said = words(&painted);

    assert!(
        !said.is_empty(),
        "the cube is drawn with no label on any of its faces",
    );
    assert!(
        said.len() < 6,
        "every face of the cube is labelled at once, back ones included: {said:?}",
    );
}

#[test]
fn a_rule_the_cut_would_take_is_marked_in_the_alert_colour() {
    let theme = Theme::default();
    let rule = Constraint::Parallel {
        first: SegmentId(0),
        second: SegmentId(1),
    };
    let going = Going {
        stretch: Stretch::Straight {
            from: DVec2::ZERO,
            to: DVec2::new(10.0, 0.0),
        },
        construction: false,
        rules: vec![rule],
        values: Vec::new(),
    };

    assert_eq!(
        mark_shade(&theme, Some(&going), &rule, false),
        tint_to_color(theme.going),
        "a rule about to go is marked like one that stays",
    );
    assert_eq!(
        mark_shade(&theme, Some(&going), &rule, true),
        tint_to_color(theme.going),
        "pointing at a rule about to go says the wrong one of the two things",
    );
    assert_eq!(
        mark_shade(
            &theme,
            Some(&going),
            &Constraint::Parallel {
                first: SegmentId(2),
                second: SegmentId(3),
            },
            false
        ),
        tint_to_color(theme.rule),
        "a rule the cut leaves alone is marked as going",
    );
    assert_eq!(
        mark_shade(&theme, None, &rule, true),
        tint_to_color(theme.highlight),
        "with no cut in sight, a rule pointed at is no longer lit",
    );
}
