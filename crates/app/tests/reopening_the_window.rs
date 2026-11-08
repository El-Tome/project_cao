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
