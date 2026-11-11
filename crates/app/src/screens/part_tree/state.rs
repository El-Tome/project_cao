//! What the tree of a part holds, worked out from the part itself.
//!
//! The history folded under it, `History`, lists what was *done*: one line
//! per operation, newest last. This lists what the part **is** — its bodies and
//! its sketches, and under each sketch what is drawn on it, by kind. A drag,
//! a dimension moved, two corners joined leave no line here, because they
//! leave no element behind.
//!
//! Which means the two disagree on purpose. Reopen a sketch after an extrusion
//! and draw a trait: the tree files it under that sketch, where it belongs,
//! while the design keeps it in the folder of the step that was open when it
//! was drawn, so that replaying it keeps the order it happened in.

use cao_part::PartDocument;
use cao_part::VolumeState;
use cao_part::history::{ExtrusionMode, Operation};
use cao_sketch::{DimensionTarget, Element, Sketch};

use crate::lang::Catalogue;
use crate::screens::sketch_pill::laid_on;
use crate::ui::badge::Level;
use crate::wording::part_tree::{self as said, Problem, Rubric};
use crate::wording::{self, formula, plane};

mod clicks;
mod history;
mod seen;
mod volumes;
pub use clicks::*;
pub use history::History;
pub use seen::Offered;
use volumes::volume;

/// What a line of the tree points at, so that clicking it can find the thing
/// itself.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Points {
    Element { sketch: usize, element: Element },
    Dimension { sketch: usize, rank: usize },
    Rule { sketch: usize, rank: usize },
    Area { sketch: usize, rank: usize },
}

impl Points {
    pub fn sketch(self) -> usize {
        match self {
            Self::Element { sketch, .. }
            | Self::Dimension { sketch, .. }
            | Self::Rule { sketch, .. }
            | Self::Area { sketch, .. } => sketch,
        }
    }
}

/// One line under a heading.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub name: String,
    /// The size it is drawn at, for a trait or a circle.
    pub value: Option<String>,
    pub points: Points,
}

/// One step that made matter, and what it stands on.
#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    /// Where it sits in the design, which is what reopening it asks for.
    pub step: usize,
    pub name: String,
    /// How far it went and which way, in one line, read only.
    pub reads: String,
    pub areas: Vec<Row>,
    /// The gravest of what is wrong with it: declined by the kernel, then an
    /// area it was raised from gone — which leaves no line under the step, so
    /// without it the step would look raised from fewer areas than it was —
    /// then built unstably.
    pub badge: Option<Problem>,
    /// What the user is told of a step the kernel built though it reads it
    /// unstably (#552), when it did.
    pub unstable: Option<String>,
    pub mode: ExtrusionMode,
    /// How it is drawn, written out on its line, and what a click on its
    /// visibility button makes of it; dimmed while invisible.
    pub shown: VolumeState,
    pub state: String,
    pub visibility_button: String,
    pub dimmed: bool,
}

/// One sketch, and what is drawn on it, by kind.
#[derive(Clone, Debug, PartialEq)]
pub struct Drawn {
    pub sketch: usize,
    /// The face it was laid on gone from the part, when it is.
    pub badge: Option<Problem>,
    pub step: usize,
    pub name: String,
    /// What it is laid on, as the sketch pill names it, in short.
    pub plane: String,
    /// Being drawn on; and hidden, which the one drawn on never is.
    pub open: bool,
    pub dimmed: bool,
    pub areas: Vec<Row>,
    pub strokes: Vec<Row>,
    pub dimensions: Vec<Row>,
    pub rules: Vec<Row>,
}

/// A variable, read only: they are edited in a panel of their own.
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    /// What it is written as and what it comes to.
    pub value: String,
}

/// The part, as the tree shows it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartTree {
    pub variables: Vec<Variable>,
    pub bodies: Vec<Body>,
    pub sketches: Vec<Drawn>,
    pub sections: Vec<Kept>,
    /// « Tout rendre opaque », offered while the part has volumes.
    pub make_all_opaque: Option<Offered>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Kept {
    pub id: usize,
    pub name: String,
    pub badge: Option<Problem>,
}

impl Problem {
    pub fn level(self) -> Level {
        match self {
            Self::Unstable | Self::FaceLost => Level::Warning,
            Self::NotBuilt | Self::AreaLost => Level::Error,
        }
    }
}

fn gravest(declined: bool, lost: bool, unstable: bool) -> Option<Problem> {
    [
        (declined, Problem::NotBuilt),
        (lost, Problem::AreaLost),
        (unstable, Problem::Unstable),
    ]
    .into_iter()
    .find_map(|(found, problem)| found.then_some(problem))
}

impl PartTree {
    pub fn of(document: &PartDocument, lang: &Catalogue) -> Self {
        let variables = document.variables();
        let mut tree = Self {
            variables: variables
                .live()
                .map(|(_, variable)| Variable {
                    name: variable.name().to_string(),
                    value: formula::sized(lang, variables, variable.formula(), |value| {
                        value.to_string()
                    }),
                })
                .collect(),
            ..Self::default()
        };
        let mut sketches = 0;

        for (step, operation) in document.history.applied_operations().iter().enumerate() {
            let number = document.history.number_at(step);
            let declined = number.is_some_and(|number| document.is_declined(number));
            let unstable = number
                .and_then(|number| document.unstable_at(number))
                .map(|unstable| wording::unstable::notice(lang, unstable));
            match operation {
                Operation::CreateSketch { .. } | Operation::CreateSketchOnSection { .. } => {
                    let sketch = sketches;
                    sketches += 1;
                    let Some(drawing) = document.sketches().get(sketch) else {
                        continue;
                    };
                    let rank = tree.sketches.len() + 1;
                    let mut shown = drawn(lang, document, drawing, sketch, step, rank);
                    shown.badge = document.is_adrift(sketch).then_some(Problem::FaceLost);
                    shown.plane = laid_on(document, sketch, drawing, lang, plane::short);
                    tree.sketches.push(shown);
                }
                Operation::Extrude { .. } | Operation::Revolve { .. } => {
                    let rank = tree.bodies.len() + 1;
                    let problems = (declined, unstable);
                    let made = volume(lang, document, (step, rank), operation, problems);
                    tree.bodies.extend(made);
                }
                _ => {}
            }
        }
        tree.sections = document
            .sections()
            .into_iter()
            .map(|kept| Kept {
                id: kept.id,
                name: numbered(lang, "part_tree.section", kept.id + 1),
                badge: kept.broken.then_some(Problem::FaceLost),
            })
            .collect();
        tree
    }

    /// The headings the panel shows, top to bottom.
    pub fn rubrics(&self) -> Vec<Rubric> {
        let made = !self.bodies.is_empty();
        [
            (Rubric::Sketches, true),
            (Rubric::Volumes, made),
            (Rubric::Sections, !self.sections.is_empty()),
            (Rubric::Variables, true),
        ]
        .into_iter()
        .filter_map(|(rubric, shown)| shown.then_some(rubric))
        .collect()
    }

    /// The badge of every element that wears one, in the order the panel
    /// shows them.
    pub fn problems(&self) -> impl Iterator<Item = Problem> + '_ {
        let sketches = self.sketches.iter().map(|drawn| drawn.badge);
        let bodies = self.bodies.iter().map(|body| body.badge);
        let sections = self.sections.iter().map(|kept| kept.badge);
        sketches.chain(bodies).chain(sections).flatten()
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
            && self.bodies.is_empty()
            && self.sketches.is_empty()
            && self.sections.is_empty()
    }
}

/// The name of the `rank`th sketch, the one its row and its line in the
/// history both go by.
fn sketch_name(lang: &Catalogue, rank: usize) -> String {
    numbered(lang, "part_tree.sketch", rank)
}

fn numbered(lang: &Catalogue, key: &str, rank: usize) -> String {
    lang.t_with(key, &[("rank", &rank.to_string())])
}

fn drawn(
    lang: &Catalogue,
    document: &PartDocument,
    sketch: &Sketch,
    index: usize,
    step: usize,
    rank: usize,
) -> Drawn {
    let measured = |target| document.measured(index, target);
    let mut strokes: Vec<Row> = sketch
        .live_segments()
        .map(|(id, _)| Row {
            name: numbered(lang, "part_tree.segment", id.0 + 1),
            value: measured(DimensionTarget::Length(id)).map(|length| said::length(lang, length)),
            points: Points::Element {
                sketch: index,
                element: Element::Segment(id),
            },
        })
        .collect();
    strokes.extend(sketch.live_circles().map(|(id, _)| Row {
        name: numbered(lang, "part_tree.circle", id.0 + 1),
        value:
            measured(DimensionTarget::Diameter(id)).map(|diameter| said::diameter(lang, diameter)),
        points: Points::Element {
            sketch: index,
            element: Element::Circle(id),
        },
    }));
    strokes.extend(sketch.live_arcs().map(|(id, _)| Row {
        name: numbered(lang, "part_tree.arc", id.0 + 1),
        value: None,
        points: Points::Element {
            sketch: index,
            element: Element::Arc(id),
        },
    }));
    strokes.extend(sketch.live_ellipses().map(|(id, _)| Row {
        name: numbered(lang, "part_tree.ellipse", id.0 + 1),
        value: None,
        points: Points::Element {
            sketch: index,
            element: Element::Ellipse(id),
        },
    }));

    Drawn {
        sketch: index,
        badge: None,
        step,
        name: sketch_name(lang, rank),
        plane: String::new(),
        open: false,
        dimmed: false,
        areas: (0..sketch.regions().len())
            .map(|rank| Row {
                name: numbered(lang, "part_tree.area", rank + 1),
                value: None,
                points: Points::Area {
                    sketch: index,
                    rank,
                },
            })
            .collect(),
        strokes,
        dimensions: (0..sketch.dimensions().len())
            .map(|rank| Row {
                name: numbered(lang, "part_tree.dimension", rank + 1),
                value: None,
                points: Points::Dimension {
                    sketch: index,
                    rank,
                },
            })
            .collect(),
        rules: (0..sketch.constraints().len())
            .map(|rank| Row {
                name: numbered(lang, "part_tree.rule", rank + 1),
                value: None,
                points: Points::Rule {
                    sketch: index,
                    rank,
                },
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests;
