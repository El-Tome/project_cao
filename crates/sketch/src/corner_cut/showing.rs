//! What shows of a side once the cuts on its ends have taken their part: a
//! trait, an arc or the stretch of an ellipse, from where they leave it.

use std::f64::consts::TAU;

use glam::DVec2;

use super::{CutSide, PulledEnds};
use crate::arc::ArcId;
use crate::arcing::sweep_of;
use crate::ellipse::EllipseId;
use crate::sketch::Sketch;

impl Sketch {
    /// The two ends of what shows of a side, the way it runs — nothing when
    /// the cuts on its ends leave none of it.
    pub(crate) fn what_shows(&self, side: CutSide, pulled: &PulledEnds) -> Option<(DVec2, DVec2)> {
        let shows = match side {
            CutSide::Trait(segment) => self.visible_ends(segment, pulled).is_some(),
            CutSide::Arc(arc) => self.arc_shows(arc, pulled).is_some(),
            CutSide::ArcOfEllipse(ellipse) => self.stretch_shows(ellipse, pulled).is_some(),
        };
        let [start, end] = self.side_ends(side)?.map(|point| self.point(point));
        let [from, to] = pulled.of(side);
        shows.then(|| (from.unwrap_or(start), to.unwrap_or(end)))
    }

    /// What shows of an arc, as the angle it starts at and how far round it
    /// runs counter-clockwise.
    pub(crate) fn arc_shows(&self, arc: ArcId, pulled: &PulledEnds) -> Option<(f64, f64)> {
        let drawn = self.arc_draft(arc);
        let angle = |place: DVec2| (place - drawn.centre).to_angle();
        shown(
            angle(drawn.start),
            sweep_of(drawn),
            pulled.of(arc.into()).map(|end| end.map(angle)),
        )
    }

    /// What shows of the stretch of an ellipse, as the turn it starts at and
    /// how far round it runs: the whole of it for an ellipse no cut has taken
    /// a stretch out of, which no fillet stands beside.
    pub(crate) fn stretch_shows(
        &self,
        ellipse: EllipseId,
        pulled: &PulledEnds,
    ) -> Option<(f64, f64)> {
        let drawn = self.ellipse_draft(ellipse);
        let (starts, sweep) = self.ellipse_run(ellipse);
        shown(
            starts,
            sweep,
            pulled
                .of(ellipse.into())
                .map(|end| end.map(|place| drawn.turn_on(place))),
        )
    }
}

/// What is left of a run starting at the turn `starts` and running `sweep`
/// round, once its start and its end are pulled to the turns given: where it
/// starts and how far it runs — nothing when nothing is left.
///
/// A turn pulled a hair behind the run's start is read as behind it, not as
/// most of a turn on. A run no cut pulls is left exactly as it is.
fn shown(starts: f64, sweep: f64, pulled: [Option<f64>; 2]) -> Option<(f64, f64)> {
    if pulled == [None, None] {
        return Some((starts, sweep));
    }
    let slack = (TAU - sweep) * 0.5;
    let along = |turn: f64| (turn - starts + slack).rem_euclid(TAU) - slack;
    let from = pulled[0].map_or(0.0, along);
    let to = pulled[1].map_or(sweep, along);
    (to - from > crate::precision::SAME * sweep).then_some((starts + from, to - from))
}
