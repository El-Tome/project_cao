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
use crate::ui::theme::tokens::ACCENT_AMBER;

/// How far two colours may drift per channel and still read as one: what
/// multiplying through a premultiplied alpha costs in rounding.
const SAME_COLOUR: u8 = 2;

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
        ACCENT_AMBER,
        "in hand, against the theme's amber",
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
        ACCENT_AMBER,
        "alone, in hand, against the theme's amber",
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
    worn_by(|ui| tool_button(ui, enabled, on, icon, "Line".to_owned()))
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
        "{state}, the button's icon shows {icon:?} where {expected:?} is expected",
    );
}
