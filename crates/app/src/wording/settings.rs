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
mod tests {
    use super::*;

    #[test]
    fn the_profile_that_always_exists_reads_as_a_name_and_not_as_its_key() {
        assert_ne!(
            profile(&Catalogue::french(), DEFAULT_PROFILE),
            DEFAULT_PROFILE,
            "the settings screen offers the key it is stored under",
        );
    }

    /// A profile the user made carries their own words: the interface has
    /// nothing to say about it.
    #[test]
    fn a_profile_the_user_named_reads_as_they_named_it() {
        assert_eq!(profile(&Catalogue::french(), "Atelier"), "Atelier");
    }
}
