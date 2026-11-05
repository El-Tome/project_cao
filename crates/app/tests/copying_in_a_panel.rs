//! The panel of the mirror and the two patterns, driven through the whole
//! application: what reaches the keyboard, and where the panel stands.
//!
//! Closes #321.
//! - `Entrée` with no field focused while the panel is open never closes the
//!   sketch — `enter_with_the_panel_open_does_not_close_the_sketch`
//! - `Échap` closes the panel, and nothing is laid: closing it is all `Échap`
//!   does — `escape_closes_the_panel`
//! - while the writing cursor is in a field of the panel, no letter shortcut
//!   fires — `a_letter_typed_in_a_field_of_the_panel_takes_no_tool`
//! - it comes back where it was last left, one position shared by every tool
//!   that uses it — `the_panel_opens_where_it_was_last_left_whichever_copy_tool_is_taken`
//! - the place it is let go at is kept across restarts —
//!   `a_panel_let_go_somewhere_is_saved_there`

mod driver;

use cao_prefs::{Locations, Profiles};
use egui_kittest::kittest::{NodeT, Queryable};

/// Wide enough for the ribbon's patterns to be on screen. The canvas is as
/// much wider, so what is drawn on it moves right by half of that.
const WIDE: (f32, f32) = (1920.0, 800.0);

fn right((x, y): (f32, f32)) -> (f32, f32) {
    (x + 320.0, y)
}

const ON_THE_TRAIT: (f32, f32) = (850.0, 510.0);
const ON_EMPTY_SPACE: (f32, f32) = (1100.0, 300.0);

/// A part with one trait drawn on its sketch, the sketch still open.
fn a_trait_drawn(test: &str, settings: cao_prefs::Settings) -> driver::App {
    let mut app = driver::open_with(test, WIDE, settings);
    driver::create_a_part(&mut app, "Platine");
    driver::click(&mut app, "Nouvelle esquisse");
    driver::click_at(&mut app, right(driver::ON_ONE_OF_THE_BASE_PLANES));
    driver::click(&mut app, "Ligne (L)");
    driver::click_at(&mut app, right(driver::A_POINT_ABOVE_THE_ORIGIN));
    driver::click_at(&mut app, right(driver::A_POINT_BELOW_AND_RIGHT));
    driver::press(&mut app, egui::Key::Escape);
    app
}

/// Where the panel's title stands: the tool's name, as a line of text rather
/// than as the ribbon's button of that name.
fn title_of_the_panel(app: &driver::App, tool: &str) -> egui::Rect {
    app.query_all_by_role(egui::accesskit::Role::Label)
        .find(|node| node.accesskit_node().value().as_deref() == Some(tool))
        .unwrap_or_else(|| panic!("a panel titled {tool} is open: {:?}", driver::said(app)))
        .rect()
}

#[test]
fn enter_with_the_panel_open_does_not_close_the_sketch() {
    let mut app = a_trait_drawn("enter_with_the_panel_open", cao_prefs::Settings::default());
    driver::click(&mut app, "Réseau circulaire");
    driver::click_at(&mut app, right(ON_THE_TRAIT));
    driver::press(&mut app, egui::Key::Enter);
    driver::click_at(&mut app, right(ON_EMPTY_SPACE));

    for pressed in 1..=6 {
        driver::press(&mut app, egui::Key::Enter);

        assert!(
            driver::shows(&app, "Terminer"),
            "Entrée number {pressed} moved through the panel, and the sketch is still open"
        );
    }
}

#[test]
fn escape_closes_the_panel() {
    let mut app = a_trait_drawn("escape_closes_the_panel", cao_prefs::Settings::default());
    driver::click(&mut app, "Symétrie");
    driver::click_at(&mut app, right(ON_THE_TRAIT));
    assert!(
        driver::shows(&app, "Valider"),
        "the panel is open from the moment the tool is taken: {:?}",
        driver::on_screen(&app)
    );

    driver::press(&mut app, egui::Key::Escape);

    assert!(
        !driver::shows(&app, "Valider"),
        "the panel is closed: {:?}",
        driver::on_screen(&app)
    );
    assert!(driver::shows(&app, "Terminer"), "the sketch stays open");
}

#[test]
fn a_letter_typed_in_a_field_of_the_panel_takes_no_tool() {
    let mut app = a_trait_drawn(
        "a_letter_typed_in_the_panel",
        cao_prefs::Settings::default(),
    );
    driver::click(&mut app, "Réseau circulaire");
    driver::click_at(&mut app, right(ON_THE_TRAIT));
    driver::press(&mut app, egui::Key::Enter);
    app.get_all_by_role(egui::accesskit::Role::TextInput)
        .find(|node| node.accesskit_node().value().as_deref() == Some("90"))
        .unwrap_or_else(|| panic!("the step's field: {:?}", driver::fields(&app)))
        .focus();
    app.run();

    driver::key(&mut app, egui::Key::L, "l");

    assert!(
        !driver::is_on(&app, "Ligne (L)"),
        "the L typed into the field did not take the line tool"
    );
    let the_pattern_s_button = app
        .query_all_by_label("Réseau circulaire")
        .find(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
        .expect("the ribbon's button for the circular pattern");
    assert_eq!(
        the_pattern_s_button.accesskit_node().toggled(),
        Some(egui::accesskit::Toggled::True),
        "the circular pattern is still the tool in hand"
    );
}

#[test]
fn the_panel_opens_where_it_was_last_left_whichever_copy_tool_is_taken() {
    let left_at = egui::pos2(900.0, 200.0);
    let settings = cao_prefs::Settings {
        copy_panel_at: Some([left_at.x, left_at.y]),
        ..cao_prefs::Settings::default()
    };
    let mut app = a_trait_drawn("the_panel_opens_where_it_was_last_left", settings);

    driver::click(&mut app, "Symétrie");
    let mirror = title_of_the_panel(&app, "Symétrie").min;
    driver::press(&mut app, egui::Key::Escape);
    driver::click(&mut app, "Réseau circulaire");
    let pattern = title_of_the_panel(&app, "Réseau circulaire").min;

    assert!(
        (mirror - pattern).length() <= 1.0,
        "one place for every tool: the mirror's at {mirror:?}, the pattern's at {pattern:?}"
    );
    assert!(
        (mirror - left_at).length() <= 40.0,
        "the title stands at the top of the panel left at {left_at:?}, here {mirror:?}"
    );
}

#[test]
fn a_panel_let_go_somewhere_is_saved_there() {
    let test = "a_panel_let_go_somewhere_is_saved_there";
    let mut app = a_trait_drawn(test, cao_prefs::Settings::default());
    driver::click(&mut app, "Symétrie");
    let title = title_of_the_panel(&app, "Symétrie");
    let moved = egui::vec2(150.0, 80.0);

    driver::drag_and_drop(&mut app, title.center(), title.center() + moved);

    let now = title_of_the_panel(&app, "Symétrie").min;
    assert!(
        (now - (title.min + moved)).length() <= 2.0,
        "the panel follows its title to {:?}, and stands at {now:?}",
        title.min + moved
    );
    let root = std::env::temp_dir().join("cao-driver").join(test);
    let at = Locations {
        config: root.join("config"),
        data: root.join("data"),
        documents: Some(root.join("documents")),
    };
    let saved = Profiles::load(&cao_app::adapters::files::DiskFiles, &at)
        .active()
        .copy_panel_at
        .expect("the place it was let go at is written down");
    let saved = egui::pos2(saved[0], saved[1]);
    assert!(
        (saved - now).length() <= 40.0,
        "what is saved is where the panel now stands: {saved:?}, its title at {now:?}"
    );
}
