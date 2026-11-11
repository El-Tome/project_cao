use std::collections::BTreeMap;

use cao_part::{
    ExtrusionMode, Formula, Operation, PartDocument, RevolutionAxis, Unusable, Variables,
    VolumeState,
};
use cao_sketch::{Area, SketchAxis};
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
    /// The step of matter the panel was reopened on, which applying changes
    /// rather than raising another.
    pub reopened: Option<Reopened>,
}

/// A step of matter reopened: the number of the operation that raised it, and
/// the areas it stands on, kept as they were while the picks are left alone.
#[derive(Clone, Debug, PartialEq)]
pub struct Reopened {
    pub raised_by: u32,
    pub areas: Vec<Area>,
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
            reopened: None,
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

    /// The panel reopened on the step of matter at `step` in the history, as
    /// the part now replays it: its sketch, its mode, its shape, its value as
    /// written, and the places its areas were picked at. A value the other way
    /// along the plane comes back written the right way, the direction
    /// reversed. Nothing for any other step.
    pub fn reopened(document: &PartDocument, step: usize) -> Option<Self> {
        let raised_by = document.history.number_at(step)?;
        let (sketch, areas, value, mode, axis) = match document.raised_as(raised_by)? {
            Operation::Extrude {
                sketch,
                areas,
                distance,
                mode,
            } => (*sketch, areas, distance, *mode, None),
            Operation::Revolve {
                sketch,
                areas,
                axis,
                angle,
                mode,
            } => (*sketch, areas, angle, *mode, Some(*axis)),
            _ => return None,
        };
        let variables = document.variables();
        let reversed = value
            .value(&variables.values())
            .is_some_and(|value| value < 0.0);
        let written = match reversed {
            true => variables.written(&value.clone().negated()),
            false => variables.written(value),
        };
        let mut reopened = Self::default();
        reopened.offer(sketch);
        reopened.mode = Some(mode);
        reopened.picks = areas.iter().map(|area| area.inside).collect();
        reopened.reversed = reversed;
        match axis {
            Some(axis) => {
                reopened.shape = Shape::Revolution;
                reopened.axis = axis;
                reopened.angle_input = written;
            }
            None => reopened.distance_input = written,
        }
        reopened.reopened = Some(Reopened {
            raised_by,
            areas: areas.clone(),
        });
        Some(reopened)
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

    /// A panel reopened on a step closes once the history moves under it: the
    /// step it names may no longer be the one in effect. One armed afresh
    /// stays.
    pub fn history_moved(&mut self) {
        if self.reopened.is_some() {
            self.close();
        }
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

impl Reopened {
    /// Whether these picks are the places its areas were picked at.
    fn is_picked_at(&self, picks: &[DVec2]) -> bool {
        picks
            .iter()
            .copied()
            .eq(self.areas.iter().map(|area| area.inside))
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
#[cfg_attr(not(test), expect(dead_code))]
pub fn apply_extrusion(
    doc: &mut PartDocument,
    extrusion: &mut ExtrusionState,
    notice: &mut Option<String>,
    lang: &Catalogue,
) -> bool {
    apply_extrusion_with(doc, extrusion, notice, lang, |doc, operation| {
        doc.apply(operation);
    })
}

/// The same, the step recorded by `record`: the canvas puts a new one in at
/// the box the part is read to.
pub fn apply_extrusion_with(
    doc: &mut PartDocument,
    extrusion: &mut ExtrusionState,
    notice: &mut Option<String>,
    lang: &Catalogue,
    record: impl FnOnce(&mut PartDocument, Operation),
) -> bool {
    let (Some(sketch), Some(mode)) = (extrusion.sketch, extrusion.mode) else {
        return false;
    };
    if !extrusion.is_ready(doc.variables()) {
        return false;
    }

    // A panel reopened on a step the history has undone since names nothing
    // in effect, and applies nothing.
    let as_raised = match &extrusion.reopened {
        None => None,
        Some(reopened) => match doc.raised_as(reopened.raised_by) {
            Some(raised) => Some(raised.clone()),
            None => return false,
        },
    };
    let reopened = extrusion.reopened.take();
    let step = reopened
        .as_ref()
        .map_or_else(|| doc.history.next_number(), |reopened| reopened.raised_by);
    // The areas are named here and never worked out again: what the click
    // meant is what the drawing said at the moment of the click.
    let picks = std::mem::take(&mut extrusion.picks);
    let areas = match &reopened {
        Some(reopened) if reopened.is_picked_at(&picks) => reopened.areas.clone(),
        _ => doc.areas_at(sketch, &picks),
    };
    let operation = if extrusion.is_revolving() {
        Operation::Revolve {
            sketch,
            areas,
            axis: extrusion.axis,
            angle: extrusion
                .angle(doc.variables())
                .map_or(Formula::Number(0.0), |(written, _)| written),
            mode,
        }
    } else {
        Operation::Extrude {
            sketch,
            areas,
            distance: extrusion
                .distance(doc.variables())
                .map_or(Formula::Number(0.0), |(written, _)| written),
            mode,
        }
    };
    extrusion.mode = None;
    if as_raised.as_ref() == Some(&operation) {
        *notice = None;
        return false;
    }
    // A change is judged by what it does to the part without the step it
    // changes, as a new step is by what it does to the part as it stands.
    let before = match &reopened {
        Some(reopened) => {
            let left_out = BTreeMap::from([(reopened.raised_by, VolumeState::Invisible)]);
            doc.shown(&left_out).body.volume()
        }
        None => doc.body().volume(),
    };
    record(
        doc,
        match reopened {
            Some(reopened) => Operation::EditMatter {
                raised_by: reopened.raised_by,
                to: Box::new(operation),
            },
            None => operation,
        },
    );

    // An extrusion that changes nothing is worth saying out loud: a cut that
    // misses the matter looks exactly like a tool that did not work. One the
    // kernel declined changed nothing either, but saying so would send the
    // user looking for a mistake that is not theirs: that one, and one built
    // at a lean the kernel reads unstably (#552), describe the part rather
    // than answer the gesture, and are its alerts while they hold.
    let alerted = doc.is_declined(step) || doc.unstable_at(step).is_some();
    *notice = (!alerted && unchanged(before, doc.body().volume())).then(|| {
        lang.t(match (mode, extrusion.is_revolving()) {
            (_, true) => "extrusion.nothing_from_revolution",
            (ExtrusionMode::Add, false) => "extrusion.nothing_added",
            (ExtrusionMode::Cut, false) => "extrusion.nothing_removed",
        })
    });
    true
}

/// How near two volumes stand, relative to the larger, to be the same
/// matter.
const SAME_MATTER: f64 = 1e-9;

/// Whether the matter is what it was, read off its volume rather than off the
/// body: the exact kernel hands back a body built anew even from a cut that
/// missed.
fn unchanged(before: f64, after: f64) -> bool {
    (after - before).abs() <= SAME_MATTER * before.abs().max(after.abs())
}

#[cfg(test)]
mod tests;
