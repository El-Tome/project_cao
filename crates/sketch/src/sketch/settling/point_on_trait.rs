//! A point and a trait made to coincide in the order they were clicked: the
//! first one stays where it is, the second comes to it, and nothing else
//! moves (#548).
//!
//! The solver alone takes the shortest way, which is not always one the
//! drawing can have: a trait brought across to a point beside its own
//! rectangle slides its whole length and squeezes the sides between to
//! nothing. So the second one is placed each way the order allows, and the
//! drawing settles with what was placed and the first held; the ordinary
//! landing is one of those ways, kept only if the first did stay. When no
//! way lands, the other order is tried — the second could not come — and
//! only then do fixed points give, as for every rule.

use glam::DVec2;

use super::super::{LengthOutcome, PointId, SegmentId, Sketch};
use crate::constraints::Constraint;
use crate::laid_from::LaidFrom;
use crate::sketch::Element;

/// One way of placing the second thing clicked before the drawing settles.
#[derive(Clone, Copy, Debug)]
enum Placing {
    /// Nothing placed: the solver brings it, the first held.
    AsItIs,
    /// Nothing placed: the rule lands the way every rule does, the first held
    /// with the ways up the drawing keeps, and is kept only if the first did
    /// stay.
    Ordinarily,
    /// The trait turned about one of its ends until its line passes through
    /// the point, its length kept.
    TurnedAbout(PointId),
    /// The point taken square onto the trait's line.
    OntoTheLine,
    /// The point, a centre, taken square onto the trait's line with the curves
    /// it turns, as a drag of the centre carries them.
    WithItsCurves,
}

impl Sketch {
    /// Lands a point laid on a trait in the order of the clicks, when it was
    /// laid in one: nothing when it was not. When the second thing clicked
    /// cannot come — the trait is fixed, say — the first comes to it; when
    /// neither can, the rule is refused rather than anything else moving.
    pub(in crate::sketch) fn meet_in_order(
        &mut self,
        rule: Constraint,
        millimeters_per_unit: f64,
    ) -> Option<LengthOutcome> {
        let Constraint::OnSegment {
            point,
            segment,
            from,
        } = rule
        else {
            return None;
        };
        let reversed = match from {
            LaidFrom::Point => LaidFrom::Trait,
            LaidFrom::Trait => LaidFrom::Point,
            LaidFrom::Nowhere | LaidFrom::Curve => return None,
        };
        Some(self.landing_or_giving(millimeters_per_unit, |sketch| {
            let met = [from, reversed]
                .into_iter()
                .any(|first| sketch.meet(point, segment, first, millimeters_per_unit));
            match met {
                true => LengthOutcome::Exact,
                false => LengthOutcome::BestEffort,
            }
        }))
    }

    /// Brings the second thing clicked to the first, each way it can be
    /// placed in turn, the first held. Whether one landed; when not, the
    /// drawing is as it was.
    fn meet(
        &mut self,
        point: PointId,
        segment: SegmentId,
        first: LaidFrom,
        millimeters_per_unit: f64,
    ) -> bool {
        let side = self.segments()[segment.0];
        let (placings, stays) = match first {
            LaidFrom::Point => {
                let (near, far) = match self.point(side.start).distance(self.point(point))
                    <= self.point(side.end).distance(self.point(point))
                {
                    true => (side.start, side.end),
                    false => (side.end, side.start),
                };
                let placings = vec![
                    Placing::AsItIs,
                    Placing::Ordinarily,
                    Placing::TurnedAbout(far),
                    Placing::TurnedAbout(near),
                ];
                (placings, vec![point])
            }
            _ => {
                let placings = vec![
                    Placing::WithItsCurves,
                    Placing::OntoTheLine,
                    Placing::AsItIs,
                    Placing::Ordinarily,
                ];
                (placings, vec![side.start, side.end])
            }
        };
        let twice = self.held_twice();
        // Read before anything is placed: a point held on the trait that
        // comes keeps its place along it, not along where the placing put it.
        let placeable = [point, side.start, side.end];
        let mut not_shared = stays.clone();
        not_shared.extend(placeable);
        for placing in placings {
            if let Placing::Ordinarily = placing {
                if self.land_keeping(&stays, first, point, segment, millimeters_per_unit) {
                    return true;
                }
                continue;
            }
            for loosened in [Vec::new(), twice.clone()] {
                let kept = self.shapes_now();
                let shares = self.shares(&not_shared, millimeters_per_unit);
                let Some(mut held) = self.placed(placing, point, segment) else {
                    self.give_back(kept);
                    break;
                };
                if held.iter().any(|placed| stays.contains(placed)) {
                    self.give_back(kept);
                    break;
                }
                held.extend(stays.iter().copied());
                self.held.loosened = loosened;
                let landed = self.settle_held(held.clone(), Vec::new(), millimeters_per_unit);
                self.held.loosened.clear();
                if landed {
                    self.keep_shares(&shares, &held, &[], millimeters_per_unit);
                    return true;
                }
                self.give_back(kept);
                if twice.is_empty() {
                    break;
                }
            }
        }
        false
    }

    /// Lands the rule the ordinary way, the first one held, and keeps it only
    /// when the first did stay: the ordinary way lets it go when it must,
    /// and here the other order is tried before that.
    fn land_keeping(
        &mut self,
        stays: &[PointId],
        first: LaidFrom,
        point: PointId,
        segment: SegmentId,
        millimeters_per_unit: f64,
    ) -> bool {
        let kept = self.shapes_now();
        let before: Vec<DVec2> = stays.iter().map(|point| self.point(*point)).collect();
        let held = match first {
            LaidFrom::Point => Element::Point(point),
            _ => Element::Segment(segment),
        };
        let still = self.drawing_size() * 1e-9;
        let landed = self.land_held(Some(held), None, millimeters_per_unit) == LengthOutcome::Exact
            && stays
                .iter()
                .zip(&before)
                .all(|(point, was)| self.point(*point).distance(*was) <= still);
        if !landed {
            self.give_back(kept);
        }
        landed
    }

    /// Places the second thing clicked one way, and says what was placed, to
    /// be held while the drawing settles. Nothing when that way does not
    /// apply here.
    fn placed(
        &mut self,
        placing: Placing,
        point: PointId,
        segment: SegmentId,
    ) -> Option<Vec<PointId>> {
        match placing {
            Placing::AsItIs | Placing::Ordinarily => Some(Vec::new()),
            Placing::TurnedAbout(pivot) => {
                let side = self.segments()[segment.0];
                let other = match side.start == pivot {
                    true => side.end,
                    false => side.start,
                };
                if self.is_held(Element::Point(pivot)) || self.is_held(Element::Point(other)) {
                    return None;
                }
                let (from, to) = (self.point(pivot), self.point(point));
                let toward = (to - from).try_normalize()?;
                let reach = self.point(other) - from;
                let along = match reach.dot(toward) < 0.0 {
                    true => -reach.length(),
                    false => reach.length(),
                };
                self.move_point(other, from + toward * along);
                Some(vec![pivot, other])
            }
            Placing::OntoTheLine => {
                if self.is_held(Element::Point(point)) {
                    return None;
                }
                let foot = self.foot_on_segment(point, segment)?;
                self.move_point(point, foot);
                Some(vec![point])
            }
            Placing::WithItsCurves => {
                if self.is_held(Element::Point(point)) {
                    return None;
                }
                let foot = self.foot_on_segment(point, segment)?;
                let step: DVec2 = foot - self.point(point);
                let mut carried = self.carry_curves_about(point, step, [point, point]);
                if carried.is_empty() {
                    return None;
                }
                self.move_point(point, foot);
                carried.push(point);
                Some(carried)
            }
        }
    }
}
