use std::path::PathBuf;

use cao_core::history::Operation;
use cao_core::{DimensionOutcome, PartDocument, RecentList};
use cao_render::SceneRenderer;
use cao_sketch::{DimensionTarget, LengthOutcome, WorkPlane};
use glam::Vec3;

use crate::MSAA_SAMPLES;
use crate::screens::history_tree::HistoryAction;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::ribbon::{Category, Ribbon, RibbonAction};
use crate::screens::sketch::SketchEditor;
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
        self.recents.push(path.clone(), doc.name().to_string());
        if let Err(err) = self.recents.save() {
            self.error = Some(err.to_string());
        }
        self.error = None;
        self.screen = Screen::PartOpened(Box::new(OpenPart {
            doc,
            path,
            viewport: ViewportState::default(),
            editor: SketchEditor::default(),
            extrusion: ExtrusionState::default(),
            ribbon: Ribbon::new(),
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
            extrusion,
            ribbon,
        } = part.as_mut();

        let mut back_to_menu = false;
        let mut changed = false;

        egui::Panel::top("part_title_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                back_to_menu = ui.button("⌂ Accueil").clicked();
                ui.separator();
                ui.strong(doc.name())
                    .on_hover_text(path.display().to_string());
                ui.separator();
                ui.weak(mode_label(viewport.mode()));
                for message in [&editor.message, &extrusion.message].into_iter().flatten() {
                    ui.separator();
                    ui.colored_label(egui::Color32::from_rgb(250, 220, 120), message);
                }
                changed |= dimension_field(ui, doc, editor);
            });
        });

        let action = ribbon.show(ui, doc, editor, extrusion);
        changed |= apply_ribbon_action(action, doc, editor, extrusion, ribbon, viewport);

        if ribbon.history_open {
            egui::Panel::left("history_panel")
                .resizable(true)
                .default_size(220.0)
                .show(ui, |ui| match screens::history_tree::show(ui, doc) {
                    HistoryAction::RewindTo(step) => {
                        doc.rewind_to(step);
                        clamp_editor_to_document(editor, doc);
                        changed = true;
                    }
                    HistoryAction::EditSketch(sketch) => {
                        if let Some(plane) = doc.sketches().get(sketch).map(|s| s.plane) {
                            editor.begin_editing(sketch, plane);
                            let (center, radius) = sketch_framing(doc, Some(sketch), plane);
                            viewport.look_at_plane(plane, center, radius);
                        }
                    }
                    HistoryAction::None => {}
                });
        }

        egui::CentralPanel::no_frame().show(ui, |ui| {
            let mut context = screens::viewport::SketchContext {
                document: doc,
                editor,
                extrusion,
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

    fn handle_shortcuts(&mut self, ui: &egui::Ui) {
        let Screen::PartOpened(part) = &mut self.screen else {
            return;
        };

        let (undo, redo) = ui.input_mut(|input| {
            (
                input.consume_shortcut(&egui::KeyboardShortcut::new(
                    egui::Modifiers::COMMAND,
                    egui::Key::Z,
                )),
                input.consume_shortcut(&egui::KeyboardShortcut::new(
                    egui::Modifiers::COMMAND,
                    egui::Key::Y,
                )) || input.consume_shortcut(&egui::KeyboardShortcut::new(
                    egui::Modifiers::COMMAND.plus(egui::Modifiers::SHIFT),
                    egui::Key::Z,
                )),
            )
        });

        let changed = (undo && part.doc.undo()) || (redo && part.doc.redo());
        if !changed {
            return;
        }

        clamp_editor_to_document(&mut part.editor, &part.doc);
        let (doc, path) = (part.doc.clone(), part.path.clone());
        self.save_part(&doc, &path);
    }

    fn save_part(&mut self, doc: &PartDocument, path: &std::path::Path) {
        if let Err(err) = doc.save(path) {
            self.error = Some(err.to_string());
        }
    }
}

fn apply_ribbon_action(
    action: RibbonAction,
    doc: &mut PartDocument,
    editor: &mut SketchEditor,
    extrusion: &mut ExtrusionState,
    ribbon: &mut Ribbon,
    viewport: &mut ViewportState,
) -> bool {
    match action {
        RibbonAction::None => false,
        RibbonAction::NewSketch => {
            extrusion.close();
            editor.start_choosing_plane();
            false
        }
        // Finishing a drawing is where an extrusion naturally begins, so the
        // tool is offered right there rather than left to be found again.
        RibbonAction::FinishSketch => {
            if let Some(index) = editor.active_sketch() {
                extrusion.offer(index);
                ribbon.category = Category::Extrusion;
            }
            editor.close();
            false
        }
        RibbonAction::CancelExtrusion => {
            extrusion.close();
            ribbon.category = Category::Sketch;
            false
        }
        RibbonAction::ApplyExtrusion => {
            let changed = apply_extrusion(doc, extrusion);
            if changed {
                // Seen from straight above its own plane, a new prism looks
                // exactly like the drawing it came from.
                if let Some((min, max)) = doc.body().bounds() {
                    viewport.look_at_part((min + max) * 0.5, (max - min).length() * 0.6);
                }
            }
            changed
        }
        RibbonAction::Undo | RibbonAction::Redo => {
            let changed = if action == RibbonAction::Undo {
                doc.undo()
            } else {
                doc.redo()
            };
            if changed {
                clamp_editor_to_document(editor, doc);
            }
            changed
        }
        RibbonAction::RecenterOnSketch => {
            if let Some(plane) = editor.plane {
                let (center, radius) = sketch_framing(doc, editor.active_sketch(), plane);
                viewport.look_at_plane(plane, center, radius);
            }
            false
        }
    }
}

/// Turns the chosen areas into matter, or takes them out of it.
fn apply_extrusion(doc: &mut PartDocument, extrusion: &mut ExtrusionState) -> bool {
    let (Some(sketch), Some(mode), Some(distance)) = (
        extrusion.sketch,
        extrusion.mode,
        extrusion.distance(),
    ) else {
        return false;
    };
    if extrusion.picks.is_empty() {
        return false;
    }

    let before = doc.body().clone();
    let picks = std::mem::take(&mut extrusion.picks);
    doc.apply(Operation::Extrude {
        sketch,
        picks,
        distance,
        mode,
    });

    // An extrusion that changes nothing is worth saying out loud: a cut that
    // misses the matter looks exactly like a tool that did not work.
    extrusion.message = (doc.body() == &before).then(|| {
        match mode {
            cao_core::ExtrusionMode::Add => "L'extrusion n'a rien ajouté.",
            cao_core::ExtrusionMode::Cut => {
                "Rien enlevé : la matière n'est pas de ce côté du plan (essayez « Sens inverse »)."
            }
        }
        .to_string()
    });
    extrusion.mode = None;
    true
}

/// After the history moves, the sketch being edited may no longer exist. The
/// editor has to let go of it rather than point at nothing.
fn clamp_editor_to_document(editor: &mut SketchEditor, doc: &PartDocument) {
    editor.reset_pending();

    let Some(index) = editor.active_sketch() else {
        return;
    };
    match doc.sketches().get(index) {
        Some(sketch) => editor.plane = Some(sketch.plane),
        None => editor.close(),
    }
}

/// The value field shown once the dimension or angle tool has picked
/// something. Returns true when a value was applied.
fn dimension_field(ui: &mut egui::Ui, doc: &mut PartDocument, editor: &mut SketchEditor) -> bool {
    let (Some(index), Some(target)) = (editor.active_sketch(), editor.selected) else {
        return false;
    };

    let driven = doc.sketches()[index]
        .dimension_of(target)
        .is_some_and(|dimension| dimension.driven);
    let angle = matches!(target, DimensionTarget::Angle { .. });

    ui.separator();
    ui.label(if angle { "Angle :" } else { "Cote :" });

    if driven {
        // A readout cannot be edited: changing it would mean nothing, since it
        // reports the geometry rather than deciding it.
        let measured = doc.measured(index, target).unwrap_or_default();
        let suffix = if angle { "°" } else { "mm" };
        ui.weak(format!("{measured:.2} {suffix} (lecture seule)"));
        return false;
    }

    let field = ui.add(
        egui::TextEdit::singleline(&mut editor.dimension_input)
            .desired_width(90.0)
            .hint_text(if angle { "degrés" } else { "mm" }),
    );
    let submitted = field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    let applied = ui.button("Appliquer").clicked() || submitted;

    if !applied {
        return false;
    }

    let Ok(value) = editor
        .dimension_input
        .trim()
        .replace(',', ".")
        .parse::<f32>()
    else {
        editor.message = Some("Valeur invalide".to_string());
        return false;
    };

    match doc.apply(Operation::SetDimension {
        sketch: index,
        target,
        value,
    }) {
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
        Some(DimensionOutcome::Reference) => {
            editor.message = Some(screens::viewport::REDUNDANT_WARNING.to_string());
            true
        }
        _ => {
            editor.message = Some("Cote impossible ici".to_string());
            false
        }
    }
}

/// Centre and radius to frame: the sketch if it has anything in it, otherwise a
/// sensible patch of the plane around its origin.
fn sketch_framing(doc: &PartDocument, sketch: Option<usize>, plane: WorkPlane) -> (Vec3, f32) {
    let bounds = sketch
        .and_then(|index| doc.sketches().get(index))
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
            self.handle_shortcuts(ui);
            self.show_part(ui);
        }
    }
}
