//! What the bar above the view says of where it stands.

use cao_sketch::WorkPlane;

use crate::lang::Catalogue;
use crate::wording::plane;

/// The free view, the plane the view is on, or — for a drawing laid on a kept
/// section — that section by its name, which the plane alone cannot tell from
/// a face of the part.
pub fn standing_on(lang: &Catalogue, plane: Option<WorkPlane>, section: Option<usize>) -> String {
    match (plane, section) {
        (None, _) => lang.t("app.free_view"),
        (Some(_), Some(section)) => {
            lang.t_with("part_tree.section", &[("rank", &(section + 1).to_string())])
        }
        (Some(plane), None) => plane::label(lang, plane.kind()),
    }
}
