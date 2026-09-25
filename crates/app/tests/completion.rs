//! A variable's name completed from a list under the field it is typed in,
//! driven with no window.
//!
//! Closes #433.
//! - typing the start of a variable's name in a size field, or in the formula
//!   column of the panel, lists the variables whose name starts with it, case
//!   aside, each with its value —
//!   `the_start_of_a_name_at_the_cursor_lists_the_variables_it_begins`,
//!   `a_name_begun_in_capitals_lists_the_same_variables`,
//!   `the_start_of_a_name_in_the_extrusion_row_is_completed_too`,
//!   `the_start_of_a_name_in_the_panel_is_completed_too`
//! - Entrée or Tab puts the chosen name in place of what was typed, a click on
//!   one does too, and the arrows move the choice —
//!   `enter_puts_the_chosen_name_in_place_of_what_was_typed`,
//!   `tab_does_the_same`, `the_arrows_move_the_choice`,
//!   `a_click_on_a_name_in_a_dimensions_field_puts_it_in`; the click where
//!   the field stays put: the fields at the cursor follow the pointer, and
//!   their list with them
//! - with the name in, the next Entrée validates the field as today; Échap
//!   closes the list and nothing else —
//!   `the_next_enter_lays_the_line_to_the_name`,
//!   `escape_closes_the_list_and_keeps_the_field`
//! - a number typed then Entrée behaves exactly as today — no test: the runs
//!   of `variables.rs` and `drawing.rs` type numbers and press Entrée, and
//!   pass untouched

mod driver;

use egui_kittest::kittest::Queryable;

/// A part holding `width` at 120, `wall` at 5 and `height` at 40, made in
/// the panel, which is closed again.
fn a_part_with_three_variables(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    driver::click_the_button(&mut app, "Variables");
    for (name, formula) in [("width", "120"), ("wall", "5"), ("height", "40")] {
        driver::type_into(&mut app, "nom", name);
        driver::type_into(&mut app, "formule", formula);
        driver::click_exactly(&mut app, "Ajouter");
    }
    driver::click_the_button(&mut app, "Variables");
    app
}

/// The same, with a line begun in a sketch.
fn a_line_begun(test: &str) -> driver::App {
    let mut app = a_part_with_three_variables(test);
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);
    app
}

fn typed(app: &mut driver::App, letters: &str) {
    for letter in letters.chars() {
        let key =
            egui::Key::from_name(&letter.to_ascii_uppercase().to_string()).expect("a letter key");
        driver::key(app, key, &letter.to_string());
    }
}

const WIDTH: &str = "width   120";
const WALL: &str = "wall   5";
const HEIGHT: &str = "height   40";

#[test]
fn the_start_of_a_name_at_the_cursor_lists_the_variables_it_begins() {
    let mut app = a_line_begun("the_start_of_a_name_lists_the_variables");

    typed(&mut app, "w");

    assert!(
        driver::shows(&app, WIDTH) && driver::shows(&app, WALL),
        "width and wall, each with its value: {:?}",
        driver::on_screen(&app),
    );
    assert!(!driver::shows(&app, HEIGHT), "and not height");
}

#[test]
fn a_name_begun_in_capitals_lists_the_same_variables() {
    let mut app = a_line_begun("a_name_begun_in_capitals");

    typed(&mut app, "WI");

    assert!(
        driver::shows(&app, WIDTH) && !driver::shows(&app, WALL),
        "{:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn enter_puts_the_chosen_name_in_place_of_what_was_typed() {
    let mut app = a_line_begun("enter_puts_the_chosen_name_in");
    typed(&mut app, "wi");

    driver::press(&mut app, egui::Key::Enter);

    assert!(
        driver::fields(&app).contains(&"width".to_string()),
        "{:?}",
        driver::fields(&app),
    );
    assert!(!driver::shows(&app, WIDTH), "and the list is closed");
}

#[test]
fn the_next_enter_lays_the_line_to_the_name() {
    let mut app = a_line_begun("the_next_enter_lays_the_line");
    typed(&mut app, "wi");
    driver::press(&mut app, egui::Key::Enter);

    driver::press(&mut app, egui::Key::Enter);

    driver::click_the_button(&mut app, "Variables");
    app.get_all_by_label("Retirer")
        .next()
        .expect("the row of width comes first")
        .click();
    app.run();
    assert!(
        driver::says(&app, "sert encore à la cote « Cote 120 mm »"),
        "the line was laid to width, and its length is written from it",
    );
}

#[test]
fn tab_does_the_same() {
    let mut app = a_line_begun("tab_puts_the_chosen_name_in");
    typed(&mut app, "wi");

    driver::press(&mut app, egui::Key::Tab);

    assert!(
        driver::fields(&app).contains(&"width".to_string()),
        "{:?}",
        driver::fields(&app),
    );
}

#[test]
fn the_arrows_move_the_choice() {
    let mut app = a_line_begun("the_arrows_move_the_choice");
    typed(&mut app, "w");

    driver::press(&mut app, egui::Key::ArrowDown);
    driver::press(&mut app, egui::Key::Enter);

    assert!(
        driver::fields(&app).contains(&"wall".to_string()),
        "the second name, one step down: {:?}",
        driver::fields(&app),
    );
}

#[test]
fn escape_closes_the_list_and_keeps_the_field() {
    let mut app = a_line_begun("escape_closes_the_list");
    typed(&mut app, "w");

    driver::press(&mut app, egui::Key::Escape);

    assert!(!driver::shows(&app, WIDTH), "the list is closed");
    assert!(
        driver::fields(&app).contains(&"w".to_string()),
        "and the line is still being drawn, what was typed kept: {:?}",
        driver::fields(&app),
    );
}

#[test]
fn a_click_on_a_name_in_a_dimensions_field_puts_it_in() {
    let mut app = a_part_with_three_variables("a_click_on_a_name_puts_it_in");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::click(&mut app, "Cote");
    driver::click_at(&mut app, (850.0, 510.0));
    driver::click_at(&mut app, (900.0, 440.0));
    let measured = driver::fields(&app)
        .pop()
        .expect("the dimension's field, holding what the trait measures");
    driver::type_over(&mut app, &measured, "wi");

    driver::click(&mut app, WIDTH);

    assert!(
        driver::fields(&app).contains(&"width".to_string()),
        "{:?}",
        driver::fields(&app),
    );
}

#[test]
fn the_start_of_a_name_in_the_extrusion_row_is_completed_too() {
    let mut app = a_part_with_three_variables("the_extrusion_row_is_completed");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click(&mut app, "Terminer");
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at(&mut app, (850.0, 510.0));
    driver::type_over(&mut app, "10", "he");

    driver::press(&mut app, egui::Key::Enter);

    assert!(
        driver::fields(&app).contains(&"height".to_string()),
        "{:?}",
        driver::fields(&app),
    );
}

#[test]
fn the_start_of_a_name_in_the_panel_is_completed_too() {
    let mut app = a_part_with_three_variables("the_panel_is_completed");
    driver::click_the_button(&mut app, "Variables");
    driver::type_into(&mut app, "nom", "twice");
    driver::type_into(&mut app, "formule", "2 * he");

    driver::press(&mut app, egui::Key::Enter);

    assert!(
        driver::fields(&app).contains(&"2 * height".to_string()),
        "{:?}",
        driver::fields(&app),
    );
}
