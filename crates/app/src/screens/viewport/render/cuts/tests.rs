//! What app · screens/viewport/render/cuts.rs is held to.
//!
//! Closes #663.
//! - A fillet's or a chamfer's own points are not lit while a box is held
//!   beside them (surfaced by review) —
//!   `a_fillet_s_point_a_box_is_held_beside_is_not_lit`; pointing at one with
//!   no button held still lights it —
//!   `a_fillet_s_point_pointed_at_with_no_button_held_is_lit`

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_sketch::{Corner, CornerCutId, SegmentId, Selection, ToolState, WorkPlane};
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;

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

#[test]
fn a_cut_lit_at_a_point_carrying_another_lights_its_own_sides_alone() {
    let mut sketch = cao_sketch::Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::ZERO);
    let ends =
        [(0.0, 40.0), (30.0, 30.0), (40.0, 0.0)].map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let [top, middle, bottom] = ends.map(|end| sketch.add_segment(corner, end));
    let shape = cao_sketch::CutShape::Fillet { radius: 2.0 };
    let near = sketch
        .cut_corner(top, middle, shape, false, 0)
        .expect("a corner");
    sketch
        .cut_corner(top, bottom, shape, false, 1)
        .expect("another corner of the same point");

    let lit = ends_of(&sketch, &[near]);

    assert!(lit.contains(&(corner, top)) && lit.contains(&(corner, middle)));
    assert!(!lit.contains(&(corner, bottom)));
}

/// How many of a fillet's points are drawn lit, the cursor resting a few
/// pixels off the first one, with or without a box being pulled.
fn fillet_points_lit_with_the_cursor_beside_one(box_pulled: bool) -> usize {
    let mut document = a_rounded_plate();
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
        1.0,
        &ViewportConfig::default(),
        1.0,
    );
    let sketch = document.sketches()[0].clone();
    let (_, tangent_point) = sketch.drawn_cuts()[0]
        .points()
        .next()
        .expect("a fillet offers its points");
    let beside = tangent_point + DVec2::new(1.0, 1.0).normalize() * scale.world_size_of(9.0);
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::new(cao_sketch::SelectState {
            band: box_pulled.then_some((DVec2::new(-30.0, -30.0), beside)),
            ..Default::default()
        })),
        cursor: Some(beside),
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let theme = Theme::default();
    let mut painted = Vec::new();
    CutCanvas::read(&sketch, &context).push_points(&mut painted, &[], (&theme, scale), &context);
    let lit = tint_at(theme.highlight, 1.0);
    painted.iter().filter(|vertex| vertex.color == lit).count()
}

#[test]
fn a_fillet_s_point_pointed_at_with_no_button_held_is_lit() {
    assert!(
        fillet_points_lit_with_the_cursor_beside_one(false) > 0,
        "the point under the cursor is lit, the way it offers itself to a click"
    );
}

#[test]
fn a_fillet_s_point_a_box_is_held_beside_is_not_lit() {
    assert_eq!(
        fillet_points_lit_with_the_cursor_beside_one(true),
        0,
        "the box will not take the fillet's point it is held beside, so the point is not lit"
    );
}
