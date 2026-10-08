//! A rule laid, or an angle typed, between two things: the one clicked first
//! stays where it is, and the second comes to it.

use super::{Kept, LengthOutcome, Sketch};
use crate::constraints::{Constraint, DimensionTarget};
use crate::equation::Equation;
use crate::laid_from::LaidFrom;
use crate::resizing::Curved;
use crate::sketch::Element;

impl Sketch {
    /// Lays a rule down and settles the drawing into it, the thing it was laid
    /// from held where it is.
    pub fn lay_rule(&mut self, rule: Constraint, millimeters_per_unit: f64) -> LengthOutcome {
        if !self.carries(rule) && self.holds_up(rule) {
            self.bring_alongside(rule);
            self.turn_onto_the_axis(rule);
        }
        self.add_constraint(rule);
        self.land(rule.laid_from(), None, millimeters_per_unit)
    }

    /// A curve beside the end of a trait touches the trait's line, never the
    /// trait: holding the first clicked, nothing would bring the other along
    /// the trait, and the tangency could not land. The second one clicked is
    /// slid along the trait first, just far enough for the curve's centre to
    /// stand over it — that is part of coming to the first.
    fn bring_alongside(&mut self, rule: Constraint) {
        let (curve, element, segment, from) = match rule {
            Constraint::Tangent {
                circle,
                segment,
                from,
                ..
            } => (
                Curved::Circle(circle),
                Element::Circle(circle),
                segment,
                from,
            ),
            Constraint::ArcTangent {
                arc, segment, from, ..
            } => (Curved::Arc(arc), Element::Arc(arc), segment, from),
            Constraint::EllipseTangent {
                ellipse,
                segment,
                from,
                ..
            } => (
                Curved::Ellipse(ellipse),
                Element::Ellipse(ellipse),
                segment,
                from,
            ),
            _ => return,
        };
        let (start, end) = self.endpoints(segment);
        let span = end - start;
        let length_squared = span.length_squared();
        if length_squared < 1e-18 {
            return;
        }
        let along = (self.centre_of(curve) - start).dot(span) / length_squared;
        let past = along - along.clamp(0.0, 1.0);
        if past == 0.0 {
            return;
        }
        let (moving, shift) = match from {
            LaidFrom::Trait => (element, -span * past),
            LaidFrom::Curve => (Element::Segment(segment), span * past),
            LaidFrom::Nowhere => return,
        };
        for point in self.points_it_leans_on(moving) {
            self.move_point(point, self.point(point) + shift);
        }
    }

    /// Settles the drawing into a value just typed, the trait an angle was
    /// typed from held where it is — as it is each time the value is typed
    /// again, whichever way round it was clicked the second time.
    pub fn land_value(
        &mut self,
        target: DimensionTarget,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let from = self
            .dimensions()
            .iter()
            .find(|dimension| dimension.target.is_the_same_as(target))
            .and_then(|dimension| dimension.target.laid_from());
        self.land(from, Some(target), millimeters_per_unit)
    }

    /// Held only while it lands: once settled, the two are as free as the rule
    /// leaves them, and a value typed later on either reaches the other through
    /// it. Where holding it leaves the rule no way to hold — the thing clicked
    /// second is fixed, or outranks the clicks some other way — the drawing is
    /// put back and settles as if nothing had been clicked first.
    ///
    /// Fixed points outrank the clicks, and give only when neither way lands.
    pub(in crate::sketch) fn land(
        &mut self,
        from: Option<Element>,
        target: Option<DimensionTarget>,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        self.landing_or_giving(millimeters_per_unit, |sketch| {
            sketch.land_held(from, target, millimeters_per_unit)
        })
    }

    fn land_held(
        &mut self,
        from: Option<Element>,
        target: Option<DimensionTarget>,
        millimeters_per_unit: f64,
    ) -> LengthOutcome {
        let Some(from) = from else {
            return self.land_near(target, millimeters_per_unit);
        };
        let held = self.points_it_leans_on(from);
        let square = self.ways_up_kept();
        let tries = [
            (held.clone(), square.clone()),
            (Vec::new(), square),
            (held, Vec::new()),
        ];
        for (held, lines) in tries
            .into_iter()
            .filter(|(held, lines)| !held.is_empty() || !lines.is_empty())
        {
            let kept = self.shapes_now();
            let shares = self.shares(&held, millimeters_per_unit);
            if let Element::Circle(circle) = from {
                self.held.sizes.push(circle);
            }
            let landed = self.settle_held(held.clone(), lines, millimeters_per_unit);
            self.held.sizes.clear();
            if landed {
                self.keep_shares(&shares, &held, &[], millimeters_per_unit);
                return LengthOutcome::Exact;
            }
            self.give_back(kept);
        }
        self.resolve_keeping_places(millimeters_per_unit)
    }

    /// The direction of a trait along an axis in the shape drawn from the
    /// origin, kept while a rule lands: that shape keeps lying along the
    /// origin's axes, and only the hand turns it off them (#471). Tried before
    /// the clicks are, and given up when the rule cannot hold without turning
    /// it — an angle typed against an axis still does. A shape drawn away from
    /// the origin is left to the clicks (#451).
    fn ways_up_kept(&self) -> Vec<Kept> {
        let groups = self.point_groups();
        self.traits_lying_square(&groups)
            .into_iter()
            .filter(|(owner, _)| *owner == groups[Sketch::ORIGIN.0])
            .filter_map(|(_, (start, end))| Kept::direction(self, start, end, None))
            .collect()
    }

    /// Takes the correction away from the sizes of the circles held while a
    /// rule lands: a pin speaks of a point, and a circle's size is not one.
    pub(crate) fn hold_sizes(&self, equation: &mut Equation) {
        for circle in &self.held.sizes {
            if let Some(column) = self.radius_column(*circle) {
                equation.gradient[column] = 0.0;
            }
        }
    }
}
