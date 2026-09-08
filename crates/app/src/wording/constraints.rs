use cao_sketch::{Constraint, SketchAxis};

/// The only place a rule of the drawing is turned into a name.
pub fn label(rule: Constraint) -> &'static str {
    match rule {
        Constraint::Perpendicular { .. } => "Perpendiculaire",
        Constraint::Parallel { .. } => "Parallèle",
        Constraint::Equal { .. } | Constraint::EqualRadius { .. } => "Égalité",
        Constraint::OnSegment { .. } | Constraint::OnCircle { .. } => "Coïncidence",
        Constraint::Collinear { .. } | Constraint::AxisCollinear { .. } => "Colinéaire",
        Constraint::Tangent { .. } => "Tangence",
        Constraint::Midpoint { .. } => "Milieu",
        Constraint::Fixed { .. } => "Fixe",
    }
}

/// The mark drawn next to what a rule holds.
///
/// Plain letters and punctuation: the drawing symbols of the trade —
/// ⊥, ∥, ½ — are not in the fonts the interface ships with, and a mark that
/// comes out as an empty box says less than nothing.
pub fn mark(rule: Constraint) -> &'static str {
    match rule {
        Constraint::Perpendicular { .. } => "|_",
        Constraint::Parallel { .. } => "//",
        Constraint::Equal { .. } | Constraint::EqualRadius { .. } => "=",
        Constraint::OnSegment { .. } | Constraint::OnCircle { .. } => "+",
        Constraint::Collinear { .. } | Constraint::AxisCollinear { .. } => "--",
        Constraint::Tangent { .. } => "T",
        Constraint::Midpoint { .. } => "1/2",
        Constraint::Fixed { .. } => "X",
    }
}

/// The only place one of the sketch's own axes is turned into a name.
pub fn axis(axis: SketchAxis) -> &'static str {
    match axis {
        SketchAxis::U => "axe horizontal",
        SketchAxis::V => "axe vertical",
    }
}

#[cfg(test)]
mod tests {
    use cao_sketch::{CircleId, Element, PointId, SegmentId};

    use super::*;

    const FIRST: SegmentId = SegmentId(0);
    const SECOND: SegmentId = SegmentId(1);

    fn every_rule() -> [Constraint; 11] {
        [
            Constraint::Perpendicular {
                first: FIRST,
                second: SECOND,
            },
            Constraint::Parallel {
                first: FIRST,
                second: SECOND,
            },
            Constraint::Equal {
                first: FIRST,
                second: SECOND,
            },
            Constraint::EqualRadius {
                first: CircleId(0),
                second: CircleId(1),
            },
            Constraint::OnSegment {
                point: PointId(1),
                segment: FIRST,
            },
            Constraint::OnCircle {
                point: PointId(1),
                circle: CircleId(0),
            },
            Constraint::Collinear {
                first: FIRST,
                second: SECOND,
            },
            Constraint::AxisCollinear {
                segment: FIRST,
                axis: SketchAxis::U,
            },
            Constraint::Tangent {
                circle: CircleId(0),
                segment: FIRST,
                at: None,
            },
            Constraint::Midpoint {
                point: PointId(1),
                segment: FIRST,
            },
            Constraint::Fixed {
                element: Element::Point(PointId(1)),
            },
        ]
    }

    #[test]
    fn rules_that_say_the_same_thing_read_the_same_and_the_others_do_not() {
        let names = [
            "Perpendiculaire",
            "Parallèle",
            "Égalité",
            "Égalité",
            "Coïncidence",
            "Coïncidence",
            "Colinéaire",
            "Colinéaire",
            "Tangence",
            "Milieu",
            "Fixe",
        ];

        for (rule, reads) in every_rule().iter().zip(names) {
            assert_eq!(label(*rule), reads, "{rule:?} reads {reads:?}");
        }
    }

    #[test]
    fn every_mark_is_drawable_in_the_fonts_the_interface_ships_with() {
        for rule in every_rule() {
            let drawn = mark(rule);

            assert!(
                drawn.is_ascii(),
                "{rule:?} is marked {drawn:?}, which needs a font the interface has not got",
            );
        }
    }

    #[test]
    fn the_two_axes_of_a_sketch_read_differently() {
        assert_eq!(axis(SketchAxis::U), "axe horizontal");
        assert_eq!(axis(SketchAxis::V), "axe vertical");
    }
}
