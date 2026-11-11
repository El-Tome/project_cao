//! What an alert of the part says, decided by its problem.

use crate::lang::Catalogue;
use crate::wording::part_tree::{Problem, badge};

/// « Extrusion 2 · non construite »: the element, then its problem in the word
/// its badge in the tree wears — save a volume not built, whose word here
/// agrees with « Extrusion » and « Révolution » where the badge's stands alone.
pub fn title(lang: &Catalogue, element: &str, problem: Problem) -> String {
    let problem = match problem {
        Problem::NotBuilt => lang.t("alerts.not_built"),
        other => badge(lang, other),
    };
    lang.t_with(
        "alerts.title",
        &[("element", element), ("problem", &problem)],
    )
}

/// Why, in a sentence: what the pill said of a step the kernel declined, or of
/// one it built unstably — `unstable`, which says what the kernel read — and
/// a sentence of its own for an area or a face lost.
pub fn why(lang: &Catalogue, problem: Problem, unstable: Option<&str>) -> String {
    match problem {
        Problem::Unstable => unstable.unwrap_or_default().to_owned(),
        Problem::NotBuilt => lang.t("extrusion.declined"),
        Problem::AreaLost => lang.t("alerts.area_lost"),
        Problem::FaceLost => lang.t("alerts.face_lost"),
    }
}
