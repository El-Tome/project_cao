//! The application driven with no window. `egui_kittest` builds `CaoApp` from a
//! `CreationContext` of its own, hands it no render state, and steps it frame by
//! frame; what the user would do with a hand is written below as gestures.

// Every test file that says `mod driver;` compiles this module afresh, and
// uses the handful of gestures it needs. A gesture another file relies on is
// dead code here, which is a fact about the compilation unit rather than about
// the repository.
#![allow(dead_code)]

use cao_app::app::CaoApp;
use cao_prefs::Locations;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

pub const SCREEN: (f32, f32) = (1280.0, 800.0);

/// Inside the drawing area, which starts to the right of the file list and the
/// panel beside it. Read off a rendered frame of that same screen size.
pub const ON_ONE_OF_THE_BASE_PLANES: (f32, f32) = (700.0, 545.0);
pub const A_POINT_ABOVE_THE_ORIGIN: (f32, f32) = (700.0, 400.0);
pub const A_POINT_BELOW_AND_RIGHT: (f32, f32) = (1000.0, 620.0);

pub type App = Harness<'static, CaoApp>;

pub fn open(test: &str) -> App {
    open_on(test, SCREEN)
}

/// The same on a screen of another size — wide enough, say, for the whole
/// ribbon of the sketch to fit, patterns included.
pub fn open_on(test: &str, (width, height): (f32, f32)) -> App {
    let root = std::env::temp_dir().join("cao-driver").join(test);
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("documents")).expect("a place to keep the parts");
    let at = Locations {
        config: root.join("config"),
        data: root.join("data"),
        documents: Some(root.join("documents")),
    };

    let mut app = Harness::builder()
        .with_size(egui::vec2(width, height))
        // What a refusal names blinks for three seconds, asking for frame
        // after frame all along; kittest's four steps of a quarter second
        // would take that for a screen that never settles.
        .with_max_steps(16)
        .build_eframe(move |cc| {
            assert!(
                cc.wgpu_render_state.is_none(),
                "no renderer was asked for, so these tests run without a GPU adapter",
            );
            CaoApp::new(cc, Some(at), None)
        });
    app.run();
    app
}

pub fn click(app: &mut App, label: &str) {
    app.get_by_label_contains(label).click();
    app.run();
}

/// The one widget whose label is exactly this — for a tool whose name is the
/// start of its own mode group's, as `Ellipse` is of `Ellipses`.
pub fn click_exactly(app: &mut App, label: &str) {
    app.get_by_label(label).click();
    app.run();
}

pub fn click_at(app: &mut App, (x, y): (f32, f32)) {
    let pos = egui::pos2(x, y);
    app.hover_at(pos);
    app.run();
    for pressed in [true, false] {
        app.event(egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        });
        app.run();
    }
}

pub fn type_into_the_only_field(app: &mut App, text: &str) {
    app.get_by_role(egui::accesskit::Role::TextInput).focus();
    app.run();
    app.event(egui::Event::Text(text.to_owned()));
    app.run();
}

pub fn press(app: &mut App, key: egui::Key) {
    app.key_press(key);
    app.run();
}

pub fn on_screen(app: &App) -> Vec<String> {
    app.get_all_by(|_| true)
        .filter_map(|node| node.accesskit_node().label())
        .collect()
}

pub fn is_enabled(app: &App, label: &str) -> bool {
    !app.get_by_label_contains(label)
        .accesskit_node()
        .is_disabled()
}

pub fn shows(app: &App, text: &str) -> bool {
    on_screen(app).iter().any(|label| label.contains(text))
}

pub fn create_a_part(app: &mut App, named: &str) {
    type_into_the_only_field(app, named);
    click(app, "Créer");
}

pub fn start_a_sketch(app: &mut App) {
    click(app, "Nouvelle esquisse");
    click_at(app, ON_ONE_OF_THE_BASE_PLANES);
}

pub fn draw_a_segment(app: &mut App) {
    click(app, "Ligne (L)");
    click_at(app, A_POINT_ABOVE_THE_ORIGIN);
    click_at(app, A_POINT_BELOW_AND_RIGHT);
    press(app, egui::Key::Escape);
}

/// The panel is a toggle and the part decides which one opens with it, so a
/// test that clicks blindly closes the panel it wanted half the time.
pub fn open_the_history(app: &mut App) {
    if !shows(app, "Pièce vide") {
        click(app, "Historique");
    }
}

/// The ellipse tool, its centre and the end of its first axis clicked, the
/// cursor then left where the second axis would reach.
pub fn start_an_ellipse(app: &mut App) {
    click_exactly(app, "Ellipse");
    click_at(app, A_POINT_ABOVE_THE_ORIGIN);
    click_at(app, A_POINT_BELOW_AND_RIGHT);
    app.hover_at(egui::pos2(760.0, 560.0));
    app.run();
}

pub fn fields_at_the_cursor(app: &App) -> usize {
    app.query_all_by_role(egui::accesskit::Role::TextInput)
        .count()
}

/// The button of that name, for a word a heading or a label carries too — the
/// ribbon's `Variables` beside the panel it opens.
pub fn click_the_button(app: &mut App, label: &str) {
    app.query_all_by_label(label)
        .find(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
        .unwrap_or_else(|| panic!("a button named {label}"))
        .click();
    app.run();
}

/// Types into the field waiting for this, the way a click then the keyboard
/// would.
pub fn type_into(app: &mut App, waiting_for: &str, text: &str) {
    app.get_all_by_role(egui::accesskit::Role::TextInput)
        .find(|node| node.accesskit_node().placeholder() == Some(waiting_for))
        .unwrap_or_else(|| panic!("a field waiting for {waiting_for}"))
        .focus();
    app.run();
    app.event(egui::Event::Text(text.to_owned()));
    app.run();
}

/// What every field on screen holds, in the order they are drawn.
pub fn fields(app: &App) -> Vec<String> {
    app.query_all_by_role(egui::accesskit::Role::TextInput)
        .filter_map(|node| node.accesskit_node().value())
        .collect()
}

/// A key pressed and its character typed, the way a keyboard sends both.
pub fn key(app: &mut App, key: egui::Key, text: &str) {
    app.key_press(key);
    app.event(egui::Event::Text(text.to_owned()));
    app.run();
    app.run();
}

/// Starts a line with one click, the cursor then left further along.
pub fn start_a_line(app: &mut App) {
    click(app, "Ligne (L)");
    click_at(app, A_POINT_ABOVE_THE_ORIGIN);
    app.hover_at(egui::pos2(900.0, 500.0));
    app.run();
    app.run();
}

/// Types over whatever the field waiting for this holds, the way selecting
/// all of it first would.
pub fn type_over(app: &mut App, holding: &str, text: &str) {
    app.get_all_by_role(egui::accesskit::Role::TextInput)
        .find(|node| node.accesskit_node().value().as_deref() == Some(holding))
        .unwrap_or_else(|| panic!("a field holding {holding}"))
        .focus();
    app.run();
    app.event(egui::Event::Key {
        key: egui::Key::A,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::COMMAND,
    });
    app.event(egui::Event::Text(text.to_owned()));
    app.run();
}

/// A key pressed with the command key held, the way a shortcut is.
pub fn press_with_command(app: &mut App, key: egui::Key) {
    app.event(egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::COMMAND,
    });
    app.run();
    app.run();
}

/// Whether a line of text on screen says this — a message, a heading — which
/// the tree carries as the value of a label rather than as its name.
pub fn says(app: &App, text: &str) -> bool {
    app.query_all_by_role(egui::accesskit::Role::Label)
        .filter_map(|node| node.accesskit_node().value())
        .any(|said| said.contains(text))
}

/// Takes what lies under `from` and lets it go over `to`, the pointer
/// travelling from one to the other as a hand would. The button is let go
/// with the pointer still there, so whatever is under it hears the drop.
pub fn drag_and_drop(app: &mut App, from: egui::Pos2, to: egui::Pos2) {
    app.hover_at(from);
    app.run();
    app.drag_at(from);
    app.run();
    app.hover_at(from + egui::vec2(12.0, 12.0));
    app.run();
    app.hover_at(to);
    app.run();
    app.event(egui::Event::PointerButton {
        pos: to,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    });
    app.run();
}

/// What every field on screen shows while nothing is typed in it — the
/// readout of what the cursor is doing — in the order they are drawn.
pub fn hints(app: &App) -> Vec<String> {
    app.query_all_by_role(egui::accesskit::Role::TextInput)
        .filter_map(|node| node.accesskit_node().placeholder().map(str::to_owned))
        .collect()
}

/// Every line of text on screen that says something — a message, a heading —
/// which the tree carries as the value of a label.
pub fn said(app: &App) -> Vec<String> {
    app.query_all_by_role(egui::accesskit::Role::Label)
        .filter_map(|node| node.accesskit_node().value())
        .collect()
}
