use std::collections::HashMap;

use cao_sketch::{
    ArcId, Area, CircleId, Constraint, CurveId, DimensionTarget, Element, PointId, SegmentId,
};

/// Where every element of one sketch landed after being re-emitted, so a later
/// feature (a `Revolve` around a drawn line, say) can translate the id it
/// names.
#[derive(Default)]
pub(super) struct SketchIdMap {
    pub(super) points: HashMap<PointId, PointId>,
    pub(super) segments: HashMap<SegmentId, SegmentId>,
    pub(super) circles: HashMap<CircleId, CircleId>,
    pub(super) arcs: HashMap<ArcId, ArcId>,
}

/// An area's name, said in the numbers the re-emitted sketch uses.
///
/// Compaction hands a sketch new numbers for everything it still holds, so a
/// name written against the old ones would point at another piece of the
/// drawing — or at nothing. A curve the re-emitted sketch does not hold is
/// dropped, which loses the area rather than renaming it to something else.
pub(super) fn remap_area(area: &Area, map: &SketchIdMap) -> Area {
    Area {
        bounds: area
            .bounds
            .iter()
            .filter_map(|curve| match curve {
                CurveId::Segment(id) => map.segments.get(id).copied().map(CurveId::Segment),
                CurveId::Arc(id) => map.arcs.get(id).copied().map(CurveId::Arc),
                CurveId::Circle(id) => map.circles.get(id).copied().map(CurveId::Circle),
            })
            .collect(),
        inside: area.inside,
    }
}

pub(super) fn remap_target(target: DimensionTarget, map: &SketchIdMap) -> DimensionTarget {
    match target {
        DimensionTarget::Length(segment) => DimensionTarget::Length(map.segments[&segment]),
        DimensionTarget::Distance { from, to } => DimensionTarget::Distance {
            from: map.points[&from],
            to: map.points[&to],
        },
        DimensionTarget::Angle { first, second } => DimensionTarget::Angle {
            first: map.segments[&first],
            second: map.segments[&second],
        },
        DimensionTarget::AxisAngle { segment, axis } => DimensionTarget::AxisAngle {
            segment: map.segments[&segment],
            axis,
        },
        DimensionTarget::PointToSegment { point, segment } => DimensionTarget::PointToSegment {
            point: map.points[&point],
            segment: map.segments[&segment],
        },
        DimensionTarget::Projected { from, to, axis } => DimensionTarget::Projected {
            from: map.points[&from],
            to: map.points[&to],
            axis,
        },
        DimensionTarget::Radius(circle) => DimensionTarget::Radius(map.circles[&circle]),
        DimensionTarget::Diameter(circle) => DimensionTarget::Diameter(map.circles[&circle]),
        DimensionTarget::ArcRadius(arc) => DimensionTarget::ArcRadius(map.arcs[&arc]),
        DimensionTarget::ArcSweep(arc) => DimensionTarget::ArcSweep(map.arcs[&arc]),
    }
}

pub(super) fn remap_constraint(constraint: Constraint, map: &SketchIdMap) -> Constraint {
    match constraint {
        Constraint::Perpendicular { first, second } => Constraint::Perpendicular {
            first: map.segments[&first],
            second: map.segments[&second],
        },
        Constraint::Parallel { first, second } => Constraint::Parallel {
            first: map.segments[&first],
            second: map.segments[&second],
        },
        Constraint::Equal { first, second } => Constraint::Equal {
            first: map.segments[&first],
            second: map.segments[&second],
        },
        Constraint::EqualRadius { first, second } => Constraint::EqualRadius {
            first: map.circles[&first],
            second: map.circles[&second],
        },
        Constraint::EqualRadiusArc { first, second } => Constraint::EqualRadiusArc {
            first: map.arcs[&first],
            second: map.arcs[&second],
        },
        Constraint::OnSegment { point, segment } => Constraint::OnSegment {
            point: map.points[&point],
            segment: map.segments[&segment],
        },
        Constraint::Collinear { first, second } => Constraint::Collinear {
            first: map.segments[&first],
            second: map.segments[&second],
        },
        Constraint::Tangent {
            circle,
            segment,
            at,
        } => Constraint::Tangent {
            circle: map.circles[&circle],
            segment: map.segments[&segment],
            at: at.map(|point| map.points[&point]),
        },
        Constraint::ArcTangent { arc, segment, at } => Constraint::ArcTangent {
            arc: map.arcs[&arc],
            segment: map.segments[&segment],
            at: at.map(|point| map.points[&point]),
        },
        Constraint::OnCircle { point, circle } => Constraint::OnCircle {
            point: map.points[&point],
            circle: map.circles[&circle],
        },
        Constraint::Midpoint { point, segment } => Constraint::Midpoint {
            point: map.points[&point],
            segment: map.segments[&segment],
        },
        Constraint::AxisCollinear { segment, axis } => Constraint::AxisCollinear {
            segment: map.segments[&segment],
            axis,
        },
        Constraint::Fixed { element } => Constraint::Fixed {
            element: remap_element(element, map),
        },
    }
}

fn remap_element(element: Element, map: &SketchIdMap) -> Element {
    match element {
        Element::Point(point) => Element::Point(map.points[&point]),
        Element::Segment(segment) => Element::Segment(map.segments[&segment]),
        Element::Circle(circle) => Element::Circle(map.circles[&circle]),
        Element::Arc(arc) => Element::Arc(map.arcs[&arc]),
    }
}
