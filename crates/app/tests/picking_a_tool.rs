//! A tool picked from the toolbar takes, when the driver is the one clicking —
//! and when it cannot take, the driver says so instead of passing quietly.
//!
//! Closes #399.
//! - a test picks a tool from the toolbar, clicks the canvas, and the drawing
//!   shows that tool's work —
//!   `the_fillet_picked_from_the_toolbar_rounds_the_corner_clicked_next`
//! - the driver's click says so when it matches a node that cannot be
//!   clicked, rather than passing quietly —
//!   `clicking_a_button_past_the_edge_of_the_screen_says_so`
//! - the tests #377 already carries keep passing — no test: they are
//!   `drawing.rs`'s own, run in the same gate

mod driver;

#[test]
fn the_fillet_picked_from_the_toolbar_rounds_the_corner_clicked_next() {
    // Wide enough for the whole ribbon; the canvas is as much wider, so what
    // is drawn on it moves right by half of that.
    let mut app = driver::open_on("the_fillet_rounds_the_corner", (1920.0, 800.0));
    let right = |(x, y): (f32, f32)| (x + 320.0, y);
    driver::create_a_part(&mut app, "Platine");
    driver::click(&mut app, "Nouvelle esquisse");
    driver::click_at(&mut app, right(driver::ON_ONE_OF_THE_BASE_PLANES));
    driver::click(&mut app, "Ligne (L)");
    driver::click_at(&mut app, right(driver::A_POINT_ABOVE_THE_ORIGIN));
    driver::click_at(&mut app, right(driver::A_POINT_BELOW_AND_RIGHT));
    driver::click_at(&mut app, right((900.0, 700.0)));
    driver::press(&mut app, egui::Key::Escape);

    driver::click(&mut app, "Congé");
    driver::click_at(&mut app, right(driver::A_POINT_BELOW_AND_RIGHT));
    driver::type_into_the_only_field(&mut app, "2");
    driver::press(&mut app, egui::Key::Enter);

    driver::open_the_history(&mut app);
    assert!(
        driver::shows(&app, "Coin arrondi"),
        "the corner clicked with the fillet in hand is a step of the history, \
         showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
#[should_panic(expected = "where a click reaches nothing")]
fn clicking_a_button_past_the_edge_of_the_screen_says_so() {
    let mut app = driver::open("clicking_a_button_past_the_edge");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);

    driver::click(&mut app, "Congé");
}
