//! Dropping a point where two traits cross, and cutting both of them there.

use glam::DVec2;

use crate::arc::ArcId;
use crate::edges::off_by;
use crate::naming::{Became, CurveId};
use crate::sketch::{PointId, SegmentId, Sketch};

/// What a division left behind: the point it dropped at the crossing, the
/// pieces the traits it cut were left as, and what the cut cost.
///
/// A division takes no stretch away, but it does take the traits themselves:
/// a rule or a value that spoke of a whole trait and of neither of its pieces
/// goes with it, exactly as under a cut.
#[derive(Clone, Debug, PartialEq)]
pub struct Split {
    pub point: PointId,
    pub pieces: Vec<SegmentId>,
    /// What the curves cut there were left as, around the centres they already
    /// turned about.
    pub arc_pieces: Vec<ArcId>,
    /// Which pieces came out of which trait, which the two lists above run
    /// together.
    pub became: Became,
    pub rules_dropped: usize,
    pub values_dropped: usize,
}

/// What stands at the place a click fell on, for a division to act upon.
#[derive(Clone, Debug, PartialEq)]
pub enum Crossing {
    /// Traits and arcs running through the crossing with no point of the
    /// drawing standing there, and the place they run through. A division cuts
    /// every one of them in two.
    Curves {
        at: DVec2,
        segments: Vec<SegmentId>,
        arcs: Vec<ArcId>,
    },
    /// A circle runs through the crossing. A circle has no ends: one point
    /// divides it into nothing at all, and the two a division of it would want
    /// are another gesture entirely. Cutting the rest of the crossing and
    /// leaving the circle round would say something nobody asked for, so the
    /// whole crossing is refused.
    Round,
}

impl Sketch {
    /// What a click at this place offers a division, within the reach given.
    ///
    /// The nearest crossing rather than the one clicked exactly: the cursor is
    /// magnetised onto a crossing before it gets here, but a drawing whose
    /// magnets are off should still be divisible.
    pub fn crossing_at(&self, at: DVec2, reach: f64) -> Option<Crossing> {
        let place = self
            .crossings()
            .into_iter()
            .filter(|place| place.distance(at) <= reach)
            .min_by(|left, right| {
                left.distance_squared(at)
                    .total_cmp(&right.distance_squared(at))
            })?;

        if self.a_circle_runs_through(place) {
            return Some(Crossing::Round);
        }

        let segments: Vec<SegmentId> = self
            .live_segments()
            .filter(|(id, segment)| !segment.construction && self.runs_through(*id, place))
            .map(|(id, _)| id)
            .collect();
        let arcs: Vec<ArcId> = self
            .live_arcs()
            .filter(|(id, arc)| !arc.construction && self.runs_round(*id, place))
            .map(|(id, _)| id)
            .collect();

        (segments.len() + arcs.len() >= 2).then_some(Crossing::Curves {
            at: place,
            segments,
            arcs,
        })
    }

    /// Whether a circle of the drawing passes through this place.
    ///
    /// Judged by the sweep's own yardstick rather than a figure of its own.
    /// The sweep merges places nearer than that into one vertex, so a crossing
    /// it reports can sit exactly on whichever pair of curves it happened to
    /// see first and a whole `off_by` away from the circle it swallowed. Asked
    /// any more strictly, the division would cut the rest and leave that circle
    /// round.
    ///
    /// Construction circles are left out, as they are left out of the sweep.
    fn a_circle_runs_through(&self, at: DVec2) -> bool {
        let reach = off_by(at);
        self.live_circles().any(|(_, circle)| {
            !circle.construction
                && (at.distance(self.point(circle.center)) - circle.radius).abs() <= reach
        })
    }

    /// Drops a point where the named traits cross and cuts each of them in two
    /// there, so that the crossing becomes something to dimension, to
    /// constrain and to drag.
    ///
    /// Built on a copy and kept only once every trait has been cut, each one
    /// asked again of that copy. What held of a trait before the first cut need
    /// not hold after it: a cut carries a point away with a tangency it drops,
    /// and `trim` cuts a trait a previous pass already erased rather than
    /// refusing it.
    pub fn split(&mut self, segments: &[SegmentId], arcs: &[ArcId], at: DVec2) -> Option<Split> {
        if self.stands_on(at) {
            return None;
        }

        let mut divided = self.clone();
        let point = divided.add_point(at);
        let mut split = Split {
            point,
            pieces: Vec::new(),
            arc_pieces: Vec::new(),
            became: Became::new(),
            rules_dropped: 0,
            values_dropped: 0,
        };
        for segment in segments {
            if !divided.runs_through(*segment, at) {
                return None;
            }
            let cut = divided.trim(*segment, point, point)?;
            split.became.push((
                CurveId::Segment(*segment),
                cut.pieces.iter().copied().map(CurveId::Segment).collect(),
            ));
            split.pieces.extend(cut.pieces);
            split.rules_dropped += cut.rules_dropped;
            split.values_dropped += cut.values_dropped;
        }
        for arc in arcs {
            if !divided.runs_round(*arc, at) {
                return None;
            }
            let cut = divided.trim_arc(*arc, point, point)?;
            split.became.push((
                CurveId::Arc(*arc),
                cut.pieces.iter().copied().map(CurveId::Arc).collect(),
            ));
            split.arc_pieces.extend(cut.pieces);
            split.rules_dropped += cut.rules_dropped;
            split.values_dropped += cut.values_dropped;
        }

        *self = divided;
        Some(split)
    }

    /// Whether the drawing already owns a point at this place. There is then
    /// nothing for a division to drop, and a second point sitting on the first
    /// is one nobody can tell from it.
    fn stands_on(&self, at: DVec2) -> bool {
        self.live_points()
            .any(|(_, place)| place.distance(at) <= off_by(at))
    }

    /// Whether a trait passes through this place, short of either of its own
    /// ends.
    ///
    /// A place level with the trait but past an end is not on it, and one an
    /// end already occupies has nothing to be cut off it.
    fn runs_through(&self, segment: SegmentId, at: DVec2) -> bool {
        if self.is_erased_segment(segment) || segment.0 >= self.segments().len() {
            return false;
        }
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let reach = span.length_squared();
        if reach == 0.0 {
            return false;
        }
        let along = (at - start).dot(span) / reach;
        let near_enough = off_by(at);
        (0.0..=1.0).contains(&along)
            && at.distance(start + span * along) <= near_enough
            && at.distance(start) > near_enough
            && at.distance(end) > near_enough
    }

    /// The same of an arc: whether the curve passes through this place, short
    /// of either of its own ends.
    fn runs_round(&self, arc: ArcId, at: DVec2) -> bool {
        if self.is_erased_arc(arc) || arc.0 >= self.arcs().len() {
            return false;
        }
        let near_enough = off_by(at);
        if self.distance_to_arc(arc, at) > near_enough {
            return false;
        }
        let curve = self.arc(arc);
        at.distance(self.point(curve.start)) > near_enough
            && at.distance(self.point(curve.end)) > near_enough
    }
}

#[cfg(test)]
mod tests;
