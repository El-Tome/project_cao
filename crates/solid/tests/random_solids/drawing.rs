//! Cases drawn from a seed, weighted towards the shapes that break kernels.
//!
//! A kernel does not fail on two solids in general position. It fails where
//! two faces lie in one plane, where a circle touches another, where an edge
//! lands on a vertex — and worse, where they very nearly do. Corners are drawn
//! on a lattice of round numbers so that coincidences come up on their own;
//! half the tools are drawn from a solid before them so that they come up
//! often; and a coincidence is now and then missed by a hair, in the band where
//! a tolerance has to decide.

use cao_solid::soundness::{NEAR, Random};
use glam::{DVec2, DVec3};

use super::{Case, Leaf, Mode, Outline, Plane, Step};

impl Case {
    /// The case a seed stands for. The same seed always draws the same case,
    /// which is what lets a failure be named by its seed alone.
    pub fn drawn(seed: u64) -> Case {
        Case::drawn_among(seed, false)
    }

    /// The case a seed stands for among the solids the exact kernel of #498
    /// raises: prisms of rectangles and circles on the three planes of the
    /// origin, none tilted, still weighted towards the coincidences and the
    /// hairs. Not the case `drawn` gives the same seed.
    pub fn drawn_square(seed: u64) -> Case {
        Case::drawn_among(seed, true)
    }

    /// The case a seed stands for. `square` keeps out every solid but prisms
    /// of rectangles and circles on the planes of the origin, by drawing
    /// among fewer kinds rather than drawing again: the cases `drawn` gives
    /// are the ones a seed has always named.
    fn drawn_among(seed: u64, square: bool) -> Case {
        let mut random = Random::seeded(seed);
        let scale = *random.pick(&[1.0, 1.0, 1.0, 5.0, 30.0]);
        let mut drawing = Drawing {
            random,
            scale,
            square,
        };

        let start = drawing.leaf(None);
        let count = *drawing.random.pick(&[1, 1, 1, 2, 2, 2, 3, 3, 4, 5]);
        let mut steps: Vec<Step> = Vec::new();
        for _ in 0..count {
            let before = if drawing.random.chance(0.5) {
                let leaves: Vec<&Leaf> = std::iter::once(&start)
                    .chain(steps.iter().map(|step| &step.tool))
                    .collect();
                Some((*drawing.random.pick(&leaves)).clone())
            } else {
                None
            };
            let tool = drawing.leaf(before.as_ref());
            let mode = if drawing.random.chance(0.6) {
                Mode::Cut
            } else {
                Mode::Add
            };
            steps.push(Step { mode, tool });
        }
        Case::new(start, steps)
    }
}

struct Drawing {
    random: Random,
    /// How big the case is drawn: most on a lattice of units, some thirty
    /// times larger, where a tolerance taken in absolute units stops holding.
    scale: f64,
    /// Prisms of rectangles and circles on the planes of the origin, and
    /// nothing else.
    square: bool,
}

impl Drawing {
    /// A coordinate: mostly on the lattice, sometimes on its half.
    fn coordinate(&mut self, low: f64, high: f64) -> f64 {
        let step = if self.random.chance(0.8) { 1.0 } else { 0.5 };
        self.random.on_lattice(low, high, step) * self.scale
    }

    fn length(&mut self, low: f64, high: f64) -> f64 {
        self.random.on_lattice(low, high, 0.5) * self.scale
    }

    /// Nothing most of the time; otherwise a miss by a hair: from half the
    /// billionth of a corner's distance the kernel written here judges two
    /// faces one plane by, where it does so at some corners and not at
    /// others, up to the noise of a face plane and the solver's tolerance.
    ///
    /// Never finer than five times the rules' own `NEAR`: under that, a skin
    /// the kernel was right to leave cannot be told from two faces lying on
    /// each other, and the failure would be the harness's.
    fn nudge(&mut self) -> f64 {
        if self.random.chance(0.6) {
            return 0.0;
        }
        let reach = 20.0 * self.scale;
        let hair = *self.random.pick(&[
            5e-10 * reach,
            1e-9 * reach,
            3e-9 * reach,
            1e-8 * reach,
            1e-7,
            1e-5,
        ]);
        let hair = hair.max(5.0 * NEAR * reach);
        if self.random.chance(0.5) { hair } else { -hair }
    }

    fn plane(&mut self) -> Plane {
        let offset = self.coordinate(-2.0, 8.0);
        let kinds = if self.square { 17 } else { 20 };
        match self.random.below(kinds) {
            0..=10 => Plane::Xy(offset),
            11..=13 => Plane::Xz(offset),
            14..=16 => Plane::Yz(offset),
            _ => {
                let turn = DVec3::new(
                    self.random.on_lattice(-90.0, 90.0, 15.0),
                    self.random.on_lattice(-90.0, 90.0, 15.0),
                    self.random.on_lattice(-90.0, 90.0, 15.0),
                );
                let origin = DVec3::new(
                    self.coordinate(0.0, 6.0),
                    self.coordinate(0.0, 6.0),
                    self.coordinate(0.0, 6.0),
                );
                Plane::Tilted { origin, turn }
            }
        }
    }

    fn point(&mut self) -> DVec2 {
        DVec2::new(self.coordinate(0.0, 10.0), self.coordinate(0.0, 10.0))
    }

    fn height(&mut self) -> f64 {
        let height = self.length(1.0, 10.0);
        if self.random.chance(0.2) {
            -height
        } else {
            height
        }
    }

    fn outline(&mut self) -> Outline {
        let kinds = if self.square { 15 } else { 20 };
        match self.random.below(kinds) {
            0..=8 => {
                let low = self.point();
                let size = DVec2::new(self.length(1.0, 8.0), self.length(1.0, 8.0));
                Outline::Rectangle {
                    low,
                    high: low + size,
                }
            }
            9..=14 => Outline::Circle {
                center: self.point(),
                radius: self.length(0.5, 6.0),
                from: if self.random.chance(0.2) {
                    self.random.between(0.0, 360.0)
                } else {
                    0.0
                },
            },
            15 | 16 => {
                let center = self.point();
                let count = 3 + self.random.below(6);
                let corners = (0..count)
                    .map(|index| {
                        let angle = std::f64::consts::TAU
                            * (index as f64 + self.random.between(0.3, 0.7))
                            / count as f64;
                        center + DVec2::from_angle(angle) * self.length(1.0, 5.0)
                    })
                    .collect();
                Outline::Star { center, corners }
            }
            _ => {
                let outer = self.random.on_lattice(1.5, 6.0, 0.5);
                let inner = self.random.on_lattice(0.5, outer - 0.5, 0.5);
                Outline::Ring {
                    center: self.point(),
                    outer: outer * self.scale,
                    inner: inner * self.scale,
                }
            }
        }
    }

    /// A rectangle turned about the plane's second axis, either way round and
    /// on either side of it: mostly off the axis, sometimes on it — a solid
    /// cylinder — and now and then a hair either side of it, which is a
    /// profile meant to touch the axis and drawn with a solver's noise.
    fn revolution(&mut self, plane: Plane) -> Leaf {
        let width = self.length(1.0, 5.0);
        let near = match self.random.below(8) {
            0 | 1 => 0.0,
            2 => *self.random.pick(&[1e-9, 1e-6, -1e-4]) * width,
            _ => self.coordinate(1.0, 6.0),
        };
        let low = DVec2::new(near, self.coordinate(-3.0, 5.0));
        let size = DVec2::new(width, self.length(1.0, 6.0));
        let (low, high) = if self.random.chance(0.25) {
            (
                DVec2::new(-low.x - size.x, low.y),
                DVec2::new(-low.x, low.y + size.y),
            )
        } else {
            (low, low + size)
        };
        let degrees = if self.random.chance(0.5) {
            360.0
        } else {
            self.random.on_lattice(15.0, 345.0, 15.0)
        };
        let degrees = if self.random.chance(0.3) {
            -degrees
        } else {
            degrees
        };
        Leaf::Revolution {
            plane,
            low,
            high,
            degrees,
        }
    }

    /// A leaf, drawn fresh or from one drawn before it.
    fn leaf(&mut self, before: Option<&Leaf>) -> Leaf {
        if let Some(before) = before {
            return self.related(before);
        }
        let plane = self.plane();
        if !self.square && self.random.chance(0.15) {
            self.revolution(plane)
        } else {
            Leaf::Prism {
                plane,
                outline: self.outline(),
                height: self.height(),
            }
        }
    }

    /// A tool drawn from a solid before it: on its plane or a hair off it, and
    /// often sharing a height, a centre, a side, or a circle it touches.
    fn related(&mut self, before: &Leaf) -> Leaf {
        let plane = self.near(*before.plane());
        let Leaf::Prism {
            outline: earlier,
            height: earlier_height,
            ..
        } = before
        else {
            return self.revolution(plane);
        };

        let height = match self.random.below(3) {
            0 => earlier_height + self.nudge(),
            1 => earlier_height * 2.0,
            _ => self.height(),
        };
        let outline = match (earlier, self.random.below(4)) {
            (Outline::Circle { center, radius, .. }, 0) => Outline::Circle {
                center: *center + DVec2::X * self.nudge(),
                radius: self.random.on_lattice(0.5, radius / self.scale + 2.0, 0.5) * self.scale,
                from: 0.0,
            },
            (Outline::Circle { center, radius, .. }, 1) => {
                let apart =
                    self.random.on_lattice(0.5, 2.0 * radius / self.scale, 0.5) * self.scale;
                let direction = *self.random.pick(&[DVec2::X, DVec2::Y, DVec2::NEG_X]);
                let touching = (radius - apart).abs().max(0.5 * self.scale) + self.nudge();
                Outline::Circle {
                    center: *center + direction * apart,
                    radius: touching,
                    from: 0.0,
                }
            }
            (Outline::Rectangle { low, high }, 0) => {
                let room = ((high.y - low.y) / self.scale - 0.5) / 2.0;
                let inset = self.random.on_lattice(0.0, room.clamp(0.0, 1.0), 0.5) * self.scale;
                Outline::Rectangle {
                    low: *low + DVec2::splat(inset) + DVec2::X * self.nudge(),
                    high: DVec2::new(high.x + 2.0 * self.scale, high.y - inset),
                }
            }
            (Outline::Rectangle { low, high }, 1) => Outline::Circle {
                center: DVec2::new(high.x + self.nudge(), (low.y + high.y) / 2.0),
                radius: self.length(0.5, 3.0),
                from: 0.0,
            },
            _ => self.outline(),
        };
        Leaf::Prism {
            plane,
            outline,
            height,
        }
    }

    /// The plane a related tool is drawn on: the same one, the same moved
    /// back so that the tool starts below the solid and goes through it, or
    /// either of those missed by a hair.
    fn near(&mut self, plane: Plane) -> Plane {
        let plane = if self.random.chance(0.3) {
            shifted(plane, -self.scale)
        } else {
            plane
        };
        let plane = shifted(plane, self.nudge());
        match plane {
            Plane::Xy(offset) if !self.square && self.random.chance(0.1) => Plane::Tilted {
                origin: DVec3::Z * offset,
                turn: DVec3::X * *self.random.pick(&[1e-7, 1e-5, 1e-3]),
            },
            _ => plane,
        }
    }
}

/// The same plane, moved along its normal.
fn shifted(plane: Plane, by: f64) -> Plane {
    match plane {
        Plane::Xy(offset) => Plane::Xy(offset + by),
        Plane::Xz(offset) => Plane::Xz(offset - by),
        Plane::Yz(offset) => Plane::Yz(offset + by),
        Plane::Tilted { origin, turn } => Plane::Tilted {
            origin: origin + plane.normal() * by,
            turn,
        },
    }
}
