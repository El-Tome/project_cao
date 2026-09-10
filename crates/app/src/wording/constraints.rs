use cao_sketch::{Constraint, Rule, SketchAxis};

use crate::lang::Catalogue;

/// The only place a rule of the drawing is turned into a name.
pub fn label(lang: &Catalogue, rule: Constraint) -> String {
    lang.t(match rule {
        Constraint::Perpendicular { .. } => "constraints.label.perpendicular",
        Constraint::Parallel { .. } => "constraints.label.parallel",
        Constraint::Equal { .. } | Constraint::EqualRadius { .. } => "constraints.label.equal",
        Constraint::OnSegment { .. } | Constraint::OnCircle { .. } => {
            "constraints.label.coincident"
        }
        Constraint::Collinear { .. } | Constraint::AxisCollinear { .. } => {
            "constraints.label.collinear"
        }
        Constraint::Tangent { .. } => "constraints.label.tangent",
        Constraint::Midpoint { .. } => "constraints.label.midpoint",
        Constraint::Fixed { .. } => "constraints.label.fixed",
    })
}

/// The mark drawn next to what a rule holds.
///
/// Plain letters and punctuation: the drawing symbols of the trade —
/// ⊥, ∥, ½ — are not in the fonts the interface ships with, and a mark that
/// comes out as an empty box says less than nothing. They stay out of the
/// language file for the same reason a digit does: they are drawn, not read.
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
pub fn axis(lang: &Catalogue, axis: SketchAxis) -> String {
    lang.t(match axis {
        SketchAxis::U => "constraints.axis.horizontal",
        SketchAxis::V => "constraints.axis.vertical",
    })
}

/// The only place the constraint tool's own rule is turned into a name.
///
/// A rule in hand and a rule already laid down read the same, so they take
/// their name from the same key rather than from two that must be kept in
/// step by hand.
pub fn rule_label(lang: &Catalogue, rule: Rule) -> String {
    lang.t(match rule {
        Rule::Perpendicular => "constraints.label.perpendicular",
        Rule::Parallel => "constraints.label.parallel",
        Rule::Equal => "constraints.label.equal",
        Rule::Coincident => "constraints.label.coincident",
        Rule::Collinear => "constraints.label.collinear",
        Rule::Tangent => "constraints.label.tangent",
        Rule::Midpoint => "constraints.label.midpoint",
        Rule::Fixed => "constraints.label.fixed",
        Rule::Concentric => "constraints.label.concentric",
    })
}

/// What to point at, said in the title bar while the constraint tool waits.
pub fn rule_asks_for(lang: &Catalogue, rule: Rule) -> String {
    lang.t(match rule {
        Rule::Perpendicular => "constraints.asks_for.perpendicular",
        Rule::Parallel => "constraints.asks_for.parallel",
        Rule::Equal => "constraints.asks_for.equal",
        Rule::Coincident => "constraints.asks_for.coincident",
        Rule::Collinear => "constraints.asks_for.collinear",
        Rule::Tangent => "constraints.asks_for.tangent",
        Rule::Midpoint => "constraints.asks_for.midpoint",
        Rule::Fixed => "constraints.asks_for.fixed",
        Rule::Concentric => "constraints.asks_for.concentric",
    })
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
        let lang = Catalogue::french();
        let named = named_rules();

        for (rule, reads) in named {
            assert_eq!(label(&lang, rule), reads, "{rule:?} reads {reads:?}");
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
        let lang = Catalogue::french();

        assert_eq!(axis(&lang, SketchAxis::U), "axe horizontal");
        assert_eq!(axis(&lang, SketchAxis::V), "axe vertical");
    }

    #[test]
    fn every_rule_of_the_constraint_tool_names_itself_and_says_what_it_wants() {
        let lang = Catalogue::french();
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

        let labels: BTreeSet<String> = rules.iter().map(|rule| rule_label(&lang, *rule)).collect();
        let asks: BTreeSet<String> = rules
            .iter()
            .map(|rule| rule_asks_for(&lang, *rule))
            .collect();

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

    #[test]
    fn a_rule_in_hand_and_the_same_rule_laid_down_read_the_same() {
        let lang = Catalogue::french();
        let laid_down = Constraint::Parallel {
            first: FIRST,
            second: SECOND,
        };

        assert_eq!(rule_label(&lang, Rule::Parallel), label(&lang, laid_down));
    }
}
