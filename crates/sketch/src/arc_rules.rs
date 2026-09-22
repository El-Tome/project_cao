//! The bookkeeping an arc-specific rule or dimension needs from `Sketch`,
//! kept apart so it does not crowd out `sketch.rs`: whether a rule still
//! speaks of something drawn, and what a not-yet-placed radius or sweep is
//! worth right now.

use crate::arc::ArcId;
use crate::constraints::Constraint;
use crate::sketch::Sketch;

impl Sketch {
    pub(crate) fn arc_rule_holds_up(&self, constraint: Constraint) -> bool {
        let arc = |id: ArcId| id.0 < self.arcs().len() && !self.is_erased_arc(id);
        match constraint {
            Constraint::EqualRadiusArc { first, second } => {
                first != second && arc(first) && arc(second)
            }
            Constraint::EqualRadiusArcCircle { arc: bent, circle } => {
                arc(bent) && circle.0 < self.circles().len() && !self.is_erased_circle(circle)
            }
            Constraint::ArcTangent {
                arc: curve,
                segment,
                ..
            } => {
                arc(curve) && segment.0 < self.segments().len() && !self.is_erased_segment(segment)
            }
            _ => false,
        }
    }

    /// What a radius dimension not yet placed on this arc would be worth, in
    /// millimetres — `None` for a broken reference.
    pub(crate) fn arc_radius_value(&self, id: ArcId, millimeters_per_unit: f64) -> Option<f64> {
        self.arcs().get(id.0)?;
        Some(self.arc_radius(id) * millimeters_per_unit.max(1e-9))
    }

    /// The same for a swept-angle dimension, in degrees.
    pub(crate) fn arc_sweep_value(&self, id: ArcId) -> Option<f64> {
        self.arcs().get(id.0)?;
        Some(self.arc_sweep(id).to_degrees())
    }
}
