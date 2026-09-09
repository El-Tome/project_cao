use cao_prefs::{Background, Profiles, Rgba, Stop, Theme};

use crate::ui::slider::slider;

pub(super) fn section(ui: &mut egui::Ui, profiles: &mut Profiles) -> bool {
    let before = profiles.active().theme.clone();
    let theme = &mut profiles.active_mut().theme;

    ui.heading("Fond");
    background_editor(ui, &mut theme.background);

    ui.add_space(10.0);
    ui.heading("Axes et grille");
    color_row(ui, "Axe X", &mut theme.axis_x);
    color_row(ui, "Axe Y", &mut theme.axis_y);
    color_row(ui, "Axe Z", &mut theme.axis_z);
    slider(ui, &mut theme.axis_width, 0.5..=8.0, "Épaisseur des axes");
    color_row(ui, "Grille fine", &mut theme.grid_minor);
    color_row(ui, "Grille principale", &mut theme.grid_major);
    slider(ui, &mut theme.grid_minor_width, 0.5..=6.0, "Épaisseur fine");
    slider(
        ui,
        &mut theme.grid_major_width,
        0.5..=6.0,
        "Épaisseur principale",
    );
    slider(ui, &mut theme.grid_major_every, 2..=20, "Une ligne sur");

    ui.add_space(10.0);
    ui.heading("Esquisse");
    color_row(ui, "Libre", &mut theme.sketch_free);
    color_row(ui, "Contraint", &mut theme.sketch_settled);
    color_row(ui, "Autre esquisse", &mut theme.sketch_inactive);
    slider(
        ui,
        &mut theme.sketch_width,
        0.5..=8.0,
        "Épaisseur des traits",
    );
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
            slider(ui, angle_degrees, 0.0..=360.0, "Angle");
            stops_editor(ui, stops);
        }
        Background::Radial {
            center,
            radius,
            stops,
        } => {
            slider(ui, &mut center[0], 0.0..=1.0, "Centre X");
            slider(ui, &mut center[1], 0.0..=1.0, "Centre Y");
            slider(ui, radius, 0.05..=2.0, "Rayon");
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
            slider(ui, &mut stop.at, 0.0..=1.0, "position");
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
