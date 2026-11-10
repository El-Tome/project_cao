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

mod driver;

/// A part with a sketch started on YZ and one segment drawn on it: two
/// actions, « Esquisse 1 » on « Plan YZ » and « Trait ».
fn a_part_with_a_segment(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    app
}

const THE_TWO_LINES: [&str; 2] = ["1. Esquisse 1", "2. Trait"];

fn the_header(app: &driver::App) -> egui::Rect {
    driver::the_row(app, "Historique")
        .unwrap_or_else(|| {
            panic!(
                "no « Historique » to unfold at the bottom of the Part panel: {:?}",
                driver::on_screen(app),
            )
        })
        .rect()
}

fn click_the_header(app: &mut driver::App) {
    driver::the_row(app, "Historique")
        .expect("a « Historique » to click")
        .click();
    app.run();
}

#[test]
fn the_history_sits_folded_at_the_bottom_of_the_part_panel() {
    let app = a_part_with_a_segment("the_history_sits_folded");

    let header = the_header(&app);
    let variables = driver::where_it_reads(&app, "VARIABLES")
        .expect("the Part panel reads its « VARIABLES » heading");

    assert!(
        driver::stands_beside_the_view(&app, header) && header.top() >= variables.bottom(),
        "« Historique » stands at {header:?}, not under the Part panel's last heading \
         at {variables:?}",
    );
    assert!(
        driver::reads(&app, "2 actions"),
        "the header does not count the two actions: {:?}",
        driver::on_screen(&app),
    );
    assert_eq!(
        driver::steps(&app),
        Vec::<String>::new(),
        "folded, the history shows none of its lines",
    );
}

#[test]
fn a_click_on_the_history_header_unfolds_it() {
    let mut app = a_part_with_a_segment("a_click_unfolds_the_history");
    the_header(&app);

    click_the_header(&mut app);

    assert_eq!(driver::steps(&app), THE_TWO_LINES);
}

#[test]
fn h_folds_and_unfolds_the_history() {
    let mut app = a_part_with_a_segment("h_folds_and_unfolds_the_history");
    the_header(&app);

    let folded = driver::steps(&app);
    driver::press(&mut app, egui::Key::H);
    let unfolded = driver::steps(&app);
    driver::press(&mut app, egui::Key::H);
    let folded_again = driver::steps(&app);

    assert_eq!(
        (folded, unfolded, folded_again),
        (
            Vec::new(),
            THE_TWO_LINES.map(String::from).to_vec(),
            Vec::new()
        ),
        "the lines shown (before, after H, after H again)",
    );
}

#[test]
fn no_history_panel_of_its_own_opens_beside_the_part() {
    let mut app = a_part_with_a_segment("no_history_panel_of_its_own");
    let before = driver::canvas(&app);

    driver::open_the_history(&mut app);

    assert_eq!(
        driver::canvas(&app),
        before,
        "the view gave room to a panel when the history unfolded",
    );
    assert!(
        !driver::shows(&app, "Pièce vide"),
        "the old panel's first line is still on screen: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn an_unfolded_line_reads_its_name_above_its_detail() {
    let mut app = a_part_with_a_segment("a_line_reads_its_detail_under_it");

    driver::open_the_history(&mut app);

    assert_eq!(
        driver::detail_of(&app, "1. Esquisse 1").as_deref(),
        Some("Plan YZ"),
        "the sketch's line does not read its plane under its name: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_click_on_a_line_puts_the_part_back_to_that_action() {
    let mut app = a_part_with_a_segment("a_click_on_a_line_rewinds");
    driver::open_the_history(&mut app);
    assert!(driver::in_the_tree(&app, "Trait 1"), "the segment is drawn");

    driver::click_the_button(&mut app, "1. Esquisse 1");

    assert!(
        !driver::in_the_tree(&app, "Trait 1"),
        "back to the sketch's own action, the segment drawn after it is gone: {:?}",
        driver::on_screen(&app),
    );
    assert_eq!(
        driver::steps(&app),
        THE_TWO_LINES,
        "the segment's action stays listed, undone, to be gone back to",
    );
    assert!(
        driver::is_enabled(&app, "Rétablir"),
        "what the click took back can be redone",
    );
}

#[test]
fn the_compact_link_at_the_bottom_asks_before_compacting() {
    let mut app = a_part_with_a_segment("the_compact_link_asks");
    driver::click(&mut app, "Annuler");
    driver::open_the_history(&mut app);
    let link = driver::where_it_reads(&app, "Compacter l'historique…").unwrap_or_else(|| {
        panic!(
            "no « Compacter l'historique… » under the unfolded history: {:?}",
            driver::on_screen(&app),
        )
    });
    let last = driver::where_it_reads(&app, "2. Trait").expect("the undone segment's line");
    assert!(
        link.top() >= last.top(),
        "the link stands at {link:?}, above the last line at {last:?}",
    );

    driver::click_the_button(&mut app, "Compacter l'historique…");

    assert!(
        driver::says(&app, "Compacter l'historique ?"),
        "nothing asked before compacting: {:?}",
        driver::said(&app),
    );
    assert_eq!(
        driver::steps(&app),
        THE_TWO_LINES,
        "the history was compacted before the question was answered",
    );

    driver::click_exactly(&mut app, "Compacter");

    assert_eq!(
        driver::steps(&app),
        ["1. Esquisse 1"],
        "compacting drops the undone segment, as it always did",
    );
    assert!(
        driver::reads(&app, "1 action"),
        "{:?}",
        driver::on_screen(&app)
    );
}

/// Fifteen traits drawn in one run of the line tool, zigzagging across the
/// sketch: a history far taller than its cap once unfolded.
fn a_part_with_a_long_history(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Ligne (L)");
    for rank in 0..16 {
        let up = if rank % 2 == 0 { 80.0 } else { -30.0 };
        driver::click_at_sketch_point(&mut app, glam::DVec2::new(-20.0 + 10.0 * rank as f64, up));
    }
    driver::press(&mut app, egui::Key::Escape);
    app
}

#[test]
fn a_long_history_unfolds_no_taller_than_230_points() {
    let mut app = a_part_with_a_long_history("a_long_history_is_capped");
    driver::open_the_history(&mut app);
    assert!(
        driver::steps(&app).len() >= 12,
        "too short a history to reach the cap: {:?}",
        driver::steps(&app),
    );

    let header = the_header(&app);
    let link = driver::where_it_reads(&app, "Compacter l'historique…")
        .expect("the link at the foot of the list");

    assert!(
        link.top() - header.bottom() <= 230.0 + LINK_SPACING,
        "the list runs {} points from its header to the link under it",
        link.top() - header.bottom(),
    );
    assert!(
        app.ctx.content_rect().contains(link.center()),
        "the link under the list is pushed off the screen, at {link:?} of {:?}",
        app.ctx.content_rect(),
    );
}

/// Room the link may take from the list above it, beyond the list's own cap.
const LINK_SPACING: f32 = 8.0;

#[test]
fn a_right_click_on_a_sketch_s_line_offers_to_edit_it_and_reopens_it() {
    let mut app = a_part_with_a_segment("a_right_click_reopens_the_sketch");
    driver::finish_the_sketch(&mut app);
    driver::open_the_history(&mut app);
    assert!(!driver::a_sketch_is_open(&app), "the sketch was finished");

    driver::right_click(&mut app, "1. Esquisse 1");
    driver::click_the_button(&mut app, "Modifier l'esquisse");

    assert!(
        driver::a_sketch_is_open(&app),
        "« Modifier l'esquisse » left the sketch closed: {:?}",
        driver::on_screen(&app),
    );
}
