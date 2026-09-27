//! One row of the system the drawing has to satisfy, and the buffer it is
//! written into.

use std::cell::RefCell;

use glam::DVec2;

use crate::sketch::PointId;

/// Which family a row of the system belongs to.
///
/// The order is the one the equations are written in, and it is read back in
/// more than one place. Naming it once is what stops a second reader from going
/// on believing there are only dimensions and rules.
pub(crate) enum Row {
    /// A line a drag keeps where it lies. First, so that the sweep lays the
    /// kept lines before any rule reads the shape they give it.
    Kept(usize),
    Dimension(usize),
    Rule(usize),
    Arc(usize),
    Ellipse(usize),
}

/// The row an index names, given how many of the first four families there
/// are.
pub(crate) fn row_at(
    index: usize,
    kept: usize,
    dimensions: usize,
    rules: usize,
    arcs: usize,
) -> Row {
    let Some(index) = index.checked_sub(kept) else {
        return Row::Kept(index);
    };
    let Some(past_dimensions) = index.checked_sub(dimensions) else {
        return Row::Dimension(index);
    };
    let Some(past_rules) = past_dimensions.checked_sub(rules) else {
        return Row::Rule(past_dimensions);
    };
    match past_rules.checked_sub(arcs) {
        Some(ellipse) => Row::Ellipse(ellipse),
        None => Row::Arc(past_rules),
    }
}

/// One equation the drawing has to satisfy, linearised around its current
/// shape: how far off it is, and how each coordinate would change that.
///
/// Gradients are analytic rather than sampled: they are short to write for
/// lengths and angles, exact, and the solver runs them hundreds of times.
pub(crate) struct Equation {
    /// Current value minus the wanted one. Zero when satisfied.
    pub error: f64,
    /// Whether that error is an angle rather than a length. An angle is judged
    /// on its own: dividing radians by the size of the drawing let a big
    /// drawing be declared settled with its corners a tenth of a degree out.
    pub angular: bool,
    /// Change of `error` per unit change of each coordinate, laid out as
    /// x0, y0, x1, y1, …
    pub gradient: Vec<f64>,
}

thread_local! {
    /// Gradients the sweep has finished with. One correction wants a buffer
    /// exactly as long as the last one did, four hundred times over, and the
    /// allocator has nothing to add to that. Only the sweep gives one back:
    /// everywhere else an equation is short-lived and freeing it is right.
    static SPARE_GRADIENTS: RefCell<Vec<Vec<f64>>> = const { RefCell::new(Vec::new()) };
}

/// How many buffers are worth keeping. A correction holds three at once at the
/// most — a tangency asks for the largest number of them.
const SPARE_GRADIENTS_KEPT: usize = 4;

impl Equation {
    pub(crate) fn new(variables: usize) -> Self {
        let mut gradient = SPARE_GRADIENTS
            .with(|spare| spare.borrow_mut().pop())
            .unwrap_or_default();
        gradient.clear();
        gradient.resize(variables, 0.0);
        Self {
            error: 0.0,
            angular: false,
            gradient,
        }
    }

    /// Hands the buffer back to whoever writes the next equation.
    pub(crate) fn recycle(self) {
        SPARE_GRADIENTS.with(|spare| {
            let mut spare = spare.borrow_mut();
            if spare.len() < SPARE_GRADIENTS_KEPT {
                spare.push(self.gradient);
            }
        });
    }

    /// How far off this equation is, in a unit that means the same thing for
    /// every kind of dimension.
    pub(crate) fn off_by(&self, scale: f64) -> f64 {
        match self.angular {
            true => self.error.abs(),
            false => self.error.abs() / scale,
        }
    }

    pub(crate) fn add(&mut self, point: PointId, value: DVec2) {
        self.gradient[point.0 * 2] += value.x;
        self.gradient[point.0 * 2 + 1] += value.y;
    }

    /// Leaves the correction to one point, and takes it away from everything
    /// else the equation speaks of.
    ///
    /// What a rule holding a point on a curve asks: the point follows the
    /// curve, and never the other way round.
    pub(crate) fn hold_to(&mut self, point: PointId) {
        for (column, gradient) in self.gradient.iter_mut().enumerate() {
            if column != point.0 * 2 && column != point.0 * 2 + 1 {
                *gradient = 0.0;
            }
        }
    }

    /// The size of a circle is an unknown like any other: the columns after the
    /// coordinates are the radii, one apiece.
    pub(crate) fn add_radius(&mut self, at: usize, value: f64) {
        self.gradient[at] += value;
    }

    /// The squared gradient below which the equation has nothing left to push
    /// on, in the units its own gradient is written in.
    ///
    /// A length reads as a length over a length and does not answer to the size
    /// of the drawing; an angle reads as radians over a length and does, so the
    /// same corner in a drawing a million units across is written with a
    /// gradient a million times smaller. An absolute figure here declares the
    /// large drawing's angles flat and leaves them where they are.
    pub(crate) fn flat_below(&self, scale: f64) -> f64 {
        match self.angular {
            true => 1e-12 / (scale * scale),
            false => 1e-12,
        }
    }

    pub(crate) fn norm_squared(&self) -> f64 {
        self.gradient.iter().map(|value| value * value).sum()
    }
}
