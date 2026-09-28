//! A rule laid, or an angle typed, between two things: the one clicked first
//! stays where it is, and the second comes to it.

use super::{LengthOutcome, Sketch};
use crate::constraints::{Constraint, DimensionTarget};
use crate::equation::Equation;
use crate::sketch::Element;

impl Sketch {
    /// Lays a rule down and settles the drawing into it, the thing it was laid
    /// from held where it is.
    pub fn lay_rule(&mut self, rule: Constraint, millimeters_per_unit: f64) -> LengthOutcome {
        self.add_constraint(rule);
        self.land(rule.laid_from(), millimeters_per_unit)
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
        self.land(from, millimeters_per_unit)
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
