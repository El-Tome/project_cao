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
use crate::laid_from::LaidFrom;
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
/// The order of the clicks is kept where the rule has two things alike to it:
/// the one clicked first stays where it is while the rule lands, and the other
/// comes to it. A point that belongs to something keeps it too, against a trait
/// or a curve. A free point makes the same coincidence whichever comes first —
/// the point comes onto the trait or the curve (#548, #554).
pub fn rule_intent(rule: Rule, picks: &[RulePick], sketch: &Sketch) -> Option<RuleIntent> {
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
        Rule::Perpendicular => match (pair(&segments), segments.as_slice(), axes.as_slice()) {
            (Some((first, second)), _, _) => constrain(Constraint::Perpendicular { first, second }),
            (None, [segment], [axis]) => constrain(Constraint::AxisPerpendicular {
                segment: *segment,
                axis: *axis,
            }),
            _ => None,
        },
        Rule::Parallel => match (pair(&segments), segments.as_slice(), axes.as_slice()) {
            (Some((first, second)), _, _) => constrain(Constraint::Parallel { first, second }),
            (None, [segment], [axis]) => constrain(Constraint::AxisParallel {
                segment: *segment,
                axis: *axis,
            }),
            _ => None,
        },
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
        Rule::Tangent => {
            // A trait and a curve have no order of their own: the clicks give
            // it, and the one clicked first is the one that stays.
            let from = match picks.first() {
                Some(RulePick::Element(Element::Segment(_))) => LaidFrom::Trait,
                _ => LaidFrom::Curve,
            };
            match (
                circles.as_slice(),
                segments.as_slice(),
                arcs.as_slice(),
                ellipses.as_slice(),
            ) {
                ([circle], [segment], [], []) => constrain(Constraint::Tangent {
                    at: None,
                    circle: *circle,
                    segment: *segment,
                    from,
                }),
                ([], [segment], [arc], []) => constrain(Constraint::ArcTangent {
                    at: None,
                    arc: *arc,
                    segment: *segment,
                    from,
                }),
                ([], [segment], [], [ellipse]) => constrain(Constraint::EllipseTangent {
                    at: None,
                    ellipse: *ellipse,
                    segment: *segment,
                    from,
                }),
                _ => None,
            }
        }
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
        Rule::Coincident => {
            // A free point comes onto what it is laid on whichever was
            // clicked first: a trait keeps no order for it, and a curve takes
            // it as clicked second, which is what lands it on what is drawn.
            let point_first = matches!(picks.first(), Some(RulePick::Element(Element::Point(_))));
            let from =
                |point: PointId, laid_on: LaidFrom| match (sketch.stands_alone(point), point_first)
                {
                    (true, _) if laid_on == LaidFrom::Trait => LaidFrom::Nowhere,
                    (true, _) | (false, false) => laid_on,
                    (false, true) => LaidFrom::Point,
                };
            match (
                points.as_slice(),
                segments.as_slice(),
                circles.as_slice(),
                arcs.as_slice(),
                ellipses.as_slice(),
            ) {
                ([point], [segment], [], [], []) => constrain(Constraint::OnSegment {
                    point: *point,
                    segment: *segment,
                    from: from(*point, LaidFrom::Trait),
                }),
                ([point], [], [circle], [], []) => constrain(Constraint::OnCircle {
                    point: *point,
                    circle: *circle,
                    from: from(*point, LaidFrom::Curve),
                }),
                ([point], [], [], [arc], []) => constrain(Constraint::OnArc {
                    point: *point,
                    arc: *arc,
                    from: from(*point, LaidFrom::Curve),
                }),
                ([point], [], [], [], [ellipse]) => constrain(Constraint::OnEllipse {
                    point: *point,
                    ellipse: *ellipse,
                    from: from(*point, LaidFrom::Curve),
                }),
                // Two points asked to coincide are one point: the origin is
                // never the one that gives way.
                ([first, second], [], [], [], []) => {
                    let (kept, dropped) = match sketch.is_origin(*second) {
                        true => (*second, *first),
                        false => (*first, *second),
                    };
                    Some(RuleIntent::Merge { kept, dropped })
                }
                _ => None,
            }
        }
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
