//! The application driven with no window. `egui_kittest` builds `CaoApp` from a
//! `CreationContext` of its own, hands it no render state, and steps it frame by
//! frame; what the user would do with a hand is written below as gestures.

// Every test file that says `mod driver;` compiles this module afresh, and
// uses the handful of gestures it needs. A gesture another file relies on is
// dead code here, which is a fact about the compilation unit rather than about
// the repository.
#![allow(dead_code)]

use std::path::PathBuf;

use cao_app::app::CaoApp;
use cao_prefs::Locations;
use egui::accesskit::Action;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable, by};
use glam::{DVec2, DVec3};

/// The window most tests open on. A size, not a point: `egui` hands a window
/// its size as a vector.
pub const SCREEN: egui::Vec2 = egui::vec2(1280.0, 800.0);

/// On the plane YZ, as a new part shows the three base planes: where the
/// tests opened their sketch before they named it, so that the view a sketch
/// opens on frames the same patch of plane.
pub const ON_THE_YZ_PLANE: DVec3 = DVec3::new(0.0, 70.11, 3.02);

/// Two points of the sketch on YZ, nodes of the grid that pulls them: where
/// the two clicks the tests were first written with laid the ends of a line.
pub const A_POINT_ABOVE_THE_ORIGIN: DVec2 = DVec2::new(-25.0, 87.5);
pub const A_POINT_BELOW_AND_RIGHT: DVec2 = DVec2::new(137.5, -37.5);

/// Where the cursor is left once a line is begun, further along: the node of
/// the grid the pixel it was first written as was pulled onto.
pub const A_POINT_FURTHER_ALONG: DVec2 = DVec2::new(75.0, 37.5);

/// On the segment `draw_a_segment` lays, a few millimetres off its line,
/// within reach of a click that takes it; and where a dimension taken from it
/// is left, above and to the right.
pub const ON_THE_SEGMENT: DVec2 = DVec2::new(59.89, 27.22);
pub const WHERE_ITS_DIMENSION_GOES: DVec2 = DVec2::new(87.39, 65.72);

/// Inside the rectangle drawn from one of the two points above to the other:
/// where the area an extrusion raises is clicked.
pub const INSIDE_THE_RECTANGLE: DVec2 = DVec2::new(62.61, -15.15);

/// The ribbon's « Terminer », by the whole name the ribbon gives it, its
/// shortcut included: the pill a sketch shows over the view carries a
/// « Terminer ↵ » of its own (#727), and a gesture aimed at « Terminer » alone
/// would find both.
pub const FINISH_IN_THE_RIBBON: &str = "Terminer (Entrée)";

pub type App = Harness<'static, CaoApp>;

/// How high the drawing area was when the fixtures above were tuned, in
/// points: on `SCREEN`, before #725 put icons in the ribbon. A part's, and a
/// sketch's, whose ribbon held a row more.
///
/// A reach of the drawing is counted in pixels and the view frames by height,
/// so the canvas's height is the drawing's scale: when the chrome grows, a
/// fixture a few pixels from what it takes takes something else. The driver
/// holds the canvas rather than the window, and the chrome can change without
/// a fixture moving.
pub const CANVAS_HEIGHT_OF_A_PART: f32 = 522.0;
pub const CANVAS_HEIGHT_OF_A_SKETCH: f32 = 492.0;

/// How far the canvas may sit from the height it is held at, in points.
const HELD_WITHIN: f32 = 0.5;

/// How many times the window is resized before the canvas is taken never to
/// settle. One does while the chrome keeps its height as the window grows; a
/// toolbar that wrapped by height would take more.
const RESIZES_TO_SETTLE: usize = 4;

/// What marks an application whose canvas the driver holds. Kept in the
/// application's own `egui` memory, under an id of the driver's: the harness
/// has nowhere else to carry it.
#[derive(Clone, Copy)]
struct HeldCanvas;

fn held_canvas() -> egui::Id {
    egui::Id::new("the driver holds the canvas")
}

fn holds_the_canvas(app: &App) -> bool {
    app.ctx
        .data(|data| data.get_temp::<HeldCanvas>(held_canvas()).is_some())
}

/// Whether the view draws a part: off the part screen, the largest node that
/// takes a click and carries no name is some other surface than the canvas.
fn a_part_is_drawn(app: &App) -> bool {
    app.state().where_the_view_shows(DVec3::ZERO).is_some()
}

/// The height the canvas is held at in the mode the application is in.
fn held_height(app: &App) -> f32 {
    let drawing_a_sketch = app
        .state()
        .where_the_view_shows_the_sketch_point(DVec2::ZERO)
        .is_some();
    if drawing_a_sketch {
        CANVAS_HEIGHT_OF_A_SKETCH
    } else {
        CANVAS_HEIGHT_OF_A_PART
    }
}

/// Makes the window taller or shorter until the canvas is as high as the
/// fixtures were tuned at. Only for an application opened by `open` or
/// `open_held_with`: one opened on a window it named keeps it, since the
/// window is then what it is testing. Every gesture of this module ends here; a test calls it
/// only after frames it ran itself, and `inside_the_canvas` says so if it
/// forgot.
pub fn hold_the_canvas(app: &mut App) {
    if !holds_the_canvas(app) {
        return;
    }
    for _ in 0..RESIZES_TO_SETTLE {
        let Some(canvas) = drawing_area(app).filter(|_| a_part_is_drawn(app)) else {
            return;
        };
        let short_by = held_height(app) - canvas.height();
        if short_by.abs() <= HELD_WITHIN {
            return;
        }
        let window = app.ctx.content_rect().size();
        app.set_size(window + egui::vec2(0.0, short_by));
        app.run();
    }
    panic!(
        "the canvas never settled {} points high: it is {:?} after the window was resized \
         {RESIZES_TO_SETTLE} times, to {:?}",
        held_height(app),
        drawing_area(app),
        app.ctx.content_rect(),
    );
}

/// The application on the default window, its drawing held at the scale the
/// fixtures were tuned at: see `hold_the_canvas`.
pub fn open(test: &str) -> App {
    open_held_with(test, cao_prefs::Settings::default())
}

/// The same, with these settings already saved, as a user who had set them
/// before would open it.
pub fn open_held_with(test: &str, settings: cao_prefs::Settings) -> App {
    let mut app = opened(test, SCREEN, settings);
    app.ctx
        .data_mut(|data| data.insert_temp(held_canvas(), HeldCanvas));
    hold_the_canvas(&mut app);
    app
}

/// The application on a window of this size, left as it is whatever the
/// chrome takes, `SCREEN` included: a test that names its window is testing
/// it — wide enough, say, for the whole ribbon of the sketch to fit, patterns
/// included.
pub fn open_on(test: &str, size: egui::Vec2) -> App {
    open_with(test, size, cao_prefs::Settings::default())
}

/// The same, with these settings already saved.
pub fn open_with(test: &str, size: egui::Vec2, settings: cao_prefs::Settings) -> App {
    opened(test, size, settings)
}

/// The application on a window of this size, a place of the window already
/// remembered where the application reads it, as a user who had closed it
/// there would open it. The place is written compact and followed by blank
/// lines, as the application never lays it out, so that one written back over
/// it shows even when it holds the same values.
pub fn open_on_remembering(test: &str, size: egui::Vec2, place: cao_prefs::WindowPlace) -> App {
    let at = laid_out(test, cao_prefs::Settings::default());
    let mut seeded = serde_json::to_vec(&place).expect("a window place is plain values");
    seeded.extend_from_slice(b"\n\n");
    std::fs::write(window_place_file(test), seeded).expect("the window's place is written");
    assert_eq!(
        cao_prefs::WindowPlace::load(&cao_app::adapters::files::DiskFiles, &at),
        Some(place),
        "the place laid down for the test is one the application would read back",
    );
    launched(at, size)
}

/// Where the application opened for `test` keeps the window's place.
pub fn window_place_file(test: &str) -> PathBuf {
    locations_of(test).config.join("window.json")
}

fn opened(test: &str, size: egui::Vec2, settings: cao_prefs::Settings) -> App {
    let at = laid_out(test, settings);
    launched(at, size)
}

/// A fresh place for `test` to keep what it writes, these settings already
/// saved in it.
fn laid_out(test: &str, settings: cao_prefs::Settings) -> Locations {
    let root = std::env::temp_dir().join("cao-driver").join(test);
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("documents")).expect("a place to keep the parts");
    let at = locations_of(test);
    let mut profiles = cao_prefs::Profiles::default();
    *profiles.active_mut() = settings;
    profiles
        .save(&cao_app::adapters::files::DiskFiles, &at)
        .expect("the settings are written where the app reads them");
    at
}

fn launched(at: Locations, size: egui::Vec2) -> App {
    let mut app = Harness::builder()
        .with_size(size)
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
    hold_the_canvas(app);
}

/// The one widget whose label is exactly this — for a tool whose name is the
/// start of its own mode group's, as `Ellipse` is of `Ellipses`.
pub fn click_exactly(app: &mut App, label: &str) {
    let node = app.get_by_label(label);
    within_reach(label, node.rect().center(), app.ctx.content_rect());
    node.click();
    app.run();
    hold_the_canvas(app);
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
    drawing_area(app).expect("no drawing area on screen: no part is open")
}

fn drawing_area(app: &App) -> Option<egui::Rect> {
    let screen = app.ctx.content_rect();
    app.query_all(
        by().predicate(|node| node.data().supports_action(Action::Click) && node.label().is_none()),
    )
    .map(|node| node.rect())
    .filter(|rect| !rect.contains_rect(screen))
    .max_by(|one, other| one.area().total_cmp(&other.area()))
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
    hold_the_canvas(app);
    let at = world_point_on_screen(app, point);
    click_at(app, at);
}

pub fn click_at_sketch_point(app: &mut App, point: DVec2) {
    hold_the_canvas(app);
    let at = sketch_point_on_screen(app, point);
    click_at(app, at);
}

/// The pointer left over a point of the sketch, as a hand resting there.
pub fn hover_at_sketch_point(app: &mut App, point: DVec2) {
    hold_the_canvas(app);
    let at = sketch_point_on_screen(app, point);
    app.hover_at(at);
    app.run();
}

/// A point of the drawing the view puts outside the canvas would be a click
/// on the chrome, so it is refused here, in the spirit of `within_reach`.
fn inside_the_canvas(app: &App, at: egui::Pos2, what: &str) -> egui::Pos2 {
    let canvas = canvas(app);
    assert!(
        !holds_the_canvas(app) || (canvas.height() - held_height(app)).abs() <= HELD_WITHIN,
        "{what} is read off a canvas {} points high, where the driver holds it at {}: \
         frames run outside the driver's gestures changed the chrome, and a fixture \
         read now is read at another scale. Call `driver::hold_the_canvas` first.",
        canvas.height(),
        held_height(app),
    );
    assert!(
        canvas.contains(at),
        "{what} lands at {at:?}, outside the canvas {canvas:?}, where a click \
         reaches the chrome instead",
    );
    at
}

/// A click at a place of the screen. A test names a point of the drawing
/// instead, through `click_at_sketch_point` or `click_at_world_point`, or a
/// place it read off the frame: `screen_points.rs` refuses one written in
/// numbers.
pub fn click_at(app: &mut App, pos: egui::Pos2) {
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
    hold_the_canvas(app);
}

pub fn type_into_the_only_field(app: &mut App, text: &str) {
    app.get_by_role(egui::accesskit::Role::TextInput).focus();
    app.run();
    app.event(egui::Event::Text(text.to_owned()));
    app.run();
    hold_the_canvas(app);
}

pub fn press(app: &mut App, key: egui::Key) {
    app.key_press(key);
    app.run();
    hold_the_canvas(app);
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
    click_at_world_point(app, ON_THE_YZ_PLANE);
}

pub fn draw_a_segment(app: &mut App) {
    click(app, "Ligne (L)");
    click_at_sketch_point(app, A_POINT_ABOVE_THE_ORIGIN);
    click_at_sketch_point(app, A_POINT_BELOW_AND_RIGHT);
    press(app, egui::Key::Escape);
}

/// The panel is a toggle and the part decides which one opens with it, so a
/// test that presses blindly closes the panel it wanted half the time. H is
/// the way to it since #726 took the « Historique » toggle off the tab row.
pub fn open_the_history(app: &mut App) {
    if !shows(app, "Pièce vide") {
        press(app, egui::Key::H);
    }
    assert!(
        shows(app, "Pièce vide"),
        "H did not open the history, the keyboard held by a field perhaps: {:?}",
        on_screen(app),
    );
}

/// The ellipse tool, its centre and the end of its first axis clicked, the
/// cursor then left where the second axis would reach: on the origin, which
/// pulls the point the cursor was first written as.
pub fn start_an_ellipse(app: &mut App) {
    click_exactly(app, "Ellipse");
    click_at_sketch_point(app, A_POINT_ABOVE_THE_ORIGIN);
    click_at_sketch_point(app, A_POINT_BELOW_AND_RIGHT);
    hover_at_sketch_point(app, DVec2::ZERO);
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
    hold_the_canvas(app);
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
    hold_the_canvas(app);
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
    hold_the_canvas(app);
}

/// Starts a line with one click, the cursor then left further along.
pub fn start_a_line(app: &mut App) {
    click(app, "Ligne (L)");
    click_at_sketch_point(app, A_POINT_ABOVE_THE_ORIGIN);
    hover_at_sketch_point(app, A_POINT_FURTHER_ALONG);
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
    hold_the_canvas(app);
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
    hold_the_canvas(app);
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
    hold_the_canvas(app);
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
    hold_the_canvas(app);
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
    hold_the_canvas(app);
}
