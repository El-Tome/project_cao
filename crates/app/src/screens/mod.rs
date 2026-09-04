pub mod part_placeholder;
pub mod start_menu;

use std::path::PathBuf;

use cao_core::PartDocument;

/// The current top-level screen. New modes (sketch/extrude, assembly, ...)
/// each get their own variant and their own module here, never a branch
/// bolted onto an existing one.
pub enum Screen {
    StartMenu,
    PartOpened { doc: PartDocument, path: PathBuf },
}
