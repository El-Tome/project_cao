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
//!   #737) —
//!   `adding_a_step_while_read_to_step_three_brings_the_reading_point_back_to_the_end`
//! - Forty steps scroll sideways in the row rather than overflowing the panel
//!   — `forty_steps_scroll_sideways_in_the_row_rather_than_overflowing_the_panel`
//!
//! A box is found by the name it gives whoever cannot see its icon, « Étape
//! n », its place in the row counting from one; ◀ and ▶ by theirs, « Étape
//! précédente » and « Étape suivante », as `docs/redesign/icons.md` names
//! them. Not by the step's own name: the tree's row already answers to it,
//! and a gesture that finds a row by a part of its name would find two.

mod driver;

/// « Bloc », a block raised off a rectangle drawn on YZ, then two sketches
/// with a trait drawn on each, finished: four boxes, « Esquisse 1 »,
/// « Extrusion 1 », « Esquisse 2 », « Esquisse 3 ».
fn a_part_of_four_key_steps(test: &str) -> driver::App {
    let mut app = driver::a_block(test);
    for _ in 0..2 {
        driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
        driver::start_a_sketch(&mut app);
        driver::draw_a_segment(&mut app);
        driver::finish_the_sketch(&mut app);
    }
    assert!(
        driver::in_the_tree(&app, "Esquisse 3"),
        "the fixture wants a third sketch in the tree: {:?}",
        driver::on_screen(&app),
    );
    app
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
