//! A variable dragged from its panel into the value of a dimension, driven
//! with no window.
//!
//! Closes #434.
//! - a variable dragged from the panel and dropped on a dimension's field
//!   goes into it at the point of the drop, or in place of the selected text
//!   — or of a plain number, the value the field opened on, which reaching
//!   for the panel leaves unselected —
//!   `a_variable_dropped_on_a_dimensions_field_takes_the_place_of_the_value_it_opened_on`,
//!   `a_variable_dropped_into_a_calculation_goes_in_where_it_lands`; the
//!   selected text, which no drag from the panel keeps, by
//!   `ui/dropping/tests.rs`
//! - the field holds the keyboard after the drop, and Entrée applies the
//!   value as it reads —
//!   `after_the_drop_the_field_holds_the_keyboard_and_enter_applies_it`,
//!   `what_is_typed_after_the_drop_goes_on_from_the_name`
//! - dropped anywhere else, nothing changes —
//!   `a_variable_dropped_anywhere_else_changes_nothing`

mod driver;

use egui_kittest::kittest::{NodeT, Queryable};

/// A part holding `width` at 120 and `wall` at 5, a trait in a sketch, and
/// a dimension of it just placed with its tool, its field open on what the
/// trait measures — the panel of the variables opened after, beside it.
fn a_dimension_open_beside_the_variables(test: &str) -> (driver::App, String) {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    driver::click_the_button(&mut app, "Variables");
    for (name, formula) in [("width", "120"), ("wall", "5")] {
        driver::type_into(&mut app, "nom", name);
        driver::type_into(&mut app, "formule", formula);
        driver::click_exactly(&mut app, "Ajouter");
    }
    driver::click_the_button(&mut app, "Variables");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::click(&mut app, "Cote");
    driver::click_at(&mut app, (850.0, 510.0));
    driver::click_at(&mut app, (900.0, 440.0));
    let measured = driver::fields(&app)
        .pop()
        .expect("the dimension's field, holding what the trait measures");
    driver::click_the_button(&mut app, "Variables");
    (app, measured)
}

/// Where the grip of the first variable's row stands.
fn the_grip_of_width(app: &driver::App) -> egui::Pos2 {
    app.query_all_by_role(egui::accesskit::Role::Label)
        .find(|node| node.accesskit_node().value().as_deref() == Some("☰"))
        .expect("a grip on the row of width")
        .rect()
        .center()
}

/// What the field standing there holds.
fn held_at(app: &driver::App, place: egui::Rect) -> String {
    app.query_all_by_role(egui::accesskit::Role::TextInput)
        .find(|node| node.rect().contains(place.center()))
        .and_then(|node| node.accesskit_node().value())
        .unwrap_or_else(|| panic!("a field at {place:?}"))
}

/// Where the field holding this stands.
fn the_field_holding(app: &driver::App, holding: &str) -> egui::Rect {
    app.query_all_by_role(egui::accesskit::Role::TextInput)
        .find(|node| node.accesskit_node().value().as_deref() == Some(holding))
        .unwrap_or_else(|| panic!("a field holding {holding}"))
        .rect()
}

#[test]
fn a_variable_dropped_on_a_dimensions_field_takes_the_place_of_the_value_it_opened_on() {
    let (mut app, measured) = a_dimension_open_beside_the_variables("dropped_on_the_value");
    let (grip, field) = (the_grip_of_width(&app), the_field_holding(&app, &measured));

    driver::drag_and_drop(&mut app, grip, field.center());

    assert_eq!(held_at(&app, field), "width");
}

#[test]
fn a_variable_dropped_into_a_calculation_goes_in_where_it_lands() {
    let (mut app, measured) = a_dimension_open_beside_the_variables("dropped_where_it_lands");
    driver::type_over(&mut app, &measured, "2 * ");
    let (grip, field) = (the_grip_of_width(&app), the_field_holding(&app, "2 * "));

    let at_the_end = egui::pos2(field.right() - 4.0, field.center().y);
    driver::drag_and_drop(&mut app, grip, at_the_end);

    assert_eq!(held_at(&app, field), "2 * width");
}

#[test]
fn after_the_drop_the_field_holds_the_keyboard_and_enter_applies_it() {
    let (mut app, measured) = a_dimension_open_beside_the_variables("the_drop_brings_the_keyboard");
    let (grip, field) = (the_grip_of_width(&app), the_field_holding(&app, &measured));
    driver::drag_and_drop(&mut app, grip, field.center());

    driver::press(&mut app, egui::Key::Enter);

    app.get_all_by_label("Retirer")
        .next()
        .expect("the row of width comes first")
        .click();
    app.run();
    assert!(
        driver::says(&app, "sert encore à la cote « Cote 120 mm »"),
        "the trait was dimensioned to width, and its value is written from it",
    );
}

#[test]
fn what_is_typed_after_the_drop_goes_on_from_the_name() {
    let (mut app, measured) = a_dimension_open_beside_the_variables("typing_after_the_drop");
    let (grip, field) = (the_grip_of_width(&app), the_field_holding(&app, &measured));
    driver::drag_and_drop(&mut app, grip, field.center());

    app.event(egui::Event::Text(" / 2".to_string()));
    app.run();

    assert_eq!(
        held_at(&app, field),
        "width / 2",
        "the keyboard came back with the name, the cursor after it and \
         nothing selected"
    );
}

#[test]
fn a_variable_dropped_anywhere_else_changes_nothing() {
    let (mut app, measured) = a_dimension_open_beside_the_variables("dropped_anywhere_else");
    let (grip, field) = (the_grip_of_width(&app), the_field_holding(&app, &measured));

    driver::drag_and_drop(&mut app, grip, egui::pos2(1100.0, 300.0));

    assert_eq!(
        held_at(&app, field),
        measured,
        "the dimension's field still holds what the trait measures"
    );
}
