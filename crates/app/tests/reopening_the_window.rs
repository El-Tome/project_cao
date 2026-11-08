//! The window reopening where it was closed, as far as the application driven
//! with no window can see it: the preference that turns it off, and the driver
//! kept out of it. How the window opens is decided in
//! `crates/prefs/src/window_place/tests.rs`.
//!
//! Closes #780.
//! - A setting « Rouvrir à la dernière taille », on by default; off, the
//!   application opens maximised whatever was remembered — the box in the
//!   preferences, ticked for a new profile and unticked by hand, in
//!   `unticking_reopen_at_the_last_size_leaves_the_open_window_alone_until_the_next_launch`
//! - Changing the setting does not move the open window; it takes effect at
//!   the next launch —
//!   `unticking_reopen_at_the_last_size_leaves_the_open_window_alone_until_the_next_launch`:
//!   the profile the next launch reads holds it off, and the window kept the
//!   size it had
//! - A size that cannot be written on close is lost: the application closes
//!   normally, no error shown — no test: the place is written in `on_exit`
//!   from what the real window reported, and the driver has none —
//!   `winit_window()` is `None` under kittest, which never calls `on_exit`
//!   either — so no test of this workspace can make that write happen, let
//!   alone fail. The error dropped there is read at review and tried in the app
//! - The driver tests depend neither on the size of the screen nor on a
//!   remembered size — `the_driver_neither_reads_nor_writes_a_remembered_window_place`
//!
//! What the driver cannot see is a move made on the real window: the place is
//! applied there once, in `CaoApp::new`, and the setting read there and nowhere
//! else. The window the box's test watches is the harness's, which follows a
//! size the application asks egui for and nothing the system is told directly. The screen's size is never asked under kittest, as there is no
//! window to ask it from: a driver test that depended on it could not run.

mod driver;

use cao_prefs::WindowPlace;
use egui_kittest::kittest::Queryable;

/// How far the window may be from a size it should have kept, in points: it is
/// set whole and never computed, so anything past rounding is a resize.
const KEPT_WITHIN: f32 = 0.5;

const REOPEN_AT_THE_LAST_SIZE: &str = "Rouvrir à la dernière taille";

#[test]
fn unticking_reopen_at_the_last_size_leaves_the_open_window_alone_until_the_next_launch() {
    let test = "unticking_reopen_at_the_last_size_leaves_the_open_window_alone";
    let mut app = driver::open_on(test, driver::SCREEN);
    driver::create_a_part(&mut app, "Fenêtre");
    driver::click_exactly(&mut app, "Préférences");
    driver::click(&mut app, "Apparence");
    assert!(
        app.query_by_label(REOPEN_AT_THE_LAST_SIZE).is_some(),
        "no « {REOPEN_AT_THE_LAST_SIZE} » in the preferences' appearance page: {:?}",
        driver::on_screen(&app),
    );
    assert!(
        driver::is_on(&app, REOPEN_AT_THE_LAST_SIZE),
        "a new profile reopens the window at its last size",
    );
    let before = app.ctx.content_rect().size();

    app.get_by_label(REOPEN_AT_THE_LAST_SIZE).scroll_to_me();
    app.run();
    app.get_by_label(REOPEN_AT_THE_LAST_SIZE).click_accesskit();
    app.run();

    assert!(
        !driver::is_on(&app, REOPEN_AT_THE_LAST_SIZE),
        "the box was clicked and stayed ticked",
    );
    assert!(
        !driver::saved_settings(test).reopen_at_last_size,
        "the profile the next launch reads still reopens at the last size",
    );
    let after = app.ctx.content_rect().size();
    assert!(
        (after - before).length() < KEPT_WITHIN,
        "the open window went from {before:?} to {after:?} when the setting changed; it \
         takes effect at the next launch",
    );
}

#[test]
fn the_driver_neither_reads_nor_writes_a_remembered_window_place() {
    let test = "the_driver_neither_reads_nor_writes_a_remembered_window_place";
    let remembered = WindowPlace {
        x: 40,
        y: 30,
        width: 1500,
        height: 900,
        maximised: false,
    };
    let mut app = driver::open_on_remembering(test, driver::SCREEN, remembered);
    let laid_down = std::fs::read(driver::window_place_file(test)).expect("the place is there");

    app.run();
    eframe::App::on_exit(app.state_mut());

    let window = app.ctx.content_rect().size();
    assert!(
        (window - driver::SCREEN).length() < KEPT_WITHIN,
        "the driver's window is the size its test named, {:?}, whatever was remembered; \
         it is {window:?}",
        driver::SCREEN,
    );
    assert_eq!(
        std::fs::read(driver::window_place_file(test)).expect("the place is still there"),
        laid_down,
        "a test closing the application wrote a window place over the one remembered",
    );
}
