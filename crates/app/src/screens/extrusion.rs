use cao_part::{ExtrusionMode, Formula, RevolutionAxis, Unusable, Variables};
use cao_sketch::SketchAxis;
use glam::DVec2;

use crate::lang::Catalogue;

/// How the matter is made from the chosen areas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Shape {
    /// Pushed straight along the plane's normal.
    #[default]
    Straight,
    /// Swept around an axis lying in the plane.
    Revolution,
}

/// Everything the extrusion workflow remembers between frames.
///
/// It is deliberately its own module rather than more fields on the sketch
/// editor: turning areas into matter is a different job from drawing them, and
/// the two only meet through the sketch they share.
pub struct ExtrusionState {
    /// The mode in hand, if the tool is armed at all.
    pub mode: Option<ExtrusionMode>,
    /// The sketch whose areas are being picked.
    pub sketch: Option<usize>,
    /// One point inside each chosen area, in the sketch's coordinates.
    pub picks: Vec<DVec2>,
    /// The area under the cursor, as an index into the sketch's areas.
    pub hovered: Option<usize>,
    /// Straight, or swept around an axis.
    pub shape: Shape,
    /// What a revolution turns around.
    pub axis: RevolutionAxis,
    /// Sweep typed by the user, in degrees.
    pub angle_input: String,
    /// Height typed by the user, in millimetres.
    pub distance_input: String,
    /// Whether the matter goes the other way along the plane.
    pub reversed: bool,
}

impl Default for ExtrusionState {
    fn default() -> Self {
        Self {
            mode: None,
            sketch: None,
            picks: Vec::new(),
            hovered: None,
            shape: Shape::default(),
            // The vertical axis is the one a profile is usually drawn beside.
            axis: RevolutionAxis::Sketch(SketchAxis::V),
            angle_input: String::new(),
            distance_input: String::new(),
            reversed: false,
        }
    }
}

impl ExtrusionState {
    pub fn is_active(&self) -> bool {
        self.mode.is_some() && self.sketch.is_some()
    }

    /// Offers the tool on a sketch that has just been finished, without arming
    /// it: the user may simply want to be done.
    pub fn offer(&mut self, sketch: usize) {
        *self = Self {
            sketch: Some(sketch),
            distance_input: "10".to_string(),
            angle_input: "360".to_string(),
            axis: RevolutionAxis::Sketch(SketchAxis::V),
            ..Self::default()
        };
    }

    pub fn is_revolving(&self) -> bool {
        self.shape == Shape::Revolution
    }

    pub fn arm(&mut self, mode: ExtrusionMode) {
        self.mode = Some(mode);
        if self.distance_input.trim().is_empty() {
            self.distance_input = "10".to_string();
        }
        if self.angle_input.trim().is_empty() {
            self.angle_input = "360".to_string();
        }
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// The value typed, in millimetres, as written and as what it comes to,
    /// signed by the chosen direction.
    pub fn distance(&self, variables: &Variables) -> Option<(Formula, f64)> {
        signed(&self.distance_input, self.reversed, variables)
    }

    /// The sweep typed, in degrees, as written and as what it comes to, signed
    /// by the chosen direction.
    pub fn angle(&self, variables: &Variables) -> Option<(Formula, f64)> {
        signed(&self.angle_input, self.reversed, variables)
            .filter(|(_, value)| value.abs() <= 360.0)
    }

    /// What is wrong with the value typed for the shape in hand, when it does
    /// not read — said where it is typed, and nothing is applied.
    pub fn wrong(&self, variables: &Variables) -> Option<Unusable> {
        let typed = match self.shape {
            Shape::Straight => &self.distance_input,
            Shape::Revolution => &self.angle_input,
        };
        match typed.trim().is_empty() {
            true => None,
            false => variables.size_of(typed).err(),
        }
    }

    /// Whether there is enough to apply: areas chosen, and a usable value.
    pub fn is_ready(&self, variables: &Variables) -> bool {
        if self.picks.is_empty() {
            return false;
        }
        match self.shape {
            Shape::Straight => self.distance(variables).is_some(),
            Shape::Revolution => self.angle(variables).is_some(),
        }
    }
}

fn signed(input: &str, reversed: bool, variables: &Variables) -> Option<(Formula, f64)> {
    let (written, value) = variables
        .size_of(input)
        .ok()
        .filter(|(_, value)| value.abs() > 1e-6)?;
    Some(match reversed {
        true => (written.negated(), -value),
        false => (written, value),
    })
}

/// Turns the chosen areas into matter, or takes them out of it.
///
/// `notice` is the one sentence the screen shows, whoever wrote it last. A
/// sentence of the extrusion's own outlived the extrusion, and stood beside
/// whatever the drawing was saying.
pub fn apply_extrusion(
    doc: &mut cao_part::PartDocument,
    extrusion: &mut ExtrusionState,
    notice: &mut Option<String>,
    lang: &Catalogue,
) -> bool {
    let (Some(sketch), Some(mode)) = (extrusion.sketch, extrusion.mode) else {
        return false;
    };
    if !extrusion.is_ready(doc.variables()) {
        return false;
    }

    let before = doc.body().clone();
    // The areas are named here and never worked out again: what the click
    // meant is what the drawing said at the moment of the click.
    let areas = doc.areas_at(sketch, &std::mem::take(&mut extrusion.picks));
    let operation = if extrusion.is_revolving() {
        cao_part::Operation::Revolve {
            sketch,
            areas,
            axis: extrusion.axis,
            angle: extrusion
                .angle(doc.variables())
                .map_or(Formula::Number(0.0), |(written, _)| written),
            mode,
        }
    } else {
        cao_part::Operation::Extrude {
            sketch,
            areas,
            distance: extrusion
                .distance(doc.variables())
                .map_or(Formula::Number(0.0), |(written, _)| written),
            mode,
        }
    };
    doc.apply(operation);

    // An extrusion that changes nothing is worth saying out loud: a cut that
    // misses the matter looks exactly like a tool that did not work.
    *notice = (doc.body() == &before).then(|| {
        lang.t(match (mode, extrusion.is_revolving()) {
            (_, true) => "extrusion.nothing_from_revolution",
            (ExtrusionMode::Add, false) => "extrusion.nothing_added",
            (ExtrusionMode::Cut, false) => "extrusion.nothing_removed",
        })
    });
    extrusion.mode = None;
    true
}

#[cfg(test)]
mod tests;
