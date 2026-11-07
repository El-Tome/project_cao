//! The driver holds the drawing area at the height its fixtures were tuned
//! at, so that the chrome can grow without a reach in pixels taking something
//! else: #725 gave the ribbon its icons, and three tests went another way
//! until the canvas was held. These hold the driver to it, and to saying so
//! when a test reads a point off a canvas it no longer holds.

mod driver;

use glam::DVec2;

/// How far a held canvas may sit from its height, in points: under a pixel.
const WITHIN: f32 = 0.5;

#[test]
fn a_part_and_a_sketch_are_drawn_on_canvases_as_high_as_their_fixtures_were_tuned_on() {
    let mut app = driver::open("canvases_as_high_as_their_fixtures");
    driver::create_a_part(&mut app, "Platine");
    assert!(
        (driver::canvas(&app).height() - driver::CANVAS_HEIGHT_OF_A_PART).abs() < WITHIN,
        "the part is drawn on a canvas of {:?}, where its fixtures were tuned on one {} high",
        driver::canvas(&app),
        driver::CANVAS_HEIGHT_OF_A_PART,
    );

    driver::start_a_sketch(&mut app);
    assert!(
        (driver::canvas(&app).height() - driver::CANVAS_HEIGHT_OF_A_SKETCH).abs() < WITHIN,
        "the sketch is drawn on a canvas of {:?}, where its fixtures were tuned on one {} high",
        driver::canvas(&app),
        driver::CANVAS_HEIGHT_OF_A_SKETCH,
    );
}

#[test]
fn a_window_a_test_chose_for_itself_is_left_as_it_chose_it() {
    let chosen = egui::vec2(1280.0, 760.0);
    let mut app = driver::open_on("a_window_a_test_chose", chosen);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);

    assert_eq!(
        app.ctx.content_rect().size(),
        chosen,
        "the driver resized a window the test had chosen",
    );
}

#[test]
#[should_panic(expected = "where the driver holds it at")]
fn a_point_read_off_a_canvas_the_driver_no_longer_holds_says_so() {
    let mut app = driver::open("a_canvas_no_longer_held");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);

    app.set_size(driver::SCREEN);
    app.run();

    driver::sketch_point_on_screen(&app, DVec2::ZERO);
}
