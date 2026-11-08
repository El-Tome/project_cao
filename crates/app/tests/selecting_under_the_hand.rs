//! The selection tool aimed in the whole application: what a press or a click
//! takes is read where the hand is, not where a magnet pulls it.
//!
//! Closes #664.
//! - A press 22 px from a side, driven whole in `crates/app/tests/`, starts a
//!   box and grabs no side — `a_press_22_px_from_a_side_pulls_a_box_and_moves_no_side`
//! - A click 22 px from a point, with the magnet's reach at its default,
//!   selects nothing; a click 10 px from it selects it —
//!   `a_click_22_px_from_a_point_takes_nothing`, `a_click_10_px_from_a_point_takes_it`
//! - Pointing 22 px from a point with the selection tool lights nothing and
//!   shows no magnet mark; dragging a point near another still shows the mark
//!   and lands on it — no test: held by
//!   `pointing_22_px_from_a_point_lights_nothing_and_marks_no_magnet` and
//!   `dragging_a_point_near_another_still_marks_the_magnet_and_lands_on_it`, in
//!   `screens/viewport/input/selecting/tests.rs`, where what is lit and the
//!   magnet the mark is drawn from can be read; both are drawn by wgpu, which
//!   this harness does not run
//! - A selection or an arc pressed a few pixels off its corner or its centre
//!   still follows the hand from where it stood, and dropped near a point lands
//!   exactly on it (surfaced by review) — no test: held by
//!   `a_selection_pressed_beside_its_corner_does_not_jump_on_the_first_frame`,
//!   `a_selection_pressed_beside_its_corner_and_dropped_near_a_point_lands_on_it`
//!   and `an_arc_pressed_beside_its_centre_and_dropped_near_a_point_lands_on_it`,
//!   in `screens/viewport/input/selecting/tests.rs`
//! - A fillet's or a chamfer's point is not lit while just pointing 22 px from
//!   it (surfaced by review) — no test: held by
//!   `a_fillet_s_point_22_px_from_the_hand_is_not_lit_while_pointing`, in
//!   `screens/viewport/render/cuts/tests.rs`
//! - While a point is dragged, the point the magnet will land it on is lit,
//!   and nothing else under the hand is (surfaced by the pixel check) — no
//!   test: held by `a_point_dragged_22_px_from_another_lights_the_one_it_will_land_on`,
//!   in `screens/viewport/input/selecting/tests.rs`

mod driver;

use egui::epaint::Shape;
use glam::DVec2;

/// The magnets at their defaults, the grid's aside: a point clicked down then
/// lands exactly under the hand, so the distances below are the ones on screen.
fn a_grid_that_pulls_nothing() -> cao_prefs::Settings {
    let mut settings = cao_prefs::Settings::default();
    settings.viewport.grid_snap = false;
    settings
}

fn a_sketch_in_the_selection_tool(test: &str, draw: impl FnOnce(&mut driver::App)) -> driver::App {
    let mut app = driver::open_with(test, driver::SCREEN, a_grid_that_pulls_nothing());
    driver::create_a_part(&mut app, "Visée");
    driver::start_a_sketch(&mut app);
    draw(&mut app);
    driver::press(&mut app, egui::Key::Escape);
    driver::click(&mut app, "Sélection (S)");
    app
}

/// The steps of the history, read with the panel opened only now: opening it
/// sooner narrows the canvas and moves the drawing from under the places aimed
/// at.
fn steps_at_the_end(app: &mut driver::App) -> Vec<String> {
    driver::open_the_history(app);
    steps(app)
}

/// Corners of the sketch, where the clicks the tests were first written with
/// landed: the top side runs high above the origin, away from the plane's axes
/// and from anything else drawn.
fn a_rectangle(app: &mut driver::App) {
    driver::click(app, "Rectangle (R)");
    driver::click_at_sketch_point(app, TOP_LEFT_CORNER);
    driver::click_at_sketch_point(app, DVec2::new(168.04, 66.69));
}

const TOP_LEFT_CORNER: DVec2 = DVec2::new(57.5, 110.91);

const LINE_START: DVec2 = driver::A_POINT_ABOVE_THE_ORIGIN;
const LINE_END: DVec2 = driver::A_POINT_BELOW_AND_RIGHT;

fn a_line(app: &mut driver::App) {
    driver::click(app, "Ligne (L)");
    driver::click_at_sketch_point(app, LINE_START);
    driver::click_at_sketch_point(app, LINE_END);
}

/// Out past the line's end, along it, this far on screen: nearer the end than
/// anything else of the drawing, and nearest the line at its end too.
fn past_the_line_s_end(app: &driver::App, pixels: f32) -> egui::Pos2 {
    let start = driver::sketch_point_on_screen(app, LINE_START);
    let end = driver::sketch_point_on_screen(app, LINE_END);
    end + (end - start).normalized() * pixels
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

#[test]
fn a_press_22_px_from_a_side_pulls_a_box_and_moves_no_side() {
    let mut app =
        a_sketch_in_the_selection_tool("a_press_22_px_from_a_side_pulls_a_box", a_rectangle);
    let boxes_before = boxes(&app);
    let on_the_top_side =
        driver::sketch_point_on_screen(&app, DVec2::new(88.46, TOP_LEFT_CORNER.y));
    let pressed = on_the_top_side - egui::vec2(0.0, 22.0);
    let held = driver::sketch_point_on_screen(&app, DVec2::new(119.41, 88.8));

    app.hover_at(pressed);
    app.run();
    app.drag_at(pressed);
    app.run();
    app.hover_at(pressed + (held - pressed) * 0.5);
    app.run();
    app.hover_at(held);
    app.run();
    app.run();
    let pulled_a_box = boxes(&app).into_iter().any(|shape| {
        !boxes_before.contains(&shape)
            && shape
                .min
                .distance(egui::Rect::from_two_pos(pressed, held).min)
                < 1.0
            && shape
                .max
                .distance(egui::Rect::from_two_pos(pressed, held).max)
                < 1.0
    });
    app.event(egui::Event::PointerButton {
        pos: held,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    });
    app.run();

    let after = steps_at_the_end(&mut app);
    assert!(
        pulled_a_box && !after.iter().any(|step| step.contains("déplacé")),
        "a press 22 px above the top side is out of reach of it: it pulls a box \
         ({pulled_a_box}) and moves nothing, got {after:?}"
    );
}

#[test]
fn a_click_22_px_from_a_point_takes_nothing() {
    let mut app = a_sketch_in_the_selection_tool("a_click_22_px_from_a_point", a_line);

    let past = past_the_line_s_end(&app, 22.0);
    driver::click_at(&mut app, past);
    driver::press(&mut app, egui::Key::Delete);

    let after = steps_at_the_end(&mut app);
    assert!(
        !after.iter().any(|step| step.contains("supprim")),
        "the click 22 px from the line's end took something, which Delete then erased: \
         {after:?}"
    );
}

#[test]
fn a_click_10_px_from_a_point_takes_it() {
    let mut app = a_sketch_in_the_selection_tool("a_click_10_px_from_a_point", a_line);

    let past = past_the_line_s_end(&app, 10.0);
    driver::click_at(&mut app, past);
    driver::press(&mut app, egui::Key::Delete);

    let after = steps_at_the_end(&mut app);
    assert!(
        after
            .last()
            .is_some_and(|last| last.contains("Point supprimé")),
        "the click 10 px from the line's end takes the end, which Delete then erases: \
         {after:?}"
    );
}
