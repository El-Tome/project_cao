//! Which tab the ribbon shows as a sketch is entered and left: « Esquisse »
//! on the way in, the tab from before on the way out by « Terminer », and the
//! user's own choice in between. The tab is read as the user reads it, from
//! which of the ribbon's tabs is drawn selected.
//!
//! Closes #779.
//! - entering a sketch, whether an existing one is reopened or a new one is
//!   started, puts the ribbon on the « Esquisse » tab, so its tools are in
//!   reach at once — `reopening_a_sketch_puts_the_ribbon_on_its_tab`, and
//!   `starting_a_sketch_by_its_shortcut_puts_the_ribbon_on_its_tab` for a new
//!   one begun while the ribbon shows another tab
//! - leaving it with « Terminer » puts the ribbon back on the tab it was on
//!   before the sketch was entered, whichever of the pill's button, the
//!   ribbon's button and Enter finishes it —
//!   `finishing_a_sketch_brings_back_the_tab_it_was_entered_from`
//! - a tab picked by hand while the sketch is edited is kept: the ribbon moves
//!   only on entering and on leaving, never during the edit, and leaving goes
//!   back to the tab from before the sketch rather than to the one picked —
//!   `a_tab_picked_by_hand_while_sketching_is_kept_through_the_edit`
//! - decided without the user: only « Terminer » brings the previous tab
//!   back. Opening an extrusion from the tree while sketching lands on the
//!   Extrusion tab, as it did —
//!   `opening_an_extrusion_from_the_tree_while_sketching_still_lands_on_its_tab`;
//!   a sketch closed by undo or redo leaves the tab where it is —
//!   `a_sketch_closed_by_undo_leaves_the_tab_where_it_is`, undo standing for
//!   both since one command answers for the two. Either way out forgets the
//!   tab from before that sketch: the next one's « Terminer » goes back to the
//!   tab the next one was entered from, which both tests hold too. No gesture
//!   of the tree opens an extrusion today; « Modifier » on a volume's alert
//!   asks the tree's `TreeAction::EditExtrusion`, and is the way the test goes
//! - decided without the user: reopening another sketch while one is open
//!   keeps the tab from before the first, so that « Terminer » goes back to
//!   the tab from before any sketch —
//!   `reopening_another_sketch_while_one_is_open_keeps_the_tab_from_before_the_first`
//!
//! What must not break:
//! - every shortcut and every command stays reachable — no test: this
//!   issue changes which tab is shown, never what a tab or a chord holds, and
//!   `every_command_of_the_default_tree_can_be_reached` in `ribbon_groups.rs`
//!   and the shortcut tests of `cao_prefs` already hold that
//! - a sketch started from the tool itself already lands on the right tab and
//!   keeps doing so — `a_sketch_started_from_its_button_lands_on_its_tab`

mod driver;

use egui_kittest::kittest::{NodeT, Queryable};

const TABS: [&str; 3] = ["Esquisse", "Extrusion", "Export"];

/// The tabs of the ribbon drawn selected: one, while the ribbon works.
fn the_tab_shown(app: &driver::App) -> Vec<&'static str> {
    TABS.into_iter()
        .filter(|tab| {
            app.query_all_by_label(tab)
                .any(|node| node.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True))
        })
        .collect()
}

fn turn_the_ribbon_to(app: &mut driver::App, tab: &'static str) {
    driver::click_exactly(app, tab);
    assert_eq!(
        the_tab_shown(app),
        [tab],
        "the ribbon did not turn to « {tab} » when it was clicked: {:?}",
        driver::on_screen(app)
    );
}

fn the_sketch_tools_are_in_reach(app: &driver::App) -> bool {
    driver::the_ribbon_s(app, "Ligne (L)").is_some()
}

#[test]
fn reopening_a_sketch_puts_the_ribbon_on_its_tab() {
    let mut app = driver::open("reopening_a_sketch_puts_the_ribbon_on_its_tab");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::finish_the_sketch(&mut app);
    turn_the_ribbon_to(&mut app, "Extrusion");

    driver::double_click(&mut app, "Esquisse 1");

    assert!(
        driver::a_sketch_is_open(&app),
        "a double-click on « Esquisse 1 » did not reopen it: {:?}",
        driver::on_screen(&app)
    );
    assert_eq!(
        the_tab_shown(&app),
        ["Esquisse"],
        "the sketch reopened with the ribbon left on another tab"
    );
    assert!(
        the_sketch_tools_are_in_reach(&app),
        "the ribbon shows « Esquisse » without its tools: {:?}",
        driver::on_screen(&app)
    );
}

#[test]
fn starting_a_sketch_by_its_shortcut_puts_the_ribbon_on_its_tab() {
    let mut app = driver::open("starting_a_sketch_by_its_shortcut_puts_the_ribbon_on_its_tab");
    driver::create_a_part(&mut app, "Platine");
    turn_the_ribbon_to(&mut app, "Extrusion");

    driver::press_with_command(&mut app, egui::Key::N);
    driver::click_at_world_point(&mut app, driver::ON_THE_YZ_PLANE);

    assert!(
        driver::a_sketch_is_open(&app),
        "⌘N then a click on the plane YZ opened no sketch: {:?}",
        driver::on_screen(&app)
    );
    assert_eq!(
        the_tab_shown(&app),
        ["Esquisse"],
        "the new sketch opened with the ribbon left on another tab"
    );
    assert!(
        the_sketch_tools_are_in_reach(&app),
        "the ribbon shows « Esquisse » without its tools: {:?}",
        driver::on_screen(&app)
    );
}

type Finish = fn(&mut driver::App);

fn finish_with_the_ribbon(app: &mut driver::App) {
    driver::click_in_the_ribbon(app, "Terminer (Entrée)");
}

fn finish_with_enter(app: &mut driver::App) {
    driver::press(app, egui::Key::Enter);
}

/// What is wrong with the ribbon once a sketch, entered by ⌘N while it showed
/// « Export », is finished this way: nothing, when the sketch is closed and
/// « Export » is the tab shown again.
fn what_finishing_left_wrong(test: &str, finish: Finish) -> Option<String> {
    let mut app = driver::open_held_with(test, driver::the_moved_commands_put_back());
    driver::create_a_part(&mut app, "Platine");
    turn_the_ribbon_to(&mut app, "Export");
    driver::press_with_command(&mut app, egui::Key::N);
    driver::click_at_world_point(&mut app, driver::ON_THE_YZ_PLANE);
    driver::draw_a_segment(&mut app);
    assert!(
        driver::a_sketch_is_open(&app),
        "⌘N then a click on the plane YZ opened no sketch: {:?}",
        driver::on_screen(&app)
    );

    finish(&mut app);

    if driver::a_sketch_is_open(&app) {
        return Some("the sketch is still open".to_owned());
    }
    let shown = the_tab_shown(&app);
    (shown != ["Export"]).then(|| format!("the ribbon shows {shown:?}"))
}

#[test]
fn finishing_a_sketch_brings_back_the_tab_it_was_entered_from() {
    let finishes: [(&str, Finish); 3] = [
        ("the pill's « Terminer ↵ »", driver::finish_the_sketch),
        ("the ribbon's « Terminer »", finish_with_the_ribbon),
        ("↵", finish_with_enter),
    ];

    let wrong: Vec<(&str, String)> = finishes
        .into_iter()
        .enumerate()
        .filter_map(|(rank, (by, finish))| {
            let test = format!("finishing_a_sketch_brings_back_the_tab_{rank}");
            what_finishing_left_wrong(&test, finish).map(|wrong| (by, wrong))
        })
        .collect();

    assert!(
        wrong.is_empty(),
        "a sketch entered from « Export » and finished was not left on « Export »: {wrong:?}"
    );
}

fn assert_the_tab_shown(app: &driver::App, tab: &str, when: &str) {
    assert_eq!(
        the_tab_shown(app),
        [tab],
        "{when}, the ribbon does not show « {tab} »: {:?}",
        driver::on_screen(app)
    );
}

fn start_a_sketch_by_its_shortcut(app: &mut driver::App) {
    driver::press_with_command(app, egui::Key::N);
    driver::click_at_world_point(app, driver::ON_THE_YZ_PLANE);
    assert!(
        driver::a_sketch_is_open(app),
        "⌘N then a click on the plane YZ opened no sketch: {:?}",
        driver::on_screen(app)
    );
}

#[test]
fn a_sketch_started_from_its_button_lands_on_its_tab() {
    let mut app = driver::open("a_sketch_started_from_its_button_lands_on_its_tab");
    driver::create_a_part(&mut app, "Platine");

    driver::click_in_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, driver::ON_THE_YZ_PLANE);

    assert!(
        driver::a_sketch_is_open(&app),
        "« Nouvelle esquisse » then a click on the plane YZ opened no sketch: {:?}",
        driver::on_screen(&app)
    );
    assert_the_tab_shown(&app, "Esquisse", "a sketch started from its button");
    assert!(
        the_sketch_tools_are_in_reach(&app),
        "the ribbon shows « Esquisse » without its tools: {:?}",
        driver::on_screen(&app)
    );
}

#[test]
fn a_tab_picked_by_hand_while_sketching_is_kept_through_the_edit() {
    let mut app = driver::open("a_tab_picked_by_hand_while_sketching_is_kept");
    driver::create_a_part(&mut app, "Platine");
    turn_the_ribbon_to(&mut app, "Export");
    start_a_sketch_by_its_shortcut(&mut app);
    turn_the_ribbon_to(&mut app, "Extrusion");

    driver::press(&mut app, egui::Key::L);
    driver::click_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at_sketch_point(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::press(&mut app, egui::Key::Escape);

    assert!(
        driver::in_the_tree(&app, "Trait 1"),
        "L and two clicks drew no trait, so nothing was edited: {:?}",
        driver::on_screen(&app)
    );
    assert_the_tab_shown(
        &app,
        "Extrusion",
        "a trait drawn after « Extrusion » was picked",
    );

    driver::finish_the_sketch(&mut app);

    assert_the_tab_shown(
        &app,
        "Export",
        "a sketch entered from « Export », « Extrusion » picked during the edit, then finished",
    );
}

#[test]
fn reopening_another_sketch_while_one_is_open_keeps_the_tab_from_before_the_first() {
    let mut app = driver::open("reopening_another_sketch_while_one_is_open");
    driver::create_a_part(&mut app, "Platine");
    for _ in 0..2 {
        driver::bring_into_the_ribbon(&mut app, "Nouvelle esquisse");
        driver::start_a_sketch(&mut app);
        driver::draw_a_segment(&mut app);
        driver::finish_the_sketch(&mut app);
    }
    turn_the_ribbon_to(&mut app, "Export");
    driver::double_click(&mut app, "Esquisse 1");
    turn_the_ribbon_to(&mut app, "Extrusion");

    driver::double_click(&mut app, "Esquisse 2");
    assert_the_tab_shown(
        &app,
        "Esquisse",
        "« Esquisse 2 » reopened over « Esquisse 1 »",
    );
    driver::finish_the_sketch(&mut app);

    assert!(
        !driver::a_sketch_is_open(&app),
        "the pill's « Terminer » left a sketch open: {:?}",
        driver::on_screen(&app)
    );
    assert_the_tab_shown(
        &app,
        "Export",
        "« Esquisse 1 » reopened from « Export », « Esquisse 2 » over it from « Extrusion », \
         then finished",
    );
}

#[test]
fn a_sketch_closed_by_undo_leaves_the_tab_where_it_is() {
    let mut app = driver::open("a_sketch_closed_by_undo_leaves_the_tab_where_it_is");
    driver::create_a_part(&mut app, "Platine");
    turn_the_ribbon_to(&mut app, "Export");
    start_a_sketch_by_its_shortcut(&mut app);

    driver::press_with_command(&mut app, egui::Key::Z);

    assert!(
        !driver::a_sketch_is_open(&app),
        "⌘Z did not take back the sketch just started: {:?}",
        driver::on_screen(&app)
    );
    assert_the_tab_shown(&app, "Esquisse", "a sketch closed by ⌘Z");

    turn_the_ribbon_to(&mut app, "Extrusion");
    start_a_sketch_by_its_shortcut(&mut app);
    driver::finish_the_sketch(&mut app);

    assert_the_tab_shown(
        &app,
        "Extrusion",
        "a sketch taken back by ⌘Z, then another entered from « Extrusion » and finished",
    );
}

/// On the plane YZ, clear of the block's rectangle: where a hand hovers
/// while « Suppr » deletes the selection rather than what is under it.
const AWAY_FROM_THE_BLOCK_S_TRAITS: glam::DVec2 = glam::DVec2::new(173.9, -56.77);

#[test]
fn opening_an_extrusion_from_the_tree_while_sketching_still_lands_on_its_tab() {
    let mut app = driver::a_block("opening_an_extrusion_while_sketching");
    turn_the_ribbon_to(&mut app, "Export");
    driver::double_click(&mut app, "Esquisse 1");
    driver::click(&mut app, "Sélection (S)");
    driver::press_with_command(&mut app, egui::Key::A);
    app.hover_at(driver::sketch_point_on_screen(
        &app,
        AWAY_FROM_THE_BLOCK_S_TRAITS,
    ));
    app.run();
    driver::press(&mut app, egui::Key::Delete);

    driver::click_the_button(&mut app, "1 alerte");
    driver::click_the_button(&mut app, "Modifier");

    assert!(
        !driver::a_sketch_is_open(&app) && driver::says(&app, "Hauteur :"),
        "« Modifier » on the block's alert did not trade the sketch for its extrusion: {:?}",
        driver::on_screen(&app)
    );
    assert_the_tab_shown(
        &app,
        "Extrusion",
        "the block's extrusion opened over its sketch",
    );

    driver::double_click(&mut app, "Esquisse 1");
    driver::finish_the_sketch(&mut app);

    assert_the_tab_shown(
        &app,
        "Extrusion",
        "the sketch left for its extrusion, then entered again from « Extrusion » and finished",
    );
}
