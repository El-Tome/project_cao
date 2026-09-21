//! What a cut would take out of the drawing, as against what a tool would lay.

use glam::DVec2;

use super::arc_carrying;
use super::carrying::{Piece, still_holds, still_measured};
use crate::arc::ArcId;
use crate::arcing::ArcDraft;
use crate::constraints::{Constraint, DimensionTarget};
use crate::sketch::{PointId, SegmentId, Sketch};

/// A place squarely inside any piece, so that what is *fastened* to a place on
/// the trait is proposed to every piece rather than to none. Which piece could
/// have taken it is not the question here; whether the drawing still carries
/// it is.
const ANYWHERE_ALONG: Option<f64> = Some(0.5);

/// What a cut would take out of the drawing.
///
/// The counterpart of [`crate::Preview`]: that one says what a click would
/// add, this one what it would take away. Both are read off the very call the
/// click commits, run on a copy, so neither can say what the click would not.
#[derive(Clone, Debug, PartialEq)]
pub struct Going {
    pub stretch: Stretch,
    /// Whether what goes is construction geometry, and is drawn dashed like
    /// the trait it is a piece of.
    pub construction: bool,
    /// The rules no piece inherits, which go with the stretch.
    pub rules: Vec<Constraint>,
    /// The values that measure neither piece, which go with it too.
    pub values: Vec<DimensionTarget>,
}

/// The run of drawing a cut would take out.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stretch {
    Straight { from: DVec2, to: DVec2 },
    Curved(ArcDraft),
}

impl Sketch {
    /// What a cut would take out of a trait, leaving this drawing untouched.
    ///
    /// Nothing when the cut cannot be made, so what is shown never promises a
    /// click that would be refused.
    pub fn trim_takes(&self, segment: SegmentId, from: PointId, to: PointId) -> Option<Going> {
        let mut trial = self.clone();
        trial.trim(segment, from, to)?;
        Some(Going {
            stretch: Stretch::Straight {
                from: self.points().get(from.0).copied()?,
                to: self.points().get(to.0).copied()?,
            },
            construction: self.segments().get(segment.0)?.construction,
            rules: self.rules_gone(&trial, |rule| {
                pieces_of(&trial).any(|piece| {
                    still_holds(rule, segment, &piece, ANYWHERE_ALONG)
                        .is_some_and(|moved| carries(&trial, moved))
                })
            }),
            values: self.values_gone(&trial, |value| {
                pieces_of(&trial).any(|piece| {
                    still_measured(value, segment, &piece, ANYWHERE_ALONG)
                        .is_some_and(|moved| measures(&trial, moved))
                })
            }),
        })
    }

    /// The same for a curve: the stretch that goes sweeps the way the arc
    /// itself does, from whichever of the two points comes first round it.
    pub fn arc_trim_takes(&self, arc: ArcId, from: PointId, to: PointId) -> Option<Going> {
        let mut trial = self.clone();
        trial.trim_arc(arc, from, to)?;
        let (opens, closes) =
            match self.point_round_the_arc(arc, from)? <= self.point_round_the_arc(arc, to)? {
                true => (from, to),
                false => (to, from),
            };
        Some(Going {
            stretch: Stretch::Curved(ArcDraft {
                centre: self.point(self.arcs().get(arc.0)?.center),
                start: self.points().get(opens.0).copied()?,
                end: self.points().get(closes.0).copied()?,
            }),
            construction: self.arcs().get(arc.0)?.construction,
            rules: self.rules_gone(&trial, |rule| {
                arcs_of(&trial).any(|piece| {
                    arc_carrying::still_holds(rule, arc, &piece, ANYWHERE_ALONG)
                        .is_some_and(|moved| carries(&trial, moved))
                })
            }),
            values: self.values_gone(&trial, |value| {
                arcs_of(&trial).any(|piece| {
                    arc_carrying::still_measured(value, arc, &piece)
                        .is_some_and(|moved| measures(&trial, moved))
                })
            }),
        })
    }

    /// The rules this drawing carries that the trial carries under no name at
    /// all. One handed over to a piece still holds the drawing after the
    /// click, and calling it lost would be a lie.
    fn rules_gone(
        &self,
        trial: &Sketch,
        handed_over: impl Fn(Constraint) -> bool,
    ) -> Vec<Constraint> {
        self.constraints()
            .iter()
            .copied()
            .filter(|rule| !carries(trial, *rule))
            .filter(|rule| !handed_over(*rule))
            .collect()
    }

    /// The same reckoning for what the drawing measures.
    fn values_gone(
        &self,
        trial: &Sketch,
        handed_over: impl Fn(DimensionTarget) -> bool,
    ) -> Vec<DimensionTarget> {
        self.dimensions()
            .iter()
            .map(|value| value.target)
            .filter(|value| !measures(trial, *value))
            .filter(|value| !handed_over(*value))
            .collect()
    }
}

/// Every trait the trial still draws, each offered as the piece the cut could
/// have handed something over to. Which one took it does not matter: the
/// drawing itself answers, by carrying the rule or not.
fn pieces_of(trial: &Sketch) -> impl Iterator<Item = Piece> + '_ {
    trial.live_segments().map(|(id, _)| Piece {
        id,
        spans: (0.0, 1.0),
        reaches_the_corner: true,
    })
}

fn arcs_of(trial: &Sketch) -> impl Iterator<Item = arc_carrying::Piece> + '_ {
    trial.live_arcs().map(|(id, _)| arc_carrying::Piece {
        id,
        spans: (0.0, 1.0),
        carries_the_reach: true,
    })
}

fn carries(sketch: &Sketch, rule: Constraint) -> bool {
    sketch.constraints().contains(&rule.normalised())
}

fn measures(sketch: &Sketch, target: DimensionTarget) -> bool {
    let target = target.normalised();
    sketch
        .dimensions()
        .iter()
        .any(|value| value.target == target)
}

#[cfg(test)]
mod tests;
