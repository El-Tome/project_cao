//! What app · screens/viewport/render/cuts.rs is held to.

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_sketch::{Corner, CornerCutId, SegmentId, Selection, ToolState, WorkPlane};
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};

/// A plate forty by twenty, its four corners rounded together by five.
fn a_rounded_plate() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
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
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: (0..4)
            .map(|side| Corner::Between(SegmentId(side), SegmentId((side + 1) % 4)))
            .collect(),
        radius: 5.0.into(),
        construction: false,
    });
    document
}

fn lit_holding(held: Vec<Selection>, hovered: Option<Selection>) -> Lit {
    let mut document = a_rounded_plate();
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::new(cao_sketch::SelectState {
            held,
            ..Default::default()
        })),
        hovered,
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    lit_cuts(&context.document.sketches()[0], &context)
}

#[test]
fn the_cut_held_is_lit_apart_from_the_cuts_laid_with_it() {
    let lit = lit_holding(vec![Selection::Cut(CornerCutId(2))], None);

    assert_eq!(lit.held, vec![CornerCutId(2)]);
    assert_eq!(
        lit.linked,
        vec![CornerCutId(0), CornerCutId(1), CornerCutId(3)]
    );
}

#[test]
fn the_cut_pointed_at_is_lit_apart_too() {
    let lit = lit_holding(Vec::new(), Some(Selection::Cut(CornerCutId(0))));

    assert_eq!(lit.held, vec![CornerCutId(0)]);
    assert_eq!(lit.linked.len(), 3);
}

#[test]
fn nothing_held_lights_no_cut() {
    assert_eq!(lit_holding(Vec::new(), None), Lit::default());
}
