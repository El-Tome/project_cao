//! The driver holds the drawing area at the height its fixtures were tuned
//! at, so that the chrome can grow without a reach in pixels taking something
//! else: #725 gave the ribbon its icons, and three tests went another way
//! until the canvas was held. These hold the driver to it, and to saying so
//! when a test reads a point off a canvas it no longer holds.
//!
//! What the hold hides from every other test is shown here: how much room the
//! default window still leaves a sketch, held to the least that was accepted.

mod driver;

use glam::DVec2;

/// How far a held canvas may sit from its height, in points: under a pixel.
const WITHIN: f32 = 0.5;

/// The least canvas a sketch may be left on the default window, in points.
/// #725 put each ribbon button's icon above its name, and the sketch's canvas
/// at 1280 × 800 went from 492 points to about 325; Tom tried it and accepted
/// it on 2026-11-07, until #731 regroups the ribbon. The figure sits a few
/// points under what he saw, so that a font's rounding does not trip it and a
/// title bar or a view bar does. It moves only once he has been told.
const THE_LEAST_A_SKETCH_IS_LEFT: f32 = 320.0;

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

/// The window chosen is the default one on purpose: a test that asks for
/// `SCREEN` by name is choosing it, as `fitting_the_window.rs` does, and is
/// not to be told apart from one that wants the drawing held by the size it
/// asks for.
#[test]
fn a_window_a_test_chose_for_itself_is_left_as_it_chose_it() {
    let chosen = driver::SCREEN;
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

#[test]
fn the_default_window_still_leaves_a_sketch_the_room_that_was_accepted() {
    let mut app = driver::open_on("the_room_left_to_a_sketch", driver::SCREEN);
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);

    let canvas = driver::canvas(&app);
    assert!(
        canvas.height() >= THE_LEAST_A_SKETCH_IS_LEFT,
        "at 1280 × 800, a sketch is drawn on {canvas:?}, under the \
         {THE_LEAST_A_SKETCH_IS_LEFT} points accepted with #725's ribbon: the chrome grew \
         again. The driver's hold keeps every other test green over it, so say it in the \
         issue that grows it, for Tom to try, before this figure moves.",
    );
}
