use cao_sketch::{Constraint, Rule, SketchAxis};

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

/// The only place the constraint tool's own rule is turned into a name.
pub fn rule_label(rule: Rule) -> &'static str {
    match rule {
        Rule::Perpendicular => "Perpendiculaire",
        Rule::Parallel => "Parallèle",
        Rule::Equal => "Égalité",
        Rule::Coincident => "Coïncidence",
        Rule::Collinear => "Colinéaire",
        Rule::Tangent => "Tangence",
        Rule::Midpoint => "Milieu",
        Rule::Fixed => "Fixe",
        Rule::Concentric => "Concentrique",
    }
}

/// What to point at, said in the title bar while the constraint tool waits.
pub fn rule_asks_for(rule: Rule) -> &'static str {
    match rule {
        Rule::Perpendicular => "Cliquez deux traits à mettre d'équerre",
        Rule::Parallel => "Cliquez deux traits à rendre parallèles",
        Rule::Equal => "Cliquez deux traits, ou deux cercles, à égaliser",
        Rule::Coincident => "Cliquez un point puis un trait, ou deux points",
        Rule::Collinear => "Cliquez deux traits à coucher sur la même droite",
        Rule::Tangent => "Cliquez un cercle puis un trait",
        Rule::Midpoint => "Cliquez un point puis le trait qui le portera",
        Rule::Fixed => "Cliquez le point à fixer",
        Rule::Concentric => "Cliquez deux cercles à ramener sur le même centre",
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

    #[test]
    fn every_rule_of_the_constraint_tool_names_itself_and_says_what_it_wants() {
        let rules = [
            Rule::Perpendicular,
            Rule::Parallel,
            Rule::Equal,
            Rule::Coincident,
            Rule::Collinear,
            Rule::Tangent,
            Rule::Midpoint,
            Rule::Fixed,
            Rule::Concentric,
        ];

        let labels: BTreeSet<&str> = rules.iter().map(|rule| rule_label(*rule)).collect();
        let asks: BTreeSet<&str> = rules.iter().map(|rule| rule_asks_for(*rule)).collect();

        assert_eq!(
            labels.len(),
            rules.len(),
            "each rule of the constraint tool reads under its own name",
        );
        assert_eq!(
            asks.len(),
            rules.len(),
            "each rule tells the title bar what to point at, and none share the wording",
        );
    }
}
