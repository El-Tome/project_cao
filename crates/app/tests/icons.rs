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

    let button = button_wearing(&app, "new-sketch");
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

/// The button an icon is painted in, found by the icon rather than by a name.
fn button_wearing(app: &driver::App, id: &str) -> egui::Rect {
    let icon = painted_within(app, app.ctx.content_rect()).icon(id);
    app.get_all_by_role(Role::Button)
        .map(|node| node.rect())
        .filter(|button| button.expand(1.0).contains_rect(icon))
        .min_by(|one, other| one.area().total_cmp(&other.area()))
        .unwrap_or_else(|| panic!("the `{id}` icon is painted in no button"))
}
