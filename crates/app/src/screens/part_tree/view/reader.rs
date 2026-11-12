//! The step reader's card above the history: the part's key steps as the
//! canvas reads them, and what a click on the card asks.

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, StepKind};
use egui::Color32;

use crate::lang::Catalogue;
use crate::screens::part_tree::state::{KeyStep, StepReader, TreeAction};
use crate::screens::theme_colours;
use crate::screens::viewport::ViewportState;
use crate::ui::badge::Level;
use crate::ui::icon::{self, Icon};
use crate::ui::reader_card::{Asked, ReaderCard, reader_card};
use crate::ui::step_box::StepBox;
use crate::ui::theme::tokens::TEXT_SECONDARY;

/// The card, nothing while the part has no key step.
pub(super) fn reader(
    ui: &mut egui::Ui,
    document: &PartDocument,
    viewport: &ViewportState,
    lang: &Catalogue,
) -> TreeAction {
    let reader = StepReader::seen(document, viewport, lang);
    let Some(line) = &reader.line else {
        return TreeAction::None;
    };
    let worn = |level: Level| theme_colours::worn(&viewport.theme, level);
    let called: Vec<String> = (1..=reader.boxes.len())
        .map(|rank| lang.t_with("part_tree.reader.step", &[("rank", &rank.to_string())]))
        .collect();
    let boxes: Vec<StepBox<'_>> = reader
        .boxes
        .iter()
        .zip(&called)
        .map(|(step, called)| StepBox {
            icon: icon_of(step),
            called,
            name: &step.name,
            applied: step.applied,
            current: step.current,
            alert: step.alert.map(worn),
        })
        .collect();
    let legend: Vec<(Color32, &str)> = reader
        .legend
        .iter()
        .map(|(level, word)| (worn(*level).1, word.as_str()))
        .collect();
    let (previous, next) = (
        lang.t("part_tree.reader.previous"),
        lang.t("part_tree.reader.next"),
    );
    let ink = line.alert.map_or(TEXT_SECONDARY, |level| worn(level).1);
    let card = ReaderCard {
        boxes: &boxes,
        arrows: &reader.arrows,
        previous: &previous,
        next: &next,
        line: (&line.text, ink, &line.counter),
        legend: &legend,
    };
    match reader_card(ui, &card) {
        Asked::Box(rank) => TreeAction::ReadTo(rank, reader.boxes.len()),
        Asked::Previous => TreeAction::ReadPrevious(reader.boxes.len()),
        Asked::Next => TreeAction::ReadNext(reader.boxes.len()),
        Asked::Nothing => TreeAction::None,
    }
}

fn icon_of(step: &KeyStep) -> Icon {
    match (step.kind, step.mode) {
        (StepKind::Sketch | StepKind::Section | StepKind::Blend, _) => icon::ITEM_SKETCH,
        (StepKind::Revolution, _) => icon::REVOLUTION,
        (StepKind::Extrusion, Some(ExtrusionMode::Cut)) => icon::EXTRUSION_CUT,
        (StepKind::Extrusion, _) => icon::EXTRUSION_ADD,
    }
}
