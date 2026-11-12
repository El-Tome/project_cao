//! What app · ui/chip.rs is held to.
//!
//! Closes #738 — the whole transcription is in
//! `crates/app/src/screens/viewport/input/copying/tests/steps.rs`; this is the
//! chip's share of one criterion.
//! - `ui/` gains the controls it lacks: the chip — the two variants of C14 a
//!   panel uses, `a_selection_chip_is_amber_in_its_fill_its_border_and_its_ink`,
//!   `only_the_chip_inviting_a_click_in_the_view_is_dashed_and_it_fills_nothing`;
//!   and that each is drawn 24 points tall with its label —
//!   `a_chip_is_drawn_24_points_tall_with_its_label_in_each_variant`

use super::*;
use crate::ui::theme::tokens::{ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_BORDER, ACCENT_AMBER_TEXT};

/// How far a size may stray from the one C14 asks for: shapes are placed on
/// rounded points.
const SIZED_WITHIN: f32 = 0.5;

#[test]
fn a_selection_chip_is_amber_in_its_fill_its_border_and_its_ink() {
    let look = ChipKind::Selection.look();

    assert_eq!(look.fill, ACCENT_AMBER_BACKGROUND);
    assert_eq!(look.border, ACCENT_AMBER_BORDER);
    assert_eq!(look.ink, ACCENT_AMBER_TEXT);
}

#[test]
fn only_the_chip_inviting_a_click_in_the_view_is_dashed_and_it_fills_nothing() {
    let invitation = ChipKind::Invitation.look();

    assert!(
        invitation.dashed,
        "« + cliquer dans la vue » is bordered in dashes"
    );
    assert_eq!(
        invitation.fill,
        egui::Color32::TRANSPARENT,
        "it holds nothing yet, and fills nothing"
    );
    assert!(
        !ChipKind::Selection.look().dashed,
        "a chip holding an element is bordered in a solid line"
    );
}

#[test]
fn a_chip_is_drawn_24_points_tall_with_its_label_in_each_variant() {
    for kind in [ChipKind::Selection, ChipKind::Invitation] {
        let (rect, texts) = drawn("Cercle 1", kind);

        assert!(
            (rect.height() - 24.0).abs() < SIZED_WITHIN,
            "{kind:?}: the chip stands {} tall, where C14 draws it 24",
            rect.height(),
        );
        assert!(
            texts.iter().any(|text| text == "Cercle 1"),
            "{kind:?}: the chip writes {texts:?}"
        );
    }
}

/// Where a chip stands and what it writes, as one frame of a themed context
/// paints it, with no window.
fn drawn(label: &str, kind: ChipKind) -> (egui::Rect, Vec<String>) {
    let ctx = egui::Context::default();
    crate::ui::theme::install(&ctx);
    let mut rect = egui::Rect::NOTHING;
    let mut texts = Vec::new();

    for _ in 0..2 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 200.0),
            )),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            rect = chip(ui, label, kind).rect;
        });
        output.textures_delta.clear();
        texts = written(output.shapes);
    }

    (rect, texts)
}

fn written(clipped: Vec<egui::epaint::ClippedShape>) -> Vec<String> {
    let mut pending: Vec<egui::Shape> = clipped.into_iter().map(|clipped| clipped.shape).collect();
    let mut texts = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            egui::Shape::Text(text) => texts.push(text.galley.text().to_owned()),
            _ => {}
        }
    }
    texts
}
