//! What app · ui/step_dot.rs is held to.
//!
//! Closes #738 — the whole transcription is in
//! `crates/app/src/screens/viewport/input/copying/tests/steps.rs`; this is the
//! step dot's share of one criterion.
//! - `ui/` gains the controls it lacks: the step dot (22 px, three states) —
//!   what each state shows, `a_done_dot_shows_a_check_on_green_at_18_percent`,
//!   `the_current_dot_shows_its_number_on_amber`,
//!   `a_dot_to_come_shows_its_number_on_the_neutral_background`; and that it
//!   is drawn so, 22 points across —
//!   `a_step_dot_is_drawn_22_points_across_with_its_face_in_each_state`

use super::*;
use crate::ui::theme::tokens::{
    ACCENT_AMBER, BACKGROUND_ACTIVE_NEUTRAL, STATE_SUCCESS, TEXT_ON_ACCENT,
};

/// How far two colours may drift per channel and still read as one: what
/// multiplying through a premultiplied alpha costs in rounding.
const SAME_COLOUR: u8 = 2;

/// How far a size may stray from the one C13 asks for: shapes are placed on
/// rounded points.
const SIZED_WITHIN: f32 = 0.5;

fn alike(one: egui::Color32, other: egui::Color32) -> bool {
    one.to_array()
        .iter()
        .zip(other.to_array())
        .all(|(a, b)| a.abs_diff(b) <= SAME_COLOUR)
}

#[test]
fn a_done_dot_shows_a_check_on_green_at_18_percent() {
    let [r, g, b, _] = STATE_SUCCESS.to_srgba_unmultiplied();
    let green_at_18_percent = egui::Color32::from_rgba_unmultiplied(r, g, b, 46);

    let face = face(Progress::Done, 2);

    assert_eq!(face.mark, "✓", "a step done shows a check, not its number");
    assert!(
        alike(face.fill, green_at_18_percent),
        "a step done is filled with {:?}, where C13 asks for the success green at 18% \
         opacity, {green_at_18_percent:?}",
        face.fill,
    );
}

#[test]
fn the_current_dot_shows_its_number_on_amber() {
    let face = face(Progress::Current, 3);

    assert_eq!(face.mark, "3");
    assert_eq!(face.fill, ACCENT_AMBER);
    assert_eq!(
        face.ink, TEXT_ON_ACCENT,
        "written on amber, in the ink the primary button writes on it"
    );
}

#[test]
fn a_dot_to_come_shows_its_number_on_the_neutral_background() {
    let face = face(Progress::ToCome, 4);

    assert_eq!(face.mark, "4");
    assert_eq!(face.fill, BACKGROUND_ACTIVE_NEUTRAL);
}

#[test]
fn a_step_dot_is_drawn_22_points_across_with_its_face_in_each_state() {
    for progress in [Progress::Done, Progress::Current, Progress::ToCome] {
        let expected = face(progress, 3);

        let drawn = drawn(progress, 3);

        assert!(
            (drawn.rect.width() - 22.0).abs() < SIZED_WITHIN
                && (drawn.rect.height() - 22.0).abs() < SIZED_WITHIN,
            "{progress:?}: the dot takes {:?}, where C13 draws it 22 across",
            drawn.rect.size(),
        );
        assert!(
            drawn.fills().iter().any(|fill| alike(*fill, expected.fill)),
            "{progress:?}: the dot is painted in {:?}, none of them its face's {:?}",
            drawn.fills(),
            expected.fill,
        );
        assert!(
            drawn.texts().contains(&expected.mark),
            "{progress:?}: the dot writes {:?}, not its face's {:?}",
            drawn.texts(),
            expected.mark,
        );
    }
}

/// A dot as one frame of a themed context paints it, with no window.
struct Drawn {
    rect: egui::Rect,
    shapes: Vec<egui::Shape>,
}

impl Drawn {
    fn fills(&self) -> Vec<egui::Color32> {
        self.shapes
            .iter()
            .filter_map(|shape| match shape {
                egui::Shape::Circle(circle) => Some(circle.fill),
                egui::Shape::Rect(rect) => Some(rect.fill),
                _ => None,
            })
            .collect()
    }

    fn texts(&self) -> Vec<String> {
        self.shapes
            .iter()
            .filter_map(|shape| match shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect()
    }
}

fn drawn(progress: Progress, number: usize) -> Drawn {
    let ctx = egui::Context::default();
    crate::ui::theme::install(&ctx);
    let mut rect = egui::Rect::NOTHING;
    let mut shapes = Vec::new();

    for _ in 0..2 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 200.0),
            )),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            rect = step_dot(ui, number, progress).rect;
        });
        output.textures_delta.clear();
        shapes = every_shape(output.shapes);
    }

    Drawn { rect, shapes }
}

fn every_shape(clipped: Vec<egui::epaint::ClippedShape>) -> Vec<egui::Shape> {
    let mut pending: Vec<egui::Shape> = clipped.into_iter().map(|clipped| clipped.shape).collect();
    let mut shapes = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            shape => shapes.push(shape),
        }
    }
    shapes
}
