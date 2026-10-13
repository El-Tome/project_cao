//! The draw of ellipses leaning (#571): planes at a slant through elliptic
//! walls, as `tilts.rs` draws them through round ones. On an elliptic boss
//! or bore, a slanted cut through its middle, a plane through the point its
//! rim reaches furthest along a world axis leaning about the rim's tangent
//! there, a plane along its axis at that reach or inside it, a leaning block,
//! a bore at a slant, now and then a cone about its axis; on a block, the
//! slanted cut that leaves a face, and an elliptic hole or boss square to
//! that face. The leans come on the
//! lattice, on a degree's step, at a real angle, and a hair from square or
//! from along: in decision 11's band, where the kernel lays the plane square
//! or along, and in the band past it where it builds the step and warns.

use glam::{DVec2, DVec3};

use super::super::super::{Case, Leaf, Mode, Outline, Oval, Plane, Step};
use super::super::Drawing;
use super::super::shifted;
use super::super::tilts::{Lean, leaving, square_through};
use super::{in_plane, levels, ovals_of};

/// Leans a hair from square or from along, in degrees, that move a tool as
/// large as the part by under twenty tolerances: decision 11 lays the plane
/// square or along.
const LAID: [f64; 4] = [2e-8, 4e-8, 1e-7, 3e-7];

/// Leans a hair from square or from along, in degrees, past what decision 11
/// lays and under a sine of a thousandth: built as drawn, and noted unstable.
const WARNED: [f64; 5] = [1e-5, 1e-4, 1e-3, 1e-2, 5e-2];

impl Drawing {
    /// A case of planes leaning through elliptic walls: an elliptic prism,
    /// a block or a plate to start, and steps on leaning planes drawn from
    /// a leaf before them, the first ellipse drawn on the start when it is
    /// none.
    pub(in super::super) fn ellipses_tilted(&mut self) -> Case {
        let axis = *self.random.pick(&[2, 2, 1, 0]);
        let start = match self.random.below(20) {
            0..=9 => {
                let mut leaf = self.oval_prism(axis);
                if let Leaf::Prism { height, .. } = &mut leaf {
                    *height = height.signum() * height.abs().max(2.0 * self.scale);
                }
                leaf
            }
            10..=16 => self.block(axis),
            _ => self.plate(axis),
        };
        let count = *self.random.pick(&[1, 2, 2, 3, 3, 4]);
        let mut steps: Vec<Step> = Vec::new();
        for _ in 0..count {
            let leaves: Vec<Leaf> = std::iter::once(&start)
                .chain(steps.iter().map(|step| &step.tool))
                .cloned()
                .collect();
            let elliptic: Vec<&Leaf> = leaves
                .iter()
                .filter(|leaf| !ovals_of(leaf).is_empty() && levels(leaf).is_some())
                .collect();
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
            let blocks: Vec<&Leaf> = leaves
                .iter()
                .filter(|leaf| {
                    matches!(leaf, Leaf::Prism { outline: Outline::Rectangle { .. }, plane, .. }
                        if !matches!(plane, Plane::Tilted { .. }))
                })
                .collect();
            let step = if elliptic.is_empty() && !self.random.chance(0.3) {
                let target = self.random.pick(&leaves).clone();
                Some(self.oval_on(&target))
            } else if let (Some(face), false) = (face, blocks.is_empty())
                && self.random.chance(0.45)
            {
                let block = (*self.random.pick(&blocks)).clone();
                let mode = if self.random.chance(0.75) {
                    Mode::Cut
                } else {
                    Mode::Add
                };
                self.oval_on_a_slanted_face(&block, face, mode)
            } else if !elliptic.is_empty() && self.random.chance(0.7) {
                let target = (*self.random.pick(&elliptic)).clone();
                self.on_an_elliptic_wall(&target)
            } else if !blocks.is_empty() && self.random.chance(0.6) {
                let block = (*self.random.pick(&blocks)).clone();
                self.slanting(&block)
            } else {
                let target = self.random.pick(&leaves).clone();
                let (low, high) = target.bounds().unwrap_or((DVec3::ZERO, DVec3::ONE));
                let middle = ((low + high) / 2.0 / self.scale).round() * self.scale;
                let plane = self.generic(middle);
                self.cut_along(plane, &target)
            };
            if let Some(step) = step {
                steps.push(step);
            }
        }
        let elliptic = std::iter::once(&start)
            .chain(steps.iter().map(|step| &step.tool))
            .any(|leaf| !ovals_of(leaf).is_empty());
        if !elliptic {
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
            let on_the_face =
                face.and_then(|face| self.oval_on_a_slanted_face(&start, face, Mode::Cut));
            let target = start.clone();
            steps.push(on_the_face.unwrap_or_else(|| self.oval_on(&target)));
        }
        Case::new(start, steps)
    }

    /// A block square to world axis `axis`, to be cut at a slant.
    fn block(&mut self, axis: usize) -> Leaf {
        let level = self.coordinate(-2.0, 4.0);
        let low = self.point() - DVec2::splat(2.0 * self.scale);
        let size = DVec2::new(self.length(3.0, 10.0), self.length(3.0, 10.0));
        Leaf::Prism {
            plane: super::square_to_axis(axis, level),
            outline: Outline::Rectangle {
                low,
                high: low + size,
            },
            height: self.length(3.0, 10.0),
        }
    }

    /// A lean off square for a plane against a world axis: on the lattice a
    /// third of the time, on a degree's step or a real angle a quarter, and
    /// a hair from square or from along otherwise, in decision 11's band or
    /// in the band past it.
    fn oval_lean(&mut self) -> Lean {
        match self.random.below(20) {
            0..=6 => Lean::off_square(self.random.on_lattice(15.0, 75.0, 15.0)),
            7..=9 => Lean::off_square(self.random.on_lattice(1.0, 89.0, 1.0)),
            10 | 11 => Lean::off_square(self.random.between(1.0, 89.0)),
            12..=15 => {
                let hair = *self.random.pick(&LAID);
                self.hair_lean(hair)
            }
            _ => {
                let hair = *self.random.pick(&WARNED);
                self.hair_lean(hair)
            }
        }
    }

    /// A lean a hair from square, or from along, either way.
    fn hair_lean(&mut self, hair: f64) -> Lean {
        if self.random.chance(0.5) {
            Lean::off_square(hair)
        } else {
            Lean::at(hair)
        }
    }

    /// A tool on a leaning plane drawn from an elliptic prism standing on a
    /// plane of the origin, by the weights the module's head gives.
    fn on_an_elliptic_wall(&mut self, target: &Leaf) -> Option<Step> {
        let (axis, [bottom, top]) = levels(target)?;
        let oval = *self.random.pick(&ovals_of(target));
        let plane = *target.plane();
        let mut middle = plane.to_world(oval.center);
        middle[axis] = (bottom + top) / 2.0;
        let across = self.square_to(axis);
        let tangent = 3 - axis - across;
        let toward = in_plane(axis, across) * self.either_way(1.0);
        let leaning = match self.random.below(20) {
            0..=4 => {
                let holds = self.square_to(axis);
                let lean = self.oval_lean();
                self.leaning_against(axis, holds, lean, middle)
            }
            5..=8 => {
                let furthest = oval.across().dot(toward).atan2(oval.first.dot(toward));
                let mut rim = plane.to_world(oval.at(furthest));
                rim[axis] = *self.random.pick(&[bottom, top]);
                let lean = self.oval_lean();
                self.leaning_against(axis, tangent, lean, rim)
            }
            9..=14 => {
                let reach = oval.support(toward);
                let distance = if self.random.below(3) < 2 {
                    reach + self.nudge()
                } else {
                    self.up_to(reach - 0.5 * self.scale).unwrap_or(reach / 2.0)
                };
                let at = middle + DVec3::AXES[across] * (toward.x + toward.y) * distance;
                if self.random.chance(0.4) {
                    let hairs = [LAID.as_slice(), &WARNED[..4]].concat();
                    let hair = *self.random.pick(&hairs);
                    self.leaning_against(axis, tangent, Lean::at(hair), at)
                } else {
                    Some(square_through(across, at))
                }
            }
            15 | 16 if self.random.chance(0.25) => return self.cone_about(target, oval),
            15 | 16 => return self.leaning_block_across(target),
            _ => return self.bore_at_a_slant(target),
        };
        self.cut_along(leaning, target)
    }

    /// A turn about the elliptic wall's own axis, a chamfer round the circle
    /// of the ellipse's minor half-axis or a cone standing on it: a cone
    /// against an elliptic wall, which the kernel declines where their faces
    /// may cross, a broken step rather than the flats.
    fn cone_about(&mut self, target: &Leaf, oval: Oval) -> Option<Step> {
        let Leaf::Prism { plane, height, .. } = target else {
            return None;
        };
        let center = plane.to_world(oval.center);
        let normal = plane.normal();
        let ends = [center, center + normal * *height];
        let tool = self.on_a_rim(center, normal, oval.minor(), ends);
        tool.is_solid().then_some(Step::cut(tool))
    }

    /// A block on a plane leaning about any world axis through a place of
    /// the lattice inside the target's box, cutting it or added to it.
    fn leaning_block_across(&mut self, target: &Leaf) -> Option<Step> {
        let (low, high) = target.bounds()?;
        let point = self.inside(low, high);
        let leaning = self.generic(point)?;
        let corner = DVec2::new(self.coordinate(-3.0, 0.0), self.coordinate(-3.0, 0.0));
        let size = DVec2::new(self.length(1.0, 6.0), self.length(1.0, 6.0));
        let tool = Leaf::Prism {
            plane: leaning,
            outline: Outline::Rectangle {
                low: corner,
                high: corner + size,
            },
            height: self.height(),
        };
        Some(self.stepped(tool))
    }

    /// A bore at a slant through the target's box, elliptic or round, on a
    /// plane leaning about any world axis through the box's middle: a wall
    /// at a skew angle to the target's own.
    fn bore_at_a_slant(&mut self, target: &Leaf) -> Option<Step> {
        let (low, high) = target.bounds()?;
        let leaning = self.generic((low + high) / 2.0)?;
        let reach = (high - low).length() / 2.0 + 0.5 * self.scale;
        let through = shifted(leaning, -reach);
        let center = DVec2::new(self.coordinate(-1.0, 1.0), self.coordinate(-1.0, 1.0));
        let outline = if self.random.chance(0.5) {
            let direction = self.oval_direction();
            Outline::Ellipse {
                center,
                first: direction * self.length(0.5, 2.0),
                second: self.length(0.5, 2.0),
                from: 0.0,
            }
        } else {
            Outline::Circle {
                center,
                radius: self.length(0.5, 2.0),
                from: 0.0,
            }
        };
        Some(Step::cut(Leaf::Prism {
            plane: through,
            outline,
            height: 2.0 * reach,
        }))
    }

    /// A cut on a plane leaning against a block's normal through one of its
    /// far edges or anywhere inside it: the slanted face an elliptic hole is
    /// laid on next.
    fn slanting(&mut self, block: &Leaf) -> Option<Step> {
        let (axis, [_, top]) = levels(block)?;
        let (low, high) = block.bounds()?;
        let holds = self.square_to(axis);
        let point = if self.random.chance(0.5) {
            let mut edge = *self.random.pick(&[low, high]);
            edge[axis] = top;
            edge
        } else {
            self.inside(low, high)
        };
        let lean = self.oval_lean();
        let leaning = self.leaning_against(axis, holds, lean, point);
        self.cut_along(leaning, block)
    }

    /// An elliptic hole, or a boss, square to the slanted face a cut left on
    /// `plane`, the cut having taken the side `side` of it: about a place of
    /// the face inside the block, from a margin over the face down through
    /// the floor, blind, or stopping a hair over it; now and then its wall
    /// reaching a side of the block the face's plane runs along, a hair off
    /// now and then.
    fn oval_on_a_slanted_face(
        &mut self,
        block: &Leaf,
        (plane, side): (Plane, f64),
        mode: Mode,
    ) -> Option<Step> {
        let Plane::Tilted { origin, turn } = plane else {
            return None;
        };
        let (_, u, v) = plane.frame();
        let normal = plane.normal();
        let (low, high) = block.bounds()?;
        let direction = self.oval_direction();
        let (first, second) = (self.length(0.5, 2.0), self.length(0.5, 2.0));
        let shape = Oval::new(DVec2::ZERO, direction * first, second);
        let world = |direction: DVec2| u * direction.x + v * direction.y;
        let extent = |axis: usize| {
            let unit = DVec3::AXES[axis];
            shape.support(DVec2::new(unit.dot(u), unit.dot(v)))
        };
        let mut center = None;
        for _ in 0..4 {
            let place = self.inside(low, high);
            let on = place - normal * (place - origin).dot(normal);
            let room = (0..3).all(|axis| {
                on[axis] - low[axis] > extent(axis) && high[axis] - on[axis] > extent(axis)
            });
            if room {
                center = Some(on);
                break;
            }
        }
        let mut center = center?;
        let down = -normal * side;
        if self.random.chance(0.15)
            && let Some(axis) = (0..3).find(|&axis| normal[axis].abs() <= 1e-12)
        {
            let reach = extent(axis);
            let wall = if self.random.chance(0.5) {
                low[axis] + reach
            } else {
                high[axis] - reach
            };
            center[axis] = wall + self.nudge();
        }
        let (exit, floor) = leaving(center, down, low, high)?;
        let depth = if mode == Mode::Add {
            exit + 0.5 * self.scale
        } else {
            match self.random.below(20) {
                0..=11 => exit + 0.5 * self.scale,
                12..=16 => self.up_to(exit - 0.5 * self.scale).unwrap_or(exit / 2.0),
                _ => {
                    let slant = down.dot(DVec3::AXES[floor]).abs();
                    let (first, second) = (world(shape.first), world(shape.across()));
                    let spread = first[floor].hypot(second[floor]) / slant;
                    (exit - spread - self.hair().abs() / slant).max(exit / 2.0)
                }
            }
        };
        let outline = self.dressed(shape);
        let margin = self.length(0.5, 1.0);
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
}
