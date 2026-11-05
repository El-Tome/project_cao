//! A sketch open in a copy tool, driven the way the input layer drives it: a
//! tool taken from the ribbon, clicks in the drawing, a slot clicked in the
//! panel, `Entrée`.

use cao_part::{Operation, PartDocument, PointRef, Variables};
use cao_prefs::Command;
use cao_sketch::{Element, PointId, Selection, WorkPlane};
use chrono::Utc;
use glam::DVec2;

use super::super::{copy, enter_in_the_panel, previewed};
use crate::lang::Catalogue;
use crate::screens::SketchContext;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::{CopyPanel, CopySlot, SketchEditor};
use crate::screens::viewport::ViewportState;

pub(crate) const SNAP: f64 = 0.5;
pub(crate) const PIXEL: f64 = 0.05;

pub(crate) struct Bench {
    pub(crate) document: PartDocument,
    pub(crate) editor: SketchEditor,
    extrusion: ExtrusionState,
    lang: Catalogue,
}

impl Bench {
    /// A sketch on XY holding what these operations lay, open for editing.
    pub(crate) fn drawn(operations: Vec<Operation>) -> Self {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        for operation in operations {
            document.apply(operation);
        }
        let mut editor = SketchEditor::default();
        editor.begin_editing(0, WorkPlane::XY);
        Self {
            document,
            editor,
            extrusion: ExtrusionState::default(),
            lang: Catalogue::french(),
        }
    }

    pub(crate) fn a_trait(start: DVec2, end: DVec2, construction: bool) -> Operation {
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction,
        }
    }

    pub(crate) fn context(&mut self) -> SketchContext<'_> {
        SketchContext {
            document: &mut self.document,
            editor: &mut self.editor,
            extrusion: &mut self.extrusion,
            lang: &self.lang,
        }
    }

    pub(crate) fn take(&mut self, command: Command) {
        crate::commands::run(
            command,
            &mut self.context(),
            &mut Ribbon::default(),
            &mut ViewportState::default(),
        );
    }

    /// The Selection tool taken, holding these, as a click or a box would
    /// have left it.
    pub(crate) fn select(&mut self, elements: &[Element]) {
        self.take(Command::ToolSelect);
        self.editor
            .select_state()
            .expect("the Selection tool is in hand")
            .held = elements.iter().copied().map(Selection::Element).collect();
    }

    pub(crate) fn click(&mut self, at: DVec2) {
        self.editor.cursor = Some(at);
        copy(&mut self.context(), 0, at, SNAP, PIXEL);
    }

    pub(crate) fn click_the_slot(&mut self, slot: CopySlot) {
        self.panel_mut().activate(slot);
    }

    pub(crate) fn enter(&mut self) {
        enter_in_the_panel(&mut self.context());
    }

    /// What a field of the panel's values holds once this is typed into it,
    /// an empty text emptying it.
    pub(crate) fn type_value(&mut self, rank: usize, text: &str) {
        self.editor.live.field(rank).text = text.to_string();
        self.editor.live.take(rank, &Variables::default(), None);
    }

    pub(crate) fn value(&self, rank: usize) -> Option<f64> {
        self.editor.live.typed(rank)
    }

    pub(crate) fn panel(&self) -> &CopyPanel {
        self.editor
            .copy_panel()
            .expect("the panel is open while a copy tool is in hand")
    }

    fn panel_mut(&mut self) -> &mut CopyPanel {
        self.editor
            .copy_panel_mut()
            .expect("the panel is open while a copy tool is in hand")
    }

    pub(crate) fn active(&self) -> CopySlot {
        self.panel().active()
    }

    pub(crate) fn elements(&self) -> Vec<Element> {
        self.panel().elements().to_vec()
    }

    pub(crate) fn laid(&self) -> usize {
        self.document.history.applied_operations().len()
    }

    pub(crate) fn last_laid(&self) -> Option<&Operation> {
        self.document.history.applied_operations().last()
    }

    pub(crate) fn point_at(&self, at: DVec2) -> PointId {
        self.document.sketches()[0]
            .nearest_point(at, 1e-6)
            .expect("a point of the drawing stands there")
    }

    /// What the drawing would gain if the copy were laid now: how many
    /// traits. Read with no cursor at all — the panel alone decides it.
    pub(crate) fn previewed_traits(&self, scale: f64) -> Option<usize> {
        let sketch = &self.document.sketches()[0];
        previewed(sketch, &self.editor, scale).map(|shown| {
            shown
                .laid
                .iter()
                .filter(|laid| matches!(laid, Element::Segment(_)))
                .count()
        })
    }
}
