//! What the part says when it refuses a change to its variables: the name
//! that does not do, the loop, what still uses a variable, what would break.

use cao_part::{Broken, NameProblem, PartDocument, Refused, Use, VariableId};
use cao_sketch::DimensionTarget;

use crate::lang::Catalogue;
use crate::wording::{dimension, history};

/// Why the part refused a change to its variables.
pub fn refused(lang: &Catalogue, document: &PartDocument, refused: &Refused) -> String {
    match refused {
        Refused::Name(problem) => name(lang, *problem),
        Refused::Loop(round) => lang.t_with(
            "variables.loop",
            &[(
                "names",
                &listed(round.iter().map(|variable| named(document, *variable))),
            )],
        ),
        Refused::InUse(uses) => lang.t_with(
            "variables.in_use",
            &[(
                "uses",
                &listed(uses.iter().map(|used| use_of(lang, document, used))),
            )],
        ),
        Refused::Breaks(broken) => lang.t_with(
            "variables.breaks",
            &[(
                "sizes",
                &listed(broken.iter().map(|size| breaking(lang, document, size))),
            )],
        ),
        Refused::Gone => lang.t("variables.gone"),
    }
}

/// Why a variable may not go by a name.
pub fn name(lang: &Catalogue, problem: NameProblem) -> String {
    match problem {
        NameProblem::Empty => lang.t("variables.name_empty"),
        NameProblem::OpensOnADigit => lang.t("variables.name_opens_on_a_digit"),
        NameProblem::NotAllowed(character) => lang.t_with(
            "variables.name_not_allowed",
            &[("character", &character.to_string())],
        ),
        NameProblem::Taken => lang.t("variables.name_taken"),
    }
}

fn use_of(lang: &Catalogue, document: &PartDocument, used: &Use) -> String {
    match used {
        Use::Variable(variable) => lang.t_with(
            "variables.use_variable",
            &[("name", &named(document, *variable))],
        ),
        Use::Dimension { sketch, target } => value_on(lang, document, *sketch, target),
        Use::Operation(number) => step(lang, document, *number),
    }
}

fn breaking(lang: &Catalogue, document: &PartDocument, broken: &Broken) -> String {
    match broken {
        Broken::Variable(variable) => lang.t_with(
            "variables.use_variable",
            &[("name", &named(document, *variable))],
        ),
        Broken::Dimension { sketch, target } => value_on(lang, document, *sketch, target),
        Broken::Operation(number) => step(lang, document, *number),
        Broken::Renumbered {
            changed,
            followed_by,
        } => lang.t_with(
            "variables.renumbered",
            &[
                ("changed", &step(lang, document, *changed)),
                ("followed_by", &step(lang, document, *followed_by)),
            ],
        ),
        Broken::Adrift(sketch) => {
            lang.t_with("variables.adrift", &[("sketch", &(sketch + 1).to_string())])
        }
    }
}

fn named(document: &PartDocument, variable: VariableId) -> String {
    document
        .variables()
        .get(variable)
        .map(|found| found.name().to_string())
        .unwrap_or_default()
}

/// A value on a drawing, by what it measures now and the sketch it is on,
/// counted from one the way the panels count sketches.
fn value_on(
    lang: &Catalogue,
    document: &PartDocument,
    sketch: usize,
    target: &DimensionTarget,
) -> String {
    let measured = document
        .measured(sketch, *target)
        .map(dimension::short)
        .unwrap_or_default();
    lang.t_with(
        "variables.use_dimension",
        &[
            ("dimension", &dimension::label(lang, target, &measured)),
            ("sketch", &(sketch + 1).to_string()),
        ],
    )
}

/// A step of the history, by the place the history panel shows it at.
fn step(lang: &Catalogue, document: &PartDocument, number: u32) -> String {
    let Some(position) = document.history.position_of(number) else {
        return String::new();
    };
    let label = document
        .history
        .operations()
        .get(position)
        .map(|operation| history::label(lang, document.variables(), operation))
        .unwrap_or_default();
    lang.t_with(
        "variables.use_step",
        &[("position", &(position + 1).to_string()), ("label", &label)],
    )
}

fn listed(said: impl Iterator<Item = String>) -> String {
    said.collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
mod tests;
