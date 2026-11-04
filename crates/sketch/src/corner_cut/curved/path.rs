//! The line or the curve a side of a fillet beside a curve lies on, read two
//! ways: in plain numbers, where the search for the fillet walks it, and over
//! jets, where the solver asks how a place on it moves with the points the
//! side is drawn from. The two readings are the same formulas, side by side.

use std::f64::consts::TAU;

use glam::DVec2;

use crate::corner_cut::points::COORDINATES;
use crate::ellipsing::EllipseDraft;
use crate::jet::{Jet, JetPlace};

/// The line or the curve a side lies on, each place of it named by one
/// number: the distance along a line, the angle round a circle, the turn
/// round an ellipse — growing counter-clockwise round a curve.
#[derive(Clone, Copy, Debug)]
pub(super) enum Path {
    Line { through: DVec2, along: DVec2 },
    Circle { centre: DVec2, radius: f64 },
    Ellipse(EllipseDraft),
}

/// How a side runs away from the corner: along its path from `from`, the
/// corner, the way the sign of `way` says, as far as `reach`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Run {
    pub(super) path: Path,
    pub(super) from: f64,
    pub(super) way: f64,
    pub(super) reach: f64,
}

/// A [`Path`] whose numbers each carry how they move with the points the
/// side is drawn from.
// Two of these live for one reading of a rule; boxing the ellipse would cost
// an allocation on every one of them, and the type its `Copy`.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Copy, Debug)]
pub(super) enum JetPath {
    Line {
        through: JetPlace<COORDINATES>,
        along: JetPlace<COORDINATES>,
    },
    Circle {
        centre: JetPlace<COORDINATES>,
        radius: Jet<COORDINATES>,
    },
    /// An ellipse's centre, from it to one end of its first axis, and how
    /// far its second axis reaches.
    Ellipse {
        centre: JetPlace<COORDINATES>,
        first: JetPlace<COORDINATES>,
        second: Jet<COORDINATES>,
    },
}

impl Run {
    /// The unit direction the side leaves the corner in.
    pub(super) fn leaving(self) -> DVec2 {
        (self.path.onwards(self.from) * self.way).normalize_or_zero()
    }

    /// Where on the path a place `back` from the corner, along the side's
    /// tangent there, lies near.
    pub(super) fn first_guess(self, back: f64) -> f64 {
        self.from + self.way * back / self.path.onwards(self.from).length()
    }

    /// How far along the side from the corner a place of its path lies, in
    /// the path's own measure: round a curve, the way the side runs, short of
    /// a whole turn.
    pub(super) fn along(self, at: f64) -> f64 {
        let along = (at - self.from) * self.way;
        match self.path {
            Path::Line { .. } => along,
            Path::Circle { .. } | Path::Ellipse(_) => along.rem_euclid(TAU),
        }
    }

    /// How far along the side there is room, in that same measure, the
    /// drawing's tolerance given.
    pub(super) fn room(self, tolerance: f64) -> f64 {
        self.reach + tolerance / self.path.onwards(self.from).length()
    }
}

impl Path {
    pub(super) fn place(self, at: f64) -> DVec2 {
        match self {
            Self::Line { through, along } => through + along * at,
            Self::Circle { centre, radius } => centre + DVec2::from_angle(at) * radius,
            Self::Ellipse(drawn) => drawn.at(at),
        }
    }

    /// How fast the place moves as its number grows.
    fn onwards(self, at: f64) -> DVec2 {
        match self {
            Self::Line { along, .. } => along,
            Self::Circle { radius, .. } => DVec2::from_angle(at).perp() * radius,
            Self::Ellipse(drawn) => {
                let (sin, cos) = at.sin_cos();
                drawn.second_axis() * cos - drawn.first * sin
            }
        }
    }

    /// How fast `onwards` changes as the number grows.
    fn bending(self, at: f64) -> DVec2 {
        match self {
            Self::Line { .. } => DVec2::ZERO,
            Self::Circle { radius, .. } => -DVec2::from_angle(at) * radius,
            Self::Ellipse(drawn) => drawn.centre - drawn.at(at),
        }
    }

    /// The unit normal on the path's right as it runs, which is outside a
    /// curve.
    pub(super) fn outwards(self, at: f64) -> DVec2 {
        let onwards = self.onwards(at);
        -onwards.perp() / onwards.length()
    }

    /// The place standing `by` off the path at `at`, outwards, and how fast
    /// that place moves.
    pub(super) fn off_by(self, at: f64, by: f64) -> (DVec2, DVec2) {
        let (onwards, bending) = (self.onwards(at), self.bending(at));
        let (speed, outwards) = (onwards.length(), self.outwards(at));
        let turning = (-bending.perp() - outwards * (onwards.dot(bending) / speed)) / speed;
        (self.place(at) + outwards * by, onwards + turning * by)
    }

    /// The radius of the circle the path bends along at `at`: no end of it
    /// along a line.
    pub(super) fn bend(self, at: f64) -> f64 {
        let (onwards, bending) = (self.onwards(at), self.bending(at));
        onwards.length().powi(3) / onwards.perp_dot(bending).abs()
    }
}

impl JetPath {
    /// [`Path::place`].
    pub(super) fn place(&self, at: Jet<COORDINATES>) -> JetPlace<COORDINATES> {
        match *self {
            Self::Line { through, along } => through + along.scaled(at),
            Self::Circle { centre, radius } => centre + turned(at).scaled(radius),
            Self::Ellipse {
                centre,
                first,
                second,
            } => {
                let across = first.perp().normalized().scaled(second);
                centre + first.scaled(at.cos()) + across.scaled(at.sin())
            }
        }
    }

    /// [`Path::onwards`].
    fn onwards(&self, at: Jet<COORDINATES>) -> JetPlace<COORDINATES> {
        match *self {
            Self::Line { along, .. } => along,
            Self::Circle { radius, .. } => turned(at).perp().scaled(radius),
            Self::Ellipse { first, second, .. } => {
                let across = first.perp().normalized().scaled(second);
                across.scaled(at.cos()) - first.scaled(at.sin())
            }
        }
    }

    /// The place standing `by` off the path at `at`, outwards: the first of
    /// what [`Path::off_by`] gives.
    pub(super) fn off_by(&self, at: Jet<COORDINATES>, by: f64) -> JetPlace<COORDINATES> {
        let onwards = self.onwards(at);
        let outwards = onwards
            .perp()
            .scaled(Jet::constant(-1.0) / onwards.length());
        self.place(at) + outwards.scaled(Jet::constant(by))
    }

    /// [`Run::leaving`]: the unit direction `run` leaves its corner in, the
    /// corner standing at `corner`.
    pub(super) fn leaving(&self, run: Run, corner: JetPlace<COORDINATES>) -> JetPlace<COORDINATES> {
        let from = Jet {
            value: run.from,
            slope: self.number_at(corner).slope,
        };
        self.onwards(from)
            .scaled(Jet::constant(run.way))
            .normalized()
    }

    /// The number naming `place` on the path, as `run_away_from` reads the
    /// corner's: how far along a line, the angle round a circle, the turn
    /// round an ellipse — the place squashed onto the circle its axes make
    /// round, as `EllipseDraft::turn_on` reads it.
    fn number_at(&self, place: JetPlace<COORDINATES>) -> Jet<COORDINATES> {
        match *self {
            Self::Line { through, along } => (place - through).dot(along),
            Self::Circle { centre, .. } => {
                let out = place - centre;
                out.y.atan2(out.x)
            }
            Self::Ellipse {
                centre,
                first,
                second,
            } => {
                let out = place - centre;
                let along = first.length();
                let across = first.perp().normalized().scaled(second);
                let squashed_across = out.dot(across) / (second * second);
                squashed_across.atan2(out.dot(first) / (along * along))
            }
        }
    }
}

/// The unit direction at angle `at`, as `DVec2::from_angle` gives it.
fn turned(at: Jet<COORDINATES>) -> JetPlace<COORDINATES> {
    JetPlace {
        x: at.cos(),
        y: at.sin(),
    }
}
