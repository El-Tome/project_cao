//! The draw of ellipses raised along one normal of the planes of the origin
//! (#571), and the tools drawn from a leaf before them: an ellipse drawn on a
//! block, a circle or another profile, and tools drawn beside an ellipse,
//! weighted towards what a kernel decides a hair either way — the same
//! ellipse moved, grown or turned by a hair, a block's side at the ellipse's
//! reach, a circle across its minor axis or at one of its vertices, inside
//! or out, two ellipses touching, half-axes a hair apart, a slot or a
//! rounded rectangle hugging it, an outline on the very ellipse.

use glam::DVec2;

use super::super::super::{Case, Leaf, Outline, Oval, Step};
use super::super::Drawing;
use super::{NEAR_A_CIRCLE, levels, ovals_of, spanning};

impl Drawing {
    /// A case of the draw of ellipses on one normal: an ellipse, a plate or
    /// another profile to start, and steps drawn from a leaf before them
    /// two times in three, fresh otherwise; the first ellipse, when the
    /// start is none, drawn on the start.
    pub(in super::super) fn ellipses_square(&mut self) -> Case {
        let axis = *self.random.pick(&[2, 2, 1, 0]);
        let start = match self.random.below(20) {
            0..=8 => self.oval_prism(axis),
            9..=16 => self.plate(axis),
            _ => self.profile_prism(axis),
        };
        let count = *self.random.pick(&[1, 1, 2, 2, 2, 3, 3, 4, 5]);
        let mut steps: Vec<Step> = Vec::new();
        for _ in 0..count {
            let leaves: Vec<Leaf> = std::iter::once(&start)
                .chain(steps.iter().map(|step| &step.tool))
                .cloned()
                .collect();
            let elliptic: Vec<&Leaf> = leaves
                .iter()
                .filter(|leaf| !ovals_of(leaf).is_empty())
                .collect();
            let step = if elliptic.is_empty() {
                let target = self.random.pick(&leaves).clone();
                self.oval_on(&target)
            } else if self.random.chance(0.65) {
                let target = if self.random.chance(0.7) {
                    (*self.random.pick(&elliptic)).clone()
                } else {
                    self.random.pick(&leaves).clone()
                };
                if ovals_of(&target).is_empty() {
                    self.oval_on(&target)
                } else {
                    self.beside_an_oval(&target)
                }
            } else {
                let tool = if self.random.chance(0.5) {
                    self.oval_prism(axis)
                } else {
                    self.profile_prism(axis)
                };
                self.stepped(tool)
            };
            steps.push(step);
        }
        Case::new(start, steps)
    }

    /// The height a tool drawn from a leaf of `earlier` height takes: the
    /// same a hair off, twice it, or fresh.
    fn related_height(&mut self, earlier: f64) -> f64 {
        match self.random.below(3) {
            0 => earlier + self.nudge(),
            1 => earlier * 2.0,
            _ => self.height(),
        }
    }

    /// An outline an ellipse bounds drawn on a leaf with no ellipse: on a
    /// block a boss on its far end, a pocket or a hole through it, an
    /// ellipse filling it or touching or crossing one of its sides; on a
    /// circle or a ring an ellipse about its centre across whose minor axis
    /// it touches, within which it touches at the ends of the major one, a
    /// hair from it, or touching it from outside at a vertex; on a rounded
    /// rectangle or a slot one about the centre of one of its arcs.
    pub(super) fn oval_on(&mut self, target: &Leaf) -> Step {
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = target
        else {
            let axis = *self.random.pick(&[0, 1, 2]);
            let tool = self.oval_prism(axis);
            return self.stepped(tool);
        };
        let round = match outline {
            Outline::Rectangle { low, high } => {
                return self.oval_on_a_block(target, *low, *high);
            }
            Outline::Circle { center, radius, .. } => (*center, *radius),
            Outline::Ring {
                center,
                outer,
                inner,
            } => (*center, *self.random.pick(&[*outer, *inner])),
            Outline::Rounded { low, radius, .. } => (*low + *radius, *radius),
            Outline::Slot { from, radius, .. } => (*from, *radius),
            _ => (self.point(), self.length(0.5, 3.0)),
        };
        let (center, radius) = round;
        let direction = self.oval_direction();
        let other = self.length(0.5, 6.0);
        let oval = match self.random.below(4) {
            0 => Oval::new(
                center,
                direction * (radius + self.nudge()),
                radius + other.max(0.5 * self.scale),
            ),
            1 => Oval::new(
                center,
                direction * (radius + self.nudge()),
                self.up_to(radius - 0.25 * self.scale)
                    .unwrap_or(radius / 2.0),
            ),
            2 => {
                let hair = *self.random.pick(&NEAR_A_CIRCLE);
                Oval::new(
                    center,
                    direction * radius,
                    radius * (1.0 + self.either_way(hair)),
                )
            }
            _ => {
                let reach = self.length(0.5, 4.0);
                Oval::new(
                    center + direction * (radius + reach + self.nudge()),
                    direction * reach,
                    other,
                )
            }
        };
        let outline = self.dressed(oval);
        let plane = self.near(*plane);
        let height = self.related_height(*height);
        let tool = Leaf::Prism {
            plane,
            outline,
            height,
        };
        self.stepped(tool)
    }

    /// An elliptic tool drawn on a block from `low` to `high`: a boss on its
    /// far end, a pocket into it or a hole through it, the ellipse inside
    /// it, filling it to its four sides, touching one of them from inside
    /// or outside a hair off now and then, or crossing one.
    fn oval_on_a_block(&mut self, block: &Leaf, low: DVec2, high: DVec2) -> Step {
        let Some((axis, [bottom, top])) = levels(block) else {
            let tool = self.oval_prism(2);
            return self.stepped(tool);
        };
        let size = high - low;
        let middle = (low + high) / 2.0;
        let unit = 0.5 * self.scale;
        let mut oval = match self.random.below(10) {
            0..=4 => {
                let center = DVec2::new(
                    self.coordinate(low.x / self.scale + 1.0, high.x / self.scale - 1.0),
                    self.coordinate(low.y / self.scale + 1.0, high.y / self.scale - 1.0),
                );
                let room = (center - low).min(high - center).max(DVec2::splat(unit));
                let direction = self.oval_direction();
                let (first, second) = self.half_axes();
                let oval = Oval::new(center, direction * first, second);
                let fits = (room.x / oval.support(DVec2::X)).min(room.y / oval.support(DVec2::Y));
                if fits < 1.0 {
                    Oval::new(center, oval.first * fits, oval.second * fits)
                } else {
                    oval
                }
            }
            5 | 6 => {
                let along = *self.random.pick(&[DVec2::X, DVec2::Y]);
                Oval::new(
                    middle,
                    along * size.dot(along) / 2.0,
                    size.dot(along.perp()).abs() / 2.0,
                )
            }
            _ => {
                let across = self.random.below(2);
                let outward = self.either_way(1.0);
                let wall = if outward > 0.0 {
                    high[across]
                } else {
                    low[across]
                };
                let shape = self.fresh_ellipse();
                let reach = shape.support(DVec2::AXES[across]);
                let inward = match self.random.below(3) {
                    0 => reach,
                    1 => -reach,
                    _ => reach - unit,
                };
                let mut center = DVec2::ZERO;
                center[across] = wall - outward * (inward + self.nudge());
                center[1 - across] =
                    self.coordinate(low[1 - across] / self.scale, high[1 - across] / self.scale);
                Oval::new(center, shape.first, shape.second)
            }
        };
        if oval.first.length() <= 0.0 || oval.second <= 0.0 {
            oval = Oval::new(middle, DVec2::X * unit, unit);
        }
        let outline = self.dressed(oval);
        let outward = (top - bottom).signum();
        let margin = unit + self.nudge().abs();
        match self.random.below(10) {
            0..=3 => {
                let (plane, height) = spanning(axis, top, top + outward * self.length(0.5, 4.0));
                Step::add(Leaf::Prism {
                    plane,
                    outline,
                    height,
                })
            }
            4..=6 => {
                let depth = self
                    .up_to((top - bottom).abs() - unit)
                    .unwrap_or((top - bottom).abs() / 2.0);
                let (plane, height) = spanning(axis, top + outward * margin, top - outward * depth);
                Step::cut(Leaf::Prism {
                    plane,
                    outline,
                    height,
                })
            }
            _ => {
                let (plane, height) =
                    spanning(axis, bottom - outward * margin, top + outward * margin);
                Step::cut(Leaf::Prism {
                    plane,
                    outline,
                    height,
                })
            }
        }
    }

    /// A tool drawn from a leaf on an ellipse, by the weights the module's
    /// head gives.
    fn beside_an_oval(&mut self, target: &Leaf) -> Step {
        let Leaf::Prism { plane, height, .. } = target else {
            let tool = self.oval_prism(2);
            return self.stepped(tool);
        };
        let ovals = ovals_of(target);
        let oval = *self.random.pick(&ovals);
        let plane = self.near(*plane);
        let height = self.related_height(*height);
        let outline = match self.random.below(20) {
            0..=2 => self.the_same_a_hair_off(oval),
            3..=5 => self.block_beside(oval),
            6 | 7 => {
                let radius = if self.random.chance(0.75) {
                    oval.minor()
                } else {
                    oval.major()
                };
                Outline::Circle {
                    center: oval.center,
                    radius: radius + self.nudge(),
                    from: 0.0,
                }
            }
            8..=10 => self.circle_at_a_vertex(oval),
            11 | 12 => self.touching(oval),
            13 => {
                let hair = *self.random.pick(&NEAR_A_CIRCLE);
                let radius = oval.minor();
                Outline::Ellipse {
                    center: oval.center,
                    first: oval.major_direction() * radius,
                    second: radius * (1.0 + self.either_way(hair)),
                    from: 0.0,
                }
            }
            14 | 15 => self.hugging(oval),
            16 | 17 => self.on_the_same_ellipse(oval),
            _ => {
                let shape = self.fresh_ellipse();
                let shift = DVec2::new(self.coordinate(-1.0, 1.0), self.coordinate(-1.0, 1.0))
                    * oval.major()
                    / self.scale;
                self.oval_outline(Oval::new(oval.center + shift, shape.first, shape.second))
            }
        };
        let tool = Leaf::Prism {
            plane,
            outline,
            height,
        };
        self.stepped(tool)
    }

    /// The same ellipse, a fifth of the time exactly, otherwise moved,
    /// grown or turned by a hair.
    fn the_same_a_hair_off(&mut self, oval: Oval) -> Outline {
        let hair = self.oval_hair();
        let moved = match self.random.below(5) {
            0 => oval,
            1 => Oval::new(
                oval.center + self.oval_direction() * hair,
                oval.first,
                oval.second,
            ),
            2 => Oval::new(
                oval.center,
                oval.first * (1.0 + hair / oval.first.length()),
                oval.second + hair,
            ),
            3 => {
                let along = self.random.chance(0.5);
                Oval::new(
                    oval.center,
                    if along {
                        oval.first.normalize() * (oval.first.length() + hair)
                    } else {
                        oval.first
                    },
                    if along {
                        oval.second
                    } else {
                        oval.second + hair
                    },
                )
            }
            _ => Oval::new(
                oval.center,
                DVec2::from_angle(hair / self.scale).rotate(oval.first),
                oval.second,
            ),
        };
        self.whole(moved)
    }

    /// A block with a side a hair from the ellipse's reach along one of the
    /// plane's axes, from outside it or across it, or crossing it on the
    /// lattice.
    fn block_beside(&mut self, oval: Oval) -> Outline {
        let side = *self
            .random
            .pick(&[DVec2::X, DVec2::Y, DVec2::NEG_X, DVec2::NEG_Y]);
        let reach = oval.support(side);
        let wall = match self.random.below(4) {
            0 | 1 => reach + self.nudge(),
            2 => reach + self.oval_hair(),
            _ => self
                .up_to(2.0 * reach - 0.5 * self.scale)
                .map_or(0.0, |inside| inside - reach),
        };
        let depth = self.length(0.5, 4.0);
        let across = side.perp().abs();
        let width = oval.support(across) + self.length(0.5, 3.0);
        let beyond = if self.random.chance(0.5) {
            wall + depth
        } else {
            wall - 2.0 * reach - depth
        };
        let corners = [
            oval.center + side * wall + across * width,
            oval.center + side * beyond - across * width,
        ];
        Outline::Rectangle {
            low: corners[0].min(corners[1]),
            high: corners[0].max(corners[1]),
        }
    }

    /// A circle touching the ellipse at the end of one of its axes: outside
    /// it, or inside it no more curved than the ellipse is there — the
    /// osculating circle itself now and then — a hair off now and then.
    fn circle_at_a_vertex(&mut self, oval: Oval) -> Outline {
        let major = self.random.chance(0.5);
        let direction = if major {
            oval.major_direction()
        } else {
            oval.major_direction().perp()
        };
        let direction = self.either_way(1.0) * direction;
        let (reach, other) = if major {
            (oval.major(), oval.minor())
        } else {
            (oval.minor(), oval.major())
        };
        let vertex = oval.center + direction * reach;
        let (radius, inward) = if self.random.chance(0.5) {
            (self.length(0.5, 3.0), -1.0)
        } else {
            let curved = other * other / reach;
            let radius = match self.random.below(3) {
                0 => curved.min(reach),
                1 => curved.min(reach) / 2.0,
                _ => self
                    .up_to(curved.min(reach))
                    .unwrap_or(curved.min(reach) / 2.0),
            };
            (radius, 1.0)
        };
        Outline::Circle {
            center: vertex - direction * (inward * radius + self.nudge()),
            radius,
            from: 0.0,
        }
    }

    /// Another ellipse touching this one at the end of one of its axes:
    /// outside it, its own axes along this one's or turned a quarter, or
    /// inside it, this one shrunk about that end.
    fn touching(&mut self, oval: Oval) -> Outline {
        let major = self.random.chance(0.5);
        let direction = if major {
            oval.major_direction()
        } else {
            oval.major_direction().perp()
        };
        let direction = self.either_way(1.0) * direction;
        let reach = if major { oval.major() } else { oval.minor() };
        let vertex = oval.center + direction * reach;
        let other = if self.random.chance(0.6) {
            let (first, second) = self.half_axes();
            let first = if self.random.chance(0.5) {
                direction * first
            } else {
                direction.perp() * first
            };
            let shape = Oval::new(DVec2::ZERO, first, second);
            Oval::new(
                vertex + direction * (shape.support(direction) + self.nudge()),
                first,
                second,
            )
        } else {
            let shrunk = *self.random.pick(&[0.5, 0.75]);
            Oval::new(
                vertex + (oval.center - vertex) * shrunk - direction * self.nudge(),
                oval.first * shrunk,
                oval.second * shrunk,
            )
        };
        self.dressed(other)
    }

    /// A slot or a rounded rectangle hugging the ellipse: a slot along its
    /// major axis as wide as its minor one, touching it at the ends of both,
    /// or a rounded rectangle whose sides touch it at its reach along the
    /// plane's axes; a hair off now and then.
    fn hugging(&mut self, oval: Oval) -> Outline {
        let lattice_aligned = {
            let direction = oval.first.normalize();
            direction.x == 0.0 || direction.y == 0.0
        };
        if lattice_aligned && self.random.chance(0.5) {
            let direction = oval.major_direction();
            let reach = oval.major() - oval.minor();
            if reach > 0.0 {
                return Outline::Slot {
                    from: oval.center - direction * reach,
                    to: oval.center + direction * reach,
                    radius: oval.minor() + self.nudge(),
                };
            }
        }
        let reach = DVec2::new(oval.support(DVec2::X), oval.support(DVec2::Y));
        let grown = reach + DVec2::splat(self.nudge());
        let half = grown.min_element();
        Outline::Rounded {
            low: oval.center - grown,
            high: oval.center + grown,
            radius: self.up_to(half).unwrap_or(half / 2.0).min(half),
        }
    }

    /// An outline on the very ellipse: cut by traits, a slot's end, or a
    /// ring around it or bored to it.
    fn on_the_same_ellipse(&mut self, oval: Oval) -> Outline {
        match self.random.below(4) {
            0 => self.chorded(oval),
            1 => {
                let way = oval.first.normalize();
                Outline::OvalSlot {
                    from: oval.center,
                    to: oval.center - way * self.length(0.5, 6.0),
                    across: oval.second,
                    along: oval.first.length(),
                }
            }
            2 => {
                let grown = self.length(0.5, 3.0);
                Outline::OvalRing {
                    center: oval.center,
                    first: oval.first.normalize() * (oval.first.length() + grown),
                    second: oval.second + grown,
                    hole: DVec2::new(oval.first.length(), oval.second),
                }
            }
            _ => self.oval_ring(oval),
        }
    }
}
