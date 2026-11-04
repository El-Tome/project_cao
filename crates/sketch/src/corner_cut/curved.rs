//! A fillet on a corner one of whose sides is a curve — an arc, or an arc of
//! ellipse — worked out every time it is read.
//!
//! The fillet stands in the angle under half a turn that the two sides'
//! tangents make at the corner. Its centre stands a radius off each side,
//! towards that angle: beside a trait, on the line running alongside it;
//! beside a curve, inside it where the curve is hollow towards the angle and
//! outside where it bulges. An ellipse's offset is no ellipse and has no
//! closed form, so the centre is found by Newton's method on where the two
//! offsets meet, started from the fillet the sides' tangents alone would take:
//! the one that grows out of the corner as the radius does. It fits when the
//! places it touches lie on what is drawn of the sides.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::drawn::{CutCurve, DrawnCut, NOTHING, short_way_round};
use super::{CornerCut, CornerCutId, CutShape, CutSide};
use crate::arcing::sweep_of;
use crate::ellipsing::EllipseDraft;
use crate::sketch::{PointId, Sketch};

/// How many steps the search for the centre takes at most.
const STEPS: usize = 64;

/// How near the two offsets must come for the centre to be taken, against
/// how far the corner stands from the origin and the radius.
const MET: f64 = 1e-10;

/// Under this share of a step, a step that brings the offsets no nearer is
/// given up on: they are as near as the arithmetic takes them.
const SHORTEST: f64 = 1.0 / 1024.0;

/// The line or the curve a side lies on, each place of it named by one
/// number: the distance along a line, the angle round a circle, the turn
/// round an ellipse — growing counter-clockwise round a curve.
#[derive(Clone, Copy, Debug)]
enum Path {
    Line { through: DVec2, along: DVec2 },
    Circle { centre: DVec2, radius: f64 },
    Ellipse(EllipseDraft),
}

/// How a side runs away from the corner: along its path from `from`, the
/// corner, the way the sign of `way` says, as far as `reach`.
#[derive(Clone, Copy, Debug)]
struct Run {
    path: Path,
    from: f64,
    way: f64,
    reach: f64,
}

impl Sketch {
    /// A cut beside a curve as it is drawn now: nothing when its corner gives
    /// it no place, or for a chamfer, which no such corner takes.
    pub(super) fn drawn_beside_a_curve(
        &self,
        id: CornerCutId,
        cut: &CornerCut,
        tolerance: f64,
    ) -> Option<DrawnCut> {
        let CutShape::Fillet { radius } = cut.shape else {
            return None;
        };
        let corner = self.point(cut.corner);
        let runs = [
            self.run_away_from(cut.sides[0], cut.corner)?,
            self.run_away_from(cut.sides[1], cut.corner)?,
        ];
        let (centre, at) = fillet_between(corner, runs, radius)?;
        let touches = [runs[0].path.place(at[0]), runs[1].path.place(at[1])];
        let fits = runs
            .iter()
            .zip(at)
            .all(|(run, at)| run.holds(at, tolerance));
        Some(DrawnCut {
            id,
            sides: cut.sides,
            corner_point: cut.corner,
            corner,
            touches,
            curve: CutCurve::Arc(short_way_round(centre, touches)),
            fits,
            construction: cut.construction,
        })
    }

    fn run_away_from(&self, side: CutSide, corner: PointId) -> Option<Run> {
        let far = self.far_end_of(side, corner)?;
        let at = self.point(corner);
        let way = |forwards: bool| match forwards {
            true => 1.0,
            false => -1.0,
        };
        Some(match side {
            CutSide::Trait(_) => {
                let to = self.point(far);
                Run {
                    path: Path::Line {
                        through: at,
                        along: (to - at).try_normalize()?,
                    },
                    from: 0.0,
                    way: 1.0,
                    reach: at.distance(to),
                }
            }
            CutSide::Arc(arc) => {
                let drawn = self.arc_draft(arc);
                Run {
                    path: Path::Circle {
                        centre: drawn.centre,
                        radius: self.arc_radius(arc),
                    },
                    from: (at - drawn.centre).to_angle(),
                    way: way(self.arc(arc).start == corner),
                    reach: sweep_of(drawn),
                }
            }
            CutSide::ArcOfEllipse(ellipse) => {
                let drawn = self.ellipse_draft(ellipse);
                let (start, _) = self.ellipse_ends(ellipse)?;
                Run {
                    path: Path::Ellipse(drawn),
                    from: drawn.turn_on(at),
                    way: way(start == corner),
                    reach: self.ellipse_run(ellipse).1,
                }
            }
        })
    }
}

/// The centre of the fillet of `radius` between two sides leaving `corner`,
/// and where on each side's path it touches — nothing when the sides leave
/// the corner straight on or the same way, when no such circle touches both,
/// or when the circle would have to stand inside a curve bending tighter than
/// itself.
fn fillet_between(corner: DVec2, runs: [Run; 2], radius: f64) -> Option<(DVec2, [f64; 2])> {
    let leaving = runs.map(Run::leaving);
    let turn = leaving[0].perp_dot(leaving[1]);
    let opening = turn.abs().atan2(leaving[0].dot(leaving[1]));
    if radius <= NOTHING || opening <= NOTHING || opening >= PI - NOTHING {
        return None;
    }
    let inwards = [
        leaving[0].perp() * turn.signum(),
        -leaving[1].perp() * turn.signum(),
    ];
    let off = [0, 1].map(|rank| {
        let outwards = runs[rank].path.outwards(runs[rank].from);
        radius * outwards.dot(inwards[rank]).signum()
    });
    let centre_from = |rank: usize, at: f64| runs[rank].path.off_by(at, off[rank]);
    let miss = |at: [f64; 2]| centre_from(1, at[1]).0 - centre_from(0, at[0]).0;
    let met = MET * (corner.length() + radius);

    let back = radius / (opening * 0.5).tan();
    let mut at = runs.map(|run| run.first_guess(back));
    'search: for _ in 0..STEPS {
        let gap = miss(at);
        if gap == DVec2::ZERO {
            break;
        }
        let (one, other) = (centre_from(0, at[0]).1, -centre_from(1, at[1]).1);
        let across = one.perp_dot(other);
        if across == 0.0 {
            break;
        }
        let step = [gap.perp_dot(other) / across, one.perp_dot(gap) / across];
        let mut share = 1.0;
        loop {
            let tried = [at[0] + step[0] * share, at[1] + step[1] * share];
            if miss(tried).length() < gap.length() {
                at = tried;
                break;
            }
            share *= 0.5;
            if share < SHORTEST {
                break 'search;
            }
        }
    }
    let tight = (0..2).any(|rank| off[rank] < 0.0 && radius >= runs[rank].path.bend(at[rank]));
    (miss(at).length() <= met && !tight).then(|| (centre_from(0, at[0]).0, at))
}

impl Run {
    /// The unit direction the side leaves the corner in.
    fn leaving(self) -> DVec2 {
        (self.path.onwards(self.from) * self.way).normalize_or_zero()
    }

    /// Where on the path a place `back` from the corner, along the side's
    /// tangent there, lies near.
    fn first_guess(self, back: f64) -> f64 {
        self.from + self.way * back / self.path.onwards(self.from).length()
    }

    /// Whether a place of the path lies on what is drawn of the side.
    fn holds(self, at: f64, tolerance: f64) -> bool {
        let along = (at - self.from) * self.way;
        let slack = tolerance / self.path.onwards(self.from).length();
        match self.path {
            Path::Line { .. } => (0.0..=self.reach + slack).contains(&along),
            Path::Circle { .. } | Path::Ellipse(_) => along.rem_euclid(TAU) <= self.reach + slack,
        }
    }
}

impl Path {
    fn place(self, at: f64) -> DVec2 {
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
    fn outwards(self, at: f64) -> DVec2 {
        let onwards = self.onwards(at);
        -onwards.perp() / onwards.length()
    }

    /// The place standing `by` off the path at `at`, outwards, and how fast
    /// that place moves.
    fn off_by(self, at: f64, by: f64) -> (DVec2, DVec2) {
        let (onwards, bending) = (self.onwards(at), self.bending(at));
        let (speed, outwards) = (onwards.length(), self.outwards(at));
        let turning = (-bending.perp() - outwards * (onwards.dot(bending) / speed)) / speed;
        (self.place(at) + outwards * by, onwards + turning * by)
    }

    /// The radius of the circle the path bends along at `at`: no end of it
    /// along a line.
    fn bend(self, at: f64) -> f64 {
        let (onwards, bending) = (self.onwards(at), self.bending(at));
        onwards.length().powi(3) / onwards.perp_dot(bending).abs()
    }
}
