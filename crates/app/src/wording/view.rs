//! What the title bar says of where the view stands, in its pill at the end of
//! the breadcrumb.

use cao_sketch::WorkPlane;

use crate::lang::Catalogue;
use crate::wording::plane;

/// What the view is showing, in the facts it is named by, the way the Part
/// tree words a sketch and a section: a sketch by the number the part gave
/// it, a section by its rank from one.
#[derive(Clone, Copy, Debug)]
pub struct Standing {
    pub open_sketch_rank: Option<u32>,
    pub section_view_on: bool,
    pub kept_section_rank: Option<usize>,
    pub plane: Option<WorkPlane>,
}

/// The sketch open, else the section view, else the kept section shown, else
/// the plane the view stands on, else the free view.
pub fn standing_on(lang: &Catalogue, standing: Standing) -> String {
    match standing {
        Standing {
            open_sketch_rank: Some(number),
            ..
        } => super::sketch::numbered(lang, number),
        Standing {
            section_view_on: true,
            ..
        } => lang.t("command.label.section_view"),
        Standing {
            kept_section_rank: Some(rank),
            ..
        } => kept_section(lang, rank),
        Standing {
            plane: Some(plane), ..
        } => plane::label(lang, plane.kind()),
        Standing { plane: None, .. } => lang.t("app.free_view"),
    }
}

/// A kept section by its rank from one, as the Part tree names it.
pub fn kept_section(lang: &Catalogue, rank: usize) -> String {
    lang.t_with("part_tree.section", &[("rank", &rank.to_string())])
}

#[cfg(test)]
mod tests;
