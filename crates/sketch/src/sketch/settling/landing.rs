//! A rule laid, or an angle typed, between two things: the one clicked first
//! stays where it is, and the second comes to it — unless only the first is
//! construction, which then comes to the profile.

use super::{LengthOutcome, Sketch};
use crate::constraints::{Constraint, DimensionTarget};
use crate::equation::Equation;
use crate::resizing::Curved;
use crate::sketch::Element;

impl Sketch {
    /// Lays a rule down and settles the drawing into it, the thing it was laid
    /// from held where it is.
    pub fn lay_rule(&mut self, rule: Constraint, millimeters_per_unit: f64) -> LengthOutcome {
        let stays = self.stays(rule.laid_between());
        if !self.carries(rule) && self.holds_up(rule) {
            self.bring_alongside(rule, stays);
        }
        self.add_constraint(rule);
        self.land(stays, millimeters_per_unit)
    }

    /// Which of the two things a rule or an angle was laid between stays: the
    /// one clicked first, unless it is construction and the other is not —
    /// scaffolding gives before the profile it helps to draw, whichever order
    /// the clicks came in.
    fn stays(&self, between: Option<(Element, Element)>) -> Option<Element> {
        let (first, second) = between?;
        match self.is_construction(first) && !self.is_construction(second) {
            true => Some(second),
            false => Some(first),
        }
    }

    /// A curve beside the end of a trait touches the trait's line, never the
    /// trait: holding the first clicked, nothing would bring the other along
    /// the trait, and the tangency could not land. The one that does not stay
    /// is slid along the trait first, just far enough for the curve's centre
    /// to stand over it — that is part of coming to the other.
    fn bring_alongside(&mut self, rule: Constraint, stays: Option<Element>) {
        let (curve, element, segment) = match rule {
            Constraint::Tangent {
                circle, segment, ..
            } => (Curved::Circle(circle), Element::Circle(circle), segment),
            Constraint::ArcTangent { arc, segment, .. } => {
                (Curved::Arc(arc), Element::Arc(arc), segment)
            }
            Constraint::EllipseTangent {
                ellipse, segment, ..
            } => (Curved::Ellipse(ellipse), Element::Ellipse(ellipse), segment),
            _ => return,
        };
        let Some(stays) = stays else {
            return;
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
        let (moving, shift) = match stays == element {
            false => (element, -span * past),
            true => (Element::Segment(segment), span * past),
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
            .and_then(|dimension| dimension.target.laid_between());
        self.land(self.stays(from), millimeters_per_unit)
    }

    /// Held only while it lands: once settled, the two are as free as the rule
    /// leaves them, and a value typed later on either reaches the other through
    /// it. Where holding it leaves the rule no way to hold — the thing clicked
    /// second is fixed, or outranks the clicks some other way — the drawing is
    /// put back and settles as if nothing had been clicked first.
    fn land(&mut self, from: Option<Element>, millimeters_per_unit: f64) -> LengthOutcome {
        let Some(from) = from else {
            return self.resolve_keeping_places(millimeters_per_unit);
        };
        let held = self.points_it_leans_on(from);
        let kept = self.shapes_now();
        let shares = self.shares(&held, millimeters_per_unit);
        if let Element::Circle(circle) = from {
            self.held.sizes.push(circle);
        }
        let landed = self.settle_held(held.clone(), Vec::new(), millimeters_per_unit);
        self.held.sizes.clear();
        if !landed {
            self.give_back(kept);
            return self.resolve_keeping_places(millimeters_per_unit);
        }
        self.keep_shares(&shares, &held, &[], millimeters_per_unit);
        LengthOutcome::Exact
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
