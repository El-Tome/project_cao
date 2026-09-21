//! What app · screens/viewport/render/extrusion.rs is held to.

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_sketch::WorkPlane;
use chrono::Utc;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::{ExtrusionState, Shape};
use crate::screens::sketch::SketchEditor;

const INSIDE: DVec2 = DVec2::new(5.0, 10.0);

/// A drawing with one closed area in it, which is what an extrusion picks
/// from.
fn a_part_with_one_area() -> PartDocument {
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
    document
}

/// What one frame paints of an extrusion being set up.
fn shown(
    prepare: impl FnOnce(&mut ExtrusionState),
) -> (Vec<cao_render::Vertex>, Vec<cao_render::Vertex>) {
    let mut document = a_part_with_one_area();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    extrusion.offer(0);
    prepare(&mut extrusion);

    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let (mut surfaces, mut lines) = (Vec::new(), Vec::new());
    push_chosen_areas(&mut surfaces, &mut lines, &Theme::default(), &context);
    (surfaces, lines)
}

#[test]
fn nothing_is_tinted_while_the_tool_is_only_offered() {
    let (surfaces, lines) = shown(|_| {});

    assert!(
        surfaces.is_empty() && lines.is_empty(),
        "an area was filled before the tool was armed",
    );
}

#[test]
fn the_area_picked_is_filled_with_the_matter_it_would_become() {
    let (added, _) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.picks = vec![INSIDE];
    });
    let (cut, _) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Cut);
        extrusion.picks = vec![INSIDE];
    });

    assert!(!added.is_empty(), "the area picked was left untinted");
    assert_eq!(
        added.len(),
        cut.len(),
        "taking matter away covers a different area from adding it",
    );
    assert_ne!(
        added[0].color, cut[0].color,
        "matter added and matter taken away are shown in the same colour",
    );
}

#[test]
fn an_area_only_pointed_at_is_shown_apart_from_one_already_picked() {
    let (hovered, _) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.hovered = Some(0);
    });
    let (picked, _) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.picks = vec![INSIDE];
    });

    assert!(
        !hovered.is_empty(),
        "the area under the cursor shows nothing"
    );
    assert_ne!(
        hovered[0].color, picked[0].color,
        "pointing at an area looks exactly like having chosen it",
    );
}

#[test]
fn a_revolution_shows_the_axis_it_would_turn_around() {
    let (_, straight) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.picks = vec![INSIDE];
    });
    let (_, revolving) = shown(|extrusion| {
        extrusion.arm(ExtrusionMode::Add);
        extrusion.picks = vec![INSIDE];
        extrusion.shape = Shape::Revolution;
    });

    assert!(
        straight.is_empty(),
        "a straight extrusion drew an axis it does not turn around",
    );
    assert_eq!(
        revolving.len(),
        2,
        "the axis of a revolution is one straight line and nothing else",
    );
}
