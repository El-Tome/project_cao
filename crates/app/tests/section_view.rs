//! The section view driven through the whole application: its button, its
//! panel, and what turns it off.
//!
//! Closes #193 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/tests.rs`; these bullets are the
//! ones only the whole application shows.
//! - the « Vue en coupe » button opens a floating panel, and closing the
//!   panel, by the button again or by Échap, turns the view off —
//!   `the_section_view_button_opens_its_panel_and_closing_it_turns_the_view_off`
//! - starting a sketch turns the section view off —
//!   `starting_a_sketch_turns_the_section_view_off`

mod driver;

use egui_kittest::kittest::{NodeT, Queryable};

/// A block raised off a rectangle drawn on a plane of the origin.
fn a_block(test: &str) -> driver::App {
    let mut app = driver::open(test);
    driver::create_a_part(&mut app, "Bloc");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Rectangle (R)");
    driver::click_at(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    driver::click_at(&mut app, driver::A_POINT_BELOW_AND_RIGHT);
    driver::click_exactly(&mut app, "Extrusion");
    driver::click_exactly(&mut app, "1");
    driver::click(&mut app, "Ajout de matière");
    driver::click_at(&mut app, (850.0, 510.0));
    driver::click(&mut app, "Appliquer");
    app
}

fn a_button_reads(app: &driver::App, label: &str) -> bool {
    app.query_all_by_role(egui::accesskit::Role::Button)
        .filter_map(|node| node.accesskit_node().label())
        .any(|read| read.starts_with(label))
}

/// The ribbon's button that reads this, whichever tab it sits under.
fn click_in_the_ribbon(app: &mut driver::App, label: &str) {
    for tab in ["Esquisse", "Extrusion", "Export"] {
        if a_button_reads(app, label) {
            break;
        }
        driver::click_the_button(app, tab);
    }
    let node = app
        .query_all_by_role(egui::accesskit::Role::Button)
        .find(|node| {
            node.accesskit_node()
                .label()
                .is_some_and(|read| read.starts_with(label))
        })
        .unwrap_or_else(|| {
            panic!(
                "no tab of the ribbon offers « {label} », showing {:?}",
                driver::on_screen(app)
            )
        });
    node.click();
    app.run();
}

fn the_panel_is_open(app: &driver::App) -> bool {
    a_button_reads(app, "Inverser")
}

#[test]
fn the_section_view_button_opens_its_panel_and_closing_it_turns_the_view_off() {
    let mut app = a_block("the_section_view_button_opens_its_panel");

    click_in_the_ribbon(&mut app, "Vue en coupe");
    let opened = the_panel_is_open(&app);
    let offers_a_face = driver::shows(&app, "Face cliquée");
    click_in_the_ribbon(&mut app, "Vue en coupe");
    let closed_by_the_button = !the_panel_is_open(&app);
    click_in_the_ribbon(&mut app, "Vue en coupe");
    let opened_again = the_panel_is_open(&app);
    driver::press(&mut app, egui::Key::Escape);
    let closed_by_escape = !the_panel_is_open(&app);

    assert!(
        opened,
        "« Vue en coupe » opens no panel with « Inverser », showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        offers_a_face,
        "the panel offers no « Face cliquée » to cut along"
    );
    assert!(
        closed_by_the_button,
        "« Vue en coupe » clicked again leaves its panel open"
    );
    assert!(
        opened_again,
        "« Vue en coupe » does not open its panel a second time"
    );
    assert!(closed_by_escape, "Échap leaves the section's panel open");
}

#[test]
fn starting_a_sketch_turns_the_section_view_off() {
    let mut app = a_block("starting_a_sketch_turns_the_section_view_off");
    click_in_the_ribbon(&mut app, "Vue en coupe");
    assert!(
        the_panel_is_open(&app),
        "« Vue en coupe » opens no panel, showing {:?}",
        driver::on_screen(&app),
    );

    click_in_the_ribbon(&mut app, "Nouvelle esquisse");
    driver::click_at(&mut app, driver::ON_ONE_OF_THE_BASE_PLANES);

    assert!(
        driver::shows(&app, "Terminer"),
        "the click started no sketch, showing {:?}",
        driver::on_screen(&app),
    );
    assert!(
        !the_panel_is_open(&app),
        "a sketch started, the section view is still on over the sketch's own cut",
    );
}
