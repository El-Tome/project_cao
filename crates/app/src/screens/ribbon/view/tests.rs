// `architecture.rs`'s language test slices each file at its own first
// "#[cfg(test)]" literal to tell test code from shipped prose. This file is
// nothing but test code, reached only through `#[cfg(test)] mod tests;` in
// `view.rs`, but carries no such literal of its own — so an unrelated import
// wears it here to keep that scan from reading this file's own assertion
// messages as sentences owed to `fr.json`.
#[cfg(test)]
use chrono::Utc;

use super::*;

const WINDOW: egui::Vec2 = egui::vec2(1280.0, 800.0);

/// One frame of the part screen, laid out the way `app.rs` lays it out: the
/// toolbar, then the history panel, then whatever is left for the part.
/// Answers how wide the history panel came out and how wide the part did.
fn a_frame(ctx: &egui::Context, ribbon: &mut Ribbon, edge: Edge) -> (f32, f32) {
    a_frame_with(ctx, ribbon, edge, Vec::new())
}

fn a_frame_with(
    ctx: &egui::Context,
    ribbon: &mut Ribbon,
    edge: Edge,
    events: Vec<egui::Event>,
) -> (f32, f32) {
    let mut settings = Settings::default();
    settings.toolbar.edge = edge;
    let document = PartDocument::new("part", Utc::now());
    let editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let (mut history, mut part) = (0.0, 0.0);

    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, WINDOW)),
        events,
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| {
        ribbon.show(ui, &settings, &document, &editor, &mut extrusion, &lang);
        let before = ui.available_rect_before_wrap().width();
        let _ = crate::screens::history_tree::panel(ui, &document, &lang);
        history = before - ui.available_rect_before_wrap().width();
        egui::CentralPanel::no_frame().show(ui, |ui| part = ui.max_rect().width());
    });
    output.textures_delta.clear();

    (history, part)
}

fn toolbar_width((history, part): (f32, f32)) -> f32 {
    WINDOW.x - history - part
}

fn press(at: egui::Pos2) -> egui::Event {
    egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::default(),
    }
}

fn release(at: egui::Pos2) -> egui::Event {
    egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::default(),
    }
}

#[test]
fn a_toolbar_moved_to_a_side_leaves_room_for_the_part() {
    let ctx = egui::Context::default();
    let mut ribbon = Ribbon::new();
    a_frame(&ctx, &mut ribbon, Edge::Top);

    for edge in [Edge::Left, Edge::Right] {
        let (_, part) = a_frame(&ctx, &mut ribbon, edge);

        assert!(
            part > WINDOW.x / 2.0,
            "{edge:?} leaves the part {part} of {} points wide",
            WINDOW.x,
        );
    }
}

#[test]
fn coming_back_from_a_side_leaves_the_history_panel_as_wide_as_it_was() {
    let ctx = egui::Context::default();
    let mut ribbon = Ribbon::new();
    a_frame(&ctx, &mut ribbon, Edge::Top);
    let (before, _) = a_frame(&ctx, &mut ribbon, Edge::Top);

    a_frame(&ctx, &mut ribbon, Edge::Left);
    a_frame(&ctx, &mut ribbon, Edge::Top);
    let (after, _) = a_frame(&ctx, &mut ribbon, Edge::Top);

    assert_eq!(after, before);
}

#[test]
fn a_side_toolbar_pulled_wider_is_that_wide_the_next_time_it_is_chosen() {
    let ctx = egui::Context::default();
    let mut ribbon = Ribbon::new();
    let before = toolbar_width(a_frame(&ctx, &mut ribbon, Edge::Left));

    let grip = egui::pos2(before, WINDOW.y / 2.0);
    let pulled = grip + egui::vec2(80.0, 0.0);
    a_frame_with(&ctx, &mut ribbon, Edge::Left, vec![press(grip)]);
    a_frame_with(
        &ctx,
        &mut ribbon,
        Edge::Left,
        vec![egui::Event::PointerMoved(pulled)],
    );
    let widened = toolbar_width(a_frame_with(
        &ctx,
        &mut ribbon,
        Edge::Left,
        vec![release(pulled)],
    ));

    assert_eq!(widened, before + 80.0, "the drag did not widen the toolbar");

    a_frame(&ctx, &mut ribbon, Edge::Top);
    let back = toolbar_width(a_frame(&ctx, &mut ribbon, Edge::Left));

    assert_eq!(back, widened);
}

/// The height a top ribbon takes at a given window width, the way `a_frame`
/// reads a side one's width: before and after `ribbon.show`, the gap is
/// what the ribbon claimed.
fn ribbon_height(ctx: &egui::Context, ribbon: &mut Ribbon, width: f32) -> f32 {
    let mut settings = Settings::default();
    settings.toolbar.edge = Edge::Top;
    let document = PartDocument::new("part", Utc::now());
    let editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut height = 0.0;

    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width, WINDOW.y),
        )),
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| {
        let before = ui.available_rect_before_wrap().height();
        ribbon.show(ui, &settings, &document, &editor, &mut extrusion, &lang);
        height = before - ui.available_rect_before_wrap().height();
    });
    output.textures_delta.clear();

    height
}

#[test]
fn a_group_too_narrow_for_its_widest_button_wraps_instead_of_breaking_its_letters() {
    let ctx = egui::Context::default();
    let mut ribbon = Ribbon::new();

    let height = ribbon_height(&ctx, &mut ribbon, 350.0);

    assert!(
        height < WINDOW.y / 2.0,
        "a 350-wide window leaves the ribbon {height} points tall out of {}: a button's own \
         label is breaking letter by letter instead of its row wrapping",
        WINDOW.y,
    );
}
