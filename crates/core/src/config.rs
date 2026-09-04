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
    /// Radians of rotation per pixel dragged.
    pub orbit_sensitivity: f32,
    pub zoom_sensitivity: f32,
    /// Smallest on-screen spacing, in pixels, before the grid step grows.
    pub grid_pixel_spacing: f32,
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            cube_corner: ViewportCorner::TopRight,
            cube_size: 96.0,
            cube_margin: 12.0,
            navigation: NavigationPreset::Fusion360,
            orbit_sensitivity: 0.008,
            zoom_sensitivity: 0.0015,
            grid_pixel_spacing: 48.0,
        }
    }
}
