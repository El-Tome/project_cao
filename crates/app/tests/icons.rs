//! A command has a picture as well as a name: the redesign's icons, worn by
//! the ribbon's buttons, the panel toggles, the menus and the eye of a sketch.
//!
//! Closes #725. The sizes, the sharpness and the colour of an icon are held
//! beside the primitives that draw it, in `ui/icon/tests.rs` and
//! `ui/tool_button/tests.rs`; what is held here is where the icons are worn.
//! - they appear in today's ribbon buttons, the icon above the name —
//!   `a_ribbon_button_shows_its_icon_above_its_name`
//! - in the panel toggles — `a_panel_toggle_shows_its_icon_beside_its_name`
//! - and in the menus — `a_menu_entry_shows_its_icon_beside_its_name`
//! - with « Noms des boutons » off, a button is its icon alone, and its name
//!   is in the tooltip —
//!   `with_names_off_a_button_is_its_icon_alone_and_its_name_comes_on_hover`
//! - the eye of a sketch is the `eye` / `eye-off` icon instead of the emoji —
//!   `a_sketch_s_eye_is_drawn_open_while_shown_and_shut_while_hidden`
//! - every button keeps its name as its accessible label, so the headless
//!   driver still finds an icon-only button —
//!   `with_names_off_a_button_is_still_found_and_pressed_by_its_name`
//! - `language.rs`: the icon files keep their English names — no test: held
//!   by `no_file_and_no_folder_carries_a_french_name` in `language.rs`, which
//!   walks every file of the workspace, wherever the icons are embedded from
//! - the shortcuts and what each button does — no test: the driver tests that
//!   pick a tool by its name or its key (`picking_a_tool.rs`,
//!   `selecting_everything.rs`, …) and the shortcut tests of `cao_prefs` hold
//!   them already, and stay green on this branch
//!
//! Decided on the way, for the last three criteria. A panel toggle wears its
//! icon at 15 points, the small button's size, to the left of its name, as
//! `docs/redesign/components.md` C5 draws all five: the ⚙ becomes the fifth,
//! the `preferences` icon beside « Préférences », which is then its accessible
//! name rather than the glyph. A menu entry wears its icon at 14 points, a
//! row's size, to the left of its name; the menu's own button (« Cercles »…)
//! wears none yet, since a group is not a command and #731 makes it a split
//! button whose icon is its tool's. The eye of a sketch is `eye` while the
//! sketch is shown and `eye-off` while it is not, and is called, for whoever
//! cannot see it, what a click on it does: the sentence its tooltip says.
//!
//! An icon is read off the frame's shapes: the textured rectangle `egui::Image`
//! paints, named by the file it was drawn from, which is the name its texture
//! is loaded under — `bytes://icon/new-sketch.svg`.

mod driver;

use egui::accesskit::{Role, Toggled};
use egui_kittest::kittest::{NodeT, Queryable};

#[test]
fn a_ribbon_button_shows_its_icon_above_its_name() {
    let mut app = driver::open("a_ribbon_button_shows_its_icon_above_its_name");
    driver::create_a_part(&mut app, "Platine");

    let button = app.get_by_label_contains("Nouvelle esquisse").rect();
    let painted = painted_within(&app, button);
    let icon = painted.icon("new-sketch");
    let name = painted.text_holding("Nouvelle esquisse");

    assert!(
        icon.bottom() <= name.top() && name.x_range().contains(icon.center().x),
        "« Nouvelle esquisse » wears its icon at {icon:?} and its name at {name:?}: \
         the icon is not above the name",
    );
}

#[test]
fn with_names_off_a_button_is_its_icon_alone_and_its_name_comes_on_hover() {
    let test = "with_names_off_a_button_is_its_icon_alone";
    let mut app = driver::open_with(test, driver::SCREEN, names_off());
    driver::create_a_part(&mut app, "Platine");

    let button = button_wearing(&app, "new-sketch").rect;
    let painted = painted_within(&app, button);
    assert!(
        painted.texts.is_empty(),
        "with names off, the button wearing `new-sketch` still writes {:?} beside its icon",
        painted.texts,
    );

    let saying_it = |app: &driver::App| {
        driver::said(app)
            .iter()
            .any(|line| line.contains("Nouvelle esquisse"))
    };
    assert!(
        !saying_it(&app),
        "« Nouvelle esquisse » is said before anything is hovered: {:?}",
        driver::said(&app),
    );
    app.hover_at(button.center());
    app.run_steps(4);
    assert!(
        saying_it(&app),
        "hovering the button wearing `new-sketch` does not say its name; the screen says {:?}",
        driver::said(&app),
    );
}

#[test]
fn with_names_off_a_button_is_still_found_and_pressed_by_its_name() {
    let test = "with_names_off_a_button_is_still_found";
    let mut app = driver::open_with(test, driver::SCREEN, names_off());
    driver::create_a_part(&mut app, "Platine");

    driver::start_a_sketch(&mut app);
    driver::click_exactly(&mut app, "Ligne");

    assert_eq!(
        app.get_by_label("Ligne").accesskit_node().toggled(),
        Some(Toggled::True),
        "with names off, the button named « Ligne » was pressed and the line is not the tool in hand",
    );
}

#[test]
fn a_panel_toggle_shows_its_icon_beside_its_name() {
    let mut app = driver::open("a_panel_toggle_shows_its_icon_beside_its_name");
    driver::create_a_part(&mut app, "Platine");

    for (name, id) in [
        ("Fichiers", "files"),
        ("Pièce", "part"),
        ("Historique", "history"),
        ("Variables", "variables"),
        ("Préférences", "preferences"),
    ] {
        let painted = painted_within(&app, the_button_named(&app, name));
        let icon = painted.icon(id);
        assert_beside(icon, painted.text_holding(name), name);
        assert_drawn_at(icon, SMALL, name);
    }
}

#[test]
fn a_menu_entry_shows_its_icon_beside_its_name() {
    let mut app = driver::open("a_menu_entry_shows_its_icon_beside_its_name");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::click(&mut app, "Cercles");

    let name = "Centre et diamètre";
    let painted = painted_within(&app, the_button_named(&app, name));
    let icon = painted.icon("circle-centre");
    assert_beside(icon, painted.text_holding(name), name);
    assert_drawn_at(icon, ROW, name);
}

#[test]
fn a_sketch_s_eye_is_drawn_open_while_shown_and_shut_while_hidden() {
    let mut app = driver::open("a_sketch_s_eye_is_drawn_open_or_shut");
    driver::create_a_part(&mut app, "Platine");
    driver::start_a_sketch(&mut app);
    driver::draw_a_segment(&mut app);
    driver::click(&mut app, "Terminer");

    for (worn, called, then) in [
        ("eye", "Ne plus afficher l'esquisse", "eye-off"),
        ("eye-off", "Afficher l'esquisse", "eye"),
    ] {
        let eye = button_wearing(&app, worn);
        assert!(
            painted_within(&app, eye.rect).texts.is_empty(),
            "the eye wearing `{worn}` still writes {:?}",
            painted_within(&app, eye.rect).texts,
        );
        assert_eq!(
            eye.label.as_deref(),
            Some(called),
            "the eye wearing `{worn}` is not called what a click on it does",
        );

        driver::click_exactly(&mut app, called);
        button_wearing(&app, then);
    }
}

/// The sizes `docs/redesign/icons.md` draws an icon at, in points: a small
/// button's, and a row's.
const SMALL: f32 = 15.0;
const ROW: f32 = 14.0;

/// How far an icon's side may be from its size, in points.
const SIZED_WITHIN: f32 = 0.01;

/// The button of that name, for a word a label of the screen carries too.
fn the_button_named(app: &driver::App, name: &str) -> egui::Rect {
    app.query_all_by_label(name)
        .find(|node| node.accesskit_node().role() == Role::Button)
        .map(|node| node.rect())
        .unwrap_or_else(|| {
            panic!(
                "no button is named « {name} »; the screen names {:?}",
                driver::on_screen(app)
            )
        })
}

fn assert_beside(icon: egui::Rect, name: egui::Rect, called: &str) {
    assert!(
        icon.right() <= name.left() && name.y_range().contains(icon.center().y),
        "« {called} » wears its icon at {icon:?} and its name at {name:?}: the icon is \
         not to the left of the name",
    );
}

fn assert_drawn_at(icon: egui::Rect, side: f32, called: &str) {
    assert!(
        (icon.width() - side).abs() < SIZED_WITHIN && (icon.height() - side).abs() < SIZED_WITHIN,
        "« {called} » wears an icon of {:?} points, where the redesign draws it at {side}",
        icon.size(),
    );
}

fn names_off() -> cao_prefs::Settings {
    let mut settings = cao_prefs::Settings::default();
    settings.toolbar.show_labels = false;
    settings
}

/// What the last frame painted inside a rectangle of the screen: each icon,
/// by the id of the file it was drawn from, and each text, with where it sits.
struct Painted {
    icons: Vec<(String, egui::Rect)>,
    texts: Vec<(String, egui::Rect)>,
}

impl Painted {
    fn icon(&self, id: &str) -> egui::Rect {
        self.icons
            .iter()
            .find(|(drawn, _)| drawn == id)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("no `{id}` icon here, only {:?}", self.icons))
    }

    fn text_holding(&self, words: &str) -> egui::Rect {
        self.texts
            .iter()
            .find(|(text, _)| text.contains(words))
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("nothing here says {words:?}, only {:?}", self.texts))
    }
}

fn painted_within(app: &driver::App, within: egui::Rect) -> Painted {
    let within = within.expand(1.0);
    let textures = app.ctx.tex_manager();
    let textures = textures.read();
    let mut painted = Painted {
        icons: Vec::new(),
        texts: Vec::new(),
    };

    let mut pending: Vec<&egui::Shape> = app
        .output()
        .shapes
        .iter()
        .map(|clipped| &clipped.shape)
        .collect();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            egui::Shape::Rect(rect)
                if rect.fill_texture_id() != egui::TextureId::default()
                    && within.contains_rect(rect.rect) =>
            {
                let loaded_as = textures
                    .meta(rect.fill_texture_id())
                    .map(|meta| meta.name.clone())
                    .unwrap_or_default();
                painted.icons.push((icon_id(&loaded_as), rect.rect));
            }
            egui::Shape::Text(text) if within.contains_rect(shape.visual_bounding_rect()) => {
                painted
                    .texts
                    .push((text.galley.text().to_owned(), shape.visual_bounding_rect()));
            }
            _ => {}
        }
    }
    painted
}

/// The id of the icon a texture was loaded from, out of the URI it carries.
fn icon_id(loaded_as: &str) -> String {
    let file = loaded_as.rsplit('/').next().unwrap_or(loaded_as);
    file.strip_suffix(".svg").unwrap_or(file).to_owned()
}

/// A button found by the icon it wears rather than by a name: where it is,
/// and what it is called for whoever cannot see it.
struct Wearing {
    rect: egui::Rect,
    label: Option<String>,
}

fn button_wearing(app: &driver::App, id: &str) -> Wearing {
    let icon = painted_within(app, app.ctx.content_rect()).icon(id);
    app.get_all_by_role(Role::Button)
        .map(|node| Wearing {
            rect: node.rect(),
            label: node.accesskit_node().label(),
        })
        .filter(|button| button.rect.expand(1.0).contains_rect(icon))
        .min_by(|one, other| one.rect.area().total_cmp(&other.rect.area()))
        .unwrap_or_else(|| panic!("the `{id}` icon is painted in no button"))
}
