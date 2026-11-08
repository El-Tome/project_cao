//! What the title bar says of where the view stands, in its pill at the end of
//! the breadcrumb.

use cao_sketch::WorkPlane;

use crate::lang::Catalogue;
use crate::wording::plane;

/// What the view is showing, in the facts it is named by. Ranks count from
/// one, the way the Part tree words a sketch and a section.
#[derive(Clone, Copy, Debug)]
pub struct Standing {
    pub open_sketch_rank: Option<usize>,
    pub section_view_on: bool,
    pub kept_section_rank: Option<usize>,
    pub plane: Option<WorkPlane>,
}

/// The sketch open, else the section view, else the kept section shown, else
/// the plane the view stands on, else the free view.
pub fn standing_on(lang: &Catalogue, standing: Standing) -> String {
    let numbered = |key: &str, rank: usize| lang.t_with(key, &[("rank", &rank.to_string())]);
    match standing {
        Standing {
            open_sketch_rank: Some(rank),
            ..
        } => numbered("part_tree.sketch", rank),
        Standing {
            section_view_on: true,
            ..
        } => lang.t("command.label.section_view"),
        Standing {
            kept_section_rank: Some(rank),
            ..
        } => numbered("part_tree.section", rank),
        Standing {
            plane: Some(plane), ..
        } => plane::label(lang, plane.kind()),
        Standing { plane: None, .. } => lang.t("app.free_view"),
    }
}

#[cfg(test)]
mod tests;
