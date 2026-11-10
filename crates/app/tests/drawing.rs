//! The application driven with no window: a part is made, a plane is picked in
//! the 3D view, a segment is drawn, and what the interface then says is read
//! back out of its accessibility tree.
//!
//! Closes #377.
//! - a part made from the start menu joins the file list —
//!   `a_part_created_from_the_start_menu_joins_the_file_list`
//! - a sketch started on a plane names that plane in the history —
//!   `a_sketch_started_on_a_plane_names_that_plane_in_the_history`
//! - a segment drawn with two clicks lands in the history as a step —
//!   `a_segment_drawn_with_two_clicks_becomes_a_step_of_the_sketch`
//! - undoing takes the segment back —
//!   `undoing_a_segment_leaves_it_in_the_history_and_something_to_redo`
//! - it runs with no GPU — `the_application_opens_with_no_gpu_adapter`
//! - the tier the tests run in is written down — no test: it is the pull
//!   request body, which no assertion reaches
//!
//! Closes #412.
//! - the width of the second axis can be typed at the cursor while placing —
//!   `the_ellipse_tool_offers_a_field_at_the_cursor_for_its_second_axis`: the
//!   list that lets a tool show its fields had never heard of the ellipse, and
//!   only the application driven whole could see it
//! - an ellipse is one step of the history —
//!   `an_ellipse_drawn_with_three_clicks_becomes_a_step_of_the_sketch`
//!
//! The fourth criterion was written expecting the step to leave the list. It
//! does not: an undone step stays, greyed out and italic, and the test says so.
//! What the tree can show is that something became redoable.
//!
//! Closes #319.
//! - clicks always reach the sketch plane, whatever the state: drawing on a
//!   stretch of plane hidden behind opaque matter still works —
//!   `a_click_lands_on_the_sketch_plane_whatever_the_matter_in_front`
//! - picking on the sketch plane does not break —
//!   `the_dimension_tool_takes_a_segment_behind_the_matter_whatever_its_state`
//! - snapping on the sketch plane does not break — no test: the snap reaches
//!   the tree only as the point a step is laid on, which the history does not
//!   name; the cursor is magnetised before the pick above takes what is under
//!   it, and neither reads the body nor the state

mod driver;

use glam::DVec2;

#[test]
fn the_application_opens_with_no_gpu_adapter() {
    let app = driver::open("the_application_opens_with_no_gpu");

    assert!(
        driver::shows(&app, "Créer"),
        "the start menu is up and offers to make a part, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_part_created_from_the_start_menu_joins_the_file_list() {
    let mut app = driver::open("a_part_joins_the_file_list");

    driver::create_a_part(&mut app, "Équerre");
    driver::click(&mut app, "Accueil");

    assert!(
        !rows_of_the_file_list_reading(&app, "Équerre").is_empty(),
        "the file list names the part that was just created, showing {:?}",
        driver::on_screen(&app),
    );
}

/// Where the file list of the home screen shows a row reading this: below its
/// buttons, and no further right than the widest of them, where the list of
/// recent parts beside it never reaches. The file list is on the home screen
/// alone since #726.
fn rows_of_the_file_list_reading(app: &driver::App, name: &str) -> Vec<egui::Rect> {
    use egui_kittest::kittest::Queryable;

    let header =
        ["Nouveau dossier", "Renommer", "Supprimer"].map(|button| app.get_by_label(button).rect());
    let below = header.iter().map(|at| at.bottom()).fold(f32::MIN, f32::max);
    let column = header.iter().map(|at| at.right()).fold(f32::MIN, f32::max);
    app.query_all_by_label(name)
        .map(|node| node.rect())
        .filter(|at| at.top() >= below && at.right() <= column)
        .collect()
}

#[test]
fn a_sketch_started_on_a_plane_names_that_plane_in_the_history() {
    let mut app = driver::open("a_sketch_names_its_plane");
    driver::create_a_part(&mut app, "Platine");

    driver::start_a_sketch(&mut app);
    driver::open_the_history(&mut app);

    assert_eq!(
        driver::detail_of(&app, "1. Esquisse 1").as_deref(),
        Some("Plan YZ"),
        "the history names the sketch and the plane it sits on, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_segment_drawn_with_two_clicks_becomes_a_step_of_the_sketch() {
    let mut app = driver::open("a_segment_becomes_a_step");
    driver::create_a_part(&mut app, "Entretoise");
    driver::start_a_sketch(&mut app);

    driver::draw_a_segment(&mut app);
    driver::open_the_history(&mut app);

    assert!(
        driver::shows(&app, "Trait"),
        "the history names the segment that was drawn, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn undoing_a_segment_leaves_it_in_the_history_and_something_to_redo() {
    let mut app = driver::open("undoing_leaves_something_to_redo");
    driver::create_a_part(&mut app, "Chape");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::open_the_history(&mut app);
    assert!(driver::shows(&app, "Trait"), "the segment is there to undo");
    assert!(
        !driver::is_enabled(&app, "Rétablir"),
        "nothing has been undone yet, so there is nothing to redo",
    );

    driver::click(&mut app, "Annuler");

    assert!(
        driver::is_enabled(&app, "Rétablir"),
        "the segment was undone, so it can be put back",
    );
    assert!(
        driver::shows(&app, "Trait"),
        "an undone step stays in the history, shown greyed out rather than removed, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn command_shift_z_redoes_what_command_z_undid() {
    let mut app = driver::open("command_shift_z_redoes");
    driver::create_a_part(&mut app, "Chape");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::open_the_history(&mut app);

    driver::press_with_command(&mut app, egui::Key::Z);
    assert!(
        driver::is_enabled(&app, "Rétablir"),
        "Cmd+Z undid the segment, so it can be put back"
    );

    driver::press_with(
        &mut app,
        egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        egui::Key::Z,
    );
    assert!(
        !driver::is_enabled(&app, "Rétablir"),
        "Cmd+Shift+Z put the segment back, so nothing is left to redo"
    );
    assert!(
        driver::is_enabled(&app, "Annuler"),
        "and the segment can be undone again"
    );
}

#[test]
fn shift_e_takes_matter_away_where_e_adds_it() {
    let mut app = driver::open("shift_e_takes_matter_away");
    driver::create_a_part(&mut app, "Chape");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at_sketch_point(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_the_button(&mut app, "1");

    driver::press_with(&mut app, egui::Modifiers::SHIFT, egui::Key::E);
    assert!(
        driver::is_on(&app, "Enlèvement de matière"),
        "{:?}",
        driver::on_screen(&app)
    );
    assert!(!driver::is_on(&app, "Ajout de matière"));

    driver::press(&mut app, egui::Key::E);
    assert!(driver::is_on(&app, "Ajout de matière"));
    assert!(!driver::is_on(&app, "Enlèvement de matière"));
}

#[test]
fn the_ellipse_tool_offers_a_field_at_the_cursor_for_its_second_axis() {
    let mut app = driver::open("an_ellipse_offers_its_fields");
    driver::create_a_part(&mut app, "Came");
    driver::start_a_sketch(&mut app);

    driver::start_an_ellipse(&mut app);

    assert!(
        driver::fields_at_the_cursor(&app) > 0,
        "nothing to type the second axis into, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn an_ellipse_drawn_with_three_clicks_becomes_a_step_of_the_sketch() {
    let mut app = driver::open("an_ellipse_becomes_a_step");
    driver::create_a_part(&mut app, "Came");
    driver::start_a_sketch(&mut app);

    driver::start_an_ellipse(&mut app);
    driver::click_at_sketch_point(&mut app, DVec2::ZERO);
    driver::press(&mut app, egui::Key::Escape);
    driver::open_the_history(&mut app);

    assert!(
        driver::on_screen(&app)
            .iter()
            .any(|label| label.ends_with(". Ellipse")),
        "the history names the ellipse as a numbered step of its own, showing {:?}",
        driver::on_screen(&app),
    );
}

/// A block raised off a sketch, and that sketch reopened from the Part panel, so
/// that the block stands between the eye and the plane being drawn on. Its
/// points, and the ones drawn behind it, are points of the sketch, where the
/// clicks the tests were first written with landed.
fn a_sketch_reopened_under_its_own_block(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Bloc");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at_sketch_point(&mut app, DVec2::new(-8.82, 44.59));
    driver::click_at_sketch_point(&mut app, DVec2::new(123.83, -52.69));
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_the_button(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at_sketch_point(&mut app, DVec2::new(59.12, -20.49));
    driver::click(&mut app, "Appliquer");
    driver::open_the_history(&mut app);
    driver::double_click(&mut app, "Esquisse 1");
    driver::click_exactly(&mut app, "Esquisse");
    app
}

/// The same block and sketch, the matter in front of the sketch set with the
/// view bar's menu to the state of that rank — none asked for the first, which
/// a sketch opens on.
fn reopened_after(rank: usize, test: &str) -> driver::App {
    let mut app = a_sketch_reopened_under_its_own_block(test);
    if rank > 0 {
        driver::click_in_the_bar(&mut app, "Matière devant");
        driver::click_the_button(&mut app, MATTER_IN_FRONT_IN_THE_MENU[rank]);
    }
    app
}

/// The three states as the view bar's menu reads them, in the same order.
const MATTER_IN_FRONT_IN_THE_MENU: [&str; 3] = ["cachée", "transparente", "opaque"];

const HIDDEN_SEE_THROUGH_OPAQUE: [&str; 3] = ["hidden", "see_through", "opaque"];

#[test]
fn a_click_lands_on_the_sketch_plane_whatever_the_matter_in_front() {
    for (clicks, state) in HIDDEN_SEE_THROUGH_OPAQUE.into_iter().enumerate() {
        let mut app = reopened_after(clicks, &format!("a_click_lands_{state}"));

        driver::click(&mut app, "Ligne (L)");
        driver::click_at_sketch_point(&mut app, DVec2::new(-9.26, 10.03));
        driver::click_at_sketch_point(&mut app, DVec2::new(43.98, -39.41));
        driver::press(&mut app, egui::Key::Escape);

        assert!(
            driver::on_screen(&app)
                .iter()
                .any(|label| label.ends_with(". Trait")),
            "{state}: two clicks on the plane behind the matter drew nothing, showing {:?}",
            driver::on_screen(&app),
        );
    }
}

#[test]
fn the_dimension_tool_takes_a_segment_behind_the_matter_whatever_its_state() {
    for (clicks, state) in HIDDEN_SEE_THROUGH_OPAQUE.into_iter().enumerate() {
        let mut app = reopened_after(clicks, &format!("the_dimension_takes_{state}"));
        driver::click(&mut app, "Ligne (L)");
        driver::click_at_sketch_point(&mut app, DVec2::new(-9.26, 10.03));
        driver::click_at_sketch_point(&mut app, DVec2::new(43.98, 10.03));
        driver::press(&mut app, egui::Key::Escape);

        driver::click(&mut app, "Cote (D)");
        let beside_the_segment =
            driver::sketch_point_on_screen(&app, DVec2::new(17.36, 10.03)) + egui::vec2(0.0, 2.0);
        driver::click_at(&mut app, beside_the_segment);
        driver::click_at_sketch_point(&mut app, DVec2::new(17.36, -16.59));

        assert!(
            driver::shows(&app, "Largeur"),
            "{state}: a click beside the segment did not take it, showing {:?}",
            driver::on_screen(&app),
        );
    }
}
