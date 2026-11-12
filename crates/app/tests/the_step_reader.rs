//! The step reader in the Part panel, driven through the whole application:
//! its boxes clicked, the part put away and opened again, a step added while
//! it is read, and a part of forty steps.
//!
//! Closes #356 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests/reader.rs`; these are the
//! bullets only the whole application holds.
//! - A click sets the reading point — a click on a box, through the panel,
//!   reads the part to it, and ◀ ▶ there move it a box —
//!   `a_click_on_a_box_reads_the_part_to_it`
//! - The reader undoes nothing. Playing to step 3 then to step 6 leaves the
//!   history, `rewind_to`, undo and redo exactly as they were —
//!   `playing_to_step_three_then_six_leaves_the_history_undo_and_redo_as_they_were`
//! - The reading point is not saved. A part saved while read to step 3 opens
//!   read to the end — `a_part_saved_while_read_to_step_three_opens_read_to_the_end`
//! - Adding a step brings the reading point back to the end first (until
//!   #737) — no test: #737 turned it round, the step now going in at the
//!   reading point and the reading point staying on it; its block below holds
//!   that
//! - Forty steps scroll sideways in the row rather than overflowing the panel
//!   — `forty_steps_scroll_sideways_in_the_row_rather_than_overflowing_the_panel`
//! - found in review, and seen in the app on a part of eight steps read to its
//!   end, its last box out of view: the row scrolls to the box read to, unread,
//!   clicked or moved to by ◀ ▶ — `the_row_scrolls_to_the_box_read_to`
//!
//! Closes #737 — the reader's half of each criterion that has one; the part's
//! half, and the criteria that have no other, are in
//! `crates/part/tests/inserting_a_step.rs`, and the names every panel shows in
//! `crates/app/tests/a_sketch_keeps_its_number.rs`.
//! - The reading point is at step 3 of 6; the user creates a sketch, an
//!   extrusion…: the new step is inserted after step 3 — the tools, which put
//!   the step in at the reading point rather than at the end —
//!   `the_reading_point_stays_on_each_step_inserted_and_the_rest_plays_after_them`
//! - The reading point stays on the new step, the rest still dashed. More steps
//!   can be inserted after it; ▶ (or a click on the last box) plays the rest —
//!   `the_reading_point_stays_on_each_step_inserted_and_the_rest_plays_after_them`
//! - Undo takes back the last action in time, even one inserted in the middle;
//!   the reading point does not move unless its box disappears. The history
//!   lists actions in the order they were done; the reader shows steps in the
//!   order of the part — the reading point, and the two lists —
//!   `undoing_an_insertion_moves_the_reading_point_only_when_its_box_goes`,
//!   `the_history_lists_the_order_done_and_the_reader_the_order_of_the_part`
//! - decided with the insertion: while the part is read to a box, the
//!   extrusion offers to raise only the sketches read, the ones the user sees —
//!   `while_read_to_a_box_the_extrusion_offers_only_the_sketches_read`
//! - decided with the insertion: a sketch past the reading point, opened from
//!   the tree and raised, is raised by a step after it, never before it —
//!   `a_sketch_opened_past_the_reading_point_is_raised_after_itself`
//! - decided with the insertion: a sketch laid on a kept section's cut is not
//!   put in at the reading point, which a section always stands after: it goes
//!   at the end, and the reading point with it —
//!   `a_sketch_laid_on_a_kept_section_while_read_to_a_box_goes_at_the_end`
//! - decided with the insertion: « L'extrusion n'a rien ajouté » judges a step
//!   put in on the part read through it, which the canvas shows, not on the
//!   whole part — `an_extrusion_put_in_is_judged_on_the_part_read_through_it`
//!
//! The rest dashed is read off the counter: the boxes played are the first
//! ones, as many as it says, and every box after them is dashed. Which box is
//! read to is told by the line under the row, whose first word is its kind,
//! « Esquisse » or « Extrusion »: by its kind rather than its number, so that
//! the reading point is held apart from the names #737 changes.
//!
//! A box is found by the name it gives whoever cannot see its icon, « Étape
//! n », its place in the row counting from one; ◀ and ▶ by theirs, « Étape
//! précédente » and « Étape suivante », as `docs/redesign/icons.md` names
//! them. Not by the step's own name: the tree's row already answers to it,
//! and a gesture that finds a row by a part of its name would find two.

use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec2;

mod driver;

/// « Bloc », a block raised off a rectangle drawn on YZ, then `sketches`
/// sketches with a trait drawn on each, finished: a box for the block's
/// sketch, one for the block, and one for each sketch after it.
fn a_block_and_sketches(test: &str, sketches: usize) -> driver::App {
    let mut app = driver::a_block(test);
    for _ in 0..sketches {
        driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
        driver::start_a_sketch(&mut app);
        driver::draw_a_segment(&mut app);
        driver::finish_the_sketch(&mut app);
    }
    let last = format!("Esquisse {}", sketches + 1);
    assert!(
        driver::in_the_tree(&app, &last),
        "the fixture wants « {last} » in the tree: {:?}",
        driver::on_screen(&app),
    );
    app
}

/// Four boxes, « Esquisse 1 », « Extrusion 1 », « Esquisse 2 », « Esquisse 3 ».
fn a_part_of_four_key_steps(test: &str) -> driver::App {
    a_block_and_sketches(test, 2)
}

const THE_BOXES: [&str; 4] = ["Étape 1", "Étape 2", "Étape 3", "Étape 4"];

/// What the reader reads: its counter, and whether its line says the rest is
/// hidden, as (the counter it shows, the line read in full when it says so).
fn what_the_reader_reads(app: &driver::App) -> (Option<String>, Option<String>) {
    let counter = (1..=THE_BOXES.len())
        .map(|played| format!("{played} / {}", THE_BOXES.len()))
        .find(|counter| driver::reads(app, counter));
    let hidden = [
        "Esquisse 1 · la suite est masquée",
        "Extrusion 1 · la suite est masquée",
        "Esquisse 2 · la suite est masquée",
    ]
    .into_iter()
    .find(|line| driver::reads(app, line))
    .map(str::to_string);
    (counter, hidden)
}

fn reading(counter: &str, hidden: Option<&str>) -> (Option<String>, Option<String>) {
    (Some(counter.to_string()), hidden.map(str::to_string))
}

#[test]
fn a_click_on_a_box_reads_the_part_to_it() {
    let mut app = a_part_of_four_key_steps("a_click_on_a_box_reads_the_part_to_it");

    let boxes: Vec<Option<egui::Rect>> = THE_BOXES
        .iter()
        .map(|name| driver::the_row(&app, name).map(|node| node.rect()))
        .collect();
    let Some(boxes) = boxes.into_iter().collect::<Option<Vec<egui::Rect>>>() else {
        panic!(
            "the Part panel shows no reader with a box named each of {THE_BOXES:?}: {:?}",
            driver::on_screen(&app),
        );
    };
    let history = driver::the_row(&app, "Historique")
        .expect("the history's header at the bottom of the Part panel")
        .rect();
    assert!(
        boxes
            .windows(2)
            .all(|pair| pair[0].center().x < pair[1].center().x)
            && boxes.iter().all(|one| one.bottom() <= history.top()),
        "the boxes do not stand left to right in the part's order above the history: \
         {boxes:?}, the history's header at {history:?}",
    );

    let mut read = vec![what_the_reader_reads(&app)];
    driver::click_the_button(&mut app, "Étape 3");
    read.push(what_the_reader_reads(&app));
    driver::click_the_button(&mut app, "Étape suivante");
    read.push(what_the_reader_reads(&app));
    for _ in 0..2 {
        driver::click_the_button(&mut app, "Étape précédente");
    }
    read.push(what_the_reader_reads(&app));

    assert_eq!(
        read,
        [
            reading("4 / 4", None),
            reading("3 / 4", Some("Esquisse 2 · la suite est masquée")),
            reading("4 / 4", None),
            reading("2 / 4", Some("Extrusion 1 · la suite est masquée")),
        ],
        "unread, then « Étape 3 » clicked, ▶, and ◀ twice: the reader does not read the part \
         to the box clicked or moved to, as (counter, the line when the rest is hidden)",
    );
    assert!(
        !driver::is_enabled(&app, "Rétablir"),
        "reading the part left something to redo: it was undone rather than read",
    );
}

/// Where the part stands, as the history and the tree say it, the reader left
/// out: each line of the history with whether the part stands at it, whether
/// « Annuler » and « Rétablir » are offered, and which of its steps the tree
/// holds.
#[derive(Debug, PartialEq)]
struct Standing {
    lines: Vec<(String, bool)>,
    undo: bool,
    redo: bool,
    tree: Vec<&'static str>,
}

const THE_SIX_STEPS: [&str; 6] = [
    "Esquisse 1",
    "Extrusion 1",
    "Esquisse 2",
    "Esquisse 3",
    "Esquisse 4",
    "Esquisse 5",
];

fn click_the_history_s_header(app: &mut driver::App) {
    driver::the_row(app, "Historique")
        .expect("the history's header at the bottom of the Part panel")
        .click();
    app.run();
}

/// The history unfolded and read, then folded again as it was.
fn standing(app: &mut driver::App) -> Standing {
    click_the_history_s_header(app);
    let lines = driver::steps(app)
        .into_iter()
        .map(|line| {
            let current = driver::the_row(app, &line).is_some_and(|node| {
                node.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True)
            });
            (line, current)
        })
        .collect();
    assert!(
        !driver::steps(app).is_empty(),
        "a click on « Historique » unfolded no line: {:?}",
        driver::on_screen(app),
    );
    click_the_history_s_header(app);
    Standing {
        lines,
        undo: driver::is_enabled(app, "Annuler"),
        redo: driver::is_enabled(app, "Rétablir"),
        tree: THE_SIX_STEPS
            .into_iter()
            .filter(|name| driver::in_the_tree(app, name))
            .collect(),
    }
}

#[test]
fn playing_to_step_three_then_six_leaves_the_history_undo_and_redo_as_they_were() {
    let mut read = a_block_and_sketches("playing_to_three_then_six_leaves_the_history", 4);
    let mut never_read = a_block_and_sketches("playing_to_three_then_six_never_read", 4);
    let before = standing(&mut read);
    assert_eq!(
        standing(&mut never_read),
        before,
        "the fixture wants two parts built alike",
    );
    assert!(
        before.undo && !before.redo && before.tree == THE_SIX_STEPS,
        "the fixture wants six steps in the tree, an action to undo and none to redo: \
         {before:?}",
    );

    let mut moved = Vec::new();
    for gesture in [
        "Étape 3",
        "Étape 6",
        "Étape précédente",
        "Étape précédente",
        "Étape suivante",
    ] {
        driver::click_the_button(&mut read, gesture);
        let now = standing(&mut read);
        if now != before {
            moved.push(format!("after « {gesture} »: {now:?}"));
        }
    }
    assert!(
        moved.is_empty(),
        "reading the part moved its history, undo or redo, which stood at {before:?}:\n{}",
        moved.join("\n"),
    );

    for asked in ["Annuler", "Rétablir"] {
        driver::click(&mut read, asked);
        driver::click(&mut never_read, asked);
        assert_eq!(
            standing(&mut read),
            standing(&mut never_read),
            "« {asked} » on the part read does not do what it does on the same part never read",
        );
    }

    let line = "4. Esquisse 2";
    go_back_to(&mut read, line);
    go_back_to(&mut never_read, line);
    let gone_back = standing(&mut read);
    assert!(
        gone_back.redo,
        "the fixture wants a click on « {line} » to put the part back, leaving something to \
         redo: {gone_back:?}",
    );
    assert_eq!(
        gone_back,
        standing(&mut never_read),
        "a click on « {line} » in the history of the part read does not put it back where it \
         puts the same part never read",
    );
}

/// A click on a line of the history, which puts the part back to just after
/// that action; the history folded again after it.
fn go_back_to(app: &mut driver::App, line: &str) {
    click_the_history_s_header(app);
    driver::click_the_button(app, line);
    click_the_history_s_header(app);
}

/// What the reader of a part of `boxes` boxes reads: its counter, and whether
/// its line says the rest is hidden.
fn read_of(app: &driver::App, boxes: usize) -> (Option<String>, bool) {
    let counter = (1..=boxes)
        .map(|played| format!("{played} / {boxes}"))
        .find(|counter| driver::reads(app, counter));
    (counter, driver::shows_or_says(app, "la suite est masquée"))
}

fn read_of_six(app: &driver::App) -> (Option<String>, bool) {
    read_of(app, 6)
}

#[test]
fn a_part_saved_while_read_to_step_three_opens_read_to_the_end() {
    let test = "a_part_saved_while_read_to_step_three";
    let mut app = a_block_and_sketches(test, 4);
    driver::click_the_button(&mut app, "Étape 3");
    assert_eq!(
        read_of_six(&app),
        (Some("3 / 6".to_string()), true),
        "the fixture wants the part read to its third box before it is put away",
    );

    let saved = driver::the_part_put_away(&mut app, test, "Bloc");
    assert_eq!(
        saved.history.applied(),
        saved.history.operations().len(),
        "the part was put away with part of its history taken back",
    );
    let recent = app
        .query_all_by_label("Bloc")
        .last()
        .expect("the start menu lists the part");
    recent.click();
    app.run();

    assert_eq!(
        read_of_six(&app),
        (Some("6 / 6".to_string()), false),
        "the part put away while read to its third box does not open read to its end, \
         as (counter, the rest hidden): {:?}",
        driver::on_screen(&app),
    );
}

/// Where the hole of `a_block_with_a_hole` is drawn, on YZ inside the block's
/// face, as two corners and a place inside them.
const THE_HOLE: [DVec2; 3] = [
    DVec2::new(20.0, 20.0),
    DVec2::new(60.0, -20.0),
    DVec2::new(40.0, 0.0),
];

/// Where the sketch inserted draws the rectangle it raises, on YZ beside the
/// block, clear of it, as two corners and a place inside them.
const BESIDE_THE_BLOCK: [DVec2; 3] = [
    DVec2::new(-60.0, 40.0),
    DVec2::new(-30.0, -40.0),
    DVec2::new(-45.0, 0.0),
];

/// Draws a rectangle in the open sketch, then extrudes it 10 mm, as `mode`
/// says, from the sketch the extrusion offers as `picked`: what
/// `driver::a_block` does, up to « Appliquer ».
fn extrude_a_rectangle(
    app: &mut driver::App,
    [corner, opposite, inside]: [DVec2; 3],
    picked: &str,
    mode: &str,
) {
    driver::click(app, "Rectangle (R)");
    driver::click_at_sketch_point(app, corner);
    driver::click_at_sketch_point(app, opposite);
    driver::click_exactly(app, "Extrusion");
    driver::click_the_button(app, picked);
    driver::click(app, mode);
    driver::click_at_sketch_point(app, inside);
    driver::click(app, "Appliquer");
}

/// Leaves the extrusion, finishes the sketch, and brings back the ribbon's
/// sketch tab, as `driver::a_block` does after « Appliquer ».
fn finish_after_extruding(app: &mut driver::App) {
    driver::press(app, egui::Key::Escape);
    driver::finish_the_sketch(app);
    driver::click_the_button(app, "Esquisse");
}

/// « Bloc » with a hole, the part of #737's « To try it »: a second sketch on
/// YZ, a rectangle inside the block's face cut through it. Four boxes,
/// « Esquisse 1 », « Extrusion 1 », « Esquisse 2 », « Extrusion 2 ».
fn a_block_with_a_hole(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::start_a_sketch(&mut app);
    extrude_a_rectangle(&mut app, THE_HOLE, "2", "Enlèvement de matière");
    finish_after_extruding(&mut app);
    assert!(
        driver::in_the_tree(&app, "Extrusion 2")
            && read_of(&app, 4) == (Some("4 / 4".to_string()), false),
        "the fixture wants a part of four boxes read to its end, its hole « Extrusion 2 »: \
         {:?}",
        driver::on_screen(&app),
    );
    app
}

/// Every text on screen, names and values alike.
fn texts(app: &driver::App) -> Vec<String> {
    let mut texts = driver::on_screen(app);
    texts.extend(driver::said(app));
    texts
}

/// Where the reader stands: its counter « n / total », whatever the total, and
/// the kind of the step read to — the first word of the line under the row —
/// while that line says the rest is hidden; no kind while the part is read
/// whole.
fn where_the_reader_stands(app: &driver::App) -> (Option<String>, Option<String>) {
    let texts = texts(app);
    let counter = texts
        .iter()
        .find(|text| {
            text.split_once(" / ").is_some_and(|(played, total)| {
                played.parse::<usize>().is_ok() && total.parse::<usize>().is_ok()
            })
        })
        .cloned();
    let kind = texts
        .iter()
        .find(|text| text.ends_with("· la suite est masquée"))
        .and_then(|line| line.split_whitespace().next())
        .map(str::to_string);
    (counter, kind)
}

fn stands(counter: &str, kind: Option<&str>) -> (Option<String>, Option<String>) {
    (Some(counter.to_string()), kind.map(str::to_string))
}

#[test]
fn the_reading_point_stays_on_each_step_inserted_and_the_rest_plays_after_them() {
    let mut app = a_block_with_a_hole("the_reading_point_stays_on_each_step_inserted");
    driver::click_the_button(&mut app, "Étape 3");
    assert_eq!(
        where_the_reader_stands(&app),
        stands("3 / 4", Some("Esquisse")),
        "the fixture wants the part read to its second sketch, the hole after it hidden",
    );

    let mut read = Vec::new();
    driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::start_a_sketch(&mut app);
    read.push(("a sketch opened", where_the_reader_stands(&app)));
    extrude_a_rectangle(&mut app, BESIDE_THE_BLOCK, "3", "Ajout de matière");
    read.push(("its rectangle raised", where_the_reader_stands(&app)));
    finish_after_extruding(&mut app);
    read.push(("the sketch finished", where_the_reader_stands(&app)));
    driver::click_the_button(&mut app, "Étape suivante");
    read.push(("▶", where_the_reader_stands(&app)));
    driver::click_the_button(&mut app, "Étape précédente");
    read.push(("◀", where_the_reader_stands(&app)));
    driver::click_the_button(&mut app, "Étape 6");
    read.push(("« Étape 6 » clicked", where_the_reader_stands(&app)));

    assert_eq!(
        read,
        [
            ("a sketch opened", stands("4 / 5", Some("Esquisse"))),
            ("its rectangle raised", stands("5 / 6", Some("Extrusion"))),
            ("the sketch finished", stands("5 / 6", Some("Extrusion"))),
            ("▶", stands("6 / 6", None)),
            ("◀", stands("5 / 6", Some("Extrusion"))),
            ("« Étape 6 » clicked", stands("6 / 6", None)),
        ],
        "read to its third box of four, the part takes a sketch and then the extrusion of \
         that sketch: the reader does not stay on each step as it is put in, the box after \
         it the hole and dashed, nor play the hole again from ▶ or the last box, as (gesture, \
         (counter, the kind of the step read to while the rest is hidden))",
    );
}

/// How far a place of the chrome may move before it is taken to have moved,
/// in points.
const STILL: f32 = 0.5;

/// The reader's row as it stands: ◀ and ▶, and which boxes of the `boxes`
/// the part has stand between them, counting from one. ▶ closes a row as wide
/// as the card, which is as wide as the panel: a panel grown wider, or a row
/// run past its end, moves it.
struct Row {
    previous: egui::Rect,
    next: egui::Rect,
    seen: Vec<usize>,
}

fn the_reader_s_row(app: &driver::App, boxes: usize) -> Row {
    let end = |name: &str| {
        driver::the_row(app, name)
            .unwrap_or_else(|| panic!("no « {name} » at the end of the reader's row"))
            .rect()
    };
    let (previous, next) = (end("Étape précédente"), end("Étape suivante"));
    let seen = (1..=boxes)
        .filter(|rank| {
            driver::the_row(app, &format!("Étape {rank}")).is_some_and(|node| {
                let at = node.rect();
                at.left() >= previous.right() && at.right() <= next.left()
            })
        })
        .collect();
    Row {
        previous,
        next,
        seen,
    }
}

fn add_a_sketch(app: &mut driver::App) {
    driver::bring_into_the_ribbon(app, "Nouvelle esquisse");
    driver::start_a_sketch(app);
    driver::draw_a_segment(app);
    driver::finish_the_sketch(app);
}

#[test]
fn forty_steps_scroll_sideways_in_the_row_rather_than_overflowing_the_panel() {
    let mut app = driver::open("forty_steps_scroll_sideways");
    driver::create_a_part(&mut app, "Platine");
    add_a_sketch(&mut app);
    assert_eq!(
        read_of(&app, 1),
        (Some("1 / 1".to_string()), false),
        "the fixture wants a part of one step",
    );
    let one = the_reader_s_row(&app, 1);
    assert!(
        one.seen == [1] && driver::the_row(&app, "Étape 2").is_none(),
        "with one step the row does not show exactly one box between ◀ and ▶: {:?} seen, \
         showing {:?}",
        one.seen,
        driver::on_screen(&app),
    );

    for _ in 1..40 {
        add_a_sketch(&mut app);
    }
    assert_eq!(
        read_of(&app, 40),
        (Some("40 / 40".to_string()), false),
        "the fixture wants a part of forty steps",
    );
    let forty = the_reader_s_row(&app, 40);

    let moved: Vec<(&str, f32, f32)> = [
        ("◀", one.previous.left(), forty.previous.left()),
        ("▶", one.next.right(), forty.next.right()),
    ]
    .into_iter()
    .filter(|(_, then, now)| (now - then).abs() > STILL)
    .collect();
    assert!(
        moved.is_empty(),
        "forty steps pushed the row out of the panel, or the panel wider, as (what, left or \
         right edge with one step, with forty): {moved:?}",
    );
    assert!(
        !forty.seen.is_empty() && forty.seen.len() < 40,
        "of forty boxes, {} stand between ◀ and ▶: the row does not scroll sideways with \
         some of them in view and the rest out of it",
        forty.seen.len(),
    );
}

/// The box the part is read to, as the counter of a part of `boxes` boxes
/// says it.
fn the_box_read_to(app: &driver::App, boxes: usize) -> Option<usize> {
    (1..=boxes).find(|played| driver::reads(app, &format!("{played} / {boxes}")))
}

#[test]
fn the_row_scrolls_to_the_box_read_to() {
    let mut app = driver::open("the_row_scrolls_to_the_box_read_to");
    driver::create_a_part(&mut app, "Platine");
    for _ in 0..40 {
        add_a_sketch(&mut app);
    }

    let mut out_of_view = Vec::new();
    let mut look = |app: &driver::App, after: &str| match the_box_read_to(app, 40) {
        None => out_of_view.push(format!("{after}: no counter « n / 40 »")),
        Some(read_to) => {
            let in_view = the_reader_s_row(app, 40).seen;
            if !in_view.contains(&read_to) {
                out_of_view.push(format!(
                    "{after}: « Étape {read_to} » out of view, {in_view:?} in view"
                ));
            }
        }
    };
    look(&app, "unread");
    for press in 1..40 {
        driver::click_the_button(&mut app, "Étape précédente");
        look(&app, &format!("◀ pressed {press} times"));
    }
    driver::click_the_button(&mut app, "Étape 2");
    look(&app, "« Étape 2 » clicked");
    for press in 1..39 {
        driver::click_the_button(&mut app, "Étape suivante");
        look(&app, &format!("▶ pressed {press} times"));
    }

    assert!(
        out_of_view.is_empty(),
        "the box the part is read to is not in view between ◀ and ▶ {} times out of 79, \
         first:\n{}",
        out_of_view.len(),
        out_of_view
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

/// The counter « n / total » the reader shows, as (n, total).
fn the_counter(app: &driver::App) -> Option<(usize, usize)> {
    texts(app).iter().find_map(|text| {
        let (played, total) = text.split_once(" / ")?;
        Some((played.parse().ok()?, total.parse().ok()?))
    })
}

/// Where the reader stands, as its counter and the name of the step read to,
/// off the line under the row while that line says the rest is hidden.
fn where_it_is_read_to(app: &driver::App) -> (Option<String>, Option<String>) {
    let counter = the_counter(app).map(|(played, total)| format!("{played} / {total}"));
    let name = texts(app)
        .iter()
        .find_map(|text| text.strip_suffix(" · la suite est masquée"))
        .map(str::to_string);
    (counter, name)
}

fn read_to(counter: &str, name: &str) -> (Option<String>, Option<String>) {
    (Some(counter.to_string()), Some(name.to_string()))
}

/// « Annuler » until the reader counts `boxes` boxes: a step put in is its
/// opening and what was drawn in it, taken back one at a time.
fn take_back_a_box(app: &mut driver::App, boxes: usize) {
    for _ in 0..5 {
        if the_counter(app).is_some_and(|(_, total)| total == boxes) {
            return;
        }
        driver::click(app, "Annuler");
    }
    panic!(
        "five « Annuler » did not bring the reader to {boxes} boxes: {:?}",
        the_counter(app),
    );
}

/// « Rétablir » until nothing is left to redo.
fn bring_back_all(app: &mut driver::App) {
    for _ in 0..5 {
        if !driver::is_enabled(app, "Rétablir") {
            return;
        }
        driver::click(app, "Rétablir");
    }
}

#[test]
fn undoing_an_insertion_moves_the_reading_point_only_when_its_box_goes() {
    let mut app = a_block_and_sketches("undoing_an_insertion_moves_the_reading_point", 3);
    let mut read = Vec::new();

    driver::click_the_button(&mut app, "Étape 4");
    add_a_sketch(&mut app);
    driver::click_the_button(&mut app, "Étape 3");
    read.push((
        "read to « Esquisse 2 », « Esquisse 5 » put in two boxes after it",
        where_it_is_read_to(&app),
    ));
    take_back_a_box(&mut app, 5);
    read.push((
        "« Esquisse 5 », after the box read to, taken back",
        where_it_is_read_to(&app),
    ));
    bring_back_all(&mut app);
    read.push(("« Esquisse 5 » brought back", where_it_is_read_to(&app)));

    driver::click_the_button(&mut app, "Étape 2");
    add_a_sketch(&mut app);
    read.push((
        "« Esquisse 6 » put in after « Extrusion 1 »",
        where_it_is_read_to(&app),
    ));
    take_back_a_box(&mut app, 6);
    read.push((
        "« Esquisse 6 », the box read to, taken back",
        where_it_is_read_to(&app),
    ));
    bring_back_all(&mut app);
    read.push(("« Esquisse 6 » brought back", where_it_is_read_to(&app)));

    driver::click_the_button(&mut app, "Étape 5");
    read.push((
        "read to « Esquisse 3 », « Esquisse 6 » before it",
        where_it_is_read_to(&app),
    ));
    take_back_a_box(&mut app, 6);
    read.push((
        "« Esquisse 6 », before the box read to, taken back",
        where_it_is_read_to(&app),
    ));
    bring_back_all(&mut app);
    read.push(("« Esquisse 6 » brought back", where_it_is_read_to(&app)));

    assert_eq!(
        read,
        [
            (
                "read to « Esquisse 2 », « Esquisse 5 » put in two boxes after it",
                read_to("3 / 6", "Esquisse 2"),
            ),
            (
                "« Esquisse 5 », after the box read to, taken back",
                read_to("3 / 5", "Esquisse 2"),
            ),
            (
                "« Esquisse 5 » brought back",
                read_to("3 / 6", "Esquisse 2")
            ),
            (
                "« Esquisse 6 » put in after « Extrusion 1 »",
                read_to("3 / 7", "Esquisse 6"),
            ),
            (
                "« Esquisse 6 », the box read to, taken back",
                read_to("2 / 6", "Extrusion 1"),
            ),
            (
                "« Esquisse 6 » brought back",
                read_to("2 / 7", "Extrusion 1")
            ),
            (
                "read to « Esquisse 3 », « Esquisse 6 » before it",
                read_to("5 / 7", "Esquisse 3"),
            ),
            (
                "« Esquisse 6 », before the box read to, taken back",
                read_to("4 / 6", "Esquisse 3"),
            ),
            (
                "« Esquisse 6 » brought back",
                read_to("5 / 7", "Esquisse 3")
            ),
        ],
        "the reading point does not follow its box through undo and redo — still when a box \
         after it goes, onto the box before when its own goes, on the same box when one \
         before it goes or comes back — as (what was done, (counter, the step read to))",
    );
}

/// Every text on screen, where it stands, and whether it is a button.
fn texts_placed(app: &driver::App) -> Vec<(egui::Rect, String, bool)> {
    app.get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            let text = read.label().or_else(|| read.value())?;
            let button = read.role() == egui::accesskit::Role::Button;
            Some((node.rect(), text, button))
        })
        .collect()
}

/// The name the line under the reader's row gives the step read to, the rest
/// hidden or not: the one text, no button and off the title bar's row, that
/// starts with the name of a step. No sketch is open while it is read.
fn the_reader_s_line(app: &driver::App) -> Option<String> {
    let title_bar = driver::the_row(app, "Accueil").map(|home| home.rect());
    texts_placed(app)
        .into_iter()
        .filter(|(at, _, button)| {
            !button && !title_bar.is_some_and(|row| row.y_range().contains(at.center().y))
        })
        .map(|(_, text, _)| text)
        .find(|text| text.starts_with("Esquisse ") || text.starts_with("Extrusion "))
        .map(|line| line.trim_end_matches(" · la suite est masquée").to_string())
}

/// The name of each of the reader's boxes, in the row's order, each read off
/// the line under the row once it is clicked.
fn the_reader_s_names(app: &mut driver::App) -> Vec<Option<String>> {
    let boxes = the_counter(app).map_or(0, |(_, total)| total);
    (1..=boxes)
        .map(|rank| {
            driver::click_the_button(app, &format!("Étape {rank}"));
            the_reader_s_line(app)
        })
        .collect()
}

/// The names of the history's lines that open a key step, in the order of the
/// lines; the history unfolded to read them, then folded again.
fn the_history_s_key_steps(app: &mut driver::App) -> Vec<String> {
    click_the_history_s_header(app);
    let lines = driver::steps(app);
    click_the_history_s_header(app);
    lines
        .iter()
        .filter_map(|line| line.split_once(". "))
        .map(|(_, name)| name.to_string())
        .filter(|name| name.starts_with("Esquisse ") || name.starts_with("Extrusion "))
        .collect()
}

#[test]
fn the_history_lists_the_order_done_and_the_reader_the_order_of_the_part() {
    let mut app = a_block_with_a_hole("the_history_lists_the_order_done");
    driver::click_the_button(&mut app, "Étape 3");
    driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::start_a_sketch(&mut app);
    extrude_a_rectangle(&mut app, BESIDE_THE_BLOCK, "3", "Ajout de matière");
    finish_after_extruding(&mut app);

    let history = the_history_s_key_steps(&mut app);
    let reader = the_reader_s_names(&mut app);

    let all = |names: &[&str]| {
        names
            .iter()
            .map(|name| name.to_string())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        (history, reader),
        (
            all(&[
                "Esquisse 1",
                "Extrusion 1",
                "Esquisse 2",
                "Extrusion 2",
                "Esquisse 3",
                "Extrusion 3",
            ]),
            all(&[
                "Esquisse 1",
                "Extrusion 1",
                "Esquisse 2",
                "Esquisse 3",
                "Extrusion 3",
                "Extrusion 2",
            ])
            .into_iter()
            .map(Some)
            .collect(),
        ),
        "a sketch and its extrusion put in after « Esquisse 2 », before the hole: the history \
         does not list them last, in the order they were done, or the reader does not show \
         them before the hole, in the order of the part, as (the history, the reader)",
    );
}

/// The sketches the extrusion offers to raise, by the numbers its buttons
/// read.
fn the_sketches_offered(app: &driver::App) -> Vec<u32> {
    let mut offered: Vec<u32> = texts_placed(app)
        .into_iter()
        .filter(|(_, _, button)| *button)
        .filter_map(|(_, text, _)| text.parse().ok())
        .collect();
    offered.sort_unstable();
    offered
}

/// The extrusion armed then let go with its own « Annuler », which leaves it
/// asking which sketch to raise.
fn ask_which_sketch_to_raise(app: &mut driver::App) {
    driver::click(app, "Ajout de matière");
    driver::click_the_button(app, "Annuler");
}

#[test]
fn while_read_to_a_box_the_extrusion_offers_only_the_sketches_read() {
    let mut app = a_block_and_sketches("the_extrusion_offers_only_the_sketches_read", 1);
    driver::click_the_button(&mut app, "Étape 2");
    ask_which_sketch_to_raise(&mut app);
    let read_to_the_block = the_sketches_offered(&app);
    driver::click_the_button(&mut app, "Étape 3");
    let read_whole = the_sketches_offered(&app);

    assert_eq!(
        (read_to_the_block, read_whole),
        (vec![1], vec![1, 2]),
        "the extrusion does not offer, while the part is read to « Extrusion 1 », only the \
         sketch made before it, and every sketch once read whole, as (read to the block, \
         read whole)",
    );
}

/// « Bloc », then a second sketch on YZ with a rectangle beside the block,
/// finished and raising nothing: three boxes, the extrusion offered on that
/// second sketch as finishing one does.
fn a_block_and_a_rectangle_beside_it(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at_sketch_point(&mut app, BESIDE_THE_BLOCK[0]);
    driver::click_at_sketch_point(&mut app, BESIDE_THE_BLOCK[1]);
    driver::finish_the_sketch(&mut app);
    assert_eq!(
        the_counter(&app),
        Some((3, 3)),
        "the fixture wants three boxes, the part read to its end",
    );
    app
}

/// `a_block_and_a_rectangle_beside_it`, read to « Extrusion 1 » first when
/// `read_to_the_block`, then « Esquisse 2 » opened from the tree and its
/// rectangle raised: the reader's names once the part is read whole, and how
/// much matter the rectangle added to the part put away.
fn the_rectangle_raised_from_the_tree(
    test: &str,
    read_to_the_block: bool,
) -> (Vec<Option<String>>, f64) {
    let mut app = a_block_and_a_rectangle_beside_it(test);
    if read_to_the_block {
        driver::click_the_button(&mut app, "Étape 2");
    }
    driver::double_click(&mut app, "Esquisse 2");
    driver::click_exactly(&mut app, "Extrusion");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at_sketch_point(&mut app, BESIDE_THE_BLOCK[2]);
    driver::click(&mut app, "Appliquer");
    finish_after_extruding(&mut app);
    driver::click_the_button(&mut app, "Étape 4");
    let reader = the_reader_s_names(&mut app);

    let part = driver::the_part_put_away(&mut app, test, "Bloc");
    let block = part
        .shown_through(&std::collections::BTreeMap::new(), 1)
        .body
        .volume();
    (reader, part.body().volume() - block)
}

#[test]
fn a_sketch_opened_past_the_reading_point_is_raised_after_itself() {
    let (reader, raised) =
        the_rectangle_raised_from_the_tree("a_sketch_opened_past_the_reading_point", true);
    let (_, raised_read_whole) =
        the_rectangle_raised_from_the_tree("a_sketch_opened_with_the_part_read_whole", false);

    assert_eq!(
        reader,
        ["Esquisse 1", "Extrusion 1", "Esquisse 2", "Extrusion 2"]
            .map(|name| Some(name.to_string())),
        "the rectangle beside the block, its sketch opened from the tree while the part was \
         read to « Extrusion 1 », was raised by a step put in before its own sketch",
    );
    assert!(
        raised_read_whole > 0.0 && (raised - raised_read_whole).abs() < 1e-6 * raised_read_whole,
        "the rectangle beside the block, raised while the part was read to « Extrusion 1 », \
         adds {raised} to the part, where the same gestures on the part read whole add \
         {raised_read_whole}",
    );
}

#[test]
fn a_sketch_laid_on_a_kept_section_while_read_to_a_box_goes_at_the_end() {
    let mut app = a_block_and_sketches("a_sketch_laid_on_a_kept_section_while_read", 1);
    driver::click_the_button(&mut app, "Étape 2");
    driver::click_in_the_bar(&mut app, "Vue en coupe");
    driver::click(&mut app, "Plan YZ");
    driver::click_the_button(&mut app, "Enregistrer la coupe");
    driver::click_the_button(&mut app, "Fermer");
    driver::click_the_button(&mut app, "Coupe 1");
    driver::click_in_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, driver::ON_THE_CUT);
    driver::draw_a_segment(&mut app);
    driver::finish_the_sketch(&mut app);

    assert!(
        driver::in_the_tree(&app, "Esquisse 3"),
        "no sketch was laid on the section's cut: {:?}",
        driver::on_screen(&app),
    );
    assert_eq!(
        (
            the_counter(&app),
            driver::shows_or_says(&app, "face perdue")
        ),
        (Some((4, 4)), false),
        "a sketch laid on a kept section's cut while the part was read to « Extrusion 1 » \
         does not go at the end, the reading point with it, or it lost the section it was \
         laid on, as (the counter, « face perdue » shown)",
    );
}

/// « Bloc » with a hole cut 30 deep: through the block's 10 and as far again
/// beyond its face at x = 10 — four boxes.
fn a_block_with_a_hole_cut_30_deep(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at_sketch_point(&mut app, THE_HOLE[0]);
    driver::click_at_sketch_point(&mut app, THE_HOLE[1]);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_the_button(&mut app, "2");
    driver::click(&mut app, "Enlèvement de matière");
    driver::click_at_sketch_point(&mut app, THE_HOLE[2]);
    driver::type_over(&mut app, "10", "30");
    driver::click(&mut app, "Appliquer");
    finish_after_extruding(&mut app);
    assert_eq!(
        the_counter(&app),
        Some((4, 4)),
        "the fixture wants four boxes, the part read to its end",
    );
    app
}

/// Where the boss put in is drawn, on YZ inside the hole's outline, as two
/// corners and a place inside them: corners the cursor lands on as they are,
/// clear of the pull the plane's axis has on a point near it.
const INSIDE_THE_HOLE: [DVec2; 3] = [
    DVec2::new(31.3, 16.7),
    DVec2::new(55.0, -17.5),
    DVec2::new(43.0, 0.0),
];

#[test]
fn an_extrusion_put_in_is_judged_on_the_part_read_through_it() {
    let mut app = a_block_with_a_hole_cut_30_deep("an_extrusion_put_in_is_judged");
    driver::click_the_button(&mut app, "Étape 2");
    driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at_sketch_point(&mut app, INSIDE_THE_HOLE[0]);
    driver::click_at_sketch_point(&mut app, INSIDE_THE_HOLE[1]);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_the_button(&mut app, "3");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at_sketch_point(&mut app, INSIDE_THE_HOLE[2]);
    driver::type_over(&mut app, "10", "20");
    driver::click(&mut app, "Appliquer");

    assert_eq!(
        the_counter(&app),
        Some((4, 6)),
        "the fixture wants the boss and its sketch put in after the block, the part read to \
         the boss",
    );
    assert!(
        !driver::shows_or_says(&app, "L'extrusion n'a rien ajouté"),
        "a boss raised 20 through the block and 10 past its face, which the part read to it \
         shows, is said to have added nothing because the hole after it cuts it away from \
         the whole part: {:?}",
        driver::said(&app),
    );
}
