//! A side pressed and pulled in the canvas, driven with no window.
//!
//! Closes #422.
//! - 14: a press on a side and a drag moves that side, and lands in the history
//!   as a step of its own — `a_side_pressed_and_pulled_moves_it_in_one_step`;
//!   a drag from empty space still pulls a box and moves nothing —
//!   `a_drag_from_empty_space_moves_nothing`; a pan of the view started on a
//!   side moves nothing either — `a_pan_started_on_a_side_moves_nothing`
//! - 15: a circle pulled out is still drawn to its new size —
//!   `a_circle_pulled_out_is_still_drawn_to_its_new_size`
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
    driver::open_the_history(&mut app);
    let before = steps(&app).len();

    driver::drag_and_drop(
        &mut app,
        egui::pos2(ON_THE_TOP_SIDE.0, ON_THE_TOP_SIDE.1),
        egui::pos2(ON_THE_TOP_SIDE.0 + 4.0, ON_THE_TOP_SIDE.1 - 60.0),
    );

    let steps = steps(&app);
    assert_eq!(
        steps.len(),
        before + 1,
        "one step for the gesture: {steps:?}"
    );
    assert!(
        steps
            .last()
            .is_some_and(|last| last.contains("Côté déplacé")),
        "the pull is the last step: {steps:?}",
    );
}

#[test]
fn a_pan_started_on_a_side_moves_nothing() {
    let mut app = driver::open("a_pan_started_on_a_side");
    driver::create_a_part(&mut app, "Cadre");
    driver::start_a_sketch(&mut app);
    a_rectangle(&mut app);
    driver::open_the_history(&mut app);
    let before = steps(&app);
    let on_the_side = egui::pos2(ON_THE_TOP_SIDE.0, ON_THE_TOP_SIDE.1);

    app.hover_at(on_the_side);
    app.run();
    let button = |pressed: bool, pos: egui::Pos2| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Middle,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    app.event(button(true, on_the_side));
    app.run();
    for step in 1..=6 {
        app.hover_at(on_the_side + egui::vec2(0.0, -10.0 * step as f32));
        app.run();
    }
    app.event(button(false, on_the_side + egui::vec2(0.0, -60.0)));
    app.run();

    assert_eq!(steps(&app), before, "moving the view writes no step");
}

#[test]
fn a_circle_pulled_out_is_still_drawn_to_its_new_size() {
    let mut app = driver::open("a_circle_pulled_out");
    driver::create_a_part(&mut app, "Rondelle");
    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Cercle (C)");
    driver::click_at(&mut app, (850.0, 500.0));
    driver::click_at(&mut app, (950.0, 500.0));
    driver::press(&mut app, egui::Key::Escape);
    driver::click(&mut app, "Sélection (S)");
    driver::open_the_history(&mut app);
    let before = steps(&app).len();

    // On the outline, away from the point it was drawn through: where the
    // view shows it, read off the canvas rather than worked out.
    let on_the_rim = egui::pos2(921.0, 571.0);
    driver::drag_and_drop(&mut app, on_the_rim, on_the_rim + egui::vec2(0.0, -40.0));

    let steps = steps(&app);
    assert_eq!(
        steps.len(),
        before + 1,
        "one step for the gesture: {steps:?}"
    );
    assert!(
        steps
            .last()
            .is_some_and(|last| last.contains("Taille changée")),
        "the pull drew the circle to a new size: {steps:?}",
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
