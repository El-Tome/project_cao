use std::cmp::Ordering;

use cao_part::Formula;
use cao_sketch::{ChosenAxis, Element, PointId, Selection, Sketch, ToolState};
use glam::DVec2;

use super::{LiveInput, SketchEditor, Tool};

/// A slot of the panel the mirror and the patterns gather what they need in,
/// top to bottom.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CopySlot {
    #[default]
    Elements,
    /// The mirror's axis, a pattern's centre or direction.
    Target,
    /// The field of that rank among the tool's values.
    Value(usize),
    /// The circular pattern's direction, its last step: ↵ there lays the copy
    /// as on Validate.
    Rotation,
    Validate,
}

/// A numbered step of the panel, top to bottom: the slots that belong to it
/// are what it asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyStep {
    Elements,
    Centre,
    CountAndStep,
    Rotation,
}

/// How a step's dot reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepState {
    Done,
    Current,
    ToCome,
}

/// Which way round the circular pattern turns. Counter-clockwise is the way a
/// positive step turns, and the way a pattern turned before it had a
/// direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Rotation {
    #[default]
    CounterClockwise,
    Clockwise,
}

impl Rotation {
    /// The step the pattern turns by, from the one typed: as it is
    /// counter-clockwise, the other way round clockwise.
    pub(crate) fn step(self, typed: f64) -> f64 {
        match self {
            Self::CounterClockwise => typed,
            Self::Clockwise => -typed,
        }
    }

    /// The same step as it is written down, a formula staying a formula.
    pub(crate) fn step_written(self, typed: Formula) -> Formula {
        match self {
            Self::CounterClockwise => typed,
            Self::Clockwise => typed.negated(),
        }
    }
}

/// What one of a tool's values is: the view names it and gives it its unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyValue {
    StepInMillimetres,
    StepInDegrees,
    Count,
}

/// What fills the target slot: an axis for the mirror and the rectangular
/// pattern, a point of the drawing for the circular one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CopyTarget {
    Axis(ChosenAxis),
    Centre(PointId),
}

#[derive(Debug, Default)]
pub struct CopyPanel {
    active: CopySlot,
    marked: Option<CopySlot>,
    elements: Vec<Element>,
    brought_in: Vec<PointId>,
    target: Option<CopyTarget>,
    values: &'static [CopyValue],
    steps: &'static [CopyStep],
    rotation: Rotation,
    /// The box being pulled across the drawing while Elements is active.
    pub(crate) band: Option<(DVec2, DVec2)>,
}

impl CopyPanel {
    fn opened(tool: Tool, elements: Vec<Element>) -> Self {
        let steps: &[CopyStep] = match tool {
            Tool::CircularPattern => &[
                CopyStep::Elements,
                CopyStep::Centre,
                CopyStep::CountAndStep,
                CopyStep::Rotation,
            ],
            _ => &[],
        };
        let values: &[CopyValue] = match tool {
            Tool::CircularPattern => &[CopyValue::Count, CopyValue::StepInDegrees],
            Tool::RectangularPattern => &[
                CopyValue::StepInMillimetres,
                CopyValue::Count,
                CopyValue::StepInMillimetres,
                CopyValue::Count,
            ],
            _ => &[],
        };
        let active = match elements.is_empty() {
            true => CopySlot::Elements,
            false => CopySlot::Target,
        };
        Self {
            active,
            elements,
            values,
            steps,
            ..Self::default()
        }
    }

    /// Each numbered step, top to bottom, with how its dot reads: current
    /// where the active slot is, done above it once it holds what it asks
    /// for, to come otherwise. A tool laid out in no steps has none.
    pub fn steps(&self, live: &LiveInput) -> Vec<(CopyStep, StepState)> {
        let current = self
            .current_step()
            .map_or(self.steps.len(), |(rank, _)| rank);
        let holds = |step: CopyStep| {
            self.slots()
                .filter(|slot| step_of(*slot) == Some(step))
                .all(|slot| self.is_filled(slot, live))
        };
        self.steps
            .iter()
            .enumerate()
            .map(|(rank, step)| {
                let state = match rank.cmp(&current) {
                    Ordering::Equal => StepState::Current,
                    Ordering::Less if holds(*step) => StepState::Done,
                    _ => StepState::ToCome,
                };
                (*step, state)
            })
            .collect()
    }

    /// The step holding the active slot, with its rank from the top — none on
    /// Validate, nor for a tool laid out in no steps.
    pub(crate) fn current_step(&self) -> Option<(usize, CopyStep)> {
        self.steps
            .iter()
            .copied()
            .enumerate()
            .find(|(_, step)| step_of(self.active) == Some(*step))
    }

    /// The slots that hold something to fill, top to bottom.
    fn slots(&self) -> impl Iterator<Item = CopySlot> {
        [CopySlot::Elements, CopySlot::Target]
            .into_iter()
            .chain((0..self.values.len()).map(CopySlot::Value))
    }

    pub fn active(&self) -> CopySlot {
        self.active
    }

    pub fn activate(&mut self, slot: CopySlot) {
        self.active = slot;
    }

    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    /// What was picked, leaving out the ends a trait brought in with it.
    pub fn picked(&self) -> impl Iterator<Item = Element> {
        self.elements
            .iter()
            .copied()
            .filter(|held| !matches!(held, Element::Point(end) if self.brought_in.contains(end)))
    }

    /// The slot `Entrée` on Validate found empty, until it is filled.
    pub fn marked(&self) -> Option<CopySlot> {
        self.marked
    }

    pub(crate) fn target(&self) -> Option<CopyTarget> {
        self.target
    }

    pub fn value_kinds(&self) -> &'static [CopyValue] {
        self.values
    }

    pub fn rotation(&self) -> Rotation {
        self.rotation
    }

    pub fn choose_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    /// Takes or lets go of one element, a trait with its two ends. An end a
    /// trait brought in is let go with the last held trait standing on it; a
    /// point picked on its own is never let go by a trait.
    pub(crate) fn toggle(&mut self, element: Element, sketch: &Sketch) {
        let Some(rank) = self.elements.iter().position(|held| *held == element) else {
            self.elements.push(element);
            for end in ends(sketch, element) {
                if !self.elements.contains(&Element::Point(end)) {
                    self.elements.push(Element::Point(end));
                    self.brought_in.push(end);
                }
            }
            return;
        };
        self.elements.remove(rank);
        if let Element::Point(point) = element {
            self.brought_in.retain(|end| *end != point);
        }
        let still: Vec<PointId> = self
            .elements
            .iter()
            .flat_map(|held| ends(sketch, *held))
            .collect();
        let freed: Vec<PointId> = ends(sketch, element)
            .into_iter()
            .filter(|end| self.brought_in.contains(end) && !still.contains(end))
            .collect();
        self.elements
            .retain(|held| !matches!(held, Element::Point(point) if freed.contains(point)));
        self.brought_in.retain(|end| !freed.contains(end));
    }

    /// Adds what a box took that is not held already.
    pub(crate) fn take_in(&mut self, caught: impl IntoIterator<Item = Element>) {
        for element in caught {
            self.brought_in
                .retain(|end| element != Element::Point(*end));
            if !self.elements.contains(&element) {
                self.elements.push(element);
            }
        }
    }

    pub(crate) fn name_the_target(&mut self, target: CopyTarget) {
        self.target = Some(target);
        self.move_down();
    }

    /// The slot below the active one becomes active, whether or not it is
    /// filled already.
    pub(crate) fn move_down(&mut self) {
        self.active = match self.active {
            CopySlot::Elements => CopySlot::Target,
            CopySlot::Target if !self.values.is_empty() => CopySlot::Value(0),
            CopySlot::Value(rank) if rank + 1 < self.values.len() => CopySlot::Value(rank + 1),
            CopySlot::Value(_) if self.steps.contains(&CopyStep::Rotation) => CopySlot::Rotation,
            _ => CopySlot::Validate,
        };
    }

    pub(crate) fn is_filled(&self, slot: CopySlot, live: &LiveInput) -> bool {
        match slot {
            CopySlot::Elements => !self.elements.is_empty(),
            CopySlot::Target => self.target.is_some(),
            CopySlot::Value(rank) => live.typed(rank).is_some(),
            CopySlot::Rotation | CopySlot::Validate => true,
        }
    }

    /// Activates and marks the first slot still empty, top to bottom. False
    /// when every slot is filled.
    pub(crate) fn mark_the_first_empty(&mut self, live: &LiveInput) -> bool {
        let Some(empty) = self.slots().find(|slot| !self.is_filled(*slot, live)) else {
            return false;
        };
        self.active = empty;
        self.marked = Some(empty);
        true
    }

    pub(crate) fn unmark_if_filled(&mut self, live: &LiveInput) {
        if self.marked.is_some_and(|slot| self.is_filled(slot, live)) {
            self.marked = None;
        }
    }
}

impl SketchEditor {
    /// The panel of the copy tool in hand, open from the moment it is taken.
    pub fn copy_panel(&self) -> Option<&CopyPanel> {
        self.copy_panel.as_ref()
    }

    pub fn copy_panel_mut(&mut self) -> Option<&mut CopyPanel> {
        self.copy_panel.as_mut()
    }

    pub(crate) fn open_copy_panel(&mut self, elements: Vec<Element>) {
        self.tool_state = ToolState::Copying;
        self.copy_panel = Some(CopyPanel::opened(self.tool, elements));
    }

    /// Closes the panel and lays nothing, handing what it held to the
    /// Selection tool.
    pub fn cancel_the_copy(&mut self) {
        self.tool = Tool::Select;
        self.reset_pending();
        self.message = None;
    }

    /// The selection a tool reached for from the panel starts with: what the
    /// panel held, when that tool works on a selection.
    pub(super) fn what_the_panel_hands_on(&mut self) -> Vec<Selection> {
        let Some(panel) = self.copy_panel.take() else {
            return Vec::new();
        };
        panel.picked().map(Selection::Element).collect()
    }
}

/// The step a slot belongs to; Validate stands below them all.
fn step_of(slot: CopySlot) -> Option<CopyStep> {
    match slot {
        CopySlot::Elements => Some(CopyStep::Elements),
        CopySlot::Target => Some(CopyStep::Centre),
        CopySlot::Value(_) => Some(CopyStep::CountAndStep),
        CopySlot::Rotation => Some(CopyStep::Rotation),
        CopySlot::Validate => None,
    }
}

fn ends(sketch: &Sketch, element: Element) -> Vec<PointId> {
    match element {
        Element::Segment(side) => sketch
            .segments()
            .get(side.0)
            .map_or_else(Vec::new, |side| vec![side.start, side.end]),
        _ => Vec::new(),
    }
}
