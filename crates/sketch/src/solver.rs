use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget};
use crate::equation::Equation;
use crate::independence::norm;
use crate::rigid::{Block, ownership, rigidify};
use crate::sketch::{Element, PointId, SegmentId, Sketch};

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
/// How many times the whole settling is begun again while it is still getting
/// somewhere.
///
/// One pass of it is not always enough: which parts are held rigid and which
/// are free to give is decided from the shape as it stands, and once the
/// drawing has moved that reading is out of date. Starting over takes the new
/// reading. This is exactly what used to happen by accident — a drawing left
/// half-corrected came right as soon as the next change gave it another go —
/// and it is what makes a tangent circle settle at the drop rather than
/// staying out of shape until something else was touched.
const ROUNDS: usize = 12;
/// How many times the shapes are put back and the drawing settled again around
/// them. The loop leaves as soon as a pass is found to have bent nothing, which
/// is the usual case; the bound is there for the drawing where putting a shape
/// back is what breaks the next equation.
const WELDS: usize = 4;
/// How much of the error a round must clear to be worth another one.
const WORTH_ANOTHER_ROUND: f64 = 0.9;
/// Below this, an equation counts as satisfied. Relative to the drawing's own
/// size, so it means the same thing at any scale.
const TOLERANCE: f64 = 1e-5;
/// How far off an axis a trait may lean and still count as square to the
/// sketch, which is what lets a shape keep its orientation without being told.
const SQUARE_DEGREES: f64 = 0.5;

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
    pub fn solve(&mut self, millimeters_per_unit: f64) -> SolveOutcome {
        if self.equations(millimeters_per_unit).is_empty() {
            return SolveOutcome::Nothing;
        }

        let scale = self.characteristic_size();
        let mut outcome = SolveOutcome::Residual;
        let mut before = f64::INFINITY;
        for _ in 0..ROUNDS {
            outcome = self.settle_once(millimeters_per_unit, scale);
            if outcome == SolveOutcome::Solved {
                break;
            }
            let left = self.worst_error(millimeters_per_unit, scale);
            if left > before * WORTH_ANOTHER_ROUND {
                break;
            }
            before = left;
        }

        // A trait pulled down to nothing is not geometry: its equations cannot
        // even be written, so the system would call itself solved while the
        // drawing had quietly fallen apart.
        match self.has_a_collapsed_trait(scale) {
            true => SolveOutcome::Residual,
            false => outcome,
        }
    }

    /// A length representative of the drawing, used where an absolute size
    /// would mean different things at different zooms.
    pub(crate) fn drawing_size(&self) -> f64 {
        self.characteristic_size()
    }

    /// Whether any trait has been squeezed down to a point.
    pub(crate) fn has_a_collapsed_trait(&self, scale: f64) -> bool {
        self.live_segments()
            .any(|(id, _)| self.segment_length(id) < scale * 1e-6)
    }

    /// One go at settling the drawing: the shapes that must not bend are read
    /// off as it stands, and every equation is corrected around them.
    fn settle_once(&mut self, millimeters_per_unit: f64, scale: f64) -> SolveOutcome {
        let held = self.orientations(millimeters_per_unit);
        let blocks = self.untouched_blocks(millimeters_per_unit, scale);
        let before: Vec<DVec2> = self.points().to_vec();

        let mut outcome = self.sweeps(millimeters_per_unit, scale, &blocks);
        if !blocks.is_empty() {
            // A block still bends where its shared and pinned points are left
            // behind, so it is put back and the drawing settled around it
            // again — until a pass is found to have bent nothing, which is what
            // makes the shape that comes out the one that went in.
            self.weld(&before, &blocks);
            for _ in 0..WELDS {
                outcome = self.sweeps(millimeters_per_unit, scale, &blocks);
                if self.weld(&before, &blocks) < scale * TOLERANCE {
                    break;
                }
            }

            // Keeping the shapes is not always possible — the values may want
            // the very thing that was being held. The drawing is then solved
            // the plain way, bending where it must.
            if self.worst_error(millimeters_per_unit, scale) >= TOLERANCE {
                for (index, position) in before.iter().enumerate() {
                    self.place_point(PointId(index), *position);
                }
                outcome = self.sweeps(millimeters_per_unit, scale, &[]);
            }
        }

        self.hold_orientations(&held);
        outcome
    }

    /// Corrects every equation a little, over and over, until nothing moves.
    ///
    /// Deterministic — same drawing, same order, same number of steps — which
    /// is what lets a part be rebuilt identically by replaying its history.
    fn sweeps(&mut self, millimeters_per_unit: f64, scale: f64, blocks: &[Block]) -> SolveOutcome {
        let pinned = self.pinned_points();
        let owner = ownership(blocks, self.points().len());

        // Both buffers have the same size on every turn, so the sweep owns them
        // rather than asking for them again on each correction. The gradients
        // are handed back to the same end, one equation at a time.
        let mut moves = vec![DVec2::ZERO; self.points().len()];
        let mut entry: Vec<Equation> = Vec::new();

        for _ in 0..MAX_ITERATIONS {
            let mut worst: f64 = 0.0;
            for index in 0..self.equation_count() {
                self.any_equation(index, millimeters_per_unit, &pinned, &mut entry);
                for equation in &entry {
                    worst = worst.max(equation.off_by(scale));

                    let norm = equation.norm_squared();
                    if norm < equation.flat_below(scale) {
                        continue;
                    }
                    // Move along the gradient just far enough to cancel the error.
                    let step = -equation.error / norm;
                    for (point, delta) in moves.iter_mut().enumerate() {
                        *delta = match pinned[point] {
                            true => DVec2::ZERO,
                            false => DVec2::new(
                                equation.gradient[point * 2] * step,
                                equation.gradient[point * 2 + 1] * step,
                            ),
                        };
                    }
                    rigidify(self.points(), &mut moves, blocks, &owner, &pinned, scale);

                    for (point, delta) in moves.iter().copied().enumerate() {
                        self.translate_point(PointId(point), delta);
                    }
                    // And the sizes of the circles, which are unknowns of the
                    // same system.
                    for circle in 0..self.circles().len() {
                        let column = self.points().len() * 2 + circle;
                        let delta = equation.gradient[column] * step;
                        if delta != 0.0 {
                            self.grow_circle(crate::sketch::CircleId(circle), delta);
                        }
                    }
                }
                for equation in entry.drain(..) {
                    equation.recycle();
                }
            }

            if worst < TOLERANCE {
                return SolveOutcome::Solved;
            }
        }
        SolveOutcome::Residual
    }

    /// The parts of the drawing the change has no reason to reshape.
    ///
    /// A value typed, or a point dragged, leaves a handful of equations no
    /// longer true. Those say where the drawing is allowed to give: an angle
    /// opens between its two traits, a length stretches its own trait. Every
    /// other trait stays welded to its neighbours, and the shapes they make are
    /// carried along and turned round, never bent.
    ///
    /// Without this, the correction spreads through the whole drawing and a
    /// figure at the far end — held by nothing in particular, as most of a
    /// drawing in progress is — quietly deforms.
    fn untouched_blocks(&self, millimeters_per_unit: f64, scale: f64) -> Vec<Block> {
        let pinned = self.pinned_points();
        let mut hot: Vec<Vec<PointId>> = Vec::new();
        let mut opened: Vec<(SegmentId, SegmentId)> = Vec::new();
        let mut stretched: Vec<SegmentId> = Vec::new();

        let mut entry: Vec<Equation> = Vec::new();
        for index in 0..self.equation_count() {
            let mut off = false;
            self.any_equation(index, millimeters_per_unit, &pinned, &mut entry);
            for equation in entry.drain(..) {
                if equation.off_by(scale) < TOLERANCE {
                    continue;
                }
                off = true;
                hot.push(
                    (0..self.points().len())
                        .filter(|point| {
                            equation.gradient[point * 2].abs() > 1e-12
                                || equation.gradient[point * 2 + 1].abs() > 1e-12
                        })
                        .map(PointId)
                        .collect(),
                );
            }
            if !off {
                continue;
            }
            // Where the drawing is allowed to give: a corner opens between its
            // two traits, a length stretches its own.
            match index.checked_sub(self.dimension_count()) {
                None => match self.dimensions()[index].target {
                    DimensionTarget::Angle { first, second } => opened.push((first, second)),
                    DimensionTarget::Length(segment) => stretched.push(segment),
                    _ => {}
                },
                Some(rule) => match self.constraints()[rule] {
                    Constraint::Perpendicular { first, second }
                    | Constraint::Parallel { first, second }
                    | Constraint::Collinear { first, second } => opened.push((first, second)),
                    Constraint::Equal { first, second } => {
                        stretched.push(first);
                        stretched.push(second);
                    }
                    Constraint::Tangent { segment, .. }
                    | Constraint::AxisCollinear { segment, .. } => stretched.push(segment),
                    _ => {}
                },
            }
        }
        if hot.is_empty() {
            return Vec::new();
        }

        let segments: Vec<(SegmentId, crate::sketch::Segment)> = self.live_segments().collect();
        let mut group: Vec<usize> = (0..segments.len()).collect();
        fn root(group: &mut [usize], mut seat: usize) -> usize {
            while group[seat] != seat {
                group[seat] = group[group[seat]];
                seat = group[seat];
            }
            seat
        }
        for (a, (first, one)) in segments.iter().enumerate() {
            for (b, (second, other)) in segments.iter().enumerate().skip(a + 1) {
                let joined = [one.start, one.end]
                    .iter()
                    .any(|point| *point == other.start || *point == other.end);
                let cut = stretched.contains(first)
                    || stretched.contains(second)
                    || opened.contains(&(*first, *second))
                    || opened.contains(&(*second, *first));
                if joined && !cut {
                    let (a, b) = (root(&mut group, a), root(&mut group, b));
                    group[a] = b;
                }
            }
        }

        let mut blocks: Vec<Vec<PointId>> = Vec::new();
        let mut owner: Vec<Option<usize>> = vec![None; segments.len()];
        for (index, (_, segment)) in segments.iter().enumerate() {
            let seat = root(&mut group, index);
            let block = *owner[seat].get_or_insert_with(|| {
                blocks.push(Vec::new());
                blocks.len() - 1
            });
            for point in [segment.start, segment.end] {
                if !blocks[block].contains(&point) {
                    blocks[block].push(point);
                }
            }
        }

        let mut kept: Vec<Block> = blocks
            .into_iter()
            .filter(|points| points.len() > 1)
            // A block holding every point of a change is the very thing that
            // has to give: it keeps no shape.
            .filter(|points| {
                !hot.iter()
                    .any(|touched| touched.iter().all(|point| points.contains(point)))
            })
            .map(|mut points| {
                points.sort_by_key(|point| point.0);
                Block {
                    anchored: points.iter().any(|point| pinned[point.0]),
                    points,
                }
            })
            .collect();

        // What cannot move is put back first, so the rest settles around it.
        kept.sort_by_key(|block| (!block.anchored, block.points[0].0));
        kept
    }

    /// How far off the drawing is, as a fraction of its own size.
    fn worst_error(&self, millimeters_per_unit: f64, scale: f64) -> f64 {
        self.equations(millimeters_per_unit)
            .iter()
            .map(|equation| equation.off_by(scale))
            .fold(0.0, f64::max)
    }

    /// The direction each free-to-turn group is sitting at, before anything
    /// moves.
    ///
    /// A group nothing holds upright can be spun without breaking a single
    /// dimension, so the solver is free to spin it — and it does: the steps are
    /// finite, and what each of them leaves behind adds up. A rectangle whose
    /// height is changed came out several degrees off, still reporting itself
    /// fully constrained, because it was: it had simply turned.
    fn orientations(&self, millimeters_per_unit: f64) -> Vec<(usize, PointId, PointId, f64)> {
        let equations = self.equations(millimeters_per_unit);
        let groups = self.point_groups();

        self.rotation_gauges(&self.pinned_points())
            .into_iter()
            .filter(|(_, gauge)| {
                equations
                    .iter()
                    .all(|equation| turns_nothing(equation, gauge))
            })
            .filter_map(|(owner, _)| {
                let (from, to) = self.orientation_pair(owner, &groups)?;
                let span = self.point(to) - self.point(from);
                (span.length() > 1e-6).then(|| (owner, from, to, span.to_angle()))
            })
            .collect()
    }

    /// The pair of points whose direction stands for a group's own. The first
    /// trait drawn in it, or — for a lone point — the line from the origin,
    /// which is the only other thing there is to lean on.
    fn orientation_pair(&self, owner: usize, groups: &[usize]) -> Option<(PointId, PointId)> {
        if let Some((_, segment)) = self
            .live_segments()
            .find(|(_, segment)| groups[segment.start.0] == owner)
        {
            return Some((segment.start, segment.end));
        }
        let lone = groups.iter().position(|group| *group == owner)?;
        (lone != Sketch::ORIGIN.0).then_some((Sketch::ORIGIN, PointId(lone)))
    }

    /// Turns each group back the way it was pointing. A rigid turn about the
    /// origin leaves every dimension of a group free to turn exactly as it
    /// found it — that is what "free to turn" means — so this straightens the
    /// drawing without touching what it measures.
    fn hold_orientations(&mut self, held: &[(usize, PointId, PointId, f64)]) {
        if held.is_empty() {
            return;
        }
        let groups = self.point_groups();
        let pinned = self.pinned_points();

        for (owner, from, to, was) in held {
            let span = self.point(*to) - self.point(*from);
            if span.length() < 1e-6 {
                continue;
            }
            let drift = wrap(span.to_angle() - was);
            if drift.abs() < 1e-6 {
                continue;
            }
            let turn = DVec2::from_angle(-drift);
            for index in 0..self.points().len() {
                if pinned[index] || groups[index] != *owner {
                    continue;
                }
                let moved = turn.rotate(self.point(PointId(index)));
                self.place_point(PointId(index), moved);
            }
        }
    }

    /// How many unknowns the drawing has: two per point, plus the size of each
    /// circle.
    ///
    /// A radius counts as one because it is one: a circle held against a trait
    /// gives on its size as readily as on its place, and a rule that could only
    /// move the circle would have to be broken to make it bigger.
    pub(crate) fn variables(&self) -> usize {
        self.points().len() * 2 + self.circles().len()
    }

    /// Which column stands for a circle's size.
    fn radius_column(&self, circle: crate::sketch::CircleId) -> Option<usize> {
        (circle.0 < self.circles().len()).then(|| self.points().len() * 2 + circle.0)
    }

    /// Points that must not move. Only the sketch's own origin, which is what
    /// everything else can be measured from.
    pub(crate) fn pinned_points(&self) -> Vec<bool> {
        let mut pinned: Vec<bool> = (0..self.points().len())
            .map(|index| self.is_origin(PointId(index)) || self.is_held_still(PointId(index)))
            .collect();
        for constraint in self.constraints() {
            let Constraint::Fixed { element } = constraint else {
                continue;
            };
            // Holding a trait means holding both its ends; holding a circle
            // means holding its centre and nothing else — its radius is free.
            let held: Vec<PointId> = match element {
                Element::Point(point) => vec![*point],
                Element::Segment(segment) => match self.segments().get(segment.0) {
                    Some(segment) => vec![segment.start, segment.end],
                    None => Vec::new(),
                },
                Element::Circle(circle) => match self.circles().get(circle.0) {
                    Some(circle) => vec![circle.center],
                    None => Vec::new(),
                },
            };
            for point in held {
                if point.0 < pinned.len() {
                    pinned[point.0] = true;
                }
            }
        }
        pinned
    }

    /// The system as the verdict "entirely constrained" reads it.
    ///
    /// Two things set it apart from the system the solver works on:
    ///
    /// Only the **sketch origin** counts as immovable. A `Fixe` holds a point
    /// still while the drawing settles, but it anchors it to nothing: the
    /// figure it belongs to could still be anywhere on the plane, and calling
    /// that finished would say the drawing is done when it is attached to
    /// nothing at all.
    ///
    /// And the drawing is granted the way up it was drawn only when it says so
    /// itself: a trait lying along an axis says which way up a shape is, and
    /// that is allowed to go without saying. A shape leaning at some other
    /// angle says nothing, and has to be told — an angle against an axis —
    /// before it can count as settled.
    pub(crate) fn anchored_system(&self, millimeters_per_unit: f64) -> Vec<Equation> {
        let anchored: Vec<bool> = (0..self.points().len())
            .map(|index| self.is_origin(PointId(index)))
            .collect();
        let mut equations = self.equations_pinned_by(millimeters_per_unit, &anchored);
        let square = self.groups_lying_square();
        for (owner, gauge) in self.rotation_gauges(&anchored) {
            if !square.contains(&owner) {
                continue;
            }
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

    /// Which group of joined geometry each point belongs to, as the index of a
    /// representative point. Two shapes drawn apart are two groups.
    pub(crate) fn point_groups(&self) -> Vec<usize> {
        let mut group: Vec<usize> = (0..self.points().len()).collect();

        fn root(group: &mut [usize], mut point: usize) -> usize {
            while group[point] != point {
                group[point] = group[group[point]];
                point = group[point];
            }
            point
        }
        for (_, segment) in self.live_segments() {
            let (a, b) = (
                root(&mut group, segment.start.0),
                root(&mut group, segment.end.0),
            );
            group[a] = b;
        }
        (0..group.len())
            .map(|point| root(&mut group, point))
            .collect()
    }

    /// The rule a drawing is never asked to state: that it does not turn on the
    /// spot.
    ///
    /// Spinning a shape about the sketch origin leaves every length and every
    /// angle exactly as it was, so no dimension can ever see it.
    ///
    /// One rule per group of joined geometry: two shapes drawn apart turn
    /// independently, so a single shared rule would leave both able to turn
    /// against each other and neither would ever count as settled.
    ///
    /// It carries no error: it never moves anything, it only accounts for a
    /// freedom that is not really there.
    fn rotation_gauges(&self, pinned: &[bool]) -> Vec<(usize, Equation)> {
        let groups = self.point_groups();

        let mut gauges: Vec<(usize, Equation)> = Vec::new();
        for (index, point) in self.points().iter().enumerate() {
            if pinned[index] {
                continue;
            }
            let owner = groups[index];
            let equation = match gauges.iter_mut().find(|(each, _)| *each == owner) {
                Some((_, equation)) => equation,
                None => {
                    gauges.push((owner, Equation::new(self.variables())));
                    &mut gauges.last_mut().expect("just pushed").1
                }
            };
            equation.add(PointId(index), DVec2::new(-point.y, point.x));
        }

        gauges.retain(|(_, equation)| equation.norm_squared() > 1e-12);
        gauges
    }

    /// The groups of joined geometry holding at least one trait along an axis.
    ///
    /// Such a trait is what lets a shape keep the way up it was drawn without
    /// being told: square to the sketch is a way up like any other, and the
    /// commonest one. Anything leaning has to carry an angle.
    fn groups_lying_square(&self) -> Vec<usize> {
        let groups = self.point_groups();
        let mut square = Vec::new();
        for (_, segment) in self.live_segments() {
            let span = self.point(segment.end) - self.point(segment.start);
            if span.length() < 1e-9 {
                continue;
            }
            let leaning = span.y.atan2(span.x).to_degrees().rem_euclid(90.0);
            if leaning.min(90.0 - leaning) > SQUARE_DEGREES {
                continue;
            }
            let owner = groups[segment.start.0];
            if !square.contains(&owner) {
                square.push(owner);
            }
        }
        square
    }

    /// A length representative of the drawing, used to judge errors relative to
    /// its size rather than in absolute units.
    fn characteristic_size(&self) -> f64 {
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
    pub(crate) fn equations(&self, millimeters_per_unit: f64) -> Vec<Equation> {
        self.equations_pinned_by(millimeters_per_unit, &self.pinned_points())
    }

    /// The same, told which points are to be treated as immovable. The solver
    /// and the verdict do not agree on that, so they each say.
    fn equations_pinned_by(&self, millimeters_per_unit: f64, pinned: &[bool]) -> Vec<Equation> {
        let mut equations = Vec::new();

        for index in 0..self.dimension_count() {
            if let Some(equation) = self.equation(index, millimeters_per_unit, pinned) {
                equations.push(equation);
            }
        }
        for index in 0..self.constraints().len() {
            self.rule_equations(index, pinned, &mut equations);
        }
        equations
    }

    /// How many equations the drawing is made of, dimensions and rules alike.
    /// The rules come after the dimensions, and a rule may bring more than one.
    fn equation_count(&self) -> usize {
        self.dimension_count() + self.constraints().len()
    }

    /// The equations of one entry, whichever kind it is, appended to what the
    /// caller already holds. Written into a buffer rather than returned so that
    /// a sweep can hand the same one back four hundred times.
    fn any_equation(
        &self,
        index: usize,
        millimeters_per_unit: f64,
        pinned: &[bool],
        into: &mut Vec<Equation>,
    ) {
        match index.checked_sub(self.dimension_count()) {
            Some(rule) => self.rule_equations(rule, pinned, into),
            None => into.extend(self.equation(index, millimeters_per_unit, pinned)),
        }
    }

    /// What one rule asks of the drawing, appended to what the caller holds.
    /// Most rules ask a single thing; lying two traits on one line, or holding
    /// a point halfway along one, asks two.
    fn rule_equations(&self, index: usize, pinned: &[bool], into: &mut Vec<Equation>) {
        let Some(constraint) = self.constraints().get(index).copied() else {
            return;
        };
        let written = into.len();

        match constraint {
            Constraint::Perpendicular { first, second } => {
                into.extend(self.direction_equation(first, second, true))
            }
            Constraint::Parallel { first, second } => {
                into.extend(self.direction_equation(first, second, false))
            }
            Constraint::Equal { first, second } => {
                into.extend(self.equal_length_equation(first, second))
            }
            Constraint::Collinear { first, second } => {
                let Some(line) = self.segments().get(second.0).copied() else {
                    return;
                };
                into.extend(
                    [line.start, line.end]
                        .into_iter()
                        .filter_map(|point| self.on_line_equation(point, first, 0.0)),
                );
            }
            Constraint::OnSegment { point, segment } => {
                into.extend(self.on_line_equation(point, segment, 0.0))
            }
            Constraint::Tangent {
                circle,
                segment,
                at,
            } => {
                let Some(round) = self.circles().get(circle.0).copied() else {
                    return;
                };
                let Some(mut equation) = self.on_line_equation(round.center, segment, round.radius)
                else {
                    return;
                };
                // Growing the circle closes the gap just as surely as moving
                // it does, so the size is part of the answer — outwards or
                // inwards according to the side of the line the circle is on.
                if let Some(column) = self.radius_column(circle) {
                    equation.add_radius(column, -self.side_of(round.center, segment));
                }
                into.push(equation);
                // Where the two touch is a point of the drawing, and it is not
                // free: it lies on the line, square under the centre. Without
                // that second half it would slide along the line, since sliding
                // a point along a circle it touches changes nothing at all to
                // first order.
                if let Some(contact) = self.live_point(at) {
                    into.extend(self.on_line_equation(contact, segment, 0.0));
                    into.extend(self.foot_equation(contact, round.center, segment));
                }
            }
            Constraint::OnCircle { point, circle } => into.extend(self.rim_equation(point, circle)),
            Constraint::EqualRadius { first, second } => {
                let (Some(one), Some(other)) =
                    (self.radius_column(first), self.radius_column(second))
                else {
                    return;
                };
                let mut equation = Equation::new(self.variables());
                equation.error = self.circles()[second.0].radius - self.circles()[first.0].radius;
                equation.add_radius(other, 1.0);
                equation.add_radius(one, -1.0);
                into.push(equation);
            }
            Constraint::Midpoint { point, segment } => {
                self.midpoint_equations(point, segment, into)
            }
            Constraint::AxisCollinear { segment, axis } => {
                self.on_axis_equations(segment, axis, into)
            }
            // Held in place by the pins rather than by an equation: a fixed
            // point simply has nowhere to go.
            Constraint::Fixed { .. } => {}
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

    /// Two traits told to stand square to each other, or to keep the same
    /// direction. Both are one and the same rule read off different halves of
    /// the pair of directions.
    fn direction_equation(
        &self,
        first: SegmentId,
        second: SegmentId,
        square: bool,
    ) -> Option<Equation> {
        let (one, other) = (
            *self.segments().get(first.0)?,
            *self.segments().get(second.0)?,
        );
        let (a, b) = (self.point(one.start), self.point(one.end));
        let (c, d) = (self.point(other.start), self.point(other.end));
        let (first_span, second_span) = (b - a, d - c);
        let (first_length, second_length) = (first_span.length(), second_span.length());
        if first_length < 1e-9 || second_length < 1e-9 {
            return None;
        }
        let (u, v) = (first_span / first_length, second_span / second_length);

        // Square: the two directions must have nothing in common. Parallel:
        // they must have nothing across. Either way the error is a sine or a
        // cosine, which is an angle in all but name.
        let (error, du, dv) = match square {
            true => (u.dot(v), v, u),
            false => (u.perp_dot(v), DVec2::new(v.y, -v.x), DVec2::new(-u.y, u.x)),
        };

        // Turning an end about the other changes the direction by the part of
        // the movement across the trait, scaled by how long it is.
        let across = |direction: DVec2, unit: DVec2, length: f64| {
            (direction - unit * direction.dot(unit)) / length
        };
        let first_gradient = across(du, u, first_length);
        let second_gradient = across(dv, v, second_length);

        let mut equation = Equation::new(self.variables());
        equation.error = error;
        equation.angular = true;
        equation.add(one.end, first_gradient);
        equation.add(one.start, -first_gradient);
        equation.add(other.end, second_gradient);
        equation.add(other.start, -second_gradient);
        Some(equation)
    }

    /// Two traits told to be the same length.
    fn equal_length_equation(&self, first: SegmentId, second: SegmentId) -> Option<Equation> {
        let (one, other) = (
            *self.segments().get(first.0)?,
            *self.segments().get(second.0)?,
        );
        let (a, b) = (self.point(one.start), self.point(one.end));
        let (c, d) = (self.point(other.start), self.point(other.end));
        let (first_span, second_span) = (b - a, d - c);
        let (first_length, second_length) = (first_span.length(), second_span.length());
        if first_length < 1e-9 || second_length < 1e-9 {
            return None;
        }

        // Only the second trait gives way: the first one clicked is the length
        // wanted, and a rule that moved both would leave neither of them the
        // size that was asked for. A corner the two share belongs to the first
        // as much as to the second, so it stays put too — otherwise stretching
        // the second would drag the first out of shape.
        let shared = |point: PointId| point == one.start || point == one.end;
        let mut equation = Equation::new(self.variables());
        equation.error = second_length - first_length;
        if !shared(other.end) {
            equation.add(other.end, second_span / second_length);
        }
        if !shared(other.start) {
            equation.add(other.start, -second_span / second_length);
        }
        Some(equation)
    }

    /// A trait laid on one of the sketch's own axes: both its ends have to sit
    /// on that line, which is two statements.
    fn on_axis_equations(
        &self,
        segment: SegmentId,
        axis: crate::constraints::SketchAxis,
        into: &mut Vec<Equation>,
    ) {
        let Some(line) = self.segments().get(segment.0).copied() else {
            return;
        };
        let direction = axis.direction();
        let normal = DVec2::new(-direction.y, direction.x);

        for point in [line.start, line.end] {
            let mut equation = Equation::new(self.variables());
            equation.error = self.point(point).dot(normal);
            equation.add(point, normal);
            into.push(equation);
        }
    }

    /// A point held at a given distance from the line a trait lies on — nought
    /// for a point on the line, a radius for a circle brushing it.
    ///
    /// Signed, unlike the dimension of the same name: a point *on* a line has
    /// no side to be on, and an unsigned error would have no gradient there.
    fn on_line_equation(&self, point: PointId, segment: SegmentId, gap: f64) -> Option<Equation> {
        let line = *self.segments().get(segment.0)?;
        if point.0 >= self.points().len() {
            return None;
        }
        let (a, b) = (self.point(line.start), self.point(line.end));
        let span = b - a;
        let length = span.length();
        if length < 1e-9 {
            return None;
        }
        let reach = self.point(point) - a;
        let cross = span.perp_dot(reach);
        let distance = cross / length;

        let unit = span / length;
        let gradient = |d_cross: DVec2, d_length: DVec2| (d_cross - d_length * distance) / length;

        let mut equation = Equation::new(self.variables());
        // A circle brushes the line on whichever side it is already on.
        equation.error = distance - gap * distance.signum();
        equation.add(point, gradient(DVec2::new(-span.y, span.x), DVec2::ZERO));
        equation.add(
            line.start,
            gradient(DVec2::new(span.y - reach.y, reach.x - span.x), -unit),
        );
        equation.add(line.end, gradient(DVec2::new(reach.y, -reach.x), unit));
        Some(equation)
    }

    /// Which side of a line a point lies on: +1 or -1, and +1 when it is on it.
    fn side_of(&self, point: PointId, segment: SegmentId) -> f64 {
        let Some(line) = self.segments().get(segment.0).copied() else {
            return 1.0;
        };
        let (a, b) = (self.point(line.start), self.point(line.end));
        match (b - a).perp_dot(self.point(point) - a) < 0.0 {
            true => -1.0,
            false => 1.0,
        }
    }

    /// A point held square under another across a line: what keeps a tangency's
    /// contact where the two actually touch.
    fn foot_equation(
        &self,
        point: PointId,
        under: PointId,
        segment: SegmentId,
    ) -> Option<Equation> {
        let line = *self.segments().get(segment.0)?;
        if point.0 >= self.points().len() || under.0 >= self.points().len() {
            return None;
        }
        let (a, b) = (self.point(line.start), self.point(line.end));
        let span = b - a;
        let length = span.length();
        if length < 1e-9 {
            return None;
        }
        let unit = span / length;
        let reach = self.point(point) - self.point(under);

        // The direction itself turns when the trait's ends move, which is what
        // carries the contact round with the line.
        let turning = (reach - unit * reach.dot(unit)) / length;

        let mut equation = Equation::new(self.variables());
        equation.error = reach.dot(unit);
        equation.add(point, unit);
        equation.add(under, -unit);
        equation.add(line.end, turning);
        equation.add(line.start, -turning);
        Some(equation)
    }

    /// A point held on a circle's rim. The size gives as readily as the place:
    /// dragging such a point is how a circle is resized by hand.
    fn rim_equation(&self, point: PointId, circle: crate::sketch::CircleId) -> Option<Equation> {
        let round = *self.circles().get(circle.0)?;
        if point.0 >= self.points().len() {
            return None;
        }
        let reach = self.point(point) - self.point(round.center);
        let length = reach.length();
        if length < 1e-9 {
            return None;
        }
        let unit = reach / length;

        let mut equation = Equation::new(self.variables());
        equation.error = length - round.radius;
        equation.add(point, unit);
        equation.add(round.center, -unit);
        if let Some(column) = self.radius_column(circle) {
            equation.add_radius(column, -1.0);
        }
        Some(equation)
    }

    /// A point a rule names, when it is still drawn.
    fn live_point(&self, point: Option<PointId>) -> Option<PointId> {
        point.filter(|id| id.0 < self.points().len() && !self.is_erased_point(*id))
    }

    /// A point held halfway along a trait: one equation for each coordinate,
    /// since being at the middle is two statements, not one.
    fn midpoint_equations(&self, point: PointId, segment: SegmentId, into: &mut Vec<Equation>) {
        let Some(line) = self.segments().get(segment.0).copied() else {
            return;
        };
        if point.0 >= self.points().len() {
            return;
        }
        let middle = (self.point(line.start) + self.point(line.end)) * 0.5;
        let held = self.point(point);

        for axis in [DVec2::X, DVec2::Y] {
            let mut equation = Equation::new(self.variables());
            equation.error = (held - middle).dot(axis);
            equation.add(point, axis);
            equation.add(line.start, -axis * 0.5);
            equation.add(line.end, -axis * 0.5);
            into.push(equation);
        }
    }

    /// The equation for one dimension, or `None` when it does not apply to
    /// anything solvable — a radius, which stands alone, or a broken reference.
    fn equation(
        &self,
        index: usize,
        millimeters_per_unit: f64,
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
            DimensionTarget::PointToSegment { point, segment } => {
                self.point_to_segment_equation(point, segment, dimension.value / scale)?
            }
            DimensionTarget::Projected { from, to, axis } => {
                self.projected_equation(from, to, axis, dimension.value / scale)?
            }
            DimensionTarget::Radius(circle) => {
                self.size_equation(circle, dimension.value / scale)?
            }
            DimensionTarget::Diameter(circle) => {
                self.size_equation(circle, dimension.value / (2.0 * scale))?
            }
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

    /// A circle told how big to be.
    fn size_equation(&self, circle: crate::sketch::CircleId, target: f64) -> Option<Equation> {
        let column = self.radius_column(circle)?;
        let mut equation = Equation::new(self.variables());
        equation.error = self.circles().get(circle.0)?.radius - target;
        equation.add_radius(column, 1.0);
        Some(equation)
    }

    fn length_equation(&self, a: PointId, b: PointId, target: f64) -> Option<Equation> {
        let span = self.point(b) - self.point(a);
        let length = span.length();
        if length < 1e-9 {
            return None;
        }
        let direction = span / length;

        let mut equation = Equation::new(self.variables());
        equation.error = length - target;
        equation.add(b, direction);
        equation.add(a, -direction);
        Some(equation)
    }

    /// The gap between two points along one axis, which is what a horizontal or
    /// vertical dimension on a slanted trait holds.
    ///
    /// Only the movement along that axis matters: the points stay free to slide
    /// across it, which is exactly what makes such a dimension weaker than a
    /// length — and the reason both can sit on the same trait.
    fn projected_equation(
        &self,
        from: PointId,
        to: PointId,
        axis: crate::constraints::SketchAxis,
        target: f64,
    ) -> Option<Equation> {
        if from.0 >= self.points().len() || to.0 >= self.points().len() {
            return None;
        }
        let direction = axis.direction();
        let gap = (self.point(to) - self.point(from)).dot(direction);
        if gap.abs() < 1e-9 {
            return None;
        }
        let sign = if gap < 0.0 { -1.0 } else { 1.0 };

        let mut equation = Equation::new(self.variables());
        equation.error = gap.abs() - target;
        equation.add(to, direction * sign);
        equation.add(from, -direction * sign);
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
        degrees: f64,
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
        let from_first = DVec2::new(-a.y, a.x) / length_a;
        let from_second = DVec2::new(-b.y, b.x) / length_b;

        let mut equation = Equation::new(self.variables());
        equation.error = signed.abs() - degrees.to_radians();
        equation.angular = true;
        equation.add(far_second, from_second * sign);
        equation.add(far_first, -from_first * sign);
        equation.add(pivot, (from_first - from_second) * sign);
        Some(equation)
    }
}

impl Sketch {
    /// The distance from a point to the line two other points define.
    ///
    /// The distance is the cross product of the line's span with the reach to
    /// the point, over that span's length; everything below is that quotient
    /// differentiated, which is why the line's own ends move too — a drawing
    /// where only the point could answer would tilt the line instead.
    fn point_to_segment_equation(
        &self,
        point: PointId,
        segment: crate::sketch::SegmentId,
        target: f64,
    ) -> Option<Equation> {
        let segment = *self.segments().get(segment.0)?;
        let (a, b) = (self.point(segment.start), self.point(segment.end));
        let span = b - a;
        let length = span.length();
        if length < 1e-9 {
            return None;
        }
        let reach = self.point(point) - a;
        let cross = span.perp_dot(reach);
        let distance = cross / length;
        if distance.abs() < 1e-9 {
            return None;
        }

        let d_cross_point = DVec2::new(-span.y, span.x);
        let d_cross_start = DVec2::new(span.y - reach.y, reach.x - span.x);
        let d_cross_end = DVec2::new(reach.y, -reach.x);
        let unit = span / length;
        let gradient = |d_cross: DVec2, d_length: DVec2| (d_cross - d_length * distance) / length;

        let sign = if distance < 0.0 { -1.0 } else { 1.0 };
        let mut equation = Equation::new(self.variables());
        equation.error = distance.abs() - target;
        equation.add(point, gradient(d_cross_point, DVec2::ZERO) * sign);
        equation.add(segment.start, gradient(d_cross_start, -unit) * sign);
        equation.add(segment.end, gradient(d_cross_end, unit) * sign);
        Some(equation)
    }

    /// The angle between a segment and a fixed direction of the sketch.
    ///
    /// Unlike an angle between two segments, this one has something immovable
    /// to lean on, so it is what finally stops a drawing from spinning about
    /// its anchor.
    fn axis_angle_equation(
        &self,
        segment: crate::sketch::SegmentId,
        axis: crate::constraints::SketchAxis,
        degrees: f64,
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
        let turn = DVec2::new(-span.y, span.x) / length;

        let mut equation = Equation::new(self.variables());
        equation.error = signed.abs() - degrees.to_radians();
        equation.angular = true;
        equation.add(segment.end, turn * sign);
        equation.add(segment.start, -turn * sign);
        Some(equation)
    }
}

/// An angle brought back into [-pi, pi], so a drift either side of a turn reads
/// as the small angle it is.
fn wrap(mut angle: f64) -> f64 {
    while angle > std::f64::consts::PI {
        angle -= std::f64::consts::TAU;
    }
    while angle < -std::f64::consts::PI {
        angle += std::f64::consts::TAU;
    }
    angle
}

/// Whether an equation says nothing about which way round the drawing sits.
///
/// A dimension taken against an axis already fixes the orientation; adding the
/// implicit rule on top of it would take away a freedom twice and report a
/// drawing as more settled than it is.
fn turns_nothing(equation: &Equation, gauge: &Equation) -> bool {
    let projection: f64 = equation
        .gradient
        .iter()
        .zip(&gauge.gradient)
        .map(|(a, b)| a * b)
        .sum();
    let sizes = norm(&equation.gradient) * norm(&gauge.gradient);
    sizes < 1e-12 || (projection / sizes).abs() < 1e-3
}
