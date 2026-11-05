//! What the tree of a part holds.
//!
//! Closes #345.
//! - an area one of whose bounding traits is erased raises nothing, and the
//!   tree says which step lost it —
//!   `a_step_whose_area_is_gone_says_so_rather_than_reading_as_one_raised_from_nothing`
//! - a step that still has every area it stands on is not marked —
//!   `a_step_whose_areas_are_all_there_is_not_marked`
//!
//! Closes #526.
//! - a step the exact kernel declines is marked in the tree, and a step it
//!   built is not — `a_step_the_kernel_declines_is_marked_in_the_tree`
//!
//! Closes #552.
//! - a step the exact kernel built at a lean it reads unstably is marked in the
//!   tree, with the sentence that says why, and a step it built stably is not
//!   — `a_step_built_at_an_unstable_lean_is_marked_in_the_tree`
//!
//! Closes #348 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/tests.rs`; these bullets are the
//! ones the tree's presenter holds.
//! - "A click on an element selects it — in the viewport, as if it had been
//!   clicked there": in the sketch being edited, a click on a trait, a
//!   dimension or a rule takes it alone, as a click in the viewport does —
//!   `a_click_on_a_trait_of_the_open_sketch_selects_it_alone`,
//!   `a_click_on_a_dimension_or_a_rule_of_the_open_sketch_selects_it`; with
//!   the modifier held, it is added, and taken back on a second click —
//!   `a_click_with_the_modifier_adds_to_what_the_open_sketch_holds_and_takes_it_back`
//! - in a sketch not open, a click on an element or an area lights it, and
//!   opens nothing — `a_click_on_a_row_of_a_closed_sketch_lights_it_and_opens_nothing`
//! - "Selecting from the viewport, which must agree with selecting from the
//!   tree — one selection, two ways in": the tree marks what the viewport
//!   holds — `the_tree_marks_what_the_viewport_holds`
//! - found in review: a row lit on the canvas is marked in the tree only while
//!   its sketch is not being drawn on —
//!   `a_lit_row_is_marked_only_while_its_sketch_is_not_being_drawn_on`
//! - decided in review: a click in the tree while the tool in hand is part-way
//!   through something does nothing —
//!   `a_click_in_the_tree_while_a_rule_waits_for_its_second_pick_does_nothing`
//! - decided in review: an area row of the open sketch lights the area —
//!   `an_area_row_of_the_open_sketch_lights_the_area`; a dimension or a rule
//!   row of a closed sketch is no click at all —
//!   `a_dimension_or_a_rule_of_a_closed_sketch_is_no_click_at_all`

use cao_part::history::{FaceAnchor, PointRef, RevolutionAxis};
use cao_sketch::{
    Constraint, DimensionTarget, Element, PointId, Rule, RulePick, SegmentId, Selection,
    SketchAxis, ToolState, WorkPlane,
};
use chrono::{DateTime, Utc};
use glam::{DVec2, DVec3};

use super::*;
use crate::screens::part_tree::TreeAction;
use crate::screens::sketch::{SketchEditor, Tool};
use crate::screens::viewport::ViewportState;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

fn french() -> Catalogue {
    Catalogue::french()
}

/// A rectangle on the XY plane, raised into a block, and a second sketch.
fn a_part() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
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
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 100.0.into(),
        placement: None,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document
}

#[test]
fn the_tree_holds_the_bodies_and_the_sketches_and_not_the_run_of_steps() {
    let tree = PartTree::of(&a_part(), &french());

    assert_eq!(tree.bodies.len(), 1);
    assert_eq!(tree.sketches.len(), 2);
    assert_eq!(tree.bodies[0].name, "Extrusion 1");
    assert_eq!(tree.sketches[0].name, "Esquisse 1");
    assert_eq!(tree.sketches[1].name, "Esquisse 2");
}

#[test]
fn a_step_that_made_matter_reads_how_far_it_went_and_which_way() {
    let tree = PartTree::of(&a_part(), &french());

    assert_eq!(tree.bodies[0].reads, "4 mm · ajout de matière");
}

#[test]
fn a_step_whose_area_is_gone_says_so_rather_than_reading_as_one_raised_from_nothing() {
    let mut document = a_part();
    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let tree = PartTree::of(&document, &french());

    assert!(tree.bodies[0].lost);
    assert!(
        tree.bodies[0].areas.is_empty(),
        "there is nothing left to point at",
    );
}

#[test]
fn a_step_whose_areas_are_all_there_is_not_marked() {
    let tree = PartTree::of(&a_part(), &french());

    assert!(!tree.bodies[0].lost);
}

#[test]
fn a_step_that_made_matter_names_the_areas_it_stands_on() {
    let tree = PartTree::of(&a_part(), &french());

    assert_eq!(
        tree.bodies[0].areas,
        [Row {
            name: "Aire 1".to_string(),
            points: Points::Area { sketch: 0, rank: 0 },
        }],
    );
}

#[test]
fn a_sketch_holds_what_is_drawn_on_it_sorted_by_kind() {
    let tree = PartTree::of(&a_part(), &french());
    let drawing = &tree.sketches[0];

    assert_eq!(drawing.areas.len(), 1, "the rectangle encloses one area");
    assert_eq!(drawing.strokes.len(), 4, "and stands on four traits");
    assert_eq!(drawing.dimensions.len(), 1);
    assert_eq!(drawing.strokes[0].name, "Trait 1");
    assert_eq!(drawing.dimensions[0].name, "Cote 1");
}

#[test]
fn an_area_named_under_a_body_is_the_one_its_sketch_names() {
    let tree = PartTree::of(&a_part(), &french());

    assert_eq!(tree.bodies[0].areas[0], tree.sketches[0].areas[0]);
}

#[test]
fn a_drag_leaves_no_line_of_its_own() {
    let mut document = a_part();
    let before = PartTree::of(&document, &french());

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: cao_sketch::PointId(2),
        position: DVec2::new(11.0, 21.0),
        merged_into: None,
        let_go: false,
        on: Vec::new(),
    });

    let after = PartTree::of(&document, &french());
    assert_eq!(
        after.sketches[0].strokes, before.sketches[0].strokes,
        "the tree shows what the part is made of, and a drag leaves no element \
         behind it",
    );
}

#[test]
fn a_part_with_nothing_drawn_on_it_holds_nothing() {
    let document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));

    assert!(PartTree::of(&document, &french()).is_empty());
}

#[test]
fn a_sketch_that_lost_its_face_is_marked_in_the_tree() {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: Some(FaceAnchor {
            face: 404,
            up: DVec3::Y,
        }),
    });

    let tree = PartTree::of(&document, &french());

    assert!(
        tree.sketches[0].adrift,
        "a drawing whose face the part no longer has must say so",
    );
}

/// A ball of `radius` about `center`: a half disc drawn on the plane holding
/// the upright through it, turned whole about that line.
fn a_ball(document: &mut PartDocument, center: DVec3, radius: f64) {
    let sketch = document.sketches().len();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: center,
            u: DVec3::X,
            v: DVec3::Z,
        },
        on: None,
    });
    let first = document.sketches()[sketch].points().len();
    document.apply(Operation::AddArc {
        sketch,
        center: PointRef::New(DVec2::ZERO),
        start: PointRef::New(DVec2::new(0.0, -radius)),
        end: PointRef::New(DVec2::new(0.0, radius)),
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch,
        start: PointRef::Existing(PointId(first + 2)),
        end: PointRef::Existing(PointId(first + 1)),
        construction: false,
    });
    document.apply(Operation::Revolve {
        sketch,
        areas: document.areas_at(sketch, &[DVec2::new(radius / 4.0, 0.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 360.0.into(),
        mode: ExtrusionMode::Add,
    });
}

/// A ball of radius 4 about (5, 10, 0), then a circle on a plane tilted 30°
/// about X raised down across its sphere, three off its centre.
fn a_ball_under_a_slanted_circle_raised() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    a_ball(&mut document, DVec3::new(5.0, 10.0, 0.0), 4.0);
    let (sin, cos) = (std::f64::consts::PI / 6.0).sin_cos();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(5.0, 10.0, 6.0),
            u: DVec3::X,
            v: DVec3::new(0.0, cos, sin),
        },
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 1,
        center: PointRef::New(DVec2::ZERO),
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::ZERO]),
        distance: (-10.0).into(),
        mode: ExtrusionMode::Add,
    });
    document
}

#[test]
fn a_step_the_kernel_declines_is_marked_in_the_tree() {
    let tree = PartTree::of(&a_ball_under_a_slanted_circle_raised(), &french());

    assert!(
        tree.bodies[1].declined,
        "a circle raised across the ball's sphere off its centre meets it \
         where the exact kernel cannot follow, and declines",
    );
    assert!(!tree.bodies[0].declined, "the ball itself was built");
}

/// A shaft Ø20 raised 30 on XY, and a slice of it turned a hundredth of a
/// degree about X cut out: the slice's second side leans that much off along
/// the shaft's axis.
fn a_shaft_cut_by_a_slice_a_hundredth_of_a_degree_thin() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: 10.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::ZERO]),
        distance: 30.0.into(),
        mode: ExtrusionMode::Add,
    });
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XZ,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(-20.0, 0.0)),
        opposite: PointRef::New(DVec2::new(20.0, 40.0)),
        construction: false,
    });
    document.apply(Operation::Revolve {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::new(0.0, 20.0)]),
        axis: RevolutionAxis::Sketch(SketchAxis::U),
        angle: 0.01.into(),
        mode: ExtrusionMode::Cut,
    });
    document
}

#[test]
fn a_step_built_at_an_unstable_lean_is_marked_in_the_tree() {
    let lang = french();
    let tree = PartTree::of(
        &a_shaft_cut_by_a_slice_a_hundredth_of_a_degree_thin(),
        &lang,
    );

    assert_eq!(
        tree.bodies[1].unstable,
        Some(lang.t_with("extrusion.unstable_along", &[("angle", "0.01")])),
        "the slice is built and said unstable",
    );
    assert!(!tree.bodies[1].declined, "built, not declined");
    assert_eq!(tree.bodies[0].unstable, None, "the shaft stands square");
}

/// The part, with its first sketch open for drawing, the line in hand as
/// reopening a sketch leaves it.
fn the_first_sketch_open() -> (PartDocument, SketchEditor) {
    let document = a_part();
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    (document, editor)
}

fn a_trait(rank: usize) -> Selection {
    Selection::Element(Element::Segment(SegmentId(rank)))
}

fn the_row_of(sketch: usize, rank: usize) -> Points {
    Points::Element {
        sketch,
        element: Element::Segment(SegmentId(rank)),
    }
}

/// A click on a row, carried out the way the panel carries it out.
fn click_on(
    document: &PartDocument,
    editor: &mut SketchEditor,
    row: Points,
    adding: bool,
) -> TreeAction {
    let action = clicked(document, editor, row, adding);
    if let TreeAction::Select { what, adding } = action {
        take(editor, what, adding);
    }
    action
}

#[test]
fn a_click_on_a_trait_of_the_open_sketch_selects_it_alone() {
    let (document, mut editor) = the_first_sketch_open();

    click_on(&document, &mut editor, the_row_of(0, 1), false);
    let action = click_on(&document, &mut editor, the_row_of(0, 0), false);

    assert_eq!(
        action,
        TreeAction::Select {
            what: a_trait(0),
            adding: false,
        },
    );
    assert_eq!(
        editor.selection(),
        [a_trait(0)],
        "a plain click in the tree does not replace what was held, as one in the viewport does",
    );
    assert!(
        editor.is_selected(a_trait(0)) && !editor.is_selected(a_trait(1)),
        "the viewport draws as held something other than what the tree took",
    );
    assert_eq!(
        editor.active_sketch(),
        Some(0),
        "the click closed the sketch"
    );
}

#[test]
fn a_click_with_the_modifier_adds_to_what_the_open_sketch_holds_and_takes_it_back() {
    let (document, mut editor) = the_first_sketch_open();

    click_on(&document, &mut editor, the_row_of(0, 1), false);
    click_on(&document, &mut editor, the_row_of(0, 0), true);
    assert_eq!(
        editor.selection(),
        [a_trait(1), a_trait(0)],
        "the modifier held, a click does not add to what is held",
    );

    click_on(&document, &mut editor, the_row_of(0, 1), true);
    assert_eq!(
        editor.selection(),
        [a_trait(0)],
        "the modifier held, a click on what is held does not take it back",
    );
}

#[test]
fn a_click_on_a_dimension_or_a_rule_of_the_open_sketch_selects_it() {
    let (mut document, mut editor) = the_first_sketch_open();
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Equal {
            first: SegmentId(0),
            second: SegmentId(2),
        },
    });
    let drawing = &document.sketches()[0];
    assert!(
        !drawing.constraints().is_empty(),
        "the two opposite sides of the rectangle were refused an equal length",
    );
    let dimension = Selection::Dimension(drawing.dimensions()[0].target);
    let rule = Selection::Rule(drawing.constraints()[0]);

    for (row, what) in [
        (Points::Dimension { sketch: 0, rank: 0 }, dimension),
        (Points::Rule { sketch: 0, rank: 0 }, rule),
    ] {
        let action = click_on(&document, &mut editor, row, false);

        assert_eq!(
            action,
            TreeAction::Select {
                what,
                adding: false
            }
        );
        assert_eq!(
            editor.selection(),
            [what],
            "a click on {row:?} does not leave it held",
        );
    }
}

#[test]
fn a_click_on_a_row_of_a_closed_sketch_lights_it_and_opens_nothing() {
    let document = a_part();
    let nothing_open = SketchEditor::default();
    let mut the_other_open = SketchEditor::default();
    the_other_open.begin_editing(1, WorkPlane::XZ);

    for editor in [&nothing_open, &the_other_open] {
        for row in [the_row_of(0, 0), Points::Area { sketch: 0, rank: 0 }] {
            assert_eq!(
                clicked(&document, editor, row, false),
                TreeAction::Light(row),
                "a click on {row:?} of a sketch that is not open, while {:?} is, \
                 does something other than light it",
                editor.active_sketch(),
            );
        }
    }
}

#[test]
fn the_tree_marks_what_the_viewport_holds() {
    let (document, mut editor) = the_first_sketch_open();
    editor.tool = Tool::Select;
    editor.reset_pending();
    let dimension = Selection::Dimension(document.sketches()[0].dimensions()[0].target);
    editor
        .select_state()
        .expect("the selection tool is in hand")
        .held = vec![a_trait(0), dimension];

    assert!(
        holds(&document, &editor, the_row_of(0, 0)),
        "the trait the viewport holds is not marked in the tree",
    );
    assert!(
        holds(&document, &editor, Points::Dimension { sketch: 0, rank: 0 }),
        "the dimension the viewport holds is not marked in the tree",
    );
    assert!(
        !holds(&document, &editor, the_row_of(0, 1)),
        "a trait the viewport does not hold is marked in the tree",
    );
    assert!(
        !holds(&document, &editor, the_row_of(1, 0)),
        "the same trait in a sketch that is not open is marked in the tree",
    );
}

#[test]
fn a_lit_row_is_marked_only_while_its_sketch_is_not_being_drawn_on() {
    let document = a_part();
    let mut viewport = ViewportState::default();
    viewport.light(the_row_of(0, 0));
    let nothing_open = SketchEditor::default();
    let (_, drawing_on_it) = the_first_sketch_open();

    assert!(
        marked(&document, &nothing_open, &viewport, the_row_of(0, 0)),
        "a row lit on the canvas is not marked in the tree",
    );
    assert!(
        !marked(&document, &nothing_open, &viewport, the_row_of(0, 1)),
        "a row nothing lit is marked in the tree",
    );
    assert!(
        !marked(&document, &drawing_on_it, &viewport, the_row_of(0, 0)),
        "a row lit before its sketch was opened is still marked while it is drawn on",
    );
}

#[test]
fn a_click_in_the_tree_while_a_rule_waits_for_its_second_pick_does_nothing() {
    let (document, mut editor) = the_first_sketch_open();
    let rule = Tool::Constrain(Rule::Perpendicular);
    let first_pick = RulePick::Element(Element::Segment(SegmentId(1)));
    editor.tool = rule;
    editor.tool_state = ToolState::Constrain {
        picks: vec![first_pick],
    };

    let action = click_on(&document, &mut editor, the_row_of(0, 0), false);

    assert_eq!(
        action,
        TreeAction::None,
        "a click in the tree asks something of a busy tool"
    );
    assert_eq!(
        editor.tool, rule,
        "a click in the tree took the rule out of hand"
    );
    assert_eq!(
        editor.rule_picks(),
        [first_pick],
        "a click in the tree dropped the rule's first pick",
    );
    assert!(
        editor.selection().is_empty(),
        "a click in the tree selected something"
    );
}

#[test]
fn an_area_row_of_the_open_sketch_lights_the_area() {
    let (document, editor) = the_first_sketch_open();
    let area = Points::Area { sketch: 0, rank: 0 };

    assert!(
        clickable(&document, &editor, area),
        "the open sketch's area row is no click"
    );
    assert_eq!(
        clicked(&document, &editor, area, false),
        TreeAction::Light(area)
    );
}

#[test]
fn a_dimension_or_a_rule_of_a_closed_sketch_is_no_click_at_all() {
    let mut document = a_part();
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Equal {
            first: SegmentId(0),
            second: SegmentId(2),
        },
    });
    let nothing_open = SketchEditor::default();
    let (_, drawing_on_it) = the_first_sketch_open();
    let read_only = [
        Points::Dimension { sketch: 0, rank: 0 },
        Points::Rule { sketch: 0, rank: 0 },
    ];
    let live = [the_row_of(0, 0), Points::Area { sketch: 0, rank: 0 }];

    for row in read_only {
        assert!(
            !clickable(&document, &nothing_open, row),
            "{row:?} of a closed sketch is drawn as a click that does nothing",
        );
        assert_eq!(
            clicked(&document, &nothing_open, row, false),
            TreeAction::None
        );
        assert!(
            clickable(&document, &drawing_on_it, row),
            "{row:?} of the open sketch is no click, though a click selects it",
        );
    }
    for row in live {
        assert!(
            clickable(&document, &nothing_open, row),
            "{row:?} of a closed sketch is no click, though a click lights it",
        );
    }
}
