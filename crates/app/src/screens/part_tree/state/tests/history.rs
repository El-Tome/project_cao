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
//!   `every_action_is_a_line_numbered_from_one_and_counted`,
//!   `a_section_takes_no_line_and_no_number_in_the_history`; the names and
//!   details of screen 06, decided by Tom on 2026-11-10 — a name numbered per
//!   kind for a sketch and an extrusion, a short detail —
//!   `the_history_reads_the_first_three_lines_of_screen_06`,
//!   `a_sketch_on_a_face_and_a_circle_read_as_on_screen_06`,
//!   `a_cut_reads_removing_matter_and_its_depth`, a sketch and a volume named
//!   as their rows above name them,
//!   `a_sketch_and_a_volume_read_the_names_their_rows_in_the_part_panel_give_them`;
//!   the kinds the screen does not show keep their name and are given a short
//!   detail in the same spirit, which nothing pins; the two lines, the 230 px
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

/// The part of screen 06: a 120 × 80 rectangle on XY raised 10 mm, then a
/// sketch on its top face holding a circle 24 across, raised 30 mm.
fn the_part_of_screen_06() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(120.0, 80.0)),
        construction: false,
    });
    raise(&mut document, 0, 10.0, ExtrusionMode::Add);
    a_circle_on_the_top_face(&mut document);
    raise(&mut document, 1, 30.0, ExtrusionMode::Add);
    document
}

fn raise(document: &mut PartDocument, sketch: usize, distance: f64, mode: ExtrusionMode) {
    let areas = document.areas_at(sketch, &[DVec2::new(60.0, 40.0)]);
    document.apply(Operation::Extrude {
        sketch,
        areas,
        distance: distance.into(),
        mode,
    });
}

/// A sketch laid on the top face of the 10 mm block, and a circle 24 across
/// in its middle.
fn a_circle_on_the_top_face(document: &mut PartDocument) {
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::new(0.0, 0.0, 10.0),
            u: DVec3::X,
            v: DVec3::Y,
        },
        on: Some(FaceAnchor {
            face: 0,
            up: DVec3::Y,
        }),
    });
    document.apply(Operation::AddCircle {
        sketch: 1,
        center: PointRef::New(DVec2::new(60.0, 40.0)),
        radius: 12.0,
        rim: Vec::new(),
        construction: false,
    });
}

fn read(history: &History) -> Vec<(usize, &str, &str)> {
    history
        .lines
        .iter()
        .map(|line| (line.number, line.name.as_str(), line.detail.as_str()))
        .collect()
}

#[test]
fn the_history_reads_the_first_three_lines_of_screen_06() {
    let history = History::of(&the_part_of_screen_06(), &french());

    assert_eq!(
        read(&history)[..3],
        [
            (1, "Esquisse 1", "Plan XY"),
            (2, "Rectangle", "120 × 80 mm"),
            (3, "Extrusion 1", "ajout de matière · 10 mm"),
        ],
    );
}

#[test]
fn a_sketch_on_a_face_and_a_circle_read_as_on_screen_06() {
    let history = History::of(&the_part_of_screen_06(), &french());

    assert_eq!(
        read(&history)[3..],
        [
            (4, "Esquisse 2", "Face de la pièce"),
            (5, "Cercle", "Ø 24 mm"),
            (6, "Extrusion 2", "ajout de matière · 30 mm"),
        ],
    );
}

#[test]
fn a_sketch_and_a_volume_read_the_names_their_rows_in_the_part_panel_give_them() {
    let document = the_part_of_screen_06();
    let lang = french();
    let history = History::of(&document, &lang);
    let tree = PartTree::of(&document, &lang);

    let named = |number: usize| history.lines[number - 1].name.as_str();
    assert_eq!(
        [named(1), named(3), named(4), named(6)],
        [
            tree.sketches[0].name.as_str(),
            tree.bodies[0].name.as_str(),
            tree.sketches[1].name.as_str(),
            tree.bodies[1].name.as_str(),
        ],
        "the history and the rows above it call one sketch or one volume by \
         two names",
    );
}

#[test]
fn a_cut_reads_removing_matter_and_its_depth() {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(120.0, 80.0)),
        construction: false,
    });
    raise(&mut document, 0, 10.0, ExtrusionMode::Add);
    a_circle_on_the_top_face(&mut document);
    raise(&mut document, 1, 5.0, ExtrusionMode::Cut);

    let history = History::of(&document, &french());

    assert_eq!(
        read(&history)[5],
        (6, "Extrusion 2", "enlèvement de matière · 5 mm"),
    );
}

#[test]
fn a_single_action_is_counted_in_the_singular() {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });

    assert_eq!(History::of(&document, &french()).actions, "1 action");
}
