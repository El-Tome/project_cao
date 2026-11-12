//! The panel of the mirror and the two patterns, driven through the whole
//! application: what reaches the keyboard, and where the panel stands.
//!
//! Closes #321.
//! - `Entrée` with no field focused while the panel is open never closes the
//!   sketch — `enter_with_the_panel_open_does_not_close_the_sketch`
//! - `Entrée` that validates another field moves nothing in the panel —
//!   `enter_in_a_field_of_another_panel_leaves_the_copy_panel_alone`
//! - `Échap` closes the panel, nothing is laid, and what it held stays selected
//!   with the Selection tool in hand —
//!   `escape_hands_what_the_panel_held_to_the_selection_tool`
//! - while the writing cursor is in a field of the panel, no letter shortcut
//!   fires — `a_letter_typed_in_a_field_of_the_panel_takes_no_tool`
//! - a value that cannot be used refuses Validate alone: a click still fills
//!   Elements and the target — `a_broken_value_refuses_validate_and_nothing_else`
//! - it comes back where it was last left, one position shared by every tool
//!   that uses it — `the_panel_opens_where_it_was_last_left_whichever_copy_tool_is_taken`
//! - the place it is let go at is kept across restarts —
//!   `a_panel_let_go_somewhere_is_saved_there`, and survives the sketch being
//!   finished or undone away — `finishing_the_sketch_keeps_the_place_the_panel_was_left_at`,
//!   `undoing_past_the_sketch_keeps_the_place_the_panel_was_left_at`
//! - with nowhere to keep the settings, letting the panel go raises no error —
//!   `a_panel_let_go_with_nowhere_to_keep_settings_says_nothing`
//!
//! Closes #738 — the whole transcription is in
//! `crates/app/src/screens/viewport/input/copying/tests/steps.rs`; these are
//! the halves only the whole application holds, the keyboard's Escape and the
//! pointer that drags.
//! - ↵ validates the current step and moves on; on the last step it applies
//!   the pattern; Escape cancels. One test per case — Escape's case,
//!   `escape_on_a_step_of_the_circular_pattern_closes_its_panel_and_lays_nothing`
//! - The panel can still be dragged and keeps its place —
//!   `the_circular_pattern_s_panel_follows_its_title_and_reopens_where_it_was_let_go`

mod driver;

use cao_prefs::{Locations, Profiles};
use egui_kittest::kittest::{NodeT, Queryable};
use glam::DVec2;

/// Wide enough for the ribbon's patterns to be on screen.
const WIDE: egui::Vec2 = egui::vec2(1920.0, 800.0);

/// Well clear of the trait, to its right: a point of the sketch where nothing
/// is drawn. Where it was first written, brought down by twice 22.11 mm, out
/// of the band the pill a sketch shows at the top of the view covers (#727).
const ON_EMPTY_SPACE: DVec2 = DVec2::new(176.27, 68.61);

/// A part with one trait drawn on its sketch, the sketch still open.
fn a_trait_drawn(test: &str, settings: cao_prefs::Settings) -> driver::App {
    let mut app = driver::open_with(test, WIDE, settings);
    driver::create_a_part(&mut app, "Platine");
    driver::click(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, driver::ON_THE_YZ_PLANE);
    driver::draw_a_segment(&mut app);
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
    driver::click_at_sketch_point(&mut app, driver::ON_THE_SEGMENT);
    driver::press(&mut app, egui::Key::Enter);
    driver::click_at_sketch_point(&mut app, ON_EMPTY_SPACE);

    for pressed in 1..=6 {
        driver::press(&mut app, egui::Key::Enter);

        assert!(
            driver::a_sketch_is_open(&app),
            "Entrée number {pressed} moved through the panel, and the sketch is still open"
        );
    }
}

#[test]
fn escape_hands_what_the_panel_held_to_the_selection_tool() {
    let mut app = a_trait_drawn(
        "escape_hands_what_the_panel_held",
        cao_prefs::Settings::default(),
    );
    driver::click(&mut app, "Symétrie");
    driver::click_at_sketch_point(&mut app, driver::ON_THE_SEGMENT);
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
    assert!(
        driver::is_on(&app, "Sélection (S)"),
        "the Selection tool is in hand"
    );
    assert!(driver::a_sketch_is_open(&app), "the sketch stays open");
    driver::click(&mut app, "Symétrie");
    assert!(
        driver::shows(&app, "Éléments : 1"),
        "the trait picked stayed selected without its ends, and the mirror \
         starts with it alone: {:?}",
        driver::on_screen(&app)
    );
}

#[test]
fn enter_in_a_field_of_another_panel_leaves_the_copy_panel_alone() {
    let mut app = a_trait_drawn(
        "enter_in_a_field_of_another_panel",
        cao_prefs::Settings::default(),
    );
    driver::click_the_button(&mut app, "Variables");
    driver::click(&mut app, "Réseau circulaire");
    let waiting = "Réseau circulaire — étape 1 : éléments à copier";
    assert!(driver::says(&app, waiting), "{:?}", driver::said(&app));

    driver::type_into(&mut app, "nom", "w");
    driver::press(&mut app, egui::Key::Enter);
    driver::type_into(&mut app, "formule", "3");
    driver::press(&mut app, egui::Key::Enter);

    assert!(
        driver::says(&app, waiting),
        "Elements is still the slot waiting: {:?}",
        driver::said(&app)
    );
}

#[test]
fn a_broken_value_refuses_validate_and_nothing_else() {
    let mut app = a_trait_drawn(
        "a_broken_value_refuses_validate",
        cao_prefs::Settings::default(),
    );
    driver::click(&mut app, "Réseau circulaire");
    driver::click_at_sketch_point(&mut app, driver::ON_THE_SEGMENT);
    driver::press(&mut app, egui::Key::Enter);
    driver::type_over(&mut app, "90", "1+");

    driver::click(&mut app, "Centre :");
    driver::click_at_sketch_point(&mut app, driver::A_POINT_ABOVE_THE_ORIGIN);
    assert!(
        driver::shows(&app, "Centre : choisi"),
        "a click in the target slot still stores the centre: {:?}",
        driver::on_screen(&app)
    );
    driver::click(&mut app, "Éléments :");
    driver::click_at_sketch_point(&mut app, driver::ON_THE_SEGMENT);
    assert!(
        driver::shows(&app, "Éléments : 0"),
        "a click in Elements still takes the trait back out: {:?}",
        driver::on_screen(&app)
    );
    driver::click_at_sketch_point(&mut app, driver::ON_THE_SEGMENT);

    driver::click(&mut app, "Valider");

    assert!(
        driver::shows(&app, "Éléments : 3"),
        "Validate refused, so the panel still holds the trait and its two ends: {:?}",
        driver::on_screen(&app)
    );
    assert!(
        driver::says(&app, "La formule s'arrête avant sa dernière valeur"),
        "and says why: {:?}",
        driver::said(&app)
    );
}

#[test]
fn a_trait_clicked_into_the_panel_is_counted_with_its_two_ends() {
    let mut app = a_trait_drawn(
        "a_trait_clicked_into_the_panel",
        cao_prefs::Settings::default(),
    );
    driver::click(&mut app, "Symétrie");

    driver::click_at_sketch_point(&mut app, driver::ON_THE_SEGMENT);

    assert!(
        driver::shows(&app, "Éléments : 3"),
        "the trait and its two ends are held: {:?}",
        driver::on_screen(&app)
    );
}

#[test]
fn a_letter_typed_in_a_field_of_the_panel_takes_no_tool() {
    let mut app = a_trait_drawn(
        "a_letter_typed_in_the_panel",
        cao_prefs::Settings::default(),
    );
    driver::click(&mut app, "Réseau circulaire");
    driver::click_at_sketch_point(&mut app, driver::ON_THE_SEGMENT);
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
    // Low enough to lie inside the drawing area whatever the chrome above it
    // takes: a panel left above it is brought back down into it.
    let left_at = egui::pos2(900.0, 450.0);
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

/// The setting as the disk holds it, for the app opened under this test's name.
fn saved_place(test: &str) -> Option<[f32; 2]> {
    let root = std::env::temp_dir().join("cao-driver").join(test);
    let at = Locations {
        config: root.join("config"),
        data: root.join("data"),
        documents: Some(root.join("documents")),
    };
    Profiles::load(&cao_app::adapters::files::DiskFiles, &at)
        .active()
        .copy_panel_at
}

/// The mirror's panel taken by its title and let go further down and right.
fn drag_the_mirror_s_panel(app: &mut driver::App) {
    driver::click(app, "Symétrie");
    let title = title_of_the_panel(app, "Symétrie");
    driver::drag_and_drop(
        app,
        title.center(),
        title.center() + egui::vec2(150.0, 80.0),
    );
}

#[test]
fn finishing_the_sketch_keeps_the_place_the_panel_was_left_at() {
    let test = "finishing_the_sketch_keeps_the_place";
    let mut app = a_trait_drawn(test, cao_prefs::Settings::default());
    drag_the_mirror_s_panel(&mut app);
    let left_at = saved_place(test);
    assert!(
        left_at.is_some(),
        "the place is written down when the panel is let go"
    );

    driver::finish_the_sketch(&mut app);

    assert_eq!(saved_place(test), left_at);
}

#[test]
fn undoing_past_the_sketch_keeps_the_place_the_panel_was_left_at() {
    let test = "undoing_past_the_sketch_keeps_the_place";
    let mut app = a_trait_drawn(test, cao_prefs::Settings::default());
    drag_the_mirror_s_panel(&mut app);
    let left_at = saved_place(test);
    assert!(
        left_at.is_some(),
        "the place is written down when the panel is let go"
    );

    for _ in 0..3 {
        driver::press_with_command(&mut app, egui::Key::Z);
    }

    assert!(
        driver::says(&app, "Rien encore. Commencez par une esquisse."),
        "the sketch is undone away: {:?}",
        driver::said(&app)
    );
    assert_eq!(saved_place(test), left_at);
}

#[test]
fn a_panel_let_go_with_nowhere_to_keep_settings_says_nothing() {
    let root = std::env::temp_dir()
        .join("cao-driver")
        .join("a_panel_let_go_with_nowhere_to_keep");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("a place for the part");
    let (_, part) = cao_part::PartDocument::create_in(
        &cao_app::adapters::files::DiskFiles,
        &root,
        "Platine",
        chrono::Utc::now(),
    )
    .expect("the part is written");
    let mut app: driver::App = egui_kittest::Harness::builder()
        .with_size(WIDE)
        .with_max_steps(16)
        .build_eframe(move |cc| cao_app::app::CaoApp::new(cc, None, Some(part)));
    app.run();
    driver::click(&mut app, "Nouvelle esquisse");
    driver::click_at_world_point(&mut app, driver::ON_THE_YZ_PLANE);

    drag_the_mirror_s_panel(&mut app);

    let nowhere = "Ce poste n'offre aucun dossier où garder les réglages.";
    assert!(
        !driver::says(&app, nowhere) && !driver::shows(&app, nowhere),
        "nothing is ever written there, and nothing complains: {:?}",
        driver::said(&app)
    );
}
