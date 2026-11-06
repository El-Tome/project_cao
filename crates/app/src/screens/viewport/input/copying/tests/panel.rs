//! What the panel of the mirror and the two patterns is held to: which slot a
//! click fills, what `Entrée` does, what is laid and what is left behind.
//!
//! Closes #321.
//! - the panel stands over the view, always in front, and is dragged anywhere
//!   in the view by its title — no test: how it is drawn and dragged is checked
//!   on rendered frames, which no assertion here reaches
//! - it comes back where it was last left, one position shared by every tool
//!   that uses it, kept across restarts — no test: here; held in
//!   `crates/app/tests/copying_in_a_panel.rs` and in `cao_prefs`'s
//!   `settings/tests.rs`, each under its own `Closes #321`
//! - a position that no longer fits the view is brought back inside it — no
//!   test: here; held in `ui/floating_panel/tests.rs`, under its own
//!   `Closes #321`
//! - the keyboard writes into the panel; the mouse stays entirely free for the
//!   drawing — no test: the panel takes a click only over itself, which is
//!   egui's own hit-testing, and where it stands is checked on rendered frames
//! - Elements: a click in the drawing adds an element —
//!   `a_click_in_the_drawing_adds_the_element_under_it`
//! - Elements: a click on one already held removes it —
//!   `a_click_on_an_element_already_held_lets_it_go`
//! - Elements: a box drawn in the drawing adds what it takes —
//!   `a_box_drawn_in_the_drawing_adds_what_it_takes`; a hand moving the view
//!   pulls no box — `a_box_pulled_with_the_button_that_pans_takes_nothing`,
//!   `a_box_pulled_while_orbiting_takes_nothing`
//! - Axis / centre / direction: filled by the next click on a trait, an axis or
//!   a point, depending on the tool —
//!   `an_ordinary_trait_clicked_in_the_target_slot_is_the_mirror_s_axis`,
//!   `a_point_clicked_in_the_target_slot_is_the_circular_pattern_s_centre`,
//!   `a_trait_clicked_in_the_target_slot_is_the_rectangular_pattern_s_direction`
//! - Values: step and count for the circular pattern, two steps and two counts
//!   for the rectangular one, none for the mirror —
//!   `each_copy_tool_has_the_values_it_needs_and_no_more`, each named by its
//!   kind — `each_value_of_the_panel_is_named_by_its_kind`
//! - Validate / cancel — `the_panel_s_cancel_hands_its_elements_to_the_selection_tool_and_lays_nothing`;
//!   Validate is held under Leaving
//! - one slot is active and shown as such — no test: how it is shown is
//!   checked on rendered frames; which one it is, by every test below
//! - a click in the drawing fills the active slot and only that one, so an
//!   ordinary trait can be an axis and a construction trait can be copied —
//!   `an_ordinary_trait_clicked_in_the_target_slot_is_the_mirror_s_axis`,
//!   `a_construction_trait_clicked_while_elements_is_active_is_copied`,
//!   `a_click_in_the_drawing_while_a_value_or_validate_is_active_fills_nothing`
//! - clicking a slot in the panel makes it active, at any time, to change what
//!   it holds — `an_axis_already_picked_is_changed_by_clicking_its_slot_again`
//! - the tool opens on Elements, or on the slot below it when a selection was
//!   already in hand — `the_mirror_opens_on_its_elements_when_nothing_was_selected`,
//!   `a_copy_tool_taken_with_a_selection_in_hand_opens_on_its_target`
//! - once a slot is filled, the slot below it becomes active, whether or not
//!   that one is already filled; picking the mirror's axis lands on Validate —
//!   `picking_the_mirror_s_axis_lands_on_validate`,
//!   `a_centre_picked_again_moves_onto_values_already_typed`
//! - `Entrée` also moves down one slot — `enter_moves_down_one_slot`
//! - on Validate, `Entrée` lays the copy if everything is filled —
//!   `enter_on_validate_lays_the_mirror_as_one_step`
//! - otherwise the first empty slot becomes active and is marked —
//!   `enter_on_validate_with_a_slot_empty_activates_and_marks_the_first_empty_one`
//! - while the writing cursor is in a field of the panel, no letter shortcut
//!   fires; `Entrée` and `Échap` still do what is said here — no test: here;
//!   held in `crates/app/tests/copying_in_a_panel.rs`, which reaches the
//!   keyboard
//! - Validate: the copy is laid, the tool stays in hand, and the panel is
//!   emptied entirely — elements, target and values — back on Elements —
//!   `a_copy_laid_leaves_the_tool_in_hand_and_the_panel_empty_on_its_elements`
//! - `Échap` or another tool taken from the ribbon: nothing is laid, the panel
//!   closes — `another_tool_taken_closes_the_panel_and_lays_nothing`; `Échap`
//!   and « Annuler » hand what was held to the Selection tool, as decided on
//!   the branch — `the_panel_s_cancel_hands_its_elements_to_the_selection_tool_and_lays_nothing`,
//!   and `Échap` in the driver
//! - the elements held stay selected if the tool reached for works on a
//!   selection; otherwise the selection is dropped —
//!   `the_selection_tool_taken_from_the_panel_keeps_its_elements_selected`,
//!   `a_drawing_tool_taken_from_the_panel_drops_its_elements`
//! - the preview (#311) updates as soon as the panel holds enough to lay
//!   something — `the_preview_shows_across_the_axis_held_not_under_the_cursor`,
//!   `the_preview_waits_for_the_panel_to_hold_enough`
//! - must not break: the preview of the copy before it is laid (#311) —
//!   `the_preview_shows_across_the_axis_held_not_under_the_cursor`; the
//!   patterns' in `preview/tests.rs`
//! - must not break: a pattern opens on a step its selection already fits in
//!   (#322) — `a_rectangular_pattern_opens_its_values_on_a_step_its_elements_fit_in`;
//!   from a selection in hand, in `opening/tests.rs`
//! - must not break: a selection made with the Selection tool is what the tool
//!   starts with — `a_copy_tool_taken_with_a_selection_in_hand_opens_on_its_target`
//! - must not break: a mirror, a circular pattern or a rectangular pattern is
//!   still one step in the history, undone in one go, and laid by the same
//!   operations as today — `enter_on_validate_lays_the_mirror_as_one_step`,
//!   `a_circular_pattern_laid_from_the_panel_is_one_step`,
//!   `a_rectangular_pattern_laid_from_the_panel_is_one_step`
//! - must not break: a mirrored arc comes back as an arc (#317), the mirror
//!   image of the original — `an_arc_mirrored_from_the_panel_comes_back_an_arc`
//! - the panel keeps the width of its widest row as the rows change — no test:
//!   a look, checked on rendered frames

use cao_part::history::RepeatsAsked;
use cao_part::{Formula, Operation, PointRef};
use cao_prefs::Command;
use cao_sketch::{ArcId, ChosenAxis, Element, SegmentId, Selection, WorkPlane};
use glam::DVec2;

use super::bench::Bench;
use crate::screens::SketchContext;
use crate::screens::sketch::{CopySlot, CopyValue, Tool};
use crate::screens::viewport::input::handle_sketch_input;
use crate::screens::viewport::navigation::handle_navigation;
use crate::screens::viewport::{ViewScale, ViewportState, to_ndc};

const ON_THE_TRAIT_TO_COPY: DVec2 = DVec2::new(-10.0, 0.0);
const ON_THE_ORDINARY_TRAIT: DVec2 = DVec2::new(2.0, 0.0);
const ON_THE_CONSTRUCTION_TRAIT: DVec2 = DVec2::new(0.0, 20.0);
const THE_ORDINARY_TRAIT_S_FOOT: DVec2 = DVec2::new(2.0, -5.0);
const THE_ORDINARY_TRAIT_S_HEAD: DVec2 = DVec2::new(2.0, 5.0);
const FAR_FROM_EVERYTHING: DVec2 = DVec2::new(40.0, 40.0);

const TRAIT_TO_COPY: SegmentId = SegmentId(0);
const ORDINARY_TRAIT: SegmentId = SegmentId(1);
const CONSTRUCTION_TRAIT: SegmentId = SegmentId(2);

/// Three traits clear of each other and of both sketch axes: one to copy, ten
/// units tall, an ordinary one upright beside it, and a construction one lying
/// above them both.
fn three_traits() -> Bench {
    Bench::drawn(vec![
        Bench::a_trait(DVec2::new(-10.0, -5.0), DVec2::new(-10.0, 5.0), false),
        Bench::a_trait(THE_ORDINARY_TRAIT_S_FOOT, THE_ORDINARY_TRAIT_S_HEAD, false),
        Bench::a_trait(DVec2::new(-5.0, 20.0), DVec2::new(5.0, 20.0), true),
    ])
}

/// The tool taken, the trait to copy held, and `Entrée` leaving Elements.
fn holding_the_trait_to_copy(command: Command) -> Bench {
    let mut bench = three_traits();
    bench.take(command);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench
}

fn traits(bench: &Bench) -> usize {
    bench.document.sketches()[0].segments().len()
}

#[test]
fn the_mirror_opens_on_its_elements_when_nothing_was_selected() {
    let mut bench = three_traits();

    bench.take(Command::ToolMirror);

    assert_eq!(bench.active(), CopySlot::Elements);
    assert_eq!(bench.elements(), Vec::new());
}

#[test]
fn a_click_in_the_drawing_adds_the_element_under_it() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);

    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click(ON_THE_ORDINARY_TRAIT);

    assert_eq!(
        bench.elements(),
        [
            bench.a_trait_and_its_ends(TRAIT_TO_COPY),
            bench.a_trait_and_its_ends(ORDINARY_TRAIT)
        ]
        .concat()
    );
    assert_eq!(
        bench.active(),
        CopySlot::Elements,
        "Elements is a list: a click adds to it and leaves it active for the next"
    );
}

#[test]
fn a_click_on_an_element_already_held_lets_it_go() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click(ON_THE_ORDINARY_TRAIT);

    bench.click(ON_THE_TRAIT_TO_COPY);

    assert_eq!(bench.elements(), bench.a_trait_and_its_ends(ORDINARY_TRAIT));
}

#[test]
fn a_construction_trait_clicked_while_elements_is_active_is_copied() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);

    bench.click(ON_THE_CONSTRUCTION_TRAIT);

    assert_eq!(
        bench.elements(),
        bench.a_trait_and_its_ends(CONSTRUCTION_TRAIT)
    );
    assert_eq!(
        bench.active(),
        CopySlot::Elements,
        "a construction trait is not guessed to be the axis"
    );
}

#[test]
fn an_ordinary_trait_clicked_in_the_target_slot_is_the_mirror_s_axis() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);

    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_ORDINARY_TRAIT);
    bench.click_the_slot(CopySlot::Elements);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click_the_slot(CopySlot::Validate);
    bench.editor.cursor = Some(ON_THE_CONSTRUCTION_TRAIT);
    bench.enter();

    assert_eq!(
        bench.last_laid(),
        Some(&Operation::Mirror {
            sketch: 0,
            elements: bench.a_trait_and_its_ends(TRAIT_TO_COPY),
            axis: ChosenAxis::Trait(ORDINARY_TRAIT),
        }),
        "the axis is the trait clicked while its slot was active, picked before \
         the elements and kept, whatever the cursor is over when the copy is laid"
    );
}

#[test]
fn a_point_clicked_in_the_target_slot_is_the_circular_pattern_s_centre() {
    let mut bench = holding_the_trait_to_copy(Command::ToolCircularPattern);
    let centre = bench.point_at(THE_ORDINARY_TRAIT_S_FOOT);

    bench.click(THE_ORDINARY_TRAIT_S_FOOT);
    assert_eq!(
        bench.active(),
        CopySlot::Value(0),
        "the centre moves down one"
    );
    bench.enter();
    bench.enter();
    bench.editor.cursor = Some(THE_ORDINARY_TRAIT_S_HEAD);
    bench.enter();

    assert_eq!(
        bench.last_laid(),
        Some(&Operation::CircularPattern {
            sketch: 0,
            elements: bench.a_trait_and_its_ends(TRAIT_TO_COPY),
            centre,
            degrees: Formula::Number(90.0),
            count: Formula::Number(4.0),
        }),
        "turned about the point clicked while the centre's slot was active, \
         whatever point the cursor is over when it is laid"
    );
}

#[test]
fn a_trait_clicked_in_the_target_slot_is_the_rectangular_pattern_s_direction() {
    let mut bench = holding_the_trait_to_copy(Command::ToolRectangularPattern);

    bench.click(ON_THE_ORDINARY_TRAIT);
    assert_eq!(
        bench.active(),
        CopySlot::Value(0),
        "the direction moves down one"
    );
    for _ in 0..4 {
        bench.enter();
    }
    bench.editor.cursor = Some(FAR_FROM_EVERYTHING);
    bench.enter();

    let Some(Operation::RectangularPattern {
        elements,
        direction,
        along,
        across,
        ..
    }) = bench.last_laid()
    else {
        panic!("a rectangular pattern is laid: {:?}", bench.last_laid());
    };
    assert_eq!(elements, &bench.a_trait_and_its_ends(TRAIT_TO_COPY));
    assert_eq!(direction, &ChosenAxis::Trait(ORDINARY_TRAIT));
    let opened = RepeatsAsked {
        step: Formula::Number(10.0),
        count: Formula::Number(2.0),
    };
    assert_eq!(
        (along, across),
        (&opened, &opened),
        "laid with the values the panel opened on"
    );
}

#[test]
fn each_copy_tool_has_the_values_it_needs_and_no_more() {
    let walked = |command: Command| {
        let mut bench = holding_the_trait_to_copy(command);
        let mut slots = vec![bench.active()];
        while bench.active() != CopySlot::Validate && slots.len() < 8 {
            bench.enter();
            slots.push(bench.active());
        }
        slots
    };

    assert_eq!(
        walked(Command::ToolMirror),
        [CopySlot::Target, CopySlot::Validate]
    );
    assert_eq!(
        walked(Command::ToolCircularPattern),
        [
            CopySlot::Target,
            CopySlot::Value(0),
            CopySlot::Value(1),
            CopySlot::Validate
        ]
    );
    assert_eq!(
        walked(Command::ToolRectangularPattern),
        [
            CopySlot::Target,
            CopySlot::Value(0),
            CopySlot::Value(1),
            CopySlot::Value(2),
            CopySlot::Value(3),
            CopySlot::Validate
        ]
    );
}

#[test]
fn the_panel_s_cancel_hands_its_elements_to_the_selection_tool_and_lays_nothing() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_ORDINARY_TRAIT);
    let laid_before = bench.laid();

    bench.editor.cancel_the_copy();

    assert!(bench.editor.copy_panel().is_none(), "the panel is closed");
    assert_eq!(bench.editor.tool, Tool::Select);
    assert_eq!(
        bench.editor.selection(),
        bench
            .a_trait_and_its_ends(TRAIT_TO_COPY)
            .into_iter()
            .map(Selection::Element)
            .collect::<Vec<_>>(),
        "what the panel held stays selected"
    );
    assert_eq!(
        bench.editor.message, None,
        "the copy tool's prompt goes with it"
    );
    bench.enter();
    assert_eq!(
        bench.laid(),
        laid_before,
        "nothing is laid, not even by an Entrée that follows"
    );
}

#[test]
fn each_value_of_the_panel_is_named_by_its_kind() {
    let kinds = |command: Command| {
        let mut bench = three_traits();
        bench.take(command);
        bench.panel().value_kinds().to_vec()
    };

    assert_eq!(kinds(Command::ToolMirror), []);
    assert_eq!(
        kinds(Command::ToolCircularPattern),
        [CopyValue::StepInDegrees, CopyValue::Count]
    );
    assert_eq!(
        kinds(Command::ToolRectangularPattern),
        [
            CopyValue::StepInMillimetres,
            CopyValue::Count,
            CopyValue::StepInMillimetres,
            CopyValue::Count
        ]
    );
}

#[test]
fn a_click_in_the_drawing_while_a_value_or_validate_is_active_fills_nothing() {
    let mut bench = holding_the_trait_to_copy(Command::ToolCircularPattern);
    let centre = bench.point_at(THE_ORDINARY_TRAIT_S_FOOT);
    bench.click(THE_ORDINARY_TRAIT_S_FOOT);
    assert_eq!(bench.active(), CopySlot::Value(0));

    bench.click(THE_ORDINARY_TRAIT_S_HEAD);
    bench.click(ON_THE_CONSTRUCTION_TRAIT);
    assert_eq!(bench.active(), CopySlot::Value(0), "a value stays active");
    bench.click_the_slot(CopySlot::Validate);
    bench.click(THE_ORDINARY_TRAIT_S_HEAD);
    bench.click(ON_THE_CONSTRUCTION_TRAIT);
    assert_eq!(bench.active(), CopySlot::Validate, "Validate stays active");
    assert_eq!(bench.elements(), bench.a_trait_and_its_ends(TRAIT_TO_COPY));
    bench.enter();

    assert!(
        matches!(
            bench.last_laid(),
            Some(Operation::CircularPattern { centre: laid_about, .. }) if *laid_about == centre
        ),
        "the centre is still the one picked in its slot: {:?}",
        bench.last_laid()
    );
}

#[test]
fn an_axis_already_picked_is_changed_by_clicking_its_slot_again() {
    let mut bench = holding_the_trait_to_copy(Command::ToolMirror);
    bench.click(ON_THE_ORDINARY_TRAIT);

    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_CONSTRUCTION_TRAIT);
    assert_eq!(bench.active(), CopySlot::Validate);
    bench.enter();

    assert_eq!(
        bench.last_laid(),
        Some(&Operation::Mirror {
            sketch: 0,
            elements: bench.a_trait_and_its_ends(TRAIT_TO_COPY),
            axis: ChosenAxis::Trait(CONSTRUCTION_TRAIT),
        }),
        "the second axis picked replaces the first"
    );
}

#[test]
fn a_copy_tool_taken_with_a_selection_in_hand_opens_on_its_target() {
    for command in [
        Command::ToolMirror,
        Command::ToolCircularPattern,
        Command::ToolRectangularPattern,
    ] {
        let mut bench = three_traits();
        bench.select(&[Element::Segment(TRAIT_TO_COPY)]);

        bench.take(command);

        assert_eq!(bench.active(), CopySlot::Target, "{command:?}");
        assert_eq!(
            bench.elements(),
            vec![Element::Segment(TRAIT_TO_COPY)],
            "{command:?} starts with what the Selection tool held"
        );
    }
}

#[test]
fn picking_the_mirror_s_axis_lands_on_validate() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);

    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_ORDINARY_TRAIT);

    assert_eq!(bench.active(), CopySlot::Validate);
    assert_eq!(
        bench.elements(),
        bench.a_trait_and_its_ends(TRAIT_TO_COPY),
        "the trait clicked for the axis is not added to what is copied"
    );
}

#[test]
fn a_centre_picked_again_moves_onto_values_already_typed() {
    let mut bench = holding_the_trait_to_copy(Command::ToolCircularPattern);
    bench.click(THE_ORDINARY_TRAIT_S_FOOT);
    bench.type_value(0, "30");

    bench.click_the_slot(CopySlot::Target);
    bench.click(THE_ORDINARY_TRAIT_S_HEAD);

    assert_eq!(
        bench.active(),
        CopySlot::Value(0),
        "down one, onto the step already typed, not on to the first empty slot"
    );
    assert_eq!(bench.value(0), Some(30.0));
}

#[test]
fn enter_moves_down_one_slot() {
    let mut bench = three_traits();
    bench.take(Command::ToolCircularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);

    bench.enter();
    assert_eq!(bench.active(), CopySlot::Target);
    bench.enter();
    assert_eq!(
        bench.active(),
        CopySlot::Value(0),
        "down one even with the centre still empty"
    );
}

#[test]
fn enter_on_validate_lays_the_mirror_as_one_step() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_ORDINARY_TRAIT);
    let laid_before = bench.laid();
    let traits_before = traits(&bench);

    bench.editor.cursor = Some(FAR_FROM_EVERYTHING);
    bench.enter();

    assert_eq!(bench.laid(), laid_before + 1, "one step in the history");
    assert!(
        matches!(bench.last_laid(), Some(Operation::Mirror { .. })),
        "laid by the same operation as before the panel: {:?}",
        bench.last_laid()
    );
    assert_eq!(traits(&bench), traits_before + 1);
    assert!(bench.document.undo());
    assert_eq!(traits(&bench), traits_before, "undone in one go");
}

#[test]
fn enter_on_validate_with_a_slot_empty_activates_and_marks_the_first_empty_one() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click_the_slot(CopySlot::Validate);
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(bench.laid(), laid_before, "nothing is laid with no axis");
    assert_eq!(bench.active(), CopySlot::Target);
    assert_eq!(bench.panel().marked(), Some(CopySlot::Target));
    bench.click(ON_THE_ORDINARY_TRAIT);
    assert_eq!(
        bench.panel().marked(),
        None,
        "the mark goes once the slot is filled"
    );

    let mut bench = holding_the_trait_to_copy(Command::ToolCircularPattern);
    bench.click(THE_ORDINARY_TRAIT_S_FOOT);
    bench.type_value(1, "");
    bench.click_the_slot(CopySlot::Validate);
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(bench.laid(), laid_before, "nothing is laid with no count");
    assert_eq!(bench.active(), CopySlot::Value(1));
    assert_eq!(bench.panel().marked(), Some(CopySlot::Value(1)));
}

#[test]
fn a_copy_laid_leaves_the_tool_in_hand_and_the_panel_empty_on_its_elements() {
    let mut bench = holding_the_trait_to_copy(Command::ToolCircularPattern);
    bench.click(THE_ORDINARY_TRAIT_S_FOOT);
    bench.click_the_slot(CopySlot::Validate);
    let laid_before = bench.laid();
    bench.enter();
    let laid = bench.laid();

    assert_eq!(laid, laid_before + 1, "Validate laid the copy");
    assert_eq!(bench.editor.tool, Tool::CircularPattern);
    assert_eq!(bench.active(), CopySlot::Elements);
    assert_eq!(bench.elements(), Vec::new());
    assert_eq!(
        (bench.value(0), bench.value(1)),
        (None, None),
        "the values go with the rest"
    );

    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click_the_slot(CopySlot::Validate);
    bench.enter();
    assert_eq!(
        bench.laid(),
        laid,
        "the centre went too: nothing is copied twice by mistake"
    );
    assert_eq!(bench.active(), CopySlot::Target);
}

#[test]
fn another_tool_taken_closes_the_panel_and_lays_nothing() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_ORDINARY_TRAIT);
    let laid_before = bench.laid();

    bench.take(Command::ToolLine);

    assert!(bench.editor.copy_panel().is_none(), "the panel is closed");
    assert_eq!(bench.laid(), laid_before, "nothing is laid");
}

#[test]
fn the_selection_tool_taken_from_the_panel_keeps_its_elements_selected() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click(ON_THE_CONSTRUCTION_TRAIT);

    bench.take(Command::ToolSelect);

    let held = [
        bench.a_trait_and_its_ends(TRAIT_TO_COPY),
        bench.a_trait_and_its_ends(CONSTRUCTION_TRAIT),
    ]
    .concat();
    assert_eq!(
        bench.editor.selection(),
        held.into_iter().map(Selection::Element).collect::<Vec<_>>()
    );
}

#[test]
fn a_drawing_tool_taken_from_the_panel_drops_its_elements() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);

    bench.take(Command::ToolLine);
    bench.take(Command::ToolSelect);

    assert!(bench.editor.selection().is_empty());
}

#[test]
fn the_preview_shows_across_the_axis_held_not_under_the_cursor() {
    let mut bench = holding_the_trait_to_copy(Command::ToolMirror);
    bench.click(ON_THE_ORDINARY_TRAIT);

    assert_eq!(
        bench.previewed_traits(1.0),
        Some(1),
        "the one trait held, mirrored across the axis the panel holds"
    );
}

#[test]
fn the_preview_waits_for_the_panel_to_hold_enough() {
    let mut bench = three_traits();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();

    assert_eq!(
        bench.previewed_traits(1.0),
        None,
        "with no axis in the panel there is nothing to lay, whatever the cursor \
         is over"
    );
}

#[test]
fn a_rectangular_pattern_opens_its_values_on_a_step_its_elements_fit_in() {
    let bench = holding_the_trait_to_copy(Command::ToolRectangularPattern);

    assert_eq!(
        [0, 1, 2, 3].map(|rank| bench.value(rank)),
        [Some(10.0), Some(2.0), Some(10.0), Some(2.0)],
        "the trait held stands ten tall, and one copy each way"
    );
}

#[test]
fn a_circular_pattern_laid_from_the_panel_is_one_step() {
    let mut bench = holding_the_trait_to_copy(Command::ToolCircularPattern);
    bench.click(THE_ORDINARY_TRAIT_S_FOOT);
    bench.click_the_slot(CopySlot::Validate);
    let laid_before = bench.laid();
    let traits_before = traits(&bench);

    bench.enter();

    assert_eq!(bench.laid(), laid_before + 1, "one step in the history");
    assert!(
        matches!(bench.last_laid(), Some(Operation::CircularPattern { .. })),
        "{:?}",
        bench.last_laid()
    );
    assert_eq!(
        traits(&bench),
        traits_before + 3,
        "four standing, three laid"
    );
    assert!(bench.document.undo());
    assert_eq!(traits(&bench), traits_before, "undone in one go");
}

#[test]
fn a_rectangular_pattern_laid_from_the_panel_is_one_step() {
    let mut bench = holding_the_trait_to_copy(Command::ToolRectangularPattern);
    bench.click(ON_THE_ORDINARY_TRAIT);
    bench.click_the_slot(CopySlot::Validate);
    let laid_before = bench.laid();
    let traits_before = traits(&bench);

    bench.enter();

    assert_eq!(bench.laid(), laid_before + 1, "one step in the history");
    assert!(
        matches!(
            bench.last_laid(),
            Some(Operation::RectangularPattern { .. })
        ),
        "{:?}",
        bench.last_laid()
    );
    assert_eq!(traits(&bench), traits_before + 3, "two by two, three laid");
    assert!(bench.document.undo());
    assert_eq!(traits(&bench), traits_before, "undone in one go");
}

#[test]
fn an_arc_mirrored_from_the_panel_comes_back_an_arc() {
    let mut bench = Bench::drawn(vec![
        Operation::AddArc {
            sketch: 0,
            center: PointRef::New(DVec2::new(-20.0, 0.0)),
            start: PointRef::New(DVec2::new(-18.0, 0.0)),
            end: PointRef::New(DVec2::new(-20.0, 2.0)),
            construction: false,
        },
        Bench::a_trait(THE_ORDINARY_TRAIT_S_FOOT, THE_ORDINARY_TRAIT_S_HEAD, false),
    ]);
    bench.take(Command::ToolMirror);
    let on_the_arc = DVec2::new(-20.0, 0.0) + DVec2::splat(2.0_f64.sqrt());

    bench.click(on_the_arc);
    assert_eq!(bench.elements(), vec![Element::Arc(ArcId(0))]);
    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_ORDINARY_TRAIT);
    bench.enter();

    let sketch = &bench.document.sketches()[0];
    assert_eq!(
        sketch.live_arcs().count(),
        2,
        "the original and its copy, both arcs"
    );
    let (copy, arc) = sketch
        .live_arcs()
        .find(|(id, _)| *id != ArcId(0))
        .expect("the copy is an arc");
    let near = |place: DVec2, expected: DVec2| (place - expected).length() <= TOLERANCE;
    assert!(
        near(sketch.point(arc.center), DVec2::new(24.0, 0.0)),
        "the centre at x = -20 mirrored across x = 2 lands at x = 24, got {}",
        sketch.point(arc.center)
    );
    assert!((sketch.arc_radius(copy) - 2.0).abs() <= TOLERANCE);
    let ends = [sketch.point(arc.start), sketch.point(arc.end)];
    let mirrored = [DVec2::new(22.0, 0.0), DVec2::new(24.0, 2.0)];
    assert!(
        mirrored
            .iter()
            .all(|end| ends.iter().any(|laid| near(*laid, *end))),
        "the copy runs between the mirror images of the original's ends: {ends:?}"
    );
}

/// A direct geometric comparison after one reflection.
const TOLERANCE: f64 = 1e-9;

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

fn on_the_plane(state: &ViewportState, at: egui::Pos2) -> DVec2 {
    let (origin, direction) = state
        .camera
        .ray(to_ndc(at, SCREEN), SCREEN.width() / SCREEN.height());
    WorkPlane::XY
        .ray_intersection(origin.as_dvec3(), direction.as_dvec3())
        .expect("the place is on the drawing's plane")
}

/// One frame of the canvas, the pointer having done `events` since the last
/// and these keys held, the camera's navigation read first as the viewport
/// reads it.
fn frame_holding(
    egui: &egui::Context,
    state: &mut ViewportState,
    context: &mut SketchContext<'_>,
    events: Vec<egui::Event>,
    modifiers: egui::Modifiers,
) {
    let mut held_then_done = vec![egui::Event::ModifiersChanged(modifiers)];
    held_then_done.extend(events);
    egui.begin_pass(egui::RawInput {
        screen_rect: Some(SCREEN),
        events: held_then_done,
        ..Default::default()
    });
    egui::Area::new(egui::Id::new("the canvas"))
        .fixed_pos(SCREEN.min)
        .show(egui, |ui| {
            ui.set_min_size(SCREEN.size());
            let response = ui.interact(
                SCREEN,
                egui::Id::new("the canvas's surface"),
                egui::Sense::click_and_drag(),
            );
            handle_navigation(ui, state, &response, egui::Rect::NOTHING);
            let scale = ViewScale::of(&state.camera, SCREEN, 1.0, &state.config, 1.0);
            handle_sketch_input(ui, state, &response, SCREEN, scale, context);
        });
    egui.end_pass().textures_delta.clear();
}

fn moved(to: egui::Pos2) -> Vec<egui::Event> {
    vec![egui::Event::PointerMoved(to)]
}

fn pressed_with(
    button: egui::PointerButton,
    modifiers: egui::Modifiers,
    at: egui::Pos2,
    pressed: bool,
) -> Vec<egui::Event> {
    vec![egui::Event::PointerButton {
        pos: at,
        button,
        pressed,
        modifiers,
    }]
}

/// Face on to the drawing, a trait inside the box pulled from `FROM` to `TO`
/// and another below it, the mirror in hand on its elements.
pub(super) fn a_trait_under_a_box() -> (ViewportState, Bench) {
    let mut state = ViewportState::default();
    state
        .camera
        .set_view_angles(0.0, std::f32::consts::FRAC_PI_2);
    let inside = [egui::pos2(820.0, 300.0), egui::pos2(900.0, 300.0)];
    let outside = [egui::pos2(820.0, 600.0), egui::pos2(900.0, 600.0)];
    let [inside, outside] = [inside, outside].map(|ends| ends.map(|at| on_the_plane(&state, at)));
    let mut bench = Bench::drawn(vec![
        Bench::a_trait(inside[0], inside[1], false),
        Bench::a_trait(outside[0], outside[1], false),
    ]);
    bench.take(Command::ToolMirror);
    (state, bench)
}

const FROM: egui::Pos2 = egui::pos2(760.0, 240.0);
const TO: egui::Pos2 = egui::pos2(960.0, 360.0);

/// The box's gesture made with this button and these keys held, up to the
/// moment the button would be let go.
fn pulling_with(
    egui: &egui::Context,
    state: &mut ViewportState,
    bench: &mut Bench,
    pressing: egui::PointerButton,
    modifiers: egui::Modifiers,
) {
    for events in [
        vec![],
        moved(FROM),
        pressed_with(pressing, modifiers, FROM, true),
        moved(FROM + egui::vec2(12.0, 12.0)),
        moved(FROM + (TO - FROM) * 0.5),
        moved(TO),
        vec![],
    ] {
        frame_holding(egui, state, &mut bench.context(), events, modifiers);
    }
}

fn let_go(
    egui: &egui::Context,
    state: &mut ViewportState,
    bench: &mut Bench,
    pressing: egui::PointerButton,
    modifiers: egui::Modifiers,
) {
    for events in [pressed_with(pressing, modifiers, TO, false), vec![]] {
        frame_holding(egui, state, &mut bench.context(), events, modifiers);
    }
}

pub(super) fn pulled_with(
    state: &mut ViewportState,
    bench: &mut Bench,
    pressing: egui::PointerButton,
    modifiers: egui::Modifiers,
) {
    let egui = egui::Context::default();
    pulling_with(&egui, state, bench, pressing, modifiers);
    let_go(&egui, state, bench, pressing, modifiers);
}

/// A gesture that moves the view, held then let go: no box is pulled along
/// the way, and nothing is added at the end.
fn moves_the_view_and_takes_nothing(pressing: egui::PointerButton, modifiers: egui::Modifiers) {
    let (mut state, mut bench) = a_trait_under_a_box();
    let egui = egui::Context::default();

    pulling_with(&egui, &mut state, &mut bench, pressing, modifiers);
    assert_eq!(
        bench.panel().band,
        None,
        "no box follows a hand that is moving the view"
    );
    let_go(&egui, &mut state, &mut bench, pressing, modifiers);

    assert_eq!(bench.elements(), Vec::new());
}

#[test]
fn a_box_pulled_with_the_button_that_pans_takes_nothing() {
    moves_the_view_and_takes_nothing(egui::PointerButton::Middle, egui::Modifiers::NONE);
}

#[test]
fn a_box_pulled_while_orbiting_takes_nothing() {
    moves_the_view_and_takes_nothing(egui::PointerButton::Primary, egui::Modifiers::ALT);
}

#[test]
fn a_box_drawn_in_the_drawing_adds_what_it_takes() {
    let (mut state, mut bench) = a_trait_under_a_box();

    pulled_with(
        &mut state,
        &mut bench,
        egui::PointerButton::Primary,
        egui::Modifiers::NONE,
    );

    let below = &bench.document.sketches()[0].segments()[1];
    let (below_start, below_end) = (below.start, below.end);
    let taken = bench.elements();
    assert!(
        taken.contains(&Element::Segment(SegmentId(0))),
        "the trait inside the box is added: {taken:?}"
    );
    for outside in [
        Element::Segment(SegmentId(1)),
        Element::Point(below_start),
        Element::Point(below_end),
    ] {
        assert!(
            !taken.contains(&outside),
            "nothing of the trait below the box is added: {taken:?}"
        );
    }
    assert_eq!(bench.active(), CopySlot::Elements);
}
