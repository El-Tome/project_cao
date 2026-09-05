use std::path::PathBuf;

use cao_core::{DimensionOutcome, PartDocument, RecentList};
use cao_render::SceneRenderer;
use cao_sketch::{LengthOutcome, WorkPlane};
use glam::Vec3;

use crate::MSAA_SAMPLES;
use crate::screens::sketch::{SketchEditor, Tool};
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
            editor: SketchEditor::default(),
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
            editor,
        } = part.as_mut();

        let mut back_to_menu = false;
        let mut changed = false;

        egui::Panel::top("part_menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                back_to_menu = ui.button("← Menu").clicked();
                ui.separator();

                ui.menu_button("Esquisse", |ui| {
                    if ui.button("Nouvelle esquisse").clicked() {
                        editor.start_choosing_plane();
                        ui.close();
                    }

                    let drawing = editor.active_sketch().is_some();
                    ui.add_enabled_ui(drawing, |ui| {
                        ui.separator();
                        for tool in [Tool::Line, Tool::Dimension] {
                            if ui
                                .selectable_label(editor.tool == tool, tool.label())
                                .clicked()
                            {
                                editor.tool = tool;
                                editor.end_chain();
                                ui.close();
                            }
                        }
                        ui.separator();
                        if ui.button("Terminer l'esquisse").clicked() {
                            editor.close();
                            ui.close();
                        }
                    });
                });

                ui.separator();
                ui.label(&doc.name)
                    .on_hover_text(path.display().to_string());
            });
        });

        egui::Panel::top("part_status_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                if let Some(plane) = editor.plane {
                    if ui
                        .button("Recadrer sur l'esquisse")
                        .on_hover_text("Remet la vue face au plan et recadre le dessin")
                        .clicked()
                    {
                        let (center, radius) = sketch_framing(doc, editor.active_sketch(), plane);
                        viewport.look_at_plane(plane, center, radius);
                    }
                    ui.separator();
                }

                ui.weak(mode_label(viewport.mode()));

                if editor.active_sketch().is_some() {
                    ui.separator();
                    ui.weak(format!("Outil : {}", editor.tool.label()));
                }

                if let Some(message) = &editor.message {
                    ui.separator();
                    ui.colored_label(egui::Color32::from_rgb(250, 220, 120), message);
                }

                changed |= dimension_field(ui, doc, editor);
            });
        });

        egui::CentralPanel::no_frame().show(ui, |ui| {
            let mut context = screens::viewport::SketchContext {
                document: doc,
                editor,
            };
            changed |= screens::viewport::show(ui, viewport, &mut context);
        });

        if changed {
            let (doc, path) = (doc.clone(), path.clone());
            self.save_part(&doc, &path);
        }

        if back_to_menu {
            self.screen = Screen::StartMenu;
        }
    }

    fn save_part(&mut self, doc: &PartDocument, path: &std::path::Path) {
        if let Err(err) = doc.save(path) {
            self.error = Some(err.to_string());
        }
    }
}

/// The length field shown once a segment is selected with the dimension tool.
/// Returns true when a length was applied.
fn dimension_field(ui: &mut egui::Ui, doc: &mut PartDocument, editor: &mut SketchEditor) -> bool {
    let (Some(index), Some(segment)) = (editor.active_sketch(), editor.selected_segment) else {
        return false;
    };

    ui.separator();
    ui.label("Cote :");
    let field = ui.add(
        egui::TextEdit::singleline(&mut editor.dimension_input)
            .desired_width(90.0)
            .hint_text("mm"),
    );
    let submitted = field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    let applied = ui.button("Appliquer").clicked() || submitted;

    if !applied {
        return false;
    }

    let Ok(millimeters) = editor
        .dimension_input
        .trim()
        .replace(',', ".")
        .parse::<f32>()
    else {
        editor.message = Some("Valeur de cote invalide".to_string());
        return false;
    };

    match doc.apply_dimension(index, segment, millimeters) {
        Some(DimensionOutcome::ScaleDefined {
            millimeters_per_unit,
        }) => {
            editor.message = Some(format!(
                "Échelle définie : 1 unité = {millimeters_per_unit:.4} mm"
            ));
            true
        }
        Some(DimensionOutcome::Geometry(LengthOutcome::Exact)) => {
            editor.message = None;
            true
        }
        Some(DimensionOutcome::Geometry(LengthOutcome::BestEffort)) => {
            editor.message = Some("Contour fermé : seul le point d'arrivée a bougé".to_string());
            true
        }
        _ => {
            editor.message = Some("Cote impossible sur ce trait".to_string());
            false
        }
    }
}

/// Centre and radius to frame: the sketch if it has anything in it, otherwise a
/// sensible patch of the plane around its origin.
fn sketch_framing(doc: &PartDocument, sketch: Option<usize>, plane: WorkPlane) -> (Vec3, f32) {
    let bounds = sketch
        .and_then(|index| doc.sketches.get(index))
        .and_then(|sketch| sketch.bounds());

    match bounds {
        Some((min, max)) if (max - min).length() > 1e-4 => {
            let center = plane.to_world((min + max) * 0.5);
            (center, (max - min).length() * 0.5)
        }
        _ => (plane.origin, screens::viewport::DEFAULT_SKETCH_RADIUS),
    }
}

fn mode_label(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Free => "Vue 3D libre",
        ViewMode::Plane(plane) => plane.label(),
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
