use std::path::PathBuf;

use cao_part::PartDocument;
use cao_prefs::{Command, Locations};
use cao_render::SceneRenderer;

use crate::commands;
use crate::lang::Catalogue;
use crate::remembered::Remembered;
use crate::screens::explorer::Explorer;
use crate::screens::viewport::{ViewMode, ViewportState};
use crate::screens::{
    self, OpenPart, Screen, extrusion::ExtrusionState, history_tree::HistoryAction, ribbon::Ribbon,
    sketch::SketchEditor, start_menu::StartMenuAction,
};
use crate::shortcuts::shortcuts_pressed;
use crate::{MSAA_SAMPLES, adapters::files::DiskFiles, autosave::Autosave, wording};

pub struct CaoApp {
    screen: Screen,
    remembered: Remembered,
    settings_open: bool,
    settings_editor: crate::screens::settings::SettingsEditor,
    new_part_name: String,
    error: Option<String>,
    /// The library panel, held by the shell rather than by a screen: the same
    /// browsing serves the start menu and the part being drawn.
    explorer: Explorer,
    /// What takes a part's picture. Nothing when the platform gave no GPU.
    painter: Option<crate::picture::Painter>,
}

impl CaoApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        locations: Option<Locations>,
        opening: Option<PathBuf>,
    ) -> Self {
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

        let painter = crate::picture::Painter::new(cc.wgpu_render_state.as_ref());
        let remembered = Remembered::read(locations);
        let explorer = Explorer::at(remembered.projects_dir().unwrap_or_default());
        let mut app = Self {
            screen: Screen::StartMenu,
            remembered,
            settings_open: false,
            settings_editor: crate::screens::settings::SettingsEditor::default(),
            new_part_name: String::new(),
            error: None,
            explorer,
            painter,
        };
        if let Some(part) = opening {
            app.open_part(part);
        }
        app
    }

    fn save_settings(&mut self) {
        if self.remembered.has_nowhere_to_keep() {
            self.error = Some(wording::storage::nowhere_to_keep_settings(
                self.remembered.lang(),
            ));
        } else if let Some(err) = self.remembered.save_profiles() {
            self.error = Some(wording::storage::say(self.remembered.lang(), &err));
        }
    }

    fn create_new_part(&mut self, name: String) {
        let Some(dir) = self.remembered.projects_dir() else {
            self.error = Some(wording::storage::nowhere_to_keep_settings(
                self.remembered.lang(),
            ));
            return;
        };
        match PartDocument::create_in(&DiskFiles, &dir, name, chrono::Utc::now()) {
            Ok((doc, path)) => {
                self.explorer.went_stale();
                self.open_document(doc, path)
            }
            Err(err) => self.error = Some(wording::part_file::say(self.remembered.lang(), &err)),
        }
    }

    fn open_part(&mut self, path: PathBuf) {
        match PartDocument::load(&DiskFiles, &path) {
            Ok(doc) => self.open_document(doc, path),
            Err(err) => self.error = Some(wording::part_file::say(self.remembered.lang(), &err)),
        }
    }

    fn open_document(&mut self, doc: PartDocument, path: PathBuf) {
        let failure = self
            .remembered
            .remember_part(&path, doc.name(), chrono::Utc::now());
        let lang = self.remembered.lang();
        self.error = failure.as_ref().map(|err| wording::storage::say(lang, err));
        self.screen = Screen::PartOpened(Box::new(OpenPart {
            doc,
            path,
            autosave: Autosave::default(),
            viewport: ViewportState::default(),
            editor: SketchEditor::default(),
            extrusion: ExtrusionState::default(),
            ribbon: Ribbon::new(),
        }));
    }

    fn show_start_menu(&mut self, ui: &mut egui::Ui) {
        // Always here, whatever the panel is doing beside a drawing: choosing
        // a part is the whole of what this screen is for.
        if let Some(part) = screens::explorer::run(
            &mut self.explorer,
            ui,
            self.remembered.lang(),
            &DiskFiles,
            &DiskFiles,
            None,
        ) {
            self.open_part(part);
            return;
        }
        let action = egui::CentralPanel::default_margins()
            .show(ui, |ui| {
                screens::start_menu::show(
                    ui,
                    self.remembered.recents(),
                    &mut self.new_part_name,
                    self.remembered.lang(),
                )
            })
            .inner;

        match action {
            StartMenuAction::CreateNew(name) => self.create_new_part(name),
            StartMenuAction::Open(path) => self.open_part(path),
            StartMenuAction::None => {}
        }
    }

    fn show_part(&mut self, ui: &mut egui::Ui) {
        let settings = self.remembered.profiles.active().clone();
        let lang = self.remembered.lang();
        let Screen::PartOpened(part) = &mut self.screen else {
            return;
        };
        let OpenPart {
            doc,
            path,
            autosave,
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
                back_to_menu = ui.button(lang.t("app.home")).clicked();
                ui.separator();
                ui.strong(doc.name())
                    .on_hover_text(path.display().to_string());
                ui.separator();
                ui.weak(mode_label(lang, viewport.mode()));
                if let Some(message) = &editor.message {
                    ui.separator();
                    ui.colored_label(egui::Color32::from_rgb(250, 220, 120), message);
                }
            });
        });

        let mut drawn = screens::ribbon::Drawn {
            document: doc,
            editor,
            extrusion,
            explorer_open: self.explorer.is_open(),
        };
        asked.extend(ribbon.show(ui, &settings, &mut drawn, lang));
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
                Command::ToggleExplorer => self.explorer.toggle(),
                _ => {
                    changed |=
                        commands::run(command, doc, editor, extrusion, ribbon, viewport, lang);
                }
            }
        }

        let mut open_elsewhere = None;
        if self.explorer.is_open() {
            open_elsewhere = screens::explorer::run(
                &mut self.explorer,
                ui,
                lang,
                &DiskFiles,
                &DiskFiles,
                Some(path),
            );
        }

        if ribbon.history_open {
            match screens::history_tree::panel(ui, doc, &mut ribbon.history_compact_confirm, lang) {
                HistoryAction::RewindTo(step) => {
                    doc.rewind_to(step);
                    commands::clamp_editor_to_document(editor, doc);
                    changed = true;
                }
                HistoryAction::EditSketch(sketch) => {
                    if let Some(plane) = doc.sketches().get(sketch).map(|s| s.plane) {
                        editor.begin_editing(sketch, plane);
                        let (center, radius) = commands::sketch_framing(doc, Some(sketch), plane);
                        viewport.look_at_plane(plane, center, radius);
                    }
                }
                HistoryAction::CompactHistory => {
                    doc.compact_history();
                    commands::clamp_editor_to_document(editor, doc);
                    changed = true;
                }
                HistoryAction::None => {}
            }
        }

        egui::CentralPanel::no_frame().show(ui, |ui| {
            let mut context = screens::viewport::SketchContext {
                document: doc,
                editor,
                extrusion,
                lang,
            };
            changed |= screens::viewport::show(ui, viewport, &mut context);
        });

        if changed {
            autosave.touched();
        }
        // On the way out the writing is left to `put_the_part_away`, which has
        // the part's fresh picture to write with it.
        let at_rest = !ui.ctx().input(|input| input.pointer.any_down());
        if !back_to_menu
            && let Some(message) = autosave.write_if_due(&DiskFiles, doc, path, at_rest, lang)
        {
            self.error = Some(message);
        }
        if back_to_menu {
            self.put_the_part_away();
            self.screen = Screen::StartMenu;
        }
        if let Some(part) = open_elsewhere
            && !crate::adapters::window::open_another(&part)
        {
            self.error = Some(self.remembered.lang().t("app.no_second_window"));
        }
    }

    /// Writes the part down with a picture of itself, on the way out of it.
    ///
    /// Here rather than in the autosave because the autosave runs at the end of
    /// every gesture, and a render with a readback at every gesture is the cost
    /// the picture exists to avoid. Once a part is closed is enough: nothing
    /// looks at the picture until a panel lists the folder.
    fn put_the_part_away(&mut self) {
        let Screen::PartOpened(part) = &mut self.screen else {
            return;
        };

        let taken = self
            .painter
            .as_mut()
            .and_then(|painter| painter.take(&part.doc));
        // A part only looked at and closed again comes out of the renderer
        // exactly as it went in, and writing it would move its hour for
        // nothing.
        if let Some(picture) = taken
            && part.doc.picture() != Some(&picture)
        {
            part.doc.set_picture(picture);
            part.autosave.touched();
            self.explorer.went_stale();
        }

        let lang = self.remembered.lang();
        if let Some(message) = part
            .autosave
            .write_if_due(&DiskFiles, &part.doc, &part.path, true, lang)
        {
            self.error = Some(message);
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

        egui::Window::new(self.remembered.lang().t("app.settings_window"))
            .open(&mut open)
            .default_size([720.0, 560.0])
            .vscroll(true)
            .show(ui.ctx(), |ui| {
                let (profiles, lang) = self.remembered.profiles_and_lang();
                touched = screens::settings::show(ui, profiles, &mut self.settings_editor, lang);
            });

        self.settings_open = open;
        if touched {
            self.save_settings();
        }
    }
}

fn mode_label(lang: &Catalogue, mode: ViewMode) -> String {
    match mode {
        ViewMode::Free => lang.t("app.free_view"),
        ViewMode::Plane(work_plane) => wording::plane::label(lang, work_plane.kind()),
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
            if ui
                .button(self.remembered.lang().t("app.settings"))
                .clicked()
            {
                self.settings_open = true;
            }
        } else {
            self.show_part(ui);
        }
        self.show_settings(ui);
    }

    fn on_exit(&mut self) {
        self.put_the_part_away();
    }
}
