//! What app · screens/viewport/input/cut_changes.rs is held to, with the
//! corner tool laying what it opens.

use cao_part::history::{ChamferAsked, CutChange, PointRef};
use cao_part::{Operation, PartDocument, Variables};
use cao_sketch::{
    Chamfer, Corner, CornerCutId, CutShape, SegmentId, Selection, ToolState, WorkPlane,
};
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchPhase;

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

/// The selection tool holding `held`, on the first sketch.
fn holding(held: Vec<Selection>) -> SketchEditor {
    SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::new(cao_sketch::SelectState {
            held,
            ..Default::default()
        })),
        ..Default::default()
    }
}

fn shapes(document: &PartDocument) -> Vec<CutShape> {
    document.sketches()[0]
        .live_corner_cuts()
        .map(|(_, cut)| cut.shape)
        .collect()
}

#[test]
fn a_double_click_opens_a_fillet_s_values_with_its_whole_group() {
    let mut document = a_rounded_plate();
    let mut editor = holding(Vec::new());
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    open_cut_values(&mut context, 0, CornerCutId(2));

    assert_eq!(context.editor.tool, Tool::Fillet);
    assert!(
        matches!(&context.editor.tool_state, ToolState::Corner { taken, .. } if taken.len() == 4),
        "every corner laid with it is taken"
    );
    assert_eq!(context.editor.live.typed(0), Some(5.0));
}

#[test]
fn retyping_the_values_of_a_whole_group_changes_it_in_place() {
    let mut document = a_rounded_plate();
    let mut editor = holding(Vec::new());
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    open_cut_values(&mut context, 0, CornerCutId(0));
    let field = context.editor.live.field(0);
    field.text = "3".to_string();
    field.take(&Variables::default());

    assert!(super::super::corner::cut(&mut context, 0));

    assert!(
        matches!(
            context.document.history.applied_operations().last(),
            Some(Operation::CutChange(CutChange::Reshape { .. }))
        ),
        "one step that changes the group, not four corners cut again"
    );
    assert_eq!(
        shapes(context.document),
        vec![CutShape::Fillet { radius: 3.0 }; 4]
    );
    assert_eq!(
        context.document.sketches()[0]
            .cuts_grouped_with(CornerCutId(0))
            .len(),
        4
    );
}

#[test]
fn one_corner_of_a_group_cut_again_leaves_the_group_with_its_new_value() {
    let mut document = a_rounded_plate();
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Fillet,
        tool_state: ToolState::Corner {
            taken: vec![Corner::Between(SegmentId(0), SegmentId(1))],
            half: None,
        },
        ..Default::default()
    };
    let field = editor.live.field(0);
    field.text = "3".to_string();
    field.take(&Variables::default());
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    assert!(super::super::corner::cut(&mut context, 0));

    let sketch = &context.document.sketches()[0];
    let (again, cut) = sketch
        .live_corner_cuts()
        .find(|(_, cut)| cut.shape == CutShape::Fillet { radius: 3.0 })
        .expect("the corner cut again");
    assert_eq!(sketch.cuts_grouped_with(again), vec![again], "{cut:?}");
    assert_eq!(
        shapes(context.document)
            .iter()
            .filter(|shape| **shape == CutShape::Fillet { radius: 5.0 })
            .count(),
        3,
        "the three others keep their group and their value"
    );
}

#[test]
fn suppr_takes_a_held_cut_off_and_its_corner_is_sharp_again() {
    let mut document = a_rounded_plate();
    let mut editor = holding(vec![Selection::Cut(CornerCutId(1))]);
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    super::super::selecting::erase(&mut context, 0, &[Selection::Cut(CornerCutId(1))]);

    let sketch = &context.document.sketches()[0];
    assert_eq!(sketch.live_corner_cuts().count(), 3);
    assert!(
        sketch
            .live_corner_cuts()
            .all(|(id, _)| id != CornerCutId(1))
    );
}

#[test]
fn construction_with_a_cut_held_switches_that_cut_alone() {
    let mut document = a_rounded_plate();
    let mut editor = holding(vec![Selection::Cut(CornerCutId(3))]);
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    assert!(switch_held_cuts(&mut context));

    let construction: Vec<bool> = context.document.sketches()[0]
        .live_corner_cuts()
        .map(|(_, cut)| cut.construction)
        .collect();
    assert_eq!(construction, vec![false, false, false, true]);
    assert!(
        !context.editor.construction,
        "the toggle itself is left as it was"
    );
}

#[test]
fn the_sides_of_a_held_chamfer_are_swapped() {
    let mut document = a_rounded_plate();
    document.apply(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        mode: ChamferAsked::from(Chamfer::Sided {
            first: 2.0,
            second: 4.0,
        }),
        construction: false,
    });
    let chamfer = CornerCutId(4);
    let mut editor = holding(vec![Selection::Cut(chamfer)]);
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    assert_eq!(
        chamfer_to_swap(context.document, context.editor),
        Some(chamfer)
    );

    assert!(swap_held_chamfer(&mut context));

    assert_eq!(
        context.document.sketches()[0].corner_cuts()[chamfer.0].sides,
        [SegmentId(1), SegmentId(0)]
    );
}

#[test]
fn a_fillet_held_offers_no_sides_to_swap() {
    let document = a_rounded_plate();
    let editor = holding(vec![Selection::Cut(CornerCutId(0))]);

    assert_eq!(chamfer_to_swap(&document, &editor), None);
}

#[test]
fn a_radius_dimension_on_a_fillet_opens_its_value() {
    let mut document = a_rounded_plate();
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Dimension,
        dimension_mode: crate::screens::sketch::DimensionMode::Radius,
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let on_the_curve = DVec2::new(5.0, 5.0) - DVec2::splat(5.0 / 2.0_f64.sqrt());

    super::super::measure::measure(&mut context, 0, on_the_curve, 0.5, 0.1);

    assert_eq!(context.editor.tool, Tool::Fillet);
    assert_eq!(context.editor.live.typed(0), Some(5.0));
    assert!(
        context.document.sketches()[0].dimensions().is_empty(),
        "no second radius is laid beside the fillet's own"
    );
}

#[test]
fn a_measure_taken_of_a_fillet_reads_its_radius() {
    let mut document = a_rounded_plate();
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Measure,
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let on_the_curve = DVec2::new(5.0, 5.0) - DVec2::splat(5.0 / 2.0_f64.sqrt());

    super::super::reading::read(&mut context, 0, on_the_curve, 0.5);

    let reading = crate::screens::viewport::render::what_is_measured(&context)
        .expect("the fillet is read")
        .reading;
    let cao_sketch::Reading::Round { radius } = reading else {
        panic!("a fillet reads as a radius: {reading:?}");
    };
    assert!((radius - 5.0).abs() < 1e-9, "{radius}");
}

#[test]
fn a_fillet_written_from_a_variable_opens_on_its_formula_and_keeps_following_it() {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::Variable(cao_part::VariableChange::Added {
        name: "arrondi".to_string(),
        formula: cao_part::Formula::Number(10.0 / 3.0),
    }));
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
    let radius = document
        .variables()
        .read("arrondi")
        .expect("a formula that reads");
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        radius,
        construction: false,
    });
    let mut editor = holding(Vec::new());
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    open_cut_values(&mut context, 0, CornerCutId(0));
    assert_eq!(context.editor.live.field(0).text, "arrondi");
    assert!(super::super::corner::cut(&mut context, 0));
    context
        .document
        .change_variable(cao_part::VariableChange::Edited {
            variable: cao_part::VariableId(0),
            name: "arrondi".to_string(),
            formula: cao_part::Formula::Number(4.0),
        })
        .expect("a smaller rounding fits");

    assert_eq!(
        shapes(context.document),
        vec![CutShape::Fillet { radius: 4.0 }]
    );
}

#[test]
fn a_chamfer_group_clicked_in_the_other_order_is_cut_again_rather_than_retyped() {
    let mut document = a_rounded_plate();
    document.apply(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        mode: ChamferAsked::from(Chamfer::Sided {
            first: 2.0,
            second: 4.0,
        }),
        construction: false,
    });
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Chamfer,
        chamfer_mode: cao_sketch::ChamferMode::Sided,
        tool_state: ToolState::Corner {
            taken: vec![Corner::Between(SegmentId(1), SegmentId(0))],
            half: None,
        },
        ..Default::default()
    };
    for (rank, text) in ["3", "1"].into_iter().enumerate() {
        let field = editor.live.field(rank);
        field.text = text.to_string();
        field.take(&Variables::default());
    }
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    assert!(super::super::corner::cut(&mut context, 0));

    assert!(
        matches!(
            context.document.history.applied_operations().last(),
            Some(Operation::Chamfer { .. })
        ),
        "the sides named the other way are the ones the new chamfer measures from"
    );
    let sketch = &context.document.sketches()[0];
    let (_, cut) = sketch
        .live_corner_cuts()
        .find(|(_, cut)| matches!(cut.shape, CutShape::Chamfer(_)))
        .expect("the corner cut again");
    assert_eq!(cut.sides, [SegmentId(1), SegmentId(0)]);
}

#[test]
fn the_value_written_beside_a_small_fillet_takes_the_fillet_rather_than_its_corner() {
    let document = a_rounded_plate();
    let sketch = &document.sketches()[0];
    let (at_the_origin, _) = sketch
        .live_corner_cuts()
        .find(|(_, cut)| sketch.point(cut.corner).length() < 1e-9)
        .expect("a fillet on the corner at the origin");
    let pixel = 0.1;
    let middle = DVec2::splat(5.0) - DVec2::splat(5.0 / 2.0_f64.sqrt());
    let written_at = middle - DVec2::splat(14.0 * pixel / 2.0_f64.sqrt());

    assert_eq!(
        crate::screens::viewport::render::cut_label_under(sketch, written_at, pixel),
        Some(at_the_origin)
    );
    assert!(
        written_at.length() < 18.0 * pixel,
        "the corner's own point is within a press of the value"
    );
}

#[test]
fn a_press_on_the_sharp_corner_of_a_small_fillet_takes_the_corner_rather_than_its_value() {
    let document = a_rounded_plate();
    let sketch = &document.sketches()[0];
    let pixel = 0.1;
    let beside = DVec2::splat(-2.0 * pixel);

    assert_eq!(
        crate::screens::viewport::render::cut_label_under(sketch, beside, pixel),
        None
    );
}

#[test]
fn the_second_pair_of_a_point_given_a_fillet_is_laid_beside_the_first_rather_than_retyping_it() {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::ZERO),
        end: PointRef::New(DVec2::new(0.0, 40.0)),
        construction: false,
    });
    let corner = document.sketches()[0].segments()[0].start;
    for end in [DVec2::new(30.0, 30.0), DVec2::new(40.0, 0.0)] {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(corner),
            end: PointRef::New(end),
            construction: false,
        });
    }
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        radius: 1.5.into(),
        construction: false,
    });
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Fillet,
        tool_state: ToolState::Corner {
            taken: vec![Corner::Between(SegmentId(0), SegmentId(2))],
            half: None,
        },
        ..Default::default()
    };
    let field = editor.live.field(0);
    field.text = "5".to_string();
    field.take(&Variables::default());
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let mut context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    assert!(super::super::corner::cut(&mut context, 0));

    assert!(
        matches!(
            context.document.history.applied_operations().last(),
            Some(Operation::Fillet { .. })
        ),
        "another pair of the point is another corner, not the first one's group"
    );
    assert_eq!(
        shapes(context.document),
        vec![
            CutShape::Fillet { radius: 1.5 },
            CutShape::Fillet { radius: 5.0 }
        ]
    );
}
