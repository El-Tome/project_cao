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
//!   only on entering and on leaving, never during the edit —
//!   `a_tab_picked_by_hand_while_sketching_is_kept_through_the_edit`
//! - decided without the user: only « Terminer » brings the previous tab
//!   back. Opening an extrusion from the tree while sketching lands on the
//!   Extrusion tab, as it did —
//!   `opening_an_extrusion_from_the_tree_while_sketching_still_lands_on_its_tab`;
//!   a sketch closed by undo or redo leaves the tab where it is —
//!   `a_sketch_closed_by_undo_leaves_the_tab_where_it_is`
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
