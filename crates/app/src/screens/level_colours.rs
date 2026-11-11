//! The colour each level of a part's alerts is seen in: the one the profile's
//! theme chose over the chrome's tokens, when it chose one.

use cao_prefs::{Rgba, Theme};

use crate::ui::badge::Level;

/// The colour the theme chose for `level`; nothing keeps its tokens.
pub(crate) fn chosen(theme: &Theme, level: Level) -> Option<egui::Color32> {
    let chosen = match level {
        Level::Warning => theme.warning,
        Level::Error => theme.error,
    };
    chosen.map(painted)
}

/// The patch and the ink `level` is worn in, as the theme has it.
pub(crate) fn worn(theme: &Theme, level: Level) -> (egui::Color32, egui::Color32) {
    level.colours(chosen(theme, level))
}

fn painted(color: Rgba) -> egui::Color32 {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    egui::Color32::from_rgba_unmultiplied(
        channel(color.r),
        channel(color.g),
        channel(color.b),
        channel(color.a),
    )
}
