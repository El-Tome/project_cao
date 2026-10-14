//! A shape turned whole, as a hand would turn it, before the solver sees an
//! angle typed or a rule of direction. Left to the solver, the turn spreads
//! over every free point: a shape comes out stretched, and blown up when the
//! value folds at a half turn (#532).

use std::f64::consts::{FRAC_PI_2, PI};

use glam::DVec2;

use super::joins::Tie;
use super::{LengthOutcome, PointId, Sketch};
use crate::constraints::{Constraint, DimensionTarget, Toward};
use crate::corner::SideTurned;
use crate::holding::Support;
use crate::sketch::{Element, SegmentId};

/// What lands, for what it says about turning.
#[derive(Clone, Copy, Debug)]
pub(in crate::sketch) enum Landing {
    /// A value typed into the dimension on this target.
    Value(DimensionTarget),
    /// A rule laid.
    Rule(Constraint),
    /// Points made one, or a point brought onto what holds it: nothing says
    /// how far anything turns.
    Nothing,
}

impl Landing {
    pub(in crate::sketch) fn target(self) -> Option<DimensionTarget> {
        match self {
            Self::Value(target) => Some(target),
            Self::Rule(_) | Self::Nothing => None,
        }
    }
}

/// How a try turns the shape that gives way: whole and held at the place it
/// turned about, whole and let go of there, or not at all.
#[derive(Clone, Copy, Debug)]
pub(super) enum Turned {
    Pinned,
    Loose,
    Not,
}

impl Turned {
    /// The order they are tried in. Turned and held is what a hand would do;
    /// let go, the shape may still slide where something else places it; and
    /// where neither lands, the solver has the drawing as it lay, as it did
    /// before #532 — the thing clicked first never gives for the turn.
    pub(super) const IN_ORDER: [Self; 3] = [Self::Pinned, Self::Loose, Self::Not];
}

/// A trait to turn, how far, what its shape is walked no further than, and
/// the cut corner it is a side of when it turns against the other side.
struct Turning {
    turns: SegmentId,
    stops: Vec<PointId>,
    cut: Option<SideTurned>,
    angle: f64,
    meeting: Option<DVec2>,
}

impl Sketch {
    /// Lands an angle typed against an axis with the trait's shape turned
    /// whole first, the solver mending only what is left. Nothing when nothing
    /// turned, or when what is left cannot be mended that way: the drawing is
    /// then put back as it was.
    pub(super) fn land_turned(
        &mut self,
        landing: Landing,
        millimeters_per_unit: f64,
    ) -> Option<LengthOutcome> {
        for how in [Turned::Pinned, Turned::Loose] {
            let kept = self.shapes_now();
            let stays = self.turned(landing, how, &[])?;
            self.held.stays.clone_from(&stays);
            let shares = self.shares(&[], millimeters_per_unit);
            let landed = self.settle_held(Vec::new(), Vec::new(), millimeters_per_unit);
            self.held.stays.clear();
            if landed {
                self.keep_shares(&shares, &stays, &[], millimeters_per_unit);
                return Some(LengthOutcome::Exact);
            }
            self.give_back(kept);
        }
        None
    }

    /// Turns the shape of what gives way to `landing` — the trait an angle is
    /// typed against an axis, the trait clicked second of two — as `how` says,
    /// walked no further than the points of `kept` besides, the traits a try
    /// keeps the direction of. Says which points stay while the drawing
    /// settles. Nothing when there is nothing to turn, so that a try turned is
    /// not run again for nothing.
    pub(super) fn turned(
        &mut self,
        landing: Landing,
        how: Turned,
        kept: &[PointId],
    ) -> Option<Vec<PointId>> {
        if let Turned::Not = how {
            return Some(Vec::new());
        }
        let turning = self.turning(landing)?;
        let mut stops = turning.stops;
        stops.extend_from_slice(kept);
        stops.extend(self.held_in_direction(landing.target()));
        let walls: &[PointId] = turning.cut.as_ref().map_or(&[], |cut| &cut.cut);
        let about =
            self.turn_whole(turning.turns, &stops, walls, turning.angle, turning.meeting)?;
        if let Some(cut) = &turning.cut {
            let corners = self.cut_corners();
            self.lay_cut_again(&cut.corner, &corners);
        }
        Some(match how {
            Turned::Pinned => about
                .into_iter()
                .chain(turning.cut.map(|cut| cut.free_end))
                .collect(),
            Turned::Loose | Turned::Not => Vec::new(),
        })
    }

    /// The cut corner `landing` turns one side of against the other, when it
    /// is laid between the two sides of one.
    pub(super) fn side_turned_by(&self, landing: Landing) -> Option<SideTurned> {
        self.turning(landing)?.cut
    }

    /// Turns the shape `turns` belongs to by `angle`, walked no further than
    /// what holds it: what the drawing keeps still — the origin, a fixed
    /// point — and `stops`. The walk never takes in `walls`, which neither
    /// turn nor hold it.
    ///
    /// It turns about the one place it is held at. Held nowhere, it turns
    /// about `meeting` when there is one, or else about its point nearest the
    /// origin, as a value typed ranks them (#455). Held at two places, it
    /// cannot turn whole, and is left to the solver. Says which point it
    /// turned about, when it was one.
    pub(super) fn turn_whole(
        &mut self,
        turns: SegmentId,
        stops: &[PointId],
        walls: &[PointId],
        angle: f64,
        meeting: Option<DVec2>,
    ) -> Option<Vec<PointId>> {
        if !angle.is_finite() || angle.abs() < 1e-12 {
            return None;
        }
        let (shape, holds) = self.walked_to_holds(turns, stops, walls);
        let (about, pivot) = match holds.split_first() {
            Some((first, rest)) => {
                let at = self.point(*first);
                if rest
                    .iter()
                    .any(|hold| self.point(*hold).distance(at) > 1e-9)
                {
                    return None;
                }
                (at, Some(*first))
            }
            None => match meeting {
                Some(place) => (place, None),
                None => {
                    let near = self.nearest_the_origin(&shape)?;
                    (self.point(near), Some(near))
                }
            },
        };
        let turn = DVec2::from_angle(angle);
        for point in shape {
            let place = about + turn.rotate(self.point(point) - about);
            self.move_point(point, place);
        }
        Some(pivot.into_iter().collect())
    }

    /// The shape that turns with `turns`: what is drawn joined to it, what a
    /// tangency ties to it and what is held on it, however far round — but
    /// never what a point of it is held on. The walk goes no further than a
    /// point the drawing keeps still or one of `stops`, and a point held on
    /// something that does not turn — an axis, a trait the walk never reached
    /// — stays where it is held: those are the places the shape is held at,
    /// and no part of it.
    fn walked_to_holds(
        &self,
        turns: SegmentId,
        stops: &[PointId],
        walls: &[PointId],
    ) -> (Vec<PointId>, Vec<PointId>) {
        let count = self.points().len();
        let mut stop = self.pinned_points();
        for point in stops.iter().filter(|point| point.0 < count) {
            stop[point.0] = true;
        }
        let tied = self.tied_pairs();
        let mut next_to: Vec<Vec<PointId>> = vec![Vec::new(); count];
        for (one, other) in self.drawn_pairs() {
            next_to[one.0].push(other);
            next_to[other.0].push(one);
        }
        for (one, other, tie) in &tied {
            next_to[other.0].push(*one);
            if *tie == Tie::Touching {
                next_to[one.0].push(*other);
            }
        }
        let mut reached = vec![false; count];
        for wall in walls.iter().filter(|wall| wall.0 < count) {
            reached[wall.0] = true;
        }
        let (mut shape, mut holds) = (Vec::new(), Vec::new());
        let mut front = self.points_it_leans_on(Element::Segment(turns));
        while let Some(point) = front.pop() {
            if point.0 >= count || reached[point.0] || self.is_erased_point(point) {
                continue;
            }
            reached[point.0] = true;
            if stop[point.0] {
                holds.push(point);
                continue;
            }
            shape.push(point);
            front.extend(next_to[point.0].iter().copied());
        }
        let rests = |point: &PointId| {
            self.constraints()
                .iter()
                .any(|rule| matches!(rule, Constraint::OnAxis { point: on, .. } if on == point))
                || tied.iter().any(|(one, other, tie)| {
                    one == point && *tie == Tie::HeldOn && !reached[other.0]
                })
        };
        let (resting, mut shape): (Vec<PointId>, Vec<PointId>) = shape.into_iter().partition(rests);
        holds.extend(resting);
        shape.sort_by_key(|point| point.0);
        holds.sort_by_key(|point| point.0);
        (shape, holds)
    }

    /// The points of every trait whose direction is given against the
    /// sketch — by a rule to an axis, or by an angle to one typed, but for
    /// `landing` — which no shape turns past.
    pub(super) fn held_in_direction(&self, landing: Option<DimensionTarget>) -> Vec<PointId> {
        let ruled = self
            .constraints()
            .iter()
            .filter_map(|rule| rule.along_an_axis())
            .map(|(segment, _)| segment);
        let typed = self
            .dimensions()
            .iter()
            .filter(|dimension| !dimension.driven)
            .filter(|dimension| {
                landing.is_none_or(|target| !dimension.target.is_the_same_as(target))
            })
            .filter_map(|dimension| match dimension.target {
                DimensionTarget::AxisAngle { segment, .. } => Some(segment),
                _ => None,
            });
        ruled
            .chain(typed)
            .flat_map(|segment| self.points_it_leans_on(Element::Segment(segment)))
            .collect()
    }

    /// What turns for `landing`, and how far: the trait of an angle typed
    /// against an axis, or the second of two traits an angle or a rule of
    /// direction was laid between.
    fn turning(&self, landing: Landing) -> Option<Turning> {
        let turning = match landing {
            Landing::Value(target) => {
                let dimension = self.dimension_of(target)?;
                if dimension.driven {
                    return None;
                }
                let wanted = dimension.value.to_radians();
                match dimension.target {
                    DimensionTarget::AxisAngle { segment, .. } => {
                        let (_, along, arm) = self.axis_arms(dimension.target)?;
                        Turning {
                            turns: segment,
                            stops: Vec::new(),
                            cut: None,
                            angle: opened_to(along, arm, wanted),
                            meeting: None,
                        }
                    }
                    DimensionTarget::Angle { first, second, .. } => {
                        let (_, one, other) = self.corner_arms(dimension.target)?;
                        self.second_turning(first, second, opened_to(one, other, wanted), None)
                    }
                    DimensionTarget::AngleBetween {
                        first,
                        first_toward,
                        second,
                        second_toward,
                    } => {
                        let (one, other) = (
                            self.arm(first, first_toward),
                            self.arm(second, second_toward),
                        );
                        self.second_turning(
                            first,
                            second,
                            opened_to(one, other, wanted),
                            self.where_traits_meet(first, second),
                        )
                    }
                    _ => return None,
                }
            }
            Landing::Rule(Constraint::Parallel { first, second }) => {
                let wanted = self.arm(first, Toward::End);
                let running = self.arm(second, Toward::End);
                let angle = quarter_turn_at_most(running.angle_to(wanted));
                self.second_turning(first, second, angle, None)
            }
            Landing::Rule(Constraint::Perpendicular { first, second }) => {
                let wanted = self.arm(first, Toward::End).perp();
                let running = self.arm(second, Toward::End);
                let angle = quarter_turn_at_most(running.angle_to(wanted));
                self.second_turning(first, second, angle, None)
            }
            Landing::Rule(_) | Landing::Nothing => return None,
        };
        Some(turning)
    }

    /// The trait clicked second, turned by `angle`: its shape is walked no
    /// further than the trait clicked first, nor than a point held on it —
    /// the foot of a T turns the stem about itself, and the corner a cut left
    /// between them turns the second side about the corner, the cut left to
    /// mend (#565).
    fn second_turning(
        &self,
        first: SegmentId,
        second: SegmentId,
        angle: f64,
        meeting: Option<DVec2>,
    ) -> Turning {
        let mut stops = self.points_it_leans_on(Element::Segment(first));
        stops.extend(
            self.live_points()
                .map(|(point, _)| point)
                .filter(|point| self.holds_on(*point).contains(&Support::Segment(first))),
        );
        Turning {
            turns: second,
            stops,
            cut: self.side_turned(first, second),
            angle,
            meeting,
        }
    }
}

/// The turn that takes `arm` to `wanted` away from `along`, on the side of it
/// the arm stands now: an angle says how wide, never which way round.
fn opened_to(along: DVec2, arm: DVec2, wanted: f64) -> f64 {
    let now = along.perp_dot(arm).atan2(along.dot(arm));
    let side = if now < 0.0 { -1.0 } else { 1.0 };
    side * wanted - now
}

/// The turn, taken the short way: a trait runs the way of a line whichever end
/// leads, so no turn needs to be more than a quarter.
pub(super) fn quarter_turn_at_most(angle: f64) -> f64 {
    let angle = angle.rem_euclid(PI);
    if angle > FRAC_PI_2 { angle - PI } else { angle }
}
