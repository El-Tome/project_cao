//! Every ellipse of the drawing, and the turns at which something runs through
//! it.
//!
//! The sibling of [`crate::circle_edges`]: an ellipse has no end either, so it
//! has no vertex, and the face walk turns at vertices. Whole, it is handed to
//! the walk as the closed loop it already is; crossed, it is broken at those
//! turns into runs the walk can turn at.

use glam::DVec2;

use crate::arcing::ArcDraft;
use crate::crossing::ellipse::{
    where_arc_crosses_ellipse, where_circle_crosses_ellipse, where_ellipses_cross,
    where_segment_crosses_ellipse,
};
use crate::ellipse::EllipseId;
use crate::ellipsing::{EllipseDraft, FULL_ELLIPSE_STEPS};
use crate::sketch::Sketch;

/// An ellipse of the drawing, and where the rest of the drawing runs through
/// it — as fractions of a whole turn, in order round it.
pub(crate) struct Oval {
    pub(crate) id: EllipseId,
    pub(crate) drawn: EllipseDraft,
    pub(crate) turns: Vec<f64>,
    /// Where the stretch that is drawn starts and how far it runs, as
    /// fractions of a whole turn — nothing at all for an ellipse no cut has
    /// taken a stretch out of, which is drawn the whole way round.
    pub(crate) run: Option<(f64, f64)>,
}

/// How far off the curve a place may stand and still be taken as on it, read
/// against how far out it stands so the drawing can be measured in anything.
const ON_THE_CURVE: f64 = 1e-9;

/// How far round the turn a place may stand past the end of a run and still be
/// that end, as a fraction of a whole turn.
const ROUND_THE_CURVE: f64 = 1e-9;

impl Oval {
    pub(crate) fn place_at(&self, turn: f64) -> DVec2 {
        self.drawn.at(std::f64::consts::TAU * turn)
    }

    /// The whole loop as a run of places, for an ellipse nothing breaks.
    pub(crate) fn sampled(&self) -> Vec<DVec2> {
        (0..FULL_ELLIPSE_STEPS)
            .map(|step| self.place_at(step as f64 / FULL_ELLIPSE_STEPS as f64))
            .collect()
    }

    fn holds(&self, place: DVec2) -> bool {
        self.drawn.distance(place) <= ON_THE_CURVE * (1.0 + place.abs().max_element())
            && self.holds_the_turn(self.turn_at(place))
    }

    /// Whether a turn falls on the stretch that is drawn.
    pub(crate) fn holds_the_turn(&self, turn: f64) -> bool {
        let Some((from, sweep)) = self.run else {
            return true;
        };
        // With slack at both ends: a crossing that lands on the end a run
        // opens at falls a hair the other side of it half the time, and a run
        // not broken there leaves the ring beside it uncut.
        let along = (turn - from + ROUND_THE_CURVE).rem_euclid(1.0) - ROUND_THE_CURVE;
        (-ROUND_THE_CURVE..=sweep + ROUND_THE_CURVE).contains(&along)
    }

    /// How far round its turn the ellipse stands at a place on it.
    fn turn_at(&self, place: DVec2) -> f64 {
        self.drawn.turn_on(place) / std::f64::consts::TAU
    }
}

impl Sketch {
    pub(crate) fn ovals(&self) -> Vec<Oval> {
        let mut ovals: Vec<Oval> = self
            .live_ellipses()
            .filter(|(_, ellipse)| !ellipse.construction)
            .map(|(id, _)| {
                let (from, sweep) = self.ellipse_run(id);
                let turn = std::f64::consts::TAU;
                Oval {
                    id,
                    drawn: self.ellipse_draft(id),
                    turns: Vec::new(),
                    run: self.ellipse_ends(id).map(|_| (from / turn, sweep / turn)),
                }
            })
            .collect();

        for index in 0..ovals.len() {
            let mut turns = self.turns_through_the_oval(&ovals[index]);
            for (other, far) in ovals.iter().enumerate() {
                if other != index {
                    let found = where_ellipses_cross(ovals[index].drawn, far.drawn);
                    turns.extend(
                        found
                            .into_iter()
                            .filter_map(|(near, there)| far.holds_the_turn(there).then_some(near)),
                    );
                }
            }
            turns.retain(|turn| ovals[index].holds_the_turn(*turn));
            turns.sort_by(f64::total_cmp);
            ovals[index].turns = turns;
        }
        ovals
    }

    /// Where the drawing's straight runs, curves, circles and points run
    /// through the ellipse. Its own points count: one sitting on it is a place
    /// the walk can turn at, exactly as one sitting on a segment is.
    fn turns_through_the_oval(&self, oval: &Oval) -> Vec<f64> {
        let mut turns = Vec::new();
        for (_, segment) in self.live_segments().filter(|(_, it)| !it.construction) {
            let found = where_segment_crosses_ellipse(
                self.point(segment.start),
                self.point(segment.end),
                oval.drawn,
            );
            turns.extend(found.into_iter().map(|(_, turn)| turn));
        }
        for (_, arc) in self.live_arcs().filter(|(_, it)| !it.construction) {
            let drawn = ArcDraft {
                centre: self.point(arc.center),
                start: self.point(arc.start),
                end: self.point(arc.end),
            };
            let found = where_arc_crosses_ellipse(drawn, oval.drawn);
            turns.extend(found.into_iter().map(|(_, turn)| turn));
        }
        for (_, circle) in self.live_circles().filter(|(_, it)| !it.construction) {
            let found =
                where_circle_crosses_ellipse(self.point(circle.center), circle.radius, oval.drawn);
            turns.extend(found.into_iter().map(|(_, turn)| turn));
        }
        // A handle of the ellipse's own stands on it and breaks nothing while
        // it is the ellipse's alone: it is part of the curve rather than
        // something running through it, and a wall raised from a curve broken
        // at its handles would come out in pieces. A trait drawn to one is
        // another matter — that is a junction, and the curve is cut there.
        for (id, place) in self.live_points() {
            if oval.holds(place) && !self.is_a_bare_handle_of(oval.id, id) {
                turns.push(oval.turn_at(place));
            }
        }
        turns
    }
}
