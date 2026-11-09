//! The chrome's own theme — the redesign's dark palette and its two typefaces,
//! as `docs/redesign/tokens.json` writes them — held against what `egui` is
//! handed.
//!
//! Closes #724. The issue carries no "Done when": what follows is read from
//! "What it does" and "What must not break". The criteria other files answer
//! are transcribed beside their tests: the colours written as literals in
//! `architecture.rs`, the marks in `glyphs.rs`, Preferences › Apparence in
//! `appearance.rs`, the view's background and the « Par défaut » profile in
//! `crates/prefs/src/theme/tests.rs`, `crates/prefs/src/profiles/tests.rs` and
//! `crates/app/src/screens/viewport/render/tests.rs`.
//! - **Inter** for text and **JetBrains Mono** for numbers and keys are
//!   embedded and set as the proportional and monospace families, every face
//!   `egui` ships staying behind them as a fallback —
//!   `text_is_drawn_in_inter_with_every_face_egui_ships_behind_it`,
//!   `numbers_are_drawn_in_jetbrains_mono_with_every_face_egui_ships_behind_it`;
//!   and they are the fonts the context draws with once the theme is installed
//!   — `the_fonts_are_installed_on_the_context`
//! - one theme module builds `egui::Visuals` and `egui::Style` from the tokens:
//!   backgrounds, borders, text levels, the amber accent, radii, spacing —
//!   `the_chrome_s_colours_are_the_tokens`,
//!   `the_chrome_s_corners_and_spacing_are_the_tokens`,
//!   `the_chrome_s_text_sizes_are_the_tokens`; and every token the theme holds,
//!   used yet or not, is the design file's (found in review) —
//!   `every_token_the_chrome_holds_is_the_design_file_s`
//! - every number in JetBrains Mono, as "To try it" promises (found in review)
//!   — no test: in this file; the viewport's figures are held in the tests of
//!   `render/dimensions.rs`, `render/cuts.rs`, `render/reading.rs` and
//!   `render/overlays.rs`, each under its own block; the keys are drawn by
//!   components still to come, #731 and #746, and no test here can see a
//!   label's typeface through the driver
//! - found when Tom tried the branch, « le carré en contour fait déplacer les
//!   éléments »: a button keeps its size, and its text its place, when the
//!   pointer comes onto it — one framed only under the pointer, its neighbour
//!   beside it included, and one framed at rest —
//!   `a_button_keeps_its_size_and_its_text_its_place_when_the_pointer_comes_onto_it`
//! - the application is dark whatever the OS says —
//!   `the_chrome_stays_dark_when_the_system_asks_for_light`; and it is from
//!   the first frame, `CaoApp::new` installing the theme —
//!   `the_application_opens_dark_in_the_token_colours`
//! - the cube's face colours in `cao_render` keep working — no test: the issue
//!   touches nothing in `crates/render/src/cube.rs`, whose own tests stay as
//!   they are
//!
//! Which face draws a character is read off `Font::characters`, which lists,
//! for each character the family reaches, the faces carrying it in the
//! family's order: the first one is the face the text is drawn in.
//!
//! A face is keyed by its PostScript name, `Inter-Regular` and
//! `JetBrainsMono-Regular`, as `egui` keys its own (`Ubuntu-Light`,
//! `NotoEmoji-Regular`): the key names one weight of a family, which is what
//! is embedded, and the day a second weight is, it has a name of its own.
//!
//! The expected style is read off the design file itself, so that the file
//! stays the authority: a token changed there is a test that fails here until
//! the theme follows.

mod driver;

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use egui::epaint::{Fonts, TextOptions};
use egui::{
    Color32, Context, CornerRadius, FontDefinitions, FontFamily, FontId, RawInput, TextStyle,
    Theme, Vec2, vec2,
};
use egui_kittest::kittest::Queryable;

const TOKENS: &str = "docs/redesign/tokens.json";

const INTER: &str = "Inter-Regular";
const JETBRAINS_MONO: &str = "JetBrainsMono-Regular";

/// A tenth of a thousandth of a point: the sizes are typed into the design
/// file, so anything wider would be a different value rather than a rounding.
const TOLERANCE: f32 = 1e-4;

#[test]
fn text_is_drawn_in_inter_with_every_face_egui_ships_behind_it() {
    let definitions = cao_app::theme::font_definitions();

    opens_on_then_keeps_what_egui_ships(&definitions, &FontFamily::Proportional, INTER);
    let elsewhere = drawn_elsewhere_than(&definitions, &FontFamily::Proportional, INTER, &prose());
    assert!(
        elsewhere.is_empty(),
        "Characters of a French sentence not drawn in Inter:\n  {}",
        elsewhere.join("\n  "),
    );
}

#[test]
fn numbers_are_drawn_in_jetbrains_mono_with_every_face_egui_ships_behind_it() {
    let definitions = cao_app::theme::font_definitions();

    opens_on_then_keeps_what_egui_ships(&definitions, &FontFamily::Monospace, JETBRAINS_MONO);
    let elsewhere = drawn_elsewhere_than(
        &definitions,
        &FontFamily::Monospace,
        JETBRAINS_MONO,
        &figures(),
    );
    assert!(
        elsewhere.is_empty(),
        "Characters of a value, a unit or a key not drawn in JetBrains Mono:\n  {}",
        elsewhere.join("\n  "),
    );
}

#[test]
fn the_fonts_are_installed_on_the_context() {
    let context = installed();

    assert_eq!(
        context.fonts(|fonts| fonts.definitions().families.clone()),
        cao_app::theme::font_definitions().families,
        "the context draws with other fonts than the theme's",
    );
}

#[test]
fn the_chrome_stays_dark_when_the_system_asks_for_light() {
    let context = installed();

    assert_eq!(
        context.theme(),
        Theme::Dark,
        "the system asked for light, and the chrome followed it",
    );
    assert!(
        context.global_style().visuals.dark_mode,
        "the style in use is a light one",
    );
}

#[test]
fn the_chrome_s_colours_are_the_tokens() {
    let tokens = Tokens::read();
    let visuals = installed().global_style().visuals.clone();
    let widgets = &visuals.widgets;

    let worn = [
        ("panel_fill", visuals.panel_fill, "background.panel"),
        ("window_fill", visuals.window_fill, "background.floating"),
        (
            "extreme_bg_color",
            visuals.extreme_bg_color,
            "background.field",
        ),
        ("faint_bg_color", visuals.faint_bg_color, "background.card"),
        ("window_stroke", visuals.window_stroke.color, "border.group"),
        (
            "noninteractive bg_stroke",
            widgets.noninteractive.bg_stroke.color,
            "border.separator",
        ),
        (
            "noninteractive fg_stroke",
            widgets.noninteractive.fg_stroke.color,
            "text.primary",
        ),
        (
            "inactive fg_stroke",
            widgets.inactive.fg_stroke.color,
            "text.button",
        ),
        (
            "inactive bg_stroke",
            widgets.inactive.bg_stroke.color,
            "border.control",
        ),
        (
            "hovered weak_bg_fill",
            widgets.hovered.weak_bg_fill,
            "background.hover",
        ),
        (
            "active weak_bg_fill",
            widgets.active.weak_bg_fill,
            "background.active_neutral",
        ),
        (
            "selection bg_fill",
            visuals.selection.bg_fill,
            "accent.amber_background",
        ),
        (
            "selection stroke",
            visuals.selection.stroke.color,
            "accent.amber",
        ),
        (
            "hyperlink_color",
            visuals.hyperlink_color,
            "accent.amber_text",
        ),
        ("warn_fg_color", visuals.warn_fg_color, "state.warning_text"),
        ("error_fg_color", visuals.error_fg_color, "state.error_text"),
    ];

    let wrong: Vec<String> = worn
        .into_iter()
        .filter_map(|(field, worn, token)| {
            let written = tokens.colour(token);
            (worn != written).then(|| format!("{field} is {worn:?}, color.{token} is {written:?}"))
        })
        .collect();
    assert!(
        wrong.is_empty(),
        "The chrome wears colours the design file does not write:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn the_chrome_s_corners_and_spacing_are_the_tokens() {
    let tokens = Tokens::read();
    let style = installed().global_style();
    let card = CornerRadius::same(tokens.radius("card"));
    let control = CornerRadius::same(tokens.radius("control"));

    assert_eq!(
        style.visuals.window_corner_radius, card,
        "a window is rounded as a card"
    );
    assert_eq!(
        style.visuals.menu_corner_radius, control,
        "a menu is rounded as a control"
    );
    assert_eq!(
        style.visuals.widgets.inactive.corner_radius, control,
        "a button at rest is rounded as a control",
    );

    let item_spacing = vec2(tokens.number("spacing.l"), tokens.number("spacing.m"));
    assert!(
        near(style.spacing.item_spacing, item_spacing),
        "item_spacing is {:?}, the tokens say {item_spacing:?} (spacing.l, spacing.m)",
        style.spacing.item_spacing,
    );
    let button_padding = vec2(tokens.number("spacing.l"), tokens.number("spacing.s"));
    assert!(
        near(style.spacing.button_padding, button_padding),
        "button_padding is {:?}, the tokens say {button_padding:?} (spacing.l, spacing.s)",
        style.spacing.button_padding,
    );
}

#[test]
fn the_chrome_s_text_sizes_are_the_tokens() {
    let tokens = Tokens::read();
    let style = installed().global_style();

    let wrong: Vec<String> = [
        (TextStyle::Body, "body", FontFamily::Proportional),
        (TextStyle::Button, "button", FontFamily::Proportional),
        (TextStyle::Small, "detail", FontFamily::Proportional),
        (
            TextStyle::Heading,
            "section_title",
            FontFamily::Proportional,
        ),
        (TextStyle::Monospace, "mono_value", FontFamily::Monospace),
    ]
    .into_iter()
    .filter_map(|(text_style, size, family)| {
        let written = FontId::new(tokens.number(&format!("typography.sizes.{size}")), family);
        let worn = style.text_styles.get(&text_style);
        let same = worn.is_some_and(|worn| {
            worn.family == written.family && (worn.size - written.size).abs() < TOLERANCE
        });
        (!same).then(|| {
            format!("{text_style:?} is {worn:?}, typography.sizes.{size} gives {written:?}")
        })
    })
    .collect();
    assert!(
        wrong.is_empty(),
        "The chrome writes at sizes the design file does not:\n  {}",
        wrong.join("\n  "),
    );
}

#[test]
fn every_token_the_chrome_holds_is_the_design_file_s() {
    use cao_app::theme::tokens;

    let design = Tokens::read();
    let colours = [
        (
            "BACKGROUND_PANEL",
            tokens::BACKGROUND_PANEL,
            "background.panel",
        ),
        (
            "BACKGROUND_CARD",
            tokens::BACKGROUND_CARD,
            "background.card",
        ),
        (
            "BACKGROUND_FLOATING",
            tokens::BACKGROUND_FLOATING,
            "background.floating",
        ),
        (
            "BACKGROUND_HOVER",
            tokens::BACKGROUND_HOVER,
            "background.hover",
        ),
        (
            "BACKGROUND_HOVER_STRONG",
            tokens::BACKGROUND_HOVER_STRONG,
            "background.hover_strong",
        ),
        (
            "BACKGROUND_ACTIVE_NEUTRAL",
            tokens::BACKGROUND_ACTIVE_NEUTRAL,
            "background.active_neutral",
        ),
        (
            "BACKGROUND_FIELD",
            tokens::BACKGROUND_FIELD,
            "background.field",
        ),
        (
            "BORDER_SEPARATOR",
            tokens::BORDER_SEPARATOR,
            "border.separator",
        ),
        ("BORDER_GROUP", tokens::BORDER_GROUP, "border.group"),
        ("BORDER_CONTROL", tokens::BORDER_CONTROL, "border.control"),
        ("BORDER_STRONG", tokens::BORDER_STRONG, "border.strong"),
        ("TEXT_TITLE", tokens::TEXT_TITLE, "text.title"),
        ("TEXT_PRIMARY", tokens::TEXT_PRIMARY, "text.primary"),
        ("TEXT_BUTTON", tokens::TEXT_BUTTON, "text.button"),
        ("TEXT_SECONDARY", tokens::TEXT_SECONDARY, "text.secondary"),
        ("TEXT_SUBDUED", tokens::TEXT_SUBDUED, "text.subdued"),
        ("TEXT_QUIET", tokens::TEXT_QUIET, "text.quiet"),
        ("TEXT_SHORTCUT", tokens::TEXT_SHORTCUT, "text.shortcut"),
        ("TEXT_DISABLED", tokens::TEXT_DISABLED, "text.disabled"),
        ("TEXT_ON_ACCENT", tokens::TEXT_ON_ACCENT, "text.on_accent"),
        ("ACCENT_AMBER", tokens::ACCENT_AMBER, "accent.amber"),
        (
            "ACCENT_AMBER_LIGHT",
            tokens::ACCENT_AMBER_LIGHT,
            "accent.amber_light",
        ),
        (
            "ACCENT_AMBER_TEXT",
            tokens::ACCENT_AMBER_TEXT,
            "accent.amber_text",
        ),
        (
            "ACCENT_AMBER_BORDER",
            tokens::ACCENT_AMBER_BORDER,
            "accent.amber_border",
        ),
        (
            "ACCENT_AMBER_BACKGROUND",
            tokens::ACCENT_AMBER_BACKGROUND,
            "accent.amber_background",
        ),
        (
            "STATE_WARNING_TEXT",
            tokens::STATE_WARNING_TEXT,
            "state.warning_text",
        ),
        (
            "STATE_ERROR_TEXT",
            tokens::STATE_ERROR_TEXT,
            "state.error_text",
        ),
        ("STATE_ERROR", tokens::STATE_ERROR, "state.error"),
        (
            "STATE_ERROR_BACKGROUND",
            tokens::STATE_ERROR_BACKGROUND,
            "state.error_background",
        ),
        (
            "STATE_SUCCESS_TEXT",
            tokens::STATE_SUCCESS_TEXT,
            "state.success_text",
        ),
    ];
    let numbers = [
        (
            "RADIUS_CONTROL",
            f32::from(tokens::RADIUS_CONTROL),
            "radius.control",
        ),
        ("RADIUS_CARD", f32::from(tokens::RADIUS_CARD), "radius.card"),
        (
            "RADIUS_BUTTON",
            f32::from(tokens::RADIUS_BUTTON),
            "radius.button",
        ),
        ("RADIUS_PILL", f32::from(tokens::RADIUS_PILL), "radius.pill"),
        ("SPACING_S", tokens::SPACING_S, "spacing.s"),
        ("SPACING_M", tokens::SPACING_M, "spacing.m"),
        ("SPACING_L", tokens::SPACING_L, "spacing.l"),
        ("SPACING_XXL", tokens::SPACING_XXL, "spacing.xxl"),
        (
            "TYPOGRAPHY_SIZES_SECTION_TITLE",
            tokens::TYPOGRAPHY_SIZES_SECTION_TITLE,
            "typography.sizes.section_title",
        ),
        (
            "TYPOGRAPHY_SIZES_BODY",
            tokens::TYPOGRAPHY_SIZES_BODY,
            "typography.sizes.body",
        ),
        (
            "TYPOGRAPHY_SIZES_BUTTON",
            tokens::TYPOGRAPHY_SIZES_BUTTON,
            "typography.sizes.button",
        ),
        (
            "TYPOGRAPHY_SIZES_DETAIL",
            tokens::TYPOGRAPHY_SIZES_DETAIL,
            "typography.sizes.detail",
        ),
        (
            "TYPOGRAPHY_SIZES_RIBBON_BUTTON",
            tokens::TYPOGRAPHY_SIZES_RIBBON_BUTTON,
            "typography.sizes.ribbon_button",
        ),
        (
            "TYPOGRAPHY_SIZES_SHORTCUT",
            tokens::TYPOGRAPHY_SIZES_SHORTCUT,
            "typography.sizes.shortcut",
        ),
        (
            "TYPOGRAPHY_SIZES_MONO_VALUE",
            tokens::TYPOGRAPHY_SIZES_MONO_VALUE,
            "typography.sizes.mono_value",
        ),
    ];

    let mut wrong: Vec<String> = colours
        .iter()
        .filter(|(_, held, path)| *held != design.colour(path))
        .map(|(name, held, path)| {
            format!(
                "{name} is {held:?}, color.{path} is {:?}",
                design.colour(path)
            )
        })
        .collect();
    wrong.extend(
        numbers
            .iter()
            .filter(|(_, held, path)| (held - design.held_as(path)).abs() >= TOLERANCE)
            .map(|(name, held, path)| {
                format!("{name} is {held}, {path} is {}", design.held_as(path))
            }),
    );
    let checked: BTreeSet<&str> = colours
        .iter()
        .map(|(name, ..)| *name)
        .chain(numbers.iter().map(|(name, ..)| *name))
        .collect();
    wrong.extend(
        tokens_declared()
            .into_iter()
            .filter(|name| !checked.contains(name.as_str()))
            .map(|name| format!("{name} is held in {TOKENS_RS} and checked against nothing here")),
    );

    assert!(
        wrong.is_empty(),
        "The chrome's tokens and the design file disagree:\n  {}",
        wrong.join("\n  "),
    );
}

const TOKENS_RS: &str = "crates/app/src/ui/theme/tokens.rs";

/// The name of every constant `tokens.rs` declares, read off its source: a
/// test cannot list a module's constants, and one added there and missing from
/// the table above would otherwise go unchecked.
fn tokens_declared() -> Vec<String> {
    let source = fs::read_to_string(workspace_root().join(TOKENS_RS))
        .unwrap_or_else(|_| panic!("{TOKENS_RS} is readable"));
    source
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("pub const "))
        .filter_map(|rest| rest.split(':').next())
        .map(|name| name.trim().to_string())
        .collect()
}

/// A row of the start menu's file list, and a neighbour beside it: buttons
/// framed only under the pointer, as `Button::selectable` draws one not chosen.
/// Below them a button framed at rest.
fn two_files_and_a_button(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let _ = ui.add(egui::Button::selectable(false, "Bloc.caopart"));
        let _ = ui.add(egui::Button::selectable(false, "Came.caopart"));
    });
    let _ = ui.button("Créer");
}

/// Where a widget stands, and where its text was painted on the last frame.
fn where_it_stands(harness: &egui_kittest::Harness<'_>, label: &str) -> (egui::Rect, egui::Pos2) {
    let rect = harness.get_by_label(label).rect();
    let mut pending: Vec<egui::Shape> = harness
        .output()
        .shapes
        .iter()
        .map(|clipped| clipped.shape.clone())
        .collect();
    let mut text_at = None;
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(shapes) => pending.extend(shapes),
            egui::Shape::Text(text) if text.galley.text() == label => text_at = Some(text.pos),
            _ => {}
        }
    }
    (
        rect,
        text_at.unwrap_or_else(|| panic!("« {label} » was painted no text")),
    )
}

#[test]
fn a_button_keeps_its_size_and_its_text_its_place_when_the_pointer_comes_onto_it() {
    let mut harness = egui_kittest::Harness::new_ui(two_files_and_a_button);
    cao_app::theme::install(&harness.ctx);
    harness.run();
    let at_rest: Vec<(egui::Rect, egui::Pos2)> = ["Bloc.caopart", "Came.caopart", "Créer"]
        .iter()
        .map(|label| where_it_stands(&harness, label))
        .collect();

    let mut moved = Vec::new();
    for (hovered, watched) in [
        (
            "Bloc.caopart",
            vec![(0, "Bloc.caopart"), (1, "Came.caopart")],
        ),
        ("Créer", vec![(2, "Créer")]),
    ] {
        harness.get_by_label(hovered).hover();
        harness.run();
        for (rank, label) in watched {
            let (rect, text) = where_it_stands(&harness, label);
            let (rect_at_rest, text_at_rest) = at_rest[rank];
            if (rect.min - rect_at_rest.min).length() > TOLERANCE
                || (rect.max - rect_at_rest.max).length() > TOLERANCE
                || (text - text_at_rest).length() > TOLERANCE
            {
                moved.push(format!(
                    "« {label} », the pointer on « {hovered} »: {rect:?} with its text at \
                     {text:?}, where at rest it was {rect_at_rest:?} with its text at {text_at_rest:?}"
                ));
            }
        }
    }

    assert!(
        moved.is_empty(),
        "The pointer moves what it comes onto:\n  {}",
        moved.join("\n  "),
    );
}

#[test]
fn the_application_opens_dark_in_the_token_colours() {
    let app = driver::open("the_application_opens_dark_in_the_token_colours");
    let visuals = app.ctx.global_style().visuals.clone();

    assert!(visuals.dark_mode, "the application opens on a light style");
    assert_eq!(
        visuals.panel_fill,
        Tokens::read().colour("background.panel"),
        "the application opens on panels of another colour than the redesign's: \
         CaoApp::new did not install the theme",
    );
}

/// A context the theme was installed on, after one frame on a system asking
/// for light: the frame is where `egui` hears what the system says, and where
/// it takes the fonts it was handed. The glyphs that frame laid out are let go
/// of by hand, since no renderer is there to take them.
fn installed() -> Context {
    let context = Context::default();
    cao_app::theme::install(&context);
    let on_a_light_system = RawInput {
        system_theme: Some(Theme::Light),
        ..Default::default()
    };
    let mut output = context.run_ui(on_a_light_system, |_| {});
    output.textures_delta.clear();
    context
}

fn near(worn: Vec2, written: Vec2) -> bool {
    (worn - written).length() < TOLERANCE
}

/// `docs/redesign/tokens.json`, read as the design wrote it.
struct Tokens(serde_json::Value);

impl Tokens {
    fn read() -> Self {
        let path = workspace_root().join(TOKENS);
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("{TOKENS} is readable"));
        Self(serde_json::from_str(&text).unwrap_or_else(|_| panic!("{TOKENS} is JSON")))
    }

    /// The value at a dotted path, `spacing.l` for `{"spacing": {"l": 8}}`.
    fn at(&self, path: &str) -> &serde_json::Value {
        path.split('.')
            .try_fold(&self.0, |value, key| value.get(key))
            .unwrap_or_else(|| panic!("{TOKENS} holds no {path}"))
    }

    fn number(&self, path: &str) -> f32 {
        self.at(path)
            .as_f64()
            .unwrap_or_else(|| panic!("{path} in {TOKENS} is not a number")) as f32
    }

    /// A number as `egui` can hold it. A corner's radius is a `u8` there, so
    /// `radius.pill`, written 999 to mean as round as the side allows, is held
    /// as `u8::MAX`: `egui` rounds no corner past half its side either way.
    fn held_as(&self, path: &str) -> f32 {
        let written = self.number(path);
        if path.starts_with("radius.") {
            written.min(f32::from(u8::MAX))
        } else {
            written
        }
    }

    fn radius(&self, name: &str) -> u8 {
        let radius = self.number(&format!("radius.{name}"));
        u8::try_from(radius as u32).unwrap_or_else(|_| panic!("radius.{name} is {radius}"))
    }

    /// A colour of `color.`, written `#RRGGBB`, `rgb(r,g,b)` or `rgba(r,g,b,a)`
    /// with the opacity from 0 to 1 and the channels not multiplied by it.
    fn colour(&self, name: &str) -> Color32 {
        let path = format!("color.{name}");
        let written = self
            .at(&path)
            .as_str()
            .unwrap_or_else(|| panic!("{path} in {TOKENS} is not a string"));
        colour_written(written).unwrap_or_else(|| panic!("{path} is {written}, not a colour"))
    }
}

fn colour_written(written: &str) -> Option<Color32> {
    if let Some(hex) = written.strip_prefix('#') {
        let channel = |at: usize| u8::from_str_radix(hex.get(at..at + 2)?, 16).ok();
        return (hex.len() == 6).then_some(Color32::from_rgb(
            channel(0)?,
            channel(2)?,
            channel(4)?,
        ));
    }
    let inside = written
        .strip_prefix("rgba(")
        .or_else(|| written.strip_prefix("rgb("))?
        .strip_suffix(')')?;
    let parts: Vec<&str> = inside.split(',').map(str::trim).collect();
    let channel = |at: usize| parts.get(at)?.parse::<u8>().ok();
    let (r, g, b) = (channel(0)?, channel(1)?, channel(2)?);
    match parts.get(3) {
        None => Some(Color32::from_rgb(r, g, b)),
        Some(opacity) => {
            let opacity: f32 = opacity.parse().ok()?;
            Some(Color32::from_rgba_unmultiplied(
                r,
                g,
                b,
                (opacity * 255.0).round() as u8,
            ))
        }
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root, two levels above crates/app")
        .to_path_buf()
}

/// What the interface writes in words: ASCII, and the accents, quotes and
/// dashes of French.
fn prose() -> String {
    ('!'..='~')
        .chain("àâäçéèêëîïôöùûüÿœæÀÂÇÉÈÊËÎÏÔÙÛŒ«»’…—–".chars())
        .collect()
}

/// What the interface writes in figures: a value, its unit, a key.
fn figures() -> String {
    ('!'..='~').chain("°±×÷".chars()).collect()
}

/// The family opens on `face`, and behind it the faces `egui` ships for that
/// family still follow, in `egui`'s own order: they are what draws a mark the
/// face lacks.
fn opens_on_then_keeps_what_egui_ships(
    definitions: &FontDefinitions,
    family: &FontFamily,
    face: &str,
) {
    let faces = definitions
        .families
        .get(family)
        .unwrap_or_else(|| panic!("the definitions hold no {family:?} family"));
    assert_eq!(
        faces.first().map(String::as_str),
        Some(face),
        "the {family:?} family is {faces:?}, and the redesign writes it in {face} first",
    );

    let shipped = &FontDefinitions::default().families[family];
    let kept: Vec<&String> = faces[1..]
        .iter()
        .filter(|name| shipped.contains(name))
        .collect();
    assert_eq!(
        kept,
        shipped.iter().collect::<Vec<_>>(),
        "behind {face}, the {family:?} family is {:?}: every face egui ships for it \
         has to follow, in egui's order, or the marks only they carry come out as \
         empty boxes",
        &faces[1..],
    );
}

/// Each character of `written` the family draws in another face than `face`,
/// with the face that draws it — or with nothing, when none does.
fn drawn_elsewhere_than(
    definitions: &FontDefinitions,
    family: &FontFamily,
    face: &str,
    written: &str,
) -> Vec<String> {
    let mut fonts = Fonts::new(TextOptions::default(), definitions.clone());
    let mut font = fonts.fonts.font(family);
    let characters = font.characters();

    written
        .chars()
        .filter_map(
            |mark| match characters.get(&mark).and_then(|faces| faces.first()) {
                Some(drawn_in) if drawn_in == face => None,
                Some(drawn_in) => Some(format!("'{mark}' U+{:04X} in {drawn_in}", mark as u32)),
                None => Some(format!("'{mark}' U+{:04X} in no face at all", mark as u32)),
            },
        )
        .collect()
}
