//! A fillet or a chamfer already laid is changed in place: its values, its
//! kind, the side a chamfer measures from, whether it is construction, or the
//! cut taken off and its corner sharp again. Each change is one step.
//!
//! Closes #618.
//! - clicking a cut corner with « Congé » or « Chanfrein » replaces its value,
//!   its kind or its chamfer mode —
//!   `a_corner_cut_again_takes_its_new_kind_and_value`; asking for the two
//!   sides wherever that is ambiguous is the tool's, unchanged since #617,
//!   held in `cao_app` by
//!   `a_click_on_a_point_too_crowded_to_be_a_corner_asks_for_the_two_traits`
//! - a corner of a linked group, re-cut, leaves the group with its new value —
//!   `a_corner_of_a_group_cut_again_leaves_its_group`
//! - a double-click on a cut's dimension retypes it, every corner of a group
//!   following; a value too big is held back with the largest offered —
//!   `a_group_given_new_values_changes_every_corner_in_place`. The
//!   double-click opening the values is held in `cao_app` by
//!   `a_double_click_opens_a_fillet_s_values_with_its_whole_group` and
//!   `retyping_the_values_of_a_whole_group_changes_it_in_place`; the value too
//!   big by `a_value_too_big_is_held_back_and_the_field_offers_the_largest`,
//!   the same tool the values open in
//! - « Cote de rayon » put on a fillet opens the fillet's own value — no
//!   test: it is the dimension tool's, held in `cao_app` by
//!   `a_radius_dimension_on_a_fillet_opens_its_value`
//! - Suppr on a selected cut makes the corner sharp again —
//!   `a_cut_taken_off_leaves_its_corner_sharp`; the key is held in `cao_app`
//!   by `suppr_takes_a_held_cut_off_and_its_corner_is_sharp_again`
//! - « Inverser les côtés » swaps a chamfer's two sides, chamfer by chamfer,
//!   from the bar under the ribbon — `a_chamfer_swapped_takes_from_its_other_side`;
//!   the button is drawn by the ribbon, which has no net, from
//!   `chamfer_to_swap`, held in `cao_app` by
//!   `the_sides_of_a_held_chamfer_are_swapped`
//! - « Construction » with a cut selected switches that cut alone —
//!   `one_cut_switched_to_construction_leaves_the_outline_sharp_there`; the
//!   toggle is held in `cao_app` by
//!   `construction_with_a_cut_held_switches_that_cut_alone`. That switching a
//!   side leaves its cut as it was cannot happen yet: no gesture switches a
//!   trait already drawn to construction. A cut keeps its own state, which is
//!   what the second test asserts of the sides the other way round
//! - each change is one step of the history —
//!   `every_change_is_taken_back_by_one_undo`,
//!   `swapping_a_chamfer_and_taking_cuts_off_with_traits_are_each_taken_back_by_one_undo`
//!
//! What must not break: a value retyped from a variable keeps following it —
//! `a_group_retyped_from_a_variable_follows_it`.
//!
//! « Mesurer » on a cut, set aside from #617 and noted on this issue, comes
//! with it: a fillet reads as its radius, held in `cao_app` by
//! `a_measure_taken_of_a_fillet_reads_its_radius`.

use std::f64::consts::PI;

use cao_part::history::{ChamferAsked, CutAsked, CutChange, Operation, PointRef};
use cao_part::{Formula, History, PartDocument, PartState, VariableChange, VariableId};
use cao_sketch::{Chamfer, Corner, CornerCutId, CutShape, CutSide, SegmentId, Sketch, WorkPlane};
use glam::DVec2;

/// How near the arithmetic an area read from its curves comes.
const EXACT: f64 = 1e-9;

/// A sketch on XY with a plate forty by twenty, its four corners rounded
/// together by five.
fn a_rounded_plate() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    history.push(Operation::Fillet {
        sketch: 0,
        corners: the_four_corners(),
        radius: 5.0.into(),
        construction: false,
    });
    history
}

fn the_four_corners() -> Vec<Corner> {
    (0..4)
        .map(|side| Corner::Between(SegmentId(side).into(), SegmentId((side + 1) % 4).into()))
        .collect()
}

fn the_drawing(history: &History) -> Sketch {
    PartState::rebuild(history).sketches[0].clone()
}

fn the_one_area(sketch: &Sketch) -> f64 {
    let regions = sketch.regions();
    assert_eq!(regions.len(), 1, "one closed area");
    regions[0].area()
}

fn near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected,
        "{what}: {made} where the arithmetic gives {expected}"
    );
}

fn change(history: &mut History, change: CutChange) {
    history.push(Operation::CutChange(change));
}

#[test]
fn a_corner_cut_again_takes_its_new_kind_and_value() {
    let mut history = a_rounded_plate();

    history.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        mode: Chamfer::Equal(3.0).into(),
        construction: false,
    });

    let sketch = the_drawing(&history);
    let shapes: Vec<CutShape> = sketch
        .live_corner_cuts()
        .map(|(_, cut)| cut.shape)
        .collect();
    assert_eq!(shapes.len(), 4, "still one cut a corner");
    assert_eq!(
        shapes
            .iter()
            .filter(|shape| matches!(shape, CutShape::Chamfer(_)))
            .count(),
        1
    );
    near(
        the_one_area(&sketch),
        800.0 - 3.0 * (4.0 - PI) * 25.0 / 4.0 - 4.5,
        "three fillets and one chamfer",
    );
}

#[test]
fn a_corner_of_a_group_cut_again_leaves_its_group() {
    let mut history = a_rounded_plate();

    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        radius: 3.0.into(),
        construction: false,
    });

    let sketch = the_drawing(&history);
    let (again, _) = sketch
        .live_corner_cuts()
        .find(|(_, cut)| cut.shape == CutShape::Fillet { radius: 3.0 })
        .expect("the corner cut again");
    assert_eq!(sketch.cuts_grouped_with(again), vec![again]);
    assert_eq!(sketch.cuts_grouped_with(CornerCutId(0)).len(), 3);
}

#[test]
fn a_group_given_new_values_changes_every_corner_in_place() {
    let mut history = a_rounded_plate();

    change(
        &mut history,
        CutChange::Reshape {
            sketch: 0,
            cut: CornerCutId(2),
            value: CutAsked::Fillet(3.0.into()),
        },
    );

    let sketch = the_drawing(&history);
    assert!(
        sketch
            .live_corner_cuts()
            .all(|(_, cut)| cut.shape == CutShape::Fillet { radius: 3.0 })
    );
    assert_eq!(
        sketch.cuts_grouped_with(CornerCutId(0)).len(),
        4,
        "one group still"
    );
    near(
        the_one_area(&sketch),
        800.0 - (4.0 - PI) * 9.0,
        "the plate rounded by three",
    );
}

#[test]
fn a_cut_taken_off_leaves_its_corner_sharp() {
    let mut history = a_rounded_plate();

    change(
        &mut history,
        CutChange::TakeOff {
            sketch: 0,
            cuts: vec![CornerCutId(1)],
        },
    );

    let sketch = the_drawing(&history);
    assert_eq!(sketch.live_corner_cuts().count(), 3);
    near(
        the_one_area(&sketch),
        800.0 - 3.0 * (4.0 - PI) * 25.0 / 4.0,
        "three corners rounded, one sharp",
    );
}

#[test]
fn a_chamfer_swapped_takes_from_its_other_side() {
    let mut history = a_rounded_plate();
    history.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        mode: ChamferAsked::from(Chamfer::Sided {
            first: 2.0,
            second: 4.0,
        }),
        construction: false,
    });
    let chamfer = CornerCutId(4);
    let before = the_drawing(&history)
        .drawn_cut(chamfer)
        .expect("the chamfer");

    change(
        &mut history,
        CutChange::SwapSides {
            sketch: 0,
            cut: chamfer,
        },
    );

    let after = the_drawing(&history)
        .drawn_cut(chamfer)
        .expect("the chamfer");
    let taken_from = |cut: cao_sketch::DrawnCut, side: SegmentId| {
        let rank = cut
            .sides
            .iter()
            .position(|on| *on == CutSide::Trait(side))
            .expect("a side");
        cut.touches[rank].distance(cut.corner)
    };
    near(taken_from(before, SegmentId(0)), 2.0, "before");
    near(
        taken_from(after, SegmentId(0)),
        4.0,
        "after: the side that gave two gives four",
    );
}

#[test]
fn one_cut_switched_to_construction_leaves_the_outline_sharp_there() {
    let mut history = a_rounded_plate();

    change(
        &mut history,
        CutChange::Construction {
            sketch: 0,
            cut: CornerCutId(0),
            construction: true,
        },
    );

    let sketch = the_drawing(&history);
    let construction: Vec<bool> = sketch
        .live_corner_cuts()
        .map(|(_, cut)| cut.construction)
        .collect();
    assert_eq!(construction, vec![true, false, false, false]);
    assert!(
        sketch.segments().iter().all(|side| !side.construction),
        "the sides are left as they were"
    );
    near(
        the_one_area(&sketch),
        800.0 - 3.0 * (4.0 - PI) * 25.0 / 4.0,
        "that corner sharp in the outline",
    );
}

#[test]
fn every_change_is_taken_back_by_one_undo() {
    let changes = [
        CutChange::Reshape {
            sketch: 0,
            cut: CornerCutId(0),
            value: CutAsked::Fillet(3.0.into()),
        },
        CutChange::TakeOff {
            sketch: 0,
            cuts: vec![CornerCutId(0)],
        },
        CutChange::Construction {
            sketch: 0,
            cut: CornerCutId(0),
            construction: true,
        },
    ];
    for one in changes {
        let mut history = a_rounded_plate();
        let before = the_drawing(&history).drawn_cuts();
        change(&mut history, one.clone());
        assert_ne!(
            the_drawing(&history).drawn_cuts(),
            before,
            "{one:?} changes something"
        );

        history.undo();

        assert_eq!(
            the_drawing(&history).drawn_cuts(),
            before,
            "{one:?} taken back"
        );
    }
}

#[test]
fn swapping_a_chamfer_and_taking_cuts_off_with_traits_are_each_taken_back_by_one_undo() {
    let mut history = a_rounded_plate();
    history.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        mode: ChamferAsked::from(Chamfer::Sided {
            first: 2.0,
            second: 4.0,
        }),
        construction: false,
    });
    let before = the_drawing(&history).drawn_cuts();

    change(
        &mut history,
        CutChange::SwapSides {
            sketch: 0,
            cut: CornerCutId(4),
        },
    );
    assert_ne!(the_drawing(&history).drawn_cuts(), before);
    history.undo();
    assert_eq!(
        the_drawing(&history).drawn_cuts(),
        before,
        "the swap taken back"
    );

    history.push(Operation::Gesture(vec![
        Operation::CutChange(CutChange::TakeOff {
            sketch: 0,
            cuts: vec![CornerCutId(1)],
        }),
        Operation::EraseMany {
            sketch: 0,
            elements: vec![cao_sketch::Element::Segment(SegmentId(2))],
            dimensions: Vec::new(),
            constraints: Vec::new(),
        },
    ]));
    assert_ne!(the_drawing(&history).drawn_cuts(), before);
    history.undo();
    assert_eq!(
        the_drawing(&history).drawn_cuts(),
        before,
        "the gesture taken back whole"
    );
}

#[test]
fn a_group_retyped_from_a_variable_follows_it() {
    let mut document = PartDocument::new("Plaque", "2026-11-02T09:00:00Z".parse().expect("a date"));
    document.apply(Operation::Variable(VariableChange::Added {
        name: "arrondi".to_string(),
        formula: Formula::Number(3.0),
    }));
    for operation in a_rounded_plate().applied_operations() {
        document.apply(operation.clone());
    }
    let radius = document
        .variables()
        .read("arrondi")
        .expect("a formula that reads");
    document.apply(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CornerCutId(0),
        value: CutAsked::Fillet(radius),
    }));

    document
        .change_variable(VariableChange::Edited {
            variable: VariableId(0),
            name: "arrondi".to_string(),
            formula: Formula::Number(4.0),
        })
        .expect("a smaller rounding fits");

    assert!(
        document.sketches()[0]
            .live_corner_cuts()
            .all(|(_, cut)| cut.shape == CutShape::Fillet { radius: 4.0 })
    );
}

#[test]
fn a_group_retyped_from_a_variable_still_follows_it_once_compacted() {
    let mut history = History::default();
    history.push(Operation::Variable(VariableChange::Added {
        name: "arrondi".to_string(),
        formula: Formula::Number(3.0),
    }));
    for operation in a_rounded_plate().applied_operations() {
        history.push(operation.clone());
    }
    let radius = Formula::Variable(VariableId(0));
    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CornerCutId(0),
        value: CutAsked::Fillet(radius),
    }));

    let mut compacted = cao_part::compact(&history);
    compacted.push(Operation::Variable(VariableChange::Edited {
        variable: VariableId(0),
        name: "arrondi".to_string(),
        formula: Formula::Number(4.0),
    }));

    assert!(
        PartState::rebuild(&compacted).sketches[0]
            .live_corner_cuts()
            .all(|(_, cut)| cut.shape == CutShape::Fillet { radius: 4.0 })
    );
}

#[test]
fn a_cut_switched_to_construction_stays_alone_in_it_once_compacted() {
    let mut history = a_rounded_plate();
    change(
        &mut history,
        CutChange::Construction {
            sketch: 0,
            cut: CornerCutId(0),
            construction: true,
        },
    );

    let compacted = PartState::rebuild(&cao_part::compact(&history));

    let construction: Vec<bool> = compacted.sketches[0]
        .live_corner_cuts()
        .map(|(_, cut)| cut.construction)
        .collect();
    assert_eq!(construction, vec![true, false, false, false]);
}
