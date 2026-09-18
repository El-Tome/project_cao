//! What the tree of a part holds, worked out from the part itself.
//!
//! The panel beside it, `history_tree`, lists what was *done*: one line per
//! operation, newest last. This lists what the part **is** — its bodies and
//! its sketches, and under each sketch what is drawn on it, by kind. A drag,
//! a dimension moved, two corners joined leave no line here, because they
//! leave no element behind.
//!
//! Which means the two disagree on purpose. Reopen a sketch after an extrusion
//! and draw a trait: the tree files it under that sketch, where it belongs,
//! while the design keeps it in the folder of the step that was open when it
//! was drawn, so that replaying it keeps the order it happened in.

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, Operation};
use cao_sketch::{Element, Sketch};

use crate::lang::Catalogue;

/// What a line of the tree points at, so that clicking it can find the thing
/// itself.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Points {
    Element { sketch: usize, element: Element },
    Dimension { sketch: usize, rank: usize },
    Rule { sketch: usize, rank: usize },
    Area { sketch: usize, rank: usize },
}

/// One line under a heading.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub name: String,
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
}

/// One sketch, and what is drawn on it, by kind.
#[derive(Clone, Debug, PartialEq)]
pub struct Drawn {
    pub sketch: usize,
    /// Whether the face this was laid on is gone from the part.
    pub adrift: bool,
    pub step: usize,
    pub name: String,
    pub areas: Vec<Row>,
    pub strokes: Vec<Row>,
    pub dimensions: Vec<Row>,
    pub rules: Vec<Row>,
}

/// The part, as the tree shows it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartTree {
    pub bodies: Vec<Body>,
    pub sketches: Vec<Drawn>,
}

impl PartTree {
    pub fn of(document: &PartDocument, lang: &Catalogue) -> Self {
        let mut tree = Self::default();
        let mut sketches = 0;

        for (step, operation) in document.history.applied_operations().iter().enumerate() {
            match operation {
                Operation::CreateSketch { .. } => {
                    let sketch = sketches;
                    sketches += 1;
                    let Some(drawing) = document.sketches().get(sketch) else {
                        continue;
                    };
                    let mut shown = drawn(lang, drawing, sketch, step, tree.sketches.len() + 1);
                    shown.adrift = document.is_adrift(sketch);
                    tree.sketches.push(shown);
                }
                Operation::Extrude {
                    sketch,
                    picks,
                    distance,
                    mode,
                } => tree.bodies.push(Body {
                    step,
                    name: numbered(lang, "part_tree.extrusion", tree.bodies.len() + 1),
                    reads: lang.t_with(
                        "part_tree.raised",
                        &[
                            ("distance", &distance.to_string()),
                            ("mode", &lang.t(mode_key(*mode))),
                        ],
                    ),
                    areas: standing_on(lang, document, *sketch, picks),
                }),
                Operation::Revolve {
                    sketch,
                    picks,
                    angle,
                    mode,
                    ..
                } => tree.bodies.push(Body {
                    step,
                    name: numbered(lang, "part_tree.revolution", tree.bodies.len() + 1),
                    reads: lang.t_with(
                        "part_tree.swept",
                        &[
                            ("angle", &angle.to_string()),
                            ("mode", &lang.t(mode_key(*mode))),
                        ],
                    ),
                    areas: standing_on(lang, document, *sketch, picks),
                }),
                _ => {}
            }
        }
        tree
    }

    pub fn is_empty(&self) -> bool {
        self.bodies.is_empty() && self.sketches.is_empty()
    }
}

fn mode_key(mode: ExtrusionMode) -> &'static str {
    match mode {
        ExtrusionMode::Add => "part_tree.mode.add",
        ExtrusionMode::Cut => "part_tree.mode.cut",
    }
}

fn numbered(lang: &Catalogue, key: &str, rank: usize) -> String {
    lang.t_with(key, &[("rank", &rank.to_string())])
}

/// The areas a step of matter was raised from, named as the sketch names them.
///
/// An area is found the way the replay finds it, by the place that was
/// clicked. One the drawing no longer encloses leaves no line: there is
/// nothing to point at, and saying so is the warning's business, not the
/// tree's.
fn standing_on(
    lang: &Catalogue,
    document: &PartDocument,
    sketch: usize,
    picks: &[glam::DVec2],
) -> Vec<Row> {
    let Some(drawing) = document.sketches().get(sketch) else {
        return Vec::new();
    };
    let regions = drawing.regions();
    picks
        .iter()
        .filter_map(|pick| {
            let rank = regions
                .iter()
                .enumerate()
                .filter(|(_, region)| region.contains(*pick))
                .max_by_key(|(_, region)| region.depth)
                .map(|(rank, _)| rank)?;
            Some(Row {
                name: numbered(lang, "part_tree.area", rank + 1),
                points: Points::Area { sketch, rank },
            })
        })
        .collect()
}

fn drawn(lang: &Catalogue, sketch: &Sketch, index: usize, step: usize, rank: usize) -> Drawn {
    let mut strokes: Vec<Row> = sketch
        .live_segments()
        .map(|(id, _)| Row {
            name: numbered(lang, "part_tree.segment", id.0 + 1),
            points: Points::Element {
                sketch: index,
                element: Element::Segment(id),
            },
        })
        .collect();
    strokes.extend(sketch.live_circles().map(|(id, _)| Row {
        name: numbered(lang, "part_tree.circle", id.0 + 1),
        points: Points::Element {
            sketch: index,
            element: Element::Circle(id),
        },
    }));
    strokes.extend(sketch.live_arcs().map(|(id, _)| Row {
        name: numbered(lang, "part_tree.arc", id.0 + 1),
        points: Points::Element {
            sketch: index,
            element: Element::Arc(id),
        },
    }));

    Drawn {
        sketch: index,
        adrift: false,
        step,
        name: numbered(lang, "part_tree.sketch", rank),
        areas: (0..sketch.regions().len())
            .map(|rank| Row {
                name: numbered(lang, "part_tree.area", rank + 1),
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
                points: Points::Dimension {
                    sketch: index,
                    rank,
                },
            })
            .collect(),
        rules: (0..sketch.constraints().len())
            .map(|rank| Row {
                name: numbered(lang, "part_tree.rule", rank + 1),
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
