//! What a side or a curve pulled by the hand asks of the drawing: to travel
//! sideways, to be drawn to another size, or to turn.
//!
//! The gesture decides, and it decides at every instant: pulled across, a side
//! travels and a curve grows or shrinks; slid along itself, the shape it
//! belongs to turns. Across and along are read about the place the shape would
//! turn on, the way the place grabbed goes round it — so a turn past a quarter
//! stays a turn, and a pull straight out stays a pull.

use glam::DVec2;

use crate::resizing::Curved;
use crate::sketch::{PointId, SegmentId, Sketch};
use crate::snap::SnapSettings;
use crate::turning::{Turn, angle_onto_grid};

/// How much plainer a turn has to be than a pull before a gesture turns: a
/// pull across that drifts sideways keeps resizing until the drift is twice
/// the pull. A turn nobody meant costs more than a resize nobody meant.
pub(crate) const TURN_BIAS: f64 = 2.0;

/// What a side pulled asks for.
#[derive(Clone, Debug, PartialEq)]
pub enum SideDrag {
    /// Travel by this much: square to the side, or wherever the hand went
    /// for a trait on its own.
    Across {
        by: DVec2,
    },
    Along(Turn),
}

/// What a curve pulled asks for.
#[derive(Clone, Debug, PartialEq)]
pub enum CurveDrag {
    /// Drawn to another size about its centre, as #375 has it.
    Resize { reach: f64 },
    /// Turned with its shape, and drawn to the size that brings the place
    /// grabbed under the hand: `reach` in [`Sketch::resize`]'s measure.
    Along { turn: Turn, reach: f64 },
}

/// What a press takes hold of to pull, when it took hold of no point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pulled {
    Side(SegmentId),
    Curve(Curved),
}

impl Sketch {
    /// The side or the curve a press takes hold of, whichever lies nearer.
    ///
    /// An ellipse's axis is the ellipse's own and is not a side, and a side
    /// whose two ends can no longer move has nowhere to go: a press on it
    /// still pulls a box, as it always did.
    pub fn pulled_at(&self, place: DVec2, reach: f64, millimeters_per_unit: f64) -> Option<Pulled> {
        let settled = self.settled_points(millimeters_per_unit);
        let pinned = self.pinned_points();
        let stuck = |point: PointId| {
            settled.get(point.0).copied().unwrap_or(false)
                || pinned.get(point.0).copied().unwrap_or(false)
        };
        let side = self
            .nearest_segment(place, reach)
            .filter(|side| self.ellipse_of_axis(*side).is_none())
            .filter(|side| {
                let line = self.segments()[side.0];
                !(stuck(line.start) && stuck(line.end))
            })
            .map(|side| (Pulled::Side(side), self.off_the_side(side, place)));
        let curve = self
            .curve_at(place, reach)
            .map(|curve| (Pulled::Curve(curve), self.off_the_curve(curve, place)));
        [side, curve]
            .into_iter()
            .flatten()
            .min_by(|one, other| one.1.total_cmp(&other.1))
            .map(|(pulled, _)| pulled)
    }

    fn off_the_side(&self, side: SegmentId, place: DVec2) -> f64 {
        let (start, end) = self.endpoints(side);
        let span = end - start;
        let along = ((place - start).dot(span) / span.length_squared().max(1e-18)).clamp(0.0, 1.0);
        place.distance(start + span * along)
    }

    fn off_the_curve(&self, curve: Curved, place: DVec2) -> f64 {
        match curve {
            Curved::Circle(circle) => {
                let round = self.circle(circle);
                (self.point(round.center).distance(place) - round.radius).abs()
            }
            Curved::Arc(arc) => self.distance_to_arc(arc, place),
            Curved::Ellipse(ellipse) => self.distance_to_ellipse(ellipse, place),
        }
    }

    /// What pulling `side` from `pressed` to `cursor` asks for. Both are the
    /// hand's own places, before any magnet: pulled back onto the side it was
    /// pressed on, a pull would read as a slide.
    ///
    /// A trait on its own travels whole, wherever the hand takes it. A shape
    /// with no place to turn about, or whose place lies on the side's own
    /// line, has no along: it only travels sideways.
    pub fn side_drag(
        &self,
        side: SegmentId,
        pressed: DVec2,
        cursor: DVec2,
        grid: &SnapSettings,
        millimeters_per_unit: f64,
    ) -> SideDrag {
        let Some(line) = self.segments().get(side.0).copied() else {
            return SideDrag::Across { by: DVec2::ZERO };
        };
        let (start, end) = (self.point(line.start), self.point(line.end));
        let Some(along) = (end - start).try_normalize() else {
            return SideDrag::Across { by: DVec2::ZERO };
        };
        let normal = along.perp();
        let across = SideDrag::Across {
            by: normal * (cursor - pressed).dot(normal),
        };

        let shape = self.shape_through(&[line.start, line.end]);
        if self.travels_whole(&shape, start, normal) {
            return SideDrag::Across {
                by: cursor - pressed,
            };
        }
        let Some(about) = self.turning_centre(&shape, side, millimeters_per_unit) else {
            return across;
        };
        if ((about - start).dot(normal)).abs() <= self.drawing_size() * 1e-6 {
            return across;
        }
        // A turn has to be asked for plainly, one of two ways. The hand going
        // along the side more than twice as far as across it reads a slide,
        // wherever the side was pressed. Or the place grabbed left more than
        // twice as far from the hand by a pull across — short by its slip
        // along the side — as by a turn, short by how far the hand went out
        // from or in towards the place it turns on: a hand going round, however
        // far round. A pull straight across slips nothing and always resizes.
        let travel = cursor - pressed;
        let slides = travel.dot(along).abs() > TURN_BIAS * travel.dot(normal).abs();
        let across_slip = travel.dot(along).abs();
        let along_slip = (cursor.distance(about) - pressed.distance(about)).abs();
        if !slides && across_slip <= TURN_BIAS * along_slip {
            return across;
        }
        let angle = turned(about, pressed, cursor);
        let end = [line.start, line.end]
            .into_iter()
            .min_by(|one, other| {
                let place = |point: PointId| {
                    about + DVec2::from_angle(angle).rotate(self.point(point) - about)
                };
                place(*one)
                    .distance(cursor)
                    .total_cmp(&place(*other).distance(cursor))
            })
            .map(|point| self.point(point))
            .unwrap_or(start);
        SideDrag::Along(Turn {
            points: shape,
            about,
            angle: angle_onto_grid(about, end, angle, grid),
        })
    }

    /// What pulling `curve` from `pressed` to `cursor` asks for.
    ///
    /// Towards or away from the centre it is drawn to another size, exactly as
    /// before — and the size is read at `snapped`, the cursor as the magnets
    /// left it, as it always was. Round the centre it turns about that centre,
    /// with the shape it belongs to, and is drawn to the size that keeps the
    /// place grabbed under the hand; only a hand gone round more than twice as
    /// far as it went out turns it. A circle looks the same whichever way up
    /// it is, so a circle is always drawn to another size.
    pub fn curve_drag(
        &self,
        curve: Curved,
        pressed: DVec2,
        cursor: DVec2,
        snapped: DVec2,
        grid: &SnapSettings,
        millimeters_per_unit: f64,
    ) -> CurveDrag {
        let resize = CurveDrag::Resize {
            reach: self.reach_through(curve, snapped),
        };
        let (centre, handles) = match curve {
            Curved::Circle(_) => return resize,
            Curved::Arc(arc) => {
                let drawn = self.arc(arc);
                (drawn.center, vec![drawn.start, drawn.end])
            }
            Curved::Ellipse(ellipse) => {
                let [centre, ends @ ..] = self.ellipse_points(ellipse);
                (centre, ends.to_vec())
            }
        };
        let about = self.point(centre);
        let shape = self.shape_through(&handles);
        if !self.turns_freely_about(&shape, about, millimeters_per_unit) {
            return resize;
        }
        let angle = turned(about, pressed, cursor);
        let round = pressed.distance(about) * angle.abs();
        let out = (cursor.distance(about) - pressed.distance(about)).abs();
        if round <= TURN_BIAS * out {
            return resize;
        }
        let reach = self.reach_through(
            curve,
            about + DVec2::from_angle(-angle).rotate(cursor - about),
        );
        let now = self.reach_through(curve, self.point(handles[0]));
        if now < 1e-12 {
            return resize;
        }
        let placed =
            |at: DVec2| about + DVec2::from_angle(angle).rotate(at - about) * (reach / now);
        let end = handles
            .iter()
            .map(|point| self.point(*point))
            .min_by(|one, other| {
                placed(*one)
                    .distance(cursor)
                    .total_cmp(&placed(*other).distance(cursor))
            })
            .unwrap_or(pressed);
        // The grid pulls the end nearest the hand onto a grid point near
        // where it would land, turned and drawn to its size: both the angle
        // and the size that put it there.
        let (angle, reach) = match (grid.node_near(placed(end)), (end - about).try_normalize()) {
            (Some(node), Some(was)) if node.distance(about) > 1e-12 => (
                was.angle_to((node - about).normalize()),
                now * node.distance(about) / end.distance(about),
            ),
            _ => (angle, reach),
        };
        CurveDrag::Along {
            turn: Turn {
                points: shape,
                about,
                angle,
            },
            reach,
        }
    }
}

/// How far round `about` the hand has gone from `pressed` to `cursor`, in
/// radians between a half turn back and a half turn on.
fn turned(about: DVec2, pressed: DVec2, cursor: DVec2) -> f64 {
    match (
        (pressed - about).try_normalize(),
        (cursor - about).try_normalize(),
    ) {
        (Some(from), Some(to)) => from.angle_to(to),
        _ => 0.0,
    }
}

mod travel;

#[cfg(test)]
mod tests;
