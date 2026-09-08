use cao_part::history::{ExtrusionMode, Operation, PointRef, RevolutionAxis};
use cao_sketch::{DimensionTarget, Element, SketchAxis};

/// The only place a history step is turned into a name.
///
/// Short, because the history tree shows one per line.
pub fn label(operation: &Operation) -> String {
    match operation {
        Operation::CreateSketch { plane } => format!("Esquisse — {}", plane.label()),
        Operation::AddPoint { .. } => "Point".to_string(),
        Operation::AddSegment { .. } => "Trait".to_string(),
        Operation::AddRectangle { .. } => "Rectangle".to_string(),
        Operation::AddCircle { .. } => "Cercle".to_string(),
        Operation::MovePoint { .. } | Operation::MoveMany { .. } => "Déplacement".to_string(),
        Operation::MoveDimension { .. } => "Cote déplacée".to_string(),
        Operation::Constrain { constraint, .. } => constraint.label().to_string(),
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
            ([], [], [rule]) => format!("{} supprimée", rule.label()),
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
        Operation::SetDimension { target, value, .. } => {
            let value = short(*value);
            match target {
                DimensionTarget::Angle { .. } => format!("Angle {value}°"),
                DimensionTarget::AxisAngle { axis, .. } => {
                    format!("Angle {value}° / {}", axis.label())
                }
                DimensionTarget::Radius(_) => format!("Rayon {value} mm"),
                DimensionTarget::Diameter(_) => format!("Diamètre {value} mm"),
                DimensionTarget::Projected { axis, .. } => match axis {
                    SketchAxis::U => format!("Largeur {value} mm"),
                    SketchAxis::V => format!("Hauteur {value} mm"),
                },
                DimensionTarget::PointToSegment { .. }
                | DimensionTarget::Length(_)
                | DimensionTarget::Distance { .. } => format!("Cote {value} mm"),
            }
        }
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
            format!("Esquisse {sketch} · {}", constraint.label())
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
        Operation::SetDimension { sketch, target, .. } => match target {
            DimensionTarget::Distance { from, to } => {
                format!("Esquisse {sketch} · points {} et {}", from.0, to.0)
            }
            DimensionTarget::Length(segment) => {
                format!("Esquisse {sketch} · trait {}", segment.0)
            }
            DimensionTarget::Angle { first, second } => {
                format!("Esquisse {sketch} · traits {} et {}", first.0, second.0)
            }
            DimensionTarget::AxisAngle { segment, axis } => {
                format!("Esquisse {sketch} · trait {} / {}", segment.0, axis.label())
            }
            DimensionTarget::PointToSegment { point, segment } => {
                format!(
                    "Esquisse {sketch} · point {} au trait {}",
                    point.0, segment.0
                )
            }
            DimensionTarget::Projected { from, to, axis } => format!(
                "Esquisse {sketch} · points {} et {} sur l'{}",
                from.0,
                to.0,
                axis.label()
            ),
            DimensionTarget::Radius(circle) | DimensionTarget::Diameter(circle) => {
                format!("Esquisse {sketch} · cercle {}", circle.0)
            }
        },
    }
}

/// The only place an axis of revolution is turned into a name.
fn revolution_axis(axis: RevolutionAxis) -> String {
    match axis {
        RevolutionAxis::Sketch(axis) => axis.label().to_string(),
        RevolutionAxis::Segment(segment) => format!("trait {}", segment.0),
    }
}

fn point_label(point: &PointRef) -> String {
    match point {
        PointRef::Existing(id) => format!("point {}", id.0),
        PointRef::New(position) => format!("({:.1}, {:.1})", position.x, position.y),
    }
}

/// A value as it reads in the history: a dimension taken from the drawing
/// itself is a full float, and "Cote 60.878967 mm" is unreadable.
fn short(value: f64) -> String {
    let text = format!("{value:.2}");
    match text.contains('.') {
        true => text.trim_end_matches('0').trim_end_matches('.').to_string(),
        false => text,
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{CircleId, Constraint, PointId, SegmentId, WorkPlane};
    use glam::DVec2;

    use super::*;

    const TRAIT: SegmentId = SegmentId(0);
    const CIRCLE: CircleId = CircleId(3);
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

    fn measured(target: DimensionTarget, value: f64) -> Operation {
        Operation::SetDimension {
            sketch: 0,
            target,
            value,
            placement: None,
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
    fn a_step_that_shapes_matter_says_which_way_and_how_far() {
        let turn = |mode| swept(RevolutionAxis::Sketch(SketchAxis::V), mode);

        assert_eq!(label(&raised(ExtrusionMode::Add)), "Extrusion 12 mm");
        assert_eq!(label(&raised(ExtrusionMode::Cut)), "Enlèvement 12 mm");
        assert_eq!(label(&turn(ExtrusionMode::Add)), "Révolution 90°");
        assert_eq!(label(&turn(ExtrusionMode::Cut)), "Révolution creusée 90°");
        assert_eq!(
            label(&Operation::CreateSketch {
                plane: WorkPlane::XY,
            }),
            "Esquisse — Plan XY",
        );
        assert_eq!(
            label(&Operation::Constrain {
                sketch: 0,
                constraint: RULE,
            }),
            "Parallèle",
        );
    }

    #[test]
    fn a_single_deletion_says_what_went_and_a_grouped_one_says_how_many() {
        let one = |element| erased(vec![element], Vec::new(), Vec::new());
        let cote = DimensionTarget::Length(TRAIT);

        assert_eq!(label(&one(Element::Point(PointId(1)))), "Point supprimé");
        assert_eq!(label(&one(Element::Segment(TRAIT))), "Trait supprimé");
        assert_eq!(label(&one(Element::Circle(CIRCLE))), "Cercle supprimé");
        assert_eq!(
            label(&erased(Vec::new(), vec![cote], Vec::new())),
            "Cote supprimée",
        );
        assert_eq!(
            label(&erased(Vec::new(), Vec::new(), vec![RULE])),
            "Parallèle supprimée",
        );
        assert_eq!(
            label(&erased(
                vec![Element::Point(PointId(1)), Element::Point(PointId(2))],
                vec![cote],
                vec![RULE],
            )),
            "4 éléments supprimés",
        );
    }

    #[test]
    fn a_dimension_is_named_after_what_it_measures() {
        let along = |axis| DimensionTarget::Projected {
            from: PointId(1),
            to: PointId(2),
            axis,
        };
        let named = [
            (
                DimensionTarget::Angle {
                    first: SegmentId(0),
                    second: SegmentId(1),
                },
                "Angle 60°",
            ),
            (
                DimensionTarget::AxisAngle {
                    segment: TRAIT,
                    axis: SketchAxis::U,
                },
                "Angle 60° / axe horizontal",
            ),
            (along(SketchAxis::U), "Largeur 60 mm"),
            (along(SketchAxis::V), "Hauteur 60 mm"),
            (DimensionTarget::Radius(CIRCLE), "Rayon 60 mm"),
            (DimensionTarget::Diameter(CIRCLE), "Diamètre 60 mm"),
            (DimensionTarget::Length(TRAIT), "Cote 60 mm"),
        ];

        for (target, reads) in named {
            assert_eq!(label(&measured(target, 60.0)), reads);
        }
    }

    /// A dimension taken from the drawing itself is a full float, and
    /// "Cote 60.878967 mm" is unreadable.
    #[test]
    fn a_measured_value_is_cut_to_two_decimals_and_keeps_no_trailing_zero() {
        let cote = |value| label(&measured(DimensionTarget::Length(TRAIT), value));

        assert_eq!(cote(60.878_967), "Cote 60.88 mm");
        assert_eq!(cote(60.0), "Cote 60 mm");
        assert_eq!(cote(60.1), "Cote 60.1 mm");
        assert_eq!(cote(0.001), "Cote 0 mm");
    }

    #[test]
    fn an_unfolded_step_says_which_sketch_it_belongs_to_and_what_it_touched() {
        assert_eq!(
            detail(&Operation::CreateSketch {
                plane: WorkPlane::XY,
            }),
            "Plan d'origine (0, 0, 1)",
        );
        assert_eq!(
            detail(&Operation::AddSegment {
                sketch: 2,
                start: PointRef::Existing(PointId(7)),
                end: PointRef::New(DVec2::new(2.0, 4.0)),
            }),
            "Esquisse 2 · point 7 → (2.0, 4.0)",
        );
        assert_eq!(
            detail(&erased(
                vec![Element::Point(PointId(1))],
                Vec::new(),
                Vec::new(),
            )),
            "Esquisse 0 · 1 tracé(s), 0 cote(s), 0 contrainte(s)",
        );
        assert_eq!(
            detail(&swept(
                RevolutionAxis::Segment(SegmentId(4)),
                ExtrusionMode::Add,
            )),
            "Esquisse 0 · 1 aire(s) autour de trait 4",
        );
        assert_eq!(
            detail(&measured(DimensionTarget::Radius(CIRCLE), 60.0)),
            "Esquisse 0 · cercle 3",
        );
    }
}
