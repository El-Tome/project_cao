use serde::{Deserialize, Serialize};

/// A corner of the viewport, used to place overlays such as the orientation
/// cube.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewportCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Mouse buttons, named independently of any UI toolkit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PointerButton {
    Primary,
    Middle,
    Secondary,
}

/// One mouse binding: a button plus the modifiers that must be held.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binding {
    pub button: PointerButton,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

impl Binding {
    pub const fn new(button: PointerButton) -> Self {
        Self {
            button,
            shift: false,
            ctrl: false,
            alt: false,
        }
    }

    pub const fn shift(mut self) -> Self {
        self.shift = true;
        self
    }

    pub const fn ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }

    pub const fn alt(mut self) -> Self {
        self.alt = true;
        self
    }
}

/// Alt + left drag orbits and Alt + Shift + left drag pans in every preset:
/// laptop trackpads have no middle button, which every CAD preset relies on.
const TRACKPAD_ORBIT: Binding = Binding::new(PointerButton::Primary).alt();
const TRACKPAD_PAN: Binding = Binding::new(PointerButton::Primary).alt().shift();

/// Which CAD package's navigation habits to follow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigationPreset {
    Fusion360,
    SolidWorks,
    Blender,
}

const MIDDLE: Binding = Binding::new(PointerButton::Middle);

const ORBIT_MIDDLE_SHIFT: [Binding; 2] = [MIDDLE.shift(), TRACKPAD_ORBIT];
const ORBIT_MIDDLE: [Binding; 2] = [MIDDLE, TRACKPAD_ORBIT];
const PAN_MIDDLE: [Binding; 2] = [MIDDLE, TRACKPAD_PAN];
const PAN_MIDDLE_CTRL: [Binding; 2] = [MIDDLE.ctrl(), TRACKPAD_PAN];
const PAN_MIDDLE_SHIFT: [Binding; 2] = [MIDDLE.shift(), TRACKPAD_PAN];

impl NavigationPreset {
    pub fn orbit(self) -> &'static [Binding] {
        match self {
            Self::Fusion360 => &ORBIT_MIDDLE_SHIFT,
            Self::SolidWorks | Self::Blender => &ORBIT_MIDDLE,
        }
    }

    pub fn pan(self) -> &'static [Binding] {
        match self {
            Self::Fusion360 => &PAN_MIDDLE,
            Self::SolidWorks => &PAN_MIDDLE_CTRL,
            Self::Blender => &PAN_MIDDLE_SHIFT,
        }
    }
}

/// What a trackpad gesture does. A trackpad has no middle button, so its
/// gestures are mapped separately from the mouse bindings above.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackpadGesture {
    Pan,
    Orbit,
    Zoom,
    Ignore,
}

/// Trackpad gestures. A two-finger scroll and a mouse wheel arrive as the same
/// kind of event but in different units (pixels vs lines), which is how they
/// are told apart.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TrackpadConfig {
    /// Two-finger scroll.
    pub scroll: TrackpadGesture,
    /// Two-finger scroll with Shift held.
    pub shift_scroll: TrackpadGesture,
    /// Whether a pinch zooms the view.
    pub pinch_zooms: bool,
    /// Multiplier on two-finger scroll distance.
    pub scroll_sensitivity: f32,
}

impl Default for TrackpadConfig {
    fn default() -> Self {
        Self {
            scroll: TrackpadGesture::Pan,
            shift_scroll: TrackpadGesture::Orbit,
            pinch_zooms: true,
            scroll_sensitivity: 1.0,
        }
    }
}

/// A unit of length for what is displayed to the user. One world unit is one
/// millimetre for now; a part will later be able to carry its own scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LengthUnit {
    Millimeter,
    Centimeter,
    Meter,
}

impl LengthUnit {
    pub fn suffix(self) -> &'static str {
        match self {
            Self::Millimeter => "mm",
            Self::Centimeter => "cm",
            Self::Meter => "m",
        }
    }

    /// How many millimetres one of this unit is worth.
    pub fn millimeters(self) -> f32 {
        match self {
            Self::Millimeter => 1.0,
            Self::Centimeter => 10.0,
            Self::Meter => 1000.0,
        }
    }

    /// Formats a length given in world units (millimetres today), trimming the
    /// decimals that a round value does not need.
    pub fn format(self, millimeters: f32) -> String {
        let value = millimeters / self.millimeters();
        let text = if value >= 100.0 {
            format!("{value:.0}")
        } else if value >= 10.0 {
            format!("{value:.1}")
        } else {
            format!("{value:.2}")
        };
        // Only trailing zeros *after a decimal point* are noise; trimming
        // blindly would turn "1000" into "1".
        let text = if text.contains('.') {
            text.trim_end_matches('0').trim_end_matches('.')
        } else {
            &text
        };
        format!("{text} {}", self.suffix())
    }
}

/// Everything tweakable about the 3D viewport. Serializable so it can be
/// persisted and exposed in a preferences screen later.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ViewportConfig {
    pub cube_corner: ViewportCorner,
    /// Side of the orientation cube's square, in logical points.
    pub cube_size: f32,
    /// Gap between the cube and the viewport edges, in logical points.
    pub cube_margin: f32,
    pub navigation: NavigationPreset,
    pub trackpad: TrackpadConfig,
    /// Radians of rotation per pixel dragged.
    pub orbit_sensitivity: f32,
    pub zoom_sensitivity: f32,
    /// Smallest on-screen spacing, in pixels, before the grid step grows.
    pub grid_pixel_spacing: f32,
    /// Corner the scale bar sits in.
    pub ruler_corner: ViewportCorner,
    pub ruler_visible: bool,
    pub unit: LengthUnit,
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            cube_corner: ViewportCorner::TopRight,
            cube_size: 96.0,
            cube_margin: 12.0,
            navigation: NavigationPreset::Fusion360,
            trackpad: TrackpadConfig::default(),
            orbit_sensitivity: 0.008,
            zoom_sensitivity: 0.0015,
            grid_pixel_spacing: 48.0,
            ruler_corner: ViewportCorner::BottomLeft,
            ruler_visible: true,
            unit: LengthUnit::Millimeter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths_are_formatted_without_useless_decimals() {
        assert_eq!(LengthUnit::Millimeter.format(10.0), "10 mm");
        assert_eq!(LengthUnit::Millimeter.format(0.5), "0.5 mm");
        assert_eq!(LengthUnit::Millimeter.format(1000.0), "1000 mm");
        assert_eq!(LengthUnit::Meter.format(1000.0), "1 m");
        assert_eq!(LengthUnit::Centimeter.format(25.0), "2.5 cm");
    }
}
