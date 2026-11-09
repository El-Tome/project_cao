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
//!
//! Closes #729. The rest of the issue is in `render/tests.rs`,
//! `state/tests.rs` and `crates/app/tests/view_bar.rs`, each under its own
//! block.
//! - The names lie in perspective on their faces (« DESSUS », « FACE »,
//!   « DROITE »…), turning with the cube —
//!   `a_face_s_name_is_painted_lying_on_its_face_and_turns_with_the_cube`;
//!   decided on the branch: never mirrored from outside, a side face reads
//!   upright with +Z up, the top reads along +X with +Y up and the bottom along
//!   +X with -Y up — `a_side_face_s_name_reads_upright_and_none_is_mirrored`;
//!   a face seen too edge-on to read draws no name —
//!   `a_face_seen_too_edge_on_to_read_draws_no_name`; the name on a lit face
//!   stays readable against the amber —
//!   `the_name_on_a_lit_face_stays_readable_against_the_amber`; the words and
//!   their family carry over —
//!   `the_names_of_the_cube_s_faces_stay_in_the_proportional_family`, the names
//!   drawn are those of the faces that can be seen —
//!   `the_labels_of_the_orientation_cube_name_the_faces_that_can_be_seen`, and a
//!   hovered face's name keeps the colour it had —
//!   `a_hovered_face_s_name_keeps_the_title_colour`; found on screen: « DESSUS »
//!   stood upside down at the view a part opens with —
//!   `every_name_reads_upright_at_the_opening_view`
//! - An X/Y/Z triad in the cube's corner, in the axis colours —
//!   `a_triad_stands_in_the_cube_s_corner_in_the_axis_colours`, turning with
//!   the camera — `the_triad_turns_with_the_camera`; decided by the human, on
//!   screen, from another CAD program, in place of a triad set beside the cube:
//!   it is built into the cube, its three axes starting from the corner
//!   (-½, -½, -½) and running along the cube's edges past the far corner —
//!   `the_triad_runs_along_the_cube_s_edges_past_their_far_corners`; an axis
//!   behind the cube is not drawn: one is shown exactly when a face its edge
//!   bounds is strictly seen, or when it lies in the plane the view is square
//!   onto, and an axis end-on draws nothing —
//!   `an_axis_is_drawn_exactly_when_a_face_its_edge_bounds_is_seen`,
//!   `from_the_back_corner_no_axis_is_drawn`,
//!   `square_onto_a_sketch_two_axes_show_along_the_outline_and_the_third_not`;
//!   found in review: at a view the cube snaps to, two axes on one line wrote
//!   their letters over each other —
//!   `at_every_view_the_cube_snaps_to_no_two_axes_overprint`,
//!   `the_top_front_edge_view_shows_x_and_z_and_not_y`;
//!   its letter stands beyond its tip, and nothing of it touches a face name at
//!   the opening view or square onto a sketch —
//!   `a_letter_stands_beyond_its_tip_inside_the_square_and_off_the_names`;
//!   found on screen: it was too small to read —
//!   `the_triad_is_drawn_large_enough_to_read_on_a_cube_of_the_default_size`;
//!   found in review: the triad does not jump as the camera turns —
//!   `the_triad_does_not_jump_as_the_camera_turns`;
//!   found in review: a cube too small to hold a letter still paints —
//!   `a_cube_too_small_to_hold_a_letter_paints_without_a_panic`
//! - A name stays inside its face (#225) —
//!   `a_name_lying_on_its_face_never_leaves_it`

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_prefs::config::ViewportConfig;
use cao_render::CubeFace;
use cao_render::camera::OrbitCamera;
use cao_sketch::{SegmentId, SelectState, Stretch, ToolState, WorkPlane};
use glam::{DVec2, Vec3};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;

use super::*;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};
use crate::screens::viewport::ViewMode;

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
    let lang = Catalogue::french();
    let mut wanted_for: BTreeMap<String, Vec<f32>> = BTreeMap::new();
    let mut read = Vec::new();

    for (yaw, pitch) in views_all_around() {
        let (state, rect) = a_cube_seen_from(yaw, pitch, ROOMY_CUBE);
        for (face, mesh) in names_painted(&state, rect) {
            let word = crate::wording::cube::face(&lang, face);
            let map = FaceMap::of(&state, rect, face);
            let glyphs = quads(&mesh);
            assert_eq!(
                glyphs.len(),
                word.chars().count(),
                "the name on {face:?} holds {} glyphs, and its word {word:?} holds {}",
                glyphs.len(),
                word.chars().count(),
            );
            let wanted = wanted_for
                .entry(word.clone())
                .or_insert_with(|| rhythm_of(&laid_out(&word, egui::FontFamily::Proportional)));
            let along: Vec<Vec<f32>> = glyphs
                .iter()
                .map(|glyph| {
                    glyph
                        .iter()
                        .map(|vertex| map.on_face(vertex.pos).0)
                        .collect()
                })
                .collect();
            let drawn = rhythm_of(&along);
            let gap = largest_gap(&drawn, wanted);
            assert!(
                gap <= SAME_RHYTHM,
                "the name on {face:?} does not read as {word:?} in the proportional family: \
                 its letters start at {drawn:?} of its length, and they start at {wanted:?} \
                 when that word is laid out in it",
            );
            if !read.contains(&face) {
                read.push(face);
            }
        }
    }

    assert_eq!(
        read.len(),
        6,
        "only the names of {read:?} were painted as meshes all around the cube",
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
    for (yaw, pitch) in [(opening_yaw(), opening_pitch()), (0.7, -0.6), (2.4, 0.8)] {
        let (state, rect) = a_cube_seen_from(yaw, pitch, ROOMY_CUBE);
        let can_be_seen: Vec<CubeFace> = CubeFace::ALL
            .into_iter()
            .filter(|face| cube::is_visible(*face, state.camera.forward()))
            .collect();

        let mut named: Vec<CubeFace> = names_painted(&state, rect)
            .into_iter()
            .map(|(face, _)| face)
            .collect();
        named.sort_by_key(|face| CubeFace::ALL.iter().position(|other| other == face));

        assert_eq!(
            named, can_be_seen,
            "seen from yaw {yaw}, pitch {pitch}, the names painted are not the faces \
             that can be seen",
        );
    }
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

/// The angles a new part opens with.
fn opening_yaw() -> f32 {
    OrbitCamera::default().yaw()
}

fn opening_pitch() -> f32 {
    OrbitCamera::default().pitch()
}

/// A cube large enough for every name to fit at the angles these tests use.
const ROOMY_CUBE: f32 = 160.0;

/// How far a letter's start may stand from where the word laid out in its
/// family puts it, as a share of the word's length.
const SAME_RHYTHM: f32 = 0.05;

/// Past a face's edge by this much of its half-side is past its edge: the
/// vertices are computed, and the face's outline is drawn thicker than this.
const INSIDE_ITS_FACE: f32 = 1e-3;

/// Within a glyph, how far two corners that share a texture row or column may
/// stand apart across it, as a share of a face's half-side.
const SQUARE_ON_THE_FACE: f32 = 1e-2;

/// How far a name's width over its height may stand from the word's own.
const SAME_PROPORTIONS: f32 = 0.2;

/// WCAG's level AA for text.
const READABLE_CONTRAST: f32 = 4.5;

/// The state of a cube of side `size`, seen from these angles, and the square
/// it stands in.
fn a_cube_seen_from(yaw: f32, pitch: f32, size: f32) -> (ViewportState, egui::Rect) {
    let mut state = ViewportState::default();
    state.config.cube_size = size;
    state.camera.set_view_angles(yaw, pitch);
    let rect = egui::Rect::from_min_size(egui::pos2(40.0, 40.0), egui::Vec2::splat(size));
    (state, rect)
}

/// Which way a name on `face` reads, and which way is up on it, in the cube's
/// space. Reading direction crossed with up gives the face's outward normal,
/// which is what never mirrored from outside means.
fn reading_axes(face: CubeFace) -> (Vec3, Vec3) {
    match face {
        CubeFace::PlusX => (Vec3::Y, Vec3::Z),
        CubeFace::MinusX => (Vec3::NEG_Y, Vec3::Z),
        CubeFace::PlusY => (Vec3::NEG_X, Vec3::Z),
        CubeFace::MinusY => (Vec3::X, Vec3::Z),
        CubeFace::PlusZ => (Vec3::X, Vec3::Y),
        CubeFace::MinusZ => (Vec3::X, Vec3::NEG_Y),
    }
}

/// A face's projected parallelogram: its centre, and where half a side along
/// the reading direction and up from there land, on screen.
struct FaceMap {
    centre: egui::Pos2,
    along: egui::Vec2,
    up: egui::Vec2,
}

impl FaceMap {
    fn of(state: &ViewportState, rect: egui::Rect, face: CubeFace) -> Self {
        let project = |point: Vec3| on_the_cube_s_screen(state, rect, point);
        let centre = cube::face_center(face);
        let (along, up) = reading_axes(face);
        let on_screen = project(centre);
        Self {
            centre: on_screen,
            along: project(centre + along * 0.5) - on_screen,
            up: project(centre + up * 0.5) - on_screen,
        }
    }

    /// Where a point of the screen stands on the face: `(along, up)`, each
    /// from -1 to 1 inside it.
    fn on_face(&self, point: egui::Pos2) -> (f32, f32) {
        let from_centre = point - self.centre;
        let determinant = self.along.x * self.up.y - self.along.y * self.up.x;
        (
            (from_centre.x * self.up.y - from_centre.y * self.up.x) / determinant,
            (self.along.x * from_centre.y - self.along.y * from_centre.x) / determinant,
        )
    }
}

fn on_the_cube_s_screen(state: &ViewportState, rect: egui::Rect, point: Vec3) -> egui::Pos2 {
    to_screen(
        point.as_dvec3(),
        cube::view_projection(state.camera.rotation()),
        rect,
    )
    .expect("the cube's camera is orthographic, and nothing stands behind it")
}

/// The face a mesh is painted on: the one whose centre, among all six, stands
/// nearest the middle of the mesh. A name painted on a hidden face would find
/// that hidden face here, so it is caught rather than given to its neighbour.
fn face_under(state: &ViewportState, rect: egui::Rect, mesh: &egui::Mesh) -> CubeFace {
    let middle = mesh
        .vertices
        .iter()
        .fold(egui::Vec2::ZERO, |sum, vertex| sum + vertex.pos.to_vec2())
        / mesh.vertices.len() as f32;
    CubeFace::ALL
        .into_iter()
        .min_by(|one, other| {
            let distance = |face: CubeFace| {
                (on_the_cube_s_screen(state, rect, cube::face_center(face)).to_vec2() - middle)
                    .length()
            };
            distance(*one).total_cmp(&distance(*other))
        })
        .expect("a cube has faces")
}

/// Every name the cube carries in this view, with the face it lies on: each is
/// one `Shape::Mesh` on the font texture, which is what tells it apart from
/// anything else the painter may lay.
fn names_painted(state: &ViewportState, rect: egui::Rect) -> Vec<(CubeFace, Arc<egui::Mesh>)> {
    let lang = Catalogue::french();
    shapes(|ui| paint_face_labels(ui, state, rect, &lang))
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Mesh(mesh) if mesh.texture_id == egui::TextureId::default() => {
                Some((face_under(state, rect, &mesh), mesh))
            }
            _ => None,
        })
        .collect()
}

/// A name's glyphs, in reading order, as the four corners egui lays for each.
fn quads(mesh: &egui::Mesh) -> Vec<&[egui::epaint::Vertex]> {
    mesh.vertices
        .as_chunks::<4>()
        .0
        .iter()
        .map(<[egui::epaint::Vertex; 4]>::as_slice)
        .collect()
}

/// A word laid out flat in `family`, large enough for its proportions to be
/// the font's own rather than a pixel grid's.
fn flat(word: &str, family: egui::FontFamily) -> Arc<egui::Galley> {
    let galley = RefCell::new(None);
    shapes(|ui| {
        *galley.borrow_mut() = Some(ui.painter().layout_no_wrap(
            word.to_string(),
            egui::FontId::new(64.0, family.clone()),
            egui::Color32::WHITE,
        ));
    });
    galley.into_inner().expect("the pass laid the word out")
}

/// A word laid out flat, as the horizontal position of each corner of each
/// glyph.
fn laid_out(word: &str, family: egui::FontFamily) -> Vec<Vec<f32>> {
    flat(word, family).rows[0]
        .row
        .visuals
        .mesh
        .vertices
        .as_chunks::<4>()
        .0
        .iter()
        .map(|glyph| glyph.iter().map(|vertex| vertex.pos.x).collect())
        .collect()
}

/// Where each glyph starts, as a share of the whole word from its first
/// glyph's start to its last glyph's end: the same for a word however large it
/// is drawn, and different for another word or another family.
fn rhythm_of(glyphs: &[Vec<f32>]) -> Vec<f32> {
    let low = |glyph: &Vec<f32>| glyph.iter().copied().fold(f32::INFINITY, f32::min);
    let high = |glyph: &Vec<f32>| glyph.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let start = glyphs.first().map_or(0.0, low);
    let end = glyphs.last().map_or(1.0, high);
    glyphs
        .iter()
        .map(|glyph| (low(glyph) - start) / (end - start))
        .collect()
}

fn largest_gap(one: &[f32], other: &[f32]) -> f32 {
    one.iter()
        .zip(other)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f32::max)
}

fn extent(values: impl Iterator<Item = f32> + Clone) -> f32 {
    values.clone().fold(f32::NEG_INFINITY, f32::max) - values.fold(f32::INFINITY, f32::min)
}

fn word_proportions(word: &str) -> f32 {
    let galley = flat(word, egui::FontFamily::Proportional);
    let vertices = &galley.rows[0].row.visuals.mesh.vertices;
    extent(vertices.iter().map(|vertex| vertex.pos.x))
        / extent(vertices.iter().map(|vertex| vertex.pos.y))
}

/// Views from every side of the cube, from above and from under it.
fn views_all_around() -> Vec<(f32, f32)> {
    let mut views = Vec::new();
    for yaw in [-2.8, -2.2, -1.2, -0.4, 0.5, 1.3, 2.1, 3.0] {
        for pitch in [-1.3, -0.7, -0.2, 0.3, 0.8, 1.3] {
            views.push((yaw, pitch));
        }
    }
    views
}

#[test]
fn a_face_s_name_is_painted_lying_on_its_face_and_turns_with_the_cube() {
    let lang = Catalogue::french();
    let mut top_edges = Vec::new();

    for (yaw, pitch) in [(opening_yaw(), opening_pitch()), (-1.9, 0.5), (0.6, 0.9)] {
        let (state, rect) = a_cube_seen_from(yaw, pitch, ROOMY_CUBE);

        let names = names_painted(&state, rect);

        assert!(
            !names.is_empty(),
            "seen from yaw {yaw}, pitch {pitch}, no name of the cube is painted as a mesh \
             lying on its face",
        );
        for (face, mesh) in &names {
            let map = FaceMap::of(&state, rect, *face);
            let seen = format!("the name on {face:?}, from yaw {yaw}, pitch {pitch}");
            assert!(
                mesh.vertices
                    .iter()
                    .all(|vertex| (0.0..=1.0).contains(&vertex.uv.x)
                        && (0.0..=1.0).contains(&vertex.uv.y)),
                "{seen} keeps a galley's coordinates in texels: a `Shape::Mesh` samples the \
                 font texture from 0 to 1",
            );
            for glyph in quads(mesh) {
                for one in glyph {
                    for other in glyph {
                        let (one_along, one_up) = map.on_face(one.pos);
                        let (other_along, other_up) = map.on_face(other.pos);
                        if one.uv.y == other.uv.y {
                            assert!(
                                (one_up - other_up).abs() < SQUARE_ON_THE_FACE,
                                "{seen} is not laid along the face: two corners of a glyph \
                                 on one line of its texture stand {one_up} and {other_up} up it",
                            );
                        }
                        if one.uv.x == other.uv.x {
                            assert!(
                                (one_along - other_along).abs() < SQUARE_ON_THE_FACE,
                                "{seen} is not laid along the face: two corners of a glyph \
                                 on one column of its texture stand {one_along} and \
                                 {other_along} along it",
                            );
                        }
                    }
                }
            }
            let word = crate::wording::cube::face(&lang, *face);
            let spots: Vec<(f32, f32)> = mesh
                .vertices
                .iter()
                .map(|vertex| map.on_face(vertex.pos))
                .collect();
            let proportions =
                extent(spots.iter().map(|spot| spot.0)) / extent(spots.iter().map(|spot| spot.1));
            let wanted = word_proportions(&word);
            assert!(
                (proportions / wanted - 1.0).abs() <= SAME_PROPORTIONS,
                "{seen} is {proportions} times as wide as high on the face, and {word:?} is \
                 {wanted} times as wide as high: the map squeezes the text",
            );
        }
        if let Some((_, mesh)) = names.iter().find(|(face, _)| *face == CubeFace::PlusZ) {
            let glyph = quads(mesh)[0];
            let left = glyph.iter().min_by(|a, b| a.uv.x.total_cmp(&b.uv.x));
            let right = glyph.iter().max_by(|a, b| a.uv.x.total_cmp(&b.uv.x));
            if let (Some(left), Some(right)) = (left, right) {
                top_edges.push((right.pos - left.pos).angle());
            }
        }
    }

    assert!(
        top_edges.len() >= 2 && (top_edges[0] - top_edges[1]).abs() > 0.1,
        "orbiting the view leaves the name on the top face at the same angle on screen, \
         {top_edges:?}: it does not turn with the cube",
    );
}

#[test]
fn a_side_face_s_name_reads_upright_and_none_is_mirrored() {
    let mut named = Vec::new();

    for (yaw, pitch) in views_all_around() {
        let (state, rect) = a_cube_seen_from(yaw, pitch, ROOMY_CUBE);
        for (face, mesh) in names_painted(&state, rect) {
            let map = FaceMap::of(&state, rect, face);
            let seen = format!("the name on {face:?}, from yaw {yaw}, pitch {pitch}");
            let glyphs = quads(&mesh);
            let (first, last) = (glyphs[0], glyphs[glyphs.len() - 1]);
            assert!(
                map.on_face(first[0].pos).0 < map.on_face(last[0].pos).0,
                "{seen} runs backwards: its first letter stands after its last along the \
                 reading direction {:?}",
                reading_axes(face).0,
            );
            for glyph in glyphs {
                let by = |key: fn(&egui::epaint::Vertex) -> f32| {
                    let low = glyph
                        .iter()
                        .min_by(|a, b| key(a).total_cmp(&key(b)))
                        .expect("a glyph has corners");
                    let high = glyph
                        .iter()
                        .max_by(|a, b| key(a).total_cmp(&key(b)))
                        .expect("a glyph has corners");
                    (map.on_face(low.pos), map.on_face(high.pos))
                };
                let (left, right) = by(|vertex| vertex.uv.x);
                assert!(
                    left.0 < right.0,
                    "{seen} is mirrored: the right of a glyph stands to the left of its \
                     left along the reading direction",
                );
                let (top, bottom) = by(|vertex| vertex.uv.y);
                assert!(
                    top.1 > bottom.1,
                    "{seen} is upside down: the top of a glyph stands below its foot",
                );
            }
            if !named.contains(&face) {
                named.push(face);
            }
        }
    }

    assert_eq!(
        named.len(),
        6,
        "across every side of the cube, only {named:?} ever carried a name painted on it",
    );
}

#[test]
fn a_face_seen_too_edge_on_to_read_draws_no_name() {
    for (yaw, pitch) in [(-2.2, 0.015), (0.015, 0.5)] {
        let (state, rect) = a_cube_seen_from(yaw, pitch, ROOMY_CUBE);
        let forward = state.camera.forward();
        let edge_on: Vec<CubeFace> = CubeFace::ALL
            .into_iter()
            .filter(|face| {
                cube::is_visible(*face, forward) && face.normal().dot(forward).abs() < 0.03
            })
            .collect();
        assert!(
            !edge_on.is_empty(),
            "the fixture, from yaw {yaw}, pitch {pitch}, was meant to leave a face all but \
             edge-on",
        );

        let names = names_painted(&state, rect);

        assert!(
            !names.is_empty(),
            "seen from yaw {yaw}, pitch {pitch}, the faces square to the view carry no name \
             painted on them",
        );
        for (face, _) in &names {
            assert!(
                !edge_on.contains(face),
                "from yaw {yaw}, pitch {pitch}, {face:?} is seen edge-on and still carries \
                 a name nobody can read",
            );
        }
    }
}

#[test]
fn the_name_on_a_lit_face_stays_readable_against_the_amber() {
    let (state, rect) = a_cube_seen_from(opening_yaw(), opening_pitch(), ROOMY_CUBE);
    let amber = srgb_luminance(tokens::ACCENT_AMBER);

    let names = names_painted(&state, rect);

    assert!(
        !names.is_empty(),
        "no name of the cube is painted as a mesh on its face",
    );
    for (face, mesh) in &names {
        let ink = mesh.vertices[0].color;
        let ink_luminance = srgb_luminance(ink);
        let contrast = contrast(amber.max(ink_luminance), amber.min(ink_luminance));
        assert!(
            contrast >= READABLE_CONTRAST,
            "the name on {face:?} is inked {ink:?}, a contrast of {contrast:.2}:1 on the \
             amber of a lit face, short of {READABLE_CONTRAST}:1",
        );
    }
}

/// The relative luminance of an opaque colour, as WCAG defines it.
fn srgb_luminance(colour: egui::Color32) -> f32 {
    let [r, g, b, _] = colour.to_srgba_unmultiplied();
    let linear = |channel: u8| {
        let channel = f32::from(channel) / 255.0;
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
}

#[test]
fn a_hovered_face_s_name_keeps_the_title_colour() {
    let (mut state, rect) = a_cube_seen_from(opening_yaw(), opening_pitch(), ROOMY_CUBE);
    let hovered = CubeFace::PlusZ;
    state.hovered_zone = Some(CubeZone::Face(hovered));

    let names = names_painted(&state, rect);

    assert!(
        names.iter().any(|(face, _)| *face == hovered),
        "no name is painted on {hovered:?}, the face under the cursor",
    );
    for (face, mesh) in &names {
        let wanted = if *face == hovered {
            tokens::TEXT_TITLE
        } else {
            tokens::TEXT_ON_ACCENT
        };
        assert!(
            mesh.vertices.iter().all(|vertex| vertex.color == wanted),
            "the name on {face:?}, {hovered:?} under the cursor, is inked {:?}, not {wanted:?}",
            mesh.vertices[0].color,
        );
    }
}

#[test]
fn a_name_lying_on_its_face_never_leaves_it() {
    for size in [96.0, 128.0, 200.0] {
        let (state, rect) = a_cube_seen_from(opening_yaw(), opening_pitch(), size);
        assert!(
            !names_painted(&state, rect).is_empty(),
            "a cube {size} wide at the opening view paints no name as a mesh on any face",
        );

        for (yaw, pitch) in views_all_around() {
            let (state, rect) = a_cube_seen_from(yaw, pitch, size);
            for (face, mesh) in names_painted(&state, rect) {
                let map = FaceMap::of(&state, rect, face);
                for vertex in &mesh.vertices {
                    let (along, up) = map.on_face(vertex.pos);
                    assert!(
                        along.abs() <= 1.0 + INSIDE_ITS_FACE && up.abs() <= 1.0 + INSIDE_ITS_FACE,
                        "on a cube {size} wide seen from yaw {yaw}, pitch {pitch}, the name on \
                         {face:?} reaches ({along}, {up}) of its face, where -1 to 1 is inside",
                    );
                }
            }
        }
    }
}

const THE_AXIS_X: Rgba = Rgba::opaque(0.90, 0.10, 0.20);
const THE_AXIS_Y: Rgba = Rgba::opaque(0.10, 0.80, 0.30);
const THE_AXIS_Z: Rgba = Rgba::opaque(0.15, 0.25, 0.95);

/// How far a letter's middle may stand from the end of its line.
const AT_THE_TIP: f32 = 16.0;

/// The views the triad is read from, one of them square down the Z axis.
fn triad_views() -> [(f32, f32); 4] {
    [
        (opening_yaw(), opening_pitch()),
        (0.6, 0.9),
        (-2.0, -0.8),
        (0.3, 1.5),
    ]
}

struct AxisDrawn {
    from: egui::Pos2,
    to: egui::Pos2,
    width: f32,
    letter: egui::Rect,
    letter_size: f32,
}

fn a_cube_with_its_axes_painted_apart(
    yaw: f32,
    pitch: f32,
    size: f32,
    at: egui::Pos2,
) -> (ViewportState, egui::Rect) {
    let (mut state, _) = a_cube_seen_from(yaw, pitch, size);
    state.theme.axis_x = THE_AXIS_X;
    state.theme.axis_y = THE_AXIS_Y;
    state.theme.axis_z = THE_AXIS_Z;
    (
        state,
        egui::Rect::from_min_size(at, egui::Vec2::splat(size)),
    )
}

/// The world axis of each line of the triad, and the two faces of the cube its
/// edge separates.
const THE_AXES: [(Vec3, &str, [CubeFace; 2]); 3] = [
    (Vec3::X, "X", [CubeFace::MinusY, CubeFace::MinusZ]),
    (Vec3::Y, "Y", [CubeFace::MinusX, CubeFace::MinusZ]),
    (Vec3::Z, "Z", [CubeFace::MinusX, CubeFace::MinusY]),
];

/// The corner of the cube the three axes start from.
const THE_AXES_CORNER: Vec3 = Vec3::splat(-0.5);

/// How far a face has to turn past edge-on to be seen, and how close to the
/// line of sight an axis is end-on: the views these tests read are exactly
/// so, or degrees away from it.
const EDGE_ON: f32 = 1e-3;

/// The triad as painted: for each axis, the line drawn in its colour and the
/// letter written in that same colour, or nothing when neither is drawn.
fn the_triad(state: &ViewportState, rect: egui::Rect) -> [Option<AxisDrawn>; 3] {
    let lang = Catalogue::french();
    let painted = shapes(|ui| paint_face_labels(ui, state, rect, &lang));

    [("X", THE_AXIS_X), ("Y", THE_AXIS_Y), ("Z", THE_AXIS_Z)].map(|(name, colour)| {
        let colour = tint_to_color(colour);
        let line = painted.iter().find_map(|shape| match shape {
            egui::Shape::LineSegment { points, stroke } if stroke.color == colour => {
                Some((*points, stroke.width))
            }
            _ => None,
        });
        let text = painted.iter().find_map(|shape| match shape {
            egui::Shape::Text(text) if text.galley.text() == name => Some(text),
            _ => None,
        });
        match (line, text) {
            (None, None) => None,
            (Some(_), None) => panic!("the line of axis {name} ends on no letter {name}"),
            (None, Some(_)) => panic!("the letter {name} is written with no line to end"),
            (Some(([from, to], width)), Some(text)) => {
                assert_eq!(
                    ink(text),
                    colour,
                    "the letter {name} is not written in the colour of its axis",
                );
                Some(AxisDrawn {
                    from,
                    to,
                    width,
                    letter: egui::Shape::Text(text.clone()).visual_bounding_rect(),
                    letter_size: text.galley.job.sections[0].format.font_id.size,
                })
            }
        }
    })
}

/// Whether the axis at `index` is meant to be drawn from a view looking along
/// `forward`. Never end-on; otherwise when one of the two faces its edge
/// separates is strictly seen, or when another axis is end-on, the view then
/// being square onto a plane this axis lies in.
fn is_shown(index: usize, forward: Vec3) -> bool {
    let end_on = |axis: Vec3| axis.dot(forward).abs() >= 1.0 - EDGE_ON;
    let (axis, _, faces) = THE_AXES[index];
    if end_on(axis) {
        return false;
    }
    let square_onto_its_plane = THE_AXES
        .iter()
        .any(|(other, _, _)| *other != axis && end_on(*other));
    square_onto_its_plane
        || faces
            .iter()
            .any(|face| face.normal().dot(forward) < -EDGE_ON)
}

/// That the line of the axis at `index` starts at the cube's corner, passes
/// through the far end of its edge and goes on past it by a little.
fn assert_runs_along_its_edge(
    state: &ViewportState,
    rect: egui::Rect,
    index: usize,
    drawn: &AxisDrawn,
    seen: &str,
) {
    let (axis, name, _) = THE_AXES[index];
    let start = on_the_cube_s_screen(state, rect, THE_AXES_CORNER);
    let far = on_the_cube_s_screen(state, rect, THE_AXES_CORNER + axis);
    assert!(
        (drawn.from - start).length() <= ON_THE_POINT,
        "{seen}: the line of axis {name} starts at {:?}, and the cube's corner is at {start:?}",
        drawn.from,
    );
    let along = (drawn.to - drawn.from).normalized();
    let off_the_line = (far - drawn.from) - along * (far - drawn.from).dot(along);
    assert!(
        off_the_line.length() <= ON_THE_POINT,
        "{seen}: the line of axis {name}, from {:?} to {:?}, misses the far end of its edge \
         at {far:?} by {} points",
        drawn.from,
        drawn.to,
        off_the_line.length(),
    );
    let overshoot = (drawn.to - far).dot(along);
    let edge = (far - start).length();
    assert!(
        overshoot > 0.0 && overshoot <= PAST_THE_FAR_CORNER * edge,
        "{seen}: the line of axis {name} goes {overshoot} points past the far end of its \
         edge, {edge} long, where more than none and at most {} is asked",
        PAST_THE_FAR_CORNER * edge,
    );
}

/// How far a line may stand from a point it is meant to reach, in points.
const ON_THE_POINT: f32 = 0.5;

/// How far past the far corner of its edge a line may go, as a share of the
/// edge.
const PAST_THE_FAR_CORNER: f32 = 0.35;

#[test]
fn a_triad_stands_in_the_cube_s_corner_in_the_axis_colours() {
    for size in [80.0, 96.0, 160.0] {
        for at in [egui::pos2(40.0, 40.0), egui::pos2(900.0, 500.0)] {
            for (yaw, pitch) in triad_views() {
                for mode in [ViewMode::Free, ViewMode::Plane(WorkPlane::XY)] {
                    let (mut state, rect) =
                        a_cube_with_its_axes_painted_apart(yaw, pitch, size, at);
                    state.mode = mode;
                    let seen =
                        format!("a cube {size} wide at {at:?}, yaw {yaw}, pitch {pitch}, {mode:?}");
                    let corner = on_the_cube_s_screen(&state, rect, THE_AXES_CORNER);

                    let triad = the_triad(&state, rect);

                    assert!(
                        triad.iter().any(Option::is_some),
                        "{seen}: not one axis of the triad is drawn",
                    );
                    for axis in triad.iter().flatten() {
                        assert!(
                            (axis.from - corner).length() <= ON_THE_POINT,
                            "{seen}: a line of the triad starts at {:?}, and the cube's corner \
                             it should start from is at {corner:?}",
                            axis.from,
                        );
                        assert!(
                            rect.contains(axis.from) && rect.contains(axis.to),
                            "{seen}: a line of the triad leaves the cube's square {rect:?}",
                        );
                        assert!(
                            (axis.letter.center() - axis.to).length() <= AT_THE_TIP,
                            "{seen}: a letter stands {} from the end of its line",
                            (axis.letter.center() - axis.to).length(),
                        );
                        assert!(
                            rect.contains_rect(axis.letter),
                            "{seen}: the letter at {:?} leaves the cube's square {rect:?}",
                            axis.letter,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn the_triad_turns_with_the_camera() {
    for (yaw, pitch) in triad_views() {
        let (state, rect) =
            a_cube_with_its_axes_painted_apart(yaw, pitch, ROOMY_CUBE, egui::pos2(40.0, 40.0));
        let camera = &state.camera;

        let triad = the_triad(&state, rect);

        let lines: Vec<(egui::Vec2, egui::Vec2, &str)> = triad
            .iter()
            .zip(THE_AXES)
            .filter_map(|(axis, (world, name, _))| {
                axis.as_ref().map(|axis| {
                    (
                        axis.to - axis.from,
                        egui::vec2(camera.right().dot(world), -camera.up().dot(world)),
                        name,
                    )
                })
            })
            .collect();
        assert!(
            !lines.is_empty(),
            "from yaw {yaw}, pitch {pitch}, no axis is drawn"
        );
        let scale = lines
            .iter()
            .map(|(line, _, _)| line.length())
            .fold(0.0, f32::max)
            / lines
                .iter()
                .map(|(_, wanted, _)| wanted.length())
                .fold(0.0, f32::max);
        for (line, wanted, name) in lines {
            assert!(
                (line - wanted * scale).length() <= 1.0,
                "from yaw {yaw}, pitch {pitch}, the line of axis {name} runs {line:?}, and \
                 its world axis projects to {:?} at the triad's scale",
                wanted * scale,
            );
        }
    }

    let (state, rect) =
        a_cube_with_its_axes_painted_apart(0.3, 1.5, ROOMY_CUBE, egui::pos2(40.0, 40.0));
    let triad = the_triad(&state, rect);
    let length = |axis: &AxisDrawn| (axis.to - axis.from).length();
    let longest = triad.iter().flatten().map(length).fold(0.0, f32::max);
    let z = triad[2]
        .as_ref()
        .expect("looking nearly down Z, « FACE » is seen, so its edge is drawn");
    assert!(
        longest > 0.0 && length(z) < longest * 0.25,
        "looking nearly down the Z axis, its line is {} long and the longest {longest}: an axis \
         seen nearly end-on is short",
        length(z),
    );
}

fn along_a_line(from: egui::Pos2, to: egui::Pos2) -> Vec<egui::Pos2> {
    (0..=24)
        .map(|step| from + (to - from) * (step as f32 / 24.0))
        .collect()
}

/// A square onto each of the three planes of the origin, as opening a sketch
/// on it lands, and the opening view.
fn the_views_the_triad_is_read_in(size: f32) -> Vec<(String, ViewportState, egui::Rect)> {
    let at = egui::pos2(40.0, 40.0);
    let (opening, rect) =
        a_cube_with_its_axes_painted_apart(opening_yaw(), opening_pitch(), size, at);
    let mut views = vec![("the opening view".to_string(), opening, rect)];
    for (name, plane) in [
        ("a sketch on XY", WorkPlane::XY),
        ("a sketch on XZ", WorkPlane::XZ),
        ("a sketch on YZ", WorkPlane::YZ),
    ] {
        let (mut state, rect) =
            a_cube_with_its_axes_painted_apart(opening_yaw(), opening_pitch(), size, at);
        state.look_at_plane(plane, DVec3::ZERO, 50.0);
        if let Some(mut turn) = state.transition.take() {
            while turn.advance(&mut state.camera, 1.0) {}
        }
        views.push((name.to_string(), state, rect));
    }
    views
}

#[test]
fn the_triad_runs_along_the_cube_s_edges_past_their_far_corners() {
    for size in [80.0, 96.0, 160.0] {
        let (state, rect) = a_cube_with_its_axes_painted_apart(
            opening_yaw(),
            opening_pitch(),
            size,
            egui::pos2(40.0, 40.0),
        );
        let seen = format!("on a cube {size} wide at the opening view");

        let triad = the_triad(&state, rect);

        for (index, name) in [(0, "X"), (2, "Z")] {
            let drawn = triad[index]
                .as_ref()
                .unwrap_or_else(|| panic!("{seen}: the axis {name} is not drawn"));
            assert_runs_along_its_edge(&state, rect, index, drawn, &seen);
        }
        assert!(
            triad[1].is_none(),
            "{seen}: the axis Y runs behind the cube and is drawn",
        );
    }
}

#[test]
fn an_axis_is_drawn_exactly_when_a_face_its_edge_bounds_is_seen() {
    for yaw in (0..360).step_by(15) {
        for pitch in (-75..=75).step_by(15) {
            let (state, rect) = a_cube_with_its_axes_painted_apart(
                (yaw as f32).to_radians(),
                (pitch as f32).to_radians(),
                96.0,
                egui::pos2(40.0, 40.0),
            );
            let forward = state.camera.forward();

            let triad = the_triad(&state, rect);

            for (index, (_, name, faces)) in THE_AXES.into_iter().enumerate() {
                assert_eq!(
                    triad[index].is_some(),
                    is_shown(index, forward),
                    "from yaw {yaw}°, pitch {pitch}°, looking along {forward:?}, the axis {name} \
                     separates {faces:?}",
                );
            }
        }
    }
}

/// Every zone of the cube a click snaps the view to: its faces, its edges and
/// its corners.
fn every_zone_of_the_cube() -> Vec<CubeZone> {
    let square = |one: CubeFace, other: CubeFace| one.normal().dot(other.normal()).abs() < 0.5;
    let mut zones: Vec<CubeZone> = CubeFace::ALL.map(CubeZone::Face).to_vec();
    for one in CubeFace::ALL {
        for other in CubeFace::ALL
            .into_iter()
            .filter(|other| square(one, *other))
        {
            let edge = CubeZone::edge(one, other);
            if !zones.contains(&edge) {
                zones.push(edge);
            }
            for third in CubeFace::ALL
                .into_iter()
                .filter(|third| square(one, *third) && square(other, *third))
            {
                let corner = CubeZone::corner(one, other, third);
                if !zones.contains(&corner) {
                    zones.push(corner);
                }
            }
        }
    }
    zones
}

/// Whether two lines lie on one line and share a stretch of it.
fn run_along_each_other(one: &AxisDrawn, other: &AxisDrawn) -> bool {
    let along = (one.to - one.from).normalized();
    let across =
        |point: egui::Pos2| ((point - one.from) - along * (point - one.from).dot(along)).length();
    let reach = |point: egui::Pos2| (point - one.from).dot(along);
    let (low, high) = (
        reach(other.from).min(reach(other.to)),
        reach(other.from).max(reach(other.to)),
    );
    across(other.from) <= ON_THE_POINT
        && across(other.to) <= ON_THE_POINT
        && high.min((one.to - one.from).length()) > low.max(0.0)
}

#[test]
fn at_every_view_the_cube_snaps_to_no_two_axes_overprint() {
    let zones = every_zone_of_the_cube();
    assert_eq!(
        zones.len(),
        26,
        "a cube has 6 faces, 12 edges and 8 corners"
    );

    let mut overprinted = Vec::new();
    for size in [96.0, 160.0] {
        for zone in &zones {
            let (yaw, pitch) = cao_render::camera::view_angles_towards(zone.direction());
            let (state, rect) =
                a_cube_with_its_axes_painted_apart(yaw, pitch, size, egui::pos2(40.0, 40.0));

            let triad = the_triad(&state, rect);

            for first in 0..3 {
                for second in first + 1..3 {
                    let (Some(one), Some(other)) = (&triad[first], &triad[second]) else {
                        continue;
                    };
                    let names = (THE_AXES[first].1, THE_AXES[second].1);
                    if one.letter.intersects(other.letter) {
                        overprinted.push(format!("{size} {zone:?}: letters {names:?}"));
                    }
                    if run_along_each_other(one, other) {
                        overprinted.push(format!("{size} {zone:?}: lines {names:?}"));
                    }
                }
            }
        }
    }

    assert!(
        overprinted.is_empty(),
        "two axes of the triad are drawn over each other at {} views the cube snaps to: \
         {overprinted:?}",
        overprinted.len(),
    );
}

#[test]
fn the_top_front_edge_view_shows_x_and_z_and_not_y() {
    let edge = CubeZone::edge(CubeFace::PlusZ, CubeFace::MinusY);
    let (yaw, pitch) = cao_render::camera::view_angles_towards(edge.direction());
    let (state, rect) =
        a_cube_with_its_axes_painted_apart(yaw, pitch, 96.0, egui::pos2(40.0, 40.0));

    let triad = the_triad(&state, rect);

    assert_eq!(
        triad.iter().map(Option::is_some).collect::<Vec<_>>(),
        vec![true, false, true],
        "the view clicked on the edge between « DESSUS » and « FACE » should draw X and Z, \
         and not Y, which would lie on Z",
    );
}

#[test]
fn from_the_back_corner_no_axis_is_drawn() {
    let (yaw, pitch) = cao_render::camera::view_angles_towards(Vec3::ONE);
    let (state, rect) =
        a_cube_with_its_axes_painted_apart(yaw, pitch, 96.0, egui::pos2(40.0, 40.0));

    let triad = the_triad(&state, rect);

    assert!(
        triad.iter().all(Option::is_none),
        "seen from the corner (+X, +Y, +Z), every edge of the triad is behind the cube, and \
         the axes drawn are {:?}",
        triad.iter().map(Option::is_some).collect::<Vec<_>>(),
    );
}

#[test]
fn square_onto_a_sketch_two_axes_show_along_the_outline_and_the_third_not() {
    for (name, wanted) in [
        ("a sketch on XY", [true, true, false]),
        ("a sketch on XZ", [true, false, true]),
        ("a sketch on YZ", [false, true, true]),
    ] {
        let (_, state, rect) = the_views_the_triad_is_read_in(96.0)
            .into_iter()
            .find(|(view, _, _)| view == name)
            .expect("the views the triad is read in hold the three sketches");

        let triad = the_triad(&state, rect);

        for (index, (_, axis, _)) in THE_AXES.into_iter().enumerate() {
            assert_eq!(
                triad[index].is_some(),
                wanted[index],
                "at {name}, the axis {axis} should be drawn: {}",
                wanted[index],
            );
            if let Some(drawn) = &triad[index] {
                assert_runs_along_its_edge(&state, rect, index, drawn, name);
            }
        }
    }
}

#[test]
fn a_letter_stands_beyond_its_tip_inside_the_square_and_off_the_names() {
    for size in [80.0, 96.0, 160.0] {
        for (view, state, rect) in the_views_the_triad_is_read_in(size) {
            let names: Vec<egui::Rect> = names_painted(&state, rect)
                .iter()
                .map(|(_, mesh)| mesh.calc_bounds())
                .collect();
            let seen = format!("on a cube {size} wide at {view}");

            let triad = the_triad(&state, rect);

            for (axis, (_, name, _)) in triad.iter().zip(THE_AXES) {
                let Some(axis) = axis else { continue };
                let along = (axis.to - axis.from).normalized();
                assert!(
                    (axis.letter.center() - axis.to).dot(along) > 0.0,
                    "{seen}: the letter {name} at {:?} stands before the tip of its line at \
                     {:?}",
                    axis.letter,
                    axis.to,
                );
                assert!(
                    rect.contains_rect(axis.letter),
                    "{seen}: the letter {name} at {:?} leaves the cube's square {rect:?}",
                    axis.letter,
                );
                for word in &names {
                    assert!(
                        !along_a_line(axis.from, axis.to)
                            .into_iter()
                            .any(|point| word.contains(point)),
                        "{seen}: the line of axis {name} crosses the face name at {word:?}",
                    );
                    assert!(
                        !axis.letter.intersects(*word),
                        "{seen}: the letter {name} at {:?} touches the face name at {word:?}",
                        axis.letter,
                    );
                }
            }
        }
    }
}

#[test]
fn the_triad_is_drawn_large_enough_to_read_on_a_cube_of_the_default_size() {
    let size = ViewportConfig::default().cube_size;
    let (state, rect) = a_cube_with_its_axes_painted_apart(
        opening_yaw(),
        opening_pitch(),
        size,
        egui::pos2(40.0, 40.0),
    );
    let names_size = (size * 0.11).max(cube_labels::MINIMUM_READABLE_FONT);

    let triad = the_triad(&state, rect);

    assert!(
        triad.iter().any(Option::is_some),
        "at the opening view no axis is drawn",
    );
    for (axis, (_, name, _)) in triad.iter().zip(THE_AXES) {
        let Some(axis) = axis else { continue };
        assert!(
            axis.width >= THICK_ENOUGH,
            "the line of axis {name} is {} wide, where {THICK_ENOUGH} is the least that reads",
            axis.width,
        );
        assert!(
            axis.letter_size >= LETTER_OVER_NAME * names_size,
            "the letter {name} is written at {}, where the face names are at {names_size} and \
             it should stand at least {LETTER_OVER_NAME} times that",
            axis.letter_size,
        );
    }
}

/// How large the triad is drawn: a line, and a letter against a face name.
const THICK_ENOUGH: f32 = 2.0;
const LETTER_OVER_NAME: f32 = 1.2;

#[test]
fn every_name_reads_upright_at_the_opening_view() {
    let (state, rect) = a_cube_seen_from(opening_yaw(), opening_pitch(), ROOMY_CUBE);

    let names = names_painted(&state, rect);

    assert!(!names.is_empty(), "no name is painted at the opening view");
    for (face, mesh) in &names {
        let glyphs = quads(mesh);
        let (first, last) = (glyphs[0], glyphs[glyphs.len() - 1]);
        let along = last[0].pos - first[0].pos;
        assert!(
            along.x > 0.0,
            "at the opening view, the name on {face:?} runs right to left across the screen",
        );
        let top = glyphs[0]
            .iter()
            .min_by(|a, b| a.uv.y.total_cmp(&b.uv.y))
            .expect("a glyph has corners");
        let foot = glyphs[0]
            .iter()
            .max_by(|a, b| a.uv.y.total_cmp(&b.uv.y))
            .expect("a glyph has corners");
        assert!(
            top.pos.y < foot.pos.y,
            "at the opening view, the name on {face:?} stands upside down: the top of a glyph \
             is below its foot on screen",
        );
    }
}

#[test]
fn a_cube_too_small_to_hold_a_letter_paints_without_a_panic() {
    let lang = Catalogue::french();

    for size in [5.0, 1.0] {
        for (yaw, pitch) in triad_views() {
            let (state, rect) = a_cube_seen_from(yaw, pitch, size);

            shapes(|ui| paint_face_labels(ui, &state, rect, &lang));
        }
    }
}

/// How far the ends of a line may move between two views a degree apart, in
/// points, and how far a letter may: about what a point of the cube's corner
/// sweeps in a degree on the larger cube, with margin.
const A_DEGREE_S_SWEEP: f32 = 1.5;
const A_LETTER_S_SWEEP: f32 = 2.5;

/// What is read of each axis at a view: the two ends of its line and the
/// middle of its letter, or nothing when the axis is not shown.
fn the_axes_at(size: f32, (yaw, pitch): (f32, f32)) -> [Option<[egui::Pos2; 3]>; 3] {
    let (state, rect) = a_cube_with_its_axes_painted_apart(
        yaw.to_radians(),
        pitch.to_radians(),
        size,
        egui::pos2(40.0, 40.0),
    );
    the_triad(&state, rect).map(|axis| axis.map(|axis| [axis.from, axis.to, axis.letter.center()]))
}

#[test]
fn the_triad_does_not_jump_as_the_camera_turns() {
    let mut sweeps: Vec<Vec<(f32, f32)>> = Vec::new();
    for pitch in [20.0, 35.0, 60.0, 89.0] {
        sweeps.push((0..=360).map(|yaw| (yaw as f32, pitch)).collect());
    }
    for yaw in [0.0, 45.0, 100.0, 215.0] {
        sweeps.push((-90..=90).map(|pitch| (yaw, pitch as f32)).collect());
    }

    let mut worst = (0.0_f32, String::new());
    let mut jumps = 0;
    for size in [96.0, 160.0] {
        for views in &sweeps {
            let read: Vec<_> = views.iter().map(|view| the_axes_at(size, *view)).collect();
            for (at, two) in read.windows(2).enumerate() {
                for (index, (_, name, _)) in THE_AXES.into_iter().enumerate() {
                    let (Some(one), Some(other)) = (&two[0][index], &two[1][index]) else {
                        continue;
                    };
                    let parts = [
                        ("its start", one[0], other[0], A_DEGREE_S_SWEEP),
                        ("its tip", one[1], other[1], A_DEGREE_S_SWEEP),
                        ("its letter's middle", one[2], other[2], A_LETTER_S_SWEEP),
                    ];
                    for (part, there, here, limit) in parts {
                        let moved = (here - there).length();
                        if moved > limit {
                            jumps += 1;
                        }
                        if moved / limit > worst.0 {
                            worst = (
                                moved / limit,
                                format!(
                                    "on a cube {size} wide, {part} of axis {name} moves {moved} \
                                     points between {:?} and {:?} (yaw, pitch in degrees), where \
                                     {limit} is the most",
                                    views[at],
                                    views[at + 1],
                                ),
                            );
                        }
                    }
                }
            }
        }
    }

    assert!(
        worst.0 <= 1.0,
        "{}; {jumps} moves are too large over the sweeps",
        worst.1,
    );
}
