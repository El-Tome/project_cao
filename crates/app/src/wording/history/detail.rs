use cao_part::history::{Operation, PointRef, RevolutionAxis};

use crate::lang::Catalogue;
use crate::wording::{constraints, dimension};

/// The line shown when a history entry is unfolded.
///
/// Every value is rounded here, before it reaches a key: a language file that
/// carried `{:.1}` would be a language file only a developer could write.
pub fn detail(lang: &Catalogue, operation: &Operation) -> String {
    match operation {
        Operation::CreateSketch { plane } => {
            let normal = plane.normal();
            lang.t_with(
                "history.detail.plane",
                &[
                    ("x", &rounded(normal.x, 0)),
                    ("y", &rounded(normal.y, 0)),
                    ("z", &rounded(normal.z, 0)),
                ],
            )
        }
        Operation::AddPoint { sketch, position } => lang.t_with(
            "history.detail.point",
            &[
                ("sketch", &sketch.to_string()),
                ("x", &rounded(position.x, 1)),
                ("y", &rounded(position.y, 1)),
            ],
        ),
        Operation::AddSegment {
            sketch, start, end, ..
        } => between(lang, *sketch, start, end),
        Operation::AddRectangle {
            sketch,
            corner,
            opposite,
            ..
        } => between(lang, *sketch, corner, opposite),
        Operation::AddCircle { sketch, radius, .. } => lang.t_with(
            "history.detail.circle",
            &[
                ("sketch", &sketch.to_string()),
                ("radius", &rounded(*radius, 2)),
            ],
        ),
        Operation::MovePoint {
            sketch,
            point,
            position,
        } => lang.t_with(
            "history.detail.point_moved",
            &[
                ("sketch", &sketch.to_string()),
                ("point", &point.0.to_string()),
                ("x", &rounded(position.x, 1)),
                ("y", &rounded(position.y, 1)),
            ],
        ),
        Operation::MoveMany { sketch, points, by } => lang.t_with(
            "history.detail.many_moved",
            &[
                ("sketch", &sketch.to_string()),
                ("count", &points.len().to_string()),
                ("x", &rounded(by.x, 1)),
                ("y", &rounded(by.y, 1)),
            ],
        ),
        Operation::MoveDimension { sketch, offset, .. } => lang.t_with(
            "history.detail.dimension_moved",
            &[
                ("sketch", &sketch.to_string()),
                ("x", &rounded(offset.x, 1)),
                ("y", &rounded(offset.y, 1)),
            ],
        ),
        Operation::Extrude { sketch, picks, .. } => lang.t_with(
            "history.detail.extrusion",
            &[
                ("sketch", &sketch.to_string()),
                ("count", &picks.len().to_string()),
            ],
        ),
        Operation::Constrain { sketch, constraint } => lang.t_with(
            "history.detail.rule",
            &[
                ("sketch", &sketch.to_string()),
                ("rule", &constraints::label(lang, *constraint)),
            ],
        ),
        Operation::EraseMany {
            sketch,
            elements,
            dimensions,
            constraints,
        } => lang.t_with(
            "history.detail.erased",
            &[
                ("sketch", &sketch.to_string()),
                ("elements", &elements.len().to_string()),
                ("dimensions", &dimensions.len().to_string()),
                ("constraints", &constraints.len().to_string()),
            ],
        ),
        Operation::MergePoints {
            sketch,
            kept,
            dropped,
        } => lang.t_with(
            "history.detail.points_merged",
            &[
                ("sketch", &sketch.to_string()),
                ("kept", &kept.0.to_string()),
                ("dropped", &dropped.0.to_string()),
            ],
        ),
        Operation::Revolve {
            sketch,
            picks,
            axis,
            ..
        } => lang.t_with(
            "history.detail.revolution",
            &[
                ("sketch", &sketch.to_string()),
                ("count", &picks.len().to_string()),
                ("axis", &revolution_axis(lang, *axis)),
            ],
        ),
        Operation::SetDimension { sketch, target, .. } => lang.t_with(
            "history.detail.dimension",
            &[
                ("sketch", &sketch.to_string()),
                ("spans", &dimension::spans(lang, target)),
            ],
        ),
    }
}

/// A shape drawn from one point to another: a segment, or a rectangle by its
/// opposite corners.
fn between(lang: &Catalogue, sketch: usize, start: &PointRef, end: &PointRef) -> String {
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
fn revolution_axis(lang: &Catalogue, axis: RevolutionAxis) -> String {
    match axis {
        RevolutionAxis::Sketch(axis) => constraints::axis(lang, axis),
        RevolutionAxis::Segment(segment) => lang.t_with(
            "history.detail.segment_axis",
            &[("segment", &segment.0.to_string())],
        ),
    }
}

fn point_label(lang: &Catalogue, point: &PointRef) -> String {
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

fn rounded(value: f64, places: usize) -> String {
    format!("{value:.places$}")
}

#[cfg(test)]
mod tests {
    use cao_part::history::ExtrusionMode;
    use cao_sketch::{
        CircleId, DimensionTarget, Element, PointId, SegmentId, SketchAxis, WorkPlane,
    };
    use glam::DVec2;

    use super::*;

    const AWAY: DVec2 = DVec2::new(3.0, 0.0);

    fn swept(axis: RevolutionAxis) -> Operation {
        Operation::Revolve {
            sketch: 0,
            picks: vec![DVec2::ZERO],
            axis,
            angle: 90.0,
            mode: ExtrusionMode::Add,
        }
    }

    fn unfolded(operation: &Operation) -> String {
        detail(&Catalogue::french(), operation)
    }

    #[test]
    fn an_unfolded_step_says_which_sketch_it_belongs_to_and_what_it_touched() {
        let steps = [
            (
                Operation::CreateSketch {
                    plane: WorkPlane::XY,
                },
                "Plan d'origine (0, 0, 1)",
            ),
            (
                Operation::AddPoint {
                    sketch: 2,
                    position: DVec2::new(1.25, -3.0),
                },
                "Esquisse 2 · (1.2, -3.0)",
            ),
            (
                Operation::AddSegment {
                    sketch: 0,
                    start: PointRef::Existing(PointId(7)),
                    end: PointRef::New(DVec2::new(2.0, 4.0)),
                    construction: false,
                },
                "Esquisse 0 · point 7 → (2.0, 4.0)",
            ),
            (
                Operation::AddCircle {
                    sketch: 1,
                    center: PointRef::New(DVec2::ZERO),
                    radius: 5.0,
                    rim: Vec::new(),
                    construction: false,
                },
                "Esquisse 1 · rayon 5.00",
            ),
            (
                Operation::MoveMany {
                    sketch: 0,
                    points: vec![PointId(1), PointId(2)],
                    by: AWAY,
                },
                "Esquisse 0 · 2 points de (3.0, 0.0)",
            ),
            (
                Operation::EraseMany {
                    sketch: 0,
                    elements: vec![Element::Point(PointId(1))],
                    dimensions: Vec::new(),
                    constraints: Vec::new(),
                },
                "Esquisse 0 · 1 tracé(s), 0 cote(s), 0 contrainte(s)",
            ),
            (
                swept(RevolutionAxis::Segment(SegmentId(4))),
                "Esquisse 0 · 1 aire(s) autour de trait 4",
            ),
            (
                swept(RevolutionAxis::Sketch(SketchAxis::V)),
                "Esquisse 0 · 1 aire(s) autour de axe vertical",
            ),
            (
                Operation::SetDimension {
                    sketch: 0,
                    target: DimensionTarget::Radius(CircleId(3)),
                    value: 60.0,
                    placement: None,
                },
                "Esquisse 0 · cercle 3",
            ),
        ];

        for (operation, reads) in steps {
            assert_eq!(unfolded(&operation), reads, "{operation:?} reads {reads:?}");
        }
    }
}
