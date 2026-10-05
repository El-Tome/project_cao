//! Solids drawn at random, the operations that combine them, and everything a
//! failing one needs to be read and run again by a human.
//!
//! A case is a first solid and the steps that add to it or cut into it, each
//! with a solid of its own — the shape of what a part is in the application.
//! Its vocabulary is the one a printed case is written in, so a case found by
//! a campaign pastes into a named test as it is.

mod along;
mod arithmetic;
mod around;
mod building;
#[cfg(feature = "campaigns")]
pub mod campaigning;
mod checking;
mod cores;
mod drawing;
mod kernels;
mod outlines;
mod printing;
mod sections;
mod smaller;

use glam::{DQuat, DVec2, DVec3};

pub use along::{Crossing, Stretch};
pub use arithmetic::{Measured, held_to_arithmetic, holds_exactly, holds_through_the_application};
pub use around::{Swept, Turned};
pub use building::Drawn;
pub use checking::{check, holds, kept_its_promise, within_reach};
pub use cores::on_every_core;
pub use kernels::{Application, Exact, Flats, Kernel, TESSELLATION, whole_circle};
pub use sections::{Along, Axis, Piece, Section};

#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    pub start: Leaf,
    pub steps: Vec<Step>,
}

impl Case {
    pub fn new(start: Leaf, steps: Vec<Step>) -> Self {
        Self { start, steps }
    }

    pub fn leaves(&self) -> impl Iterator<Item = &Leaf> {
        std::iter::once(&self.start).chain(self.steps.iter().map(|step| &step.tool))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Add,
    Cut,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Step {
    pub mode: Mode,
    pub tool: Leaf,
}

impl Step {
    pub fn add(tool: Leaf) -> Self {
        Self {
            mode: Mode::Add,
            tool,
        }
    }

    pub fn cut(tool: Leaf) -> Self {
        Self {
            mode: Mode::Cut,
            tool,
        }
    }
}

/// One solid of a case, as the kernel is asked to raise it.
#[derive(Clone, Debug, PartialEq)]
pub enum Leaf {
    /// An area pushed along the normal of its plane. A negative height pushes
    /// it the other way.
    Prism {
        plane: Plane,
        outline: Outline,
        height: f64,
    },
    /// A rectangle of the plane turned about the plane's second axis, which
    /// the rectangle's first coordinates keep to one side of.
    Revolution {
        plane: Plane,
        low: DVec2,
        high: DVec2,
        degrees: f64,
    },
    /// A section of straight runs turned about a line of the plane by
    /// `degrees`, either way, a whole turn at most.
    Turned {
        plane: Plane,
        axis: Axis,
        section: Section,
        degrees: f64,
    },
}

impl Leaf {
    pub fn prism(plane: Plane, outline: Outline, height: f64) -> Self {
        Self::Prism {
            plane,
            outline,
            height,
        }
    }

    pub fn revolution(plane: Plane, low: [f64; 2], high: [f64; 2], degrees: f64) -> Self {
        Self::Revolution {
            plane,
            low: DVec2::from(low),
            high: DVec2::from(high),
            degrees,
        }
    }

    pub fn turned(plane: Plane, axis: Axis, section: Section, degrees: f64) -> Self {
        Self::Turned {
            plane,
            axis,
            section,
            degrees,
        }
    }

    pub fn plane(&self) -> &Plane {
        match self {
            Leaf::Prism { plane, .. }
            | Leaf::Revolution { plane, .. }
            | Leaf::Turned { plane, .. } => plane,
        }
    }
}

/// Where a leaf is drawn: one of the three planes of the origin moved along
/// its normal, or a plane turned by three angles in degrees about X, Y and Z.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Plane {
    Xy(f64),
    Xz(f64),
    Yz(f64),
    Tilted { origin: DVec3, turn: DVec3 },
}

impl Plane {
    pub fn xy(offset: f64) -> Self {
        Self::Xy(offset)
    }

    pub fn xz(offset: f64) -> Self {
        Self::Xz(offset)
    }

    pub fn yz(offset: f64) -> Self {
        Self::Yz(offset)
    }

    pub fn tilted(origin: [f64; 3], turn: [f64; 3]) -> Self {
        Self::Tilted {
            origin: DVec3::from(origin),
            turn: DVec3::from(turn),
        }
    }

    /// The plane's origin and its two axes, which are square to each other and
    /// of unit length, so that areas drawn in it keep their size in space.
    pub fn frame(self) -> (DVec3, DVec3, DVec3) {
        match self {
            Plane::Xy(offset) => (DVec3::Z * offset, DVec3::X, DVec3::Y),
            Plane::Xz(offset) => (DVec3::Y * offset, DVec3::X, DVec3::Z),
            Plane::Yz(offset) => (DVec3::X * offset, DVec3::Y, DVec3::Z),
            Plane::Tilted { origin, turn } => {
                let rotation = DQuat::from_euler(
                    glam::EulerRot::XYZ,
                    turn.x.to_radians(),
                    turn.y.to_radians(),
                    turn.z.to_radians(),
                );
                (origin, rotation * DVec3::X, rotation * DVec3::Y)
            }
        }
    }

    pub fn to_world(self, point: DVec2) -> DVec3 {
        let (origin, u, v) = self.frame();
        origin + u * point.x + v * point.y
    }

    pub fn normal(self) -> DVec3 {
        let (_, u, v) = self.frame();
        u.cross(v)
    }
}

/// What a prism is raised from, in the plane's own coordinates.
#[derive(Clone, Debug, PartialEq)]
pub enum Outline {
    Rectangle {
        low: DVec2,
        high: DVec2,
    },
    /// Sampled the way the application samples a circle, into as many flats
    /// and marked as one curve, from the angle in degrees where its first
    /// corner lies: nought for a whole circle, the crossing for one a trait
    /// has broken.
    Circle {
        center: DVec2,
        radius: f64,
        from: f64,
    },
    /// Corners around a centre every one of them can be seen from, so that a
    /// fan from the centre fills it whether it is convex or not.
    Star {
        center: DVec2,
        corners: Vec<DVec2>,
    },
    /// A circle with a smaller one bored through its middle: a tube once
    /// raised.
    Ring {
        center: DVec2,
        outer: f64,
        inner: f64,
    },
    /// A rectangle whose four corners are turned into quarter circles of
    /// `radius`, no larger than half its shorter side: at half, the two
    /// quarters at either end of that side meet and leave it no straight run.
    Rounded {
        low: DVec2,
        high: DVec2,
        radius: f64,
    },
    /// Two half circles of `radius` about `from` and `to`, joined by two
    /// straight runs: an oblong along the plane's first axis or its second.
    Slot {
        from: DVec2,
        to: DVec2,
        radius: f64,
    },
}

impl Outline {
    pub fn rectangle(low: [f64; 2], high: [f64; 2]) -> Self {
        Self::Rectangle {
            low: DVec2::from(low),
            high: DVec2::from(high),
        }
    }

    pub fn circle(center: [f64; 2], radius: f64) -> Self {
        Self::circle_from(center, radius, 0.0)
    }

    pub fn circle_from(center: [f64; 2], radius: f64, from: f64) -> Self {
        Self::Circle {
            center: DVec2::from(center),
            radius,
            from,
        }
    }

    pub fn star(center: [f64; 2], corners: &[[f64; 2]]) -> Self {
        Self::Star {
            center: DVec2::from(center),
            corners: corners.iter().map(|corner| DVec2::from(*corner)).collect(),
        }
    }

    pub fn ring(center: [f64; 2], outer: f64, inner: f64) -> Self {
        Self::Ring {
            center: DVec2::from(center),
            outer,
            inner,
        }
    }

    pub fn rounded(low: [f64; 2], high: [f64; 2], radius: f64) -> Self {
        Self::Rounded {
            low: DVec2::from(low),
            high: DVec2::from(high),
            radius,
        }
    }

    pub fn slot(from: [f64; 2], to: [f64; 2], radius: f64) -> Self {
        Self::Slot {
            from: DVec2::from(from),
            to: DVec2::from(to),
            radius,
        }
    }
}

/// How many flats a circle is sampled into: what the application uses, so
/// that the cases drawn here are the shapes it makes.
pub const CIRCLE_STEPS: usize = 48;
