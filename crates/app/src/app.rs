use std::path::PathBuf;

use cao_core::{PartDocument, RecentList};
use cao_render::SceneRenderer;

use crate::MSAA_SAMPLES;
use crate::screens::viewport::{ViewMode, ViewportState};
use crate::screens::{self, OpenPart, Screen, start_menu::StartMenuAction};

pub struct CaoApp {
    screen: Screen,
    recents: RecentList,
    new_part_name: String,
    error: Option<String>,
}

impl CaoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(render_state) = &cc.wgpu_render_state {
            let renderer = SceneRenderer::new(
                &render_state.device,
                render_state.target_format,
                MSAA_SAMPLES as u32,
            );
            render_state
                .renderer
                .write()
                .callback_resources
                .insert(renderer);
        }

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
        self.screen = Screen::PartOpened(Box::new(OpenPart {
            doc,
            path,
            viewport: ViewportState::default(),
        }));
    }

    fn show_start_menu(&mut self, ui: &mut egui::Ui) {
        let action = egui::CentralPanel::default_margins()
            .show(ui, |ui| {
                screens::start_menu::show(ui, self.recents.entries(), &mut self.new_part_name)
            })
            .inner;

        match action {
            StartMenuAction::CreateNew(name) => self.create_new_part(name),
            StartMenuAction::Open(path) => self.open_part(path),
            StartMenuAction::None => {}
        }
    }

    fn show_part(&mut self, ui: &mut egui::Ui) {
        let Screen::PartOpened(part) = &mut self.screen else {
            return;
        };
        let OpenPart {
            doc,
            path,
            viewport,
        } = part.as_mut();

        let mut back_to_menu = false;
        egui::Panel::top("part_toolbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                back_to_menu = ui.button("← Menu").clicked();
                ui.separator();
                ui.label(&doc.name)
                    .on_hover_text(path.display().to_string());
                ui.separator();
                ui.weak(mode_label(viewport.mode()));
            });
        });

        egui::CentralPanel::no_frame().show(ui, |ui| {
            screens::viewport::show(ui, viewport);
        });

        if back_to_menu {
            self.screen = Screen::StartMenu;
        }
    }
}

fn mode_label(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Free => "Vue 3D libre",
        ViewMode::Plane(plane) => match plane {
            cao_render::GridPlane::Xy => "Plan XY",
            cao_render::GridPlane::Xz => "Plan XZ",
            cao_render::GridPlane::Yz => "Plan YZ",
        },
    }
}

impl eframe::App for CaoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(error) = &self.error {
            let message = error.clone();
            egui::Panel::bottom("error_bar").show(ui, |ui| {
                ui.colored_label(egui::Color32::RED, message);
            });
        }

        if matches!(self.screen, Screen::StartMenu) {
            self.show_start_menu(ui);
        } else {
            self.show_part(ui);
        }
    }
}
