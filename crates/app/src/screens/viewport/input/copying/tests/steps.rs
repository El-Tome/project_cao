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
//!   under its own `Closes #738`. Decided on the way: ↵ moves on from a step
//!   still empty, as it did from a slot (`enter_moves_down_one_slot` in
//!   `tests/panel.rs`), and on the last step with one left empty it lays
//!   nothing and brings the first empty one back, marked, as Validate did —
//!   `enter_on_the_last_step_with_a_step_left_empty_brings_that_step_back_marked`
//! - The message pill follows the step
//!   (« Réseau circulaire — étape 3 : nombre de copies »);
//!   a test on the wording checks it —
//!   `the_message_pill_follows_the_step_of_the_circular_pattern`, as ↵ and the
//!   clicks in the drawing move it, and as a step clicked in the panel does —
//!   `a_step_clicked_in_the_panel_takes_the_message_pill_with_it`. Decided on
//!   the way: the number and the step are one step, and say one thing on both
//!   their fields, the issue's « nombre de copies »; the other three are
//!   « éléments à copier », « centre de rotation » and « sens de rotation ».
//!   The mirror and the rectangular pattern, which the next issue moves, still
//!   ask what they asked —
//!   `the_mirror_and_the_rectangular_pattern_still_ask_what_they_asked`, and a
//!   row clicked in their panel still leaves the message as it was —
//!   `a_row_clicked_in_the_mirror_s_or_the_rectangular_pattern_s_panel_leaves_the_message_as_it_was`
//! - « Horaire » applies the same pattern with the step of opposite sign;
//!   « Anti-horaire », the default, gives exactly today's result. A test
//!   compares the two —
//!   `clockwise_lays_the_same_pattern_with_its_step_of_opposite_sign`; a step
//!   typed as a formula stays one, turned the other way —
//!   `clockwise_lays_a_step_typed_as_a_formula_as_that_formula_negated`; and
//!   the preview turns the way the pattern will be laid —
//!   `the_preview_turns_clockwise_once_clockwise_is_chosen`
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

use cao_part::{Formula, Operation, Variables};
use cao_prefs::Command;
use cao_sketch::{Element, SegmentId};
use glam::DVec2;

use super::super::{activate_in_the_panel, previewed};
use super::bench::Bench;
use crate::screens::sketch::{CopySlot, CopyStep, Rotation, StepState};

const ON_THE_TRAIT_TO_COPY: DVec2 = DVec2::new(-10.0, 0.0);
const THE_CENTRE: DVec2 = DVec2::new(2.0, -5.0);
const TRAIT_TO_COPY: SegmentId = SegmentId(0);

/// A copy is the original turned about a point: no solver, a few products.
const TOLERANCE: f64 = 1e-9;

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

/// What « Horaire » or « Anti-horaire » does in the panel.
fn choose(bench: &mut Bench, rotation: Rotation) {
    bench
        .editor
        .copy_panel_mut()
        .expect("the panel is open while a copy tool is in hand")
        .choose_rotation(rotation);
}

/// The circular pattern « Valider » lays: the trait held, the centre clicked,
/// the number and the step it opens on, then whatever `set` does.
fn laid_by_the_panel(set: impl FnOnce(&mut Bench)) -> Operation {
    let mut bench = a_trait_and_a_centre();
    bench.take(Command::ToolCircularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click(THE_CENTRE);
    set(&mut bench);
    bench.click_the_slot(CopySlot::Validate);
    bench.enter();
    bench
        .last_laid()
        .cloned()
        .expect("« Valider » with every step filled lays the pattern")
}

#[test]
fn clockwise_lays_the_same_pattern_with_its_step_of_opposite_sign() {
    let bench = a_trait_and_a_centre();
    let elements = bench.a_trait_and_its_ends(TRAIT_TO_COPY);
    let centre = bench.point_at(THE_CENTRE);

    let by_default = laid_by_the_panel(|_| {});
    let clockwise = laid_by_the_panel(|bench| choose(bench, Rotation::Clockwise));
    let counter_clockwise = laid_by_the_panel(|bench| choose(bench, Rotation::CounterClockwise));

    assert_eq!(
        by_default,
        Operation::CircularPattern {
            sketch: 0,
            elements: elements.clone(),
            centre,
            degrees: Formula::Number(90.0),
            count: Formula::Number(4.0),
        },
        "with no direction chosen, the pattern laid is exactly the one laid before \
         the panel had a direction"
    );
    assert_eq!(
        counter_clockwise, by_default,
        "counter-clockwise is the default"
    );
    assert_eq!(
        clockwise,
        Operation::CircularPattern {
            sketch: 0,
            elements,
            centre,
            degrees: Formula::Number(-90.0),
            count: Formula::Number(4.0),
        },
        "clockwise lays the same copies of the same elements about the same \
         centre, its step turned the other way"
    );
}

#[test]
fn clockwise_lays_a_step_typed_as_a_formula_as_that_formula_negated() {
    let typed = |bench: &mut Bench| {
        bench.type_value(0, "360 / 6");
        bench.type_value(1, "6");
    };
    let written = |laid: &Operation| match laid {
        Operation::CircularPattern { degrees, .. } => Variables::default().written(degrees),
        other => panic!("a circular pattern is laid, not {other:?}"),
    };

    let counter_clockwise = laid_by_the_panel(typed);
    let clockwise = laid_by_the_panel(|bench| {
        typed(bench);
        choose(bench, Rotation::Clockwise);
    });

    assert_eq!(written(&counter_clockwise), "360 / 6");
    assert_eq!(
        written(&clockwise),
        "-(360 / 6)",
        "the formula typed, turned the other way, rather than the number it \
         comes to"
    );
}

/// Where the one copy the preview shows stands: the middle of its trait.
fn the_copy_previewed(bench: &Bench) -> DVec2 {
    let shown = previewed(&bench.document.sketches()[0], &bench.editor, 1.0)
        .expect("the panel holds what to copy, where, and its values");
    let middles: Vec<DVec2> = shown
        .laid
        .iter()
        .filter_map(|laid| match laid {
            Element::Segment(side) => {
                let side = shown.sketch.segments()[side.0];
                Some((shown.sketch.point(side.start) + shown.sketch.point(side.end)) / 2.0)
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        middles.len(),
        1,
        "a count of two is the original and one copy"
    );
    middles[0]
}

#[test]
fn the_preview_turns_clockwise_once_clockwise_is_chosen() {
    let mut bench = a_trait_and_a_centre();
    bench.take(Command::ToolCircularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click(THE_CENTRE);
    bench.type_value(1, "2");
    let counter_clockwise = the_copy_previewed(&bench);

    choose(&mut bench, Rotation::Clockwise);
    let clockwise = the_copy_previewed(&bench);

    assert!(
        (counter_clockwise - DVec2::new(-3.0, -17.0)).length() < TOLERANCE,
        "a quarter turn counter-clockwise about (2, -5) takes the trait's middle, \
         at (-10, 0), to (-3, -17); the copy shown stands at {counter_clockwise}"
    );
    assert!(
        (clockwise - DVec2::new(7.0, 7.0)).length() < TOLERANCE,
        "a quarter turn clockwise takes it to (7, 7); the copy shown stands at \
         {clockwise}"
    );
}

#[test]
fn enter_validates_the_current_step_and_moves_on_to_the_next() {
    let mut bench = a_trait_and_a_centre();
    bench.take(Command::ToolCircularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    let laid_before = bench.laid();

    bench.enter();
    assert_eq!(
        bench.active(),
        CopySlot::Target,
        "the elements held, ↵ moves on to the centre"
    );
    bench.click(THE_CENTRE);
    bench.click_the_slot(CopySlot::Target);
    bench.enter();
    assert_eq!(
        bench.active(),
        CopySlot::Value(0),
        "the centre held, ↵ moves on to the number and the step"
    );
    bench.enter();
    assert_eq!(
        bench.active(),
        CopySlot::Value(1),
        "the number and the step are two fields of one step: ↵ on the first \
         moves on to the second"
    );
    bench.enter();
    assert_eq!(
        bench.active(),
        CopySlot::Rotation,
        "the number and the step held, ↵ moves on to the direction"
    );
    assert_eq!(bench.laid(), laid_before, "moving on lays nothing");
}

#[test]
fn enter_on_the_last_step_applies_the_pattern() {
    let mut bench = a_trait_and_a_centre();
    let elements = bench.a_trait_and_its_ends(TRAIT_TO_COPY);
    let centre = bench.point_at(THE_CENTRE);
    bench.take(Command::ToolCircularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click(THE_CENTRE);
    bench.click_the_slot(CopySlot::Rotation);
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(
        bench.laid(),
        laid_before + 1,
        "one ↵ on the direction lays the pattern, with no Validate to reach first"
    );
    assert_eq!(
        bench.last_laid(),
        Some(&Operation::CircularPattern {
            sketch: 0,
            elements,
            centre,
            degrees: Formula::Number(90.0),
            count: Formula::Number(4.0),
        })
    );
}

#[test]
fn enter_on_the_last_step_with_a_step_left_empty_brings_that_step_back_marked() {
    let mut bench = a_trait_and_a_centre();
    bench.take(Command::ToolCircularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click_the_slot(CopySlot::Rotation);
    let laid_before = bench.laid();

    bench.enter();

    assert_eq!(bench.laid(), laid_before, "nothing is laid with no centre");
    assert_eq!(
        bench.active(),
        CopySlot::Target,
        "the first step still empty is the one asked for again"
    );
    assert_eq!(bench.panel().marked(), Some(CopySlot::Target));
}

/// What the message pill at the top of the screen reads.
fn said(bench: &Bench) -> Option<&str> {
    bench.editor.message.as_deref()
}

const STEP_1: &str = "Réseau circulaire — étape 1 : éléments à copier";
const STEP_2: &str = "Réseau circulaire — étape 2 : centre de rotation";
const STEP_3: &str = "Réseau circulaire — étape 3 : nombre de copies";
const STEP_4: &str = "Réseau circulaire — étape 4 : sens de rotation";

#[test]
fn the_message_pill_follows_the_step_of_the_circular_pattern() {
    let mut bench = a_trait_and_a_centre();

    bench.take(Command::ToolCircularPattern);
    assert_eq!(said(&bench), Some(STEP_1));
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    assert_eq!(said(&bench), Some(STEP_2));
    bench.click(THE_CENTRE);
    assert_eq!(said(&bench), Some(STEP_3));
    bench.enter();
    assert_eq!(
        said(&bench),
        Some(STEP_3),
        "the number and the step are one step: its second field says what its \
         first said"
    );
    bench.enter();
    assert_eq!(said(&bench), Some(STEP_4));
    bench.enter();
    assert_eq!(
        said(&bench),
        Some(STEP_1),
        "the pattern laid, the panel is back on its elements"
    );
}

#[test]
fn a_step_clicked_in_the_panel_takes_the_message_pill_with_it() {
    let mut bench = a_trait_and_a_centre();
    bench.take(Command::ToolCircularPattern);
    bench.click(ON_THE_TRAIT_TO_COPY);
    bench.enter();
    bench.click(THE_CENTRE);

    activate_in_the_panel(&mut bench.context(), CopySlot::Rotation);
    assert_eq!(said(&bench), Some(STEP_4));
    activate_in_the_panel(&mut bench.context(), CopySlot::Target);
    assert_eq!(
        said(&bench),
        Some(STEP_2),
        "the centre clicked again to change it"
    );
    assert_eq!(
        bench.active(),
        CopySlot::Target,
        "the click makes that step current"
    );
}

#[test]
fn the_mirror_and_the_rectangular_pattern_still_ask_what_they_asked() {
    for (command, elements, target) in [
        (
            Command::ToolMirror,
            "Cliquez ou encadrez ce qui doit être miroité, puis Entrée",
            "Cliquez le trait ou l'axe de symétrie",
        ),
        (
            Command::ToolRectangularPattern,
            "Cliquez ou encadrez ce qui doit être répété, puis Entrée",
            "Cliquez le trait ou l'axe le long duquel répéter",
        ),
    ] {
        let mut bench = a_trait_and_a_centre();

        bench.take(command);
        assert_eq!(said(&bench), Some(elements), "{command:?}");
        bench.click(ON_THE_TRAIT_TO_COPY);
        bench.enter();
        assert_eq!(said(&bench), Some(target), "{command:?}");
        bench.click(THE_CENTRE + DVec2::new(0.0, 5.0));
        assert_eq!(
            said(&bench),
            None,
            "{command:?}: past its target, the panel says nothing above it"
        );
    }
}

#[test]
fn a_row_clicked_in_the_mirror_s_or_the_rectangular_pattern_s_panel_leaves_the_message_as_it_was() {
    for (command, rows) in [
        (Command::ToolMirror, vec![CopySlot::Target]),
        (
            Command::ToolRectangularPattern,
            vec![CopySlot::Target, CopySlot::Value(0)],
        ),
    ] {
        let mut bench = a_trait_and_a_centre();
        bench.take(command);
        bench.click(ON_THE_TRAIT_TO_COPY);
        let before = said(&bench).map(str::to_owned);

        for row in rows {
            activate_in_the_panel(&mut bench.context(), row);

            assert_eq!(
                said(&bench),
                before.as_deref(),
                "{command:?}: the row {row:?} clicked"
            );
            assert_eq!(
                bench.active(),
                row,
                "{command:?}: the row clicked is active"
            );
        }
    }
}
