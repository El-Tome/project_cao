//! What a click on a row of the tree asks, and how a row is marked: one
//! selection with the canvas in the sketch being drawn on, a light on the
//! canvas in the others.

use cao_part::PartDocument;
use cao_sketch::{Selection, ToolState};

use super::Points;
use crate::screens::sketch::{SketchEditor, Tool};
use crate::screens::viewport::ViewportState;

/// What the user asked of the tree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TreeAction {
    None,
    /// Reopen this sketch for drawing.
    EditSketch(usize),
    /// Show this sketch on the canvas, or stop showing it.
    ToggleEye(usize),
    /// Take this into the open sketch's selection, alone or added to it.
    Select {
        what: Selection,
        adding: bool,
    },
    /// Light this on the canvas, in a sketch that is not open.
    Light(Points),
    /// Draw the volume made at this step of the history the next way round:
    /// opaque, see-through, invisible.
    NextVolumeMode(usize),
    /// Draw every volume opaque again.
    MakeAllOpaque,
    /// Switch this kept section on, or off when it is on.
    ToggleSection(usize),
    /// Reopen the section view on this kept section, to change it.
    EditSection(usize),
    /// Delete this kept section, asking first when a drawing stands on it.
    DeleteSection(usize),
    /// Unfold the history, or fold it away.
    ToggleHistory,
    /// Put the part back to this many applied operations.
    RewindTo(usize),
    /// Ask whether to rewrite the history down to the steps that still
    /// describe the part.
    AskToCompact,
}

/// What a click on a row asks: in the open sketch, to select what it names,
/// as a click on the canvas would; in another, to light it, opening nothing.
/// An area has no selection, so it is lit wherever it is. While the tool in
/// hand is part-way through something, a click asks nothing.
pub fn clicked(
    document: &PartDocument,
    editor: &SketchEditor,
    row: Points,
    adding: bool,
) -> TreeAction {
    if !clickable(document, editor, row) || busy(editor) {
        return TreeAction::None;
    }
    match row {
        Points::Area { .. } => TreeAction::Light(row),
        Points::Element { .. } if !is_open(editor, row) => TreeAction::Light(row),
        _ => selection_of(document, row)
            .map_or(TreeAction::None, |what| TreeAction::Select { what, adding }),
    }
}

/// Lights what the row under the hand names: in the open sketch as the hand
/// over it on the canvas would, selecting nothing; in another as its click
/// would, leaving the click's own light alone. Leaving a row puts out only
/// what the tree lit, never a hover the canvas set since.
pub fn hover(
    document: &PartDocument,
    editor: &mut SketchEditor,
    viewport: &mut ViewportState,
    row: Option<Points>,
) {
    let row = row.filter(|row| clickable(document, editor, *row));
    let left = viewport
        .hovered_in_the_tree()
        .filter(|before| Some(*before) != row);
    let lit_by_the_tree = left.and_then(|before| in_the_open_sketch(document, editor, before));
    if lit_by_the_tree.is_some() && editor.hovered == lit_by_the_tree {
        editor.hovered = None;
    }
    if let Some(what) = row.and_then(|row| in_the_open_sketch(document, editor, row)) {
        editor.hovered = Some(what);
    }
    viewport.hover_in_the_tree(row);
}

/// What a row of the open sketch names, as the canvas lights it.
fn in_the_open_sketch(
    document: &PartDocument,
    editor: &SketchEditor,
    row: Points,
) -> Option<Selection> {
    is_open(editor, row)
        .then(|| selection_of(document, row))
        .flatten()
}

/// Whether a click on a row asks anything: every row of the open sketch, and
/// the elements and areas of the others.
pub fn clickable(document: &PartDocument, editor: &SketchEditor, row: Points) -> bool {
    let named = document.sketches().get(row.sketch()).is_some();
    named && (is_open(editor, row) || matches!(row, Points::Element { .. } | Points::Area { .. }))
}

/// Takes what a row names into the selection, the selection tool put in hand.
pub fn take(editor: &mut SketchEditor, what: Selection, adding: bool) {
    if editor.tool != Tool::Select {
        editor.tool = Tool::Select;
        editor.reset_pending();
    }
    crate::screens::viewport::take_a_selection(editor, Some(what), adding);
}

/// Whether the open sketch's selection holds what a row names.
pub fn holds(document: &PartDocument, editor: &SketchEditor, row: Points) -> bool {
    is_open(editor, row) && selection_of(document, row).is_some_and(|what| editor.is_selected(what))
}

/// Whether a row is marked: held by the open sketch's selection, or lit on
/// the canvas. A lit element is marked only while its sketch is not drawn on,
/// where the selection speaks instead; an area has no selection to speak.
pub fn marked(
    document: &PartDocument,
    editor: &SketchEditor,
    viewport: &ViewportState,
    row: Points,
) -> bool {
    let lit = viewport.lit() == Some(row)
        && (!is_open(editor, row) || matches!(row, Points::Area { .. }));
    lit || holds(document, editor, row)
}

fn is_open(editor: &SketchEditor, row: Points) -> bool {
    editor.active_sketch() == Some(row.sketch())
}

/// Part-way through something, the way Échap judges it: a rule waiting for
/// its next pick, a trait half drawn, a value being typed.
fn busy(editor: &SketchEditor) -> bool {
    let selecting = matches!(editor.tool_state, ToolState::Select(_));
    editor.editing.is_some() || (editor.tool_state.is_busy() && !selecting)
}

fn selection_of(document: &PartDocument, row: Points) -> Option<Selection> {
    let sketch = document.sketches().get(row.sketch())?;
    match row {
        Points::Element { element, .. } => Some(Selection::Element(element)),
        Points::Dimension { rank, .. } => {
            let dimension = sketch.dimensions().get(rank)?;
            Some(Selection::Dimension(dimension.target))
        }
        Points::Rule { rank, .. } => sketch.constraints().get(rank).copied().map(Selection::Rule),
        Points::Area { .. } => None,
    }
}
