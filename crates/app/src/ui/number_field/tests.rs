//! What app · ui/number_field.rs is held to.
//!
//! Closes #738 — the whole transcription is in
//! `crates/app/src/screens/viewport/input/copying/tests/steps.rs`; this is the
//! number field's share of one criterion.
//! - `ui/` gains the controls it lacks: the number field with − / + — what the
//!   two buttons make of what the field holds,
//!   `plus_adds_one_to_the_number_in_the_field`,
//!   `minus_takes_one_off_down_to_the_least_and_no_further`,
//!   `minus_and_plus_leave_a_formula_as_it_was_written`; that a click on one
//!   is handed back as the field changed —
//!   `a_click_on_plus_puts_the_next_number_in_the_field_and_says_it_changed`;
//!   and how it is drawn, 34 points tall on the field background, bordered
//!   amber for the current step's field —
//!   `a_number_field_is_drawn_34_points_tall_on_the_field_background`,
//!   `the_field_of_the_current_step_is_bordered_amber_and_no_other`

use super::*;
use crate::ui::theme::tokens::{ACCENT_AMBER, BACKGROUND_FIELD};

/// How far two colours may drift per channel and still read as one: what
/// multiplying through a premultiplied alpha costs in rounding.
const SAME_COLOUR: u8 = 2;

/// How far a size may stray from the one C14 asks for: shapes are placed on
/// rounded points.
const SIZED_WITHIN: f32 = 0.5;

/// The fewest copies a pattern lays: the original and one more.
const LEAST: f64 = 2.0;

fn alike(one: egui::Color32, other: egui::Color32) -> bool {
    one.to_array()
        .iter()
        .zip(other.to_array())
        .all(|(a, b)| a.abs_diff(b) <= SAME_COLOUR)
}

#[test]
fn plus_adds_one_to_the_number_in_the_field() {
    assert_eq!(nudged("6", Nudge::Plus, LEAST), Some("7".to_string()));
}

#[test]
fn minus_takes_one_off_down_to_the_least_and_no_further() {
    assert_eq!(nudged("6", Nudge::Minus, LEAST), Some("5".to_string()));
    assert_eq!(
        nudged("2", Nudge::Minus, LEAST),
        None,
        "at the least, − changes nothing"
    );
}

#[test]
fn minus_and_plus_leave_a_formula_as_it_was_written() {
    for nudge in [Nudge::Minus, Nudge::Plus] {
        assert_eq!(
            nudged("holes / 2", nudge, LEAST),
            None,
            "{nudge:?} on a formula: what was written is the user's, and stays"
        );
    }
}

#[test]
fn a_click_on_plus_puts_the_next_number_in_the_field_and_says_it_changed() {
    let mut text = "6".to_string();
    let ctx = themed();
    let plus = drawn(&ctx, &mut text, false, Vec::new())
        .written("+")
        .expect("the field draws a + button");

    let mut changed = false;
    for pressed in [true, false] {
        let click = vec![
            egui::Event::PointerMoved(plus),
            egui::Event::PointerButton {
                pos: plus,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            },
        ];
        changed |= drawn(&ctx, &mut text, false, click).changed;
    }

    assert_eq!(text, "7");
    assert!(changed, "the field hands the click back as a change");
}

#[test]
fn a_number_field_is_drawn_34_points_tall_on_the_field_background() {
    let mut text = "6".to_string();

    let drawn = drawn(&themed(), &mut text, false, Vec::new());

    assert!(
        (drawn.rect.height() - 34.0).abs() < SIZED_WITHIN,
        "the field stands {} tall, where C14 draws it 34",
        drawn.rect.height(),
    );
    assert!(
        drawn
            .frames
            .iter()
            .any(|frame| alike(frame.fill, BACKGROUND_FIELD)),
        "the field is filled with {:?}, none of them {BACKGROUND_FIELD:?}",
        drawn
            .frames
            .iter()
            .map(|frame| frame.fill)
            .collect::<Vec<_>>(),
    );
}

#[test]
fn the_field_of_the_current_step_is_bordered_amber_and_no_other() {
    let bordered_amber = |current: bool| {
        let mut text = "6".to_string();
        drawn(&themed(), &mut text, current, Vec::new())
            .frames
            .iter()
            .any(|frame| frame.stroke.width > 0.0 && alike(frame.stroke.color, ACCENT_AMBER))
    };

    assert!(
        bordered_amber(true),
        "the current step's field is bordered amber"
    );
    assert!(
        !bordered_amber(false),
        "a field of another step wears the border every control wears"
    );
}

/// A number field as one frame of a themed context paints it, with no window.
struct Drawn {
    rect: egui::Rect,
    changed: bool,
    frames: Vec<egui::epaint::RectShape>,
    texts: Vec<(String, egui::Pos2)>,
}

impl Drawn {
    /// Where the field writes exactly `said`.
    fn written(&self, said: &str) -> Option<egui::Pos2> {
        self.texts
            .iter()
            .find(|(text, _)| text == said)
            .map(|(_, at)| *at)
    }
}

fn themed() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::theme::install(&ctx);
    ctx
}

/// Two frames of the field holding `text`, the second hearing `events`.
fn drawn(ctx: &egui::Context, text: &mut String, current: bool, events: Vec<egui::Event>) -> Drawn {
    let mut rect = egui::Rect::NOTHING;
    let mut changed = false;
    let mut shapes = Vec::new();

    for events in [Vec::new(), events] {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 200.0),
            )),
            events,
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            let response = number_field(ui, text, current, LEAST);
            rect = response.rect;
            changed |= response.changed();
        });
        output.textures_delta.clear();
        shapes = every_shape(output.shapes);
    }

    let frames = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(frame) => Some(frame.clone()),
            _ => None,
        })
        .collect();
    let texts = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(written) => Some((
                written.galley.text().to_owned(),
                shape.visual_bounding_rect().center(),
            )),
            _ => None,
        })
        .collect();
    Drawn {
        rect,
        changed,
        frames,
        texts,
    }
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
