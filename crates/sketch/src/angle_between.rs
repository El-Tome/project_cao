//! The angle between two traits that meet without sharing an end: where they
//! cross, and where one ends on the middle of the other.
//!
//! A corner's two traits name its one angle, read from the end they share. Two
//! traits that cross name four, and no shared end says which: each arm is taken
//! one way along its trait instead, out from where the two meet.

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget, Toward};
use crate::sketch::{SegmentId, Sketch};

/// Below this the two traits run the same way and never meet, as a fraction of
/// the product of their lengths — the same threshold a crossing is found with.
const PARALLEL: f64 = 1e-12;

/// How near one of its own ends a trait may be met and still be met at that
/// end, as a fraction of its length: the foot of a T. Far wider than rounding,
/// since a solver leaves a foot held on the other trait a hair off it.
const AT_AN_END: f64 = 1e-4;

/// How far past its own ends a trait may be met and still count as met, as a
/// fraction of its length. Only asked of traits nothing holds together: a T
/// whose foot a rule holds on the other trait meets it whatever the solver left
/// between them.
const MEETS_WITHIN: f64 = 1e-9;

impl Sketch {
    /// Where the lines two traits lie on cross — through the traits themselves
    /// or out past their ends. Nothing only for two traits running the same way.
    ///
    /// Where an angle between them is read and drawn. The traits need not
    /// still touch there: the solver leaves a held foot a hair off the other
    /// trait, and an angle that went on turning the drawing after it stopped
    /// being drawn would be the worst of both.
    pub fn where_lines_cross(&self, first: SegmentId, second: SegmentId) -> Option<DVec2> {
        let (a1, _, along, t, _) = self.crossing_of(first, second)?;
        Some(a1 + along * t)
    }

    /// Whether two traits meet: the place both run through, when it lies on
    /// both of them — where they cross, or where one ends on the other.
    ///
    /// Asked when the two are clicked, to offer an angle between them at all.
    /// A shared end is a corner, and a corner is measured as one — this is
    /// asked only once that has been ruled out.
    pub fn where_traits_meet(&self, first: SegmentId, second: SegmentId) -> Option<DVec2> {
        let (a1, _, along, on_first, on_second) = self.crossing_of(first, second)?;
        let meet = a1 + along * on_first;
        if self.has_its_foot_held_on(first, second) || self.has_its_foot_held_on(second, first) {
            return Some(meet);
        }
        let on = |fraction: f64| (-MEETS_WITHIN..=1.0 + MEETS_WITHIN).contains(&fraction);
        (on(on_first) && on(on_second)).then_some(meet)
    }

    /// The two lines' crossing, as the first trait's start and run and how far
    /// along each trait it falls.
    fn crossing_of(
        &self,
        first: SegmentId,
        second: SegmentId,
    ) -> Option<(DVec2, DVec2, DVec2, f64, f64)> {
        self.segments().get(first.0)?;
        self.segments().get(second.0)?;
        let (a1, a2) = self.endpoints(first);
        let (b1, b2) = self.endpoints(second);
        let (along, across) = (a2 - a1, b2 - b1);
        let turn = along.perp_dot(across);
        if turn.abs() < along.length() * across.length() * PARALLEL {
            return None;
        }
        let gap = b1 - a1;
        Some((
            a1,
            a2,
            along,
            gap.perp_dot(across) / turn,
            gap.perp_dot(along) / turn,
        ))
    }

    /// Whether one of a trait's ends is held on another by a rule: the foot of
    /// a T, laid there on purpose.
    fn has_its_foot_held_on(&self, stem: SegmentId, bar: SegmentId) -> bool {
        let drawn = self.segments()[stem.0];
        [drawn.start, drawn.end].into_iter().any(|point| {
            self.constraints().contains(&Constraint::OnSegment {
                point,
                segment: bar,
            })
        })
    }

    /// One arm of an angle: a trait's own direction, run the way asked.
    pub(crate) fn arm(&self, segment: SegmentId, toward: Toward) -> DVec2 {
        let (start, end) = self.endpoints(segment);
        match toward {
            Toward::End => end - start,
            Toward::Start => start - end,
        }
    }

    /// The angle a dimension already reads between these two traits, whichever
    /// way round they were clicked and whichever of their angles it names.
    ///
    /// Two traits that cross carry one angle between them at most: the others
    /// are that one or what it leaves of a half turn, and a second one laid
    /// beside it would sooner or later say something the first does not.
    pub fn angle_already_between(
        &self,
        first: SegmentId,
        second: SegmentId,
    ) -> Option<DimensionTarget> {
        self.dimensions()
            .iter()
            .map(|dimension| dimension.target)
            .find(|target| match *target {
                DimensionTarget::AngleBetween {
                    first: one,
                    second: other,
                    ..
                } => (one, other) == (first, second) || (one, other) == (second, first),
                _ => false,
            })
    }

    /// The same angle, turned to face `placed`: of the angles the two traits
    /// make where their lines cross, the one opening towards where the
    /// dimension is put down.
    ///
    /// A trait crossed through its middle runs both ways from the crossing, and
    /// its arm heads towards that side. A trait met at one of its own ends — the
    /// stem of a T — has one arm only, running away from that end: past it
    /// nothing is drawn, and an angle read towards it would measure empty
    /// space. So an X opens four ways, and a T two.
    ///
    /// Any other target comes back as it was, and so does an angle whose
    /// traits run the same way — there is no side to face without a place to
    /// face it from.
    pub(crate) fn opening_toward(&self, target: DimensionTarget, placed: DVec2) -> DimensionTarget {
        let DimensionTarget::AngleBetween { first, second, .. } = target else {
            return target;
        };
        if let Some(laid) = self.angle_already_between(first, second) {
            return laid;
        }
        let Some((a1, _, along, on_first, on_second)) = self.crossing_of(first, second) else {
            return target;
        };
        let side = placed - (a1 + along * on_first);
        let toward = |segment: SegmentId, other: SegmentId, fraction: f64| {
            self.only_arm(segment, other, fraction).unwrap_or(
                match side.dot(self.arm(segment, Toward::End)) >= 0.0 {
                    true => Toward::End,
                    false => Toward::Start,
                },
            )
        };
        DimensionTarget::AngleBetween {
            first,
            first_toward: toward(first, second, on_first),
            second,
            second_toward: toward(second, first, on_second),
        }
    }

    /// The one arm a trait has when the other is met at one of its own ends,
    /// running away from that end; nothing when it is crossed through its
    /// middle and runs both ways.
    ///
    /// An end a rule holds on the other trait is that end, wherever the solver
    /// left it. Otherwise it is read off where the lines cross, with room for
    /// what a solver leaves: a hair from an end is that end.
    fn only_arm(&self, segment: SegmentId, other: SegmentId, fraction: f64) -> Option<Toward> {
        let drawn = self.segments().get(segment.0)?;
        let held = |point| {
            self.constraints().contains(&Constraint::OnSegment {
                point,
                segment: other,
            })
        };
        match () {
            _ if held(drawn.start) || fraction <= AT_AN_END => Some(Toward::End),
            _ if held(drawn.end) || fraction >= 1.0 - AT_AN_END => Some(Toward::Start),
            _ => None,
        }
    }

    /// What an angle between two traits measures right now, in degrees: the
    /// opening between the two arms it names, never more than a half turn.
    pub fn opening(&self, target: DimensionTarget) -> Option<f64> {
        let DimensionTarget::AngleBetween {
            first,
            first_toward,
            second,
            second_toward,
        } = target
        else {
            return None;
        };
        self.segments().get(first.0)?;
        self.segments().get(second.0)?;
        let (one, other) = (
            self.arm(first, first_toward),
            self.arm(second, second_toward),
        );
        if one.length_squared() == 0.0 || other.length_squared() == 0.0 {
            return None;
        }
        Some(one.angle_to(other).to_degrees().abs())
    }
}
