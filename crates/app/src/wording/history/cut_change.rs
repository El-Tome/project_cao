//! What the history says of a fillet or a chamfer changed after it was laid.

use cao_part::Variables;
use cao_part::history::{CutAsked, CutChange};

use crate::lang::Catalogue;
use crate::wording::formula::sized;
use crate::wording::history::values::chamfer as chamfer_mode;

/// The line the history tree shows.
pub(super) fn label(lang: &Catalogue, change: &CutChange) -> String {
    lang.t(match change {
        CutChange::Reshape {
            value: CutAsked::Fillet(_),
            ..
        } => "history.fillet_changed",
        CutChange::Reshape {
            value: CutAsked::Chamfer(_),
            ..
        } => "history.chamfer_changed",
        CutChange::TakeOff { .. } => "history.cut_taken_off",
        CutChange::SwapSides { .. } => "history.chamfer_sides_swapped",
        CutChange::Construction {
            construction: true, ..
        } => "history.cut_in_construction",
        CutChange::Construction { .. } => "history.cut_drawn",
    })
}

/// The line shown when the entry is unfolded.
pub(super) fn detail(lang: &Catalogue, variables: &Variables, change: &CutChange) -> String {
    let sketch = change.sketch().to_string();
    match change {
        CutChange::Reshape {
            value: CutAsked::Fillet(radius),
            ..
        } => lang.t_with(
            "history.detail.fillet_changed",
            &[
                ("sketch", &sketch),
                (
                    "radius",
                    &sized(lang, variables, radius, |value| format!("{value:.3}")),
                ),
            ],
        ),
        CutChange::Reshape {
            value: CutAsked::Chamfer(mode),
            ..
        } => lang.t_with(
            "history.detail.chamfer_changed",
            &[
                ("sketch", &sketch),
                ("mode", &chamfer_mode(lang, variables, mode)),
            ],
        ),
        CutChange::TakeOff { cuts, .. } => lang.t_with(
            "history.detail.cut_taken_off",
            &[("sketch", &sketch), ("corners", &cuts.len().to_string())],
        ),
        CutChange::SwapSides { .. } | CutChange::Construction { .. } => {
            lang.t_with("history.detail.cut_changed", &[("sketch", &sketch)])
        }
    }
}
