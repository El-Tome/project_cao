//! Select-all, driven whole in the application: Cmd+A pressed in a sketch, and
//! what it took read back from the step Delete then writes in the history —
//! the step's detail counts the traits, the dimensions and the rules apart.
//!
//! Closes #703.
//! - A new command, « Tout sélectionner », with the shortcut Cmd+A on macOS and
//!   Ctrl+A elsewhere, in the palette and the shortcut list like any other
//!   command (so it can be rebound in the preferences, and put on the toolbar)
//!   — `select_all_is_listed_among_the_shortcuts_as_tout_selectionner_on_cmd_a`,
//!   `select_all_is_offered_in_the_toolbar_palette`,
//!   `a_rebound_select_all_takes_everything_from_its_new_key`
//! - Ctrl+A rather than Cmd+A off macOS — no test: held by
//!   `every_default_chord_fires_its_own_command_and_no_other`, in
//!   `crates/app/src/shortcuts/tests.rs`, which presses every default chord
//!   both as a Mac and as a PC sends it
//! - In a sketch with the Selection tool, it selects every element of the
//!   active sketch: traits, circles, arcs, ellipses and points, construction
//!   included — `command_a_in_the_selection_tool_takes_one_of_every_kind_construction_included`,
//!   which also holds the origin out, as decided on the issue
//! - Dimensions and rules are not selected: they are not moved —
//!   `command_a_leaves_the_dimensions_and_the_rules_out`
//! - No corner cut is selected, as a box takes none (decided on the issue) —
//!   `command_a_leaves_a_fillet_s_cut_out`
//! - What was selected before is replaced —
//!   `command_a_lets_go_of_what_was_held_before`
//! - An empty sketch selects nothing and says nothing —
//!   `in_an_empty_sketch_command_a_takes_nothing_and_says_nothing`
//! - With another tool, or outside a sketch, the command does nothing and the
//!   text fields keep their own Cmd+A / Ctrl+A —
//!   `with_another_tool_in_hand_command_a_leaves_the_tool_at_its_work`,
//!   `outside_a_sketch_command_a_changes_nothing`,
//!   `a_field_being_edited_keeps_command_a_for_its_own_text`, and the command
//!   greyed out there, in `select_all_is_offered_only_in_a_sketch_with_the_selection_tool`
//!   in `crates/app/src/screens/ribbon/state/tests.rs`
//! - A box selection, a click and Shift+click selection, and dragging a
//!   selection, must not break — `a_shift_click_still_adds_to_what_a_click_took`,
//!   `a_drag_on_a_trait_carries_everything_command_a_took`;
//!   the rest held by `a_box_over_the_whole_drawing_catches_one_of_every_kind`
//!   in `crates/sketch/src/banding/tests.rs`,
//!   `the_box_drawn_on_the_canvas_starts_and_ends_under_the_hand` in
//!   `pulling_a_box.rs`, `a_click_10_px_from_a_point_takes_it` in
//!   `selecting_under_the_hand.rs`, and
//!   `a_side_pressed_and_pulled_moves_it_in_one_step` in `pulling_a_side.rs`
//! - The shortcut of a text field being edited (a formula, a name) still
//!   selects its text — `a_field_being_edited_keeps_command_a_for_its_own_text`
//! - The shortcut with several modifiers fires its own command (#639) —
//!   `command_shift_a_takes_nothing`, and every default chord against every
//!   other in `every_default_chord_fires_its_own_command_and_no_other`, in
//!   `crates/app/src/shortcuts/tests.rs`

mod driver;

use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec2;

/// The magnets at their defaults, the grid's aside: every click lands where it
/// is aimed, so no two drawings of the sketch share a point by accident.
fn a_grid_that_pulls_nothing() -> cao_prefs::Settings {
    let mut settings = cao_prefs::Settings::default();
    settings.viewport.grid_snap = false;
    settings
}

fn a_sketch(test: &str) -> driver::App {
    let mut app = driver::open_held_with(test, a_grid_that_pulls_nothing());
    driver::create_a_part(&mut app, "Tout");
    driver::start_a_sketch(&mut app);
    app
}

fn in_the_selection_tool(app: &mut driver::App) {
    driver::press(app, egui::Key::Escape);
    driver::click(app, "Sélection (S)");
}

fn select_all(app: &mut driver::App) {
    driver::press_with_command(app, egui::Key::A);
}

/// Delete pressed, then the step it wrote read in detail, which for an
/// erasure counts the traits, the dimensions and the rules taken away apart.
fn what_delete_takes(app: &mut driver::App) -> String {
    delete_over_the_drawing(app);
    the_last_step_in_detail(app)
}

/// The last step of the history read in detail — the line the history shows
/// when the hand rests on it.
fn the_last_step_in_detail(app: &mut driver::App) -> String {
    driver::open_the_history(app);
    let last = driver::steps(app)
        .last()
        .cloned()
        .expect("the sketch's own step at least");
    let at = app.get_by_label(&last).rect().center();
    app.hover_at(at);
    for _ in 0..10 {
        app.step();
    }
    app.run();
    driver::on_screen(app)
        .into_iter()
        .chain(driver::said(app))
        .find(|line| line.starts_with("Esquisse 0 ·"))
        .unwrap_or(last)
}

/// Delete is read only while the pointer is over the drawing, so the pointer
/// is first brought to a spot every drawing of this file leaves empty.
fn delete_over_the_drawing(app: &mut driver::App) {
    let empty = driver::sketch_point_on_screen(app, DVec2::new(212.26, -65.95));
    app.hover_at(empty);
    app.run();
    driver::press(app, egui::Key::Delete);
}

/// Delete pressed, then every step of the history: with nothing held, Delete
/// writes none.
fn steps_after_delete(app: &mut driver::App) -> Vec<String> {
    delete_over_the_drawing(app);
    driver::open_the_history(app);
    driver::steps(app)
}

fn erased(traits: usize) -> String {
    format!("Esquisse 0 · {traits} tracé(s), 0 cote(s), 0 contrainte(s)")
}

/// Every point of this file is a point of the sketch: where the clicks the
/// tests were first written with landed, or, for a drawing that stood at the
/// top of the view, that drawing brought down whole by a step or two of 22.11
/// mm, below the pill a sketch shows there (#727). Only those: brought down,
/// a drawing low on the plane would come within reach of its axis, whose
/// magnet pulls 28 pixels, some 15 mm, and lay its points otherwise.
fn a_line(app: &mut driver::App, from: DVec2, to: DVec2) {
    driver::click(app, "Ligne (L)");
    driver::click_at_sketch_point(app, from);
    driver::click_at_sketch_point(app, to);
    driver::press(app, egui::Key::Escape);
}

/// A point, a trait, a circle, an arc, an ellipse and a construction trait,
/// apart from one another: 1 + 3 + 3 + 4 + 8 + 3 elements, ends and centres
/// counted, the ellipse with the two axes it is drawn with. A box pulled over
/// all of it takes those 22 and 2 rules besides.
fn one_of_every_kind(app: &mut driver::App) {
    driver::click(app, "Point (P)");
    driver::click_at_sketch_point(app, DVec2::new(190.15, 66.69));
    driver::press(app, egui::Key::Escape);

    a_line(app, DVec2::new(17.71, 88.8), DVec2::new(61.93, 75.53));

    driver::click_exactly(app, "Cercle (C)");
    driver::click_at_sketch_point(app, DVec2::new(101.72, 84.38));
    driver::click_at_sketch_point(app, DVec2::new(119.41, 84.38));
    driver::press(app, egui::Key::Escape);

    driver::click_exactly(app, "Arc");
    driver::click_at_sketch_point(app, DVec2::new(190.15, 22.48));
    driver::click_at_sketch_point(app, DVec2::new(212.26, 22.48));
    driver::click_at_sketch_point(app, DVec2::new(190.15, 44.59));
    driver::press(app, egui::Key::Escape);

    driver::click_exactly(app, "Ellipse");
    driver::click_at_sketch_point(app, DVec2::new(123.83, 13.64));
    driver::click_at_sketch_point(app, DVec2::new(168.04, -18.64));
    let on_the_ellipse = driver::sketch_point_on_screen(app, DVec2::new(132.67, -9.8));
    app.hover_at(on_the_ellipse);
    app.run();
    driver::click_at_sketch_point(app, DVec2::new(132.67, -9.8));
    driver::press(app, egui::Key::Escape);

    driver::click(app, "Construction");
    a_line(app, DVec2::new(17.71, -52.69), DVec2::new(61.93, -61.53));
    driver::click(app, "Construction");
}

const EVERY_ELEMENT_OF_ONE_OF_EVERY_KIND: usize = 22;

/// Corners at (57.5, 66.69) and (168.04, 22.47) of the sketch, its right
/// angles held by rules: four points and four sides. Low enough that what the
/// tests aim above its top side stays clear of the sketch's pill too.
fn a_rectangle(app: &mut driver::App) {
    driver::click(app, "Rectangle (R)");
    driver::click_at_sketch_point(app, DVec2::new(57.5, 66.69));
    driver::click_at_sketch_point(app, DVec2::new(168.04, 22.47));
    driver::press(app, egui::Key::Escape);
}

#[test]
fn command_a_in_the_selection_tool_takes_one_of_every_kind_construction_included() {
    let mut app = a_sketch("command_a_takes_one_of_every_kind");
    one_of_every_kind(&mut app);
    in_the_selection_tool(&mut app);

    select_all(&mut app);

    assert_eq!(
        what_delete_takes(&mut app),
        erased(EVERY_ELEMENT_OF_ONE_OF_EVERY_KIND),
        "Cmd+A takes every element of the sketch, construction included, the \
         origin and the rules left out",
    );
}

#[test]
fn command_a_leaves_the_dimensions_and_the_rules_out() {
    let mut app = a_sketch("command_a_leaves_the_dimensions_and_the_rules_out");
    a_rectangle(&mut app);
    a_line(
        &mut app,
        DVec2::new(-8.82, 44.59),
        DVec2::new(123.83, -52.69),
    );
    driver::click(&mut app, "Cote");
    driver::click_at_sketch_point(&mut app, DVec2::new(57.5, -4.05));
    driver::click_at_sketch_point(&mut app, DVec2::new(35.39, -35.0));
    driver::press(&mut app, egui::Key::Enter);
    in_the_selection_tool(&mut app);

    select_all(&mut app);

    assert_eq!(
        what_delete_takes(&mut app),
        erased(8 + 3),
        "the rectangle's and the segment's points and traits are taken; its right \
         angles and the segment's dimension are not",
    );
}

#[test]
fn command_a_leaves_a_fillet_s_cut_out() {
    let mut app = a_sketch("command_a_leaves_a_fillet_s_cut_out");
    a_rectangle(&mut app);
    driver::click(&mut app, "Congé");
    driver::click_at_sketch_point(&mut app, DVec2::new(168.04, 66.69));
    driver::key(&mut app, egui::Key::Num5, "5");
    driver::press(&mut app, egui::Key::Enter);
    in_the_selection_tool(&mut app);

    select_all(&mut app);

    assert_eq!(
        what_delete_takes(&mut app),
        erased(8),
        "the rectangle is taken and its rounded corner is not: Delete erases the \
         elements in one step, and takes no cut off first",
    );
}

#[test]
fn command_a_lets_go_of_what_was_held_before() {
    let mut app = a_sketch("command_a_lets_go_of_what_was_held_before");
    a_rectangle(&mut app);
    in_the_selection_tool(&mut app);
    app.run();
    let (_, right_angle, _) = driver::right_angle_marks(&app)
        .into_iter()
        .next()
        .expect("the rectangle's right angles are marked");
    driver::click_at(&mut app, right_angle);

    select_all(&mut app);

    assert_eq!(
        what_delete_takes(&mut app),
        erased(8),
        "the right angle clicked first is let go: Cmd+A replaces what was held",
    );
}

#[test]
fn in_an_empty_sketch_command_a_takes_nothing_and_says_nothing() {
    let mut app = a_sketch("in_an_empty_sketch_command_a_takes_nothing");
    in_the_selection_tool(&mut app);
    let said_before = driver::said(&app);

    select_all(&mut app);

    assert_eq!(driver::said(&app), said_before, "Cmd+A says nothing");
    assert_eq!(
        steps_after_delete(&mut app),
        ["1. Esquisse — Plan YZ"],
        "nothing was taken, so Delete erases nothing",
    );
}

#[test]
fn with_another_tool_in_hand_command_a_leaves_the_tool_at_its_work() {
    let mut app = a_sketch("with_another_tool_command_a_leaves_the_tool");
    driver::click(&mut app, "Ligne (L)");
    driver::click_at_sketch_point(&mut app, DVec2::new(17.71, 88.8));

    select_all(&mut app);
    driver::click_at_sketch_point(&mut app, DVec2::new(61.93, 75.53));
    driver::press(&mut app, egui::Key::Escape);

    assert_eq!(
        steps_after_delete(&mut app),
        ["1. Esquisse — Plan YZ", "2. Trait"],
        "the line started before Cmd+A is finished by the next click, and nothing \
         is taken for Delete to erase",
    );
}

#[test]
fn outside_a_sketch_command_a_changes_nothing() {
    let mut app = a_sketch("outside_a_sketch_command_a_changes_nothing");
    driver::draw_a_segment(&mut app);
    driver::finish_the_sketch(&mut app);
    driver::open_the_history(&mut app);
    let (steps_before, said_before) = (driver::steps(&app), driver::said(&app));

    select_all(&mut app);

    assert_eq!(driver::steps(&app), steps_before);
    assert_eq!(driver::said(&app), said_before);
}

#[test]
fn a_field_being_edited_keeps_command_a_for_its_own_text() {
    let mut app = a_sketch("a_field_being_edited_keeps_command_a");
    a_line(&mut app, DVec2::new(17.71, 88.8), DVec2::new(61.93, 75.53));
    in_the_selection_tool(&mut app);
    driver::click_the_button(&mut app, "Variables");
    driver::type_into(&mut app, "nom", "width");
    driver::type_into(&mut app, "formule", "120");
    driver::click_exactly(&mut app, "Ajouter");

    driver::type_over(&mut app, "120", "200");
    driver::type_over(&mut app, "width", "depth");

    let fields = driver::fields(&app);
    assert!(
        fields.contains(&"200".to_string()) && fields.contains(&"depth".to_string()),
        "Cmd+A in a formula and in a name selects what the field holds, which \
         the typing then replaces: {fields:?}",
    );
}

#[test]
fn command_shift_a_takes_nothing() {
    let mut app = a_sketch("command_shift_a_takes_nothing");
    a_line(&mut app, DVec2::new(17.71, 88.8), DVec2::new(61.93, 75.53));
    in_the_selection_tool(&mut app);

    driver::press_with(
        &mut app,
        egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        egui::Key::A,
    );

    assert_eq!(
        steps_after_delete(&mut app),
        ["1. Esquisse — Plan YZ", "2. Trait"],
        "Cmd+Shift+A is not Cmd+A, and takes nothing for Delete to erase",
    );
}

#[test]
fn a_shift_click_still_adds_to_what_a_click_took() {
    let mut app = a_sketch("a_shift_click_still_adds");
    a_rectangle(&mut app);
    in_the_selection_tool(&mut app);

    driver::click_at_sketch_point(&mut app, DVec2::new(112.77, 66.69));
    let on_the_bottom_side = driver::sketch_point_on_screen(&app, DVec2::new(112.77, 22.47));
    driver::click_with(&mut app, on_the_bottom_side, egui::Modifiers::SHIFT);

    assert_eq!(
        what_delete_takes(&mut app),
        "Esquisse 0 · 2 tracé(s), 0 cote(s), 0 contrainte(s)",
        "the top side clicked, then the bottom one Shift+clicked, are both held",
    );
}

#[test]
fn a_drag_on_a_trait_carries_everything_command_a_took() {
    let mut app = a_sketch("a_drag_carries_everything_command_a_took");
    a_rectangle(&mut app);
    a_line(&mut app, DVec2::new(17.71, 22.48), DVec2::new(44.24, 0.37));
    in_the_selection_tool(&mut app);

    select_all(&mut app);
    let (on_the_top_side, above_it) = (
        driver::sketch_point_on_screen(&app, DVec2::new(112.77, 66.69)),
        driver::sketch_point_on_screen(&app, DVec2::new(112.77, 88.8)),
    );
    driver::drag_and_drop(&mut app, on_the_top_side, above_it);

    let moved = the_last_step_in_detail(&mut app);
    assert!(
        moved.starts_with("Esquisse 0 · 6 points de"),
        "the rectangle's four corners and the line's two ends moved together, in \
         one step: {moved}",
    );
}

#[test]
fn select_all_is_listed_among_the_shortcuts_as_tout_selectionner_on_cmd_a() {
    let mut app = a_sketch("select_all_is_listed_among_the_shortcuts");
    the_shortcuts_in_the_preferences(&mut app);

    let row = the_shortcut_of(&app, "Tout sélectionner");

    assert_eq!(row.as_deref(), Some("Cmd+A"));
}

#[test]
fn select_all_is_offered_in_the_toolbar_palette() {
    let mut app = a_sketch("select_all_is_offered_in_the_toolbar_palette");
    driver::press_with_command(&mut app, egui::Key::Comma);
    driver::click_exactly(&mut app, "Barre d'outils");

    let offered = app
        .query_all_by_label("Tout sélectionner")
        .any(|node| node.accesskit_node().role() == egui::accesskit::Role::Button);

    assert!(
        offered,
        "the palette offers « Tout sélectionner » to put on the toolbar: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_rebound_select_all_takes_everything_from_its_new_key() {
    let mut app = a_sketch("a_rebound_select_all_takes_everything");
    a_line(&mut app, DVec2::new(17.71, 88.8), DVec2::new(61.93, 75.53));
    in_the_selection_tool(&mut app);
    the_shortcuts_in_the_preferences(&mut app);
    let chord = app
        .query_by_label("Cmd+A")
        .expect("Cmd+A is bound to a command the list shows");
    chord.scroll_to_me();
    chord.click_accesskit();
    app.run();
    driver::press(&mut app, egui::Key::Q);
    assert_eq!(
        the_shortcut_of(&app, "Tout sélectionner").as_deref(),
        Some("Q"),
        "Q now selects everything",
    );

    driver::press(&mut app, egui::Key::Q);
    driver::click_exactly(&mut app, "Close window");
    delete_over_the_drawing(&mut app);
    driver::open_the_history(&mut app);

    assert_eq!(
        driver::steps(&app).last().map(String::as_str),
        Some("3. 3 éléments supprimés"),
        "Q took the line's two ends and the line: {:?}",
        driver::steps(&app),
    );
}

fn the_shortcuts_in_the_preferences(app: &mut driver::App) {
    driver::press_with_command(app, egui::Key::Comma);
    driver::click_exactly(app, "Raccourcis");
}

/// What the shortcut list shows beside a command's name: the chord its button
/// reads, between the name and the button that takes the chord away. The name
/// alone does not find the row, which the ribbon and the history may share.
fn the_shortcut_of(app: &driver::App, command: &str) -> Option<String> {
    let buttons: Vec<(egui::Rect, String)> = app
        .get_all_by_role(egui::accesskit::Role::Button)
        .filter_map(|node| Some((node.rect(), node.accesskit_node().label()?)))
        .collect();
    let on_the_row = |name: egui::Rect, rect: egui::Rect| {
        (rect.center().y - name.center().y).abs() < 4.0 && rect.left() >= name.right()
    };
    app.query_all_by(|node| {
        node.value().as_deref() == Some(command) || node.label().as_deref() == Some(command)
    })
    .find_map(|name| {
        let name = name.rect();
        let unbind = buttons
            .iter()
            .filter(|(rect, label)| label == "Retirer" && on_the_row(name, *rect))
            .map(|(rect, _)| rect.left())
            .min_by(f32::total_cmp)?;
        buttons
            .iter()
            .find(|(rect, label)| {
                label != "Retirer" && on_the_row(name, *rect) && rect.right() <= unbind
            })
            .map(|(_, label)| label.clone())
    })
}
