//! The measure tool driven through the whole application with no window: the
//! button is taken off the toolbar the user sees, a trait drawn a moment
//! earlier is clicked, and the history is read back out of the accessibility
//! tree to show that nothing was written.
//!
//! This is the half the unit tests cannot reach. Each piece of the wiring — a
//! `Command`, a `Tool`, a default toolbar entry, a label — was asserted where
//! it lives, and a tool can still be unreachable with every one of them green:
//! that is exactly what #412 found for the ellipse, where the list letting a
//! tool show its fields had never heard of it.
//!
//! Closes #176.
//! - the Mesurer button is on the toolbar and takes the tool in hand —
//!   `the_measure_button_is_on_the_toolbar_and_lights_up_when_taken`
//! - measuring writes nothing to the history —
//!   `measuring_a_trait_leaves_the_history_with_nothing_but_the_trait`
//! - the smart dimension tool still places real dimensions, which is what a
//!   measure must not become —
//!   `the_dimension_tool_still_writes_where_the_measure_tool_does_not`
//! - what the measure says is shown on the canvas — no test: the driver's tree
//!   carries buttons and fields, not painted prose, so the label is asserted
//!   through `wording/measure/tests.rs` and its drawing through
//!   `render/reading/tests.rs`

mod driver;

/// A part with a sketch and one trait in it, the history panel open.
fn a_part_with_a_trait(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Équerre");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::open_the_history(&mut app);
    app
}

#[test]
fn the_measure_button_is_on_the_toolbar_and_lights_up_when_taken() {
    let mut app = a_part_with_a_trait("the_measure_button_is_on_the_toolbar");

    assert!(
        driver::shows(&app, "Mesurer"),
        "the toolbar offers the measure tool, showing {:?}",
        driver::on_screen(&app),
    );

    driver::click(&mut app, "Mesurer");

    assert!(
        driver::is_enabled(&app, "Mesurer"),
        "the tool stays offered once taken, showing {:?}",
        driver::on_screen(&app),
    );
}

#[test]
fn measuring_a_trait_leaves_the_history_with_nothing_but_the_trait() {
    let mut app = a_part_with_a_trait("measuring_writes_nothing");
    let before = steps(&app);

    driver::click(&mut app, "Mesurer");
    driver::click_at(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(&mut app, driver::A_POINT_BELOW_AND_RIGHT);

    assert_eq!(
        steps(&app),
        before,
        "a measure is a way of looking: the history says exactly what it said \
         before, showing {:?}",
        driver::on_screen(&app),
    );
    assert_eq!(
        before,
        vec!["1. Esquisse — Plan YZ".to_string(), "2. Trait".to_string()],
        "the sketch and the trait are the two steps a measure had better not \
         join, and this says so out loud rather than comparing two empties",
    );
}

/// The numbered steps of the history, as the panel lists them.
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
fn the_dimension_tool_still_writes_where_the_measure_tool_does_not() {
    let mut app = a_part_with_a_trait("the_dimension_tool_still_writes");

    driver::click(&mut app, "Cote");
    driver::click_at(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(&mut app, driver::A_POINT_BELOW_AND_RIGHT);

    assert!(
        driver::is_enabled(&app, "Annuler"),
        "the dimension tool records what it places, and sharing its aim with \
         the measure tool must not have taken that away, showing {:?}",
        driver::on_screen(&app),
    );
}
