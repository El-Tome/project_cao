//! What a sketch is called wherever its name shows — its row in the Part tree,
//! its box in the step reader, its line in the history and the detail under
//! what was drawn in it, and the pill and the title bar while it is open:
//! « Esquisse » and the number it was given, which
//! no sketch put in before it, and no sketch dropped before it, changes.
//!
//! Closes #737 — the names the interface shows; the number they are written
//! from is the part's, held in `crates/part/tests/inserting_a_step.rs`, and the
//! reading point in `crates/app/tests/the_step_reader.rs`.
//! - Names: a sketch inserted between « Esquisse 2 » and « Esquisse 3 » takes
//!   the next number, « Esquisse 4 ». The others are not renumbered —
//!   `a_sketch_inserted_after_the_second_takes_the_next_number_and_the_others_keep_theirs`
//! - Numbering rule: "the next number" is the highest number ever used plus
//!   one; a name is never reused, even if the sketch that bore it was deleted —
//!   `a_number_taken_back_and_dropped_is_not_shown_again_on_the_next_sketch`
//! - Must not break: every name an alert or a dimension uses to find its step
//!   still finds it after an insertion — the sentences that name a sketch, an
//!   alert's and a dimension's line in the history, naming it as its row does —
//!   `a_sentence_naming_a_sketch_names_it_as_its_row_does_once_a_number_was_dropped`
//!
//! A name is read where the user reads it. The tree's row is a button that
//! reads exactly it; the history's line reads « n. » before it; the pill and
//! the title bar are told apart by where they stand, the pill inside the view
//! on the row of its « Terminer », the title bar on the row of « Accueil ». The
//! reader's boxes answer to « Étape n » alone, so a box's name is read off the
//! line under the row once the box is clicked: the one text outside the view
//! and the title bar, and no button, that starts with the name of a step.

use egui_kittest::kittest::{NodeT, Queryable};

mod driver;

/// « Bloc », a block raised off a rectangle drawn on YZ, then `sketches`
/// sketches with a trait drawn on each, finished.
fn a_block_and_sketches(test: &str, sketches: usize) -> driver::App {
    let mut app = driver::a_block(test);
    for _ in 0..sketches {
        add_a_sketch(&mut app);
    }
    app
}

fn open_a_sketch(app: &mut driver::App) {
    driver::bring_into_the_ribbon(app, "Nouvelle esquisse");
    driver::start_a_sketch(app);
}

fn add_a_sketch(app: &mut driver::App) {
    open_a_sketch(app);
    draw_and_finish(app);
}

fn draw_and_finish(app: &mut driver::App) {
    driver::draw_a_segment(app);
    driver::finish_the_sketch(app);
}

/// Every text on screen, where it stands, and whether it is a button.
fn texts(app: &driver::App) -> Vec<(egui::Rect, String, bool)> {
    app.get_all_by(|_| true)
        .filter_map(|node| {
            let read = node.accesskit_node();
            let text = read.label().or_else(|| read.value())?;
            let button = read.role() == egui::accesskit::Role::Button;
            Some((node.rect(), text, button))
        })
        .collect()
}

/// The sketch a text starts by naming, « Esquisse n ».
fn sketch_named_by(text: &str) -> Option<String> {
    let mut words = text.split_whitespace();
    match (words.next(), words.next()) {
        (Some("Esquisse"), Some(number)) if number.parse::<u32>().is_ok() => {
            Some(format!("Esquisse {number}"))
        }
        _ => None,
    }
}

fn the_title_bar_s_row(app: &driver::App) -> Option<egui::Rect> {
    driver::the_row(app, "Accueil").map(|home| home.rect())
}

/// The sketch the title bar names, on the row of « Accueil ».
fn the_title_bar_s_sketch(app: &driver::App) -> Option<String> {
    let row = the_title_bar_s_row(app)?;
    texts(app)
        .into_iter()
        .filter(|(at, _, _)| row.y_range().contains(at.center().y))
        .find_map(|(_, text, _)| sketch_named_by(&text))
}

/// The sketch the pill names, inside the view on the row of its « Terminer ».
fn the_pill_s_sketch(app: &driver::App) -> Option<String> {
    let finish = driver::the_pill_s_finish(app)?.rect();
    let view = driver::canvas(app);
    texts(app)
        .into_iter()
        .filter(|(at, _, _)| view.contains_rect(*at) && finish.y_range().contains(at.center().y))
        .find_map(|(_, text, _)| sketch_named_by(&text))
}

/// The sketches the Part tree has a row for, among the first ten numbers.
fn the_tree_s_sketches(app: &driver::App) -> Vec<String> {
    (1..=10)
        .map(|number| format!("Esquisse {number}"))
        .filter(|name| driver::in_the_tree(app, name))
        .collect()
}

fn click_the_history_s_header(app: &mut driver::App) {
    driver::the_row(app, "Historique")
        .expect("the history's header at the bottom of the Part panel")
        .click();
    app.run();
}

/// The sketch the detail under a line of the history names: a text just below
/// the top of the line, within its width. Not `driver::detail_of`, which takes
/// the nearest text there and, under the last line, finds « Compacter
/// l'historique… » as readily.
fn the_sketch_under(app: &driver::App, line: &str) -> Option<String> {
    let at = texts(app).into_iter().find(|(_, text, _)| text == line)?.0;
    texts(app)
        .into_iter()
        .filter(|(rect, _, _)| {
            rect.top() >= at.top() + 4.0
                && rect.top() <= at.top() + 40.0
                && rect.left() < at.right()
                && rect.right() > at.left()
        })
        .find_map(|(_, text, _)| sketch_named_by(&text))
}

/// The sketches the history's lines name, in the order of the lines, and the
/// sketch the detail under its last line names; the history unfolded to read
/// them, then folded again.
fn the_history_s_sketches(app: &mut driver::App) -> (Vec<String>, Option<String>) {
    click_the_history_s_header(app);
    let lines = driver::steps(app);
    let last = lines.last().and_then(|line| the_sketch_under(app, line));
    click_the_history_s_header(app);
    assert!(
        !lines.is_empty(),
        "a click on « Historique » unfolded no line: {:?}",
        driver::on_screen(app),
    );
    let named = lines
        .iter()
        .filter_map(|line| line.split_once(". "))
        .filter_map(|(_, name)| sketch_named_by(name))
        .collect();
    (named, last)
}

/// The name the line under the reader's row gives the step read to, without
/// « · la suite est masquée ».
fn the_reader_s_line(app: &driver::App) -> Option<String> {
    let view = driver::canvas(app);
    let title_bar = the_title_bar_s_row(app);
    texts(app)
        .into_iter()
        .filter(|(at, _, button)| {
            !button
                && !view.contains_rect(*at)
                && !title_bar.is_some_and(|row| row.y_range().contains(at.center().y))
        })
        .map(|(_, text, _)| text)
        .find(|text| text.starts_with("Esquisse ") || text.starts_with("Extrusion "))
        .map(|line| line.trim_end_matches(" · la suite est masquée").to_string())
}

/// The name of each of the reader's `boxes` boxes, in the row's order, each
/// read off the line under the row once it is clicked: the part is left read
/// to its end.
fn the_reader_s_names(app: &mut driver::App, boxes: usize) -> Vec<Option<String>> {
    (1..=boxes)
        .map(|rank| {
            driver::click_the_button(app, &format!("Étape {rank}"));
            the_reader_s_line(app)
        })
        .collect()
}

/// Every place a sketch's name shows: the pill and the title bar while the
/// last sketch was open, then the tree, the history — its lines, and the
/// detail under the trait drawn in that sketch — and the reader once it was
/// finished.
#[derive(Debug, PartialEq)]
struct Named {
    pill: Option<String>,
    title_bar: Option<String>,
    tree: Vec<String>,
    history: Vec<String>,
    the_trait_s_detail: Option<String>,
    reader: Vec<Option<String>>,
}

/// Where the sketch just made is `made`, and the others as listed.
fn named(made: &str, tree: &[&str], history: &[&str], reader: &[&str]) -> Named {
    let all = |names: &[&str]| names.iter().map(|name| name.to_string()).collect();
    Named {
        pill: Some(made.to_string()),
        title_bar: Some(made.to_string()),
        tree: all(tree),
        history: all(history),
        the_trait_s_detail: Some(made.to_string()),
        reader: reader.iter().map(|name| Some(name.to_string())).collect(),
    }
}

/// Opens a sketch, reads its name in the pill and the title bar, draws a
/// trait in it and finishes it, then reads every name the tree, the history
/// and the reader's `boxes` boxes give.
fn a_sketch_opened_then_every_name(app: &mut driver::App, boxes: usize) -> Named {
    open_a_sketch(app);
    let (pill, title_bar) = (the_pill_s_sketch(app), the_title_bar_s_sketch(app));
    draw_and_finish(app);
    let (history, the_trait_s_detail) = the_history_s_sketches(app);
    Named {
        pill,
        title_bar,
        tree: the_tree_s_sketches(app),
        history,
        the_trait_s_detail,
        reader: the_reader_s_names(app, boxes),
    }
}

#[test]
fn a_sketch_inserted_after_the_second_takes_the_next_number_and_the_others_keep_theirs() {
    let mut app = a_block_and_sketches("a_sketch_inserted_after_the_second", 2);
    driver::click_the_button(&mut app, "Étape 3");
    assert_eq!(
        the_reader_s_line(&app).as_deref(),
        Some("Esquisse 2"),
        "the fixture wants the part read to « Esquisse 2 », « Esquisse 3 » after it: {:?}",
        driver::on_screen(&app),
    );

    let seen = a_sketch_opened_then_every_name(&mut app, 5);

    assert_eq!(
        seen,
        named(
            "Esquisse 4",
            &["Esquisse 1", "Esquisse 2", "Esquisse 3", "Esquisse 4"],
            &["Esquisse 1", "Esquisse 2", "Esquisse 3", "Esquisse 4"],
            &[
                "Esquisse 1",
                "Extrusion 1",
                "Esquisse 2",
                "Esquisse 4",
                "Esquisse 3",
            ],
        ),
        "a sketch put in between « Esquisse 2 » and « Esquisse 3 » is not « Esquisse 4 » \
         everywhere its name shows, or a sketch before or after it changed its name",
    );
}

#[test]
fn a_number_taken_back_and_dropped_is_not_shown_again_on_the_next_sketch() {
    let mut app = a_block_and_sketches("a_number_taken_back_and_dropped", 2);
    for _ in 0..5 {
        if !driver::in_the_tree(&app, "Esquisse 3") {
            break;
        }
        driver::click(&mut app, "Annuler");
    }
    assert_eq!(
        the_tree_s_sketches(&app),
        ["Esquisse 1", "Esquisse 2"],
        "the fixture wants « Esquisse 3 » taken back, and nothing before it",
    );

    let seen = a_sketch_opened_then_every_name(&mut app, 4);

    assert_eq!(
        seen,
        named(
            "Esquisse 4",
            &["Esquisse 1", "Esquisse 2", "Esquisse 4"],
            &["Esquisse 1", "Esquisse 2", "Esquisse 4"],
            &["Esquisse 1", "Extrusion 1", "Esquisse 2", "Esquisse 4"],
        ),
        "the sketch opened once « Esquisse 3 » was taken back, which dropped it, is not \
         « Esquisse 4 » everywhere its name shows: the 3 is given again",
    );
}
