//! A selection box pulled across the drawing, in the whole application: the
//! box is read off the frame egui painted, where the user sees it.
//!
//! Closes #657.
//! - The box drawn on the canvas starts and ends under the hand, wherever the
//!   drawing's magnets are — `the_box_drawn_on_the_canvas_starts_and_ends_under_the_hand`
//! - A box drawn around a corner's mark, driven whole in
//!   `crates/app/tests/taking_a_rule.rs`, takes the rule alone — no test:
//!   held by `a_box_drawn_around_a_corner_mark_takes_the_rule_alone`, in
//!   `taking_a_rule.rs`
//! - While a box is being pulled, no magnet mark is shown (surfaced by review)
//!   — no test: held by `no_magnet_is_marked_while_a_box_is_pulled`, in
//!   `screens/viewport/input/selecting/tests.rs`, where the editor's snap,
//!   which the mark is drawn from, can be read; the mark itself is drawn by
//!   wgpu, which this harness does not run

mod driver;

use egui::epaint::Shape;
use glam::DVec2;

/// Every four-sided shape filled and outlined on the last frame, by its
/// bounds on screen: the box is one of them, and only while it is pulled.
fn boxes(app: &driver::App) -> Vec<egui::Rect> {
    fn walk(shape: &Shape, out: &mut Vec<egui::Rect>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            Shape::Path(path)
                if path.closed
                    && path.points.len() == 4
                    && path.fill != egui::Color32::TRANSPARENT =>
            {
                out.push(egui::Rect::from_points(&path.points));
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &app.output().shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

fn pull_without_letting_go(app: &mut driver::App, from: egui::Pos2, to: egui::Pos2) {
    app.hover_at(from);
    app.run();
    app.drag_at(from);
    app.run();
    app.hover_at(from + (to - from) * 0.5);
    app.run();
    app.hover_at(to);
    app.run();
    app.run();
}

/// A grid whose every node pulls from across the whole canvas: wherever the
/// hand is, the magnets are somewhere else, and by many pixels.
fn a_grid_that_pulls_from_everywhere() -> cao_prefs::Settings {
    let mut settings = cao_prefs::Settings::default();
    settings.viewport.grid_snap = true;
    settings.viewport.grid_snap_divisions = 1;
    settings.viewport.grid_snap_pixels = 10_000.0;
    settings
}

/// Where the box is pressed, a point of the sketch: where the press was first
/// written, brought down by twice 22.11 mm, out of the band the pill a sketch
/// shows at the top of the view covers (#727).
const WHERE_THE_BOX_STARTS: DVec2 = DVec2::new(2.21, 82.0);

#[test]
fn the_box_drawn_on_the_canvas_starts_and_ends_under_the_hand() {
    let mut app = driver::open_held_with(
        "the_box_drawn_on_the_canvas_starts_and_ends_under_the_hand",
        a_grid_that_pulls_from_everywhere(),
    );
    driver::create_a_part(&mut app, "Cadre");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Sélection (S)");
    app.run();
    let pressed = driver::sketch_point_on_screen(&app, WHERE_THE_BOX_STARTS);
    let held = driver::sketch_point_on_screen(&app, DVec2::new(123.21, -27.78));
    let before = boxes(&app);

    pull_without_letting_go(&mut app, pressed, held);

    let drawn: Vec<egui::Rect> = boxes(&app)
        .into_iter()
        .filter(|shape| !before.contains(shape))
        .collect();
    let hand = egui::Rect::from_two_pos(pressed, held);
    assert!(
        drawn.iter().any(|shape| {
            shape.min.distance(hand.min) < 1.0 && shape.max.distance(hand.max) < 1.0
        }),
        "the box runs from {pressed:?} to {held:?}, where the hand is, not where the magnets \
         pull: drawn {drawn:?}"
    );
}
