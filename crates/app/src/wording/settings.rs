use cao_prefs::config::ALL_FIGURES;
use cao_prefs::settings::DEFAULT_PROFILE;

use crate::lang::Catalogue;

/// The only place a profile is turned into a name.
///
/// The profile that always exists is keyed, because `settings.json` holds that
/// key and the code compares against it to refuse deleting it. Every other
/// profile is named by the user and is handed back untouched.
pub fn profile(lang: &Catalogue, name: &str) -> String {
    match name {
        DEFAULT_PROFILE => lang.t("settings.profile.default"),
        theirs => theirs.to_string(),
    }
}

/// What a notch of the measure's figures slider says: how many figures, and at
/// the top every figure a tenth of a micron needs, which is no one count.
pub fn measure_figures(lang: &Catalogue, figures: u32) -> String {
    match figures >= ALL_FIGURES {
        true => lang.t("settings.viewport.measure_figures_all"),
        false => figures.to_string(),
    }
}

#[cfg(test)]
mod tests;
