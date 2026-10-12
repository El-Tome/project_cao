//! The draw of ellipses crossed (#571): prisms on two planes of the origin
//! square to each other — a round or an elliptic wall through an elliptic or
//! a round one from the side — weighted towards where the curve two walls
//! square to each other meet along changes kind. Across the direction both
//! walls run square to, a wall reaches its half-width either side of its
//! axis: two half-widths equal with the axes meeting cross at nodes, one
//! ending where the other does touches it there, inside or out.

use glam::DVec2;

use super::super::super::{Case, Leaf, Outline, Oval, Step};
use super::super::Drawing;
use super::{in_plane, levels, ovals_of, plane_axes, spanning, square_to_axis};

/// How a tool drawn from the side stands against the wall it crosses,
/// across the direction both run square to.
#[derive(Clone, Copy)]
enum Across {
    /// The same half-width, the axes meeting.
    Node,
    /// The same half-width, the axes apart.
    Beside,
    /// Narrower, ending where the wall does.
    Inside,
    /// Ending where the wall does, from beyond it.
    Outside,
    /// Any width, anywhere across it.
    Anywhere,
}

impl Drawing {
    /// A case of ellipses crossed: an upright prism, round or elliptic, on a
    /// plate now and then, and tools from the side through it or through a
    /// tool before them, at least one of every pair crossing elliptic.
    pub(in super::super) fn ellipses_crossed(&mut self) -> Case {
        let axis = *self.random.pick(&[2, 2, 1, 0]);
        let elliptic = self.random.chance(0.65);
        let upright = self.standing(axis, elliptic);
        let (start, mut steps) = if self.random.chance(0.25) {
            (self.plate_under(&upright), vec![Step::add(upright.clone())])
        } else {
            (upright.clone(), Vec::new())
        };
        let count = *self.random.pick(&[1, 1, 2, 2, 3]);
        let mut crossed = vec![upright];
        for _ in 0..count {
            let target = if crossed.len() > 1 && self.random.chance(0.25) {
                self.random.pick(&crossed[1..]).clone()
            } else {
                crossed[0].clone()
            };
            let Some(tool) = self.tool_across(&target) else {
                continue;
            };
            crossed.push(tool.clone());
            steps.push(if self.random.chance(0.65) {
                Step::cut(tool)
            } else {
                Step::add(tool)
            });
        }
        if steps.is_empty() {
            let target = crossed[0].clone();
            let tool = self
                .tool_across(&target)
                .unwrap_or_else(|| self.oval_prism(axis));
            steps.push(Step::cut(tool));
        }
        Case::new(start, steps)
    }

    /// A prism square to world axis `axis` tall enough to cross: an ellipse,
    /// whole most of the time, or a circle or a ring.
    fn standing(&mut self, axis: usize, elliptic: bool) -> Leaf {
        let level = self.coordinate(-2.0, 2.0);
        let center = self.point();
        let outline = if elliptic {
            let direction = self.oval_direction();
            let first = self.length(1.5, 6.0);
            let second = if self.random.chance(0.15) {
                first
            } else {
                self.length(1.5, 6.0)
            };
            let oval = Oval::new(center, direction * first, second);
            self.dressed(oval)
        } else {
            let radius = self.length(1.5, 6.0);
            if self.random.chance(0.8) {
                Outline::Circle {
                    center,
                    radius,
                    from: 0.0,
                }
            } else {
                Outline::Ring {
                    center,
                    outer: radius,
                    inner: self
                        .up_to(radius - 0.5 * self.scale)
                        .unwrap_or(radius / 2.0),
                }
            }
        };
        let height = self.length(3.0, 10.0);
        Leaf::Prism {
            plane: square_to_axis(axis, level),
            outline,
            height: if self.random.chance(0.2) {
                -height
            } else {
                height
            },
        }
    }

    /// A plate the upright prism stands on, its far end flush with the
    /// prism's near one.
    fn plate_under(&mut self, upright: &Leaf) -> Leaf {
        let Some((axis, [bottom, top])) = levels(upright) else {
            return upright.clone();
        };
        let (low, high) = upright.bounds().expect("a prism has a box");
        let [first, second] = plane_axes(axis);
        let margin = self.length(1.0, 3.0);
        let low = DVec2::new(low[first], low[second]) - DVec2::splat(margin);
        let high = DVec2::new(high[first], high[second]) + DVec2::splat(margin);
        let down = (bottom - top).signum();
        let (plane, height) = spanning(axis, bottom, bottom + down * self.length(1.0, 3.0));
        Leaf::Prism {
            plane,
            outline: Outline::Rectangle { low, high },
            height,
        }
    }

    /// A tool from the side through a round or elliptic prism: square to a
    /// world axis square to the prism's own, round when the prism is
    /// elliptic half the time and elliptic otherwise, standing against the
    /// prism by the weights of `Across`, a hair off a third of the time,
    /// through the prism or stopping at its axis. Nothing for a leaf with no
    /// wall to cross.
    fn tool_across(&mut self, target: &Leaf) -> Option<Leaf> {
        let (axis, [bottom, top]) = levels(target)?;
        let (wall, elliptic) = self.wall_to_cross(target);
        let sides: Vec<usize> = (0..3).filter(|other| *other != axis).collect();
        let side = *self.random.pick(&sides);
        let shared = 3 - axis - side;
        let (width, offset) = self.standing_against(wall.support(in_plane(axis, shared)))?;
        let round = elliptic && self.random.chance(0.5);
        let shape = self.shape_across(round, in_plane(side, shared), width);

        let unit = 0.5 * self.scale;
        let tall = shape.support(in_plane(side, axis));
        let (low, high) = (bottom.min(top), bottom.max(top));
        let level = if self.random.chance(0.7) && high - low > 2.0 * tall + unit {
            self.coordinate((low + tall) / self.scale, (high - tall) / self.scale)
                .clamp(low + tall, high - tall)
        } else {
            self.coordinate(low / self.scale, high / self.scale)
        };
        let middle = target.plane().to_world(wall.center);
        let mut place = middle;
        place[shared] += offset;
        place[axis] = level;
        let [first, second] = plane_axes(side);
        let center = DVec2::new(place[first], place[second]);

        let (box_low, box_high) = target.bounds()?;
        let margin = unit + self.length(0.0, 1.0);
        let (from, to) = match self.random.below(4) {
            0 => (box_low[side] - margin, middle[side]),
            1 => (box_high[side] + margin, middle[side]),
            _ => (box_low[side] - margin, box_high[side] + margin),
        };
        let (plane, height) = spanning(side, from, to);
        let outline = if round {
            Outline::Circle {
                center,
                radius: width,
                from: 0.0,
            }
        } else {
            self.whole(Oval::new(center, shape.first, shape.second))
        };
        let tool = Leaf::Prism {
            plane,
            outline,
            height,
        };
        tool.is_solid().then_some(tool)
    }

    /// The wall a tool from the side crosses, as an ellipse — a circle's or
    /// a ring's outer one as one of equal half-axes — and whether it is
    /// elliptic.
    fn wall_to_cross(&mut self, target: &Leaf) -> (Oval, bool) {
        match target {
            Leaf::Prism {
                outline:
                    Outline::Circle { center, radius, .. }
                    | Outline::Ring {
                        center,
                        outer: radius,
                        ..
                    },
                ..
            } => (Oval::new(*center, DVec2::X * *radius, *radius), false),
            _ => {
                let ovals = ovals_of(target);
                (*self.random.pick(&ovals), true)
            }
        }
    }

    /// How a tool from the side stands against a wall reaching `reach`
    /// either side of its axis across the direction both run square to:
    /// the tool's own half-width across it and how far its axis stands from
    /// the wall's, by the weights of `Across`, either a hair off a third of
    /// the time. Nothing when the tool would be no wider than nothing.
    fn standing_against(&mut self, reach: f64) -> Option<(f64, f64)> {
        let unit = 0.5 * self.scale;
        let across = match self.random.below(20) {
            0..=7 => Across::Node,
            8..=10 => Across::Beside,
            11..=13 => Across::Inside,
            14 | 15 => Across::Outside,
            _ => Across::Anywhere,
        };
        let (width, offset) = match across {
            Across::Node => (reach, 0.0),
            Across::Beside => {
                let apart = self.up_to(reach).unwrap_or(unit);
                (reach, self.either_way(apart))
            }
            Across::Inside => {
                let width = self.up_to(reach - unit / 2.0).unwrap_or(reach / 2.0);
                (width, self.either_way(reach - width))
            }
            Across::Outside => {
                let width = self.length(0.5, 4.0);
                (width, self.either_way(reach + width))
            }
            Across::Anywhere => {
                let width = self.length(0.5, reach / self.scale + 1.0);
                let apart = self.up_to(reach).unwrap_or(0.0);
                (width, self.either_way(apart))
            }
        };
        let (width, offset) = if !self.random.chance(0.35) {
            (width, offset)
        } else if self.random.chance(0.5) {
            (width + self.nudge(), offset)
        } else {
            (width, offset + self.nudge())
        };
        (width > 0.0).then_some((width, offset))
    }

    /// The section of a tool from the side, about the origin of its plane:
    /// a circle of `width`, or an ellipse whose axes lie along its plane's,
    /// across or along the wall it crosses, or turned, its size taken so
    /// that it reaches `width` along `across`.
    fn shape_across(&mut self, round: bool, across: DVec2, width: f64) -> Oval {
        if round {
            return Oval::new(DVec2::ZERO, across * width, width);
        }
        let direction = match self.random.below(10) {
            0..=3 => across,
            4..=6 => across.perp().abs(),
            _ => self.oval_direction(),
        };
        let (first, second) = self.half_axes();
        let shape = Oval::new(DVec2::ZERO, direction * first, second);
        let scaled = width / shape.support(across);
        Oval::new(DVec2::ZERO, shape.first * scaled, shape.second * scaled)
    }
}
