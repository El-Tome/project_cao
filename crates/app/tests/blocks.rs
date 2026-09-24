//! A variable's name in a formula is a block, driven with no window.
//!
//! Closes #437.
//! - a whole variable name in a formula field is shown as a block, on a
//!   tinted background, whether it was typed, completed or dropped — no
//!   test: the tint is painted, and the driver's tree carries no colour; the
//!   layout that lays it, and which names are blocks, are held by
//!   `ui/formula_field/blocks/tests.rs`
//! - Retour arrière just after a block, or Suppr just before it, erases it
//!   whole — `retour_arriere_right_after_a_name_erases_it_whole`,
//!   `suppr_right_before_a_name_erases_it_whole`,
//!   `a_name_the_list_put_in_is_a_block_too`
//! - ← and → step over a block in one press, and a click inside one puts the
//!   cursor at its nearer edge — `the_arrows_step_over_a_name`,
//!   `a_click_inside_a_name_puts_the_cursor_at_one_of_its_edges`
//! - a name still being typed, or one no variable goes by, is plain text;
//!   numbers and operators behave as today —
//!   `a_name_still_being_typed_is_erased_letter_by_letter`,
//!   `a_name_no_variable_goes_by_is_erased_letter_by_letter`

mod driver;

use egui_kittest::kittest::{NodeT, Queryable};

/// A part holding `width` at 120 and `wall` at 5, made in the panel, which
/// is closed again.
fn a_part_with_two_variables(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    driver::click_the_button(&mut app, "Variables");
    for (name, formula) in [("width", "120"), ("wall", "5")] {
        driver::type_into(&mut app, "nom", name);
        driver::type_into(&mut app, "formule", formula);
        driver::click_exactly(&mut app, "Ajouter");
    }
    driver::click_the_button(&mut app, "Variables");
    app
}

/// The same, a line begun in a sketch and this typed into its length.
fn a_length_typed(test: &str, text: &str) -> driver::App {
    let mut app = a_part_with_two_variables(test);
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);
    app.event(egui::Event::Text(text.to_string()));
    app.run();
    app
}

fn the_length(app: &driver::App) -> String {
    driver::fields(app)
        .into_iter()
        .next()
        .expect("the line's length, first of its fields")
}

#[test]
fn retour_arriere_right_after_a_name_erases_it_whole() {
    let mut app = a_length_typed("retour_arriere_erases_a_name", "2*width");

    driver::press(&mut app, egui::Key::Backspace);

    assert_eq!(the_length(&app), "2*");
}

#[test]
fn suppr_right_before_a_name_erases_it_whole() {
    let mut app = a_length_typed("suppr_erases_a_name", "width+1");
    driver::press(&mut app, egui::Key::Home);

    driver::press(&mut app, egui::Key::Delete);

    assert_eq!(the_length(&app), "+1");
}

#[test]
fn a_name_the_list_put_in_is_a_block_too() {
    let mut app = a_length_typed("a_completed_name_is_a_block", "wi");
    driver::press(&mut app, egui::Key::Enter);
    assert_eq!(the_length(&app), "width");

    driver::press(&mut app, egui::Key::Backspace);

    assert_eq!(the_length(&app), "");
}

#[test]
fn the_arrows_step_over_a_name() {
    let mut app = a_length_typed("the_arrows_step_over_a_name", "width+1");
    driver::press(&mut app, egui::Key::ArrowLeft);
    driver::press(&mut app, egui::Key::ArrowLeft);

    driver::press(&mut app, egui::Key::ArrowLeft);
    app.event(egui::Event::Text("2*".to_string()));
    app.run();

    assert_eq!(
        the_length(&app),
        "2*width+1",
        "the third step crossed width whole, to its start"
    );
}

#[test]
fn a_click_inside_a_name_puts_the_cursor_at_one_of_its_edges() {
    let mut app = a_part_with_two_variables("a_click_inside_a_name");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::click(&mut app, "Cote");
    driver::click_at(&mut app, (850.0, 510.0));
    driver::click_at(&mut app, (900.0, 440.0));
    let measured = driver::fields(&app)
        .pop()
        .expect("the dimension's field, holding what the trait measures");
    driver::type_over(&mut app, &measured, "width");
    let field = app
        .query_all_by_role(egui::accesskit::Role::TextInput)
        .find(|node| node.accesskit_node().value().as_deref() == Some("width"))
        .expect("the dimension's field, holding width")
        .rect();

    driver::click_at(&mut app, (field.left() + 18.0, field.center().y));
    app.event(egui::Event::Text("X".to_string()));
    app.run();

    let held = driver::fields(&app).pop().expect("the dimension's field");
    assert!(
        held == "Xwidth" || held == "widthX",
        "nothing is written in the middle of a name: {held}"
    );
}

#[test]
fn a_name_still_being_typed_is_erased_letter_by_letter() {
    let mut app = a_length_typed("a_name_being_typed_is_text", "wid");

    driver::press(&mut app, egui::Key::Backspace);

    assert_eq!(the_length(&app), "wi");
}

#[test]
fn a_name_no_variable_goes_by_is_erased_letter_by_letter() {
    let mut app = a_length_typed("a_name_nobody_goes_by_is_text", "2*widths");

    driver::press(&mut app, egui::Key::Backspace);

    assert_eq!(
        the_length(&app),
        "2*width",
        "and what is left is a block again"
    );
}
