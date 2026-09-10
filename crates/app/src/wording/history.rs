use cao_part::history::{ExtrusionMode, Operation, PointRef, RevolutionAxis};
use cao_sketch::Element;

use crate::lang::Catalogue;
use crate::wording::{constraints, dimension, plane};

/// The only place a history step is turned into a name.
///
/// Short, because the history tree shows one per line.
pub fn label(lang: &Catalogue, operation: &Operation) -> String {
    match operation {
        Operation::CreateSketch { plane } => {
            lang.t_with("history.sketch", &[("plane", plane::label(plane.kind()))])
        }
        Operation::AddPoint { .. } => lang.t("history.point"),
        Operation::AddSegment { .. } => "Trait".to_string(),
        Operation::AddRectangle { .. } => "Rectangle".to_string(),
        Operation::AddCircle { .. } => "Cercle".to_string(),
        Operation::MovePoint { .. } | Operation::MoveMany { .. } => "Déplacement".to_string(),
        Operation::MoveDimension { .. } => "Cote déplacée".to_string(),
        Operation::Constrain { constraint, .. } => constraints::label(*constraint).to_string(),
        Operation::EraseMany {
            elements,
            dimensions,
            constraints,
            ..
        } => match (
            elements.as_slice(),
            dimensions.as_slice(),
            constraints.as_slice(),
        ) {
            ([Element::Point(_)], [], []) => "Point supprimé".to_string(),
            ([Element::Segment(_)], [], []) => "Trait supprimé".to_string(),
            ([Element::Circle(_)], [], []) => "Cercle supprimé".to_string(),
            ([], [_], []) => "Cote supprimée".to_string(),
            ([], [], [rule]) => format!("{} supprimée", constraints::label(*rule)),
            _ => format!(
                "{} éléments supprimés",
                elements.len() + dimensions.len() + constraints.len()
            ),
        },
        Operation::MergePoints { .. } => "Sommets fusionnés".to_string(),
        Operation::Revolve { angle, mode, .. } => {
            let verb = match mode {
                ExtrusionMode::Add => "Révolution",
                ExtrusionMode::Cut => "Révolution creusée",
            };
            format!("{verb} {angle}°")
        }
        Operation::Extrude { distance, mode, .. } => {
            let verb = match mode {
                ExtrusionMode::Add => "Extrusion",
                ExtrusionMode::Cut => "Enlèvement",
            };
            format!("{verb} {distance} mm")
        }
        Operation::SetDimension { target, value, .. } => dimension::label(target, *value),
    }
}

/// The line shown when a history entry is unfolded.
pub fn detail(operation: &Operation) -> String {
    match operation {
        Operation::CreateSketch { plane } => format!(
            "Plan d'origine ({:.0}, {:.0}, {:.0})",
            plane.normal().x,
            plane.normal().y,
            plane.normal().z
        ),
        Operation::AddPoint { sketch, position } => {
            format!("Esquisse {sketch} · ({:.1}, {:.1})", position.x, position.y)
        }
        Operation::AddSegment { sketch, start, end } => {
            format!(
                "Esquisse {sketch} · {} → {}",
                point_label(start),
                point_label(end)
            )
        }
        Operation::AddRectangle {
            sketch,
            corner,
            opposite,
        } => format!(
            "Esquisse {sketch} · {} → {}",
            point_label(corner),
            point_label(opposite)
        ),
        Operation::AddCircle { sketch, radius, .. } => {
            format!("Esquisse {sketch} · rayon {radius:.2}")
        }
        Operation::MovePoint {
            sketch,
            point,
            position,
        } => format!(
            "Esquisse {sketch} · point {} vers ({:.1}, {:.1})",
            point.0, position.x, position.y
        ),
        Operation::MoveMany { sketch, points, by } => format!(
            "Esquisse {sketch} · {} points de ({:.1}, {:.1})",
            points.len(),
            by.x,
            by.y
        ),
        Operation::MoveDimension { sketch, offset, .. } => format!(
            "Esquisse {sketch} · décalage ({:.1}, {:.1})",
            offset.x, offset.y
        ),
        Operation::Extrude { sketch, picks, .. } => {
            format!("Esquisse {sketch} · {} aire(s)", picks.len())
        }
        Operation::Constrain { sketch, constraint } => {
            format!("Esquisse {sketch} · {}", constraints::label(*constraint))
        }
        Operation::EraseMany {
            sketch,
            elements,
            dimensions,
            constraints,
        } => format!(
            "Esquisse {sketch} · {} tracé(s), {} cote(s), {} contrainte(s)",
            elements.len(),
            dimensions.len(),
            constraints.len()
        ),
        Operation::MergePoints {
            sketch,
            kept,
            dropped,
        } => format!("Esquisse {sketch} · points {} et {}", kept.0, dropped.0),
        Operation::Revolve {
            sketch,
            picks,
            axis,
            ..
        } => format!(
            "Esquisse {sketch} · {} aire(s) autour de {}",
            picks.len(),
            revolution_axis(*axis)
        ),
        Operation::SetDimension { sketch, target, .. } => {
            format!("Esquisse {sketch} · {}", dimension::spans(target))
        }
    }
}

/// The only place an axis of revolution is turned into a name.
fn revolution_axis(axis: RevolutionAxis) -> String {
    match axis {
        RevolutionAxis::Sketch(axis) => constraints::axis(axis).to_string(),
        RevolutionAxis::Segment(segment) => format!("trait {}", segment.0),
    }
}

fn point_label(point: &PointRef) -> String {
    match point {
        PointRef::Existing(id) => format!("point {}", id.0),
        PointRef::New(position) => format!("({:.1}, {:.1})", position.x, position.y),
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{
        CircleId, Constraint, DimensionTarget, PointId, SegmentId, SketchAxis, WorkPlane,
    };
    use glam::DVec2;

    use super::*;

    fn said(operation: &Operation) -> String {
        label(&Catalogue::french(), operation)
    }

    const SEGMENT: SegmentId = SegmentId(0);
    const CIRCLE: CircleId = CircleId(3);
    const AWAY: DVec2 = DVec2::new(3.0, 0.0);
    const RULE: Constraint = Constraint::Parallel {
        first: SegmentId(0),
        second: SegmentId(1),
    };

    fn erased(
        elements: Vec<Element>,
        dimensions: Vec<DimensionTarget>,
        constraints: Vec<Constraint>,
    ) -> Operation {
        Operation::EraseMany {
            sketch: 0,
            elements,
            dimensions,
            constraints,
        }
    }

    fn raised(mode: ExtrusionMode) -> Operation {
        Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::ZERO],
            distance: 12.0,
            mode,
        }
    }

    fn swept(axis: RevolutionAxis, mode: ExtrusionMode) -> Operation {
        Operation::Revolve {
            sketch: 0,
            picks: vec![DVec2::ZERO],
            axis,
            angle: 90.0,
            mode,
        }
    }

    #[test]
    fn a_drawn_step_is_named_after_the_shape_it_left_behind() {
        let here = PointRef::New(DVec2::ZERO);
        let drawn = [
            Operation::AddPoint {
                sketch: 0,
                position: DVec2::ZERO,
            },
            Operation::AddSegment {
                sketch: 0,
                start: here,
                end: PointRef::New(AWAY),
            },
            Operation::AddRectangle {
                sketch: 0,
                corner: here,
                opposite: PointRef::New(AWAY),
            },
            Operation::AddCircle {
                sketch: 0,
                center: here,
                radius: 5.0,
                rim: Vec::new(),
            },
            Operation::MovePoint {
                sketch: 0,
                point: PointId(1),
                position: AWAY,
            },
            Operation::MoveMany {
                sketch: 0,
                points: vec![PointId(1)],
                by: AWAY,
            },
            Operation::MoveDimension {
                sketch: 0,
                target: DimensionTarget::Length(SEGMENT),
                offset: AWAY,
            },
            Operation::MergePoints {
                sketch: 0,
                kept: PointId(1),
                dropped: PointId(2),
            },
        ];
        let names = [
            "Point",
            "Trait",
            "Rectangle",
            "Cercle",
            "Déplacement",
            "Déplacement",
            "Cote déplacée",
            "Sommets fusionnés",
        ];

        for (operation, reads) in drawn.iter().zip(names) {
            assert_eq!(said(operation), reads, "{operation:?} reads {reads:?}");
        }
    }

    #[test]
    fn a_step_says_which_plane_which_rule_and_which_way_matter_went() {
        let turn = |mode| swept(RevolutionAxis::Sketch(SketchAxis::V), mode);

        assert_eq!(said(&raised(ExtrusionMode::Add)), "Extrusion 12 mm");
        assert_eq!(said(&raised(ExtrusionMode::Cut)), "Enlèvement 12 mm");
        assert_eq!(said(&turn(ExtrusionMode::Add)), "Révolution 90°");
        assert_eq!(said(&turn(ExtrusionMode::Cut)), "Révolution creusée 90°");
        assert_eq!(
            said(&Operation::CreateSketch {
                plane: WorkPlane::XY,
            }),
            "Esquisse — Plan XY",
        );
        assert_eq!(
            said(&Operation::Constrain {
                sketch: 0,
                constraint: RULE,
            }),
            "Parallèle",
        );
    }

    #[test]
    fn a_single_deletion_says_what_went_and_a_grouped_one_says_how_many() {
        let one = |element| erased(vec![element], Vec::new(), Vec::new());
        let measure = DimensionTarget::Length(SEGMENT);

        assert_eq!(said(&one(Element::Point(PointId(1)))), "Point supprimé");
        assert_eq!(said(&one(Element::Segment(SEGMENT))), "Trait supprimé");
        assert_eq!(said(&one(Element::Circle(CIRCLE))), "Cercle supprimé");
        assert_eq!(
            said(&erased(Vec::new(), vec![measure], Vec::new())),
            "Cote supprimée",
        );
        assert_eq!(
            said(&erased(Vec::new(), Vec::new(), vec![RULE])),
            "Parallèle supprimée",
        );
        assert_eq!(
            said(&erased(
                vec![Element::Point(PointId(1)), Element::Point(PointId(2))],
                vec![measure],
                vec![RULE],
            )),
            "4 éléments supprimés",
        );
    }

    #[test]
    fn an_unfolded_step_says_which_sketch_it_belongs_to_and_what_it_touched() {
        let unfolded = [
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
                },
                "Esquisse 0 · point 7 → (2.0, 4.0)",
            ),
            (
                Operation::AddCircle {
                    sketch: 1,
                    center: PointRef::New(DVec2::ZERO),
                    radius: 5.0,
                    rim: Vec::new(),
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
                erased(vec![Element::Point(PointId(1))], Vec::new(), Vec::new()),
                "Esquisse 0 · 1 tracé(s), 0 cote(s), 0 contrainte(s)",
            ),
            (
                swept(RevolutionAxis::Segment(SegmentId(4)), ExtrusionMode::Add),
                "Esquisse 0 · 1 aire(s) autour de trait 4",
            ),
            (
                swept(RevolutionAxis::Sketch(SketchAxis::V), ExtrusionMode::Add),
                "Esquisse 0 · 1 aire(s) autour de axe vertical",
            ),
            (
                Operation::SetDimension {
                    sketch: 0,
                    target: DimensionTarget::Radius(CIRCLE),
                    value: 60.0,
                    placement: None,
                },
                "Esquisse 0 · cercle 3",
            ),
        ];

        for (operation, reads) in unfolded {
            assert_eq!(detail(&operation), reads, "{operation:?} reads {reads:?}");
        }
    }
}
