use cao_prefs::Profiles;
use cao_prefs::config::{LengthUnit, UnitDisplay, ViewportCorner};

use crate::lang::Catalogue;
use crate::ui::slider::slider;

pub(super) fn section(ui: &mut egui::Ui, profiles: &mut Profiles, lang: &Catalogue) -> bool {
    let before = profiles.active().viewport;
    let config = &mut profiles.active_mut().viewport;

    ui.heading(lang.t("settings.viewport.cube"));
    corner_picker(
        ui,
        &lang.t("settings.viewport.corner"),
        &mut config.cube_corner,
    );
    slider(
        ui,
        &mut config.cube_size,
        40.0..=200.0,
        &lang.t("settings.viewport.cube_size"),
    );
    slider(
        ui,
        &mut config.cube_margin,
        0.0..=64.0,
        &lang.t("settings.viewport.cube_margin"),
    );

    ui.add_space(10.0);
    ui.heading(lang.t("settings.viewport.grid"));
    slider(
        ui,
        &mut config.grid_pixel_spacing,
        12.0..=160.0,
        &lang.t("settings.viewport.grid_spacing"),
    );
    ui.checkbox(&mut config.grid_snap, lang.t("settings.viewport.grid_snap"));
    slider(
        ui,
        &mut config.grid_snap_divisions,
        1..=16,
        &lang.t("settings.viewport.grid_divisions"),
    );
    slider(
        ui,
        &mut config.grid_snap_pixels,
        1.0..=48.0,
        &lang.t("settings.viewport.grid_snap_reach"),
    );
    ui.add(
        egui::Slider::new(&mut config.segment_snap_pixels, 1.0..=48.0)
            .text(lang.t("settings.viewport.segment_snap_reach"))
            .clamping(egui::SliderClamping::Never),
    )
    .on_hover_text(lang.t("settings.viewport.segment_snap_hint"));

    ui.add_space(10.0);
    ui.heading(lang.t("settings.viewport.ruler"));
    ui.checkbox(
        &mut config.ruler_visible,
        lang.t("settings.viewport.ruler_visible"),
    );
    corner_picker(
        ui,
        &lang.t("settings.viewport.corner"),
        &mut config.ruler_corner,
    );
    ui.horizontal(|ui| {
        ui.label(lang.t("settings.viewport.unit"));
        let mut automatic = matches!(config.unit, UnitDisplay::Auto);
        if ui
            .selectable_label(automatic, lang.t("settings.viewport.unit_auto"))
            .clicked()
        {
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
    ui.heading(lang.t("settings.viewport.camera"));
    ui.add(
        egui::Slider::new(&mut config.min_distance, 1e-4..=1.0)
            .logarithmic(true)
            .text(lang.t("settings.viewport.min_distance")),
    );
    ui.add(
        egui::Slider::new(&mut config.max_distance, 1e3..=1e10)
            .logarithmic(true)
            .text(lang.t("settings.viewport.max_distance")),
    );

    before != profiles.active().viewport
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
