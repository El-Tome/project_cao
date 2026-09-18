//! Every circle of the drawing, and the turns at which something runs through
//! it.
//!
//! A circle has no end, so it has no vertex, and the face walk turns at
//! vertices. As long as nothing crosses one that costs nothing: the loop is
//! exact, and it is what makes two concentric circles a tube. The moment
//! something does cross it, the loop has to be broken at those turns and
//! handed to the walk as arcs, or the drawing answers differently depending on
//! which tool drew the same shape.

use glam::DVec2;

use crate::arcing::{ArcDraft, FULL_CIRCLE_STEPS};
use crate::crossing::{
    turn_at, where_arc_crosses_circle, where_circles_cross, where_segment_crosses_circle,
};
use crate::sketch::{CircleId, Sketch};

/// A circle of the drawing, and where the rest of the drawing runs through it
/// — as fractions of a whole turn, in order round it.
pub(crate) struct Round {
    pub(crate) id: CircleId,
    pub(crate) centre: DVec2,
    pub(crate) radius: f64,
    pub(crate) turns: Vec<f64>,
}

impl Round {
    pub(crate) fn place_at(&self, turn: f64) -> DVec2 {
        self.centre + DVec2::from_angle(std::f64::consts::TAU * turn) * self.radius
    }

    /// The whole loop as a run of places, for a circle nothing breaks.
    pub(crate) fn sampled(&self) -> Vec<DVec2> {
        (0..FULL_CIRCLE_STEPS)
            .map(|step| self.place_at(step as f64 / FULL_CIRCLE_STEPS as f64))
            .collect()
    }

    /// How far off the circle a place can stand and still be taken as on it,
    /// read against how far out it stands so the drawing can be measured in
    /// anything.
    fn holds(&self, place: DVec2) -> bool {
        let off = ON_THE_CIRCLE * (1.0 + place.abs().max_element());
        (place.distance(self.centre) - self.radius).abs() <= off
    }
}

const ON_THE_CIRCLE: f64 = 1e-9;

impl Sketch {
    pub(crate) fn rounds(&self) -> Vec<Round> {
        let mut rounds: Vec<Round> = self
            .live_circles()
            .filter(|(_, circle)| !circle.construction)
            .map(|(id, circle)| Round {
                id,
                centre: self.point(circle.center),
                radius: circle.radius,
                turns: Vec::new(),
            })
            .collect();

        for index in 0..rounds.len() {
            let mut turns = self.turns_through(&rounds[index]);
            for (other, far) in rounds.iter().enumerate() {
                if other != index {
                    let found = where_circles_cross(
                        rounds[index].centre,
                        rounds[index].radius,
                        far.centre,
                        far.radius,
                    );
                    turns.extend(found.into_iter().map(|(near, _)| near));
                }
            }
            turns.sort_by(f64::total_cmp);
            rounds[index].turns = turns;
        }
        rounds
    }

    /// Where the drawing's straight runs, curves and points run through the
    /// circle. Its own points count: one sitting on a circle is a place the
    /// walk can turn at, exactly as one sitting on a segment is.
    fn turns_through(&self, round: &Round) -> Vec<f64> {
        let mut turns = Vec::new();
        for (_, segment) in self.live_segments().filter(|(_, it)| !it.construction) {
            let found = where_segment_crosses_circle(
                self.point(segment.start),
                self.point(segment.end),
                round.centre,
                round.radius,
            );
            turns.extend(found.into_iter().map(|(_, turn)| turn));
        }
        for (_, arc) in self.live_arcs().filter(|(_, it)| !it.construction) {
            let drawn = ArcDraft {
                centre: self.point(arc.center),
                start: self.point(arc.start),
                end: self.point(arc.end),
            };
            let found = where_arc_crosses_circle(drawn, round.centre, round.radius);
            turns.extend(found.into_iter().map(|(_, turn)| turn));
        }
        for (_, place) in self.live_points() {
            if round.holds(place) {
                turns.push(turn_at(round.centre, place));
            }
        }
        turns
    }
}
