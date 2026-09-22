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

mod driver;

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

    assert!(
        driver::shows(&app, "Équerre"),
        "the file list names the part that was just created, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn a_sketch_started_on_a_plane_names_that_plane_in_the_history() {
    let mut app = driver::open("a_sketch_names_its_plane");
    driver::create_a_part(&mut app, "Platine");

    driver::start_a_sketch(&mut app);
    driver::open_the_history(&mut app);

    assert!(
        driver::shows(&app, "Esquisse — Plan"),
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
    driver::click_at(&mut app, (760.0, 560.0));
    driver::press(&mut app, egui::Key::Escape);
    driver::open_the_history(&mut app);

    assert!(
        driver::on_screen(&app)
            .iter()
            .filter(|label| label.contains("Ellipse"))
            .count()
            > 1,
        "the history names the ellipse beside the tool's own button, showing {:?}",
        driver::on_screen(&app),
    );
}
