use cao_prefs::Profiles;
use cao_prefs::config::{LengthUnit, UnitDisplay, ViewportCorner};

use crate::ui::slider::slider;

pub(super) fn section(ui: &mut egui::Ui, profiles: &mut Profiles) -> bool {
    let before = profiles.active().viewport;
    let config = &mut profiles.active_mut().viewport;

    ui.heading("Cube d'orientation");
    corner_picker(ui, "Coin", &mut config.cube_corner);
    slider(ui, &mut config.cube_size, 40.0..=200.0, "Taille");
    slider(ui, &mut config.cube_margin, 0.0..=64.0, "Marge");

    ui.add_space(10.0);
    ui.heading("Grille");
    slider(
        ui,
        &mut config.grid_pixel_spacing,
        12.0..=160.0,
        "Espacement minimal",
    );
    ui.checkbox(&mut config.grid_snap, "Aimantation sur la grille");
    slider(ui, &mut config.grid_snap_divisions, 1..=16, "Subdivisions");
    slider(
        ui,
        &mut config.grid_snap_pixels,
        1.0..=48.0,
        "Portée de l'aimant",
    );
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
