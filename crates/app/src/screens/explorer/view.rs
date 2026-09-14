use std::path::{Path, PathBuf};

use cao_part::library::{self, Folder};

use crate::lang::Catalogue;
use crate::ui::text_edit::text_edit;
use crate::wording;

use super::state::{Explorer, Naming};

/// What the panel asks of whoever put it on screen. The panel itself holds no
/// port: the shell decides what a file system is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExplorerAction {
    None,
    /// Open this part, in this window or another.
    Open(PathBuf),
    /// Ask for the name of a folder that does not exist yet.
    NewFolder,
    /// Ask for another name for what is selected.
    Rename,
    /// Write down the name being typed.
    ConfirmNaming,
    /// Throw away what the question was about.
    Discard,
    /// Read the library off the disk again.
    Refresh,
}

/// Where each name typed in the panel is given room to be read.
const NAME_WIDTH: f32 = 150.0;

/// What the user did to the tree this frame, gathered while the panel is only
/// being read so that acting on it comes afterwards.
#[derive(Default)]
struct Touched {
    select: Option<PathBuf>,
    open: Option<PathBuf>,
    fold: Option<(PathBuf, bool)>,
}

/// The panel the library is browsed in, chrome and all.
pub fn panel(ui: &mut egui::Ui, explorer: &mut Explorer, lang: &Catalogue) -> ExplorerAction {
    egui::Panel::left("explorer_panel")
        .resizable(true)
        .default_size(230.0)
        .show(ui, |ui| show(ui, explorer, lang))
        .inner
}

fn show(ui: &mut egui::Ui, explorer: &mut Explorer, lang: &Catalogue) -> ExplorerAction {
    let mut action = ExplorerAction::None;
    let mut touched = Touched::default();

    ui.horizontal(|ui| {
        ui.heading(lang.t("explorer.title"));
        if ui
            .small_button(lang.t("explorer.refresh"))
            .on_hover_text(lang.t("explorer.refresh_hint"))
            .clicked()
        {
            action = ExplorerAction::Refresh;
        }
    });
    ui.horizontal(|ui| {
        if ui.small_button(lang.t("explorer.new_folder")).clicked() {
            action = ExplorerAction::NewFolder;
        }
        ui.add_enabled_ui(explorer.may_tidy(), |ui| {
            if ui.small_button(lang.t("explorer.rename")).clicked() {
                action = ExplorerAction::Rename;
            }
            if ui.small_button(lang.t("explorer.discard")).clicked() {
                action = ExplorerAction::Discard;
            }
        });
    });
    ui.separator();

    // The name being typed is taken out for the length of the walk and put
    // back at the end: that is what lets the tree be read while the field it
    // holds is written to.
    let mut naming = explorer.naming().cloned();
    egui::ScrollArea::vertical().show(ui, |ui| {
        let reading: &Explorer = explorer;
        contents(
            ui,
            reading,
            reading.library(),
            &mut naming,
            &mut touched,
            lang,
        );
        if reading.library().folders.is_empty() && reading.library().parts.is_empty() {
            ui.weak(lang.t("explorer.empty"));
        }
    });

    if let Some(fate) = explorer.trouble() {
        ui.separator();
        ui.colored_label(
            egui::Color32::from_rgb(230, 140, 140),
            wording::part_file::say(lang, fate),
        );
    }

    explorer.set_naming(naming);
    if let Some((path, open)) = touched.fold {
        explorer.fold(&path, open);
    }
    if let Some(path) = touched.select {
        explorer.select(&path);
    }
    if let Some(path) = touched.open {
        action = ExplorerAction::Open(path);
    }
    if explorer.naming().is_some() {
        if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
            action = ExplorerAction::ConfirmNaming;
        } else if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
            explorer.cancel_naming();
        }
    }
    action
}

/// One folder's worth of rows, its subfolders before its parts.
fn contents(
    ui: &mut egui::Ui,
    explorer: &Explorer,
    folder: &Folder,
    naming: &mut Option<Naming>,
    touched: &mut Touched,
    lang: &Catalogue,
) {
    for under in &folder.folders {
        if being_renamed(naming, &under.path) {
            field(ui, naming, lang);
            continue;
        }
        let unfolded = explorer.unfolded(&under.path);
        ui.horizontal(|ui| {
            let arrow = lang.t(if unfolded {
                "explorer.unfolded"
            } else {
                "explorer.folded"
            });
            if ui.small_button(arrow).clicked() {
                touched.fold = Some((under.path.clone(), !unfolded));
            }
            if ui
                .selectable_label(explorer.selected() == Some(&under.path), &under.name)
                .clicked()
            {
                touched.select = Some(under.path.clone());
            }
        });
        if unfolded {
            ui.indent(&under.path, |ui| {
                contents(ui, explorer, under, naming, touched, lang);
            });
        }
    }

    for part in &folder.parts {
        if being_renamed(naming, &part.path) {
            field(ui, naming, lang);
            continue;
        }
        let response = ui
            .selectable_label(explorer.selected() == Some(&part.path), &part.name)
            .on_hover_text(lang.t("explorer.open_hint"));
        if response.clicked() {
            touched.select = Some(part.path.clone());
        }
        if response.double_clicked() {
            touched.open = explorer.asked_to_open(&part.path);
        }
    }

    if let Some(Naming::NewFolder { inside, .. }) = naming
        && inside == &folder.path
    {
        field(ui, naming, lang);
    }
}

fn being_renamed(naming: &Option<Naming>, path: &Path) -> bool {
    matches!(naming, Some(Naming::Rename { path: at, .. }) if at == path)
}

fn field(ui: &mut egui::Ui, naming: &mut Option<Naming>, lang: &Catalogue) {
    let Some(naming) = naming else { return };
    let response = text_edit(ui, naming.typed_mut(), NAME_WIDTH, &lang.t("explorer.name"));
    // Only when nothing else has the keyboard: asking every frame takes focus
    // back the frame after the user clicks anywhere else, and the field can
    // then only be left by Entrée or Échap.
    if ui.memory(|memory| memory.focused().is_none()) {
        response.request_focus();
    }
}

/// The question asked before a part or a folder is thrown away.
///
/// `Some(true)` once confirmed, `Some(false)` once backed out of, `None` while
/// the user has not answered.
pub fn discard_confirm(ui: &mut egui::Ui, path: &Path, lang: &Catalogue) -> Option<bool> {
    let mut decision = None;
    let modal = egui::Modal::new(egui::Id::new("explorer_discard_confirm")).show(ui.ctx(), |ui| {
        ui.heading(lang.t("explorer.discard_confirm_title"));
        ui.label(lang.t_with(
            "explorer.discard_confirm_body",
            &[("name", &library::name_of(path))],
        ));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button(lang.t("explorer.discard_confirm_ok")).clicked() {
                decision = Some(true);
            }
            if ui
                .button(lang.t("explorer.discard_confirm_cancel"))
                .clicked()
            {
                decision = Some(false);
            }
        });
    });
    if decision.is_none() && modal.should_close() {
        decision = Some(false);
    }
    decision
}
