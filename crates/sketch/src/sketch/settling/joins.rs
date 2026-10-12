//! The pairs of points a drawing joins — what is drawn as one, and what a rule
//! ties — read for what moves with what: the shape a drag pulls, and the shape
//! an angle typed turns.

use crate::constraints::Constraint;
use crate::sketch::{PointId, SegmentId, Sketch};

/// How a rule ties two points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Tie {
    /// The first is held on what the second stands for: it follows that, and
    /// does not carry it.
    HeldOn,
    /// A curve, or where it touches, against the trait it brushes.
    Touching,
}

impl Sketch {
    /// Every pair of points the drawing joins: the two ends of a trait, the
    /// centre of an arc to each of its ends, an ellipse's centre to the ends
    /// of its axes, a point to what a rule holds it on, and a tangency's
    /// curve and touch to the trait they brush.
    ///
    /// What lies on a trait is joined to both its ends: joined to the start
    /// alone, it would count as nearer the start than the end is, and a drag
    /// of the end would keep it rather than the far end.
    pub(crate) fn joined_pairs(&self) -> Vec<(PointId, PointId)> {
        let mut pairs = self.drawn_pairs();
        pairs.extend(
            self.tied_pairs()
                .into_iter()
                .map(|(one, other, _)| (one, other)),
        );
        pairs
    }

    /// The pairs of points a rule ties, and how: a point to what holds it,
    /// and a tangency's curve and touch to the trait they brush.
    pub(crate) fn tied_pairs(&self) -> Vec<(PointId, PointId, Tie)> {
        let ends_of = |segment: SegmentId| -> Vec<PointId> {
            self.segments()
                .get(segment.0)
                .map(|line| vec![line.start, line.end])
                .unwrap_or_default()
        };
        let circle_centre = |circle: crate::sketch::CircleId| {
            self.circles().get(circle.0).map(|round| round.center)
        };
        let arc_centre = |arc: crate::arc::ArcId| self.arcs().get(arc.0).map(|curve| curve.center);
        let ellipse_centre = |ellipse: crate::ellipse::EllipseId| {
            self.ellipses().get(ellipse.0).map(|curve| curve.center)
        };
        let mut pairs = Vec::new();
        for constraint in self.constraints() {
            let (joined, to, tie): (Vec<PointId>, Vec<PointId>, Tie) = match *constraint {
                Constraint::OnSegment { point, segment, .. }
                | Constraint::Midpoint { point, segment } => {
                    (vec![point], ends_of(segment), Tie::HeldOn)
                }
                Constraint::OnCircle { point, circle, .. } => (
                    vec![point],
                    circle_centre(circle).into_iter().collect(),
                    Tie::HeldOn,
                ),
                Constraint::OnArc { point, arc, .. } => (
                    vec![point],
                    arc_centre(arc).into_iter().collect(),
                    Tie::HeldOn,
                ),
                Constraint::OnEllipse { point, ellipse, .. } => (
                    vec![point],
                    ellipse_centre(ellipse).into_iter().collect(),
                    Tie::HeldOn,
                ),
                Constraint::Tangent {
                    circle,
                    segment,
                    at,
                    ..
                } => (
                    circle_centre(circle).into_iter().chain(at).collect(),
                    ends_of(segment),
                    Tie::Touching,
                ),
                Constraint::ArcTangent {
                    arc, segment, at, ..
                } => (
                    arc_centre(arc).into_iter().chain(at).collect(),
                    ends_of(segment),
                    Tie::Touching,
                ),
                Constraint::EllipseTangent {
                    ellipse,
                    segment,
                    at,
                    ..
                } => (
                    ellipse_centre(ellipse).into_iter().chain(at).collect(),
                    ends_of(segment),
                    Tie::Touching,
                ),
                _ => continue,
            };
            for one in &joined {
                pairs.extend(to.iter().map(|other| (*one, *other, tie)));
            }
        }
        let count = self.points().len();
        pairs.retain(|(one, other, _)| one.0 < count && other.0 < count);
        pairs
    }

    /// The points drawn as one: a trait's two ends, an arc's centre and ends,
    /// an ellipse's centre and the ends of its axes and of its drawn stretch.
    pub(crate) fn drawn_pairs(&self) -> Vec<(PointId, PointId)> {
        let mut pairs: Vec<(PointId, PointId)> = self
            .live_segments()
            .map(|(_, line)| (line.start, line.end))
            .collect();
        for (_, arc) in self.live_arcs() {
            pairs.extend([(arc.center, arc.start), (arc.center, arc.end)]);
        }
        for (id, ellipse) in self.live_ellipses() {
            let [centre, ends @ ..] = self.ellipse_points(id);
            pairs.extend(ends.map(|end| (centre, end)));
            if let Some((from, to)) = ellipse.drawn {
                pairs.extend([(centre, from), (centre, to)]);
            }
        }
        pairs
    }
}
