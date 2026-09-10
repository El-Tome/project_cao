mod appearance;
mod navigation;
mod profiles;
mod shortcuts;
mod toolbar;
mod viewport;

use cao_prefs::{Command, Path, Profiles};

use crate::lang::Catalogue;

/// Which part of the preferences is open.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Section {
    #[default]
    Profiles,
    Viewport,
    Navigation,
    Appearance,
    Shortcuts,
    Toolbar,
}

impl Section {
    const ALL: [Self; 6] = [
        Self::Profiles,
        Self::Viewport,
        Self::Navigation,
        Self::Appearance,
        Self::Shortcuts,
        Self::Toolbar,
    ];

    fn label(self, lang: &Catalogue) -> String {
        lang.t(match self {
            Self::Profiles => "settings.section.profiles",
            Self::Viewport => "settings.section.viewport",
            Self::Navigation => "settings.section.navigation",
            Self::Appearance => "settings.section.appearance",
            Self::Shortcuts => "settings.section.shortcuts",
            Self::Toolbar => "settings.section.toolbar",
        })
    }
}

/// What the preferences screen remembers between frames: which section is open,
/// what is selected, and the text being typed. None of it is a setting, so none
/// of it is saved.
#[derive(Default)]
pub struct SettingsEditor {
    section: Section,
    /// The toolbar entry being worked on.
    selected: Path,
    /// The command waiting for a key to be pressed.
    recording: Option<Command>,
    new_profile_name: String,
    new_group_name: String,
    profile_path: String,
    notice: Option<String>,
}

/// Draws the preferences. Returns true when something changed and the profiles
/// should be written back to disk.
pub fn show(
    ui: &mut egui::Ui,
    profiles: &mut Profiles,
    editor: &mut SettingsEditor,
    lang: &Catalogue,
) -> bool {
    let mut touched = false;

    ui.horizontal_wrapped(|ui| {
        for section in Section::ALL {
            if ui
                .selectable_label(editor.section == section, section.label(lang))
                .clicked()
            {
                editor.section = section;
                editor.notice = None;
            }
        }
    });
    ui.separator();

    match editor.section {
        Section::Profiles => touched |= profiles::section(ui, profiles, editor, lang),
        Section::Viewport => touched |= viewport::section(ui, profiles),
        Section::Navigation => touched |= navigation::section(ui, profiles),
        Section::Appearance => touched |= appearance::section(ui, profiles),
        Section::Shortcuts => touched |= shortcuts::section(ui, profiles, editor, lang),
        Section::Toolbar => touched |= toolbar::section(ui, profiles, editor, lang),
    }

    if let Some(notice) = &editor.notice {
        ui.separator();
        ui.weak(notice);
    }
    touched
}
