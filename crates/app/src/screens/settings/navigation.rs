use cao_prefs::Profiles;
use cao_prefs::config::{NavigationPreset, TrackpadGesture};

use crate::lang::Catalogue;
use crate::ui::slider::slider;

pub(super) fn section(ui: &mut egui::Ui, profiles: &mut Profiles, lang: &Catalogue) -> bool {
    let before = profiles.active().viewport;
    let config = &mut profiles.active_mut().viewport;

    ui.heading(lang.t("settings.navigation.mouse"));
    ui.horizontal_wrapped(|ui| {
        ui.label(lang.t("settings.navigation.preset"));
        for preset in [
            NavigationPreset::Fusion360,
            NavigationPreset::SolidWorks,
            NavigationPreset::Blender,
        ] {
            let name = lang.t(match preset {
                NavigationPreset::Fusion360 => "settings.navigation.preset.fusion360",
                NavigationPreset::SolidWorks => "settings.navigation.preset.solidworks",
                NavigationPreset::Blender => "settings.navigation.preset.blender",
            });
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
        &lang.t("settings.navigation.orbit_speed"),
    );
    slider(
        ui,
        &mut config.wheel_zoom_sensitivity,
        0.01..=1.0,
        &lang.t("settings.navigation.wheel_zoom"),
    );

    ui.add_space(10.0);
    ui.heading(lang.t("settings.navigation.trackpad"));
    gesture_picker(
        ui,
        &lang.t("settings.navigation.scroll"),
        &mut config.trackpad.scroll,
        lang,
    );
    gesture_picker(
        ui,
        &lang.t("settings.navigation.shift_scroll"),
        &mut config.trackpad.shift_scroll,
        lang,
    );
    ui.checkbox(
        &mut config.trackpad.pinch_zooms,
        lang.t("settings.navigation.pinch_zooms"),
    );
    slider(
        ui,
        &mut config.trackpad.scroll_sensitivity,
        0.1..=4.0,
        &lang.t("settings.navigation.scroll_sensitivity"),
    );
    slider(
        ui,
        &mut config.zoom_sensitivity,
        0.0001..=0.01,
        &lang.t("settings.navigation.scroll_zoom"),
    );

    before != profiles.active().viewport
}

fn gesture_picker(ui: &mut egui::Ui, label: &str, gesture: &mut TrackpadGesture, lang: &Catalogue) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        for (key, value) in [
            ("settings.navigation.gesture.pan", TrackpadGesture::Pan),
            ("settings.navigation.gesture.orbit", TrackpadGesture::Orbit),
            ("settings.navigation.gesture.zoom", TrackpadGesture::Zoom),
            (
                "settings.navigation.gesture.ignore",
                TrackpadGesture::Ignore,
            ),
        ] {
            if ui
                .selectable_label(*gesture == value, lang.t(key))
                .clicked()
            {
                *gesture = value;
            }
        }
    });
}
