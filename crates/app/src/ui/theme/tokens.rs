//! The values of `docs/redesign/tokens.json` the chrome wears, each named
//! after its path there, `color.` left out.

use egui::Color32;

pub const BACKGROUND_PANEL: Color32 = Color32::from_rgb(0x16, 0x18, 0x1D);
pub const BACKGROUND_CARD: Color32 = Color32::from_rgb(0x18, 0x1B, 0x21);
pub const BACKGROUND_FLOATING: Color32 = Color32::from_rgb(0x1C, 0x1F, 0x26);
pub const BACKGROUND_HOVER: Color32 = Color32::from_rgb(0x1F, 0x23, 0x2B);
pub const BACKGROUND_HOVER_STRONG: Color32 = Color32::from_rgb(0x23, 0x27, 0x31);
pub const BACKGROUND_ACTIVE_NEUTRAL: Color32 = Color32::from_rgb(0x2C, 0x31, 0x3B);
pub const BACKGROUND_FIELD: Color32 = Color32::from_rgb(0x12, 0x14, 0x19);

pub const BORDER_SEPARATOR: Color32 = Color32::from_rgb(0x22, 0x26, 0x2E);
pub const BORDER_GROUP: Color32 = Color32::from_rgb(0x26, 0x2A, 0x33);
pub const BORDER_CONTROL: Color32 = Color32::from_rgb(0x2E, 0x33, 0x3D);
pub const BORDER_STRONG: Color32 = Color32::from_rgb(0x3A, 0x40, 0x4C);

pub const TEXT_TITLE: Color32 = Color32::from_rgb(0xF4, 0xF5, 0xF7);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xE6, 0xE8, 0xEC);
pub const TEXT_BUTTON: Color32 = Color32::from_rgb(0xD5, 0xD9, 0xE0);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0xC9, 0xCF, 0xD8);
pub const TEXT_SUBDUED: Color32 = Color32::from_rgb(0xA9, 0xB0, 0xBC);
pub const TEXT_QUIET: Color32 = Color32::from_rgb(0x8A, 0x91, 0x9D);
pub const TEXT_ON_ACCENT: Color32 = Color32::from_rgb(0x1A, 0x12, 0x06);

pub const ACCENT_AMBER: Color32 = Color32::from_rgb(0xF2, 0xA5, 0x41);
pub const ACCENT_AMBER_TEXT: Color32 = Color32::from_rgb(0xF7, 0xC4, 0x7A);
/// `rgba(242,165,65,0.13)`: 0.13 of 255 is 33.
pub const ACCENT_AMBER_BACKGROUND: Color32 =
    Color32::from_rgba_unmultiplied_const(242, 165, 65, 33);

pub const STATE_WARNING_TEXT: Color32 = Color32::from_rgb(0xF0, 0xD6, 0x6E);
pub const STATE_ERROR: Color32 = Color32::from_rgb(0xF0, 0x8A, 0x7E);
pub const STATE_ERROR_TEXT: Color32 = Color32::from_rgb(0xF5, 0xA9, 0x9F);
/// `rgba(240,138,126,0.15)`: 0.15 of 255 is 38.
pub const STATE_ERROR_BACKGROUND: Color32 =
    Color32::from_rgba_unmultiplied_const(240, 138, 126, 38);

pub const RADIUS_BUTTON: u8 = 5;
pub const RADIUS_CONTROL: u8 = 6;
pub const RADIUS_CARD: u8 = 10;
/// `999` there, as round as the side allows: a corner is a `u8` in `egui`,
/// which rounds none past half its side either way.
pub const RADIUS_PILL: u8 = u8::MAX;

pub const SPACING_S: f32 = 4.0;
pub const SPACING_M: f32 = 6.0;
pub const SPACING_L: f32 = 8.0;
pub const SPACING_XXL: f32 = 14.0;

pub const TYPOGRAPHY_SIZES_SECTION_TITLE: f32 = 16.0;
pub const TYPOGRAPHY_SIZES_BODY: f32 = 13.0;
pub const TYPOGRAPHY_SIZES_BUTTON: f32 = 12.5;
pub const TYPOGRAPHY_SIZES_DETAIL: f32 = 11.5;
pub const TYPOGRAPHY_SIZES_MONO_VALUE: f32 = 11.0;
