//! One row of the system the drawing has to satisfy, and the buffer it is
//! written into.

use std::cell::RefCell;

use glam::DVec2;

use crate::sketch::PointId;

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
