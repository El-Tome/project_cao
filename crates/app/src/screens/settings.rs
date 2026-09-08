use cao_prefs::command::Command;
use cao_prefs::config::{
    LengthUnit, NavigationPreset, TrackpadGesture, UnitDisplay, ViewportCorner,
};
use cao_prefs::settings::{DEFAULT_PROFILE, PROFILE_EXTENSION, Profile, Profiles};
use cao_prefs::shortcuts::{Chord, Key};
use cao_prefs::theme::{Background, Rgba, Stop, Theme};
use cao_prefs::toolbar::{Edge, Item, Path};

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

    fn label(self) -> &'static str {
        match self {
            Self::Profiles => "Profils",
            Self::Viewport => "Viewport",
            Self::Navigation => "Navigation",
            Self::Appearance => "Apparence",
            Self::Shortcuts => "Raccourcis",
            Self::Toolbar => "Barre d'outils",
        }
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
pub fn show(ui: &mut egui::Ui, profiles: &mut Profiles, editor: &mut SettingsEditor) -> bool {
    let mut touched = false;

    ui.horizontal_wrapped(|ui| {
        for section in Section::ALL {
            if ui
                .selectable_label(editor.section == section, section.label())
                .clicked()
            {
                editor.section = section;
                editor.notice = None;
            }
        }
    });
    ui.separator();

    match editor.section {
        Section::Profiles => touched |= profiles_section(ui, profiles, editor),
        Section::Viewport => touched |= viewport_section(ui, profiles),
        Section::Navigation => touched |= navigation_section(ui, profiles),
        Section::Appearance => touched |= appearance_section(ui, profiles),
        Section::Shortcuts => touched |= shortcuts_section(ui, profiles, editor),
        Section::Toolbar => touched |= toolbar_section(ui, profiles, editor),
    }

    if let Some(notice) = &editor.notice {
        ui.separator();
        ui.weak(notice);
    }
    touched
}

fn profiles_section(
    ui: &mut egui::Ui,
    profiles: &mut Profiles,
    editor: &mut SettingsEditor,
) -> bool {
    let mut touched = false;

    ui.label("Chaque profil est un jeu de réglages complet. Ce qui est modifié ailleurs va dans le profil actif.");
    ui.add_space(6.0);

    let names: Vec<String> = profiles.names().map(str::to_string).collect();
    let active = profiles.active_name().to_string();
    ui.horizontal_wrapped(|ui| {
        for name in &names {
            if ui.selectable_label(*name == active, name).clicked() {
                touched |= profiles.switch_to(name);
            }
        }
    });

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut editor.new_profile_name)
                .desired_width(160.0)
                .hint_text("nom du profil"),
        );
        if ui.button("Dupliquer l'actif").clicked() {
            let name = if editor.new_profile_name.trim().is_empty() {
                format!("{active} (copie)")
            } else {
                editor.new_profile_name.trim().to_string()
            };
            let created = profiles.duplicate_active(&name);
            editor.notice = Some(format!("Profil « {created} » créé."));
            editor.new_profile_name.clear();
            touched = true;
        }
        let removable = active != DEFAULT_PROFILE;
        ui.add_enabled_ui(removable, |ui| {
            if ui.button("Supprimer l'actif").clicked() {
                touched |= profiles.remove(&active);
            }
        });
    });

    ui.add_space(12.0);
    ui.heading("Partager");
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut editor.profile_path)
                .desired_width(320.0)
                .hint_text(format!("chemin d'un fichier .{PROFILE_EXTENSION}")),
        );
        if ui.button("Exporter").clicked() {
            let path = std::path::PathBuf::from(editor.profile_path.trim());
            match profiles.active_profile().export(&path) {
                Ok(()) => editor.notice = Some(format!("Écrit dans {}", path.display())),
                Err(err) => editor.notice = Some(err.to_string()),
            }
        }
        if ui.button("Importer").clicked() {
            let path = std::path::PathBuf::from(editor.profile_path.trim());
            match Profile::import(&path) {
                Ok(profile) => {
                    let name = profiles.add(profile);
                    editor.notice = Some(format!("Profil « {name} » importé."));
                    touched = true;
                }
                Err(err) => editor.notice = Some(err.to_string()),
            }
        }
    });

    ui.add_space(12.0);
    ui.separator();
    if ui
        .button("↺ Tout remettre par défaut")
        .on_hover_text("Ne touche que le profil actif")
        .clicked()
    {
        profiles.reset_active();
        editor.notice = Some(format!(
            "Profil « {active} » remis à ses valeurs d'origine."
        ));
        touched = true;
    }

    touched
}

fn viewport_section(ui: &mut egui::Ui, profiles: &mut Profiles) -> bool {
    let before = profiles.active().viewport;
    let config = &mut profiles.active_mut().viewport;

    ui.heading("Cube d'orientation");
    corner_picker(ui, "Coin", &mut config.cube_corner);
    ui.add(egui::Slider::new(&mut config.cube_size, 40.0..=200.0).text("Taille"));
    ui.add(egui::Slider::new(&mut config.cube_margin, 0.0..=64.0).text("Marge"));

    ui.add_space(10.0);
    ui.heading("Grille");
    ui.add(
        egui::Slider::new(&mut config.grid_pixel_spacing, 12.0..=160.0).text("Espacement minimal"),
    );
    ui.checkbox(&mut config.grid_snap, "Aimantation sur la grille");
    ui.add(egui::Slider::new(&mut config.grid_snap_divisions, 1..=16).text("Subdivisions"));
    ui.add(egui::Slider::new(&mut config.grid_snap_pixels, 1.0..=48.0).text("Portée de l'aimant"));
    ui.add(
        egui::Slider::new(&mut config.segment_snap_pixels, 1.0..=48.0)
            .text("Portée des traits")
            .clamping(egui::SliderClamping::Never),
    )
    .on_hover_text("Un trait déjà dessiné attire plus fort que la grille");

    ui.add_space(10.0);
    ui.heading("Règle");
    ui.checkbox(&mut config.ruler_visible, "Afficher la barre d'échelle");
    corner_picker(ui, "Coin", &mut config.ruler_corner);
    ui.horizontal(|ui| {
        ui.label("Unité :");
        let mut automatic = matches!(config.unit, UnitDisplay::Auto);
        if ui.selectable_label(automatic, "Auto").clicked() {
            config.unit = UnitDisplay::Auto;
            automatic = true;
        }
        for unit in [
            LengthUnit::Micrometer,
            LengthUnit::Millimeter,
            LengthUnit::Centimeter,
            LengthUnit::Meter,
            LengthUnit::Kilometer,
        ] {
            let chosen = !automatic && config.unit == UnitDisplay::Fixed(unit);
            if ui.selectable_label(chosen, unit.suffix()).clicked() {
                config.unit = UnitDisplay::Fixed(unit);
            }
        }
    });

    ui.add_space(10.0);
    ui.heading("Caméra");
    ui.add(
        egui::Slider::new(&mut config.min_distance, 1e-4..=1.0)
            .logarithmic(true)
            .text("Distance minimale"),
    );
    ui.add(
        egui::Slider::new(&mut config.max_distance, 1e3..=1e10)
            .logarithmic(true)
            .text("Distance maximale"),
    );

    before != profiles.active().viewport
}

fn navigation_section(ui: &mut egui::Ui, profiles: &mut Profiles) -> bool {
    let before = profiles.active().viewport;
    let config = &mut profiles.active_mut().viewport;

    ui.heading("Souris");
    ui.horizontal_wrapped(|ui| {
        ui.label("Habitudes :");
        for preset in [
            NavigationPreset::Fusion360,
            NavigationPreset::SolidWorks,
            NavigationPreset::Blender,
        ] {
            let name = match preset {
                NavigationPreset::Fusion360 => "Fusion 360",
                NavigationPreset::SolidWorks => "SolidWorks",
                NavigationPreset::Blender => "Blender",
            };
            if ui
                .selectable_label(config.navigation == preset, name)
                .clicked()
            {
                config.navigation = preset;
            }
        }
    });
    ui.add(
        egui::Slider::new(&mut config.orbit_sensitivity, 0.001..=0.05).text("Vitesse de rotation"),
    );
    ui.add(
        egui::Slider::new(&mut config.wheel_zoom_sensitivity, 0.01..=1.0).text("Zoom à la molette"),
    );

    ui.add_space(10.0);
    ui.heading("Trackpad");
    gesture_picker(ui, "Défilement", &mut config.trackpad.scroll);
    gesture_picker(ui, "Maj + défilement", &mut config.trackpad.shift_scroll);
    ui.checkbox(&mut config.trackpad.pinch_zooms, "Le pincement zoome");
    ui.add(
        egui::Slider::new(&mut config.trackpad.scroll_sensitivity, 0.1..=4.0).text("Sensibilité"),
    );
    ui.add(
        egui::Slider::new(&mut config.zoom_sensitivity, 0.0001..=0.01).text("Zoom au défilement"),
    );

    before != profiles.active().viewport
}

fn appearance_section(ui: &mut egui::Ui, profiles: &mut Profiles) -> bool {
    let before = profiles.active().theme.clone();
    let theme = &mut profiles.active_mut().theme;

    ui.heading("Fond");
    background_editor(ui, &mut theme.background);

    ui.add_space(10.0);
    ui.heading("Axes et grille");
    color_row(ui, "Axe X", &mut theme.axis_x);
    color_row(ui, "Axe Y", &mut theme.axis_y);
    color_row(ui, "Axe Z", &mut theme.axis_z);
    ui.add(egui::Slider::new(&mut theme.axis_width, 0.5..=8.0).text("Épaisseur des axes"));
    color_row(ui, "Grille fine", &mut theme.grid_minor);
    color_row(ui, "Grille principale", &mut theme.grid_major);
    ui.add(egui::Slider::new(&mut theme.grid_minor_width, 0.5..=6.0).text("Épaisseur fine"));
    ui.add(egui::Slider::new(&mut theme.grid_major_width, 0.5..=6.0).text("Épaisseur principale"));
    ui.add(egui::Slider::new(&mut theme.grid_major_every, 2..=20).text("Une ligne sur"));

    ui.add_space(10.0);
    ui.heading("Esquisse");
    color_row(ui, "Libre", &mut theme.sketch_free);
    color_row(ui, "Contraint", &mut theme.sketch_settled);
    color_row(ui, "Autre esquisse", &mut theme.sketch_inactive);
    ui.add(egui::Slider::new(&mut theme.sketch_width, 0.5..=8.0).text("Épaisseur des traits"));
    color_row(ui, "Cote", &mut theme.dimension);
    color_row(ui, "Cote en lecture seule", &mut theme.dimension_driven);
    color_row(ui, "Élément fixé", &mut theme.fixed);
    color_row(ui, "Marque de contrainte", &mut theme.rule);
    color_row(ui, "Teinte des aires", &mut theme.region_fill);

    ui.add_space(10.0);
    ui.heading("Volume");
    color_row(ui, "Matière", &mut theme.solid);
    color_row(ui, "Survol", &mut theme.highlight);
    color_row(ui, "Ajout de matière", &mut theme.extrusion_add);
    color_row(ui, "Enlèvement", &mut theme.extrusion_cut);

    ui.add_space(10.0);
    if ui.button("↺ Couleurs par défaut").clicked() {
        *theme = Theme::default();
    }

    before != profiles.active().theme
}

/// The background editor: the kind of gradient, then its stops.
fn background_editor(ui: &mut egui::Ui, background: &mut Background) {
    ui.horizontal_wrapped(|ui| {
        let kinds = [
            ("Uni", 0usize),
            ("Dégradé linéaire", 1),
            ("Dégradé radial", 2),
        ];
        let current = match background {
            Background::Solid(_) => 0,
            Background::Linear { .. } => 1,
            Background::Radial { .. } => 2,
        };
        for (name, kind) in kinds {
            if ui.selectable_label(current == kind, name).clicked() && current != kind {
                // The colours already chosen are kept across a change of kind:
                // starting from black again every time would make trying the
                // three of them tedious.
                let stops = keep_stops(background);
                *background = match kind {
                    0 => Background::Solid(stops[0].color),
                    1 => Background::Linear {
                        angle_degrees: 0.0,
                        stops,
                    },
                    _ => Background::Radial {
                        center: [0.5, 0.5],
                        radius: 0.75,
                        stops,
                    },
                };
            }
        }
    });

    match background {
        Background::Solid(color) => color_row(ui, "Couleur", color),
        Background::Linear {
            angle_degrees,
            stops,
        } => {
            ui.add(egui::Slider::new(angle_degrees, 0.0..=360.0).text("Angle"));
            stops_editor(ui, stops);
        }
        Background::Radial {
            center,
            radius,
            stops,
        } => {
            ui.add(egui::Slider::new(&mut center[0], 0.0..=1.0).text("Centre X"));
            ui.add(egui::Slider::new(&mut center[1], 0.0..=1.0).text("Centre Y"));
            ui.add(egui::Slider::new(radius, 0.05..=2.0).text("Rayon"));
            stops_editor(ui, stops);
        }
    }
}

fn keep_stops(background: &Background) -> Vec<Stop> {
    match background {
        Background::Solid(color) => vec![Stop::new(0.0, *color), Stop::new(1.0, *color)],
        _ if background.stops().len() >= 2 => background.stops().to_vec(),
        _ => vec![
            Stop::new(0.0, Rgba::opaque(0.04, 0.05, 0.07)),
            Stop::new(1.0, Rgba::opaque(0.10, 0.12, 0.16)),
        ],
    }
}

fn stops_editor(ui: &mut egui::Ui, stops: &mut Vec<Stop>) {
    let mut remove = None;
    for (rank, stop) in stops.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            let mut color = to_egui_color(stop.color);
            if ui.color_edit_button_srgba(&mut color).changed() {
                stop.color = from_egui_color(color);
            }
            ui.add(egui::Slider::new(&mut stop.at, 0.0..=1.0).text("position"));
            if ui.button("✕").clicked() {
                remove = Some(rank);
            }
        });
    }
    if let Some(rank) = remove
        && stops.len() > 1
    {
        stops.remove(rank);
    }
    if ui.button("+ Ajouter une couleur").clicked() {
        let color = stops
            .last()
            .map(|stop| stop.color)
            .unwrap_or(Rgba::opaque(0.5, 0.5, 0.5));
        stops.push(Stop::new(1.0, color));
    }
}

fn shortcuts_section(
    ui: &mut egui::Ui,
    profiles: &mut Profiles,
    editor: &mut SettingsEditor,
) -> bool {
    let mut touched = false;
    ui.label("Cliquer un raccourci, puis appuyer sur la touche voulue. Les modificateurs tenus au moment de la frappe sont pris avec.");
    ui.add_space(6.0);

    // A key pressed while recording is read here, before any widget sees it.
    let pressed = editor.recording.and_then(|_| read_chord(ui));
    if let (Some(command), Some(chord)) = (editor.recording, pressed) {
        profiles.active_mut().shortcuts.bind(command, chord);
        editor.recording = None;
        touched = true;
    }

    let mut family = "";
    let shortcuts = profiles.active().shortcuts.clone();
    for command in Command::ALL {
        if command.family() != family {
            family = command.family();
            ui.add_space(8.0);
            ui.heading(family);
        }
        ui.horizontal(|ui| {
            ui.label(command.label());
            let recording = editor.recording == Some(command);
            let label = if recording {
                "… appuyez sur une touche".to_string()
            } else {
                shortcuts
                    .chord_for(command)
                    .map(crate::wording::shortcuts::chord)
                    .unwrap_or_else(|| "—".to_string())
            };
            if ui.selectable_label(recording, label).clicked() {
                editor.recording = if recording { None } else { Some(command) };
            }
            if ui.button("✕").clicked() {
                profiles.active_mut().shortcuts.unbind(command);
                touched = true;
            }
        });
    }

    ui.add_space(10.0);
    if ui.button("↺ Raccourcis par défaut").clicked() {
        profiles.active_mut().shortcuts = Default::default();
        touched = true;
    }
    touched
}

/// The first key pressed this frame, with the modifiers held with it.
fn read_chord(ui: &egui::Ui) -> Option<Chord> {
    ui.input(|input| {
        let modifiers = input.modifiers;
        input.events.iter().find_map(|event| match event {
            egui::Event::Key {
                key,
                pressed: true,
                repeat: false,
                ..
            } => from_egui_key(*key).map(|key| Chord {
                key,
                command: modifiers.command,
                shift: modifiers.shift,
                alt: modifiers.alt,
            }),
            _ => None,
        })
    })
}

fn toolbar_section(
    ui: &mut egui::Ui,
    profiles: &mut Profiles,
    editor: &mut SettingsEditor,
) -> bool {
    let before = profiles.active().toolbar.clone();
    let layout = &mut profiles.active_mut().toolbar;

    ui.horizontal_wrapped(|ui| {
        ui.label("Emplacement :");
        for edge in Edge::ALL {
            if ui
                .selectable_label(layout.edge == edge, crate::wording::toolbar::edge(edge))
                .clicked()
            {
                layout.edge = edge;
            }
        }
    });
    ui.horizontal(|ui| {
        ui.checkbox(&mut layout.show_logo, "Logo");
        ui.add(
            egui::TextEdit::singleline(&mut layout.logo_text)
                .desired_width(140.0)
                .hint_text("texte du logo"),
        );
        ui.checkbox(&mut layout.show_labels, "Noms des boutons");
    });

    ui.add_space(8.0);
    ui.label("Les groupes de premier niveau sont les onglets. Un groupe peut en contenir d'autres, sans limite.");
    ui.add_space(6.0);

    let selected = editor.selected.clone();
    ui.horizontal(|ui| {
        ui.add_enabled_ui(!selected.is_empty(), |ui| {
            if ui.button("↑").clicked()
                && let Some(moved) = layout.shift(&selected, false)
            {
                editor.selected = moved;
            }
            if ui.button("↓").clicked()
                && let Some(moved) = layout.shift(&selected, true)
            {
                editor.selected = moved;
            }
            if ui
                .button("→ Grouper")
                .on_hover_text("Entrer dans le groupe juste au-dessus")
                .clicked()
                && let Some(moved) = layout.nest(&selected)
            {
                editor.selected = moved;
            }
            if ui.button("← Sortir").clicked()
                && let Some(moved) = layout.unnest(&selected)
            {
                editor.selected = moved;
            }
            if ui.button("✕ Retirer").clicked() {
                layout.remove(&selected);
                editor.selected.clear();
            }
        });
    });

    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut editor.new_group_name)
                .desired_width(140.0)
                .hint_text("nom du groupe"),
        );
        if ui.button("+ Groupe").clicked() {
            let name = if editor.new_group_name.trim().is_empty() {
                "Groupe".to_string()
            } else {
                editor.new_group_name.trim().to_string()
            };
            layout.items.push(Item::group(&name, Vec::new()));
            editor.new_group_name.clear();
        }
        if ui.button("+ Séparateur").clicked() {
            layout.items.push(Item::Separator);
        }
        if ui.button("Renommer").clicked() && !editor.new_group_name.trim().is_empty() {
            layout.rename(&selected, editor.new_group_name.trim().to_string());
            editor.new_group_name.clear();
        }
        if ui.button("↺ Barre par défaut").clicked() {
            *layout = Default::default();
            editor.selected.clear();
        }
    });

    ui.separator();
    egui::ScrollArea::vertical()
        .max_height(220.0)
        .id_salt("toolbar_tree")
        .show(ui, |ui| {
            let items = layout.items.clone();
            tree(ui, &items, &mut Vec::new(), &mut editor.selected);
        });

    ui.separator();
    ui.label("Ajouter une commande dans la sélection (ou à la racine) :");
    egui::ScrollArea::vertical()
        .max_height(160.0)
        .id_salt("command_palette")
        .show(ui, |ui| {
            let mut family = "";
            for command in Command::ALL {
                if command.family() != family {
                    family = command.family();
                    ui.add_space(6.0);
                    ui.weak(family);
                }
                if ui.button(command.label()).clicked() {
                    let into = group_path(layout, &editor.selected);
                    layout.push_into(&into, Item::Command(command));
                }
            }
        });

    before != profiles.active().toolbar
}

/// Where a new entry goes: into the selection when it is a group, otherwise
/// into the group holding it, otherwise at the root.
fn group_path(layout: &cao_prefs::ToolbarLayout, selected: &[usize]) -> Path {
    if matches!(layout.at(selected), Some(Item::Group { .. })) {
        return selected.to_vec();
    }
    let mut parent = selected.to_vec();
    parent.pop();
    parent
}

fn tree(ui: &mut egui::Ui, items: &[Item], path: &mut Path, selected: &mut Path) {
    for (rank, item) in items.iter().enumerate() {
        path.push(rank);
        ui.horizontal(|ui| {
            ui.add_space(12.0 * (path.len() - 1) as f32);
            let name = crate::wording::toolbar::item(item);
            if ui.selectable_label(*path == *selected, name).clicked() {
                *selected = path.clone();
            }
        });
        if let Item::Group { items, .. } = item {
            tree(ui, items, path, selected);
        }
        path.pop();
    }
}

fn corner_picker(ui: &mut egui::Ui, label: &str, corner: &mut ViewportCorner) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        for (name, value) in [
            ("↖", ViewportCorner::TopLeft),
            ("↗", ViewportCorner::TopRight),
            ("↙", ViewportCorner::BottomLeft),
            ("↘", ViewportCorner::BottomRight),
        ] {
            if ui.selectable_label(*corner == value, name).clicked() {
                *corner = value;
            }
        }
    });
}

fn gesture_picker(ui: &mut egui::Ui, label: &str, gesture: &mut TrackpadGesture) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        for (name, value) in [
            ("Déplacer", TrackpadGesture::Pan),
            ("Orbiter", TrackpadGesture::Orbit),
            ("Zoomer", TrackpadGesture::Zoom),
            ("Rien", TrackpadGesture::Ignore),
        ] {
            if ui.selectable_label(*gesture == value, name).clicked() {
                *gesture = value;
            }
        }
    });
}

fn color_row(ui: &mut egui::Ui, label: &str, color: &mut Rgba) {
    ui.horizontal(|ui| {
        let mut value = to_egui_color(*color);
        if ui.color_edit_button_srgba(&mut value).changed() {
            *color = from_egui_color(value);
        }
        ui.label(label);
    });
}

fn to_egui_color(color: Rgba) -> egui::Color32 {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    egui::Color32::from_rgba_unmultiplied(
        channel(color.r),
        channel(color.g),
        channel(color.b),
        channel(color.a),
    )
}

fn from_egui_color(color: egui::Color32) -> Rgba {
    let [r, g, b, a] = color.to_srgba_unmultiplied();
    Rgba::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
}

fn from_egui_key(key: egui::Key) -> Option<Key> {
    use egui::Key as E;
    Some(match key {
        E::A => Key::A,
        E::B => Key::B,
        E::C => Key::C,
        E::D => Key::D,
        E::E => Key::E,
        E::F => Key::F,
        E::G => Key::G,
        E::H => Key::H,
        E::I => Key::I,
        E::J => Key::J,
        E::K => Key::K,
        E::L => Key::L,
        E::M => Key::M,
        E::N => Key::N,
        E::O => Key::O,
        E::P => Key::P,
        E::Q => Key::Q,
        E::R => Key::R,
        E::S => Key::S,
        E::T => Key::T,
        E::U => Key::U,
        E::V => Key::V,
        E::W => Key::W,
        E::X => Key::X,
        E::Y => Key::Y,
        E::Z => Key::Z,
        E::Num0 => Key::Num0,
        E::Num1 => Key::Num1,
        E::Num2 => Key::Num2,
        E::Num3 => Key::Num3,
        E::Num4 => Key::Num4,
        E::Num5 => Key::Num5,
        E::Num6 => Key::Num6,
        E::Num7 => Key::Num7,
        E::Num8 => Key::Num8,
        E::Num9 => Key::Num9,
        E::F1 => Key::F1,
        E::F2 => Key::F2,
        E::F3 => Key::F3,
        E::F4 => Key::F4,
        E::F5 => Key::F5,
        E::F6 => Key::F6,
        E::F7 => Key::F7,
        E::F8 => Key::F8,
        E::F9 => Key::F9,
        E::F10 => Key::F10,
        E::F11 => Key::F11,
        E::F12 => Key::F12,
        E::Escape => Key::Escape,
        E::Tab => Key::Tab,
        E::Space => Key::Space,
        E::Enter => Key::Enter,
        E::Backspace => Key::Backspace,
        E::Delete => Key::Delete,
        E::Home => Key::Home,
        E::End => Key::End,
        E::PageUp => Key::PageUp,
        E::PageDown => Key::PageDown,
        E::ArrowLeft => Key::Left,
        E::ArrowRight => Key::Right,
        E::ArrowUp => Key::Up,
        E::ArrowDown => Key::Down,
        E::Plus => Key::Plus,
        E::Minus => Key::Minus,
        E::Comma => Key::Comma,
        E::Period => Key::Period,
        _ => return None,
    })
}
