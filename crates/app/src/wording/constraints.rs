use cao_sketch::{Constraint, Rule, SketchAxis};

use crate::lang::Catalogue;

/// The only place a rule of the drawing is turned into a name.
pub fn label(lang: &Catalogue, rule: Constraint) -> String {
    lang.t(match rule {
        Constraint::Perpendicular { .. } => "constraints.label.perpendicular",
        Constraint::Parallel { .. } | Constraint::AxisParallel { .. } => {
            "constraints.label.parallel"
        }
        Constraint::Equal { .. }
        | Constraint::EqualRadius { .. }
        | Constraint::EqualRadiusArc { .. }
        | Constraint::EqualRadiusArcCircle { .. } => "constraints.label.equal",
        Constraint::OnSegment { .. }
        | Constraint::OnCircle { .. }
        | Constraint::OnArc { .. }
        | Constraint::OnEllipse { .. }
        | Constraint::OnAxis { .. } => "constraints.label.coincident",
        Constraint::Collinear { .. } | Constraint::AxisCollinear { .. } => {
            "constraints.label.collinear"
        }
        Constraint::Tangent { .. }
        | Constraint::ArcTangent { .. }
        | Constraint::EllipseTangent { .. } => "constraints.label.tangent",
        Constraint::Midpoint { .. } => "constraints.label.midpoint",
        Constraint::Fixed { .. } => "constraints.label.fixed",
    })
}

/// What the history writes when a rule is taken away.
///
/// A full sentence per rule, not a hole filled with `label`: the participle
/// agrees with the rule's own gender, and `Milieu` is masculine where every
/// other rule reads feminine.
pub fn erased_label(lang: &Catalogue, rule: Constraint) -> String {
    lang.t(match rule {
        Constraint::Perpendicular { .. } => "constraints.erased.perpendicular",
        Constraint::Parallel { .. } | Constraint::AxisParallel { .. } => {
            "constraints.erased.parallel"
        }
        Constraint::Equal { .. }
        | Constraint::EqualRadius { .. }
        | Constraint::EqualRadiusArc { .. }
        | Constraint::EqualRadiusArcCircle { .. } => "constraints.erased.equal",
        Constraint::OnSegment { .. }
        | Constraint::OnCircle { .. }
        | Constraint::OnArc { .. }
        | Constraint::OnEllipse { .. }
        | Constraint::OnAxis { .. } => "constraints.erased.coincident",
        Constraint::Collinear { .. } | Constraint::AxisCollinear { .. } => {
            "constraints.erased.collinear"
        }
        Constraint::Tangent { .. }
        | Constraint::ArcTangent { .. }
        | Constraint::EllipseTangent { .. } => "constraints.erased.tangent",
        Constraint::Midpoint { .. } => "constraints.erased.midpoint",
        Constraint::Fixed { .. } => "constraints.erased.fixed",
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
        Constraint::Parallel { .. } | Constraint::AxisParallel { .. } => "//",
        Constraint::Equal { .. }
        | Constraint::EqualRadius { .. }
        | Constraint::EqualRadiusArc { .. }
        | Constraint::EqualRadiusArcCircle { .. } => "=",
        Constraint::OnSegment { .. }
        | Constraint::OnCircle { .. }
        | Constraint::OnArc { .. }
        | Constraint::OnEllipse { .. }
        | Constraint::OnAxis { .. } => "+",
        Constraint::Collinear { .. } | Constraint::AxisCollinear { .. } => "--",
        Constraint::Tangent { .. }
        | Constraint::ArcTangent { .. }
        | Constraint::EllipseTangent { .. } => "T",
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

/// What is said when the rule in hand already sits on the drawing.
///
/// A full sentence per rule, for the same reason as `erased_label`: `Milieu`
/// does not take the same ending as the rules around it.
pub fn already_there_label(lang: &Catalogue, rule: Rule) -> String {
    lang.t(match rule {
        Rule::Perpendicular => "constraints.already_there.perpendicular",
        Rule::Parallel => "constraints.already_there.parallel",
        Rule::Equal => "constraints.already_there.equal",
        Rule::Coincident => "constraints.already_there.coincident",
        Rule::Collinear => "constraints.already_there.collinear",
        Rule::Tangent => "constraints.already_there.tangent",
        Rule::Midpoint => "constraints.already_there.midpoint",
        Rule::Fixed => "constraints.already_there.fixed",
        Rule::Concentric => "constraints.already_there.concentric",
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
mod tests;
