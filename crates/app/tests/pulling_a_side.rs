//! A side pressed and pulled in the canvas, driven with no window.
//!
//! Closes #422.
//! - 14: a press on a side and a drag moves that side, and lands in the history
//!   as a step of its own — `a_side_pressed_and_pulled_moves_it_in_one_step`;
//!   a drag from empty space still pulls a box and moves nothing —
//!   `a_drag_from_empty_space_moves_nothing`
//! - 23: each gesture checked by hand in the application, on a rectangle, a
//!   trait, an arc and an ellipse — no test: the hand is the check, and the
//!   list of what to try is in the pull request

mod driver;

/// Where the rectangle's top side is drawn on screen: between the two corners
/// the tool is clicked at, at the height of the first.
const ON_THE_TOP_SIDE: (f32, f32) = (850.0, 400.0);

fn a_rectangle(app: &mut driver::App) {
    driver::click(app, "Rectangle (R)");
    driver::click_at(app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::press(app, egui::Key::Escape);
    driver::click(app, "Sélection (S)");
}

/// The numbered steps of the history, in order.
fn steps(app: &driver::App) -> Vec<String> {
    driver::on_screen(app)
        .into_iter()
        .filter(|label| {
            label
                .split_once(". ")
                .is_some_and(|(rank, _)| rank.parse::<usize>().is_ok())
        })
        .collect()
}

#[test]
fn a_side_pressed_and_pulled_moves_it_in_one_step() {
    let mut app = driver::open("a_side_pressed_and_pulled");
    driver::create_a_part(&mut app, "Cadre");
    driver::start_a_sketch(&mut app);
    a_rectangle(&mut app);

    driver::drag_and_drop(
        &mut app,
        egui::pos2(ON_THE_TOP_SIDE.0, ON_THE_TOP_SIDE.1),
        egui::pos2(ON_THE_TOP_SIDE.0 + 4.0, ON_THE_TOP_SIDE.1 - 60.0),
    );
    driver::open_the_history(&mut app);

    let steps = steps(&app);
    assert!(
        steps
            .last()
            .is_some_and(|last| last.contains("Côté déplacé")),
        "the pull is the last step: {steps:?}",
    );
}

#[test]
fn a_drag_from_empty_space_moves_nothing() {
    let mut app = driver::open("a_drag_from_empty_space");
    driver::create_a_part(&mut app, "Cadre");
    driver::start_a_sketch(&mut app);
    a_rectangle(&mut app);
    driver::open_the_history(&mut app);
    let before = steps(&app);

    driver::drag_and_drop(&mut app, egui::pos2(420.0, 250.0), egui::pos2(520.0, 320.0));

    assert_eq!(steps(&app), before, "a box selects, and writes no step");
}
