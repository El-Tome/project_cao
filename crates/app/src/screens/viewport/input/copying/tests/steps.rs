//! The numbered steps of the circular pattern's panel, driven the way the
//! input layer drives them: which step reads done, current or to come, what
//! ↵ does on each, what the message says, and which way the copies
//! turn.
//!
//! Closes #738.
//! - The four steps of the circular pattern (elements, centre, number and
//!   step, direction) each read done, current or to come from the state of the
//!   `CopyPanel`; a presenter test checks it, with no window —
//!   `each_step_left_filled_reads_done_and_the_step_moved_to_reads_current`,
//!   and a step reads done only once it holds what it asks for —
//!   `a_step_skipped_while_still_empty_reads_to_come_rather_than_done`
//! - ↵ validates the current step and moves on; on the last step it applies
//!   the pattern; Escape cancels. One test per case —
//!   `enter_validates_the_current_step_and_moves_on_to_the_next`,
//!   `enter_on_the_last_step_applies_the_pattern`; Escape, which only the
//!   whole application's keyboard reaches, in
//!   `crates/app/tests/copying_in_a_panel.rs` by
//!   `escape_on_a_step_of_the_circular_pattern_closes_its_panel_and_lays_nothing`,
//!   under its own `Closes #738`
//! - The message pill follows the step
//!   (« Réseau circulaire — étape 3 : nombre de copies »);
//!   a test on the wording checks it —
//!   `the_message_pill_follows_the_step_of_the_circular_pattern`
//! - « Horaire » applies the same pattern with the step of opposite sign;
//!   « Anti-horaire », the default, gives exactly today's result. A test
//!   compares the two —
//!   `clockwise_lays_the_same_pattern_with_its_step_of_opposite_sign`
//! - `ui/` gains the controls it lacks: the step dot (22 px, three states), the
//!   chip and the number field with − / +. The controls already there
//!   (`floating_panel`, `formula_field`, `primary_button`, `secondary_button`)
//!   are reused. None knows a `cao_*` crate; the architecture test keeps it —
//!   no test: here. Each new control is held beside itself, in
//!   `ui/step_dot/tests.rs`, `ui/chip/tests.rs` and `ui/number_field/tests.rs`,
//!   each under its own `Closes #738`; that none of them knows a `cao_*` crate,
//!   by `a_primitive_knows_the_interface_and_nothing_else` in
//!   `crates/app/tests/architecture.rs`, which already reads every file of
//!   `ui/`. That the controls already there are reused rather than written
//!   again is a shape of the code, read in review, not something an assertion
//!   reaches
//! - In the view: the copies dashed in amber, the circle, the radius,
//!   « Centre », the arrowed arc « 60° », the source lit amber — no test: the
//!   tests do not reach the screen; it is tried by hand, see « To try it »
//! - The panel can still be dragged and keeps its place — no test: here. A
//!   drag is egui's pointer on a drawn panel, which only the whole application
//!   reaches: held in `crates/app/tests/copying_in_a_panel.rs` by
//!   `the_circular_pattern_s_panel_follows_its_title_and_reopens_where_it_was_let_go`,
//!   under its own `Closes #738`, beside the tests of #321 that hold the same
//!   for the mirror
//! - Mirror and the rectangular pattern work as before; a pattern's result and
//!   what a capped pattern says are unchanged — no test: of its own. Held by
//!   the tests already there, which stay green on this branch: the whole of
//!   `tests/panel.rs` and `tests/ends.rs` beside this file, among them
//!   `enter_on_validate_lays_the_mirror_as_one_step`,
//!   `a_rectangular_pattern_laid_from_the_panel_is_one_step` and
//!   `a_circular_pattern_laid_from_the_panel_is_one_step`; the cap, by
//!   `a_count_typed_above_a_full_turn_is_brought_down_to_it` in
//!   `copying/tests.rs`, by
//!   `a_count_brought_down_names_the_copy_that_would_land_on_the_original` and
//!   `a_capped_step_says_how_many_it_lays_at_its_angle_and_how_many_were_asked`
//!   in `wording/pattern/tests.rs`, and by
//!   `crates/part/tests/a_circular_pattern_stops_at_a_full_turn.rs`; and the
//!   pattern laid counter-clockwise, by the comparison of the fourth bullet

use cao_prefs::Command;
use glam::DVec2;

use super::bench::Bench;
use crate::screens::sketch::{CopySlot, CopyStep, StepState};

const ON_THE_TRAIT_TO_COPY: DVec2 = DVec2::new(-10.0, 0.0);
const THE_CENTRE: DVec2 = DVec2::new(2.0, -5.0);

/// A trait to copy, ten units tall, and an upright clear of it whose foot is
/// the point to turn about.
fn a_trait_and_a_centre() -> Bench {
    Bench::drawn(vec![
        Bench::a_trait(DVec2::new(-10.0, -5.0), DVec2::new(-10.0, 5.0), false),
        Bench::a_trait(THE_CENTRE, DVec2::new(2.0, 5.0), false),
    ])
}

/// Each numbered step of the panel, top to bottom, as its dot reads.
fn steps(bench: &Bench) -> Vec<(CopyStep, StepState)> {
    bench.panel().steps(&bench.editor.live)
}

#[test]
fn each_step_left_filled_reads_done_and_the_step_moved_to_reads_current() {
    let mut bench = a_trait_and_a_centre();

    bench.take(Command::ToolCircularPattern);
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Current),
            (CopyStep::Centre, StepState::ToCome),
            (CopyStep::CountAndStep, StepState::ToCome),
            (CopyStep::Rotation, StepState::ToCome),
        ],
        "the tool taken with nothing in hand opens on its elements"
    );

    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Done),
            (CopyStep::Centre, StepState::Current),
            (CopyStep::CountAndStep, StepState::ToCome),
            (CopyStep::Rotation, StepState::ToCome),
        ],
        "the trait held and ↵ pressed, the centre is asked for; the number \
         and the step stand below it, to come, though their fields already hold \
         the values they open on"
    );

    bench.click(THE_CENTRE);
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Done),
            (CopyStep::Centre, StepState::Done),
            (CopyStep::CountAndStep, StepState::Current),
            (CopyStep::Rotation, StepState::ToCome),
        ],
        "the centre clicked, the number and the step are asked for; the direction \
         stands below, to come, though it holds its default"
    );

    bench.click_the_slot(CopySlot::Rotation);
    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::Done),
            (CopyStep::Centre, StepState::Done),
            (CopyStep::CountAndStep, StepState::Done),
            (CopyStep::Rotation, StepState::Current),
        ],
        "the number and the step left filled, the direction clicked"
    );
}

#[test]
fn a_step_skipped_while_still_empty_reads_to_come_rather_than_done() {
    let mut bench = a_trait_and_a_centre();
    bench.take(Command::ToolCircularPattern);

    bench.click_the_slot(CopySlot::Target);

    assert_eq!(
        steps(&bench),
        [
            (CopyStep::Elements, StepState::ToCome),
            (CopyStep::Centre, StepState::Current),
            (CopyStep::CountAndStep, StepState::ToCome),
            (CopyStep::Rotation, StepState::ToCome),
        ],
        "nothing is held yet: a check on the elements would tell the user they \
         were picked"
    );
}
