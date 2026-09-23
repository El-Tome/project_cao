//! The application driven with no window. `egui_kittest` builds `CaoApp` from a
//! `CreationContext` of its own, hands it no render state, and steps it frame by
//! frame; what the user would do with a hand is written below as gestures.

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
    let root = std::env::temp_dir().join("cao-driver").join(test);
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("documents")).expect("a place to keep the parts");
    let at = Locations {
        config: root.join("config"),
        data: root.join("data"),
        documents: Some(root.join("documents")),
    };

    let mut app = Harness::builder()
        .with_size(egui::vec2(SCREEN.0, SCREEN.1))
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
    app.get_all_by_role(egui::accesskit::Role::TextInput)
        .count()
}
