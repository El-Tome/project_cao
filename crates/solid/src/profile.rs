//! A profile as the drawing hands it to a kernel: closed loops of straight runs
//! and arcs, and the frame they stand in. The exact kernel (`brep`, #498)
//! raises one.

use glam::{DVec2, DVec3};

/// How a stretch of a profile runs to the next corner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Run {
    Straight,
    /// Along the circle about `center`, the short way round or the long way
    /// as `turn` says, in radians, signed as the turn goes.
    Round {
        center: DVec2,
        turn: f64,
    },
}

/// A closed loop of a profile: `corners[i]` runs to `corners[i + 1]` as
/// `runs[i]` says.
#[derive(Clone, Debug, PartialEq)]
pub struct Contour {
    pub corners: Vec<DVec2>,
    pub runs: Vec<Run>,
}

impl Contour {
    pub fn straight(corners: Vec<DVec2>) -> Self {
        let runs = vec![Run::Straight; corners.len()];
        Self { corners, runs }
    }

    /// A whole circle, as two halves meeting at two corners: the drawing may
    /// hand a circle cut in arcs, and the kernel makes one wall of two arcs on
    /// one circle.
    pub fn circle(center: DVec2, radius: f64) -> Self {
        let half = std::f64::consts::PI;
        Self {
            corners: vec![center + DVec2::X * radius, center - DVec2::X * radius],
            runs: vec![
                Run::Round { center, turn: half },
                Run::Round { center, turn: half },
            ],
        }
    }

    /// An axis-aligned rectangle between two opposite corners, turning
    /// anticlockwise.
    pub fn rectangle(low: DVec2, high: DVec2) -> Self {
        Self::straight(vec![
            low,
            DVec2::new(high.x, low.y),
            high,
            DVec2::new(low.x, high.y),
        ])
    }
}

/// Where a profile stands: an origin and two axes, square and of unit length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub origin: DVec3,
    pub u: DVec3,
    pub v: DVec3,
}

impl Frame {
    pub fn at(&self, point: DVec2) -> DVec3 {
        self.origin + self.u * point.x + self.v * point.y
    }

    pub fn normal(&self) -> DVec3 {
        self.u.cross(self.v)
    }
}
