use glam::Vec2;

use crate::constraints::DimensionTarget;
use crate::sketch::{PointId, Sketch};

/// One equation the drawing has to satisfy, linearised around its current
/// shape: how far off it is, and how each coordinate would change that.
///
/// Gradients are analytic rather than sampled: they are short to write for
/// lengths and angles, exact, and the solver runs them hundreds of times.
pub(crate) struct Equation {
    /// Current value minus the wanted one. Zero when satisfied.
    pub error: f32,
    /// Change of `error` per unit change of each coordinate, laid out as
    /// x0, y0, x1, y1, …
    pub gradient: Vec<f32>,
}

impl Equation {
    fn new(variables: usize) -> Self {
        Self {
            error: 0.0,
            gradient: vec![0.0; variables],
        }
    }

    fn add(&mut self, point: PointId, value: Vec2) {
        self.gradient[point.0 * 2] += value.x;
        self.gradient[point.0 * 2 + 1] += value.y;
    }

    fn norm_squared(&self) -> f32 {
        self.gradient.iter().map(|value| value * value).sum()
    }
}

/// How the solve went.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolveOutcome {
    /// Every constraint is satisfied.
    Solved,
    /// The solver ran out of steps with some error left: the constraints
    /// contradict each other, or the shape cannot reach them.
    Residual,
    /// Nothing to solve.
    Nothing,
}

const MAX_ITERATIONS: usize = 400;
/// Below this, an equation counts as satisfied. Relative to the drawing's own
/// size, so it means the same thing at any scale.
const TOLERANCE: f32 = 1e-4;

impl Sketch {
    /// Moves the drawing until every dimension holds at once.
    ///
    /// This is why a value stays true after another one is changed: the whole
    /// system is re-solved, rather than each dimension being applied once and
    /// then forgotten.
    ///
    /// The method is projection: each equation is corrected a little in turn,
    /// over and over, until nothing moves. It is deterministic — same drawing,
    /// same order, same number of steps — which is what lets a part be rebuilt
    /// identically by replaying its history.
    pub fn solve(&mut self, millimeters_per_unit: f32) -> SolveOutcome {
        let equations = self.equations(millimeters_per_unit);
        if equations.is_empty() {
            return SolveOutcome::Nothing;
        }

        let pinned = self.pinned_points();
        let scale = self.characteristic_size();

        for _ in 0..MAX_ITERATIONS {
            let mut worst: f32 = 0.0;
            for index in 0..self.dimension_count() {
                let Some(equation) = self.equation(index, millimeters_per_unit, &pinned) else {
                    continue;
                };
                worst = worst.max(equation.error.abs() / scale);

                let norm = equation.norm_squared();
                if norm < 1e-12 {
                    continue;
                }
                // Move along the gradient just far enough to cancel the error.
                let step = -equation.error / norm;
                let moves: Vec<_> = pinned
                    .iter()
                    .enumerate()
                    .filter(|(_, pinned)| !**pinned)
                    .map(|(point, _)| {
                        (
                            PointId(point),
                            Vec2::new(
                                equation.gradient[point * 2] * step,
                                equation.gradient[point * 2 + 1] * step,
                            ),
                        )
                    })
                    .collect();
                for (point, delta) in moves {
                    self.translate_point(point, delta);
                }
            }

            if worst < TOLERANCE {
                return SolveOutcome::Solved;
            }
        }

        SolveOutcome::Residual
    }

    /// Points that must not move. Only the sketch's own origin, which is what
    /// everything else can be measured from.
    fn pinned_points(&self) -> Vec<bool> {
        (0..self.points().len())
            .map(|index| self.is_origin(PointId(index)))
            .collect()
    }

    /// Every equation the drawing must satisfy, plus the one rule it is never
    /// asked to state: that it does not turn on the spot.
    ///
    /// Spinning a whole drawing about the sketch origin leaves every length and
    /// every angle exactly as it was, so no dimension can ever see it. Without
    /// this, a shape could carry all its values and still be reported loose,
    /// and the user had to add an angle to an axis by hand purely to say "and
    /// it stays this way up". That orientation is implicit now, just as the
    /// origin point is.
    ///
    /// It carries no error: it never moves anything, it only accounts for the
    /// freedom that is already gone.
    pub(crate) fn analysed_system(&self, millimeters_per_unit: f32) -> Vec<Equation> {
        let mut equations = self.equations(millimeters_per_unit);
        for gauge in self.rotation_gauges() {
            // A shape already measured against an axis says which way up it is;
            // adding the implicit rule on top would take that freedom twice and
            // report a drawing as more settled than it is.
            if equations
                .iter()
                .all(|equation| turns_nothing(equation, &gauge))
            {
                equations.push(gauge);
            }
        }
        equations
    }

    /// One rule per group of joined geometry. Two shapes drawn apart can be
    /// turned independently, so one shared rule would leave both of them able
    /// to turn against each other and neither would ever count as settled.
    fn rotation_gauges(&self) -> Vec<Equation> {
        let pinned = self.pinned_points();
        let mut group: Vec<usize> = (0..self.points().len()).collect();

        fn root(group: &mut [usize], mut point: usize) -> usize {
            while group[point] != point {
                group[point] = group[group[point]];
                point = group[point];
            }
            point
        }
        for segment in self.segments() {
            let (a, b) = (root(&mut group, segment.start.0), root(&mut group, segment.end.0));
            group[a] = b;
        }

        let mut gauges: Vec<(usize, Equation)> = Vec::new();
        for (index, point) in self.points().iter().enumerate() {
            if pinned[index] {
                continue;
            }
            let owner = root(&mut group, index);
            let equation = match gauges.iter_mut().find(|(each, _)| *each == owner) {
                Some((_, equation)) => equation,
                None => {
                    gauges.push((owner, Equation::new(self.points().len() * 2)));
                    &mut gauges.last_mut().expect("just pushed").1
                }
            };
            equation.add(PointId(index), Vec2::new(-point.y, point.x));
        }

        gauges
            .into_iter()
            .map(|(_, equation)| equation)
            .filter(|equation| equation.norm_squared() > 1e-12)
            .collect()
    }

    /// A length representative of the drawing, used to judge errors relative to
    /// its size rather than in absolute units.
    fn characteristic_size(&self) -> f32 {
        self.bounds()
            .map(|(min, max)| (max - min).length())
            .filter(|size| *size > 1e-6)
            .unwrap_or(1.0)
    }

    fn dimension_count(&self) -> usize {
        self.dimensions().len()
    }

    /// Every equation the drawing must satisfy, including the pins that hold it
    /// in place.
    pub(crate) fn equations(&self, millimeters_per_unit: f32) -> Vec<Equation> {
        let mut equations = Vec::new();
        let pinned = self.pinned_points();

        for index in 0..self.dimension_count() {
            if let Some(equation) = self.equation(index, millimeters_per_unit, &pinned) {
                equations.push(equation);
            }
        }
        equations
    }

    /// The equation for one dimension, or `None` when it does not apply to
    /// anything solvable — a radius, which stands alone, or a broken reference.
    fn equation(
        &self,
        index: usize,
        millimeters_per_unit: f32,
        pinned: &[bool],
    ) -> Option<Equation> {
        let dimension = *self.dimensions().get(index)?;
        if dimension.driven {
            return None;
        }
        let scale = if millimeters_per_unit > 1e-9 {
            millimeters_per_unit
        } else {
            1.0
        };

        let mut equation = match dimension.target {
            DimensionTarget::Length(segment) => {
                let segment = *self.segments().get(segment.0)?;
                self.length_equation(segment.start, segment.end, dimension.value / scale)?
            }
            DimensionTarget::Distance { from, to } => {
                self.length_equation(from, to, dimension.value / scale)?
            }
            DimensionTarget::Angle { first, second } => {
                self.angle_equation(first, second, dimension.value)?
            }
            DimensionTarget::AxisAngle { segment, axis } => {
                self.axis_angle_equation(segment, axis, dimension.value)?
            }
            // A radius has no bearing on where the points are.
            DimensionTarget::Radius(_) => return None,
        };

        // A pinned coordinate cannot absorb any correction.
        for (point, pinned) in pinned.iter().enumerate() {
            if *pinned {
                equation.gradient[point * 2] = 0.0;
                equation.gradient[point * 2 + 1] = 0.0;
            }
        }
        Some(equation)
    }

    fn length_equation(&self, a: PointId, b: PointId, target: f32) -> Option<Equation> {
        let span = self.point(b) - self.point(a);
        let length = span.length();
        if length < 1e-9 {
            return None;
        }
        let direction = span / length;

        let mut equation = Equation::new(self.points().len() * 2);
        equation.error = length - target;
        equation.add(b, direction);
        equation.add(a, -direction);
        Some(equation)
    }

    /// The angle at the corner two segments share.
    ///
    /// The wanted value keeps the sign the corner currently has, so asking for
    /// 30° on a corner that opens one way does not flip it to the other.
    fn angle_equation(
        &self,
        first: crate::sketch::SegmentId,
        second: crate::sketch::SegmentId,
        degrees: f32,
    ) -> Option<Equation> {
        let (pivot, far_first, far_second) = self.shared_corner(first, second)?;
        let a = self.point(far_first) - self.point(pivot);
        let b = self.point(far_second) - self.point(pivot);
        let (length_a, length_b) = (a.length_squared(), b.length_squared());
        if length_a < 1e-12 || length_b < 1e-12 {
            return None;
        }

        let signed = a.perp_dot(b).atan2(a.dot(b));
        let sign = if signed < 0.0 { -1.0 } else { 1.0 };

        // Turning a point about the pivot changes the angle by the component
        // perpendicular to its arm, scaled by how far out it sits.
        let from_first = Vec2::new(-a.y, a.x) / length_a;
        let from_second = Vec2::new(-b.y, b.x) / length_b;

        let mut equation = Equation::new(self.points().len() * 2);
        equation.error = signed.abs() - degrees.to_radians();
        equation.add(far_second, from_second * sign);
        equation.add(far_first, -from_first * sign);
        equation.add(pivot, (from_first - from_second) * sign);
        Some(equation)
    }
}

impl Sketch {
    /// The angle between a segment and a fixed direction of the sketch.
    ///
    /// Unlike an angle between two segments, this one has something immovable
    /// to lean on, so it is what finally stops a drawing from spinning about
    /// its anchor.
    fn axis_angle_equation(
        &self,
        segment: crate::sketch::SegmentId,
        axis: crate::constraints::SketchAxis,
        degrees: f32,
    ) -> Option<Equation> {
        let segment = *self.segments().get(segment.0)?;
        let span = self.point(segment.end) - self.point(segment.start);
        let length = span.length_squared();
        if length < 1e-12 {
            return None;
        }

        let reference = axis.direction();
        let signed = reference.perp_dot(span).atan2(reference.dot(span));
        let sign = if signed < 0.0 { -1.0 } else { 1.0 };
        let turn = Vec2::new(-span.y, span.x) / length;

        let mut equation = Equation::new(self.points().len() * 2);
        equation.error = signed.abs() - degrees.to_radians();
        equation.add(segment.end, turn * sign);
        equation.add(segment.start, -turn * sign);
        Some(equation)
    }
}

/// Whether an equation says nothing about which way round the drawing sits.
///
/// A dimension taken against an axis already fixes the orientation; adding the
/// implicit rule on top of it would take away a freedom twice and report a
/// drawing as more settled than it is.
fn turns_nothing(equation: &Equation, gauge: &Equation) -> bool {
    let projection: f32 = equation
        .gradient
        .iter()
        .zip(&gauge.gradient)
        .map(|(a, b)| a * b)
        .sum();
    let sizes = norm(&equation.gradient) * norm(&gauge.gradient);
    sizes < 1e-12 || (projection / sizes).abs() < 1e-3
}

/// How many of a set of equations are genuinely independent.
///
/// Gram–Schmidt: each row is stripped of whatever the earlier rows already
/// said; what is left over, if anything, is new information. This is what tells
/// a constraint that adds nothing from one that pins a shape down further —
/// counting constraints could never see that a triangle's third side follows
/// from its other sides and angles.
pub(crate) fn rank(equations: &[Equation]) -> usize {
    independent_rows(equations, None).0
}

/// The ways the drawing can still move without breaking anything.
///
/// Each returned vector is a direction the coordinates may travel in. Where a
/// point has no component in any of them, it cannot move at all: that point is
/// settled, whatever the rest of the drawing is doing. This is what lets one
/// line be shown as fixed while its neighbour is still loose.
pub(crate) fn null_space(
    equations: &[Equation],
    pinned: &[bool],
    variables: usize,
) -> Vec<Vec<f32>> {
    let mut basis: Vec<Vec<f32>> = Vec::new();
    for equation in equations {
        if let Some(row) = reduce(&equation.gradient, &basis) {
            basis.push(row);
        }
    }

    // Anything left once the constraints have had their say is free movement.
    let mut free: Vec<Vec<f32>> = Vec::new();
    for index in 0..variables {
        // A pinned point cannot move, so it is not a direction to consider.
        if pinned.get(index / 2).copied().unwrap_or(false) {
            continue;
        }
        let mut candidate = vec![0.0; variables];
        candidate[index] = 1.0;

        let mut combined = basis.clone();
        combined.extend(free.iter().cloned());
        if let Some(direction) = reduce(&candidate, &combined) {
            free.push(direction);
        }
    }
    free
}

/// True when `candidate` says nothing the others do not already say.
pub(crate) fn is_dependent(equations: &[Equation], candidate: &Equation) -> bool {
    independent_rows(equations, Some(candidate)).1
}

fn independent_rows(equations: &[Equation], candidate: Option<&Equation>) -> (usize, bool) {
    let mut basis: Vec<Vec<f32>> = Vec::new();

    for equation in equations {
        if let Some(row) = reduce(&equation.gradient, &basis) {
            basis.push(row);
        }
    }

    let dependent = match candidate {
        Some(candidate) => reduce(&candidate.gradient, &basis).is_none(),
        None => false,
    };
    (basis.len(), dependent)
}

/// Removes from `row` everything the basis already covers, returning what is
/// left once normalised, or `None` when nothing is.
fn reduce(row: &[f32], basis: &[Vec<f32>]) -> Option<Vec<f32>> {
    let mut residual = row.to_vec();
    let original = norm(&residual);
    if original < 1e-9 {
        return None;
    }

    for existing in basis {
        let projection: f32 = residual
            .iter()
            .zip(existing)
            .map(|(value, base)| value * base)
            .sum();
        for (value, base) in residual.iter_mut().zip(existing) {
            *value -= projection * base;
        }
    }

    let length = norm(&residual);
    // Relative to the original: a row a thousand times shorter than it started
    // is numerical dust, not information.
    if length / original < 1e-4 {
        return None;
    }
    for value in &mut residual {
        *value /= length;
    }
    Some(residual)
}

fn norm(row: &[f32]) -> f32 {
    row.iter().map(|value| value * value).sum::<f32>().sqrt()
}
