//! How one value inside a history line is written: the two points a trait
//! runs between, an axis, a rounded number, what a chamfer took.

use cao_part::history::{PointRef, RevolutionAxis};
use cao_sketch::Chamfer;

use crate::lang::Catalogue;
use crate::wording::constraints;

/// A shape drawn from one point to another: a segment, or a rectangle by its
/// opposite corners.
pub(super) fn between(lang: &Catalogue, sketch: usize, start: &PointRef, end: &PointRef) -> String {
    lang.t_with(
        "history.detail.between",
        &[
            ("sketch", &sketch.to_string()),
            ("start", &point_label(lang, start)),
            ("end", &point_label(lang, end)),
        ],
    )
}

/// The only place an axis of revolution is turned into a name.
pub(super) fn revolution_axis(lang: &Catalogue, axis: RevolutionAxis) -> String {
    match axis {
        RevolutionAxis::Sketch(axis) => constraints::axis(lang, axis),
        RevolutionAxis::Segment(segment) => lang.t_with(
            "history.detail.segment_axis",
            &[("segment", &segment.0.to_string())],
        ),
    }
}

pub(super) fn point_label(lang: &Catalogue, point: &PointRef) -> String {
    match point {
        PointRef::Existing(id) => lang.t_with(
            "history.detail.existing_point",
            &[("point", &id.0.to_string())],
        ),
        PointRef::New(position) => lang.t_with(
            "history.detail.new_point",
            &[
                ("x", &rounded(position.x, 1)),
                ("y", &rounded(position.y, 1)),
            ],
        ),
    }
}

pub(super) fn rounded(value: f64, places: usize) -> String {
    format!("{value:.places$}")
}

/// What a chamfer took off each side, in the words of the mode it was cut in.
pub(super) fn chamfer(lang: &Catalogue, mode: Chamfer) -> String {
    let say = |value: f64| format!("{value:.3}");
    match mode {
        Chamfer::Equal(reach) => lang.t_with("history.chamfer.equal", &[("reach", &say(reach))]),
        Chamfer::Sided { first, second } => lang.t_with(
            "history.chamfer.sided",
            &[("first", &say(first)), ("second", &say(second))],
        ),
        Chamfer::Angled { along, degrees } => lang.t_with(
            "history.chamfer.angled",
            &[("along", &say(along)), ("degrees", &say(degrees))],
        ),
    }
}
