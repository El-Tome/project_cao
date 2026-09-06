use serde::{Deserialize, Serialize};

/// A colour as the user picks it: sRGB channels from 0 to 1, plus opacity.
///
/// Stored in sRGB rather than the linear space the shader blends in, because
/// this is what a colour picker shows and what a shared profile should carry.
/// The conversion happens once, where the geometry is built.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn opaque(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    pub fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn from_array([r, g, b, a]: [f32; 4]) -> Self {
        Self { r, g, b, a }
    }

    /// Straight interpolation, opacity included.
    pub fn mix(self, other: Self, amount: f32) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        let blend = |a: f32, b: f32| a + (b - a) * amount;
        Self {
            r: blend(self.r, other.r),
            g: blend(self.g, other.g),
            b: blend(self.b, other.b),
            a: blend(self.a, other.a),
        }
    }
}

/// One colour along a gradient, at a position from 0 to 1.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub at: f32,
    pub color: Rgba,
}

impl Stop {
    pub const fn new(at: f32, color: Rgba) -> Self {
        Self { at, color }
    }
}

/// What is painted behind everything else.
///
/// Kept as a list of stops rather than a fixed pair of colours: two-colour
/// gradients are what people ask for first and three-colour ones immediately
/// after, and a list costs nothing more to draw.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Background {
    Solid(Rgba),
    /// Straight across the viewport, at any angle. 0° runs bottom to top.
    Linear { angle_degrees: f32, stops: Vec<Stop> },
    /// Out from a point, given in fractions of the viewport.
    Radial {
        center: [f32; 2],
        radius: f32,
        stops: Vec<Stop>,
    },
}

impl Background {
    pub fn stops(&self) -> &[Stop] {
        match self {
            Self::Solid(_) => &[],
            Self::Linear { stops, .. } | Self::Radial { stops, .. } => stops,
        }
    }

    pub fn stops_mut(&mut self) -> Option<&mut Vec<Stop>> {
        match self {
            Self::Solid(_) => None,
            Self::Linear { stops, .. } | Self::Radial { stops, .. } => Some(stops),
        }
    }

    /// The colour at `t` along the gradient, with the stops taken in order.
    ///
    /// Out-of-order or duplicated stops are not an error: the user is moving
    /// them about, and a gradient that refuses to draw while being edited is
    /// worse than one that reads its stops as given.
    pub fn sample(&self, t: f32) -> Rgba {
        let stops = match self {
            Self::Solid(color) => return *color,
            Self::Linear { stops, .. } | Self::Radial { stops, .. } => stops,
        };
        let Some(first) = stops.first() else {
            return Rgba::opaque(0.0, 0.0, 0.0);
        };
        if t <= first.at {
            return first.color;
        }
        for pair in stops.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            if t <= b.at {
                let span = b.at - a.at;
                let amount = if span.abs() < 1e-6 {
                    1.0
                } else {
                    (t - a.at) / span
                };
                return a.color.mix(b.color, amount);
            }
        }
        stops.last().map(|stop| stop.color).unwrap_or(first.color)
    }
}

/// Every colour the viewport draws with.
///
/// All of it here rather than scattered through the rendering code: a colour
/// that lives in a `const` somewhere is a colour nobody can change.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub background: Background,

    pub axis_x: Rgba,
    pub axis_y: Rgba,
    pub axis_z: Rgba,
    pub axis_width: f32,

    pub grid_minor: Rgba,
    pub grid_major: Rgba,
    pub grid_minor_width: f32,
    pub grid_major_width: f32,
    /// Every n-th grid line is a major one.
    pub grid_major_every: i32,

    /// A sketch element that can still move.
    pub sketch_free: Rgba,
    /// One that is entirely held in place.
    pub sketch_settled: Rgba,
    /// A sketch other than the one being edited.
    pub sketch_inactive: Rgba,
    pub sketch_width: f32,

    pub dimension: Rgba,
    /// A read-only dimension: it reports rather than decides.
    pub dimension_driven: Rgba,
    /// Something held in place by a rule, which cannot be dragged any more.
    #[serde(default = "Theme::default_fixed")]
    pub fixed: Rgba,
    /// The marks of the rules, written beside what they hold.
    #[serde(default = "Theme::default_rule")]
    pub rule: Rgba,

    /// Tint of a closed area of a sketch. Deeper areas get more of it.
    pub region_fill: Rgba,
    pub solid: Rgba,
    /// A plane or a face under the cursor.
    pub highlight: Rgba,
    pub extrusion_add: Rgba,
    pub extrusion_cut: Rgba,
}

impl Theme {
    /// Warm and clearly apart from the rest: what is fixed should read as
    /// fixed at a glance, not as one more yellow trait.
    fn default_fixed() -> Rgba {
        Rgba::new(0.95, 0.45, 0.25, 1.0)
    }

    /// Cool and quiet: the marks are there to be read when looked for, not to
    /// compete with the drawing.
    fn default_rule() -> Rgba {
        Rgba::new(0.55, 0.75, 0.95, 1.0)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Background::Linear {
                angle_degrees: 0.0,
                stops: vec![
                    Stop::new(0.0, Rgba::opaque(0.04, 0.05, 0.07)),
                    Stop::new(1.0, Rgba::opaque(0.10, 0.12, 0.16)),
                ],
            },

            axis_x: Rgba::opaque(0.90, 0.30, 0.35),
            axis_y: Rgba::opaque(0.45, 0.75, 0.30),
            axis_z: Rgba::opaque(0.30, 0.55, 0.95),
            axis_width: 2.0,

            grid_minor: Rgba::new(0.55, 0.58, 0.62, 0.35),
            grid_major: Rgba::new(0.65, 0.68, 0.73, 0.60),
            grid_minor_width: 1.0,
            grid_major_width: 1.5,
            grid_major_every: 10,

            sketch_free: Rgba::opaque(0.98, 0.85, 0.35),
            sketch_settled: Rgba::opaque(0.45, 0.85, 0.55),
            sketch_inactive: Rgba::new(0.70, 0.72, 0.76, 0.8),
            sketch_width: 2.5,

            dimension: Rgba::new(0.98, 0.85, 0.35, 0.9),
            dimension_driven: Rgba::new(0.62, 0.65, 0.70, 0.8),

            region_fill: Rgba::new(0.45, 0.65, 0.95, 0.10),
            solid: Rgba::opaque(0.78, 0.80, 0.84),
            fixed: Theme::default_fixed(),
            rule: Theme::default_rule(),
            highlight: Rgba::new(0.30, 0.60, 0.95, 0.40),
            extrusion_add: Rgba::new(0.40, 0.85, 0.60, 0.45),
            extrusion_cut: Rgba::new(0.95, 0.45, 0.40, 0.45),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gradient() -> Background {
        Background::Linear {
            angle_degrees: 0.0,
            stops: vec![
                Stop::new(0.0, Rgba::opaque(0.0, 0.0, 0.0)),
                Stop::new(0.5, Rgba::opaque(1.0, 0.0, 0.0)),
                Stop::new(1.0, Rgba::opaque(1.0, 1.0, 1.0)),
            ],
        }
    }

    #[test]
    fn a_gradient_reads_between_its_stops() {
        let background = gradient();
        assert_eq!(background.sample(0.0), Rgba::opaque(0.0, 0.0, 0.0));
        assert_eq!(background.sample(0.5), Rgba::opaque(1.0, 0.0, 0.0));
        assert_eq!(background.sample(1.0), Rgba::opaque(1.0, 1.0, 1.0));

        let quarter = background.sample(0.25);
        assert!((quarter.r - 0.5).abs() < 1e-5, "{quarter:?}");
        assert!(quarter.g.abs() < 1e-5);
    }

    /// Past either end, a gradient holds its last colour rather than fading to
    /// nothing.
    #[test]
    fn a_gradient_holds_its_ends() {
        let background = gradient();
        assert_eq!(background.sample(-3.0), Rgba::opaque(0.0, 0.0, 0.0));
        assert_eq!(background.sample(9.0), Rgba::opaque(1.0, 1.0, 1.0));
    }

    #[test]
    fn a_gradient_with_no_stops_still_gives_a_colour() {
        let empty = Background::Linear {
            angle_degrees: 0.0,
            stops: Vec::new(),
        };
        assert_eq!(empty.sample(0.5), Rgba::opaque(0.0, 0.0, 0.0));
    }

    #[test]
    fn a_solid_background_is_the_same_everywhere() {
        let background = Background::Solid(Rgba::opaque(0.2, 0.3, 0.4));
        assert_eq!(background.sample(0.0), background.sample(1.0));
    }
}
