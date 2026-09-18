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

#[cfg(test)]
mod tests;
