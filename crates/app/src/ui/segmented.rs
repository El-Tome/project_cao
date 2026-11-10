//! A segmented selector: C6 of `docs/redesign/components.md`. One segment per
//! choice, side by side in one frame, the chosen one lit.

use egui::{Button, Color32, CornerRadius, Frame, Margin, RichText, Stroke, Ui, vec2};

use crate::ui::theme::tokens::{
    BACKGROUND_ACTIVE_NEUTRAL, BACKGROUND_FIELD, BORDER_GROUP, RADIUS_CONTROL, TEXT_SECONDARY,
    TEXT_TITLE,
};

/// C6's measures in points, which the tokens do not carry: a segment's height
/// and how far inside the frame the segments stand.
const HEIGHT: f32 = 24.0;
const MARGIN: i8 = 2;

/// One segment: what it reads, what it stands for, and whether it is lit.
pub struct Segment<'a, T> {
    pub name: &'a str,
    pub value: T,
    pub lit: bool,
}

/// The selector across the room it is given, its segments sharing it alike.
/// Says what the segment clicked stands for.
pub fn segmented<T: Copy>(ui: &mut Ui, segments: &[Segment<'_, T>]) -> Option<T> {
    let mut chosen = None;
    Frame::new()
        .fill(BACKGROUND_FIELD)
        .stroke(Stroke::new(1.0, BORDER_GROUP))
        .corner_radius(CornerRadius::same(RADIUS_CONTROL))
        .inner_margin(Margin::same(MARGIN))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let width = ui.available_width() / segments.len().max(1) as f32;
            ui.horizontal(|ui| {
                for segment in segments {
                    let (fill, ink) = match segment.lit {
                        true => (BACKGROUND_ACTIVE_NEUTRAL, TEXT_TITLE),
                        false => (Color32::TRANSPARENT, TEXT_SECONDARY),
                    };
                    let button = Button::new(RichText::new(segment.name).color(ink))
                        .fill(fill)
                        .stroke(Stroke::NONE)
                        .corner_radius(CornerRadius::same(RADIUS_CONTROL - MARGIN as u8))
                        .min_size(vec2(width, HEIGHT));
                    if ui.add(button).clicked() {
                        chosen = Some(segment.value);
                    }
                }
            });
        });
    chosen
}
