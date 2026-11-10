//! What app · screens/ribbon/view.rs is held to.
//!
//! Closes #731. The rest is in `crates/app/tests/ribbon_groups.rs`; here is
//! what needs a part of the test's own, or the bar's own answer.
//! - a click on the icon of the STL export reaches the normal level —
//!   `a_click_on_the_icon_of_the_stl_export_reaches_the_normal_level`, read
//!   from what the bar answers: past it a save dialog opens, which no driver
//!   can stand in front of
//! - found in review: down a side the chamfer swap button is small too —
//!   `down_a_side_the_chamfer_swap_button_is_small`
//! - found in review: no line of the bar breaks inside a word —
//!   `a_group_too_narrow_for_its_widest_button_wraps_instead_of_breaking_its_letters`

// `architecture.rs`'s language test slices each file at its own first
// "#[cfg(test)]" literal to tell test code from shipped prose. This file is
// nothing but test code, reached only through `#[cfg(test)] mod tests;` in
// `view.rs`, but carries no such literal of its own — so an unrelated import
// wears it here to keep that scan from reading this file's own assertion
// messages as sentences owed to `fr.json`.
#[cfg(test)]
use chrono::Utc;

use super::*;
use crate::screens::viewport::ViewportState;

const WINDOW: egui::Vec2 = egui::vec2(1280.0, 800.0);

/// One frame of the part screen, laid out the way `app.rs` lays it out: the
/// toolbar, then the Part panel, then whatever is left for the part. Answers
/// how wide the Part panel came out and how wide the part did.
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
    let viewport = ViewportState::default();
    let lang = Catalogue::french();
    let (mut beside, mut part) = (0.0, 0.0);

    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, WINDOW)),
        events,
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| {
        let mut drawn = Drawn {
            document: &document,
            editor: &editor,
            extrusion: &mut extrusion,
        };
        ribbon.show(ui, &settings, &mut drawn, &lang);
        let before = ui.available_rect_before_wrap().width();
        let _ = crate::screens::part_tree::panel(ui, &document, &editor, &viewport, false, &lang);
        beside = before - ui.available_rect_before_wrap().width();
        egui::CentralPanel::no_frame().show(ui, |ui| part = ui.max_rect().width());
    });
    output.textures_delta.clear();

    (beside, part)
}

fn toolbar_width((beside, part): (f32, f32)) -> f32 {
    WINDOW.x - beside - part
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
fn coming_back_from_a_side_leaves_the_part_panel_as_wide_as_it_was() {
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

/// Whether one line stops in the middle of a word and the next one goes on
/// with it: a line ended by a space, or by the end of the text, breaks nothing.
fn breaks_inside_a_word(lines: &[String]) -> bool {
    lines.windows(2).any(|pair| {
        pair[0]
            .chars()
            .next_back()
            .is_some_and(|last| !last.is_whitespace())
            && pair[1]
                .chars()
                .next()
                .is_some_and(|first| !first.is_whitespace())
    })
}

/// Every piece of text a top ribbon draws at a given window width, with the
/// lines each was laid on. The first frame only measures the panel and
/// draws nothing of what is in it.
fn ribbon_texts(
    ctx: &egui::Context,
    ribbon: &mut Ribbon,
    width: f32,
) -> Vec<(String, Vec<String>)> {
    let mut settings = Settings::default();
    settings.toolbar.edge = Edge::Top;
    let document = PartDocument::new("part", Utc::now());
    let editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();

    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width, WINDOW.y),
        )),
        ..Default::default()
    };
    let mut frame = || {
        let mut output = ctx.run_ui(input.clone(), |ui| {
            let mut drawn = Drawn {
                document: &document,
                editor: &editor,
                extrusion: &mut extrusion,
            };
            ribbon.show(ui, &settings, &mut drawn, &lang);
        });
        output.textures_delta.clear();
        output
    };
    frame();

    frame()
        .shapes
        .into_iter()
        .filter_map(|clipped| match clipped.shape {
            egui::Shape::Text(text) => {
                let lines = text
                    .galley
                    .rows
                    .iter()
                    .map(|placed| placed.row.glyphs.iter().map(|glyph| glyph.chr).collect())
                    .collect();
                Some((text.galley.text().to_owned(), lines))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn a_group_too_narrow_for_its_widest_button_wraps_instead_of_breaking_its_letters() {
    let ctx = egui::Context::default();
    let mut ribbon = Ribbon::new();

    let texts = ribbon_texts(&ctx, &mut ribbon, 350.0);

    assert!(
        texts.iter().any(|(text, _)| text.contains("Esquisse")),
        "the ribbon is drawn: {texts:?}"
    );
    let broken: Vec<_> = texts
        .iter()
        .filter(|(_, lines)| breaks_inside_a_word(lines))
        .collect();
    assert!(
        broken.is_empty(),
        "a 350-wide window breaks a word of these labels across two lines, where only spaces may \
         end a line: {broken:?}"
    );
}

fn a_raised_block() -> PartDocument {
    use cao_part::{ExtrusionMode, Operation, PointRef};
    use cao_sketch::WorkPlane;
    use glam::DVec2;

    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert!(!document.body().is_empty(), "the block is raised");
    document
}

/// The ribbon on a bench of its own: what `act` found, and every command the
/// bar answered while it was drawn.
fn on_a_bench<R>(
    settings: Settings,
    document: PartDocument,
    editor: SketchEditor,
    tab: usize,
    act: impl FnOnce(&mut egui_kittest::Harness<'_>) -> R,
) -> (R, Vec<Command>) {
    let (mut ribbon, lang) = (Ribbon::new(), Catalogue::french());
    ribbon.tab = tab;
    let mut extrusion = ExtrusionState::default();
    let mut asked = Vec::new();
    let mut installed = false;

    let found = {
        let mut harness = egui_kittest::Harness::builder()
            .with_size(WINDOW)
            .build_ui(|ui| {
                if !installed {
                    crate::ui::icon::install(ui.ctx());
                    installed = true;
                }
                let mut drawn = Drawn {
                    document: &document,
                    editor: &editor,
                    extrusion: &mut extrusion,
                };
                asked.extend(ribbon.show(ui, &settings, &mut drawn, &lang));
            });
        harness.run();
        act(&mut harness)
    };
    (found, asked)
}

#[test]
fn a_click_on_the_icon_of_the_stl_export_reaches_the_normal_level() {
    use egui_kittest::kittest::Queryable;

    let (_, asked) = on_a_bench(
        Settings::default(),
        a_raised_block(),
        SketchEditor::default(),
        2,
        |harness| {
            harness.get_by_label("Exporter en STL…").click();
            harness.run();
        },
    );

    assert_eq!(asked, [Command::ExportStlNormal]);
}

/// A plain rectangle and its first corner chamfered, the selection holding
/// that chamfer: what the bar offers its swap for.
fn a_chamfer_held() -> (PartDocument, SketchEditor) {
    use cao_part::Operation;
    use cao_part::history::{ChamferAsked, PointRef};
    use cao_sketch::{Chamfer, Corner, CornerCutId, SegmentId, Selection, ToolState, WorkPlane};
    use glam::DVec2;

    use crate::screens::sketch::{SketchPhase, Tool};

    let mut document = PartDocument::new("part", Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        mode: ChamferAsked::from(Chamfer::Sided {
            first: 2.0,
            second: 4.0,
        }),
        construction: false,
    });
    let editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::new(cao_sketch::SelectState {
            held: vec![Selection::Cut(CornerCutId(0))],
            ..Default::default()
        })),
        ..Default::default()
    };
    (document, editor)
}

#[test]
fn down_a_side_the_chamfer_swap_button_is_small() {
    use egui_kittest::kittest::Queryable;

    let (document, editor) = a_chamfer_held();
    assert!(
        chamfer_to_swap(&document, &editor).is_some(),
        "the chamfer held is one the bar offers its swap for",
    );
    let mut settings = Settings::default();
    settings.toolbar.edge = Edge::Left;

    let (height, _) = on_a_bench(settings, document, editor, 0, |harness| {
        harness.get_by_label("Inverser les côtés").rect().height()
    });

    assert!(
        height <= 27.0,
        "down a side, « Inverser les côtés » is {height} high, where a small button is 20 or 26",
    );
}
