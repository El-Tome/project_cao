//! How a size written from the part's variables is said, and what is wrong
//! with a formula that does not read.

use cao_part::{Formula, Unreadable, Unusable, Variables};

use crate::lang::Catalogue;

/// A size as a line of the interface says it: the number it is, or — when it
/// was written from variables — what it was written as and what that comes
/// to now. `say` writes the number, to whatever precision the line wants.
pub fn sized(
    lang: &Catalogue,
    variables: &Variables,
    formula: &Formula,
    say: impl Fn(f64) -> String,
) -> String {
    if let Some(number) = formula.as_number() {
        return say(number);
    }
    let value = formula
        .value(&variables.values())
        .map(&say)
        .unwrap_or_else(|| lang.t("formula.no_number"));
    lang.t_with(
        "formula.shown",
        &[("formula", &variables.written(formula)), ("value", &value)],
    )
}

/// What is wrong with a formula that does not read.
pub fn unreadable(lang: &Catalogue, wrong: &Unreadable) -> String {
    match wrong {
        Unreadable::Empty => lang.t("formula.empty"),
        Unreadable::UnknownName(name) => lang.t_with("formula.unknown_name", &[("name", name)]),
        Unreadable::StrayCharacter(character) => lang.t_with(
            "formula.stray_character",
            &[("character", &character.to_string())],
        ),
        Unreadable::MissingValue(Some(found)) => {
            lang.t_with("formula.missing_value", &[("found", &found.to_string())])
        }
        Unreadable::MissingValue(None) => lang.t("formula.missing_last_value"),
        Unreadable::MissingOperator => lang.t("formula.missing_operator"),
        Unreadable::Unclosed => lang.t("formula.unclosed"),
        Unreadable::Unopened => lang.t("formula.unopened"),
    }
}

/// What is wrong with a size typed as text that cannot be used.
pub fn unusable(lang: &Catalogue, wrong: &Unusable) -> String {
    match wrong {
        Unusable::Unreadable(wrong) => unreadable(lang, wrong),
        Unusable::NoNumber => lang.t("formula.no_value"),
    }
}

/// Said of a count that comes to anything but a whole number above zero.
pub fn not_a_count(lang: &Catalogue) -> String {
    lang.t("formula.not_a_count")
}

#[cfg(test)]
mod tests;
