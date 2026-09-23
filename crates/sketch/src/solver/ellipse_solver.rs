//! What being an ellipse asks of the solver, kept apart so it does not crowd
//! out `solver.rs`.

use crate::ellipse::EllipseId;
use crate::equation::Equation;
use crate::sketch::Sketch;

impl Sketch {
    /// What being an ellipse asks of the drawing, for every ellipse.
    pub(crate) fn ellipse_equations(&self, pinned: &[bool], into: &mut Vec<Equation>) {
        for index in 0..self.ellipses().len() {
            self.ellipse_equation(EllipseId(index), pinned, into);
        }
    }

    /// Its two axes square to each other and both halved by its centre.
    ///
    /// Written for every ellipse rather than kept as rules anybody could take
    /// away: five points are ten numbers where an ellipse has five, and a curve
    /// whose axes had come apart would be no ellipse at all.
    pub(crate) fn ellipse_equation(
        &self,
        id: EllipseId,
        pinned: &[bool],
        into: &mut Vec<Equation>,
    ) {
        if self.is_erased_ellipse(id) {
            return;
        }
        let Some(ellipse) = self.ellipses().get(id.0).copied() else {
            return;
        };
        let written = into.len();
        into.extend(self.direction_equation(ellipse.first, ellipse.second, true));
        // An axis laid out from the centre already has it for an end, by the
        // very point it stands on. Asked for a midpoint on top of that, the
        // solver would read "the centre is halfway between itself and the far
        // end" and pull that end onto the centre until the curve was nothing.
        for axis in [ellipse.first, ellipse.second] {
            if !self.axis_stands_on_the_centre(ellipse.center, axis) {
                self.midpoint_equations(ellipse.center, axis, into);
            }
        }
        // The ends a cut left are on the curve and stay on it, the way an
        // arc's ends stay the same reach from its centre.
        //
        // Never an end that is one of the ellipse's own handles: an axis end
        // is on the curve because it is what draws it, so the row would say
        // nothing — and a row that says nothing still asks the solver to
        // divide one almost-nothing by another, which is how the drawing came
        // back a thousand million units across.
        if let Some((from, to)) = ellipse.drawn {
            let handles = self.ellipse_points(id);
            for end in [from, to].into_iter().filter(|end| !handles.contains(end)) {
                into.extend(self.on_ellipse_equation(end, id));
            }
        }
        for equation in &mut into[written..] {
            for (point, pinned) in pinned.iter().enumerate() {
                if *pinned {
                    equation.gradient[point * 2] = 0.0;
                    equation.gradient[point * 2 + 1] = 0.0;
                }
            }
        }
    }
}
