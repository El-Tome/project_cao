//! Whether something closes around a place, reckoned from the curves of a
//! drawing alone.
//!
//! Never from the walk the drawing finds its areas with: a rule checked by the
//! code it checks would agree with every bug. Here the curves are cut wherever
//! they meet — crossing, touching, or one ending on another — into a graph of
//! their own, with no order round a vertex and no faces. A place is enclosed
//! when some loop of that graph winds round it. That is enough: a place shut
//! in is wound round once by the boundary of the piece of plane it sits in,
//! that boundary is a sum of the graph's fundamental loops, and so one of them
//! winds round it too; a place with a way out has every loop winding round it
//! nought times.
//!
//! The winding is read exactly rather than by sampling: a straight step from
//! one vertex to the next turns by the angle it is seen under, and a curved
//! one by that same angle, plus a whole turn when the place lies between the
//! curve and the straight step joining its two ends.

use std::f64::consts::TAU;

use cao_sketch::{EllipseDraft, Sketch, sweep_of};
use glam::DVec2;

/// Nearer than this, two places are one: twice the distance the drawing calls
/// one place (`THE_SAME_PLACE`, `crates/sketch/src/edges.rs`), and under the
/// five times a campaign misses a coincidence by when it misses one on purpose.
const ONE_PLACE: f64 = 2e-7;

pub fn one_place(at: DVec2) -> f64 {
    ONE_PLACE * (1.0 + at.abs().max_element())
}

/// How finely a curve is searched for where it meets an ellipse: eight times
/// finer than the drawing's own hunt. Two crossings closer than one step are
/// taken for a single touch, which shuts in the same places — all but the
/// sliver between them, far thinner than any place looked at stands from a
/// curve.
const OVAL_STEPS: usize = 2048;

/// A curve of the drawing, as plain geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Curve {
    Straight {
        from: DVec2,
        to: DVec2,
    },
    /// Counter-clockwise about `centre`, from the angle `from` over `sweep`
    /// radians: a whole circle when `sweep` is a whole turn.
    Round {
        centre: DVec2,
        radius: f64,
        from: f64,
        sweep: f64,
    },
    /// Counter-clockwise round the ellipse, from the turn `from` over `sweep`.
    Oval {
        drawn: EllipseDraft,
        from: f64,
        sweep: f64,
    },
}

/// The curves a drawing closes places with: every one still drawn, and none
/// of its construction geometry.
pub fn curves_of(sketch: &Sketch) -> Vec<Curve> {
    let mut curves = Vec::new();
    for (_, segment) in sketch.live_segments().filter(|(_, it)| !it.construction) {
        let (from, to) = (sketch.point(segment.start), sketch.point(segment.end));
        if from.distance(to) > one_place(from) {
            curves.push(Curve::Straight { from, to });
        }
    }
    for (id, arc) in sketch.live_arcs().filter(|(_, it)| !it.construction) {
        let drawn = sketch.arc_draft(id);
        let radius = drawn.centre.distance(drawn.start);
        let sweep = sweep_of(drawn);
        if radius > 0.0 && sweep > 0.0 {
            curves.push(Curve::Round {
                centre: sketch.point(arc.center),
                radius,
                from: (drawn.start - drawn.centre).to_angle(),
                sweep,
            });
        }
    }
    for (_, circle) in sketch.live_circles().filter(|(_, it)| !it.construction) {
        if circle.radius > 0.0 {
            curves.push(Curve::Round {
                centre: sketch.point(circle.center),
                radius: circle.radius,
                from: 0.0,
                sweep: TAU,
            });
        }
    }
    for (id, _) in sketch.live_ellipses().filter(|(_, it)| !it.construction) {
        let (from, sweep) = sketch.ellipse_run(id);
        if sweep > 0.0 {
            curves.push(Curve::Oval {
                drawn: sketch.ellipse_draft(id),
                from,
                sweep,
            });
        }
    }
    curves
}

/// A place in the ellipse's own measure, where the curve is the circle of
/// radius one about the origin.
fn squashed(drawn: &EllipseDraft, at: DVec2) -> DVec2 {
    let out = at - drawn.centre;
    DVec2::new(
        out.dot(drawn.first) / drawn.first.length_squared(),
        out.dot(drawn.second_axis()) / (drawn.second * drawn.second),
    )
}

impl Curve {
    /// Where the curve stands a fraction of the way along it.
    pub fn at(&self, along: f64) -> DVec2 {
        match *self {
            Curve::Straight { from, to } => from + (to - from) * along,
            Curve::Round {
                centre,
                radius,
                from,
                sweep,
            } => centre + DVec2::from_angle(from + sweep * along) * radius,
            Curve::Oval { drawn, from, sweep } => drawn.at(from + sweep * along),
        }
    }

    pub fn is_closed(&self) -> bool {
        match *self {
            Curve::Straight { .. } => false,
            Curve::Round { sweep, .. } | Curve::Oval { sweep, .. } => sweep >= TAU - 1e-12,
        }
    }

    /// How far round its circle or ellipse the curve runs, nothing for a trait:
    /// an open curve whose two ends are one place is a loop when it runs most
    /// of the way round, and a sliver standing on a point when it runs hardly
    /// at all.
    fn sweep(&self) -> f64 {
        match *self {
            Curve::Straight { .. } => 0.0,
            Curve::Round { sweep, .. } | Curve::Oval { sweep, .. } => sweep,
        }
    }

    /// How far the curve bends away from a straight step: its radius, the
    /// longer half-axis of its ellipse, nothing for a trait.
    pub fn bend(&self) -> f64 {
        match *self {
            Curve::Straight { .. } => 0.0,
            Curve::Round { radius, .. } => radius,
            Curve::Oval { drawn, .. } => drawn.first.length().max(drawn.second),
        }
    }

    /// The angle a place stands at round the curve's centre, in the curve's
    /// own measure, for a curve that has one.
    fn angle_of(&self, at: DVec2) -> f64 {
        match self {
            Curve::Straight { .. } => 0.0,
            Curve::Round { centre, .. } => (at - *centre).to_angle(),
            Curve::Oval { drawn, .. } => squashed(drawn, at).to_angle(),
        }
    }

    /// How far along the curve a place lying on it or beside it stands, as a
    /// fraction — or nothing when it stands past either end by more than one
    /// place.
    pub fn along(&self, at: DVec2) -> Option<f64> {
        match *self {
            Curve::Straight { from, to } => {
                let step = to - from;
                let along = (at - from).dot(step) / step.length_squared();
                let slack = one_place(at) / step.length();
                (-slack..=1.0 + slack)
                    .contains(&along)
                    .then(|| along.clamp(0.0, 1.0))
            }
            Curve::Round { from, sweep, .. } | Curve::Oval { from, sweep, .. } => {
                let turned = (self.angle_of(at) - from).rem_euclid(TAU);
                if self.is_closed() {
                    return Some(turned / TAU);
                }
                let slack = one_place(at) / self.bend().max(1e-12);
                if turned <= sweep + slack {
                    Some((turned / sweep).min(1.0))
                } else if turned >= TAU - slack {
                    Some(0.0)
                } else {
                    None
                }
            }
        }
    }

    /// How far a place stands from the curve, never more than it truly does:
    /// a stretch of ellipse is measured against the whole of it.
    pub fn distance(&self, at: DVec2) -> f64 {
        match *self {
            Curve::Straight { from, to } => {
                let step = to - from;
                let along = ((at - from).dot(step) / step.length_squared()).clamp(0.0, 1.0);
                at.distance(from + step * along)
            }
            Curve::Round {
                centre,
                radius,
                from,
                sweep,
            } => {
                let turned = ((at - centre).to_angle() - from).rem_euclid(TAU);
                if turned <= sweep {
                    (at.distance(centre) - radius).abs()
                } else {
                    at.distance(self.at(0.0)).min(at.distance(self.at(1.0)))
                }
            }
            Curve::Oval { drawn, .. } => drawn.distance(at),
        }
    }

    /// Whether a place lies inside the circle or the ellipse a curved piece
    /// is cut from.
    fn holds_inside(&self, at: DVec2) -> bool {
        match self {
            Curve::Straight { .. } => false,
            Curve::Round { centre, radius, .. } => at.distance(*centre) < *radius,
            Curve::Oval { drawn, .. } => squashed(drawn, at).length() < 1.0,
        }
    }

    /// The same curve as an ellipse, for what is only solved by searching.
    fn as_oval(&self) -> Option<EllipseDraft> {
        match *self {
            Curve::Straight { .. } => None,
            Curve::Round { centre, radius, .. } => Some(EllipseDraft {
                centre,
                first: DVec2::X * radius,
                second: radius,
            }),
            Curve::Oval { drawn, .. } => Some(drawn),
        }
    }

    /// How far off the circle or ellipse a place stands in its own measure:
    /// nought on it, negative inside.
    fn off(&self, at: DVec2) -> f64 {
        match self.as_oval() {
            Some(drawn) => squashed(&drawn, at).length() - 1.0,
            None => 0.0,
        }
    }
}

/// Where two curves meet, crossing or touching, on both their stretches.
fn meetings(first: &Curve, second: &Curve) -> Vec<DVec2> {
    let found = match (first, second) {
        (
            Curve::Straight { from, to },
            Curve::Straight {
                from: start,
                to: end,
            },
        ) => straights_meet(*from, *to, *start, *end),
        (Curve::Straight { from, to }, Curve::Round { centre, radius, .. })
        | (Curve::Round { centre, radius, .. }, Curve::Straight { from, to }) => {
            straight_meets_round(*from, *to, *centre, *radius)
        }
        (Curve::Straight { from, to }, Curve::Oval { drawn, .. })
        | (Curve::Oval { drawn, .. }, Curve::Straight { from, to }) => {
            straight_meets_oval(*from, *to, drawn)
        }
        (
            Curve::Round { centre, radius, .. },
            Curve::Round {
                centre: other,
                radius: across,
                ..
            },
        ) => rounds_meet(*centre, *radius, *other, *across),
        _ => searched(first, second),
    };
    found
        .into_iter()
        .filter(|at| first.along(*at).is_some() && second.along(*at).is_some())
        .collect()
}

fn straights_meet(from: DVec2, to: DVec2, start: DVec2, end: DVec2) -> Vec<DVec2> {
    let (one, other) = (to - from, end - start);
    let across = one.perp_dot(other);
    if across.abs() <= 1e-12 * one.length() * other.length() {
        return Vec::new();
    }
    let along = (start - from).perp_dot(other) / across;
    vec![from + one * along]
}

fn straight_meets_round(from: DVec2, to: DVec2, centre: DVec2, radius: f64) -> Vec<DVec2> {
    let step = to - from;
    let foot = from + step * ((centre - from).dot(step) / step.length_squared());
    let height = foot.distance(centre);
    if (height - radius).abs() <= one_place(foot) {
        let out = (foot - centre)
            .try_normalize()
            .unwrap_or(step.perp().normalize());
        return vec![centre + out * radius];
    }
    if height > radius {
        return Vec::new();
    }
    let half = (radius * radius - height * height).sqrt() / step.length();
    vec![foot - step * half, foot + step * half]
}

fn rounds_meet(centre: DVec2, radius: f64, other: DVec2, across: f64) -> Vec<DVec2> {
    let between = other - centre;
    let span = between.length();
    let room = one_place(centre).max(one_place(other));
    if span <= room {
        return Vec::new();
    }
    let toward = between / span;
    if (span - (radius + across)).abs() <= room {
        return vec![centre + toward * radius];
    }
    if (span - (radius - across).abs()).abs() <= room {
        let side = if radius > across { 1.0 } else { -1.0 };
        return vec![centre + toward * radius * side];
    }
    if span > radius + across || span < (radius - across).abs() {
        return Vec::new();
    }
    let along = (radius * radius - across * across + span * span) / (2.0 * span);
    let height = (radius * radius - along * along).max(0.0).sqrt();
    let base = centre + toward * along;
    vec![base + toward.perp() * height, base - toward.perp() * height]
}

/// A trait against an ellipse, solved in the ellipse's own measure, where the
/// ellipse is a circle and the trait is still straight.
fn straight_meets_oval(from: DVec2, to: DVec2, drawn: &EllipseDraft) -> Vec<DVec2> {
    let (near, far) = (squashed(drawn, from), squashed(drawn, to));
    let step = far - near;
    let foot = near + step * (-near.dot(step) / step.length_squared());
    let unsquashed = |at: DVec2| drawn.centre + drawn.first * at.x + drawn.second_axis() * at.y;
    let brushed = unsquashed(foot.try_normalize().unwrap_or(step.perp().normalize()));
    let line = to - from;
    let gap = (brushed - from).perp_dot(line).abs() / line.length();
    if gap <= one_place(brushed) {
        return vec![brushed];
    }
    if foot.length() >= 1.0 {
        return Vec::new();
    }
    let half = (1.0 - foot.length_squared()).sqrt() / step.length();
    vec![
        unsquashed(foot - step * half),
        unsquashed(foot + step * half),
    ]
}

/// Two curves of which one at least is an ellipse, met by walking the first
/// along its stretch and watching which side of the second it stands on: a
/// change of side is a crossing, closed in on by halving; a closest approach
/// within one place, with no change of side, is a touch.
fn searched(first: &Curve, second: &Curve) -> Vec<DVec2> {
    if same_curve(first, second) {
        return Vec::new();
    }
    let sweep = match first {
        Curve::Round { sweep, .. } | Curve::Oval { sweep, .. } => *sweep,
        Curve::Straight { .. } => return Vec::new(),
    };
    let steps = ((sweep / TAU * OVAL_STEPS as f64).ceil() as usize).max(16);
    let side = |along: f64| second.off(first.at(along));
    let alongs: Vec<f64> = (0..=steps).map(|step| step as f64 / steps as f64).collect();
    let sides: Vec<f64> = alongs.iter().map(|along| side(*along)).collect();
    let mut found = Vec::new();
    for step in 0..steps {
        let (low, high) = (alongs[step], alongs[step + 1]);
        if sides[step] == 0.0 {
            found.push(first.at(low));
        } else if sides[step].signum() != sides[step + 1].signum() && sides[step + 1] != 0.0 {
            found.push(first.at(halved(&side, low, high)));
        }
    }
    for (gap, at) in minima(first, second) {
        if gap <= one_place(at) {
            found.push(at);
        }
    }
    found
}

/// Where the first curve, walked along its stretch, comes closest to the
/// second without crossing it, and how far apart the two stand there.
fn minima(first: &Curve, second: &Curve) -> Vec<(f64, DVec2)> {
    let sweep = match first {
        Curve::Round { sweep, .. } | Curve::Oval { sweep, .. } => *sweep,
        Curve::Straight { .. } => return Vec::new(),
    };
    let steps = ((sweep / TAU * OVAL_STEPS as f64).ceil() as usize).max(16);
    let side = |along: f64| second.off(first.at(along));
    let alongs: Vec<f64> = (0..=steps).map(|step| step as f64 / steps as f64).collect();
    let sides: Vec<f64> = alongs.iter().map(|along| side(*along)).collect();
    let closed = first.is_closed();
    let width = 1.0 / steps as f64;
    let mut found = Vec::new();
    for step in 0..=steps {
        if closed && step == steps {
            break;
        }
        let before = match step {
            0 if closed => Some(sides[steps - 1]),
            0 => None,
            _ => Some(sides[step - 1]),
        };
        let after = (step < steps).then(|| sides[step + 1]);
        let here = sides[step];
        let lowest = [before, after]
            .into_iter()
            .flatten()
            .all(|other| other.signum() == here.signum() && here.abs() <= other.abs());
        if !lowest {
            continue;
        }
        let (low, high) = (alongs[step] - width, alongs[step] + width);
        let (low, high) = match closed {
            true => (low, high),
            false => (low.max(0.0), high.min(1.0)),
        };
        let at = first.at(closest(&|along: f64| side(along).abs(), low, high));
        found.push((second.distance(at), at));
    }
    found
}

fn same_curve(first: &Curve, second: &Curve) -> bool {
    [0.1, 0.3, 0.5, 0.7, 0.9].iter().all(|along| {
        let at = first.at(*along);
        second.off(at).abs() * second.bend() <= one_place(at)
    })
}

fn halved(side: &impl Fn(f64) -> f64, mut low: f64, mut high: f64) -> f64 {
    let below = side(low).signum();
    for _ in 0..60 {
        let middle = (low + high) / 2.0;
        if side(middle).signum() == below {
            low = middle;
        } else {
            high = middle;
        }
    }
    (low + high) / 2.0
}

fn closest(distance: &impl Fn(f64) -> f64, mut low: f64, mut high: f64) -> f64 {
    let golden = (5.0f64.sqrt() - 1.0) / 2.0;
    for _ in 0..80 {
        let left = high - golden * (high - low);
        let right = low + golden * (high - low);
        if distance(left) < distance(right) {
            high = right;
        } else {
            low = left;
        }
    }
    (low + high) / 2.0
}

/// Nearer than this to touching, and farther than a tenth of the distance
/// the drawing calls one place, two curves or a curve and an end stand where
/// the walk and this reckoning may each read a touch the other does not: the
/// walk welds at once that distance (`THE_SAME_PLACE`), this at twice it, and
/// a miss a campaign draws on purpose stays farther than five times it. A miss
/// along a curve's tangent stays clear of the curve only to the second order,
/// so a hair can still land in between.
const DOUBT: (f64, f64) = (1e-8, 5e-7);

fn doubtful(gap: f64, at: DVec2) -> bool {
    let scale = 1.0 + at.abs().max_element();
    gap > DOUBT.0 * scale && gap < DOUBT.1 * scale
}

/// The places where two curves come closest without crossing, and how far
/// apart they stand there.
fn approaches(first: &Curve, second: &Curve) -> Vec<(f64, DVec2)> {
    match (first, second) {
        (Curve::Straight { .. }, Curve::Straight { .. }) => Vec::new(),
        (Curve::Straight { from, to }, Curve::Round { centre, radius, .. })
        | (Curve::Round { centre, radius, .. }, Curve::Straight { from, to }) => {
            let step = *to - *from;
            let foot = *from + step * ((*centre - *from).dot(step) / step.length_squared());
            let out = (foot - *centre)
                .try_normalize()
                .unwrap_or(step.perp().normalize());
            vec![(foot.distance(*centre) - radius, *centre + out * *radius)]
        }
        (Curve::Straight { from, to }, Curve::Oval { drawn, .. })
        | (Curve::Oval { drawn, .. }, Curve::Straight { from, to }) => {
            let (near, far) = (squashed(drawn, *from), squashed(drawn, *to));
            let step = far - near;
            let foot = near + step * (-near.dot(step) / step.length_squared());
            if foot.length() < 1.0 {
                return Vec::new();
            }
            let way = foot.try_normalize().unwrap_or(step.perp().normalize());
            let brushed = drawn.centre + drawn.first * way.x + drawn.second_axis() * way.y;
            let line = *to - *from;
            vec![(
                (brushed - *from).perp_dot(line).abs() / line.length(),
                brushed,
            )]
        }
        (
            Curve::Round { centre, radius, .. },
            Curve::Round {
                centre: other,
                radius: across,
                ..
            },
        ) => {
            let between = *other - *centre;
            let span = between.length();
            let Some(toward) = between.try_normalize() else {
                return Vec::new();
            };
            let inner = if radius > across { 1.0 } else { -1.0 };
            vec![
                (span - (radius + across), *centre + toward * *radius),
                (
                    (radius - across).abs() - span,
                    *centre + toward * *radius * inner,
                ),
            ]
        }
        _ if same_curve(first, second) => Vec::new(),
        _ => minima(first, second),
    }
}

/// One piece of a curve, from one vertex of the graph to the next.
#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    curve: usize,
}

/// The curves of a drawing as a graph cut at every place two of them meet,
/// and the loops of it that every other loop is a sum of.
#[derive(Clone, Debug)]
pub struct Enclosure {
    curves: Vec<Curve>,
    vertices: Vec<DVec2>,
    edges: Vec<Edge>,
    /// The edges of a spanning forest, in an order where every vertex is
    /// reached after the one it hangs from: the vertex reached, the edge, and
    /// whether the edge runs towards it.
    tree: Vec<(usize, usize, bool)>,
    /// The vertex each reached one hangs from.
    hanging: Vec<Option<usize>>,
    /// Every edge outside the forest: each closes one fundamental loop.
    closing: Vec<usize>,
    /// Whether two curves, or a curve and an end, stand where the walk and
    /// this reckoning may each read a touch the other does not.
    doubtful: bool,
}

impl Enclosure {
    /// Whether the drawing holds a near touch the walk may read either way:
    /// what a drawing moved or turned may then read the other way, as its
    /// tolerance grows with how far out a place stands.
    pub fn is_doubtful(&self) -> bool {
        self.doubtful
    }

    pub fn of(curves: &[Curve]) -> Self {
        let mut cuts: Vec<Vec<(f64, DVec2)>> = curves
            .iter()
            .map(|curve| match curve.is_closed() {
                true => Vec::new(),
                false => vec![(0.0, curve.at(0.0)), (1.0, curve.at(1.0))],
            })
            .collect();
        let mut cut = |index: usize, at: DVec2| {
            if let Some(along) = curves[index].along(at) {
                cuts[index].push((along, at));
            }
        };
        for first in 0..curves.len() {
            for second in first + 1..curves.len() {
                for at in meetings(&curves[first], &curves[second]) {
                    cut(first, at);
                    cut(second, at);
                }
            }
        }
        for (index, curve) in curves.iter().enumerate() {
            if curve.is_closed() {
                continue;
            }
            for end in [curve.at(0.0), curve.at(1.0)] {
                for (other, beside) in curves.iter().enumerate() {
                    if other != index && beside.distance(end) <= one_place(end) {
                        cut(other, end);
                    }
                }
            }
        }

        let mut doubt = false;
        for first in 0..curves.len() {
            for second in first + 1..curves.len() {
                doubt |=
                    approaches(&curves[first], &curves[second])
                        .into_iter()
                        .any(|(gap, at)| {
                            doubtful(gap, at)
                                && curves[first].along(at).is_some()
                                && curves[second].along(at).is_some()
                        });
            }
        }
        for (index, curve) in curves.iter().enumerate() {
            if curve.is_closed() {
                continue;
            }
            for end in [curve.at(0.0), curve.at(1.0)] {
                doubt |= curves
                    .iter()
                    .enumerate()
                    .any(|(other, beside)| other != index && doubtful(beside.distance(end), end));
            }
        }

        let mut vertices: Vec<DVec2> = Vec::new();
        let mut vertex_at = |at: DVec2| match vertices
            .iter()
            .position(|known| known.distance(at) <= one_place(at))
        {
            Some(known) => known,
            None => {
                vertices.push(at);
                vertices.len() - 1
            }
        };
        let mut edges = Vec::new();
        for (index, curve) in curves.iter().enumerate() {
            let mut along = std::mem::take(&mut cuts[index]);
            along.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut stops: Vec<usize> = along.iter().map(|(_, at)| vertex_at(*at)).collect();
            stops.dedup();
            if curve.is_closed() {
                if stops.len() > 1 && stops.first() == stops.last() {
                    stops.pop();
                }
                if stops.is_empty() {
                    stops.push(vertex_at(curve.at(0.0)));
                }
                stops.push(stops[0]);
            } else if stops.len() == 1 && curve.sweep() > std::f64::consts::PI {
                stops.push(stops[0]);
            }
            for pair in stops.windows(2) {
                edges.push(Edge {
                    from: pair[0],
                    to: pair[1],
                    curve: index,
                });
            }
        }

        let mut hanging = vec![None; vertices.len()];
        let mut reached = vec![false; vertices.len()];
        let mut tree = Vec::new();
        let mut in_tree = vec![false; edges.len()];
        for root in 0..vertices.len() {
            if reached[root] {
                continue;
            }
            reached[root] = true;
            let mut queue = std::collections::VecDeque::from([root]);
            while let Some(here) = queue.pop_front() {
                for (index, edge) in edges.iter().enumerate() {
                    let (there, towards) = match (edge.from == here, edge.to == here) {
                        (true, _) => (edge.to, true),
                        (_, true) => (edge.from, false),
                        _ => continue,
                    };
                    if !reached[there] {
                        reached[there] = true;
                        hanging[there] = Some(here);
                        in_tree[index] = true;
                        tree.push((there, index, towards));
                        queue.push_back(there);
                    }
                }
            }
        }
        let closing = (0..edges.len()).filter(|index| !in_tree[*index]).collect();
        Self {
            curves: curves.to_vec(),
            vertices,
            edges,
            tree,
            hanging,
            closing,
            doubtful: doubt,
        }
    }

    /// Whether some loop of the curves winds round a place — or nothing, when
    /// the drawing holds a near touch the walk may read either way, or when the
    /// arithmetic comes back a fraction of a turn away from a whole one and
    /// cannot be trusted to say.
    pub fn encloses(&self, at: DVec2) -> Option<bool> {
        if self.doubtful {
            return None;
        }
        let turns: Vec<f64> = self.edges.iter().map(|edge| self.turn(edge, at)).collect();
        let mut wound = vec![0.0; self.vertices.len()];
        for (vertex, edge, towards) in &self.tree {
            let parent = self.hanging[*vertex].expect("a vertex reached from another");
            let turned = if *towards {
                turns[*edge]
            } else {
                -turns[*edge]
            };
            wound[*vertex] = wound[parent] + turned;
        }
        let mut enclosed = false;
        for index in &self.closing {
            let edge = self.edges[*index];
            let windings = (turns[*index] + wound[edge.from] - wound[edge.to]) / TAU;
            if (windings - windings.round()).abs() > 1e-6 {
                return None;
            }
            enclosed |= windings.round() != 0.0;
        }
        Some(enclosed)
    }

    /// How far round a place an edge turns, walked from its first vertex to
    /// its second.
    fn turn(&self, edge: &Edge, at: DVec2) -> f64 {
        let curve = &self.curves[edge.curve];
        let (start, end) = (self.vertices[edge.from], self.vertices[edge.to]);
        if edge.from == edge.to {
            return match curve.holds_inside(at) {
                true => TAU,
                false => 0.0,
            };
        }
        let (near, far) = (start - at, end - at);
        let across = near.perp_dot(far);
        let seen = if across == 0.0 && near.dot(far) < 0.0 {
            -std::f64::consts::PI
        } else {
            across.atan2(near.dot(far))
        };
        let between = across <= 0.0 && curve.holds_inside(at);
        match between {
            true => seen + TAU,
            false => seen,
        }
    }
}
