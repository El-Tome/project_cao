use cao_prefs::Profiles;
use cao_prefs::config::{NavigationPreset, TrackpadGesture};

use crate::ui::slider::slider;

pub(super) fn section(ui: &mut egui::Ui, profiles: &mut Profiles) -> bool {
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
    slider(
        ui,
        &mut config.orbit_sensitivity,
        0.001..=0.05,
        "Vitesse de rotation",
    );
    slider(
        ui,
        &mut config.wheel_zoom_sensitivity,
        0.01..=1.0,
        "Zoom à la molette",
    );

    ui.add_space(10.0);
    ui.heading("Trackpad");
    gesture_picker(ui, "Défilement", &mut config.trackpad.scroll);
    gesture_picker(ui, "Maj + défilement", &mut config.trackpad.shift_scroll);
    ui.checkbox(&mut config.trackpad.pinch_zooms, "Le pincement zoome");
    slider(
        ui,
        &mut config.trackpad.scroll_sensitivity,
        0.1..=4.0,
        "Sensibilité",
    );
    slider(
        ui,
        &mut config.zoom_sensitivity,
        0.0001..=0.01,
        "Zoom au défilement",
    );

    before != profiles.active().viewport
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
