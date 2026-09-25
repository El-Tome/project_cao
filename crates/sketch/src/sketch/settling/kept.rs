//! The lines a drag keeps where they lie.
//!
//! A shape pulled by one of its points would rather turn than give: turning it
//! breaks none of its rules, and the solver's corrections for an angle turn
//! traits rather than stretch them. Keeping the direction of every trait a
//! rule of direction ties leaves the shape one way to follow the hand, which
//! is to stretch — and turning becomes what happens only when stretching
//! cannot.

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget};
use crate::sketch::{PointId, SegmentId, Sketch};

/// One line kept: `to` stays `at` along `normal` from `from`, or from the
/// plane's own origin when there is no `from`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Kept {
    pub(crate) from: Option<PointId>,
    pub(crate) to: PointId,
    /// Unit, square to the line kept.
    pub(crate) normal: DVec2,
    pub(crate) at: f64,
    /// The way the line ran when it was kept, which its equation alone cannot
    /// tell from the way back.
    along: DVec2,
    /// The trait whose direction this is, when it is one.
    pub(crate) segment: Option<SegmentId>,
}

impl Kept {
    /// The direction `from` to `to` has now, kept. Nothing for two points on
    /// top of each other, which have none.
    pub(crate) fn direction(
        sketch: &Sketch,
        from: PointId,
        to: PointId,
        segment: Option<SegmentId>,
    ) -> Option<Self> {
        let along = (sketch.point(to) - sketch.point(from)).try_normalize()?;
        Some(Self {
            from: Some(from),
            to,
            normal: along.perp(),
            at: 0.0,
            along,
            segment,
        })
    }

    /// Whether a direction kept has come out the other way round. Its equation
    /// cannot see it — a line is the same line both ways — but a trait that
    /// went through nought to get there has been turned half round.
    pub(crate) fn runs_backwards(&self, sketch: &Sketch) -> bool {
        let Some(from) = self.from else {
            return false;
        };
        (sketch.point(self.to) - sketch.point(from)).dot(self.along) <= 0.0
    }
}

impl Sketch {
    /// The lines the drag under way keeps; none outside one.
    pub(crate) fn kept_lines(&self) -> &[Kept] {
        &self.held.lines
    }

    /// The traits whose direction the drag under way keeps. A trait kept is
    /// never welded rigid to its neighbours: it has to be free to stretch.
    pub(crate) fn kept_segments(&self) -> Vec<SegmentId> {
        self.held
            .lines
            .iter()
            .filter_map(|line| line.segment)
            .collect()
    }

    /// The traits of a shape a rule of direction ties to another: square,
    /// parallel, on one line, an angle between them, or the two axes of an
    /// ellipse, which are square to each other by what an ellipse is.
    ///
    /// A trait held by a length alone is left out: two bars of typed length
    /// hinged together still have to be free to bend.
    pub(crate) fn tied_by_direction(&self, shape: &[PointId]) -> Vec<SegmentId> {
        let mut tied: Vec<SegmentId> = Vec::new();
        for constraint in self.constraints() {
            if let Constraint::Perpendicular { first, second }
            | Constraint::Parallel { first, second }
            | Constraint::Collinear { first, second } = *constraint
            {
                tied.extend([first, second]);
            }
        }
        for dimension in self.dimensions() {
            if let DimensionTarget::Angle { first, second }
            | DimensionTarget::AngleBetween { first, second, .. } = dimension.target
            {
                tied.extend([first, second]);
            }
        }
        for (_, ellipse) in self.live_ellipses() {
            tied.extend([ellipse.first, ellipse.second]);
        }
        tied.sort();
        tied.dedup();
        tied.retain(|segment| {
            !self.is_erased_segment(*segment)
                && self
                    .segments()
                    .get(segment.0)
                    .is_some_and(|line| shape.contains(&line.start) && shape.contains(&line.end))
        });
        tied
    }

    /// What a drag of `dragged` keeps in its shape: the direction of every
    /// trait tied by a rule of direction, and the ray from each arc's centre
    /// to every one of its ends but the one being pulled, so that an arc
    /// opens or closes by the end the hand holds and turns by neither.
    pub(crate) fn lines_kept_in(&self, shape: &[PointId], dragged: PointId) -> Vec<Kept> {
        let mut lines: Vec<Kept> = self
            .tied_by_direction(shape)
            .into_iter()
            .filter_map(|segment| {
                let line = self.segments()[segment.0];
                Kept::direction(self, line.start, line.end, Some(segment))
            })
            .collect();
        for (_, arc) in self.live_arcs() {
            if !shape.contains(&arc.center) {
                continue;
            }
            for end in [arc.start, arc.end] {
                if end != dragged {
                    lines.extend(Kept::direction(self, arc.center, end, None));
                }
            }
        }
        lines
    }
}
