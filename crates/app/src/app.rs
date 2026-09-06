use std::path::PathBuf;

use cao_core::history::Operation;
use cao_core::{Command, PartDocument, Profiles, RecentList};
use cao_render::SceneRenderer;
use cao_sketch::WorkPlane;
use glam::DVec3;

use crate::MSAA_SAMPLES;
use crate::screens::history_tree::HistoryAction;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::{ViewMode, ViewportState};
use crate::screens::{self, OpenPart, Screen, start_menu::StartMenuAction};

pub struct CaoApp {
    screen: Screen,
    recents: RecentList,
    /// Every set of settings the user has, and which one is in use. Held here
    /// rather than in the open part: settings outlive the part being drawn.
    profiles: Profiles,
    settings_open: bool,
    settings_editor: crate::screens::settings::SettingsEditor,
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
            profiles: Profiles::load(),
            settings_open: false,
            settings_editor: crate::screens::settings::SettingsEditor::default(),
            new_part_name: String::new(),
            error: None,
        }
    }

    fn save_settings(&mut self) {
        if let Err(err) = self.profiles.save() {
            self.error = Some(err.to_string());
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
        let settings = self.profiles.active().clone();
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

        // The viewport reads its own copy: it is handed to the renderer every
        // frame and must not borrow from the settings being edited.
        viewport.config = settings.viewport;
        viewport.theme = settings.theme.clone();

        let mut back_to_menu = false;
        let mut changed = false;
        let mut asked: Vec<Command> = Vec::new();

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
            });
        });

        asked.extend(ribbon.show(ui, &settings, doc, editor, extrusion));
        asked.extend(
            shortcuts_pressed(ui, &settings)
                .into_iter()
                .filter(|command| {
                    crate::screens::ribbon::is_enabled(*command, doc, editor, extrusion)
                }),
        );
        for command in asked {
            match command {
                Command::OpenSettings => self.settings_open = true,
                Command::BackToMenu => back_to_menu = true,
                _ => {
                    changed |= run(command, doc, editor, extrusion, ribbon, viewport);
                }
            }
        }

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

    /// The preferences, in a window over whatever is open. Kept out of the
    /// screen routing on purpose: settings apply to the whole application, and
    /// closing them must not close the part being drawn.
    fn show_settings(&mut self, ui: &mut egui::Ui) {
        if !self.settings_open {
            return;
        }
        let mut open = true;
        let mut touched = false;

        egui::Window::new("Préférences")
            .open(&mut open)
            .default_size([720.0, 560.0])
            .vscroll(true)
            .show(ui.ctx(), |ui| {
                touched = screens::settings::show(ui, &mut self.profiles, &mut self.settings_editor);
            });

        self.settings_open = open;
        if touched {
            self.save_settings();
        }
    }

    fn save_part(&mut self, doc: &PartDocument, path: &std::path::Path) {
        if let Err(err) = doc.save(path) {
            self.error = Some(err.to_string());
        }
    }
}

/// Carries out one command, wherever it came from.
///
/// A button and a shortcut both end up here: two paths would drift apart, and
/// a shortcut that does almost what its button does is worse than none.
/// Returns true when the part changed and needs saving.
fn run(
    command: Command,
    doc: &mut PartDocument,
    editor: &mut SketchEditor,
    extrusion: &mut ExtrusionState,
    ribbon: &mut Ribbon,
    viewport: &mut ViewportState,
) -> bool {
    use crate::screens::extrusion::Shape;
    use crate::screens::sketch::{DimensionMode, Tool};

    let tool = |editor: &mut SketchEditor, wanted: Tool| {
        editor.tool = wanted;
        editor.reset_pending();
    };

    match command {
        Command::OpenSettings | Command::BackToMenu => false,
        Command::NewSketch => {
            extrusion.close();
            editor.start_choosing_plane();
            false
        }
        // Finishing a drawing is where an extrusion naturally begins, so the
        // tool is offered right there rather than left to be found again.
        Command::FinishSketch => {
            if let Some(index) = editor.active_sketch() {
                extrusion.offer(index);
                ribbon.tab = 1;
            }
            editor.close();
            false
        }
        Command::RecenterOnSketch => {
            if let Some(plane) = editor.plane {
                let (center, radius) = sketch_framing(doc, editor.active_sketch(), plane);
                viewport.look_at_plane(plane, center, radius);
            }
            false
        }
        Command::Undo | Command::Redo => {
            let changed = if command == Command::Undo {
                doc.undo()
            } else {
                doc.redo()
            };
            if changed {
                clamp_editor_to_document(editor, doc);
            }
            changed
        }
        Command::ToolSelect => {
            tool(editor, Tool::Select);
            false
        }
        Command::ToolLine => {
            tool(editor, Tool::Line);
            false
        }
        Command::ToolRectangle => {
            tool(editor, Tool::Rectangle);
            false
        }
        Command::ToolCircle => {
            tool(editor, Tool::Circle);
            false
        }
        Command::ToolPoint => {
            tool(editor, Tool::Point);
            false
        }
        Command::ToolDimension => {
            tool(editor, Tool::Dimension);
            false
        }
        Command::DimensionAuto
        | Command::DimensionPointToPoint
        | Command::DimensionLength
        | Command::DimensionAngle
        | Command::DimensionRadius => {
            editor.dimension_mode = match command {
                Command::DimensionPointToPoint => DimensionMode::PointToPoint,
                Command::DimensionLength => DimensionMode::Length,
                Command::DimensionAngle => DimensionMode::Angle,
                Command::DimensionRadius => DimensionMode::Radius,
                _ => DimensionMode::Auto,
            };
            editor.reset_pending();
            false
        }
        Command::ExtrusionAdd => {
            extrusion.arm(cao_core::ExtrusionMode::Add);
            false
        }
        Command::ExtrusionCut => {
            extrusion.arm(cao_core::ExtrusionMode::Cut);
            false
        }
        Command::ExtrusionStraight => {
            extrusion.shape = Shape::Straight;
            false
        }
        Command::ExtrusionRevolution => {
            extrusion.shape = Shape::Revolution;
            false
        }
        Command::ExtrusionCancel => {
            extrusion.close();
            ribbon.tab = 0;
            false
        }
        Command::ExtrusionApply => {
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
        Command::ToggleHistory => {
            ribbon.history_open = !ribbon.history_open;
            false
        }
        Command::ToggleToolbarDocked => false,
    }
}

/// The commands whose shortcut was pressed this frame.
///
/// Nothing is read while a text field has the keyboard: typing "50" into a
/// dimension must not also fire whatever those keys are bound to.
fn shortcuts_pressed(ui: &egui::Ui, settings: &cao_core::Settings) -> Vec<Command> {
    if ui.ctx().egui_wants_keyboard_input() {
        return Vec::new();
    }
    ui.input_mut(|input| {
        settings
            .shortcuts
            .bindings
            .iter()
            .filter(|(_, chord)| {
                to_egui_key(chord.key).is_some_and(|key| {
                    input.consume_shortcut(&egui::KeyboardShortcut::new(modifiers(*chord), key))
                })
            })
            .map(|(command, _)| *command)
            .collect()
    })
}

fn modifiers(chord: cao_core::Chord) -> egui::Modifiers {
    let mut modifiers = egui::Modifiers::NONE;
    if chord.command {
        modifiers = modifiers.plus(egui::Modifiers::COMMAND);
    }
    if chord.shift {
        modifiers = modifiers.plus(egui::Modifiers::SHIFT);
    }
    if chord.alt {
        modifiers = modifiers.plus(egui::Modifiers::ALT);
    }
    modifiers
}

fn to_egui_key(key: cao_core::Key) -> Option<egui::Key> {
    use cao_core::Key as K;
    use egui::Key as E;
    Some(match key {
        K::A => E::A, K::B => E::B, K::C => E::C, K::D => E::D, K::E => E::E,
        K::F => E::F, K::G => E::G, K::H => E::H, K::I => E::I, K::J => E::J,
        K::K => E::K, K::L => E::L, K::M => E::M, K::N => E::N, K::O => E::O,
        K::P => E::P, K::Q => E::Q, K::R => E::R, K::S => E::S, K::T => E::T,
        K::U => E::U, K::V => E::V, K::W => E::W, K::X => E::X, K::Y => E::Y,
        K::Z => E::Z,
        K::Num0 => E::Num0, K::Num1 => E::Num1, K::Num2 => E::Num2,
        K::Num3 => E::Num3, K::Num4 => E::Num4, K::Num5 => E::Num5,
        K::Num6 => E::Num6, K::Num7 => E::Num7, K::Num8 => E::Num8,
        K::Num9 => E::Num9,
        K::F1 => E::F1, K::F2 => E::F2, K::F3 => E::F3, K::F4 => E::F4,
        K::F5 => E::F5, K::F6 => E::F6, K::F7 => E::F7, K::F8 => E::F8,
        K::F9 => E::F9, K::F10 => E::F10, K::F11 => E::F11, K::F12 => E::F12,
        K::Escape => E::Escape,
        K::Tab => E::Tab,
        K::Space => E::Space,
        K::Enter => E::Enter,
        K::Backspace => E::Backspace,
        K::Delete => E::Delete,
        K::Home => E::Home,
        K::End => E::End,
        K::PageUp => E::PageUp,
        K::PageDown => E::PageDown,
        K::Left => E::ArrowLeft,
        K::Right => E::ArrowRight,
        K::Up => E::ArrowUp,
        K::Down => E::ArrowDown,
        K::Plus => E::Plus,
        K::Minus => E::Minus,
        K::Comma => E::Comma,
        K::Period => E::Period,
    })
}

/// Turns the chosen areas into matter, or takes them out of it.
fn apply_extrusion(doc: &mut PartDocument, extrusion: &mut ExtrusionState) -> bool {
    let (Some(sketch), Some(mode)) = (extrusion.sketch, extrusion.mode) else {
        return false;
    };
    if !extrusion.is_ready() {
        return false;
    }

    let before = doc.body().clone();
    let picks = std::mem::take(&mut extrusion.picks);
    let operation = if extrusion.is_revolving() {
        Operation::Revolve {
            sketch,
            picks,
            axis: extrusion.axis,
            angle: extrusion.angle().unwrap_or_default(),
            mode,
        }
    } else {
        Operation::Extrude {
            sketch,
            picks,
            distance: extrusion.distance().unwrap_or_default(),
            mode,
        }
    };
    doc.apply(operation);

    // An extrusion that changes nothing is worth saying out loud: a cut that
    // misses the matter looks exactly like a tool that did not work.
    extrusion.message = (doc.body() == &before).then(|| {
        match (mode, extrusion.is_revolving()) {
            (_, true) => {
                "Rien produit : l'aire est peut-être à cheval sur l'axe, ce qui la ferait passer à travers elle-même."
            }
            (cao_core::ExtrusionMode::Add, false) => "L'extrusion n'a rien ajouté.",
            (cao_core::ExtrusionMode::Cut, false) => {
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

/// Centre and radius to frame: the sketch if it has anything in it, otherwise a
/// sensible patch of the plane around its origin.
fn sketch_framing(doc: &PartDocument, sketch: Option<usize>, plane: WorkPlane) -> (DVec3, f64) {
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
            if ui.button("⚙ Préférences").clicked() {
                self.settings_open = true;
            }
        } else {
            self.show_part(ui);
        }
        self.show_settings(ui);
    }
}
