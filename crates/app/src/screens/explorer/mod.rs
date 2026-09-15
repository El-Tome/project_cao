mod state;
mod view;

use std::path::{Path, PathBuf};

use cao_part::{Files, Folders};

pub use state::Explorer;
pub use view::ExplorerAction;

use crate::lang::Catalogue;

/// Puts the library panel on screen and carries out what was asked of it,
/// handing back a part to open when one was double-clicked.
///
/// The panel's wiring lives here rather than in the shell: which presenter
/// call an action leads to is the screen's own business, and the one branch
/// nobody could reach from a test is the one a bug hides in.
pub fn run(
    explorer: &mut Explorer,
    ui: &mut egui::Ui,
    lang: &Catalogue,
    files: &impl Files,
    folders: &impl Folders,
    drawing: Option<&Path>,
) -> Option<PathBuf> {
    explorer.drawing(drawing);
    explorer.refresh_if_stale(folders);

    let mut opening = None;
    match view::panel(ui, explorer, lang) {
        ExplorerAction::Open(part) => opening = Some(part),
        ExplorerAction::NewFolder => explorer.start_new_folder(),
        ExplorerAction::Rename => explorer.start_rename(),
        ExplorerAction::ConfirmNaming => explorer.confirm_naming(files, folders),
        ExplorerAction::Discard => explorer.ask_to_discard(),
        ExplorerAction::Refresh => explorer.went_stale(),
        ExplorerAction::None => {}
    }

    if let Some(path) = explorer.confirming().map(Path::to_path_buf) {
        match view::discard_confirm(ui, &path, lang) {
            Some(true) => explorer.discard(folders),
            Some(false) => explorer.cancel_discard(),
            None => {}
        }
    }
    opening
}
