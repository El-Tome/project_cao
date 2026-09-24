//! The scale a part learns from the first value it is given, driven with no
//! window: what a unit is worth is read off the message the application says.
//!
//! Closes #423.
//! - a length typed while a shape is drawn, when it is the first value the
//!   part is given, sets the scale, and the shape is laid where it is drawn:
//!   the millimetres a unit is worth are what was typed over the length the
//!   field read on screen — `the_first_length_typed_on_a_line_is_worth_the_length_on_screen`

mod driver;

/// What the message on screen says a unit is worth, in millimetres.
fn a_unit_is_worth(app: &driver::App) -> Option<f64> {
    driver::said(app).iter().find_map(|said| {
        let (_, worth) = said.split_once("1 unité = ")?;
        worth.trim_end_matches(" mm").parse().ok()
    })
}

fn typed(app: &mut driver::App, digits: &str) {
    for digit in digits.chars() {
        let key = egui::Key::from_name(&digit.to_string()).expect("a digit key");
        driver::key(app, key, &digit.to_string());
    }
}

#[test]
fn the_first_length_typed_on_a_line_is_worth_the_length_on_screen() {
    let mut app = driver::open("the_first_length_typed_on_a_line");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::start_a_line(&mut app);
    let on_screen: f64 = driver::hints(&app)
        .first()
        .and_then(|hint| hint.parse().ok())
        .expect("the length field reads what the line measures");

    typed(&mut app, "100");
    driver::press(&mut app, egui::Key::Enter);

    let worth = a_unit_is_worth(&app).unwrap_or_else(|| {
        panic!(
            "no scale was said, and the screen says {:?}",
            driver::said(&app)
        )
    });
    let expected = 100.0 / on_screen;
    assert!(
        (worth - expected).abs() < expected * 1e-3,
        "the line read {on_screen} mm on screen when 100 was typed, so a unit is worth \
         {expected} mm; the part says {worth}",
    );
}
