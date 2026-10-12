//! A point and a trait made to coincide in the order they were clicked: the
//! first one stays where it is, the second comes to it, and nothing else
//! moves (#548). A point and an arc, a circle or an ellipse land the same way
//! (#554); how a curve comes, and where a point lands on one, is
//! `point_on_curve.rs`.
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
use super::point_on_curve::curve_element;
use crate::constraints::Constraint;
use crate::laid_from::LaidFrom;
use crate::resizing::Curved;
use crate::sketch::Element;

/// What a point is laid on.
#[derive(Clone, Copy, Debug)]
enum LaidOn {
    Trait(SegmentId),
    Curve(Curved),
}

impl LaidOn {
    fn element(self) -> Element {
        match self {
            Self::Trait(segment) => Element::Segment(segment),
            Self::Curve(curve) => curve_element(curve),
        }
    }
}

/// One way of placing the second thing clicked before the drawing settles.
#[derive(Clone, Copy, Debug)]
enum Placing {
    /// Nothing placed: the solver brings it, the first held.
    AsItIs,
    /// Nothing placed: the rule lands the way every rule does, the first held
    /// with the ways up the drawing keeps, and is kept only if the first did
    /// stay.
    Ordinarily,
    /// The trait slid square to itself until its line passes through the
    /// point, its direction and its length kept.
    SlidAcross,
    /// The trait turned about one of its ends until its line passes through
    /// the point, its length kept: when sliding it would squeeze its own
    /// shape.
    TurnedAbout(PointId),
    /// The curve carried whole until it passes through the point — alone, or
    /// with the whole shape it belongs to when alone it would bend what it is
    /// joined to — and lengthened past the point when the point falls beyond
    /// what is drawn, or else carried until what is drawn reaches it.
    CurveBrought {
        with_its_shape: bool,
        lengthened: bool,
    },
    /// The point taken where it lands on what it is laid on: square onto a
    /// trait's line, onto what is drawn of a curve clear of its ends.
    Onto,
    /// The point, a centre, taken where it lands with the curves it turns, as
    /// a drag of the centre carries them.
    WithItsCurves,
    /// The point taken where it lands with the whole shape it belongs to.
    ShapeOnto,
}

impl Sketch {
    /// Lands a point laid on a trait or a curve in the order of the clicks,
    /// when it was laid in one: nothing when it was not — a point born on what
    /// holds it, a free point on a trait, a rule a compaction lays again on a
    /// drawing that already holds it. When the second thing clicked cannot
    /// come — the trait is fixed, say — the first comes to it; when neither
    /// can, the rule is refused rather than anything else moving. Nothing
    /// either for a point the curve stands on.
    pub(in crate::sketch) fn meet_in_order(
        &mut self,
        rule: Constraint,
        millimeters_per_unit: f64,
    ) -> Option<LengthOutcome> {
        let (point, on, from) = match rule {
            Constraint::OnSegment {
                point,
                segment,
                from,
            } => (point, LaidOn::Trait(segment), from),
            Constraint::OnCircle {
                point,
                circle,
                from,
            } => (point, LaidOn::Curve(Curved::Circle(circle)), from),
            Constraint::OnArc { point, arc, from } => {
                (point, LaidOn::Curve(Curved::Arc(arc)), from)
            }
            Constraint::OnEllipse {
                point,
                ellipse,
                from,
            } => (point, LaidOn::Curve(Curved::Ellipse(ellipse)), from),
            _ => return None,
        };
        if let LaidOn::Curve(_) = on
            && self.points_it_leans_on(on.element()).contains(&point)
        {
            return None;
        }
        let laid_on = match on {
            LaidOn::Trait(_) => LaidFrom::Trait,
            LaidOn::Curve(_) => LaidFrom::Curve,
        };
        let order = match (on, from) {
            (_, LaidFrom::Nowhere) => return None,
            (_, LaidFrom::Point) => [LaidFrom::Point, laid_on],
            (LaidOn::Trait(_), LaidFrom::Trait) | (LaidOn::Curve(_), LaidFrom::Curve) => {
                [laid_on, LaidFrom::Point]
            }
            (LaidOn::Trait(_), LaidFrom::Curve) | (LaidOn::Curve(_), LaidFrom::Trait) => {
                return None;
            }
        };
        Some(self.landing_or_giving(millimeters_per_unit, |sketch| {
            let met = order
                .into_iter()
                .any(|first| sketch.meet(point, on, first, millimeters_per_unit));
            match met {
                true => LengthOutcome::Exact,
                false => LengthOutcome::BestEffort,
            }
        }))
    }

    /// Brings the second thing clicked to the first, each way it can be
    /// placed in turn, the first held. `first` is the point, or what it is
    /// laid on. Whether one landed; when not, the drawing is as it was.
    fn meet(
        &mut self,
        point: PointId,
        on: LaidOn,
        first: LaidFrom,
        millimeters_per_unit: f64,
    ) -> bool {
        let element = on.element();
        let own = self.points_it_leans_on(element);
        let (placings, stays) = match (first, on) {
            (LaidFrom::Point, LaidOn::Trait(segment)) => {
                let side = self.segments()[segment.0];
                let (near, far) = match self.point(side.start).distance(self.point(point))
                    <= self.point(side.end).distance(self.point(point))
                {
                    true => (side.start, side.end),
                    false => (side.end, side.start),
                };
                let placings = vec![
                    Placing::SlidAcross,
                    Placing::AsItIs,
                    Placing::Ordinarily,
                    Placing::TurnedAbout(far),
                    Placing::TurnedAbout(near),
                ];
                (placings, vec![point])
            }
            // A curve comes whole or not at all: left to the solver, it would
            // grow about a centre that cannot move rather than come.
            (LaidFrom::Point, LaidOn::Curve(_)) => {
                let brought = |with_its_shape, lengthened| Placing::CurveBrought {
                    with_its_shape,
                    lengthened,
                };
                let placings = vec![
                    brought(false, true),
                    brought(true, true),
                    brought(false, false),
                    brought(true, false),
                ];
                (placings, vec![point])
            }
            _ => {
                let placings = vec![
                    Placing::WithItsCurves,
                    Placing::Onto,
                    Placing::ShapeOnto,
                    Placing::AsItIs,
                    Placing::Ordinarily,
                ];
                (placings, own.clone())
            }
        };
        let twice = self.held_twice();
        // Read before anything is placed: a point held on what comes keeps its
        // place along it, not along where the placing put it.
        let mut not_shared = stays.clone();
        not_shared.push(point);
        not_shared.extend(own);
        not_shared.extend(self.points_held_on(element));
        for placing in placings {
            if let Placing::Ordinarily = placing {
                if self.land_keeping(&stays, first, point, on, millimeters_per_unit) {
                    return true;
                }
                continue;
            }
            for loosened in [Vec::new(), twice.clone()] {
                let kept = self.shapes_now();
                let shares = self.shares(&not_shared, millimeters_per_unit);
                let Some(mut held) = self.placed(placing, point, on) else {
                    self.give_back(kept);
                    break;
                };
                if held.iter().any(|placed| stays.contains(placed)) {
                    self.give_back(kept);
                    break;
                }
                held.extend(stays.iter().copied());
                self.held.loosened = loosened;
                if let Element::Circle(circle) = element {
                    self.held.sizes.push(circle);
                }
                let landed = self.settle_held(held.clone(), Vec::new(), millimeters_per_unit)
                    && self.lies_on(point, on);
                self.held.loosened.clear();
                if landed {
                    // The point where it landed, and a circle's size, stay
                    // while the points held elsewhere are put back.
                    held.push(point);
                    self.keep_shares(&shares, &held, &[], millimeters_per_unit);
                }
                self.held.sizes.clear();
                if landed {
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
    /// when the first did stay — a circle its size as well as its centre: the
    /// ordinary way lets it go when it must, and here the other order is tried
    /// before that.
    fn land_keeping(
        &mut self,
        stays: &[PointId],
        first: LaidFrom,
        point: PointId,
        on: LaidOn,
        millimeters_per_unit: f64,
    ) -> bool {
        let kept = self.shapes_now();
        let before: Vec<DVec2> = stays.iter().map(|point| self.point(*point)).collect();
        let held = match first {
            LaidFrom::Point => Element::Point(point),
            _ => on.element(),
        };
        let size = |sketch: &Self| match held {
            Element::Circle(circle) => sketch.circle(circle).radius,
            _ => 0.0,
        };
        let size_before = size(self);
        let still = self.drawing_size() * 1e-9;
        let landed = self.land_held(Some(held), None, millimeters_per_unit) == LengthOutcome::Exact
            && self.lies_on(point, on)
            && (size(self) - size_before).abs() <= still
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
    fn placed(&mut self, placing: Placing, point: PointId, on: LaidOn) -> Option<Vec<PointId>> {
        match (placing, on) {
            (Placing::AsItIs | Placing::Ordinarily, _) => Some(Vec::new()),
            (Placing::SlidAcross, LaidOn::Trait(segment)) => {
                let side = self.segments()[segment.0];
                if [side.start, side.end]
                    .iter()
                    .any(|end| self.is_held(Element::Point(*end)))
                {
                    return None;
                }
                let step = self.point(point) - self.foot_on_segment(point, segment)?;
                for end in [side.start, side.end] {
                    self.move_point(end, self.point(end) + step);
                }
                Some(vec![side.start, side.end])
            }
            (Placing::TurnedAbout(pivot), LaidOn::Trait(segment)) => {
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
            (
                Placing::CurveBrought {
                    with_its_shape,
                    lengthened,
                },
                LaidOn::Curve(curve),
            ) => self.bring_curve(curve, point, with_its_shape, lengthened),
            (Placing::ShapeOnto, _) => {
                let step = self.landing(point, on)? - self.point(point);
                let own = self.points_it_leans_on(on.element());
                self.shape_carried(&[point], &own, (point, own[0]), step)
            }
            (Placing::Onto, _) => {
                if self.is_held(Element::Point(point)) {
                    return None;
                }
                let landing = self.landing(point, on)?;
                self.move_point(point, landing);
                Some(vec![point])
            }
            (Placing::WithItsCurves, _) => {
                if self.is_held(Element::Point(point)) {
                    return None;
                }
                let step: DVec2 = self.landing(point, on)? - self.point(point);
                let mut carried = self.carry_curves_about(point, step, [point, point]);
                if carried.is_empty() {
                    return None;
                }
                self.move_point(point, self.point(point) + step);
                carried.push(point);
                Some(carried)
            }
            (
                Placing::SlidAcross | Placing::TurnedAbout(_) | Placing::CurveBrought { .. },
                LaidOn::Trait(_) | LaidOn::Curve(_),
            ) => None,
        }
    }

    /// Whether the point stands on what it is laid on once the drawing has
    /// settled: on a curve, on what is drawn of it. The rule holds the whole
    /// curve, and the solver alone lands the point anywhere on it — or
    /// nowhere, from the curve's very centre, where no way leads out to it and
    /// the rule is let go of.
    fn lies_on(&self, point: PointId, on: LaidOn) -> bool {
        match on {
            LaidOn::Trait(_) => true,
            LaidOn::Curve(curve) => self.on_what_is_drawn(curve, self.point(point)),
        }
    }

    /// Where a point comes to stand on what it is laid on: square onto a
    /// trait's line, onto what is drawn of a curve clear of its ends.
    fn landing(&self, point: PointId, on: LaidOn) -> Option<DVec2> {
        match on {
            LaidOn::Trait(segment) => self.foot_on_segment(point, segment),
            LaidOn::Curve(curve) => Some(self.landing_on(curve, self.point(point))),
        }
    }
}
