//! What the constraint tool is pointed at, and what it means once it has been
//! shown enough.
//!
//! Two of the nine rules are not constraints at all: a point brought onto
//! another, or two circles onto a single centre, are merges — holding them
//! apart with an equation would leave two points sitting on top of each other
//! for ever, which is exactly what the drawing does not want. `RuleIntent`
//! keeps that apart from `Constraint` so the caller, which does know how a
//! merge is recorded, can tell the two apart.

use crate::arc::ArcId;
use crate::constraints::{Constraint, SketchAxis};
use crate::ellipse::EllipseId;
use crate::sketch::{CircleId, Element, PointId, SegmentId, Sketch};

/// What the constraint tool has been pointed at: a piece of the drawing, or
/// one of the sketch's own axes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RulePick {
    Element(Element),
    Axis(SketchAxis),
}

/// Which rule the constraint tool is about to lay down.
///
/// A rule is placed by pointing at what it speaks of: two traits for a right
/// angle, a point and a trait for a coincidence. The tool holds what has been
/// picked so far and lays the rule down as soon as it has enough.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    Perpendicular,
    Parallel,
    Equal,
    Coincident,
    Collinear,
    Tangent,
    Midpoint,
    Fixed,
    Concentric,
}

impl Rule {
    /// How many things it needs before it can be laid down.
    pub fn arity(self) -> usize {
        match self {
            Self::Fixed => 1,
            _ => 2,
        }
    }
}

/// The step a rule becomes, once it has been shown what it speaks of.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuleIntent {
    Constrain(Constraint),
    Merge { kept: PointId, dropped: PointId },
}

/// Turns what the constraint tool has been shown into what it means.
///
/// The order of the clicks does not matter: a point and a trait make the same
/// coincidence whichever comes first, so this is built from the kinds
/// gathered rather than from their order.
pub fn rule_intent(rule: Rule, picks: &[RulePick], sketch: &Sketch) -> Option<RuleIntent> {
    // The order is kept: for an equality, the first trait clicked is the one
    // whose length the other takes.
    let segments: Vec<SegmentId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Segment(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let points: Vec<PointId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Point(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let circles: Vec<CircleId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Circle(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let arcs: Vec<ArcId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Arc(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let ellipses: Vec<EllipseId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Ellipse(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let axes: Vec<SketchAxis> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Axis(axis) => Some(*axis),
            _ => None,
        })
        .collect();

    let constrain = |constraint: Constraint| Some(RuleIntent::Constrain(constraint));
    let pair = |list: &[SegmentId]| (list.len() == 2).then(|| (list[0], list[1]));

    match rule {
        Rule::Perpendicular => pair(&segments)
            .and_then(|(first, second)| constrain(Constraint::Perpendicular { first, second })),
        Rule::Parallel => pair(&segments)
            .and_then(|(first, second)| constrain(Constraint::Parallel { first, second })),
        Rule::Collinear => match (pair(&segments), segments.as_slice(), axes.as_slice()) {
            (Some((first, second)), _, _) => constrain(Constraint::Collinear { first, second }),
            // A trait laid on one of the sketch's own axes, which is the same
            // rule against a line that cannot move.
            (None, [segment], [axis]) => constrain(Constraint::AxisCollinear {
                segment: *segment,
                axis: *axis,
            }),
            _ => None,
        },
        Rule::Equal => match (pair(&segments), circles.as_slice(), arcs.as_slice()) {
            (Some((first, second)), _, _) => constrain(Constraint::Equal { first, second }),
            (None, [first, second], []) => constrain(Constraint::EqualRadius {
                first: *first,
                second: *second,
            }),
            (None, [], [first, second]) => constrain(Constraint::EqualRadiusArc {
                first: *first,
                second: *second,
            }),
            _ => None,
        },
        Rule::Tangent => match (
            circles.as_slice(),
            segments.as_slice(),
            arcs.as_slice(),
            ellipses.as_slice(),
        ) {
            ([circle], [segment], [], []) => constrain(Constraint::Tangent {
                at: None,
                circle: *circle,
                segment: *segment,
            }),
            ([], [segment], [arc], []) => constrain(Constraint::ArcTangent {
                at: None,
                arc: *arc,
                segment: *segment,
            }),
            ([], [segment], [], [ellipse]) => constrain(Constraint::EllipseTangent {
                at: None,
                ellipse: *ellipse,
                segment: *segment,
            }),
            _ => None,
        },
        Rule::Midpoint => match (points.as_slice(), segments.as_slice()) {
            ([point], [segment]) => constrain(Constraint::Midpoint {
                point: *point,
                segment: *segment,
            }),
            _ => None,
        },
        Rule::Fixed => match picks {
            [RulePick::Element(element)] => constrain(Constraint::Fixed { element: *element }),
            _ => None,
        },
        Rule::Coincident => match (points.as_slice(), segments.as_slice()) {
            ([point], [segment]) => constrain(Constraint::OnSegment {
                point: *point,
                segment: *segment,
            }),
            // Two points asked to coincide are one point: the origin is never
            // the one that gives way.
            ([first, second], []) => {
                let (kept, dropped) = match sketch.is_origin(*second) {
                    true => (*second, *first),
                    false => (*first, *second),
                };
                Some(RuleIntent::Merge { kept, dropped })
            }
            _ => None,
        },
        Rule::Concentric => match circles.as_slice() {
            [first, second] => {
                let (kept, dropped) = (sketch.circle(*first).center, sketch.circle(*second).center);
                (kept != dropped).then_some(RuleIntent::Merge { kept, dropped })
            }
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests;
