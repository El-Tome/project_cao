//! A rule taken by its mark, in the whole application: the marks are read off
//! the frame egui painted, where the user sees them, and aimed at from there.
//!
//! Closes #538.
//! - pointing at a mark lights it — `pointing_at_a_mark_lights_it`
//! - a click on a mark takes the rule, alone —
//!   `a_click_on_a_mark_takes_its_rule_and_delete_erases_it`
//! - a Cmd or Shift click adds the rule to the selection —
//!   `a_command_click_on_a_second_mark_adds_its_rule`
//! - a box takes a rule whose mark is inside it — no test: held by
//!   `a_box_takes_each_rule_with_a_mark_inside_it_once`, in `input/marks/tests.rs`;
//!   driven whole, the magnets pull a box's ends onto the corner beside the mark
//! - Delete or Backspace erases what is held, rules included, in one step —
//!   `a_click_on_a_mark_takes_its_rule_and_delete_erases_it`
//! - a mark is found where it is drawn, including when another pushed it aside
//!   — no test: held by `a_mark_set_aside_by_another_is_taken_where_it_was_set`,
//!   in `render/overlays/tests.rs`, since one function now places the marks
//!   for the painter and the pick both
//! - a click on a trait away from its mark still takes the trait —
//!   `a_click_on_a_side_away_from_its_marks_still_takes_the_side`
//! - a drag that starts on a mark draws a box —
//!   `a_press_on_a_mark_pulls_a_box_rather_than_the_side_under_it`
//! - `docs/sketch.md` says how a mark is picked — no test: prose

mod driver;

use egui::epaint::Shape;

/// Every mark painted on the last frame: what it says, where, in what colour.
fn marks(app: &driver::App) -> Vec<(String, egui::Pos2, egui::Color32)> {
    fn walk(shape: &Shape, out: &mut Vec<(String, egui::Pos2, egui::Color32)>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            Shape::Text(text) if text.galley.text() == "|_" => out.push((
                text.galley.text().to_owned(),
                text.visual_bounding_rect().center(),
                text.fallback_color,
            )),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &app.output().shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// A rectangle, its right angles marked, in the selection tool.
fn a_rectangle(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Équerre");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click(&mut app, "Sélection (S)");
    app.run();
    assert!(
        marks(&app).len() >= 2,
        "the rectangle's right angles are marked: {:?}",
        marks(&app)
    );
    app
}

fn first_mark(app: &driver::App) -> egui::Pos2 {
    marks(app)[0].1
}

fn delete_and_read_the_history(app: &mut driver::App) -> Vec<String> {
    driver::press(app, egui::Key::Delete);
    driver::open_the_history(app);
    driver::on_screen(app)
}

fn click_with(app: &mut driver::App, at: egui::Pos2, modifiers: egui::Modifiers) {
    app.hover_at(at);
    app.event(egui::Event::ModifiersChanged(modifiers));
    app.run();
    for pressed in [true, false] {
        app.event(egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers,
        });
        app.run();
    }
    app.event(egui::Event::ModifiersChanged(egui::Modifiers::NONE));
    app.run();
}

/// The mark of the rectangle's top right corner, up and to the right of it,
/// where nothing else of the drawing is near.
fn the_mark_out_in_the_open(app: &driver::App) -> egui::Pos2 {
    marks(app)
        .into_iter()
        .map(|(_, at, _)| at)
        .max_by(|one, other| (one.x - one.y).total_cmp(&(other.x - other.y)))
        .expect("a mark")
}

#[test]
fn pointing_at_a_mark_lights_it() {
    let mut app = a_rectangle("pointing_at_a_mark_lights_it");
    let (_, at, resting) = marks(&app)[0].clone();

    app.hover_at(at);
    app.run();
    app.run();

    let lit = marks(&app)
        .into_iter()
        .find(|(_, place, _)| place.distance(at) < 1.0)
        .expect("the mark is still there");
    assert_ne!(lit.2, resting, "the mark pointed at changes colour");
}

#[test]
fn a_click_on_a_mark_takes_its_rule_and_delete_erases_it() {
    let mut app = a_rectangle("a_click_on_a_mark_takes_its_rule");
    let at = first_mark(&app);

    driver::click_at(&mut app, (at.x, at.y));
    let history = delete_and_read_the_history(&mut app);

    assert!(
        history
            .iter()
            .any(|line| line.contains("Perpendiculaire supprimée")),
        "the rule went, and nothing else: {history:?}"
    );
}

#[test]
fn a_command_click_on_a_second_mark_adds_its_rule() {
    let mut app = a_rectangle("a_command_click_on_a_second_mark");
    let (first, second) = (marks(&app)[0].1, marks(&app)[1].1);

    driver::click_at(&mut app, (first.x, first.y));
    click_with(&mut app, second, egui::Modifiers::COMMAND);
    let history = delete_and_read_the_history(&mut app);

    assert!(
        history
            .iter()
            .any(|line| line.contains("2 éléments supprimés")),
        "both rules went, in one step: {history:?}"
    );
}

#[test]
fn a_press_on_a_mark_pulls_a_box_rather_than_the_side_under_it() {
    let mut app = a_rectangle("a_press_on_a_mark_pulls_a_box");
    let at = the_mark_out_in_the_open(&app);

    driver::drag_and_drop(&mut app, at, at + egui::vec2(40.0, -40.0));
    driver::open_the_history(&mut app);

    let history = driver::on_screen(&app);
    assert!(
        history.iter().any(|line| line.starts_with("2. "))
            && !history.iter().any(|line| line.starts_with("3. ")),
        "the press moved nothing, and wrote nothing: {history:?}"
    );
}

#[test]
fn a_click_on_a_side_away_from_its_marks_still_takes_the_side() {
    let mut app = a_rectangle("a_click_on_a_side_away_from_its_marks");
    let (left, top) = driver::A_POINT_ABOVE_THE_ORIGIN;
    let (_, bottom) = driver::A_POINT_BELOW_AND_RIGHT;

    driver::click_at(&mut app, (left, top + (bottom - top) * 0.3));
    let history = delete_and_read_the_history(&mut app);

    assert!(
        history.iter().any(|line| line.contains("Trait supprimé")),
        "{history:?}"
    );
}
