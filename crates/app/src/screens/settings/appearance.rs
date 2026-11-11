use cao_prefs::{Background, Profiles, Rgba, Stop, Theme};

use crate::lang::Catalogue;
use crate::screens::theme_colours::{self, painted};
use crate::ui::badge::Level;
use crate::ui::slider::slider;

pub(super) fn section(ui: &mut egui::Ui, profiles: &mut Profiles, lang: &Catalogue) -> bool {
    let before = profiles.active().theme.clone();
    let theme = &mut profiles.active_mut().theme;

    ui.heading(lang.t("settings.appearance.background"));
    background_editor(ui, &mut theme.background, lang);

    ui.add_space(10.0);
    ui.heading(lang.t("settings.appearance.sketch_background"));
    background_editor(ui, &mut theme.sketch_background, lang);

    ui.add_space(10.0);
    ui.heading(lang.t("settings.appearance.axes_and_grid"));
    color_row(ui, &lang.t("settings.appearance.axis_x"), &mut theme.axis_x);
    color_row(ui, &lang.t("settings.appearance.axis_y"), &mut theme.axis_y);
    color_row(ui, &lang.t("settings.appearance.axis_z"), &mut theme.axis_z);
    slider(
        ui,
        &mut theme.axis_width,
        0.5..=8.0,
        &lang.t("settings.appearance.axis_width"),
    );
    color_row(
        ui,
        &lang.t("settings.appearance.grid_minor"),
        &mut theme.grid_minor,
    );
    color_row(
        ui,
        &lang.t("settings.appearance.grid_major"),
        &mut theme.grid_major,
    );
    slider(
        ui,
        &mut theme.grid_minor_width,
        0.5..=6.0,
        &lang.t("settings.appearance.grid_minor_width"),
    );
    slider(
        ui,
        &mut theme.grid_major_width,
        0.5..=6.0,
        &lang.t("settings.appearance.grid_major_width"),
    );
    slider(
        ui,
        &mut theme.grid_major_every,
        2..=20,
        &lang.t("settings.appearance.grid_major_every"),
    );

    ui.add_space(10.0);
    ui.heading(lang.t("settings.appearance.sketch"));
    color_row(
        ui,
        &lang.t("settings.appearance.sketch_free"),
        &mut theme.sketch_free,
    );
    color_row(
        ui,
        &lang.t("settings.appearance.sketch_settled"),
        &mut theme.sketch_settled,
    );
    color_row(
        ui,
        &lang.t("settings.appearance.sketch_inactive"),
        &mut theme.sketch_inactive,
    );
    slider(
        ui,
        &mut theme.sketch_width,
        0.5..=8.0,
        &lang.t("settings.appearance.sketch_width"),
    );
    color_row(
        ui,
        &lang.t("settings.appearance.dimension"),
        &mut theme.dimension,
    );
    color_row(
        ui,
        &lang.t("settings.appearance.dimension_driven"),
        &mut theme.dimension_driven,
    );
    color_row(ui, &lang.t("settings.appearance.fixed"), &mut theme.fixed);
    color_row(ui, &lang.t("settings.appearance.rule"), &mut theme.rule);
    color_row(ui, &lang.t("settings.appearance.picked"), &mut theme.picked);
    color_row(ui, &lang.t("settings.appearance.going"), &mut theme.going);
    color_row(
        ui,
        &lang.t("settings.appearance.measure"),
        &mut theme.measure,
    );
    color_row(
        ui,
        &lang.t("settings.appearance.region_fill"),
        &mut theme.region_fill,
    );

    ui.add_space(10.0);
    ui.heading(lang.t("settings.appearance.volume"));
    color_row(ui, &lang.t("settings.appearance.solid"), &mut theme.solid);
    color_row(
        ui,
        &lang.t("settings.appearance.highlight"),
        &mut theme.highlight,
    );
    color_row(
        ui,
        &lang.t("settings.appearance.refused"),
        &mut theme.refused,
    );
    color_row(
        ui,
        &lang.t("settings.appearance.extrusion_add"),
        &mut theme.extrusion_add,
    );
    color_row(
        ui,
        &lang.t("settings.appearance.extrusion_cut"),
        &mut theme.extrusion_cut,
    );

    ui.add_space(10.0);
    ui.heading(lang.t("settings.appearance.alerts"));
    let warning = lang.t("settings.appearance.warning");
    level_row(ui, (&warning, Level::Warning), theme);
    let error = lang.t("settings.appearance.error");
    level_row(ui, (&error, Level::Error), theme);

    ui.add_space(10.0);
    if ui.button(lang.t("settings.appearance.reset")).clicked() {
        *theme = Theme::default();
    }

    ui.add_space(10.0);
    ui.heading(lang.t("settings.appearance.window"));
    let reopening_toggled = ui
        .checkbox(
            &mut profiles.active_mut().reopen_at_last_size,
            lang.t("settings.appearance.reopen_at_last_size"),
        )
        .changed();

    reopening_toggled || before != profiles.active().theme
}

/// The background editor: the kind of gradient, then its stops.
fn background_editor(ui: &mut egui::Ui, background: &mut Background, lang: &Catalogue) {
    ui.horizontal_wrapped(|ui| {
        let kinds = [
            ("settings.appearance.kind.solid", 0usize),
            ("settings.appearance.kind.linear", 1),
            ("settings.appearance.kind.radial", 2),
        ];
        let current = match background {
            Background::Solid(_) => 0,
            Background::Linear { .. } => 1,
            Background::Radial { .. } => 2,
        };
        for (name, kind) in kinds {
            if ui.selectable_label(current == kind, lang.t(name)).clicked() && current != kind {
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
        Background::Solid(color) => color_row(ui, &lang.t("settings.appearance.color"), color),
        Background::Linear {
            angle_degrees,
            stops,
        } => {
            slider(
                ui,
                angle_degrees,
                0.0..=360.0,
                &lang.t("settings.appearance.angle"),
            );
            stops_editor(ui, stops, lang);
        }
        Background::Radial {
            center,
            radius,
            stops,
        } => {
            slider(
                ui,
                &mut center[0],
                0.0..=1.0,
                &lang.t("settings.appearance.center_x"),
            );
            slider(
                ui,
                &mut center[1],
                0.0..=1.0,
                &lang.t("settings.appearance.center_y"),
            );
            slider(
                ui,
                radius,
                0.05..=2.0,
                &lang.t("settings.appearance.radius"),
            );
            stops_editor(ui, stops, lang);
        }
    }
}

fn keep_stops(background: &Background) -> Vec<Stop> {
    match background {
        Background::Solid(color) => vec![Stop::new(0.0, *color), Stop::new(1.0, *color)],
        _ if background.stops().len() >= 2 => background.stops().to_vec(),
        _ => Theme::default().background.stops().to_vec(),
    }
}

fn stops_editor(ui: &mut egui::Ui, stops: &mut Vec<Stop>, lang: &Catalogue) {
    let mut remove = None;
    for (rank, stop) in stops.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            let mut color = painted(stop.color);
            if ui.color_edit_button_srgba(&mut color).changed() {
                stop.color = from_egui_color(color);
            }
            slider(
                ui,
                &mut stop.at,
                0.0..=1.0,
                &lang.t("settings.appearance.stop_position"),
            );
            if ui
                .button(lang.t("settings.appearance.remove_color"))
                .clicked()
            {
                remove = Some(rank);
            }
        });
    }
    if let Some(rank) = remove
        && stops.len() > 1
    {
        stops.remove(rank);
    }
    if ui.button(lang.t("settings.appearance.add_color")).clicked() {
        let color = stops
            .last()
            .map(|stop| stop.color)
            .unwrap_or(Rgba::opaque(0.5, 0.5, 0.5));
        stops.push(Stop::new(1.0, color));
    }
}

fn color_row(ui: &mut egui::Ui, label: &str, color: &mut Rgba) {
    ui.horizontal(|ui| {
        let mut value = painted(*color);
        if ui.color_edit_button_srgba(&mut value).changed() {
            *color = from_egui_color(value);
        }
        ui.label(label);
    });
}

/// The colour a level of the part's alerts is seen in: the one chosen, or
/// the chrome's while none is. Picking one chooses it.
fn level_row(ui: &mut egui::Ui, (label, level): (&str, Level), theme: &mut Theme) {
    ui.horizontal(|ui| {
        let (_, mut value) = theme_colours::worn(theme, level);
        if ui.color_edit_button_srgba(&mut value).changed() {
            theme_colours::choose(theme, level, from_egui_color(value));
        }
        ui.label(label);
    });
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
