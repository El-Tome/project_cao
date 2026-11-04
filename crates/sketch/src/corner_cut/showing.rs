//! What shows of a side once the cuts on its ends have taken their part: a
//! trait, an arc or the stretch of an ellipse, from where they leave it.

use std::f64::consts::TAU;

use glam::DVec2;

use super::{CutSide, PulledEnds};
use crate::arc::ArcId;
use crate::arcing::sweep_of;
use crate::ellipse::EllipseId;
use crate::sketch::Sketch;

/// A curve side split by the cuts on its ends: the stretch that shows, and
/// the stretch a cut hides up to the sharp corner at its start, then at its
/// end — each as the angle or the turn it starts at and how far round it
/// runs, the way the curve runs. A side the cuts leave nothing of is hidden
/// whole, as the stretch at its start.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stretches {
    pub shows: Option<(f64, f64)>,
    pub hidden: [Option<(f64, f64)>; 2],
}

impl Sketch {
    /// The two ends of what shows of a side, the way it runs — nothing when
    /// the cuts on its ends leave none of it.
    pub(crate) fn what_shows(&self, side: CutSide, pulled: &PulledEnds) -> Option<(DVec2, DVec2)> {
        let shows = match side {
            CutSide::Trait(segment) => self.visible_ends(segment, pulled).is_some(),
            CutSide::Arc(arc) => self.arc_stretches(arc, pulled).shows.is_some(),
            CutSide::ArcOfEllipse(ellipse) => {
                self.ellipse_stretches(ellipse, pulled).shows.is_some()
            }
        };
        let [start, end] = self.side_ends(side)?.map(|point| self.point(point));
        let [from, to] = pulled.of(side);
        shows.then(|| (from.unwrap_or(start), to.unwrap_or(end)))
    }

    /// An arc split by the cuts on its ends, as angles round its centre,
    /// counter-clockwise.
    pub fn arc_stretches(&self, arc: ArcId, pulled: &PulledEnds) -> Stretches {
        let drawn = self.arc_draft(arc);
        let angle = |place: DVec2| (place - drawn.centre).to_angle();
        split(
            angle(drawn.start),
            sweep_of(drawn),
            pulled.of(arc.into()).map(|end| end.map(angle)),
        )
    }

    /// The stretch of an ellipse split by the cuts on its ends, as turns
    /// round it: the whole of it for an ellipse no cut has taken a stretch
    /// out of, which no fillet stands beside.
    pub fn ellipse_stretches(&self, ellipse: EllipseId, pulled: &PulledEnds) -> Stretches {
        let drawn = self.ellipse_draft(ellipse);
        let (starts, sweep) = self.ellipse_run(ellipse);
        split(
            starts,
            sweep,
            pulled
                .of(ellipse.into())
                .map(|end| end.map(|place| drawn.turn_on(place))),
        )
    }
}

/// A run starting at the turn `starts` and running `sweep` round, split once
/// its start and its end are pulled to the turns given.
///
/// A turn pulled a hair behind the run's start is read as behind it, not as
/// most of a turn on. A run no cut pulls is left exactly as it is, and one
/// they leave nothing of is hidden whole.
fn split(starts: f64, sweep: f64, pulled: [Option<f64>; 2]) -> Stretches {
    if pulled == [None, None] {
        return Stretches {
            shows: Some((starts, sweep)),
            hidden: [None, None],
        };
    }
    let slack = (TAU - sweep) * 0.5;
    let along = |turn: f64| (turn - starts + slack).rem_euclid(TAU) - slack;
    let from = pulled[0].map_or(0.0, along);
    let to = pulled[1].map_or(sweep, along);
    if to - from <= crate::precision::SAME * sweep {
        return Stretches {
            shows: None,
            hidden: [Some((starts, sweep)), None],
        };
    }
    let hidden = [
        (pulled[0].is_some(), (starts, from)),
        (pulled[1].is_some(), (starts + to, sweep - to)),
    ]
    .map(|(pulled, run)| (pulled && run.1 > 0.0).then_some(run));
    Stretches {
        shows: Some((starts + from, to - from)),
        hidden,
    }
}
