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
    use std::collections::BTreeSet;

    use cao_sketch::{CircleId, Element, PointId, SegmentId};

    use super::*;

    const FIRST: SegmentId = SegmentId(0);
    const SECOND: SegmentId = SegmentId(1);

    /// Every rule, next to how it reads, so that adding a variant without
    /// saying how it reads cannot slip past the way two parallel lists would.
    fn named_rules() -> [(Constraint, &'static str); 11] {
        [
            (
                Constraint::Perpendicular {
                    first: FIRST,
                    second: SECOND,
                },
                "Perpendiculaire",
            ),
            (
                Constraint::Parallel {
                    first: FIRST,
                    second: SECOND,
                },
                "Parallèle",
            ),
            (
                Constraint::Equal {
                    first: FIRST,
                    second: SECOND,
                },
                "Égalité",
            ),
            (
                Constraint::EqualRadius {
                    first: CircleId(0),
                    second: CircleId(1),
                },
                "Égalité",
            ),
            (
                Constraint::OnSegment {
                    point: PointId(1),
                    segment: FIRST,
                },
                "Coïncidence",
            ),
            (
                Constraint::OnCircle {
                    point: PointId(1),
                    circle: CircleId(0),
                },
                "Coïncidence",
            ),
            (
                Constraint::Collinear {
                    first: FIRST,
                    second: SECOND,
                },
                "Colinéaire",
            ),
            (
                Constraint::AxisCollinear {
                    segment: FIRST,
                    axis: SketchAxis::U,
                },
                "Colinéaire",
            ),
            (
                Constraint::Tangent {
                    circle: CircleId(0),
                    segment: FIRST,
                    at: None,
                },
                "Tangence",
            ),
            (
                Constraint::Midpoint {
                    point: PointId(1),
                    segment: FIRST,
                },
                "Milieu",
            ),
            (
                Constraint::Fixed {
                    element: Element::Point(PointId(1)),
                },
                "Fixe",
            ),
        ]
    }

    #[test]
    fn rules_that_say_the_same_thing_read_the_same_and_the_others_do_not() {
        let named = named_rules();

        for (rule, reads) in named {
            assert_eq!(label(rule), reads, "{rule:?} reads {reads:?}");
        }

        let read: BTreeSet<&str> = named.iter().map(|(_, reads)| *reads).collect();

        assert_eq!(
            read.len(),
            8,
            "eleven rules read as eight names, three of them shared: {read:?}",
        );
    }

    #[test]
    fn every_mark_is_drawable_in_the_fonts_the_interface_ships_with() {
        for (rule, _) in named_rules() {
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
