pub mod annotations;
pub mod history_tree;
pub mod ribbon;
pub mod sketch;
pub mod start_menu;
pub mod viewport;

use std::path::PathBuf;

use cao_core::PartDocument;

use ribbon::Ribbon;
use sketch::SketchEditor;
use viewport::ViewportState;

/// A part being worked on, with the state of the view onto it and of the
/// sketch being drawn.
pub struct OpenPart {
    pub doc: PartDocument,
    pub path: PathBuf,
    pub viewport: ViewportState,
    pub editor: SketchEditor,
    pub ribbon: Ribbon,
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
