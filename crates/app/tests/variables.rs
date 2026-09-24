//! The part's variables, driven with no window: made in their panel, and
//! typed into the fields at the cursor.
//!
//! Closes #175.
//! - a panel opened from the ribbon, in a sketch and outside one, lists every
//!   variable as its name, its formula and its computed value; a row is
//!   added, edited or deleted there —
//!   `a_variable_is_made_in_its_panel_opened_from_the_ribbon`,
//!   `the_panel_is_there_inside_a_sketch_too`,
//!   `changing_a_variable_in_its_panel_raises_the_plate_again`,
//!   `a_variable_nothing_uses_is_retired_from_its_panel`,
//!   `escape_in_a_row_drops_what_was_typed_and_records_nothing`
//! - a formula that does not read is refused where it is typed, with a
//!   message saying what is wrong, and nothing is applied —
//!   `a_formula_that_does_not_read_is_refused_where_it_is_typed`
//! - a formula can be typed as an extrusion's distance —
//!   `an_extrusion_raised_by_a_formula_says_it_in_the_history`
//! - and as the value of a dimension the tool has just placed, which takes a
//!   plain number as it always did —
//!   `a_dimension_placed_with_its_tool_takes_the_number_typed_for_it`,
//!   `a_dimension_placed_with_its_tool_takes_a_formula_typed_for_it`
//! - and into the steps and counts of the patterns, whose fields show once
//!   `Entrée` has closed the selection rather than the sketch —
//!   `a_pattern_shows_its_values_while_the_centre_is_awaited`
//! - changing a variable changes every size written from it, and the part is
//!   rebuilt — `changing_a_variable_in_its_panel_raises_the_plate_again`
//! - in the fields at the cursor, letters stay shortcuts until a digit or `=`
//!   enters the field, which then keeps every key until `Entrée` or `Échap` —
//!   `a_letter_typed_before_anything_else_while_drawing_is_a_shortcut`,
//!   `an_equals_sign_opens_a_formula_that_keeps_every_key`,
//!   `a_digit_starts_the_field_the_way_it_always_did`,
//!   `a_name_typed_in_the_panel_while_a_line_is_half_drawn_goes_into_the_field`
//! - changing a variable is one step of the history: undo puts its former
//!   formula back, and the part with it —
//!   `undoing_a_change_to_a_variable_puts_the_plate_back`
//! - deleting a variable something uses is refused, with the list of what
//!   uses it — `a_variable_in_use_is_kept_and_the_panel_says_what_uses_it`
//! - a value typed as a plain number behaves exactly as today — no test: none
//!   of its own, since every test the workspace held before this branch
//!   builds its sizes as plain numbers and passes with its assertions
//!   untouched

mod driver;

/// A part holding `width` at 120, made in the panel — left open.
fn a_part_with_a_width(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Platine");
    add_a_width(&mut app);
    app
}

fn add_a_width(app: &mut driver::App) {
    driver::click_the_button(app, "Variables");
    driver::type_into(app, "nom", "width");
    driver::type_into(app, "formule", "120");
    driver::click_exactly(app, "Ajouter");
}

#[test]
fn a_variable_is_made_in_its_panel_opened_from_the_ribbon() {
    let app = a_part_with_a_width("a_variable_is_made_in_its_panel");

    let fields = driver::fields(&app);

    assert!(
        fields.contains(&"width".to_string()) && fields.contains(&"120".to_string()),
        "the new row holds its name and its formula: {fields:?}",
    );
}

#[test]
fn the_panel_is_there_inside_a_sketch_too() {
    let mut app = driver::open("the_panel_is_there_inside_a_sketch");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    assert!(driver::is_enabled(&app, "Terminer"), "a sketch is open");

    add_a_width(&mut app);

    assert!(
        driver::fields(&app).contains(&"width".to_string()),
        "a variable is made while drawing: {:?}",
        driver::fields(&app),
    );
}

#[test]
fn a_letter_typed_before_anything_else_while_drawing_is_a_shortcut() {
    let mut app = driver::open("a_letter_while_drawing_is_a_shortcut");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);
    assert!(
        driver::fields_at_the_cursor(&app) > 0,
        "the line shows its fields"
    );

    driver::key(&mut app, egui::Key::R, "r");

    assert_eq!(
        driver::fields_at_the_cursor(&app),
        0,
        "R took the rectangle tool rather than landing in the length: {:?}",
        driver::fields(&app),
    );
}

#[test]
fn an_equals_sign_opens_a_formula_that_keeps_every_key() {
    let mut app = a_part_with_a_width("an_equals_sign_opens_a_formula");
    // Out of the way of the drawing, which starts right of the panels.
    driver::click_the_button(&mut app, "Variables");
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);

    driver::key(&mut app, egui::Key::Equals, "=");
    for (key, text) in [
        (egui::Key::W, "w"),
        (egui::Key::I, "i"),
        (egui::Key::D, "d"),
    ] {
        driver::key(&mut app, key, text);
    }

    assert!(
        driver::fields(&app).contains(&"=wid".to_string()),
        "once started the field keeps the letters: {:?}",
        driver::fields(&app),
    );
}

#[test]
fn a_digit_starts_the_field_the_way_it_always_did() {
    let mut app = driver::open("a_digit_starts_the_field");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);

    driver::key(&mut app, egui::Key::Num4, "4");
    driver::key(&mut app, egui::Key::Num0, "0");

    assert!(
        driver::fields(&app).contains(&"40".to_string()),
        "{:?}",
        driver::fields(&app)
    );
}

/// A part holding `width` at 120, a sketch with one trait in it, the
/// dimension tool in hand and a dimension of the trait just placed: its
/// field open, holding what the trait measures. Nothing opens beside the
/// canvas in between, which would move the trait from under the clicks.
fn a_dimension_just_placed(test: &str) -> (driver::App, String) {
    let mut app = a_part_with_a_width(test);
    driver::click_the_button(&mut app, "Variables");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::click(&mut app, "Cote");
    driver::click_at(&mut app, (850.0, 510.0));
    driver::click_at(&mut app, (900.0, 440.0));
    let open = driver::fields(&app);
    assert_eq!(
        open.len(),
        1,
        "the dimension's field, and it alone: {open:?}"
    );
    let measured = open[0].clone();
    (app, measured)
}

#[test]
fn a_dimension_placed_with_its_tool_takes_the_number_typed_for_it() {
    let (mut app, measured) = a_dimension_just_placed("a_dimension_placed_takes_a_number");

    driver::type_over(&mut app, &measured, "50");
    driver::press(&mut app, egui::Key::Enter);
    driver::open_the_history(&mut app);

    assert!(
        driver::shows(&app, "Cote 50 mm"),
        "the trait was dimensioned to what was typed: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_dimension_placed_with_its_tool_takes_a_formula_typed_for_it() {
    let (mut app, measured) = a_dimension_just_placed("a_dimension_placed_takes_a_formula");

    driver::type_over(&mut app, &measured, "width / 4");
    driver::press(&mut app, egui::Key::Enter);
    driver::open_the_history(&mut app);

    assert!(
        driver::shows(&app, "width / 4 = 30"),
        "the value says what it was written from and what that comes to: {:?}",
        driver::on_screen(&app),
    );
}

/// A plate drawn and raised by `width / 10`, with `width` at 120, the
/// history open.
fn a_plate_raised_by_a_formula(test: &str) -> driver::App {
    let mut app = a_part_with_a_width(test);
    driver::click_the_button(&mut app, "Variables");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click(&mut app, "Terminer");
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at(&mut app, (850.0, 510.0));

    driver::type_over(&mut app, "10", "=width/10");
    driver::click(&mut app, "Appliquer");
    driver::open_the_history(&mut app);
    app
}

#[test]
fn an_extrusion_raised_by_a_formula_says_it_in_the_history() {
    let app = a_plate_raised_by_a_formula("an_extrusion_raised_by_a_formula");

    assert!(
        driver::shows(&app, "width / 10 = 12"),
        "the step says what it was raised by and what that comes to: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn changing_a_variable_in_its_panel_raises_the_plate_again() {
    let mut app = a_plate_raised_by_a_formula("changing_a_variable_raises_again");
    driver::click_the_button(&mut app, "Variables");

    driver::type_over(&mut app, "120", "200");
    driver::press(&mut app, egui::Key::Enter);

    assert!(
        driver::shows(&app, "width / 10 = 20"),
        "the extrusion follows width: {:?}",
        driver::on_screen(&app),
    );
    assert!(
        driver::shows(&app, "Variable width = 200"),
        "and the change is a step of the history: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn undoing_a_change_to_a_variable_puts_the_plate_back() {
    let mut app = a_plate_raised_by_a_formula("undoing_a_change_to_a_variable");
    driver::click_the_button(&mut app, "Variables");
    driver::type_over(&mut app, "120", "200");
    driver::press(&mut app, egui::Key::Enter);

    driver::press_with_command(&mut app, egui::Key::Z);

    assert!(
        driver::shows(&app, "width / 10 = 12"),
        "{:?}",
        driver::on_screen(&app)
    );
    assert!(driver::fields(&app).contains(&"120".to_string()));
}

#[test]
fn a_variable_in_use_is_kept_and_the_panel_says_what_uses_it() {
    let mut app = a_plate_raised_by_a_formula("a_variable_in_use_is_kept");
    driver::click_the_button(&mut app, "Variables");

    driver::click_exactly(&mut app, "Retirer");

    assert!(
        driver::says(&app, "sert encore") && driver::says(&app, "width / 10 = 12"),
        "the refusal names the extrusion written from it",
    );
    assert!(driver::fields(&app).contains(&"width".to_string()));
}

#[test]
fn a_formula_that_does_not_read_is_refused_where_it_is_typed() {
    let mut app = a_part_with_a_width("a_formula_that_does_not_read");

    driver::type_into(&mut app, "nom", "height");
    driver::type_into(&mut app, "formule", "width *");
    driver::click_exactly(&mut app, "Ajouter");

    assert!(driver::says(&app, "s'arrête avant sa dernière valeur"));
    assert!(
        driver::fields(&app).contains(&"width *".to_string()),
        "what was typed stays to be fixed: {:?}",
        driver::fields(&app),
    );
}

#[test]
fn a_variable_nothing_uses_is_retired_from_its_panel() {
    let mut app = a_part_with_a_width("a_variable_nothing_uses_is_retired");

    driver::click_exactly(&mut app, "Retirer");

    assert!(
        !driver::fields(&app).contains(&"width".to_string()),
        "the row went with the variable: {:?}",
        driver::fields(&app),
    );
}

#[test]
fn a_pattern_shows_its_values_while_the_centre_is_awaited() {
    // Wide enough for the ribbon's patterns to be on screen. The canvas is as
    // much wider, so what is drawn on it moves right by half of that.
    let mut app = driver::open_on("a_pattern_shows_its_values", (1920.0, 800.0));
    let right = |(x, y): (f32, f32)| (x + 320.0, y);
    driver::create_a_part(&mut app, "Platine");
    driver::click(&mut app, "Nouvelle esquisse");
    driver::click_at(&mut app, right(driver::ON_ONE_OF_THE_BASE_PLANES));
    driver::click(&mut app, "Ligne (L)");
    driver::click_at(&mut app, right(driver::A_POINT_ABOVE_THE_ORIGIN));
    driver::click_at(&mut app, right(driver::A_POINT_BELOW_AND_RIGHT));
    driver::press(&mut app, egui::Key::Escape);
    driver::click(&mut app, "Réseau circulaire");
    driver::click_at(&mut app, right((850.0, 510.0)));

    driver::press(&mut app, egui::Key::Enter);

    assert!(
        driver::fields(&app).contains(&"4".to_string()),
        "Enter closed the selection rather than the sketch, and the step and \
         the count are there to be typed over: {:?}",
        driver::fields(&app),
    );
}

#[test]
fn escape_in_a_row_drops_what_was_typed_and_records_nothing() {
    let mut app = a_part_with_a_width("escape_in_a_row_drops_what_was_typed");

    driver::type_over(&mut app, "120", "200");
    driver::press(&mut app, egui::Key::Escape);
    driver::click_the_button(&mut app, "Historique");

    assert!(
        driver::fields(&app).contains(&"120".to_string()),
        "the row shows the formula it had: {:?}",
        driver::fields(&app),
    );
    assert!(
        !driver::shows(&app, "width = 200"),
        "nothing went into the history: {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_name_typed_in_the_panel_while_a_line_is_half_drawn_goes_into_the_field() {
    let mut app = driver::open("a_name_typed_while_a_line_is_half_drawn");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);
    driver::click_the_button(&mut app, "Variables");
    app.hover_at(egui::pos2(10.0, 790.0));
    app.run();

    driver::type_into(&mut app, "nom", "h");
    driver::key(&mut app, egui::Key::B, "b");

    assert!(
        driver::fields(&app).contains(&"hb".to_string()),
        "letters typed in a field are the field's, not shortcuts: {:?}",
        driver::fields(&app),
    );
}
