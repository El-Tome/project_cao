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
use egui::accesskit::Action;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable, by};
use glam::{DVec2, DVec3};

pub const SCREEN: (f32, f32) = (1280.0, 800.0);

/// A point of the drawing area given from its centre, right and down, in
/// pixels of a canvas as tall as the one the points are written for — not a
/// point of the screen. A type of its own, so that handing one to a gesture
/// that takes a point of the screen does not compile.
#[derive(Clone, Copy, Debug)]
pub struct FromTheCentre(pub f32, pub f32);

/// On the YZ plane, as a new part shows the three base planes.
pub const ON_ONE_OF_THE_BASE_PLANES: FromTheCentre = FromTheCentre(-197.3, 59.5);

/// The height of the drawing area the points given from its centre are written
/// for: the canvas a sketch showed on a 1280×800 screen before #724 dressed the
/// chrome, which is where most of them were first read off a rendered frame.
const CANVAS_HEIGHT_THE_POINTS_ARE_WRITTEN_FOR: f32 = 612.0;

/// Inside the drawing area, which starts to the right of the file list and the
/// panel beside it. Read off a rendered frame of that same screen size, so they
/// move with the chrome around the canvas, as the plane above no longer does.
pub const A_POINT_ABOVE_THE_ORIGIN: (f32, f32) = (700.0, 400.0);
pub const A_POINT_BELOW_AND_RIGHT: (f32, f32) = (1000.0, 620.0);

pub type App = Harness<'static, CaoApp>;

pub fn open(test: &str) -> App {
    open_on(test, SCREEN)
}

/// The same on a screen of another size — wide enough, say, for the whole
/// ribbon of the sketch to fit, patterns included.
pub fn open_on(test: &str, size: (f32, f32)) -> App {
    open_with(test, size, cao_prefs::Settings::default())
}

/// The same, with these settings already saved, as a user who had set them
/// before would open it.
pub fn open_with(test: &str, (width, height): (f32, f32), settings: cao_prefs::Settings) -> App {
    let root = std::env::temp_dir().join("cao-driver").join(test);
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("documents")).expect("a place to keep the parts");
    let at = locations_of(test);
    let mut profiles = cao_prefs::Profiles::default();
    *profiles.active_mut() = settings;
    profiles
        .save(&cao_app::adapters::files::DiskFiles, &at)
        .expect("the settings are written where the app reads them");

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

/// Where the application opened for `test` keeps what it writes.
fn locations_of(test: &str) -> Locations {
    let root = std::env::temp_dir().join("cao-driver").join(test);
    Locations {
        config: root.join("config"),
        data: root.join("data"),
        documents: Some(root.join("documents")),
    }
}

/// The settings the application opened for `test` has written down, as the
/// next start would read them.
pub fn saved_settings(test: &str) -> cao_prefs::Settings {
    cao_prefs::Profiles::load(&cao_app::adapters::files::DiskFiles, &locations_of(test))
        .active()
        .clone()
}

pub fn click(app: &mut App, label: &str) {
    let node = app.get_by_label_contains(label);
    within_reach(label, node.rect().center(), app.ctx.content_rect());
    node.click();
    app.run();
}

/// The one widget whose label is exactly this — for a tool whose name is the
/// start of its own mode group's, as `Ellipse` is of `Ellipses`.
pub fn click_exactly(app: &mut App, label: &str) {
    let node = app.get_by_label(label);
    within_reach(label, node.rect().center(), app.ctx.content_rect());
    node.click();
    app.run();
}

/// A widget laid out past the edge of the screen is still in the accessibility
/// tree, enabled and uniquely labelled, but the pointer events a click sends at
/// its centre land on nothing — the gesture passes quietly and the tool never
/// takes. Saying so here is what tells that apart from a button that is broken.
fn within_reach(label: &str, centre: egui::Pos2, screen: egui::Rect) {
    assert!(
        screen.contains(centre),
        "{label} sits at {centre:?}, off the {screen:?} screen, where a click \
         reaches nothing: open the application wider, with `open_on`",
    );
}

/// The drawing area, as the accessibility tree has it: the largest node that
/// takes a click and carries no name. `egui` names every button, field and
/// label; the canvas is the one surface it has nothing to call. A modal's
/// backdrop takes a click and carries no name too, but it covers the whole
/// screen, which the canvas, with the chrome around it, never does.
pub fn canvas(app: &App) -> egui::Rect {
    let screen = app.ctx.content_rect();
    app.query_all(
        by().predicate(|node| node.data().supports_action(Action::Click) && node.label().is_none()),
    )
    .map(|node| node.rect())
    .filter(|rect| !rect.contains_rect(screen))
    .max_by(|one, other| one.area().total_cmp(&other.area()))
    .expect("no drawing area on screen: no part is open")
}

/// Where a point given from the centre of the drawing area stands on screen
/// now. The view is centred on the canvas and its field of view is vertical,
/// so scaled by the canvas's height the point stays on the same spot of the
/// scene, however much room the chrome around it takes. A point that lands
/// outside the canvas would be a click on the chrome, so it is refused here,
/// in the spirit of `within_reach`.
pub fn on_canvas(app: &App, FromTheCentre(right, down): FromTheCentre) -> egui::Pos2 {
    let canvas = canvas(app);
    let scale = canvas.height() / CANVAS_HEIGHT_THE_POINTS_ARE_WRITTEN_FOR;
    let at = canvas.center() + egui::vec2(right, down) * scale;
    assert!(
        canvas.contains(at),
        "({right}, {down}) from the centre of the drawing area lands at {at:?}, \
         outside the canvas {canvas:?}, where a click reaches the chrome instead",
    );
    at
}

pub fn click_on_canvas(app: &mut App, from_its_centre: FromTheCentre) {
    let at = on_canvas(app, from_its_centre);
    click_at(app, (at.x, at.y));
}

/// Where a point of the world stands on screen, as the last frame drew it:
/// through the application's own camera and canvas, so that it follows the
/// view wherever the chrome around it puts it. A reach in pixels is added to
/// what this returns, on screen, after the projection.
pub fn world_point_on_screen(app: &App, point: DVec3) -> egui::Pos2 {
    let at = app.state().where_the_view_shows(point).unwrap_or_else(|| {
        panic!(
            "the view shows no {point:?} of the world: no part is drawn yet, \
             or the point stands behind the eye"
        )
    });
    inside_the_canvas(app, at, &format!("{point:?} of the world"))
}

/// The same for a point of the plane of the sketch being drawn, given in that
/// sketch's own coordinates.
pub fn sketch_point_on_screen(app: &App, point: DVec2) -> egui::Pos2 {
    let at = app
        .state()
        .where_the_view_shows_the_sketch_point(point)
        .unwrap_or_else(|| {
            panic!(
                "the view shows no {point:?} of a sketch: no sketch is open, \
                 or the point stands behind the eye"
            )
        });
    inside_the_canvas(app, at, &format!("{point:?} of the sketch"))
}

pub fn click_at_world_point(app: &mut App, point: DVec3) {
    let at = world_point_on_screen(app, point);
    click_at(app, (at.x, at.y));
}

pub fn click_at_sketch_point(app: &mut App, point: DVec2) {
    let at = sketch_point_on_screen(app, point);
    click_at(app, (at.x, at.y));
}

/// A point of the drawing the view puts outside the canvas would be a click
/// on the chrome, so it is refused here, in the spirit of `within_reach`.
fn inside_the_canvas(app: &App, at: egui::Pos2, what: &str) -> egui::Pos2 {
    let canvas = canvas(app);
    assert!(
        canvas.contains(at),
        "{what} lands at {at:?}, outside the canvas {canvas:?}, where a click \
         reaches the chrome instead",
    );
    at
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

pub fn is_on(app: &App, label: &str) -> bool {
    app.get_by_label_contains(label).accesskit_node().toggled()
        == Some(egui::accesskit::Toggled::True)
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
    click_on_canvas(app, ON_ONE_OF_THE_BASE_PLANES);
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
    let node = app
        .query_all_by_label(label)
        .find(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
        .unwrap_or_else(|| panic!("a button named {label}"));
    within_reach(label, node.rect().center(), app.ctx.content_rect());
    node.click();
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
    press_with(app, egui::Modifiers::COMMAND, key);
}

pub fn press_with(app: &mut App, modifiers: egui::Modifiers, key: egui::Key) {
    app.event(egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
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

/// Every right-angle mark painted on the last frame: what it says, where, in
/// what colour.
pub fn right_angle_marks(app: &App) -> Vec<(String, egui::Pos2, egui::Color32)> {
    fn walk(shape: &egui::epaint::Shape, out: &mut Vec<(String, egui::Pos2, egui::Color32)>) {
        match shape {
            egui::epaint::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            egui::epaint::Shape::Text(text) if text.galley.text() == "|_" => out.push((
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

/// A click with these modifiers held, from the press to the release.
pub fn click_with(app: &mut App, at: egui::Pos2, modifiers: egui::Modifiers) {
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

/// The field a label names: the one it is attached to, or else the first drawn
/// to its right on the same row — a panel lays a label and its field side by
/// side.
pub fn the_field_beside<'tree>(app: &'tree App, label: &str) -> egui_kittest::Node<'tree> {
    use egui::accesskit::Role;

    if let Some(field) = app
        .query_all_by_role(Role::TextInput)
        .find(|field| field.accesskit_node().label().as_deref() == Some(label))
    {
        return field;
    }
    let named = app
        .get_all_by(|_| true)
        .find(|node| {
            let node = node.accesskit_node();
            node.role() != Role::TextInput
                && (node.label().as_deref() == Some(label)
                    || node.value().as_deref() == Some(label))
        })
        .unwrap_or_else(|| panic!("nothing on screen reads « {label} »: {:?}", on_screen(app)));
    let row = named.rect();
    app.query_all_by_role(Role::TextInput)
        .filter(|field| {
            let at = field.rect();
            at.min.x >= row.max.x - 1.0 && at.min.y <= row.center().y && row.center().y <= at.max.y
        })
        .min_by(|a, b| a.rect().min.x.total_cmp(&b.rect().min.x))
        .unwrap_or_else(|| panic!("no field stands beside « {label} »"))
}

/// What the field beside a label holds.
pub fn field_beside(app: &App, label: &str) -> String {
    the_field_beside(app, label)
        .accesskit_node()
        .value()
        .unwrap_or_default()
}

/// Types over the field beside a label, the way selecting all of it first
/// would.
pub fn type_beside(app: &mut App, label: &str, text: &str) {
    the_field_beside(app, label).focus();
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
