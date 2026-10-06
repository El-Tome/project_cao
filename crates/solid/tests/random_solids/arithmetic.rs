//! A case run through a kernel and held at every step to what arithmetic
//! promised along the harness's lines, rather than to its inputs' own
//! triangles.
//!
//! The flats' check compares a result's triangles with its inputs': that
//! holds a kernel whose result is made of its inputs' own facets. A kernel
//! that lays every body's curves afresh meets a line grazing a wall on two
//! different chords, before and after. So the promise is worked out from the
//! leaves — `Leaf::along` — and combined step by step as the flats' is, and
//! the triangles are given along every line the room their tolerance takes
//! there.
//!
//! That room is not the tolerance over the cosine at each crossing, which is
//! what a line crossing a smooth wall needs: near an edge the mesh can let a
//! line in through another face than the true surface does, and near a
//! tangent it can miss the wall altogether. What bounds it everywhere is how
//! much of the line lies within the tolerance of some leaf's boundary: a
//! place in the triangles and not in the promise, or the other way round,
//! is that close to the true surface, and the true surface is made of the
//! leaves' own. Each leaf grown by the tolerance, less the same leaf shrunk
//! by it, holds all of that.

use cao_solid::Declined;
use cao_solid::brep::Scale;
use cao_solid::soundness::{
    Along, Flaw, Lines, Silence, Spans, Triangle, closed, enclosed, listed, repeatable, uncrossed,
};
use glam::{DVec2, DVec3};

use super::{Case, Crossing, Kernel, Leaf, Mode, Outline, within_reach};

/// How many lines of measure cross a case each way, as in the flats' check.
const LINES: usize = 48;

/// Under this cosine, a line crossing a curved wall is left out and counted,
/// rather than held with a room as long as the stretch it grazes: how many
/// there are says how much of a campaign looked at nothing.
const GRAZING: f64 = 0.05;

/// How far the exact body may stand from the promise along a line, as a
/// fraction of the reach: what the flats are held to along their own lines,
/// a thousand times the kernel's tolerance.
const EXACTLY: f64 = 1e-6;

/// How many lines a case was held along, and how many were left out for
/// grazing a curved wall, summed over every body the case was checked at;
/// how many cases the kernel declined as asking for a curve it does not
/// build, and how many it declined to raise for a turned wall a hair thin,
/// which are no answers it owed (#533).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Measured {
    pub held: usize,
    pub grazing: usize,
    pub declined: usize,
    pub thin: usize,
}

/// What a case promises along every line, one entry per leaf: where each
/// line lies inside the leaf, and every place it crosses the leaf's surface.
struct Promise {
    spans: Vec<Vec<Spans>>,
    crossings: Vec<Vec<Crossing>>,
}

/// Whether a kernel keeps every rule on a case, held to the arithmetic: at
/// every leaf raised and every step, its listing when it keeps one, its
/// triangles closed and uncrossed, what they hold along every line against
/// what the leaves promised, and nothing beyond the box the leaves span; and
/// the whole case run twice, bit for bit.
///
/// A leaf or a step the kernel declines is no answer, but for a case asking
/// for an ellipse declined as unsupported, and a turned leaf with a wall a
/// hair thin declined as no profile, which are counted. A case with a
/// leaf that is no solid, or none this arithmetic covers — a star — holds
/// nothing.
pub fn held_to_arithmetic<K: Kernel>(case: &Case, kernel: &K) -> Result<Measured, Flaw> {
    let mut measured = Measured::default();
    if !case.leaves().all(covered) {
        return Ok(measured);
    }
    let Some(region) = region(case) else {
        return Ok(measured);
    };
    let leaves: Vec<&Leaf> = case.leaves().collect();
    let lines = Lines::across(region.0, region.1, LINES);
    let promise = promise(case, &lines);

    let bodies = match raised(case, kernel) {
        Ok(bodies) => bodies,
        Err(Declined::Profile) if case.has_a_wall_a_hair_thin() => {
            measured.thin += 1;
            return Ok(measured);
        }
        Err(declined) => return declined_on(case, declined, measured),
    };
    for (leaf, body) in bodies.iter().enumerate() {
        let held = Held {
            lines: &lines,
            region,
            leaves: &leaves[leaf..=leaf],
            promised: &promise.spans[leaf],
            crossings: &promise.crossings,
        };
        held.by(kernel, body, &mut measured)?;
    }
    let mut body = bodies[0].clone();
    let mut promised = promise.spans[0].clone();
    for (index, (step, tool)) in case.steps.iter().zip(&bodies[1..]).enumerate() {
        body = match kernel.combined(&body, tool, step.mode) {
            Ok(body) => body,
            Err(declined) => return declined_on(case, declined, measured),
        };
        promised = promised
            .iter()
            .zip(&promise.spans[index + 1])
            .map(|(before, tool)| match step.mode {
                Mode::Add => before.union(tool),
                Mode::Cut => before.without(tool),
            })
            .collect();
        let held = Held {
            lines: &lines,
            region,
            leaves: &leaves[..index + 2],
            promised: &promised,
            crossings: &promise.crossings,
        };
        held.by(kernel, &body, &mut measured)?;
    }

    let again = replayed(case, kernel)?;
    repeatable(&kernel.triangles(&body).0, &kernel.triangles(&again).0)?;
    Ok(measured)
}

/// Checks a case on the exact kernel and ends the test on the first rule it
/// breaks, with the case printed beside the flaw.
pub fn holds_exactly(case: &Case) {
    if let Err(flaw) = held_to_arithmetic(case, &super::Exact) {
        panic!("{:?} broke the rule: {flaw:?}\n{case}", flaw.rule());
    }
}

/// Checks a case through the application's body, which draws finer than the
/// exact kernel's own triangles and declines what it cannot draw, and ends
/// the test on the first rule it breaks.
pub fn holds_through_the_application(case: &Case) {
    if let Err(flaw) = held_to_arithmetic(case, &super::Application) {
        panic!("{:?} broke the rule: {flaw:?}\n{case}", flaw.rule());
    }
}

/// A kernel's decline, judged: counted when the case asks for an ellipse and
/// the kernel says it does not build one, no answer otherwise.
fn declined_on(case: &Case, declined: Declined, mut measured: Measured) -> Result<Measured, Flaw> {
    if declined == Declined::Unsupported && case.asks_for_an_ellipse() {
        measured.declined += 1;
        return Ok(measured);
    }
    Err(Flaw::NoAnswer(Silence::Refused))
}

/// Whether the arithmetic here covers a leaf: a solid prism of any outline
/// but a star, which it can grow and shrink, or a solid turn.
fn covered(leaf: &Leaf) -> bool {
    leaf.is_solid()
        && match leaf {
            Leaf::Prism { outline, .. } => !matches!(outline, Outline::Star { .. }),
            Leaf::Revolution { .. } | Leaf::Turned { .. } => leaf
                .as_turned()
                .is_some_and(|turned| turned.section.is_solid()),
        }
}

/// The box the leaves span, or `None` when one of them is no solid or no
/// prism.
fn region(case: &Case) -> Option<(DVec3, DVec3)> {
    case.leaves()
        .map(|leaf| leaf.is_solid().then(|| leaf.bounds()).flatten())
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .reduce(|(low, high), (other_low, other_high)| (low.min(other_low), high.max(other_high)))
}

fn promise(case: &Case, lines: &Lines) -> Promise {
    let mut crossings = vec![Vec::new(); lines.count()];
    let spans = case
        .leaves()
        .map(|leaf| {
            (0..lines.count())
                .map(|index| {
                    let (origin, direction) = lines.line(index);
                    let stretches = leaf.along(origin, direction).unwrap_or_default();
                    crossings[index].extend(
                        stretches
                            .iter()
                            .flat_map(|stretch| [stretch.from, stretch.to]),
                    );
                    Spans::gathered(
                        stretches
                            .iter()
                            .map(|stretch| (stretch.from.at, stretch.to.at))
                            .collect(),
                    )
                })
                .collect()
        })
        .collect();
    Promise { spans, crossings }
}

fn raised<K: Kernel>(case: &Case, kernel: &K) -> Result<Vec<K::Body>, Declined> {
    case.leaves().map(|leaf| kernel.raised(leaf)).collect()
}

fn replayed<K: Kernel>(case: &Case, kernel: &K) -> Result<K::Body, Flaw> {
    let refused = |_| Flaw::NoAnswer(Silence::Refused);
    let leaves = raised(case, kernel).map_err(refused)?;
    case.steps
        .iter()
        .zip(&leaves[1..])
        .try_fold(leaves[0].clone(), |body, (step, tool)| {
            kernel.combined(&body, tool, step.mode).map_err(refused)
        })
}

/// One body held against what it was promised, and the leaves it was made
/// of.
struct Held<'a> {
    lines: &'a Lines,
    region: (DVec3, DVec3),
    leaves: &'a [&'a Leaf],
    promised: &'a [Spans],
    crossings: &'a [Vec<Crossing>],
}

impl Held<'_> {
    fn by<K: Kernel>(
        &self,
        kernel: &K,
        body: &K::Body,
        measured: &mut Measured,
    ) -> Result<(), Flaw> {
        if let Some((listing, reach)) = kernel.listing(body) {
            listed(&listing, reach)?;
        }
        self.exactly(kernel, body)?;
        let (triangles, tolerance) = kernel.triangles(body);
        closed(&triangles)?;
        uncrossed(&triangles)?;
        self.along_every_line(&triangles, tolerance, measured)?;
        let (low, high) = self.region;
        let reach = low.abs().max(high.abs()).max_element();
        // Decision 8 takes a wall of the second operand for the first's a
        // hair off and moves the whole operand with it: a side the move does
        // not slide along itself ends up to that hair past where its leaf
        // drew it.
        let merged = Scale::HAIR * Scale::of(reach).eps();
        within_reach(
            (low - merged, high + merged),
            self.lines,
            self.promised,
            &triangles,
        )
    }

    /// Whether the exact body holds along every line what was promised there,
    /// before any triangle is laid: the kernel's own answer, held as tightly
    /// as the flats are, where it can tell and no line grazes a curved wall.
    fn exactly<K: Kernel>(&self, kernel: &K, body: &K::Body) -> Result<(), Flaw> {
        let (low, high) = self.region;
        let room = EXACTLY * low.abs().max(high.abs()).max_element().max(1.0);
        for (index, promise) in self.promised.iter().enumerate() {
            if self.grazes(index, promise) {
                continue;
            }
            let (origin, direction) = self.lines.line(index);
            let Some(crossings) = kernel.crossings(body, origin, direction) else {
                continue;
            };
            let found = Spans::swept(crossings);
            let gap = promise.without(&found).length()
                + found.without(promise).length()
                + found.surplus().abs();
            if gap > room {
                return Err(Flaw::Spans(Along {
                    origin,
                    direction,
                    promised: promise.length(),
                    enclosed: found.length() + found.surplus(),
                }));
            }
        }
        Ok(())
    }

    /// Whether a line ends a promised stretch on a curved wall it meets at a
    /// slant too shallow to measure along.
    fn grazes(&self, index: usize, promise: &Spans) -> bool {
        promise
            .stretches()
            .iter()
            .flat_map(|&(from, to)| [from, to])
            .map(|at| self.crossed_at(index, at))
            .any(|end| end.curved && end.cosine < GRAZING)
    }

    /// Whether the triangles hold along every line what was promised there,
    /// but for the room their tolerance takes along it.
    fn along_every_line(
        &self,
        triangles: &[Triangle],
        tolerance: f64,
        measured: &mut Measured,
    ) -> Result<(), Flaw> {
        let found = self.lines.inside(triangles);
        let (low, high) = self.region;
        let rounding = Scale::of(low.abs().max(high.abs()).max_element()).eps();
        for (index, (promise, measure)) in self.promised.iter().zip(&found).enumerate() {
            if self.grazes(index, promise) {
                measured.grazing += 1;
                continue;
            }
            measured.held += 1;
            let room = rounding + self.near_the_leaves(index, tolerance);
            let gap = promise.without(measure).length()
                + measure.without(promise).length()
                + measure.surplus().abs();
            if gap > room {
                let (origin, direction) = self.lines.line(index);
                return Err(Flaw::Volume {
                    promised: self.lines.volume(self.promised),
                    enclosed: enclosed(triangles),
                    worst: Some(Along {
                        origin,
                        direction,
                        promised: promise.length(),
                        enclosed: measure.length() + measure.surplus(),
                    }),
                });
            }
        }
        Ok(())
    }

    /// How much of line `index` lies within `by` of some leaf's boundary:
    /// inside the leaf grown by `by` and not inside it shrunk by `by`.
    fn near_the_leaves(&self, index: usize, by: f64) -> f64 {
        let (origin, direction) = self.lines.line(index);
        let length = |leaf: &Leaf, by: f64| {
            leaf.along_grown(origin, direction, by)
                .unwrap_or_default()
                .iter()
                .map(|stretch| stretch.to.at - stretch.from.at)
                .sum::<f64>()
        };
        self.leaves
            .iter()
            .map(|leaf| length(leaf, by) - length(leaf, -by))
            .sum()
    }

    /// The leaf's crossing a promise ends at: the very distance, since `union`
    /// and `without` only ever keep the ends they were given. Where several
    /// leaves cross at that place, the one crossed least squarely.
    fn crossed_at(&self, line: usize, at: f64) -> Crossing {
        self.crossings[line]
            .iter()
            .filter(|crossing| crossing.at == at)
            .copied()
            .min_by(|one, other| one.cosine.total_cmp(&other.cosine))
            .unwrap_or(Crossing {
                at,
                cosine: 1.0,
                curved: false,
            })
    }
}

impl Case {
    /// Whether a turned leaf of the case has a wall a hair thin, which the
    /// exact kernel may decline to raise and the application turns on the
    /// flats.
    pub fn has_a_wall_a_hair_thin(&self) -> bool {
        self.leaves()
            .filter_map(Leaf::as_turned)
            .any(|turned| turned.has_a_wall_a_hair_thin())
    }

    /// Whether the case may ask the exact kernel for a curve it does not
    /// build (#533): a plane meeting a cylinder neither square to its axis
    /// nor along it, an ellipse, or two cylinders whose axes are neither
    /// parallel nor square, a skew meeting. Turned leaves bring them, with
    /// an end that stops off the quarter turns, or an axis slanted in its
    /// plane. The kernel may decline such a case as unsupported; it may
    /// also hold it, the leaves never meeting there.
    pub fn asks_for_an_ellipse(&self) -> bool {
        let (planes, cylinders): (Vec<DVec3>, Vec<DVec3>) = self.leaves().map(walls).fold(
            (Vec::new(), Vec::new()),
            |(mut planes, mut cylinders), (more, round)| {
                planes.extend(more);
                cylinders.extend(round);
                (planes, cylinders)
            },
        );
        cylinders.iter().any(|axis| {
            planes.iter().any(|normal| oblique(*normal, *axis))
                || cylinders.iter().any(|other| oblique(*other, *axis))
        })
    }
}

/// The normals of the planes a leaf is bounded by, and the axes of its
/// cylinders.
fn walls(leaf: &Leaf) -> (Vec<DVec3>, Vec<DVec3>) {
    if let Some(turned) = leaf.as_turned() {
        let swept = turned.swept(1.0);
        let mut planes = vec![swept.along];
        if !turned.is_whole() {
            let end = DVec2::from_angle(turned.angle());
            planes.push(swept.onward);
            planes.push(swept.onward * end.x - swept.out * end.y);
        }
        return (planes, vec![swept.along]);
    }
    let Leaf::Prism { plane, outline, .. } = leaf else {
        return (Vec::new(), Vec::new());
    };
    let (_, u, v) = plane.frame();
    let normal = u.cross(v);
    match outline {
        Outline::Rectangle { .. } => (vec![normal, u, v], Vec::new()),
        Outline::Circle { .. } | Outline::Ring { .. } => (vec![normal], vec![normal]),
        Outline::Rounded { .. } | Outline::Slot { .. } => (vec![normal, u, v], vec![normal]),
        Outline::Star { corners, .. } => {
            let sides = (0..corners.len()).map(|index| {
                let side = corners[(index + 1) % corners.len()] - corners[index];
                u * side.y - v * side.x
            });
            (std::iter::once(normal).chain(sides).collect(), Vec::new())
        }
    }
}

/// Whether two directions are neither parallel nor square to each other,
/// by more than the rounding of a direction of unit length.
fn oblique(one: DVec3, other: DVec3) -> bool {
    let (one, other) = (one.normalize(), other.normalize());
    one.dot(other).abs() > 1e-12 && one.cross(other).length() > 1e-12
}
