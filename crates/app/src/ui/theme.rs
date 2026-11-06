//! The chrome's theme: the redesign's dark palette and its two typefaces, as
//! `docs/redesign/tokens.json` writes them.

pub mod tokens;

use std::sync::Arc;

use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::{
    Color32, Context, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Margin, Stroke,
    Style, TextStyle, Theme, ThemePreference, Visuals, vec2,
};

use tokens::*;

const INTER: &[u8] = include_bytes!("../../assets/fonts/inter_regular.ttf");
const JETBRAINS_MONO: &[u8] = include_bytes!("../../assets/fonts/jetbrains_mono_regular.ttf");

/// The border every state of a widget wears, and how far its frame reaches
/// past the room it takes, in points.
const BORDER: i8 = 1;

/// Dark whatever the system asks for, in the tokens' colours, sizes and
/// typefaces.
pub fn install(ctx: &Context) {
    ctx.set_theme(ThemePreference::Dark);
    ctx.set_style_of(Theme::Dark, style());
    ctx.set_fonts(font_definitions());
}

/// Inter for text and JetBrains Mono for numbers and keys, each opening its
/// family ahead of the faces `egui` ships, which still draw the marks they lack.
pub fn font_definitions() -> FontDefinitions {
    let mut definitions = FontDefinitions::default();
    put_first(
        &mut definitions,
        FontFamily::Proportional,
        "Inter-Regular",
        INTER,
    );
    put_first(
        &mut definitions,
        FontFamily::Monospace,
        "JetBrainsMono-Regular",
        JETBRAINS_MONO,
    );
    definitions
}

fn put_first(
    definitions: &mut FontDefinitions,
    family: FontFamily,
    name: &str,
    font: &'static [u8],
) {
    definitions
        .font_data
        .insert(name.to_owned(), Arc::new(FontData::from_static(font)));
    definitions
        .families
        .entry(family)
        .or_default()
        .insert(0, name.to_owned());
}

fn style() -> Style {
    let mut style = Style {
        visuals: visuals(),
        ..Theme::Dark.default_style()
    };
    style.spacing.item_spacing = vec2(SPACING_L, SPACING_M);
    style.spacing.button_padding = vec2(SPACING_L, SPACING_S);
    // A frame reaching past its room is clipped at a scroll area's edge, where
    // the content would otherwise stand flush.
    style.spacing.scroll.content_margin = Margin::same(BORDER);
    style.text_styles = [
        (
            TextStyle::Small,
            FontId::proportional(TYPOGRAPHY_SIZES_DETAIL),
        ),
        (TextStyle::Body, FontId::proportional(TYPOGRAPHY_SIZES_BODY)),
        (
            TextStyle::Button,
            FontId::proportional(TYPOGRAPHY_SIZES_BUTTON),
        ),
        (
            TextStyle::Heading,
            FontId::proportional(TYPOGRAPHY_SIZES_SECTION_TITLE),
        ),
        (
            TextStyle::Monospace,
            FontId::monospace(TYPOGRAPHY_SIZES_MONO_VALUE),
        ),
    ]
    .into();
    style
}

fn visuals() -> Visuals {
    Visuals {
        panel_fill: BACKGROUND_PANEL,
        window_fill: BACKGROUND_FLOATING,
        window_stroke: Stroke::new(1.0, BORDER_GROUP),
        window_corner_radius: CornerRadius::same(RADIUS_CARD),
        menu_corner_radius: CornerRadius::same(RADIUS_CONTROL),
        extreme_bg_color: BACKGROUND_FIELD,
        faint_bg_color: BACKGROUND_CARD,
        weak_text_color: Some(TEXT_QUIET),
        hyperlink_color: ACCENT_AMBER_TEXT,
        warn_fg_color: STATE_WARNING_TEXT,
        error_fg_color: STATE_ERROR_TEXT,
        selection: Selection {
            bg_fill: ACCENT_AMBER_BACKGROUND,
            stroke: Stroke::new(1.0, ACCENT_AMBER),
        },
        widgets: widgets(),
        ..Visuals::dark()
    }
}

/// The colours of each state over `egui`'s own dark ones. The text keeps
/// `egui`'s stroke widths; every state wears a border of `BORDER`, the one at
/// rest included, so that a button at rest is drawn transparent within a
/// border, as the redesign's secondary button is.
///
/// Every state's frame also reaches `BORDER` past its room. A button framed
/// only under the pointer, as `Button::selectable` draws one not chosen, is
/// padded at rest by its padding plus that reach less its border, and framed by
/// its padding alone: the two agree only when the reach is the border, or the
/// pointer coming onto it moves it and what stands beside it. And a frame keeps
/// its size from one state to the next only when every state reaches alike.
fn widgets() -> Widgets {
    let egui = Widgets::dark();
    Widgets {
        noninteractive: worn(
            egui.noninteractive,
            BACKGROUND_PANEL,
            BACKGROUND_PANEL,
            BORDER_SEPARATOR,
            TEXT_PRIMARY,
        ),
        inactive: worn(
            egui.inactive,
            Color32::TRANSPARENT,
            BACKGROUND_FIELD,
            BORDER_CONTROL,
            TEXT_BUTTON,
        ),
        hovered: worn(
            egui.hovered,
            BACKGROUND_HOVER,
            BACKGROUND_HOVER,
            BORDER_STRONG,
            TEXT_TITLE,
        ),
        active: worn(
            egui.active,
            BACKGROUND_ACTIVE_NEUTRAL,
            BACKGROUND_ACTIVE_NEUTRAL,
            BORDER_STRONG,
            TEXT_TITLE,
        ),
        open: worn(
            egui.open,
            BACKGROUND_HOVER_STRONG,
            BACKGROUND_PANEL,
            BORDER_CONTROL,
            TEXT_TITLE,
        ),
    }
}

fn worn(
    state: WidgetVisuals,
    weak_bg_fill: Color32,
    bg_fill: Color32,
    border: Color32,
    text: Color32,
) -> WidgetVisuals {
    WidgetVisuals {
        weak_bg_fill,
        bg_fill,
        bg_stroke: Stroke::new(f32::from(BORDER), border),
        fg_stroke: Stroke {
            color: text,
            ..state.fg_stroke
        },
        corner_radius: CornerRadius::same(RADIUS_CONTROL),
        expansion: f32::from(BORDER),
    }
}
