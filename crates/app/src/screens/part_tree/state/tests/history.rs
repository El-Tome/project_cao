//! What the history at the bottom of the Part panel lists, worked out from the
//! part with no window.
//!
//! Closes #735 — the whole transcription is here; the bullets only the panel
//! drawn shows are transcribed again, checked, in
//! `crates/app/tests/the_history.rs`.
//! - At the bottom of the Part panel, « ▸ Historique · N actions » is folded by
//!   default; a click on it unfolds it, and H folds and unfolds it — the count,
//!   every line listed undone or not, `every_action_is_a_line_numbered_from_one_and_counted`
//!   and `a_single_action_is_counted_in_the_singular`; the folding, held in
//!   `crates/app/tests/the_history.rs` by
//!   `the_history_sits_folded_at_the_bottom_of_the_part_panel`,
//!   `a_click_on_the_history_header_unfolds_it` and
//!   `h_folds_and_unfolds_the_history`
//! - Unfolded, it is a flat list with no groups per step, at most 230 px high
//!   and scrolling. Each line shows the number in monospace and the name, with
//!   the detail on a second line (« 120 × 80 mm »,
//!   « ajout de matière · 10 mm ») — one list, one line per action, a
//!   section's step taking no line and no number,
//!`every_action_is_a_line_numbered_from_one_and_counted`,
//!   `a_section_takes_no_line_and_no_number_in_the_history`; the names and
//!   details of screen 06, decided by Tom on 2026-11-10 — a name numbered per
//!   kind for a sketch and an extrusion, a short detail —
//!   `the_history_reads_the_first_three_lines_of_screen_06`,
//!   `a_sketch_on_a_face_and_a_circle_read_as_on_screen_06`,
//!   `a_cut_reads_removing_matter_and_its_depth`; the two lines, the 230 px
//!   and the scrolling, held in `crates/app/tests/the_history.rs` by
//!   `an_unfolded_line_reads_its_name_above_its_detail` and
//!   `a_long_history_unfolds_no_taller_than_230_points`; the monospace number
//!   is the font it is painted in, which neither the presenter nor the
//!   accessibility tree carries
//! - A click on a line goes back to that action, as today; undone lines stay
//!   dimmed — what an undone line is, `an_undone_action_stays_listed_and_reads_undone`;
//!   the click, held in `crates/app/tests/the_history.rs` by
//!   `a_click_on_a_line_puts_the_part_back_to_that_action`; the dimming is the
//!   colour the view paints from that flag, which no assertion here reaches
//! - « Modifier l'esquisse » stays on the right click of a sketch's line — no
//!   test: here, the menu is the view's; held in
//!   `crates/app/tests/the_history.rs` by
//!   `a_right_click_on_a_sketch_s_line_offers_to_edit_it_and_reopens_it`
//! - « Compacter l'historique… » is a quiet link at the bottom of the unfolded
//!   list, and its confirmation stays — no test: here, the link and its
//!   question are the view's; held in `crates/app/tests/the_history.rs` by
//!   `the_compact_link_at_the_bottom_asks_before_compacting`
//! - The separate history panel is gone — no test: here, the presenter never
//!   drew a panel; held in `crates/app/tests/the_history.rs` by
//!   `no_history_panel_of_its_own_opens_beside_the_part`
//! - `rewind_to`, undo, redo and compacting behave as before — no test: here;
//!   the document's own are unchanged and held by `rewinding_reproduces_the_earlier_state`
//!   in `crates/part/tests/replaying.rs`, `one_undo_takes_back_everything_one_gesture_laid`
//!   and `compacting_a_gesture_keeps_everything_it_laid` in
//!   `crates/part/tests/a_gesture_is_undone_whole.rs`, among others; reached
//!   from the panel, in `crates/app/tests/the_history.rs`

use super::*;
use crate::screens::part_tree::state::History;

fn numbers(history: &History) -> Vec<usize> {
    history.lines.iter().map(|line| line.number).collect()
}

#[test]
fn every_action_is_a_line_numbered_from_one_and_counted() {
    let history = History::of(&a_part(), &french());

    assert_eq!(
        numbers(&history),
        [1, 2, 3, 4, 5],
        "a sketch, a rectangle, a dimension, an extrusion and a second sketch \
         are five actions, one line each, numbered as the list reads",
    );
    assert_eq!(history.actions, "5 actions");
}

#[test]
fn an_undone_action_stays_listed_and_reads_undone() {
    let mut document = a_part();
    document.undo();
    document.undo();

    let history = History::of(&document, &french());

    let undone: Vec<(usize, bool)> = history
        .lines
        .iter()
        .map(|line| (line.number, line.undone))
        .collect();
    assert_eq!(
        undone,
        [(1, false), (2, false), (3, false), (4, true), (5, true)],
        "two undos leave the extrusion and the second sketch listed, undone",
    );
    assert_eq!(history.actions, "5 actions");
}

#[test]
fn a_section_takes_no_line_and_no_number_in_the_history() {
    let mut document = a_part();
    document.apply(Operation::SaveSection(SectionAsked {
        plane: WorkPlane::YZ,
        on: None,
        offset: 5.0,
        behind: SideDrawing::SeeThrough,
        in_front: SideDrawing::Opaque,
        tilt_about_u_degrees: 0.0,
        tilt_about_v_degrees: 0.0,
        pivot: DVec3::ZERO,
    }));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::YZ,
        on: None,
    });

    let history = History::of(&document, &french());

    assert_eq!(
        numbers(&history),
        [1, 2, 3, 4, 5, 6],
        "the section kept between the second and the third sketch is a way of \
         looking, not an action",
    );
    assert_eq!(history.actions, "6 actions");
}
