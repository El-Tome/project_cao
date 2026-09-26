//! A point pulled by the hand: what stays where it is, what keeps its
//! direction, and where the point can go — read once, when the press lands,
//! and then laid down at every place the hand takes it to.
//!
//! The rule it holds is the one a hand expects from a mouse: the shape
//! stretches to follow, the point farthest from the hand stays where it is,
//! and the shape turns only when stretching cannot follow at all.

use glam::DVec2;

use super::give::Give;
use super::kept::Kept;
use crate::constraints::Constraint;
use crate::length::LengthOutcome;
use crate::sketch::{PointId, Sketch};

/// How close to the last place a shape can still follow to the way there is
/// halved, as a share of the drawing's size — a hundred-thousandth, the
/// solver's own precision — and how many halvings that may take at the most.
const REACHED_WITHIN: f64 = 1e-5;
const HALVINGS: usize = 24;
/// The least way along the hand's line, as a share of the drawing's size,
/// that counts as having followed the hand at all.
const PROGRESS: f64 = 1e-3;

mod landing;

/// What a drag of one point may do to the drawing, read from the drawing as
/// the press found it.
#[derive(Clone, Debug)]
pub struct PointPull {
    point: PointId,
    from: DVec2,
    way: Way,
}

#[derive(Clone, Debug)]
enum Way {
    /// The drag as it always was: nothing stays but the point, nothing keeps
    /// its direction. A curve's centre carries its curve that way, a point
    /// held halfway along a trait carries the trait, and a lone point has
    /// nothing to hold.
    Plain,
    Held(Holding),
}

#[derive(Clone, Debug)]
struct Holding {
    pins: Vec<PointId>,
    lines: Vec<Kept>,
    give: Give,
    shape: Vec<PointId>,
    about: Option<PointId>,
    /// Whether the point is held on a curve, where the place the hand asks
    /// for is already on what holds it.
    on_a_curve: bool,
}

impl Sketch {
    /// What a drag of `point` may do, read from the drawing as it stands.
    pub fn pull(&self, point: PointId, millimeters_per_unit: f64) -> PointPull {
        let from = self.points().get(point.0).copied().unwrap_or_default();
        let plain = PointPull {
            point,
            from,
            way: Way::Plain,
        };
        if point.0 >= self.points().len() || self.out_of_play(point) || self.only_a_centre(point) {
            return plain;
        }
        let halfway = self
            .constraints()
            .iter()
            .any(|rule| matches!(rule, Constraint::Midpoint { point: held, .. } if *held == point));
        let shape = self.shape_through(&self.with_its_curves(point));
        if halfway || shape.len() < 2 {
            return plain;
        }

        let pinned = self.pinned_points();
        let fixed: Vec<PointId> = shape
            .iter()
            .copied()
            .filter(|each| pinned[each.0])
            .collect();
        let centres = self.centres_under(point);
        let stay = match fixed.is_empty() && centres.is_empty() {
            true => self.stay_point(point, &shape),
            false => None,
        };
        let about = self.turned_about(&fixed, &centres, stay);
        let on_a_curve = !centres.is_empty();
        let mut pins = centres;
        pins.extend(stay);
        let lines = self.lines_kept_in(&shape, &[point]);
        let give = self.give_of(point, &shape, &pins, &lines, about, millimeters_per_unit);
        PointPull {
            point,
            from,
            way: Way::Held(Holding {
                pins,
                lines,
                give,
                shape,
                about,
                on_a_curve,
            }),
        }
    }

    /// Lays one place of the drag down: the point taken towards `position`
    /// and the drawing settled around it, as `pull` allows.
    ///
    /// `Exact` when the drawing settled, the point wherever it could go;
    /// `BestEffort` when nothing could be had and the drawing is given back
    /// as it was.
    pub fn settle_pulled(
        &mut self,
        pull: &PointPull,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let Way::Held(holding) = &pull.way else {
            return self.settle_plainly(pull.point, position, millimeters_per_unit);
        };
        let from = pull.from;
        let reached = match holding.give {
            // The last place the hand's way can be followed to comes before
            // letting the point go: let go of beyond its reach, the drawing
            // pulls it back wherever it happens to settle, which can be the
            // very shape the press found.
            Give::Free => {
                self.stretch(pull.point, holding, position, millimeters_per_unit)
                    || self.stretch_towards(pull, holding, position, millimeters_per_unit)
                    || self.retract(pull, holding, position, millimeters_per_unit)
            }
            // The one line read at the press is only a tangent when the point
            // is held on a curve: the place the hand asks for, already on the
            // curve, is tried first there — and only there, since off the line
            // it is a place no settling reaches, and finding that out costs
            // every iteration the solver has.
            Give::Along(way) => {
                let landing = from + way * (position - from).dot(way);
                (holding.on_a_curve
                    && self.stretch(pull.point, holding, position, millimeters_per_unit))
                    || self.stretch(pull.point, holding, landing, millimeters_per_unit)
                    || self.stretch_towards(pull, holding, landing, millimeters_per_unit)
                    || self.retract(pull, holding, landing, millimeters_per_unit)
            }
            Give::Nowhere => self.pivot(pull, holding, position, millimeters_per_unit),
            Give::Stuck => false,
        };
        match reached {
            true => LengthOutcome::Exact,
            false => LengthOutcome::BestEffort,
        }
    }

    /// The point held as far towards `landing` as the shape can stretch: the
    /// way there halved until the last place it still follows to.
    fn stretch_towards(
        &mut self,
        pull: &PointPull,
        holding: &Holding,
        landing: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        let (mut reached, mut missed) = (0.0, 1.0);
        let within = self.drawing_size() * REACHED_WITHIN / pull.from.distance(landing).max(1e-12);
        for _ in 0..HALVINGS {
            if missed - reached <= within {
                break;
            }
            let halfway = (reached + missed) / 2.0;
            let place = pull.from.lerp(landing, halfway);
            let kept = self.shapes_now();
            match self.stretch(pull.point, holding, place, millimeters_per_unit) {
                true => reached = halfway,
                false => missed = halfway,
            }
            self.give_back(kept);
        }
        // A hair's breadth past the press is no way at all: the hand's way
        // runs off a curve the point is held to, and letting it go is what
        // brings it round.
        reached * pull.from.distance(landing) > self.drawing_size() * PROGRESS
            && self.stretch(
                pull.point,
                holding,
                pull.from.lerp(landing, reached),
                millimeters_per_unit,
            )
    }

    /// One try: the point held at `landing`, the shape's pins and kept lines
    /// with it. Given back whole when it does not come out exact.
    fn stretch(
        &mut self,
        point: PointId,
        holding: &Holding,
        landing: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        let kept = self.shapes_now();
        self.move_point(point, landing);
        let mut held = holding.pins.clone();
        held.push(point);
        let outcome = self.settle_held(held, holding.lines.clone(), millimeters_per_unit);
        if outcome {
            return true;
        }
        self.give_back(kept);
        false
    }

    /// The point placed where it may go and let go of, the drawing pulling it
    /// back onto whatever it can really reach. What a freedom read off a
    /// curve at the press leaves: the line it gave is only the curve's tangent.
    fn retract(
        &mut self,
        pull: &PointPull,
        holding: &Holding,
        landing: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        let kept = self.shapes_now();
        self.move_point(pull.point, landing);
        if self.settle_held(
            holding.pins.clone(),
            holding.lines.clone(),
            millimeters_per_unit,
        ) {
            return true;
        }
        self.give_back(kept);
        false
    }

    /// The whole shape turned about the place it pivots on, as far round as
    /// the hand has gone, and the rest of the drawing settled around it.
    fn pivot(
        &mut self,
        pull: &PointPull,
        holding: &Holding,
        position: DVec2,
        millimeters_per_unit: f64,
    ) -> bool {
        let Some(about) = holding.about else {
            return false;
        };
        let centre = self.point(about);
        let (Some(was), Some(wanted)) = (
            (pull.from - centre).try_normalize(),
            (position - centre).try_normalize(),
        ) else {
            return false;
        };
        let turn = DVec2::from_angle(was.angle_to(wanted));
        let kept = self.shapes_now();
        for each in &holding.shape {
            let place = centre + turn.rotate(self.point(*each) - centre);
            self.move_point(*each, place);
        }
        if self.settle_held(holding.shape.clone(), Vec::new(), millimeters_per_unit) {
            return true;
        }
        self.give_back(kept);
        false
    }

    /// Settles the drawing with `points` held still and `lines` kept, and
    /// says whether it came out whole: every value true, no trait squeezed to
    /// nothing, no tangency slid off. A kept trait may come out the other way
    /// round: a corner pulled past the opposite one turns its shape inside
    /// out, as the hand asked.
    pub(crate) fn settle_held(
        &mut self,
        points: Vec<PointId>,
        lines: Vec<Kept>,
        millimeters_per_unit: f64,
    ) -> bool {
        self.held.points = points;
        self.held.lines = lines;
        let outcome = self.resolve(millimeters_per_unit);
        self.held.points.clear();
        self.held.lines.clear();
        outcome == LengthOutcome::Exact
            && !self.has_a_collapsed_trait(self.drawing_size())
            && !self.has_a_flipped_tangent()
    }
}
