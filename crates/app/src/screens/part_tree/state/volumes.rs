//! The rows of « VOLUMES »: one per step of matter, read off its operation.

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, Operation};
use cao_render::MatterInFront;
use cao_sketch::Area;

use super::{Body, Points, Row, gravest, numbered};
use crate::lang::Catalogue;
use crate::wording::formula;

/// The row of a step of matter, the `rank`th of them, drawn opaque until the
/// view says otherwise; nothing for any other step.
pub(super) fn volume(
    lang: &Catalogue,
    document: &PartDocument,
    (step, rank): (usize, usize),
    operation: &Operation,
    (declined, unstable): (bool, Option<String>),
) -> Option<Body> {
    let variables = document.variables();
    let (name, reads, sketch, areas, mode) = match operation {
        Operation::Extrude {
            sketch,
            areas,
            distance,
            mode,
        } => {
            let travel = formula::sized(lang, variables, distance, |at| at.to_string());
            let reads = lang.t_with(
                "part_tree.raised",
                &[("distance", &travel), ("mode", &lang.t(mode_key(*mode)))],
            );
            ("part_tree.extrusion", reads, *sketch, areas, *mode)
        }
        Operation::Revolve {
            sketch,
            areas,
            angle,
            mode,
            ..
        } => {
            let turn = formula::sized(lang, variables, angle, |at| at.to_string());
            let reads = lang.t_with(
                "part_tree.swept",
                &[("angle", &turn), ("mode", &lang.t(mode_key(*mode)))],
            );
            ("part_tree.revolution", reads, *sketch, areas, *mode)
        }
        _ => return None,
    };
    let standing = standing_on(lang, document, sketch, areas);
    Some(Body {
        step,
        name: numbered(lang, name, rank),
        reads,
        badge: gravest(declined, standing.len() < areas.len(), unstable.is_some()),
        unstable,
        areas: standing,
        mode,
        shown: MatterInFront::Opaque,
        state: String::new(),
        visibility_button: String::new(),
        dimmed: false,
    })
}

fn mode_key(mode: ExtrusionMode) -> &'static str {
    match mode {
        ExtrusionMode::Add => "part_tree.mode.add",
        ExtrusionMode::Cut => "part_tree.mode.cut",
    }
}

/// The areas a step of matter was raised from, named as the sketch names them.
///
/// Which area a name answers to is `cao_part`'s answer and not one worked out
/// here: a panel that reasons about it for itself is a panel that reads
/// "fine" while the step raises nothing. One the drawing no longer encloses
/// leaves no line, and the step's badge is what says so.
fn standing_on(
    lang: &Catalogue,
    document: &PartDocument,
    sketch: usize,
    areas: &[Area],
) -> Vec<Row> {
    document
        .areas_standing(sketch, areas)
        .into_iter()
        .map(|rank| Row {
            name: numbered(lang, "part_tree.area", rank + 1),
            value: None,
            points: Points::Area { sketch, rank },
        })
        .collect()
}
