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

mod driver;

use egui::epaint::Shape;

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

#[test]
fn the_box_drawn_on_the_canvas_starts_and_ends_under_the_hand() {
    let mut app = driver::open("the_box_drawn_on_the_canvas_starts_and_ends_under_the_hand");
    driver::create_a_part(&mut app, "Cadre");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click(&mut app, "Sélection (S)");
    app.run();
    let (right, bottom) = driver::A_POINT_BELOW_AND_RIGHT;
    let beside_a_grid_node_above_the_drawing = egui::pos2(732.4, 345.7);
    let below_the_bottom_right_corner = egui::pos2(right, bottom + 23.0);
    let (pressed, held) = (
        beside_a_grid_node_above_the_drawing,
        below_the_bottom_right_corner,
    );
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
