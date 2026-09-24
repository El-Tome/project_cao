//! The unfolded line of a step written with sizes of its own: a chamfer, a
//! fillet, and the two patterns.

use cao_sketch::{Chamfer, ChosenAxis, PointId, Repeats};

use crate::lang::Catalogue;
use crate::wording::history::values::{chamfer as chamfer_mode, chosen_axis, rounded};

pub(super) fn chamfer(lang: &Catalogue, sketch: usize, corners: usize, mode: Chamfer) -> String {
    lang.t_with(
        "history.detail.chamfer",
        &[
            ("sketch", &sketch.to_string()),
            ("corners", &corners.to_string()),
            ("mode", &chamfer_mode(lang, mode)),
        ],
    )
}

pub(super) fn fillet(lang: &Catalogue, sketch: usize, corners: usize, radius: f64) -> String {
    lang.t_with(
        "history.detail.fillet",
        &[
            ("sketch", &sketch.to_string()),
            ("corners", &corners.to_string()),
            ("radius", &format!("{radius:.3}")),
        ],
    )
}

pub(super) fn circular_pattern(
    lang: &Catalogue,
    sketch: usize,
    elements: usize,
    centre: PointId,
    degrees: f64,
    count: usize,
) -> String {
    lang.t_with(
        "history.detail.circular_pattern",
        &[
            ("sketch", &sketch.to_string()),
            ("elements", &elements.to_string()),
            ("centre", &centre.0.to_string()),
            ("degrees", &rounded(degrees, 3)),
            ("count", &count.to_string()),
        ],
    )
}

pub(super) fn rectangular_pattern(
    lang: &Catalogue,
    sketch: usize,
    elements: usize,
    direction: ChosenAxis,
    along: Repeats,
    across: Repeats,
) -> String {
    lang.t_with(
        "history.detail.rectangular_pattern",
        &[
            ("sketch", &sketch.to_string()),
            ("elements", &elements.to_string()),
            ("direction", &chosen_axis(lang, direction)),
            ("along", &along.count.to_string()),
            ("along_step", &rounded(along.step, 3)),
            ("across", &across.count.to_string()),
            ("across_step", &rounded(across.step, 3)),
        ],
    )
}
