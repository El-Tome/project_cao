pub mod annotations;
mod blinking;
pub mod explorer;
pub mod extrusion;
mod extrusion_row;
pub mod origin_view;
pub mod part_tree;
pub mod ribbon;
pub mod settings;
pub mod sketch;
pub mod sketch_pill;
pub mod start_menu;
pub mod title_bar;
pub mod variables;
pub mod view_bar;
pub mod viewport;

use std::path::PathBuf;

use cao_part::PartDocument;

use crate::lang::Catalogue;
use extrusion::ExtrusionState;
use ribbon::Ribbon;
use sketch::SketchEditor;
use viewport::ViewportState;

/// A part being worked on, with the state of the view onto it and of the
/// sketch being drawn.
pub struct OpenPart {
    pub doc: PartDocument,
    pub path: PathBuf,
    pub autosave: crate::autosave::Autosave,
    pub viewport: ViewportState,
    pub editor: SketchEditor,
    pub extrusion: ExtrusionState,
    pub ribbon: Ribbon,
    pub variables: variables::VariablesPanel,
    /// The error the title bar's pill holds over the message, if one does, and
    /// whether the list of alerts is open.
    pub title_bar: title_bar::Held,
}

/// The part being worked on, borrowed for one gesture or one command: what the
/// canvas and the commands may read and change about it, without the view onto
/// it or the ribbon around it.
pub struct SketchContext<'a> {
    pub document: &'a mut PartDocument,
    pub editor: &'a mut SketchEditor,
    pub extrusion: &'a mut ExtrusionState,
    pub lang: &'a Catalogue,
}

/// The current top-level screen. New modes (sketch/extrude, assembly, ...)
/// each get their own variant and their own module here, never a branch
/// bolted onto an existing one.
///
/// The open part is boxed so that the menu variant stays cheap: an editing
/// mode carries far more state than a menu ever will.
pub enum Screen {
    StartMenu,
    PartOpened(Box<OpenPart>),
}
