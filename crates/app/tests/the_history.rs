//! The history at the bottom of the Part panel, driven through the whole
//! application: folded, unfolded, clicked.
//!
//! Closes #735 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests/history.rs`; these are the
//! bullets only the panel drawn shows.
//! - At the bottom of the Part panel, « ▸ Historique · N actions » is folded by
//!   default; a click on it unfolds it, and H folds and unfolds it —
//!   `the_history_sits_folded_at_the_bottom_of_the_part_panel`,
//!   `a_click_on_the_history_header_unfolds_it`,
//!   `h_folds_and_unfolds_the_history`
//! - Unfolded, it is a flat list with no groups per step, at most 230 px high
//!   and scrolling. Each line shows the number in monospace and the name, with
//!   the detail on a second line — the detail standing under the name,
//!   `an_unfolded_line_reads_its_name_above_its_detail`; a history longer than
//!   the cap ends no more than 230 points under its header,
//!   `a_long_history_unfolds_no_taller_than_230_points`; the monospace number
//!   is a font, which the accessibility tree does not carry
//! - A click on a line goes back to that action, as today; undone lines stay
//!   dimmed — `a_click_on_a_line_puts_the_part_back_to_that_action`; the
//!   dimming is a colour, held as a flag by the presenter
//! - « Modifier l'esquisse » stays on the right click of a sketch's line —
//!   `a_right_click_on_a_sketch_s_line_offers_to_edit_it_and_reopens_it`
//! - « Compacter l'historique… » is a quiet link at the bottom of the unfolded
//!   list, and its confirmation stays —
//!   `the_compact_link_at_the_bottom_asks_before_compacting`
//! - The separate history panel is gone —
//!   `no_history_panel_of_its_own_opens_beside_the_part`
//! - `rewind_to`, undo, redo and compacting behave as before, reached from the
//!   panel — `a_click_on_a_line_puts_the_part_back_to_that_action`,
//!   `the_compact_link_at_the_bottom_asks_before_compacting`
