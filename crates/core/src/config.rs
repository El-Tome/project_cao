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
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    Micrometer,
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
}

impl LengthUnit {
    pub fn suffix(self) -> &'static str {
        match self {
            Self::Micrometer => "µm",
            Self::Millimeter => "mm",
            Self::Centimeter => "cm",
            Self::Meter => "m",
            Self::Kilometer => "km",
        }
    }

    /// How many millimetres one of this unit is worth.
    pub fn millimeters(self) -> f32 {
        match self {
            Self::Micrometer => 0.001,
            Self::Millimeter => 1.0,
            Self::Centimeter => 10.0,
            Self::Meter => 1000.0,
            Self::Kilometer => 1_000_000.0,
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
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    /// Zoom per pixel of trackpad scroll.
    pub zoom_sensitivity: f32,
    /// Zoom per notch of mouse wheel. A notch reports one line, not fifty
    /// pixels, so it needs its own much larger factor.
    pub wheel_zoom_sensitivity: f32,
    /// How close and how far the camera may get, in world units (mm).
    pub min_distance: f32,
    pub max_distance: f32,
    /// Smallest on-screen spacing, in pixels, before the grid step grows.
    pub grid_pixel_spacing: f32,
    /// Whether the cursor is pulled onto the grid while drawing.
    pub grid_snap: bool,
    /// How many parts each grid square is divided into for snapping.
    pub grid_snap_divisions: u32,
    /// How close, in pixels, the cursor must be for the grid to pull it.
    pub grid_snap_pixels: f32,
    /// How close, in pixels, the cursor must be for a line already drawn to
    /// pull it. Larger than the grid's: drawing onto a line already there is
    /// far more common than drawing near it.
    #[serde(default = "default_segment_snap_pixels")]
    pub segment_snap_pixels: f32,
    /// Corner the scale bar sits in.
    pub ruler_corner: ViewportCorner,
    pub ruler_visible: bool,
    pub unit: UnitDisplay,
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
            wheel_zoom_sensitivity: 0.12,
            min_distance: 1e-3,
            max_distance: 1e9,
            grid_pixel_spacing: 48.0,
            grid_snap: true,
            grid_snap_divisions: 4,
            grid_snap_pixels: 12.0,
            segment_snap_pixels: default_segment_snap_pixels(),
            ruler_corner: ViewportCorner::BottomLeft,
            ruler_visible: true,
            unit: UnitDisplay::Auto,
        }
    }
}

fn default_segment_snap_pixels() -> f32 {
    28.0
}

/// How the scale bar labels a length: always in the same unit, or in whichever
/// one keeps the number readable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitDisplay {
    Auto,
    Fixed(LengthUnit),
}

impl UnitDisplay {
    /// Picks the unit that keeps a length short: no "50000 mm" when "50 m"
    /// says the same thing. Centimetres are skipped, being unusual in
    /// mechanical design.
    pub fn unit_for(self, millimeters: f32) -> LengthUnit {
        match self {
            Self::Fixed(unit) => unit,
            Self::Auto => {
                let magnitude = millimeters.abs();
                if magnitude >= 1_000_000.0 {
                    LengthUnit::Kilometer
                } else if magnitude >= 1000.0 {
                    LengthUnit::Meter
                } else if magnitude >= 0.1 {
                    LengthUnit::Millimeter
                } else {
                    LengthUnit::Micrometer
                }
            }
        }
    }

    pub fn format(self, millimeters: f32) -> String {
        self.unit_for(millimeters).format(millimeters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_units_keep_numbers_short() {
        assert_eq!(UnitDisplay::Auto.format(10.0), "10 mm");
        assert_eq!(UnitDisplay::Auto.format(500.0), "500 mm");
        assert_eq!(UnitDisplay::Auto.format(1000.0), "1 m");
        assert_eq!(UnitDisplay::Auto.format(50_000.0), "50 m");
        assert_eq!(UnitDisplay::Auto.format(2_000_000.0), "2 km");
        assert_eq!(UnitDisplay::Auto.format(0.05), "50 µm");
        assert_eq!(
            UnitDisplay::Fixed(LengthUnit::Millimeter).format(50_000.0),
            "50000 mm"
        );
    }

    #[test]
    fn lengths_are_formatted_without_useless_decimals() {
        assert_eq!(LengthUnit::Millimeter.format(10.0), "10 mm");
        assert_eq!(LengthUnit::Millimeter.format(0.5), "0.5 mm");
        assert_eq!(LengthUnit::Millimeter.format(1000.0), "1000 mm");
        assert_eq!(LengthUnit::Meter.format(1000.0), "1 m");
        assert_eq!(LengthUnit::Centimeter.format(25.0), "2.5 cm");
    }
}
