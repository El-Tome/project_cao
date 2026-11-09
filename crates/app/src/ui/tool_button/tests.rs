//! What app · ui/tool_button.rs is held to.
//!
//! Closes #725.
//! - they take the colour of the widget's text: a disabled button greys its
//!   icon, an active tool turns it amber —
//!   `at_rest_a_tool_button_s_icon_wears_the_colour_of_its_name`,
//!   `a_greyed_out_tool_button_greys_its_icon_as_it_greys_its_name`,
//!   `the_tool_in_hand_turns_its_icon_amber_with_its_name`; with names off,
//!   the icon alone wears what its name would have been written in —
//!   `with_names_off_an_icon_at_rest_wears_the_colour_its_name_would_be_written_in`,
//!   `with_names_off_a_greyed_out_button_greys_its_icon_as_it_would_its_name`,
//!   `with_names_off_the_tool_in_hand_turns_its_icon_amber`; and in a menu —
//!   `a_greyed_out_menu_entry_greys_its_icon_as_it_greys_its_name`,
//!   `the_menu_entry_in_hand_turns_its_icon_amber_with_its_name`
//!
//! Closes #731. The ribbon around these buttons is held in
//! `crates/app/tests/ribbon_groups.rs`.
//! - Large button (C1): at least 50x64 —
//!   `a_large_button_is_at_least_50_wide_and_64_tall`
//! - icon of 22 px above the name —
//!   `a_large_button_wears_an_icon_of_22_points_above_its_name`
//! - the key in the top right corner —
//!   `a_large_button_writes_its_key_in_its_top_right_corner`
//! - drawn apart from the name —
//!   `a_large_button_writes_its_key_apart_from_its_name`
//! - in monospace, 9.5 px, the colour `text.shortcut` —
//!   `a_large_button_writes_its_key_small_in_monospace_and_the_shortcut_colour`
//! - two lines for a long name — `a_long_name_on_a_large_button_takes_two_lines`
//! - and one for a short name —
//!   `a_short_name_on_a_large_button_stays_on_one_line`
//! - its accessible label stays its name and its key, `Ligne (L)`, as the
//!   driver finds it — `a_large_button_is_called_by_its_name_and_its_key`
//! - state normal: no background, text `text.button` —
//!   `a_large_button_at_rest_has_no_background_and_writes_in_the_button_colour`
//! - state hover: background `background.hover_strong` —
//!   `a_hovered_large_button_wears_the_strong_hover_background`
//! - state active (amber): background, border and text —
//!   `the_tool_in_hand_is_amber_in_its_background_its_border_and_its_text`
//! - state disabled: text `text.disabled`, no hover —
//!   `a_greyed_out_large_button_writes_in_the_disabled_colour_and_does_not_answer_to_hover`
//! - all its text, read so: the key of a disabled button is `text.disabled`
//!   too, never brighter than its name —
//!   `a_greyed_out_large_button_writes_its_key_in_the_disabled_colour`
//!
//! The colour an icon shows is the picture it is painted from times the tint
//! it is painted with, which is how the GPU will draw it. Both are read: a
//! picture drawn black stays black under any tint, and a white one under a
//! tint that never changes wears one colour in every state.
//!
//! What is read are the shapes the frame painted, not the triangles they
//! become: `architecture.rs` keeps the word for those to `cao_solid`, whose
//! matter it names.

use std::sync::Arc;

use super::*;
use crate::ui::icon::{self, Icon};
use crate::ui::theme::tokens::{
    ACCENT_AMBER, ACCENT_AMBER_BACKGROUND, ACCENT_AMBER_BORDER, ACCENT_AMBER_LIGHT,
    BACKGROUND_HOVER_STRONG, TEXT_BUTTON, TEXT_DISABLED, TEXT_SHORTCUT,
};

/// How far two colours may drift per channel and still read as one: what
/// multiplying through a premultiplied alpha costs in rounding.
const SAME_COLOUR: u8 = 2;

/// How far two sizes may differ and still be the one the redesign asks for:
/// a texture and a galley are placed on rounded points.
const SIZED_WITHIN: f32 = 0.5;

/// How far from an edge of the button a thing still sits in that corner: the
/// padding of a frame, and a little.
const CORNER_REACH: f32 = 10.0;

#[test]
fn at_rest_a_tool_button_s_icon_wears_the_colour_of_its_name() {
    let worn = worn(icon::LINE, true, false);

    assert_alike(worn.icon, worn.name(), "at rest");
}

#[test]
fn a_greyed_out_tool_button_greys_its_icon_as_it_greys_its_name() {
    let at_rest = worn(icon::LINE, true, false);
    let greyed = worn(icon::LINE, false, false);

    assert_alike(greyed.icon, greyed.name(), "greyed out");
    assert!(
        !alike(greyed.icon, at_rest.icon),
        "a greyed-out button's icon shows {:?}, as it does at rest: nothing says it cannot \
         be pressed",
        greyed.icon,
    );
}

#[test]
fn the_tool_in_hand_turns_its_icon_amber_with_its_name() {
    let in_hand = worn(icon::LINE, true, true);

    assert_alike(in_hand.icon, in_hand.name(), "in hand");
    assert_alike(
        in_hand.icon,
        ACCENT_AMBER_LIGHT,
        "in hand, against the theme's light amber",
    );
}

#[test]
fn with_names_off_an_icon_at_rest_wears_the_colour_its_name_would_be_written_in() {
    let alone = worn_alone(true, false);

    assert_alike(
        alone.icon,
        worn(icon::LINE, true, false).name(),
        "alone, at rest",
    );
}

#[test]
fn with_names_off_a_greyed_out_button_greys_its_icon_as_it_would_its_name() {
    let at_rest = worn_alone(true, false);
    let greyed = worn_alone(false, false);

    assert_alike(
        greyed.icon,
        worn(icon::LINE, false, false).name(),
        "alone, greyed out",
    );
    assert!(
        !alike(greyed.icon, at_rest.icon),
        "a greyed-out icon alone shows {:?}, as it does at rest: nothing says it cannot \
         be pressed",
        greyed.icon,
    );
}

#[test]
fn with_names_off_the_tool_in_hand_turns_its_icon_amber() {
    let in_hand = worn_alone(true, true);

    assert_alike(
        in_hand.icon,
        worn(icon::LINE, true, true).name(),
        "alone, in hand",
    );
    assert_alike(
        in_hand.icon,
        ACCENT_AMBER_LIGHT,
        "alone, in hand, against the theme's light amber",
    );
}

#[test]
fn a_greyed_out_menu_entry_greys_its_icon_as_it_greys_its_name() {
    let at_rest = worn_in_a_menu(true, false);
    let greyed = worn_in_a_menu(false, false);

    assert_alike(greyed.icon, greyed.name(), "a menu entry greyed out");
    assert!(
        !alike(greyed.icon, at_rest.icon),
        "a greyed-out menu entry's icon shows {:?}, as it does at rest",
        greyed.icon,
    );
}

/// Greying out only fades, so a tint fixed at the colour of rest would fade
/// with the name and pass for it: the entry in hand is where it shows.
#[test]
fn the_menu_entry_in_hand_turns_its_icon_amber_with_its_name() {
    let in_hand = worn_in_a_menu(true, true);

    assert_alike(in_hand.icon, in_hand.name(), "a menu entry in hand");
    assert_alike(
        in_hand.icon,
        ACCENT_AMBER,
        "a menu entry in hand, against the theme's amber",
    );
}

#[test]
fn a_large_button_is_at_least_50_wide_and_64_tall() {
    let button = at_rest("I", None).button;

    assert!(
        button.width() >= 50.0 && button.height() >= 64.0,
        "a large button with a short name measures {:?}, where C1 asks for 50 by 64 at least",
        button.size(),
    );
}

#[test]
fn a_large_button_wears_an_icon_of_22_points_above_its_name() {
    let drawn = at_rest("Ligne", None);
    let icon = drawn.icon();
    let (name, _) = drawn.written("Ligne");

    assert!(
        (icon.width() - 22.0).abs() < SIZED_WITHIN && (icon.height() - 22.0).abs() < SIZED_WITHIN,
        "the icon is painted at {:?} points, where C1 draws it at 22",
        icon.size(),
    );
    assert!(
        icon.bottom() <= name.top() && name.x_range().contains(icon.center().x),
        "the icon sits at {icon:?} and the name at {name:?}: the icon is not above the name",
    );
}

#[test]
fn a_large_button_writes_its_key_in_its_top_right_corner() {
    let drawn = at_rest("Ligne", Some("L"));
    let button = drawn.button;
    let (key, _) = drawn.written("L");

    assert!(
        button.right() - key.right() <= CORNER_REACH
            && key.top() - button.top() <= CORNER_REACH
            && button.expand(1.0).contains_rect(key),
        "the key sits at {key:?} in a button at {button:?}: not in its top right corner",
    );
}

#[test]
fn a_large_button_writes_its_key_apart_from_its_name() {
    let drawn = at_rest("Ligne", Some("L"));
    let (name, _) = drawn.written("Ligne");
    let (key, _) = drawn.written("L");

    assert!(
        !name.intersects(key),
        "the name at {name:?} and the key at {key:?} are written over each other",
    );
}

#[test]
fn a_large_button_writes_its_key_small_in_monospace_and_the_shortcut_colour() {
    let drawn = at_rest("Ligne", Some("L"));
    let (_, key) = drawn.written("L");
    let font = &key.galley.job.sections[0].format.font_id;

    assert_eq!(font.family, egui::FontFamily::Monospace, "the key's font");
    assert!(
        (font.size - 9.5).abs() < SIZED_WITHIN,
        "the key is written at {} points, where C1 writes it at 9.5",
        font.size,
    );
    assert_alike(written_in(key), TEXT_SHORTCUT, "the key");
}

#[test]
fn a_long_name_on_a_large_button_takes_two_lines() {
    for long in ["Nouvelle esquisse", "Ajout de matière"] {
        let drawn = at_rest(long, None);
        let (name, text) = drawn.written(long);

        assert_eq!(text.galley.rows.len(), 2, "the lines of « {long} »");
        assert!(
            name.width() <= 70.0 + SIZED_WITHIN,
            "« {long} » is {} wide, where C1 wraps it within 70",
            name.width(),
        );
        assert!(
            drawn.button.expand(1.0).contains_rect(name),
            "« {long} » at {name:?} runs out of its button at {:?}",
            drawn.button,
        );
    }
}

#[test]
fn a_short_name_on_a_large_button_stays_on_one_line() {
    let drawn = at_rest("Ligne", None);
    let (_, text) = drawn.written("Ligne");

    assert_eq!(text.galley.rows.len(), 1, "the lines of « Ligne »");
}

#[test]
fn a_large_button_is_called_by_its_name_and_its_key() {
    assert_eq!(called("Ligne", Some("L")), "Ligne (L)");
    assert_eq!(called("Ligne", None), "Ligne");
}

#[test]
fn a_large_button_at_rest_has_no_background_and_writes_in_the_button_colour() {
    let drawn = at_rest("Ligne", None);
    let (_, name) = drawn.written("Ligne");

    assert_eq!(
        drawn.backgrounds(),
        [],
        "the backgrounds of a button at rest"
    );
    assert_alike(written_in(name), TEXT_BUTTON, "at rest, its name");
}

#[test]
fn a_hovered_large_button_wears_the_strong_hover_background() {
    let backgrounds = drawn("Ligne", None, true, false, true).backgrounds();

    assert!(
        backgrounds
            .iter()
            .any(|fill| alike(*fill, BACKGROUND_HOVER_STRONG)),
        "a hovered button is filled with {backgrounds:?}, where C1 asks for {BACKGROUND_HOVER_STRONG:?}",
    );
}

#[test]
fn the_tool_in_hand_is_amber_in_its_background_its_border_and_its_text() {
    let drawn = drawn("Ligne", None, true, true, false);
    let (_, name) = drawn.written("Ligne");
    let frames = drawn.frames();

    assert!(
        frames
            .iter()
            .any(|frame| alike(frame.fill, ACCENT_AMBER_BACKGROUND)),
        "the tool in hand is filled with {:?}, where C1 asks for {ACCENT_AMBER_BACKGROUND:?}",
        drawn.backgrounds(),
    );
    assert!(
        frames
            .iter()
            .any(|frame| frame.stroke.width > 0.0 && alike(frame.stroke.color, ACCENT_AMBER_BORDER)),
        "the tool in hand has the borders {:?}, where C1 asks for {ACCENT_AMBER_BORDER:?}",
        frames.iter().map(|frame| frame.stroke).collect::<Vec<_>>(),
    );
    assert_alike(written_in(name), ACCENT_AMBER_LIGHT, "in hand, its name");
}

#[test]
fn a_greyed_out_large_button_writes_in_the_disabled_colour_and_does_not_answer_to_hover() {
    let drawn = drawn("Ligne", None, false, false, true);
    let (_, name) = drawn.written("Ligne");

    assert_alike(written_in(name), TEXT_DISABLED, "greyed out, its name");
    assert_eq!(
        drawn.backgrounds(),
        [],
        "the backgrounds of a greyed-out button under the pointer",
    );
}

#[test]
fn a_greyed_out_large_button_writes_its_key_in_the_disabled_colour() {
    let drawn = drawn("Ligne", Some("L"), false, false, false);
    let (_, key) = drawn.written("L");

    assert_alike(written_in(key), TEXT_DISABLED, "greyed out, its key");
}

/// The colours a button shows in one frame: what a pixel its icon fully
/// covers shows, and what its name's glyphs are painted in, if it writes one.
struct Worn {
    icon: egui::Color32,
    name: Option<egui::Color32>,
}

impl Worn {
    fn name(&self) -> egui::Color32 {
        self.name.expect("the button wrote no name")
    }
}

fn worn(icon: Icon, enabled: bool, on: bool) -> Worn {
    worn_by(|ui| tool_button(ui, enabled, on, icon, "Line", None))
}

fn worn_alone(enabled: bool, on: bool) -> Worn {
    let alone = worn_by(|ui| icon_button(ui, enabled, on, icon::LINE, "Line"));
    assert!(
        alone.name.is_none(),
        "with names off, the button still writes its name",
    );
    alone
}

fn worn_in_a_menu(enabled: bool, on: bool) -> Worn {
    worn_by(|ui| menu_entry(ui, enabled, on, icon::LINE, "Line".to_owned()))
}

fn worn_by(mut draw: impl FnMut(&mut egui::Ui) -> egui::Response) -> Worn {
    let ctx = egui::Context::default();
    crate::ui::theme::install(&ctx);
    icon::install(&ctx);

    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(400.0, 200.0),
        )),
        ..Default::default()
    };
    let mut button = egui::Rect::NOTHING;
    let mut output = ctx.run_ui(input, |ui| {
        button = draw(ui).rect;
    });
    let mut pictures = std::mem::take(&mut output.textures_delta.set);
    output.textures_delta.clear();

    let shapes = every_shape(output.shapes);
    let within = button.expand(1.0);

    let drawn = shapes
        .iter()
        .find_map(|shape| match shape {
            egui::Shape::Rect(rect)
                if rect.fill_texture_id() != egui::TextureId::default()
                    && within.contains_rect(rect.rect) =>
            {
                Some(rect)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("the button painted no picture inside {button:?}"));
    let picture: Arc<egui::ColorImage> = pictures
        .remove(&drawn.fill_texture_id())
        .into_iter()
        .flatten()
        .find(|delta| delta.pos.is_none())
        .map(|delta| {
            let egui::ImageData::Color(image) = delta.image;
            image
        })
        .expect("the button painted a picture the frame never handed over");

    let name = shapes.iter().find_map(|shape| match shape {
        egui::Shape::Text(text) if within.contains_rect(shape.visual_bounding_rect()) => {
            Some(written_in(text))
        }
        _ => None,
    });

    Worn {
        icon: seen_through(drawn_in(&picture), drawn.fill),
        name,
    }
}

/// What a large button painted, in the last of a few frames.
struct Drawn {
    button: egui::Rect,
    shapes: Vec<egui::Shape>,
}

impl Drawn {
    fn icon(&self) -> egui::Rect {
        self.shapes
            .iter()
            .find_map(|shape| match shape {
                egui::Shape::Rect(rect) if rect.fill_texture_id() != egui::TextureId::default() => {
                    Some(rect.rect)
                }
                _ => None,
            })
            .expect("the button painted no icon")
    }

    fn written(&self, exactly: &str) -> (egui::Rect, &egui::epaint::TextShape) {
        self.shapes
            .iter()
            .find_map(|shape| match shape {
                egui::Shape::Text(text) if text.galley.text() == exactly => {
                    Some((shape.visual_bounding_rect(), text))
                }
                _ => None,
            })
            .unwrap_or_else(|| {
                panic!(
                    "nothing written reads exactly {exactly:?}; the button wrote {:?}",
                    self.written_texts(),
                )
            })
    }

    fn written_texts(&self) -> Vec<String> {
        self.shapes
            .iter()
            .filter_map(|shape| match shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect()
    }

    /// The rectangles painted over the button that are no picture.
    fn frames(&self) -> Vec<&egui::epaint::RectShape> {
        let within = self.button.expand(1.0);
        self.shapes
            .iter()
            .filter_map(|shape| match shape {
                egui::Shape::Rect(rect)
                    if rect.fill_texture_id() == egui::TextureId::default()
                        && within.contains_rect(rect.rect) =>
                {
                    Some(rect)
                }
                _ => None,
            })
            .collect()
    }

    fn backgrounds(&self) -> Vec<egui::Color32> {
        self.frames()
            .iter()
            .map(|frame| frame.fill)
            .filter(|fill| fill.a() > 0)
            .collect()
    }
}

fn at_rest(name: &str, key: Option<&str>) -> Drawn {
    drawn(name, key, true, false, false)
}

fn themed() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::theme::install(&ctx);
    icon::install(&ctx);
    ctx
}

fn screen(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(400.0, 200.0),
        )),
        events,
        ..Default::default()
    }
}

/// The first frame finds where the button lands; the pointer comes onto it
/// after that, the frame after which a widget learns it is hovered.
fn drawn(name: &str, key: Option<&str>, enabled: bool, on: bool, hovered: bool) -> Drawn {
    let ctx = themed();
    let mut button = egui::Rect::NOTHING;
    let mut shapes = Vec::new();

    for frame in 0..4 {
        let events = if hovered && frame > 0 {
            vec![egui::Event::PointerMoved(button.center())]
        } else {
            Vec::new()
        };
        let mut output = ctx.run_ui(screen(events), |ui| {
            button = tool_button(ui, enabled, on, icon::LINE, name, key).rect;
        });
        output.textures_delta.clear();
        shapes = every_shape(output.shapes);
    }

    Drawn { button, shapes }
}

/// What a button is called for whoever cannot see it, and for the driver.
fn called(name: &str, key: Option<&str>) -> String {
    let ctx = themed();
    ctx.enable_accesskit();
    let mut output = ctx.run_ui(screen(Vec::new()), |ui| {
        tool_button(ui, true, false, icon::LINE, name, key);
    });
    output.textures_delta.clear();

    let update = output
        .platform_output
        .accesskit_update
        .expect("the frame describes itself for accessibility");
    update
        .nodes
        .iter()
        .find(|(_, node)| node.role() == egui::accesskit::Role::Button)
        .and_then(|(_, node)| node.label())
        .expect("a button with a label")
        .to_owned()
}

/// The shapes a frame painted, with every group opened.
fn every_shape(clipped: Vec<egui::epaint::ClippedShape>) -> Vec<egui::Shape> {
    let mut pending: Vec<egui::Shape> = clipped.into_iter().map(|clipped| clipped.shape).collect();
    let mut shapes = Vec::new();
    while let Some(shape) = pending.pop() {
        match shape {
            egui::Shape::Vec(group) => pending.extend(group),
            shape => shapes.push(shape),
        }
    }
    shapes
}

/// The colour a text's glyphs reach the screen in, settled as epaint's
/// tessellator settles it: the override if there is one, the fallback for a
/// glyph left to it, then the fading of a greyed-out widget.
fn written_in(text: &egui::epaint::TextShape) -> egui::Color32 {
    let visuals = text
        .galley
        .rows
        .iter()
        .map(|placed| &placed.row.visuals)
        .find(|visuals| !visuals.glyph_vertex_range.is_empty())
        .expect("a name has glyphs");
    let glyph = visuals.mesh.vertices[visuals.glyph_vertex_range.start].color;

    let colour = match text.override_text_color {
        Some(overridden) => overridden,
        None if glyph == egui::Color32::PLACEHOLDER => text.fallback_color,
        None => glyph,
    };
    if text.opacity_factor < 1.0 {
        colour.gamma_multiply(text.opacity_factor)
    } else {
        colour
    }
}

/// The colour the picture's strokes are drawn in, read where they cover a
/// pixel most and freed of that coverage.
fn drawn_in(picture: &egui::ColorImage) -> [u8; 3] {
    let densest = picture
        .pixels
        .iter()
        .max_by_key(|pixel| pixel.a())
        .expect("a picture has pixels");
    assert!(densest.a() > 0, "the icon's picture is blank");

    let freed = |channel: u8| (u32::from(channel) * 255 / u32::from(densest.a())).min(255) as u8;
    [freed(densest.r()), freed(densest.g()), freed(densest.b())]
}

/// What a pixel drawn in `drawn` and painted with `tint` shows.
fn seen_through(drawn: [u8; 3], tint: egui::Color32) -> egui::Color32 {
    let times = |channel: u8, by: u8| (u32::from(channel) * u32::from(by) / 255) as u8;
    egui::Color32::from_rgba_premultiplied(
        times(drawn[0], tint.r()),
        times(drawn[1], tint.g()),
        times(drawn[2], tint.b()),
        tint.a(),
    )
}

fn alike(one: egui::Color32, other: egui::Color32) -> bool {
    one.to_array()
        .iter()
        .zip(other.to_array())
        .all(|(a, b)| a.abs_diff(b) <= SAME_COLOUR)
}

fn assert_alike(icon: egui::Color32, expected: egui::Color32, state: &str) {
    assert!(
        alike(icon, expected),
        "{state}, the button shows {icon:?} where {expected:?} is expected",
    );
}
