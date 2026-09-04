use std::path::PathBuf;

use cao_core::{PartDocument, RecentList};

use crate::screens::{self, start_menu::StartMenuAction, Screen};

pub struct CaoApp {
    screen: Screen,
    recents: RecentList,
    new_part_name: String,
    error: Option<String>,
}

impl CaoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut recents = RecentList::load().unwrap_or_default();
        recents.prune_missing();
        Self {
            screen: Screen::StartMenu,
            recents,
            new_part_name: String::new(),
            error: None,
        }
    }

    fn create_new_part(&mut self, name: String) {
        let dir = match cao_core::default_projects_dir() {
            Ok(dir) => dir,
            Err(err) => {
                self.error = Some(err.to_string());
                return;
            }
        };
        match PartDocument::create_in(&dir, name) {
            Ok((doc, path)) => self.open_document(doc, path),
            Err(err) => self.error = Some(err.to_string()),
        }
    }

    fn open_part(&mut self, path: PathBuf) {
        match PartDocument::load(&path) {
            Ok(doc) => self.open_document(doc, path),
            Err(err) => self.error = Some(err.to_string()),
        }
    }

    fn open_document(&mut self, doc: PartDocument, path: PathBuf) {
        self.recents.push(path.clone(), doc.name.clone());
        if let Err(err) = self.recents.save() {
            self.error = Some(err.to_string());
        }
        self.error = None;
        self.screen = Screen::PartOpened { doc, path };
    }
}

impl eframe::App for CaoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(error) = self.error.clone() {
            egui::TopBottomPanel::bottom("error_bar").show(ctx, |ui| {
                ui.colored_label(egui::Color32::RED, error);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| match &self.screen {
            Screen::StartMenu => {
                let action = screens::start_menu::show(ui, self.recents.entries(), &mut self.new_part_name);
                match action {
                    StartMenuAction::CreateNew(name) => self.create_new_part(name),
                    StartMenuAction::Open(path) => self.open_part(path),
                    StartMenuAction::None => {}
                }
            }
            Screen::PartOpened { doc, path } => {
                if screens::part_placeholder::show(ui, doc, path) {
                    self.screen = Screen::StartMenu;
                }
            }
        });
    }
}
