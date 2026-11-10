use std::path::PathBuf;

use cao_part::PartDocument;
use cao_prefs::{Command, Locations};
use cao_render::SceneRenderer;

use crate::adapters::window_place::WindowKeeper;
use crate::remembered::Remembered;
use crate::screens::explorer::Explorer;
use crate::screens::viewport::ViewportState;
use crate::screens::{
    self, OpenPart, Screen, extrusion::ExtrusionState, ribbon::Ribbon, sketch::SketchEditor,
    start_menu::StartMenuAction,
};
use crate::shortcuts::shortcuts_pressed;
use crate::{MSAA_SAMPLES, adapters::files::DiskFiles, autosave::Autosave, wording};
use crate::{commands, export};
pub mod on_screen;
mod settings_window;

pub struct CaoApp {
    screen: Screen,
    remembered: Remembered,
    settings_open: bool,
    settings_editor: crate::screens::settings::SettingsEditor,
    new_part_name: String,
    error: Option<String>,
    /// The library panel, held by the shell rather than by the start menu:
    /// what it read and folded is still there on the way back from a part.
    explorer: Explorer,
    /// What takes a part's picture. Nothing when the platform gave no GPU.
    painter: Option<crate::picture::Painter>,
    window: WindowKeeper,
}

impl CaoApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        locations: Option<Locations>,
        opening: Option<PathBuf>,
    ) -> Self {
        crate::ui::install(&cc.egui_ctx);
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
        let window = WindowKeeper::open(
            cc.winit_window(),
            remembered.locations(),
            remembered.profiles.active().reopen_at_last_size,
        );
        let mut app = Self {
            screen: Screen::StartMenu,
            remembered,
            settings_open: false,
            settings_editor: crate::screens::settings::SettingsEditor::default(),
            new_part_name: String::new(),
            error: None,
            explorer,
            painter,
            window,
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
            variables: screens::variables::VariablesPanel::default(),
            covering: None,
        }));
    }

    fn show_start_menu(&mut self, ui: &mut egui::Ui) {
        // Always here: choosing a part is the whole of what this screen is for.
        if let Some(part) = screens::explorer::run(
            &mut self.explorer,
            ui,
            self.remembered.lang(),
            &DiskFiles,
            &DiskFiles,
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
            variables,
            covering,
        } = part.as_mut();

        // The viewport reads its own copy: it is handed to the renderer every
        // frame and must not borrow from the settings being edited.
        viewport.config = settings.viewport;
        viewport.theme = settings.theme.clone();
        viewport.let_go = settings.shortcuts.let_go;
        editor.sketching = settings.sketching;
        viewport.copy_panel_at = settings.copy_panel_at;

        // An error raised on this screen goes to the title bar's pill, which
        // the frame's end settles; until then the shell holds it.
        let at_start = editor.message.clone();
        let error = covering.as_ref().map(|lying| lying.error.as_str());
        let bar = screens::title_bar::TitleBar::of(doc, path, editor, viewport, error, lang);
        let mut back_to_menu = screens::title_bar::show(ui, &bar, lang);
        let mut changed = false;
        let mut asked: Vec<Command> = Vec::new();

        let mut drawn = screens::ribbon::Drawn {
            document: doc,
            editor,
            extrusion,
        };
        ribbon.section_on = viewport.section().is_some();
        asked.extend(ribbon.show(ui, &settings, &mut drawn, lang));
        asked.extend(
            shortcuts_pressed(ui, &settings, |command| {
                // Enter moves through the panel of a tool laying copies rather
                // than finishing the sketch under it.
                command == Command::FinishSketch && editor.copy_panel().is_some()
            })
            .into_iter()
            .filter(|command| crate::screens::ribbon::is_enabled(*command, doc, editor, extrusion)),
        );
        let mut worked_on = screens::SketchContext {
            document: doc,
            editor,
            extrusion,
            lang,
        };
        let commanded = !asked.is_empty();
        for command in asked {
            match command {
                Command::OpenSettings => self.settings_open = true,
                Command::BackToMenu => back_to_menu = true,
                Command::ExportStep => export::step(
                    &mut worked_on,
                    &DiskFiles,
                    crate::adapters::save_dialog::step_destination,
                    &mut self.error,
                ),
                _ => match export::export_request(command, worked_on.document) {
                    Some(request) => export::stl(
                        command,
                        request,
                        &mut worked_on,
                        &DiskFiles,
                        crate::adapters::save_dialog::stl_destination,
                        &mut self.error,
                    ),
                    None => changed |= commands::run(command, &mut worked_on, ribbon, viewport),
                },
            }
        }

        let beside = crate::panels::Beside { ribbon, variables };
        changed |= crate::panels::beside_the_part(ui, doc, editor, viewport, beside, lang);

        let view = egui::CentralPanel::no_frame().show(ui, |ui| {
            let mut context = screens::SketchContext {
                document: doc,
                editor,
                extrusion,
                lang,
            };
            screens::viewport::show(ui, viewport, &mut context)
        });
        changed |= view.inner.changed;
        let copy_panel_let_go_at = view.inner.copy_panel_let_go_at;

        let finish_on = settings.shortcuts.chord_for(Command::FinishSketch);
        let finished = screens::sketch_pill::SketchPill::of(doc, editor, finish_on, lang)
            .is_some_and(|pill| screens::sketch_pill::show(ui.ctx(), view.response.rect, &pill));
        let mut worked_on = screens::SketchContext {
            document: doc,
            editor,
            extrusion,
            lang,
        };
        let bar =
            screens::view_bar::ViewBar::of(&worked_on, viewport, &view.inner, &settings.shortcuts);
        let from_the_bar = screens::view_bar::show(ui.ctx(), view.response.rect, &bar);
        let from_the_cube = screens::origin_view::OriginViewButton::of(
            &worked_on,
            viewport,
            view.response.rect,
            &settings.shortcuts,
        )
        .and_then(|button| screens::origin_view::show(ui.ctx(), view.response.rect, &button));
        let over_the_view = finished
            .then_some(Command::FinishSketch.into())
            .or(from_the_bar)
            .or(from_the_cube);
        if let Some(asked) = over_the_view {
            changed |= commands::run(asked, &mut worked_on, ribbon, viewport);
        }

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
        let settled = screens::title_bar::settle(
            covering.clone(),
            self.error.take(),
            at_start.as_deref(),
            &mut editor.message,
            commanded || over_the_view.is_some() || back_to_menu,
        );
        if settled != *covering {
            ui.ctx().request_repaint();
        }
        *covering = settled;
        if let Some(at) = copy_panel_let_go_at {
            self.remembered.profiles.active_mut().copy_panel_at = Some(at);
            // With nowhere to keep it, the place holds for the session alone.
            if !self.remembered.has_nowhere_to_keep() {
                self.save_settings();
            }
        }
        if back_to_menu {
            self.put_the_part_away();
            self.screen = Screen::StartMenu;
        }
    }

    /// Writes the part down with a picture of itself and the geometry it is
    /// showing, on the way out of it.
    ///
    /// Here rather than in the autosave because the autosave runs at the end of
    /// every gesture, and a render with a readback at every gesture is the cost
    /// the picture exists to avoid. Once a part is closed is enough for both:
    /// nothing looks at the picture until a panel lists the folder, and nothing
    /// reads the geometry back until the part is opened again.
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
            .put_away_if_due(&DiskFiles, &part.doc, &part.path, lang)
        {
            self.error = Some(message);
        }
    }
}

impl eframe::App for CaoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.window.follow();
        let error_before = self.error.clone();
        // The part screen says its errors in its title bar's pill.
        if matches!(self.screen, Screen::StartMenu)
            && let Some(error) = &self.error
        {
            let message = error.clone();
            egui::Panel::bottom("error_bar").show(ui, |ui| {
                ui.colored_label(crate::ui::theme::tokens::STATE_ERROR_TEXT, message);
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
        screens::viewport::repaint_if_the_atlas_grew(ui.ctx());
        // An error found at the end of a frame is drawn by the next one, which
        // nothing else may be about to ask for.
        if self.error != error_before {
            ui.ctx().request_repaint();
        }
    }

    fn on_exit(&mut self) {
        self.put_the_part_away();
        self.window.put_down();
    }
}
