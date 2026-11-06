//! What app · screens/viewport/render/overlays.rs is held to.
//!
//! Closes #286.
//! - the rule marks that will go are written in the alert colour, even the one
//!   under the cursor — `a_rule_the_cut_would_take_is_marked_in_the_alert_colour`
//!
//! Closes #656.
//! - On a rendered frame, a mark pointed at and a mark held are brighter than a
//!   mark at rest, on every shipped theme —
//!   `a_rule_s_mark_pointed_at_is_painted_brighter_than_a_mark_at_rest`,
//!   `a_rule_s_mark_held_is_painted_brighter_than_a_mark_at_rest`
//! - A test reads the colour off the painted mark and holds it apart from the
//!   resting one by more than its alpha —
//!   `a_rule_s_mark_pointed_at_is_painted_brighter_than_a_mark_at_rest`,
//!   `a_rule_s_mark_held_is_painted_brighter_than_a_mark_at_rest`: each takes
//!   the colour egui tessellates for the mark, weighs it laid over every colour
//!   the canvas takes, and wants it brighter than the resting mark by a contrast
//!   of `CONTRAST_TO_STAND_OUT` or more, and brighter than the resting mark
//!   would be at the same opacity
//!
//! Closes #724.
//! - found in review: every number in JetBrains Mono, as "To try it" promises —
//!   the scale bar's figure is written in the monospace family —
//!   `the_scale_bar_s_figure_is_written_in_the_monospace_family`; the words
//!   stay in Inter, the cube's face names among them —
//!   `the_names_of_the_cube_s_faces_stay_in_the_proportional_family`

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;
use cao_sketch::{SegmentId, SelectState, Stretch, ToolState, WorkPlane};
use glam::DVec2;

use super::*;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};

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
        // An area fades in over its first frames: read two frames in, every
        // colour would come back at a fraction of its opacity.
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .fade_in(false)
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

/// The family each piece of text a frame painted is written in, beside the
/// text itself.
fn families(shapes: &[egui::Shape]) -> Vec<(String, egui::FontFamily)> {
    shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) => Some(&text.galley),
            _ => None,
        })
        .flat_map(|galley| {
            galley
                .job
                .sections
                .iter()
                .map(|section| {
                    (
                        galley.text().to_string(),
                        section.format.font_id.family.clone(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn the_scale_bar_s_figure_is_written_in_the_monospace_family() {
    let (state, scale) = a_view();

    let written = families(&shapes(|ui| paint_ruler(ui, &state, SCREEN, scale)));

    assert!(
        written.iter().any(|(text, _)| holds_a_figure(text))
            && written
                .iter()
                .filter(|(text, _)| holds_a_figure(text))
                .all(|(_, family)| *family == egui::FontFamily::Monospace),
        "the scale bar's figure is written {written:?}: the figures go in the monospace family",
    );
}

#[test]
fn the_names_of_the_cube_s_faces_stay_in_the_proportional_family() {
    let (state, _) = a_view();
    let lang = Catalogue::french();

    let written = families(&shapes(|ui| paint_face_labels(ui, &state, SCREEN, &lang)));

    assert!(
        !written.is_empty()
            && written
                .iter()
                .all(|(_, family)| *family == egui::FontFamily::Proportional),
        "the cube's face names are written {written:?}",
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
    let pointed = mark_shade(&theme, None, &rule, true);
    assert_ne!(
        pointed,
        tint_to_color(theme.going),
        "with no cut in sight, a rule pointed at is marked as going",
    );
    assert_ne!(
        pointed,
        tint_to_color(theme.rule),
        "with no cut in sight, a rule pointed at is no longer lit",
    );
}

/// Every theme the application ships with. There is one today; a theme added
/// beside it belongs here, so that the marks are held to it too.
fn shipped_themes() -> Vec<Theme> {
    vec![Theme::default()]
}

const LIT: Constraint = Constraint::Parallel {
    first: SegmentId(0),
    second: SegmentId(1),
};

const AT_REST: Constraint = Constraint::Equal {
    first: SegmentId(2),
    second: SegmentId(3),
};

/// A drawing being edited with two rules on it, each marked on its own traits:
/// the two long sides held parallel, the two short uprights held equal.
fn a_drawing_with_two_rules() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    for (start, end) in [
        ((0.0, 0.0), (40.0, 0.0)),
        ((0.0, 20.0), (40.0, 20.0)),
        ((10.0, 5.0), (10.0, 15.0)),
        ((30.0, 5.0), (30.0, 15.0)),
    ] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(start.0, start.1)),
            end: PointRef::New(DVec2::new(end.0, end.1)),
            construction: false,
        });
    }
    for constraint in [LIT, AT_REST] {
        document.apply(Operation::Constrain {
            sketch: 0,
            constraint,
        });
    }
    document
}

/// The colour a piece of painted text reaches the screen in: the shape is
/// tessellated, so whatever egui does to it on the way — a fallback, an
/// override, an opacity — is already applied.
fn ink(text: &egui::epaint::TextShape) -> egui::Color32 {
    let mut tessellated = Default::default();
    egui::epaint::Tessellator::new(1.0, Default::default(), [1, 1], Vec::new())
        .tessellate_text(text, &mut tessellated);
    let mut colours: Vec<egui::Color32> = tessellated
        .vertices
        .iter()
        .map(|vertex| vertex.color)
        .collect();
    colours.dedup();
    assert_eq!(
        colours.len(),
        1,
        "a mark painted in {colours:?} rather than in one colour"
    );
    colours[0]
}

/// Each mark one frame painted, by its glyph, with the colour it was painted in.
fn marks_painted(theme: &Theme, mut editor: SketchEditor) -> Vec<(String, egui::Color32)> {
    let mut state = ViewportState::default();
    state.theme = theme.clone();
    let mut document = a_drawing_with_two_rules();
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    shapes(|ui| paint_rule_marks(ui, &state, SCREEN, &context, None))
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) => Some((text.galley.text().to_string(), ink(text))),
            _ => None,
        })
        .collect()
}

/// How bright `ink` reads laid over `behind`: egui blends a premultiplied
/// colour in gamma space, and what comes out is weighed by its relative
/// luminance — the sRGB definition contrast is measured with.
fn brightness_over(ink: egui::Color32, behind: Rgba) -> f32 {
    let [r, g, b, alpha] = ink.to_normalized_gamma_f32();
    let linear = |channel: f32| {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    };
    let seen = |ink: f32, behind: f32| linear(ink + (1.0 - alpha) * behind);
    0.2126 * seen(r, behind.r) + 0.7152 * seen(g, behind.g) + 0.0722 * seen(b, behind.b)
}

/// `ink`'s colour, at another opacity.
fn at_opacity(ink: egui::Color32, alpha: u8) -> egui::Color32 {
    let [r, g, b, _] = ink.to_srgba_unmultiplied();
    egui::Color32::from_rgba_unmultiplied(r, g, b, alpha)
}

/// Below this contrast against its neighbours, a 13-point glyph does not stand
/// out from them.
const CONTRAST_TO_STAND_OUT: f32 = 1.4;

/// The contrast ratio of a brightness against another, as WCAG measures it.
fn contrast(lighter: f32, darker: f32) -> f32 {
    (lighter + 0.05) / (darker + 0.05)
}

/// On every shipped theme, and over every colour the canvas takes behind
/// them, each mark of the rule `editor` lights reads brighter than each mark
/// of the rule left at rest, by [`CONTRAST_TO_STAND_OUT`] or more — and
/// brighter than the resting mark would read at the lit one's opacity, so that
/// a mark made merely more opaque does not pass for one that lights up.
fn lit_marks_outshine_the_marks_at_rest(how: &str, editor: impl Fn() -> SketchEditor) {
    for (index, theme) in shipped_themes().iter().enumerate() {
        let painted = marks_painted(theme, editor());
        let of = |rule: Constraint| -> Vec<egui::Color32> {
            painted
                .iter()
                .filter(|(glyph, _)| glyph == constraints::mark(rule))
                .map(|(_, colour)| *colour)
                .collect()
        };
        let (lit, at_rest) = (of(LIT), of(AT_REST));
        assert!(
            !lit.is_empty() && !at_rest.is_empty(),
            "the frame does not mark both rules: it painted {painted:?}"
        );
        for canvas in (0..=10).map(|step| theme.background.sample(step as f32 / 10.0)) {
            for &lit in &lit {
                for &rest in &at_rest {
                    let (lit_seen, rest_seen) =
                        (brightness_over(lit, canvas), brightness_over(rest, canvas));
                    let against_rest = contrast(lit_seen, rest_seen);
                    assert!(
                        against_rest >= CONTRAST_TO_STAND_OUT,
                        "on shipped theme {index}, over the canvas at {canvas:?}, the mark {how} \
                         ({lit:?}) reads {lit_seen:.3} bright where a mark at rest ({rest:?}) \
                         reads {rest_seen:.3}, a contrast of {against_rest:.2}:1 short of \
                         {CONTRAST_TO_STAND_OUT}:1: it does not stand out from the others",
                    );
                    let rest_at_that_opacity = brightness_over(at_opacity(rest, lit.a()), canvas);
                    assert!(
                        lit_seen > rest_at_that_opacity,
                        "on shipped theme {index}, over the canvas at {canvas:?}, the mark {how} \
                         reads {lit_seen:.3} bright, no more than a mark at rest painted at its \
                         opacity ({rest_at_that_opacity:.3}): it stands out by its alpha alone",
                    );
                }
            }
        }
    }
}

#[test]
fn a_rule_s_mark_pointed_at_is_painted_brighter_than_a_mark_at_rest() {
    lit_marks_outshine_the_marks_at_rest("pointed at", || SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        hovered: Some(Selection::Rule(LIT)),
        ..Default::default()
    });
}

#[test]
fn a_rule_s_mark_held_is_painted_brighter_than_a_mark_at_rest() {
    lit_marks_outshine_the_marks_at_rest("held", || SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::new(SelectState {
            held: vec![Selection::Rule(LIT)],
            ..Default::default()
        })),
        ..Default::default()
    });
}

#[test]
fn a_mark_set_aside_by_another_is_taken_where_it_was_set() {
    let mut sketch = cao_sketch::Sketch::new(cao_sketch::WorkPlane::XY);
    let corners = [(0.0, 0.0), (40.0, 0.0), (0.0, 20.0), (40.0, 20.0)]
        .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let low = sketch.add_segment(corners[0], corners[1]);
    let high = sketch.add_segment(corners[2], corners[3]);
    let parallel = Constraint::Parallel {
        first: low,
        second: high,
    };
    let equal = Constraint::Equal {
        first: low,
        second: high,
    };
    sketch.add_constraint(parallel);
    sketch.add_constraint(equal);
    let (state, _) = a_view();
    let placed = placed_marks(
        &sketch,
        state.camera.view_projection(SCREEN.aspect_ratio()),
        SCREEN,
    );

    let written: Vec<egui::Pos2> = placed
        .iter()
        .filter(|(rule, _)| *rule == equal)
        .map(|(_, at)| *at)
        .collect();
    assert_eq!(
        written.len(),
        2,
        "the equality is marked on both traits: {placed:?}"
    );
    for at in written {
        assert_eq!(
            mark_under(&placed, at),
            Some(equal),
            "taken where it was set aside to"
        );
        assert_eq!(
            mark_under(&placed, at - egui::vec2(MARK_SPACING, 0.0)),
            Some(parallel)
        );
        assert_eq!(
            mark_under(&placed, at + egui::vec2(MARK_REACH + 1.0, 0.0)),
            None,
            "and no further"
        );
    }
}

/// Whether a piece of text holds a figure: a label may set a word beside it,
/// « Distance » or « R », and the word may stay in the proportional family.
fn holds_a_figure(text: &str) -> bool {
    text.chars().any(|character| character.is_ascii_digit())
}
