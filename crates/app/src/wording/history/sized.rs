//! The unfolded line of a step written with sizes of its own: a chamfer, a
//! fillet, and the two patterns. Each size is said as what it was written as.

use cao_part::history::{ChamferAsked, RepeatsAsked};
use cao_part::{Formula, Variables};
use cao_sketch::{ChosenAxis, PointId};

use crate::lang::Catalogue;
use crate::wording::formula::sized;
use crate::wording::history::values::{chamfer as chamfer_mode, chosen_axis, rounded};

pub(super) fn chamfer(
    lang: &Catalogue,
    variables: &Variables,
    sketch: usize,
    corners: usize,
    mode: &ChamferAsked,
) -> String {
    lang.t_with(
        "history.detail.chamfer",
        &[
            ("sketch", &sketch.to_string()),
            ("corners", &corners.to_string()),
            ("mode", &chamfer_mode(lang, variables, mode)),
        ],
    )
}

pub(super) fn fillet(
    lang: &Catalogue,
    variables: &Variables,
    sketch: usize,
    corners: usize,
    radius: &Formula,
) -> String {
    lang.t_with(
        "history.detail.fillet",
        &[
            ("sketch", &sketch.to_string()),
            ("corners", &corners.to_string()),
            (
                "radius",
                &sized(lang, variables, radius, |value| format!("{value:.3}")),
            ),
        ],
    )
}

/// `taken` is the sketch the copies are laid in and how many elements were
/// taken to be copied.
pub(super) fn circular_pattern(
    lang: &Catalogue,
    variables: &Variables,
    (sketch, elements): (usize, usize),
    centre: PointId,
    degrees: &Formula,
    count: &Formula,
) -> String {
    lang.t_with(
        "history.detail.circular_pattern",
        &[
            ("sketch", &sketch.to_string()),
            ("elements", &elements.to_string()),
            ("centre", &centre.0.to_string()),
            (
                "degrees",
                &sized(lang, variables, degrees, |value| rounded(value, 3)),
            ),
            ("count", &sized(lang, variables, count, whole)),
        ],
    )
}

pub(super) fn rectangular_pattern(
    lang: &Catalogue,
    variables: &Variables,
    (sketch, elements): (usize, usize),
    direction: ChosenAxis,
    along: &RepeatsAsked,
    across: &RepeatsAsked,
) -> String {
    let step = |run: &RepeatsAsked| sized(lang, variables, &run.step, |value| rounded(value, 3));
    lang.t_with(
        "history.detail.rectangular_pattern",
        &[
            ("sketch", &sketch.to_string()),
            ("elements", &elements.to_string()),
            ("direction", &chosen_axis(lang, direction)),
            ("along", &sized(lang, variables, &along.count, whole)),
            ("along_step", &step(along)),
            ("across", &sized(lang, variables, &across.count, whole)),
            ("across_step", &step(across)),
        ],
    )
}

/// A count, as the whole number it is meant to be.
fn whole(value: f64) -> String {
    format!("{value:.0}")
}

/// A variable's formula, said with what it comes to when it is more than a
/// number.
pub(super) fn formula(lang: &Catalogue, variables: &Variables, written: &Formula) -> String {
    sized(lang, variables, written, crate::wording::dimension::short)
}
