//! How a history step reads: its name in the tree, and the line it unfolds
//! into.

mod detail;

pub use detail::detail;

use cao_part::history::{ExtrusionMode, Operation};
use cao_sketch::{Constraint, DimensionTarget, Element};

use crate::lang::Catalogue;
use crate::wording::{constraints, dimension, plane};

/// The only place a history step is turned into a name.
///
/// Short, because the history tree shows one per line.
pub fn label(lang: &Catalogue, operation: &Operation) -> String {
    match operation {
        Operation::CreateSketch { plane } => lang.t_with(
            "history.sketch",
            &[("plane", &plane::label(lang, plane.kind()))],
        ),
        Operation::AddPoint { .. } => lang.t("history.point"),
        Operation::AddSegment { .. } => lang.t("history.segment"),
        Operation::AddRectangle { .. } => lang.t("history.rectangle"),
        Operation::AddCircle { .. } => lang.t("history.circle"),
        Operation::MovePoint { .. } | Operation::MoveMany { .. } => lang.t("history.move"),
        Operation::MoveDimension { .. } => lang.t("history.dimension_moved"),
        Operation::Constrain { constraint, .. } => constraints::label(lang, *constraint),
        Operation::EraseMany {
            elements,
            dimensions,
            constraints,
            ..
        } => erased(lang, elements, dimensions, constraints),
        Operation::MergePoints { .. } => lang.t("history.points_merged"),
        Operation::Revolve { angle, mode, .. } => lang.t_with(
            match mode {
                ExtrusionMode::Add => "history.revolution",
                ExtrusionMode::Cut => "history.revolution_cut",
            },
            &[("angle", &angle.to_string())],
        ),
        Operation::Extrude { distance, mode, .. } => lang.t_with(
            match mode {
                ExtrusionMode::Add => "history.extrusion",
                ExtrusionMode::Cut => "history.extrusion_cut",
            },
            &[("distance", &distance.to_string())],
        ),
        Operation::SetDimension { target, value, .. } => dimension::label(lang, target, *value),
    }
}

/// One thing taken away is named; several are only counted, since a line of
/// the tree has no room to list them.
fn erased(
    lang: &Catalogue,
    elements: &[Element],
    dimensions: &[DimensionTarget],
    rules: &[Constraint],
) -> String {
    match (elements, dimensions, rules) {
        ([Element::Point(_)], [], []) => lang.t("history.point_erased"),
        ([Element::Segment(_)], [], []) => lang.t("history.segment_erased"),
        ([Element::Circle(_)], [], []) => lang.t("history.circle_erased"),
        ([], [_], []) => lang.t("history.dimension_erased"),
        ([], [], [rule]) => lang.t_with(
            "history.rule_erased",
            &[("rule", &constraints::label(lang, *rule))],
        ),
        _ => lang.t_with(
            "history.many_erased",
            &[(
                "count",
                &(elements.len() + dimensions.len() + rules.len()).to_string(),
            )],
        ),
    }
}

#[cfg(test)]
mod tests {
    use cao_part::history::{PointRef, RevolutionAxis};
    use cao_sketch::{CircleId, PointId, SegmentId, SketchAxis, WorkPlane};
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

    fn scrubbed(
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
                construction: false,
            },
            Operation::AddRectangle {
                sketch: 0,
                corner: here,
                opposite: PointRef::New(AWAY),
                construction: false,
            },
            Operation::AddCircle {
                sketch: 0,
                center: here,
                radius: 5.0,
                rim: Vec::new(),
                construction: false,
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
        let origin = Operation::CreateSketch {
            plane: WorkPlane::XY,
        };
        assert_eq!(said(&origin), "Esquisse — Plan XY");
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
        let one = |element| scrubbed(vec![element], Vec::new(), Vec::new());
        let measure = DimensionTarget::Length(SEGMENT);

        assert_eq!(said(&one(Element::Point(PointId(1)))), "Point supprimé");
        assert_eq!(said(&one(Element::Segment(SEGMENT))), "Trait supprimé");
        assert_eq!(said(&one(Element::Circle(CIRCLE))), "Cercle supprimé");
        assert_eq!(
            said(&scrubbed(Vec::new(), vec![measure], Vec::new())),
            "Cote supprimée",
        );
        assert_eq!(
            said(&scrubbed(Vec::new(), Vec::new(), vec![RULE])),
            "Parallèle supprimée",
        );
        assert_eq!(
            said(&scrubbed(
                vec![Element::Point(PointId(1)), Element::Point(PointId(2))],
                vec![measure],
                vec![RULE],
            )),
            "4 éléments supprimés",
        );
    }
}
