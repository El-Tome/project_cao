//! What the panel of the mirror and the two patterns is held to: which slot a
//! click fills, what `Entrée` does, what is laid and what is left behind.
//!
//! Closes #321.
//! - the panel stands over the view, always in front, and is dragged anywhere
//!   in the view by its title — no test: how it is drawn and dragged is checked
//!   on rendered frames, which no assertion here reaches
//! - it comes back where it was last left, one position shared by every tool
//!   that uses it, kept across restarts —
//!   `the_panel_opens_where_it_was_last_left_whichever_copy_tool_is_taken`, in
//!   the driver, and `a_settings_file_written_before_the_panel_still_loads`, in
//!   `cao_prefs`
//! - a position that no longer fits the view is brought back inside it —
//!   `a_panel_left_outside_a_smaller_view_is_brought_back_inside_it`, beside
//!   the `ui/` primitive
//! - the keyboard writes into the panel; the mouse stays entirely free for the
//!   drawing — `a_click_on_the_drawing_with_the_panel_open_reaches_the_drawing`,
//!   in the driver
//! - Elements: a click in the drawing adds an element —
//!   `a_click_in_the_drawing_adds_the_element_under_it`
//! - Elements: a click on one already held removes it —
//!   `a_click_on_an_element_already_held_lets_it_go`
//! - Elements: a box drawn in the drawing adds what it takes —
//!   `a_box_drawn_in_the_drawing_adds_what_it_takes`
//! - Axis / centre / direction: filled by the next click on a trait, an axis or
//!   a point, depending on the tool —
//!   `an_ordinary_trait_clicked_in_the_target_slot_is_the_mirror_s_axis`,
//!   `a_point_clicked_in_the_target_slot_is_the_circular_pattern_s_centre`,
//!   `a_trait_clicked_in_the_target_slot_is_the_rectangular_pattern_s_direction`
//! - Values: step and count for the circular pattern, two steps and two counts
//!   for the rectangular one, none for the mirror —
//!   `each_copy_tool_has_the_values_it_needs_and_no_more`
//! - Validate / cancel — `the_panel_s_cancel_closes_it_and_lays_nothing`; Validate
//!   is held under Leaving
//! - one slot is active and shown as such — no test: how it is shown is
//!   checked on rendered frames; which one it is, by every test below
//! - a click in the drawing fills the active slot and only that one, so an
//!   ordinary trait can be an axis and a construction trait can be copied —
//!   `an_ordinary_trait_clicked_in_the_target_slot_is_the_mirror_s_axis`,
//!   `a_construction_trait_clicked_while_elements_is_active_is_copied`
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
//!   fires — `a_letter_typed_in_a_field_of_the_panel_takes_no_tool`, in the
//!   driver
//! - `Entrée` and `Échap` still do what is said here —
//!   `enter_with_the_panel_open_does_not_close_the_sketch` and
//!   `escape_closes_the_panel_and_lays_nothing`, in the driver
//! - Validate: the copy is laid, the tool stays in hand, and the panel is
//!   emptied entirely — elements, target and values — back on Elements —
//!   `a_copy_laid_leaves_the_tool_in_hand_and_the_panel_empty_on_its_elements`
//! - `Échap` or another tool taken from the ribbon: nothing is laid, the panel
//!   closes — `escape_closes_the_panel_and_lays_nothing`, in the driver, and
//!   `another_tool_taken_closes_the_panel_and_lays_nothing`
//! - the elements held stay selected if the tool reached for works on a
//!   selection; otherwise the selection is dropped —
//!   `the_selection_tool_taken_from_the_panel_keeps_its_elements_selected`,
//!   `a_drawing_tool_taken_from_the_panel_drops_its_elements`
//! - the preview (#311) updates as soon as the panel holds enough to lay
//!   something — `the_preview_shows_across_the_axis_held_not_under_the_cursor`
//! - must not break: the preview of the copy before it is laid (#311) —
//!   `the_preview_shows_across_the_axis_held_not_under_the_cursor`
//! - must not break: a pattern opens on a step its selection already fits in
//!   (#322) — `a_rectangular_pattern_opens_its_values_on_a_step_its_elements_fit_in`
//! - must not break: a selection made with the Selection tool is what the tool
//!   starts with — `a_copy_tool_taken_with_a_selection_in_hand_opens_on_its_target`
//! - must not break: a mirror, a circular pattern or a rectangular pattern is
//!   still one step in the history, undone in one go, and laid by the same
//!   operations as today — `enter_on_validate_lays_the_mirror_as_one_step`,
//!   `a_circular_pattern_laid_from_the_panel_is_one_step`,
//!   `a_rectangular_pattern_laid_from_the_panel_is_one_step`
//! - must not break: a mirrored arc comes back as an arc (#317) —
//!   `an_arc_mirrored_from_the_panel_comes_back_an_arc`

use cao_part::{Operation, PartDocument, PointRef};
use cao_prefs::Command;
use cao_sketch::{ChosenAxis, Element, SegmentId, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::super::{copy, enter_in_the_panel};
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::{CopySlot, SketchEditor};
use crate::screens::viewport::ViewportState;

const SNAP: f64 = 0.5;
const PIXEL: f64 = 0.05;

const ON_THE_TRAIT_TO_COPY: DVec2 = DVec2::new(-10.0, 0.0);
const ON_THE_ORDINARY_TRAIT: DVec2 = DVec2::new(2.0, 0.0);
const ON_THE_CONSTRUCTION_TRAIT: DVec2 = DVec2::new(0.0, 20.0);
const FAR_FROM_EVERYTHING: DVec2 = DVec2::new(40.0, 40.0);

const TRAIT_TO_COPY: SegmentId = SegmentId(0);
const ORDINARY_TRAIT: SegmentId = SegmentId(1);
const CONSTRUCTION_TRAIT: SegmentId = SegmentId(2);

/// A sketch open on three traits clear of each other and of both sketch axes:
/// one to copy, an ordinary one upright beside it, and a construction one lying
/// above them both.
struct Bench {
    document: PartDocument,
    editor: SketchEditor,
    extrusion: ExtrusionState,
    lang: Catalogue,
}

impl Bench {
    fn new() -> Self {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        for (start, end, construction) in [
            (DVec2::new(-10.0, -5.0), DVec2::new(-10.0, 5.0), false),
            (DVec2::new(2.0, -5.0), DVec2::new(2.0, 5.0), false),
            (DVec2::new(-5.0, 20.0), DVec2::new(5.0, 20.0), true),
        ] {
            document.apply(Operation::AddSegment {
                sketch: 0,
                start: PointRef::New(start),
                end: PointRef::New(end),
                construction,
            });
        }
        let mut editor = SketchEditor::default();
        editor.begin_editing(0, WorkPlane::XY);
        Self {
            document,
            editor,
            extrusion: ExtrusionState::default(),
            lang: Catalogue::french(),
        }
    }

    fn context(&mut self) -> SketchContext<'_> {
        SketchContext {
            document: &mut self.document,
            editor: &mut self.editor,
            extrusion: &mut self.extrusion,
            lang: &self.lang,
        }
    }

    fn take(&mut self, command: Command) {
        crate::commands::run(
            command,
            &mut self.context(),
            &mut Ribbon::default(),
            &mut ViewportState::default(),
        );
    }

    fn click(&mut self, at: DVec2) {
        self.editor.cursor = Some(at);
        copy(&mut self.context(), 0, at, SNAP, PIXEL);
    }

    fn click_the_slot(&mut self, slot: CopySlot) {
        self.editor
            .copy_panel_mut()
            .expect("the panel is open while a copy tool is in hand")
            .activate(slot);
    }

    fn enter(&mut self) {
        enter_in_the_panel(&mut self.context());
    }

    fn active(&self) -> CopySlot {
        self.editor
            .copy_panel()
            .expect("the panel is open while a copy tool is in hand")
            .active()
    }

    fn elements(&self) -> Vec<Element> {
        self.editor
            .copy_panel()
            .expect("the panel is open while a copy tool is in hand")
            .elements()
            .to_vec()
    }

    fn laid(&self) -> usize {
        self.document.history.applied_operations().len()
    }

    fn last_laid(&self) -> Option<&Operation> {
        self.document.history.applied_operations().last()
    }

    fn traits(&self) -> usize {
        self.document.sketches()[0].segments().len()
    }
}

#[test]
fn the_mirror_opens_on_its_elements_when_nothing_was_selected() {
    let mut bench = Bench::new();

    bench.take(Command::ToolMirror);

    assert_eq!(bench.active(), CopySlot::Elements);
    assert_eq!(bench.elements(), Vec::new());
}

#[test]
fn a_click_in_the_drawing_adds_the_element_under_it() {
    let mut bench = Bench::new();
    bench.take(Command::ToolMirror);

    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click(ON_THE_ORDINARY_TRAIT);

    assert_eq!(
        bench.elements(),
        vec![
            Element::Segment(TRAIT_TO_COPY),
            Element::Segment(ORDINARY_TRAIT)
        ]
    );
    assert_eq!(
        bench.active(),
        CopySlot::Elements,
        "Elements is a list: a click adds to it and leaves it active for the next"
    );
}

#[test]
fn a_construction_trait_clicked_while_elements_is_active_is_copied() {
    let mut bench = Bench::new();
    bench.take(Command::ToolMirror);

    bench.click(ON_THE_CONSTRUCTION_TRAIT);

    assert_eq!(bench.elements(), vec![Element::Segment(CONSTRUCTION_TRAIT)]);
    assert_eq!(
        bench.active(),
        CopySlot::Elements,
        "a construction trait is not guessed to be the axis"
    );
}

#[test]
fn an_ordinary_trait_clicked_in_the_target_slot_is_the_mirror_s_axis() {
    let mut bench = Bench::new();
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
            elements: vec![Element::Segment(TRAIT_TO_COPY)],
            axis: ChosenAxis::Trait(ORDINARY_TRAIT),
        }),
        "the axis is the trait clicked while its slot was active, picked before \
         the elements and kept, whatever the cursor is over when the copy is laid"
    );
}

#[test]
fn picking_the_mirror_s_axis_lands_on_validate() {
    let mut bench = Bench::new();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);

    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_ORDINARY_TRAIT);

    assert_eq!(bench.active(), CopySlot::Validate);
    assert_eq!(
        bench.elements(),
        vec![Element::Segment(TRAIT_TO_COPY)],
        "the trait clicked for the axis is not added to what is copied"
    );
}

#[test]
fn enter_on_validate_lays_the_mirror_as_one_step() {
    let mut bench = Bench::new();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.click_the_slot(CopySlot::Target);
    bench.click(ON_THE_ORDINARY_TRAIT);
    let laid_before = bench.laid();
    let traits_before = bench.traits();

    bench.editor.cursor = Some(FAR_FROM_EVERYTHING);
    bench.enter();

    assert_eq!(bench.laid(), laid_before + 1, "one step in the history");
    assert!(
        matches!(bench.last_laid(), Some(Operation::Mirror { .. })),
        "laid by the same operation as before the panel: {:?}",
        bench.last_laid()
    );
    assert_eq!(bench.traits(), traits_before + 1);
    assert!(bench.document.undo());
    assert_eq!(bench.traits(), traits_before, "undone in one go");
}
