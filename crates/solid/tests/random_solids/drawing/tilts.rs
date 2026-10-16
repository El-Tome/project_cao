//! Tools drawn on planes leaning off the planes of the origin (#552): what a
//! plane meeting a cylinder or a cone at a slant cuts along a conic.
//!
//! The draw is the slanted one, its turned leaves still about lines of the
//! planes of the origin, and to it come tools drawn from a leaf before them
//! on a plane leaning about a world axis: through a block, a slanted cut, a
//! hole or a boss square to the slanted face an earlier cut left, a bore at
//! a slant, a leaning block; on a shaft or a tube, a slanted cut through its
//! middle or through a rim point leaning about the rim's tangent, a plane
//! along its axis at its radius — a line, not an ellipse — and a flat; on a
//! cone, a flat along its axis, a slant steeper than its slope, a plane
//! parallel to a ruling, a plane through its tip flatter, steeper or as
//! steep as its slope, one through a rim point, and a second slant. Off the
//! lattice the angles come a hair from parallel, from square, and from the
//! cone's own slope, where the kernel has to decide which curve it cuts.

use cao_solid::soundness::Lines;
use glam::{DVec2, DVec3};

use super::super::{Leaf, Mode, Outline, Plane, Step};
use super::turns::{Line, world_axis};
use super::{Drawing, in_steps, shifted};

/// The hairs a campaign leans a plane by, in degrees, off parallel, off
/// square, or off a cone's own slope: either side of the kernel's bound of
/// half a billionth of a radian, of the reach's own rounding, and far off it.
const HAIRS: [f64; 5] = [2e-8, 4e-8, 1e-7, 1e-5, 1e-3];

/// The least cosine a leaning plane's normal and its two axes make with the
/// harness's lines of measure: a line running nearly along a face, or along
/// a wall's axis, crosses it where a hair moves the crossing far.
const OFF_THE_LINES: f64 = 0.05;

/// How a plane leans against a world axis: its angle with the axis, nought
/// when it holds the axis's direction, and how far it stands from square to
/// it, each written so a hair off either keeps its digits.
#[derive(Clone, Copy, Debug)]
pub(super) struct Lean {
    with_the_axis: f64,
    off_square: f64,
}

impl Lean {
    pub(super) fn at(with_the_axis: f64) -> Lean {
        Lean {
            with_the_axis,
            off_square: 90.0 - with_the_axis,
        }
    }

    pub(super) fn off_square(by: f64) -> Lean {
        Lean {
            with_the_axis: 90.0 - by,
            off_square: by,
        }
    }
}

/// A shaft, a bore or a wall of a tube to draw from: a point of its axis,
/// the world axis it runs along, its radius, and the levels along that axis
/// its ends stand at.
#[derive(Clone, Copy, Debug)]
struct Round {
    point: DVec3,
    axis: usize,
    radius: f64,
    levels: [f64; 2],
}

/// A cone to draw from: its tip, the world axis it turns about, its slope
/// in degrees off the axis, and its two rims, each a level along the axis
/// and a radius.
#[derive(Clone, Copy, Debug)]
struct Cone {
    tip: DVec3,
    axis: usize,
    slope: f64,
    rims: [(f64, f64); 2],
}

impl Cone {
    fn at(&self, level: f64) -> DVec3 {
        let mut point = self.tip;
        point[self.axis] = level;
        point
    }
}

impl Plane {
    /// A plane through `origin` holding the world axis `holds`, leaning by
    /// `degrees` off the plane of the origin it starts from: XY about X or
    /// Y, XZ about Z.
    pub fn leaning(holds: usize, degrees: f64, origin: DVec3) -> Plane {
        let turn = match holds {
            0 => DVec3::new(degrees, 0.0, 0.0),
            1 => DVec3::new(0.0, degrees, 0.0),
            _ => DVec3::new(90.0, degrees, 0.0),
        };
        Plane::Tilted { origin, turn }
    }
}

/// The world axis the normal of a plane leaning about `holds` starts along.
fn starting_normal(holds: usize) -> usize {
    if holds == 2 { 1 } else { 2 }
}

/// The direction the harness's lines of measure run along.
fn lines_direction() -> DVec3 {
    Lines::across(DVec3::ZERO, DVec3::ONE, 1).line(0).1
}

/// Whether a plane's two axes and its normal all stand clear of square to
/// the lines of measure.
pub fn clear_of_the_lines(plane: Plane) -> bool {
    let (_, u, v) = plane.frame();
    let direction = lines_direction();
    [u, v, plane.normal()]
        .iter()
        .all(|axis| axis.dot(direction).abs() >= OFF_THE_LINES)
}

fn unit(axis: usize) -> DVec3 {
    DVec3::AXES[axis]
}

/// The point at `level` along world axis `axis` through `point`.
fn on_axis(point: DVec3, axis: usize, level: f64) -> DVec3 {
    let mut moved = point;
    moved[axis] = level;
    moved
}

impl Drawing {
    /// Whether the draw brings tools on leaning planes (#552). Every number
    /// those take is drawn only then.
    pub(super) fn tilted(&self) -> bool {
        matches!(self.among, super::Among::Turned { tilted: true, .. })
    }

    /// A fresh block on a leaning plane, to start a case with.
    pub(super) fn leaning_block(&mut self) -> Leaf {
        let origin = DVec3::new(
            self.coordinate(0.0, 6.0),
            self.coordinate(0.0, 6.0),
            self.coordinate(0.0, 6.0),
        );
        let plane = self.generic(origin).unwrap_or(Plane::Xy(origin.z));
        let low = self.point() - DVec2::splat(4.0 * self.scale);
        let size = DVec2::new(self.length(1.0, 8.0), self.length(1.0, 8.0));
        Leaf::Prism {
            plane,
            outline: Outline::Rectangle {
                low,
                high: low + size,
            },
            height: self.height(),
        }
    }

    /// A step on a leaning plane drawn from a leaf before it, or nothing
    /// when every leaning the draw tried stood square to the lines of
    /// measure.
    pub(super) fn tilted_step(&mut self, start: &Leaf, steps: &[Step]) -> Option<Step> {
        let leaves: Vec<&Leaf> = std::iter::once(start)
            .chain(steps.iter().map(|step| &step.tool))
            .collect();
        let coned: Vec<&Leaf> = leaves
            .iter()
            .copied()
            .filter(|leaf| !cones_of(leaf).is_empty())
            .collect();
        let rounded: Vec<&Leaf> = leaves
            .iter()
            .copied()
            .filter(|leaf| !rounds_of(leaf, self.scale).is_empty())
            .collect();
        let target = if !coned.is_empty() && self.random.chance(0.75) {
            *self.random.pick(&coned)
        } else if !rounded.is_empty() && self.random.chance(0.4) {
            *self.random.pick(&rounded)
        } else {
            *self.random.pick(&leaves)
        }
        .clone();
        let face = steps.iter().rev().find_map(|step| match step {
            Step {
                mode: Mode::Cut,
                tool:
                    Leaf::Prism {
                        plane: plane @ Plane::Tilted { .. },
                        outline: Outline::Rectangle { .. },
                        height,
                    },
            } => Some((*plane, height.signum())),
            _ => None,
        });
        let cones = cones_of(&target);
        let rounds = rounds_of(&target, self.scale);
        if !cones.is_empty() && (rounds.is_empty() || self.random.chance(0.7)) {
            let cone = *self.random.pick(&cones);
            return self.on_a_cone(&target, cone, face.is_some());
        }
        if !rounds.is_empty()
            && !matches!(
                target,
                Leaf::Prism {
                    outline: Outline::Rectangle { .. },
                    ..
                }
            )
        {
            let round = *self.random.pick(&rounds);
            return self.on_a_round(&target, round);
        }
        match &target {
            Leaf::Prism {
                plane,
                outline: Outline::Rectangle { .. } | Outline::Rounded { .. } | Outline::Slot { .. },
                ..
            } if !matches!(plane, Plane::Tilted { .. }) => self.on_a_block(&target, face),
            _ => self.slanted_cut(&target),
        }
    }

    /// An angle off square for a plane against a world axis: on the lattice
    /// a step of fifteen degrees short of square and of parallel; in a
    /// campaign a degree's step, a real angle, or a hair from square or from
    /// parallel.
    fn lean(&mut self) -> Lean {
        if !self.off_the_lattice() {
            return Lean::off_square(self.random.on_lattice(15.0, 75.0, 15.0));
        }
        match self.random.below(10) {
            0..=4 => Lean::off_square(self.random.on_lattice(1.0, 89.0, 1.0)),
            5 | 6 => Lean::off_square(self.random.between(1.0, 89.0)),
            7 | 8 => Lean::off_square(*self.random.pick(&HAIRS)),
            _ => Lean::at(*self.random.pick(&HAIRS)),
        }
    }

    /// A hair off a lean in a campaign, a quarter of the time, either way;
    /// the lean itself otherwise.
    fn a_hair_off(&mut self, lean: Lean) -> Lean {
        if !self.off_the_lattice() || !self.random.chance(0.25) {
            return lean;
        }
        let hair = *self.random.pick(&HAIRS);
        let hair = if self.random.chance(0.5) { hair } else { -hair };
        Lean::at(lean.with_the_axis + hair)
    }

    /// A plane through `origin` holding `holds`, leaning against the world
    /// axis `axis` square to it by `lean`, either way; nothing when it, and
    /// the same leaning the other way, stand square to the lines of measure.
    pub(super) fn leaning_against(
        &mut self,
        axis: usize,
        holds: usize,
        lean: Lean,
        origin: DVec3,
    ) -> Option<Plane> {
        let degrees = if starting_normal(holds) == axis {
            lean.off_square
        } else {
            lean.with_the_axis
        };
        let first = if self.random.chance(0.5) { 1.0 } else { -1.0 };
        [first, -first]
            .into_iter()
            .map(|sign| Plane::leaning(holds, sign * degrees, origin))
            .find(|plane| clear_of_the_lines(*plane))
    }

    /// A world axis square to `axis`, drawn.
    pub(super) fn square_to(&mut self, axis: usize) -> usize {
        let others = [(axis + 1) % 3, (axis + 2) % 3];
        *self.random.pick(&others)
    }

    /// A plane through `origin` leaning about any world axis, now and then
    /// about two, clear of the lines of measure.
    pub(super) fn generic(&mut self, origin: DVec3) -> Option<Plane> {
        if self.random.chance(0.1) {
            let turn = DVec3::new(
                self.random.on_lattice(-75.0, 75.0, 15.0),
                self.random.on_lattice(-75.0, 75.0, 15.0),
                0.0,
            );
            let plane = Plane::Tilted { origin, turn };
            if clear_of_the_lines(plane) && turn.x != 0.0 && turn.y != 0.0 {
                return Some(plane);
            }
        }
        let axis = self.random.below(3);
        let holds = self.square_to(axis);
        let lean = self.lean();
        self.leaning_against(axis, holds, lean, origin)
    }

    /// A prism on `plane` taking out everything of `target` on the side of
    /// the plane its normal points to, or the other one: a rectangle
    /// covering the box the target spans as seen on the plane, half a unit
    /// over every side, on the half lattice.
    fn covering(&mut self, plane: Plane, target: &Leaf) -> Leaf {
        let (origin, u, v) = plane.frame();
        let normal = plane.normal();
        let half = 0.5 * self.scale;
        let (low, high) = target.bounds().unwrap_or((origin, origin));
        let seen: Vec<DVec3> = (0..8)
            .map(|corner| {
                let at = DVec3::new(
                    if corner & 1 == 0 { low.x } else { high.x },
                    if corner & 2 == 0 { low.y } else { high.y },
                    if corner & 4 == 0 { low.z } else { high.z },
                ) - origin;
                DVec3::new(at.dot(u), at.dot(v), at.dot(normal))
            })
            .collect();
        let least = seen.iter().copied().fold(DVec3::MAX, DVec3::min) - half;
        let most = seen.iter().copied().fold(DVec3::MIN, DVec3::max) + half;
        let down = |value: f64| in_steps(value, half).floor() * half;
        let up = |value: f64| in_steps(value, half).ceil() * half;
        let height = if self.random.chance(0.5) {
            up(most.z).max(half)
        } else {
            -up(-least.z).max(half)
        };
        Leaf::Prism {
            plane,
            outline: Outline::Rectangle {
                low: DVec2::new(down(least.x), down(least.y)),
                high: DVec2::new(up(most.x), up(most.y)),
            },
            height,
        }
    }

    /// A cut through `target` along `plane`, moved off it by a hair now and
    /// then; nothing when no plane was drawn.
    pub(super) fn cut_along(&mut self, plane: Option<Plane>, target: &Leaf) -> Option<Step> {
        let plane = shifted(plane?, self.nudge());
        Some(Step::cut(self.covering(plane, target)))
    }

    /// A cut on a plane leaning about any world axis through the middle of
    /// the target's box, on the lattice.
    fn slanted_cut(&mut self, target: &Leaf) -> Option<Step> {
        let (low, high) = target.bounds()?;
        let middle = ((low + high) / 2.0).map(|at| in_steps(at, self.scale).round() * self.scale);
        let plane = self.generic(middle);
        self.cut_along(plane, target)
    }

    /// A tool drawn on a block: a slanted cut, a hole or a boss square to
    /// the slanted face an earlier cut left, a bore at a slant through it,
    /// or a leaning block crossing it.
    fn on_a_block(&mut self, block: &Leaf, face: Option<(Plane, f64)>) -> Option<Step> {
        let Leaf::Prism { plane, height, .. } = block else {
            return None;
        };
        let normal = world_axis(plane.normal());
        let (low, high) = block.bounds()?;
        match (self.random.below(20), face) {
            (0..=5, _) | (6..=10, None) | (11 | 12, None) => {
                let holds = self.square_to(normal);
                let point = if self.random.chance(0.5) {
                    let top = plane.to_world(DVec2::ZERO) + plane.normal() * *height;
                    let mut edge = *self.random.pick(&[low, high]);
                    edge[normal] = top[normal];
                    edge
                } else {
                    self.inside(low, high)
                };
                let lean = self.lean();
                let plane = self.leaning_against(normal, holds, lean, point);
                self.cut_along(plane, block)
            }
            (6..=10, Some(face)) => self.on_a_slanted_face(block, face, Mode::Cut),
            (11 | 12, Some(face)) => self.on_a_slanted_face(block, face, Mode::Add),
            (13..=15, _) => {
                let middle = (low + high) / 2.0;
                let plane = self.generic(middle)?;
                let reach = (high - low).length() / 2.0 + 0.5 * self.scale;
                let center = DVec2::new(self.coordinate(-1.0, 1.0), self.coordinate(-1.0, 1.0));
                let through = shifted(plane, -reach);
                Some(Step::cut(Leaf::Prism {
                    plane: through,
                    outline: Outline::Circle {
                        center,
                        radius: self.length(0.5, 2.0),
                        from: 0.0,
                    },
                    height: 2.0 * reach,
                }))
            }
            _ => {
                let point = self.inside(low, high);
                let plane = self.generic(point)?;
                let low = DVec2::new(self.coordinate(-3.0, 0.0), self.coordinate(-3.0, 0.0));
                let size = DVec2::new(self.length(1.0, 6.0), self.length(1.0, 6.0));
                let tool = Leaf::Prism {
                    plane,
                    outline: Outline::Rectangle {
                        low,
                        high: low + size,
                    },
                    height: self.height(),
                };
                Some(if self.random.chance(0.6) {
                    Step::cut(tool)
                } else {
                    Step::add(tool)
                })
            }
        }
    }

    /// A point on the lattice inside a box.
    pub(super) fn inside(&mut self, low: DVec3, high: DVec3) -> DVec3 {
        let unit = self.scale;
        let mut along =
            |from: f64, to: f64| self.random.on_lattice(from / unit, to / unit, 0.5) * unit;
        DVec3::new(
            along(low.x, high.x),
            along(low.y, high.y),
            along(low.z, high.z),
        )
    }

    /// A hole, or a boss, square to the slanted face a cut left on `plane`,
    /// the cut having taken the side `side` of it: a circle, now and then a
    /// slot or a ring, about a place of the face inside the block, from a
    /// margin over the face down through the floor, blind, or a hair from
    /// it; now and then its wall tangent to a side of the block, or its
    /// ellipse in the floor reaching the floor's edge, either a hair off.
    fn on_a_slanted_face(
        &mut self,
        block: &Leaf,
        (plane, side): (Plane, f64),
        mode: Mode,
    ) -> Option<Step> {
        let Plane::Tilted { origin, turn } = plane else {
            return None;
        };
        let normal = plane.normal();
        let (low, high) = block.bounds()?;
        let radius = self.length(0.5, 2.0);
        let margin = self.length(0.5, 1.0);
        let mut center = None;
        for _ in 0..4 {
            let place = self.inside(low, high);
            let on = place - normal * (place - origin).dot(normal);
            if (on - low).min_element() > radius && (high - on).min_element() > radius {
                center = Some(on);
                break;
            }
        }
        let mut center = center?;
        let down = -normal * side;
        let (exit, floor) = leaving(center, down, low, high)?;
        let placed = self.random.chance(0.15);
        if placed {
            let held = (0..3).find(|&axis| normal[axis].abs() <= 1e-12);
            match held {
                Some(axis) => {
                    let wall = if self.random.chance(0.5) {
                        low[axis] + radius
                    } else {
                        high[axis] - radius
                    };
                    center[axis] = wall + self.nudge();
                }
                None => {
                    let edge = (floor + 1 + self.random.below(2)) % 3;
                    let toward = if self.random.chance(0.5) { 1.0 } else { -1.0 };
                    let across = unit(edge) * toward;
                    let skew = down.dot(across) / down.dot(unit(floor));
                    let reaches = radius * (1.0 + skew * skew).sqrt();
                    let lands = center + down * exit;
                    let bound = if toward > 0.0 { high[edge] } else { low[edge] };
                    let missing = (bound * toward - lands.dot(across) - reaches) + self.nudge();
                    let (_, u, v) = plane.frame();
                    let slide = if u.dot(across).abs() >= v.dot(across).abs() {
                        u
                    } else {
                        v
                    };
                    let moved = slide.dot(across) - skew * slide.dot(unit(floor));
                    let slid = center + slide * (missing / moved);
                    if moved.abs() > 0.1 && slid.cmpge(low).all() && slid.cmple(high).all() {
                        center = slid;
                    }
                }
            }
        }
        let (exit, floor) = leaving(center, down, low, high)?;
        let depth = if mode == Mode::Add || placed {
            exit + 0.5 * self.scale
        } else {
            match self.random.below(20) {
                0..=11 => exit + 0.5 * self.scale,
                12..=16 => self.up_to(exit - 0.5 * self.scale).unwrap_or(exit / 2.0),
                _ => {
                    let slant = down.dot(unit(floor)).abs();
                    let spread = radius * (1.0 - slant * slant).max(0.0).sqrt() / slant;
                    (exit - spread - self.hair().abs() / slant).max(exit / 2.0)
                }
            }
        };
        let outline = match self.random.below(10) {
            0 => Outline::Slot {
                from: DVec2::ZERO,
                to: DVec2::X * self.length(0.5, 2.0),
                radius,
            },
            1 => Outline::Ring {
                center: DVec2::ZERO,
                outer: radius + self.length(0.5, 1.0),
                inner: radius,
            },
            _ => Outline::Circle {
                center: DVec2::ZERO,
                radius,
                from: 0.0,
            },
        };
        let tool = if mode == Mode::Cut {
            Leaf::Prism {
                plane: Plane::Tilted {
                    origin: center + normal * side * margin,
                    turn,
                },
                outline,
                height: -side * (margin + depth),
            }
        } else {
            Leaf::Prism {
                plane: Plane::Tilted {
                    origin: center + down * margin,
                    turn,
                },
                outline,
                height: side * (margin + self.length(0.5, 3.0)),
            }
        };
        Some(Step { mode, tool })
    }

    /// A tool drawn on a shaft, a bore or a tube: a slanted cut through the
    /// middle of its axis or through a rim point leaning about the rim's
    /// tangent, a plane along its axis at its radius, which touches it along
    /// a line, or a flat inside it; each moved a hair off now and then.
    fn on_a_round(&mut self, target: &Leaf, round: Round) -> Option<Step> {
        let [bottom, top] = round.levels;
        let across = self.square_to(round.axis);
        let away = unit(across) * if self.random.chance(0.5) { 1.0 } else { -1.0 };
        let tangent = 3 - round.axis - across;
        match self.random.below(10) {
            0 | 1 | 8 | 9 => {
                let middle = on_axis(round.point, round.axis, (bottom + top) / 2.0);
                let holds = self.square_to(round.axis);
                let lean = self.lean();
                let plane = self.leaning_against(round.axis, holds, lean, middle);
                self.cut_along(plane, target)
            }
            2 | 3 => {
                let level = *self.random.pick(&round.levels);
                let rim = on_axis(round.point, round.axis, level) + away * round.radius;
                let lean = self.lean();
                let plane = self.leaning_against(round.axis, tangent, lean, rim);
                self.cut_along(plane, target)
            }
            kind => {
                let distance = if kind < 6 {
                    round.radius
                } else {
                    self.up_to(round.radius - 0.5 * self.scale).unwrap_or(0.0)
                };
                let middle = on_axis(round.point, round.axis, (bottom + top) / 2.0);
                let at = middle + away * distance;
                let plane = if self.off_the_lattice() && self.random.chance(0.3) {
                    let lean = Lean::at(*self.random.pick(&HAIRS));
                    self.leaning_against(round.axis, tangent, lean, at)
                } else {
                    Some(square_through(across, at))
                };
                self.cut_along(plane, target)
            }
        }
    }

    /// A tool drawn on a cone: a flat along its axis between its rims or at
    /// one of them, a slant steeper than its slope, a plane parallel to a
    /// ruling, a plane through its tip flatter, steeper or as steep as its
    /// slope — touching it along a ruling then — a slant through a rim point
    /// leaning about the rim's tangent, or a second slant after a first.
    fn on_a_cone(&mut self, target: &Leaf, cone: Cone, slanted_before: bool) -> Option<Step> {
        let across = self.square_to(cone.axis);
        let away = unit(across) * if self.random.chance(0.5) { 1.0 } else { -1.0 };
        let tangent = 3 - cone.axis - across;
        let [(first, first_radius), (last, last_radius)] = cone.rims;
        let (narrowest, widest) = (first_radius.min(last_radius), first_radius.max(last_radius));
        let middle = cone.at((first + last) / 2.0);
        let steeper = |slope: f64| (slope + 90.0) / 2.0;
        let near = (cone.tip - middle).length() <= 30.0 * self.scale;
        match self.random.below(20) {
            0..=4 => {
                let distance = match self.random.below(3) {
                    0 => widest + self.nudge(),
                    1 => narrowest + self.nudge(),
                    _ => self.random.between(narrowest, widest),
                };
                let at = middle + away * distance;
                let plane = if self.off_the_lattice() && self.random.chance(0.3) {
                    let lean = Lean::at(*self.random.pick(&HAIRS));
                    self.leaning_against(cone.axis, tangent, lean, at)
                } else {
                    Some(square_through(across, at))
                };
                self.cut_along(plane, target)
            }
            5..=7 => {
                let lean = self.lean();
                let lean = if lean.with_the_axis > cone.slope {
                    lean
                } else {
                    Lean::at(steeper(cone.slope))
                };
                let lean = self.a_hair_off(lean);
                let holds = self.square_to(cone.axis);
                let plane = self.leaning_against(cone.axis, holds, lean, middle);
                self.cut_along(plane, target)
            }
            8..=10 => {
                let lean = self.a_hair_off(Lean::at(cone.slope));
                let holds = self.square_to(cone.axis);
                let plane = self.leaning_against(cone.axis, holds, lean, middle);
                self.cut_along(plane, target)
            }
            11..=13 if near => {
                let lean = match self.random.below(3) {
                    0 => Lean::at(cone.slope / 2.0),
                    1 => Lean::at(cone.slope),
                    _ => Lean::at(steeper(cone.slope)),
                };
                let lean = self.a_hair_off(lean);
                let holds = self.square_to(cone.axis);
                let plane = self.leaning_against(cone.axis, holds, lean, cone.tip);
                self.cut_along(plane, target)
            }
            14 | 15 if near => {
                let plane =
                    self.leaning_against(cone.axis, tangent, Lean::at(cone.slope), cone.tip);
                self.cut_along(plane, target)
            }
            16 | 17 => {
                let (level, radius) = *self.random.pick(&cone.rims);
                let rim = cone.at(level) + away * radius;
                let lean = self.lean();
                let plane = self.leaning_against(cone.axis, tangent, lean, rim);
                self.cut_along(plane, target)
            }
            _ => {
                let level = self.random.between(first.min(last), first.max(last));
                let lean = Lean::at(steeper(cone.slope));
                let holds = if slanted_before {
                    tangent
                } else {
                    self.square_to(cone.axis)
                };
                let plane = self.leaning_against(cone.axis, holds, lean, cone.at(level));
                self.cut_along(plane, target)
            }
        }
    }
}

/// The plane of the origin square to world axis `axis` through `at`.
pub(super) fn square_through(axis: usize, at: DVec3) -> Plane {
    match axis {
        0 => Plane::Yz(at.x),
        1 => Plane::Xz(at.y),
        _ => Plane::Xy(at.z),
    }
}

/// How far the ray from `from` along `direction` runs inside the box from
/// `low` to `high` before it leaves it, and the world axis of the side it
/// leaves through; nothing when it starts outside.
pub(super) fn leaving(
    from: DVec3,
    direction: DVec3,
    low: DVec3,
    high: DVec3,
) -> Option<(f64, usize)> {
    (0..3)
        .filter(|&axis| direction[axis] != 0.0)
        .map(|axis| {
            let bound = if direction[axis] > 0.0 {
                high[axis]
            } else {
                low[axis]
            };
            ((bound - from[axis]) / direction[axis], axis)
        })
        .min_by(|one, other| one.0.total_cmp(&other.0))
        .filter(|(exit, _)| *exit > 0.0)
}

/// The shafts, bores and tubes a leaf brings, about lines along the world
/// axes: a circle or a ring raised on a plane of the origin, or a level band
/// of a turn whose axis does not lean, of some radius.
fn rounds_of(leaf: &Leaf, unit: f64) -> Vec<Round> {
    match leaf {
        Leaf::Prism {
            plane,
            outline,
            height,
        } if !matches!(plane, Plane::Tilted { .. }) => {
            let radii = match outline {
                Outline::Circle { radius, .. } => vec![*radius],
                Outline::Ring { outer, inner, .. } => vec![*outer, *inner],
                _ => return Vec::new(),
            };
            let (Outline::Circle { center, .. } | Outline::Ring { center, .. }) = outline else {
                return Vec::new();
            };
            let point = plane.to_world(*center);
            let axis = world_axis(plane.normal());
            let ends = [point[axis], (point + plane.normal() * *height)[axis]];
            let levels = [ends[0].min(ends[1]), ends[0].max(ends[1])];
            radii
                .into_iter()
                .map(|radius| Round {
                    point,
                    axis,
                    radius,
                    levels,
                })
                .collect()
        }
        Leaf::Turned {
            plane,
            axis,
            section,
            ..
        } if axis.lean == 0.0 => {
            let line = Line::of(*plane, *axis);
            let ends = section.ends();
            (0..section.bands.len())
                .filter(|&index| !section.slopes_at(index))
                .flat_map(|index| {
                    let [_, low, high] = section.bands[index];
                    let levels =
                        [ends[index], ends[index + 1]].map(|at| line.at(*axis, at)[line.axis]);
                    [low.abs(), high.abs()]
                        .into_iter()
                        .filter(|radius| *radius > unit / 4.0)
                        .map(move |radius| Round {
                            point: line.point,
                            axis: line.axis,
                            radius,
                            levels: [levels[0].min(levels[1]), levels[0].max(levels[1])],
                        })
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

/// The cones a turn whose axis does not lean brings: one for every slanted
/// edge of its section running some length along the axis.
fn cones_of(leaf: &Leaf) -> Vec<Cone> {
    let Leaf::Turned {
        plane,
        axis,
        section,
        ..
    } = leaf
    else {
        return Vec::new();
    };
    if axis.lean != 0.0 {
        return Vec::new();
    }
    let line = Line::of(*plane, *axis);
    let ends = section.ends();
    let level = |along: f64| line.at(*axis, along)[line.axis];
    let mut cones = Vec::new();
    for index in 0..section.bands.len() {
        let (start, end) = section.edges(index);
        for edge in 0..2 {
            let (from, to) = (start[edge].abs(), end[edge].abs());
            let run = ends[index + 1] - ends[index];
            if from == to || run <= 0.0 || from.max(to) <= 0.0 {
                continue;
            }
            let tip = ends[index] - from * run / (to - from);
            cones.push(Cone {
                tip: on_axis(line.point, line.axis, level(tip)),
                axis: line.axis,
                slope: (to - from).abs().atan2(run).to_degrees(),
                rims: [(level(ends[index]), from), (level(ends[index + 1]), to)],
            });
        }
    }
    cones
}
