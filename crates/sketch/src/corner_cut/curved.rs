//! A fillet on a corner one of whose sides is a curve — an arc, or an arc of
//! ellipse — worked out every time it is read.
//!
//! The fillet stands in the angle under half a turn that the two sides'
//! tangents make at the corner. Its centre stands a radius off each side,
//! towards that angle: beside a trait, on the line running alongside it;
//! beside a curve, inside it where the curve is hollow towards the angle and
//! outside where it bulges. An ellipse's offset is no ellipse and has no
//! closed form, so the centre is found by Newton's method on where the two
//! offsets meet. The fillet followed is the one that grows out of the corner
//! as the radius does: one small enough for the sides' tangents alone to say
//! where it touches is found first, and its radius doubled up to the one
//! asked, each search starting where the last one ended — so a fillet far
//! bigger than the curve beside it does not land on the other crossing. It
//! fits when the places it touches lie on what is drawn of the sides,
//! together with what the cuts at their other ends take.

use std::f64::consts::PI;

use glam::DVec2;

use super::drawn::{CutCurve, NOTHING, Worked, short_way_round};
use super::{CornerCut, CornerCutId, CutShape, CutSide};
use crate::arcing::sweep_of;
use crate::sketch::{PointId, Sketch};

mod path;
mod places;

use path::{Path, Run};

/// How many steps the search for the centre takes at most.
const STEPS: usize = 64;

/// How near the two offsets must come for the centre to be taken, against
/// how far the corner stands from the origin and the radius.
const MET: f64 = 1e-10;

/// Under this share of a step, a step that brings the offsets no nearer is
/// given up on: they are as near as the arithmetic takes them.
const SHORTEST: f64 = 1.0 / 1024.0;

/// How far back along the sides' tangents a fillet may touch, against the
/// tightest bend of a side at the corner, for those tangents to be a good
/// guess at where it touches the sides themselves.
const GUESSED: f64 = 0.25;

/// How many times at most a fillet's radius is doubled on its way out of the
/// corner to the one asked.
const DOUBLINGS: i32 = 16;

/// How many times the search for the largest value that fits halves what is
/// left to search: as many as a number carries digits.
const HALVINGS: usize = 52;

impl Sketch {
    /// A cut beside a curve worked out on its own: nothing when its corner
    /// gives it no place, or for a chamfer, which no such corner takes.
    pub(super) fn worked_beside_a_curve(&self, cut: &CornerCut, tolerance: f64) -> Option<Worked> {
        let CutShape::Fillet { radius } = cut.shape else {
            return None;
        };
        let runs = self.runs_away_from(cut.sides, cut.corner)?;
        fillet_between(self.point(cut.corner), runs, radius, tolerance)
    }

    /// How much of a fillet's radius corners among which one stands beside a
    /// curve take at most, as a share of it: 1 when they fit as given,
    /// nothing when one gives it no place.
    ///
    /// Such a fillet takes nothing from its sides in proportion to its
    /// radius, so the share is found by halving: each share tried is laid on
    /// a copy of the drawing, where every corner laid must be drawn and fit.
    pub(super) fn share_by_halving(
        &self,
        corners: &[(PointId, CutSide, CutSide)],
        radius: f64,
    ) -> f64 {
        let fits = |share: f64| {
            let mut trial = self.clone();
            let group = trial.fresh_cut_group();
            let shape = CutShape::Fillet {
                radius: radius * share,
            };
            let laid: Vec<CornerCutId> = corners
                .iter()
                .filter_map(|(corner, first, second)| {
                    trial
                        .cut_corner_at(*corner, *first, *second, shape, false, group)
                        .ok()
                })
                .collect();
            let drawn = trial.drawn_cuts();
            laid.iter()
                .filter(|id| !trial.is_erased_cut(**id))
                .map(|id| drawn.iter().find(|cut| cut.id == *id).map(|cut| cut.fits))
                .collect::<Option<Vec<bool>>>()
                .map(|fits| fits.into_iter().all(|fit| fit))
        };
        match fits(1.0) {
            None => return 0.0,
            Some(true) => return 1.0,
            Some(false) => {}
        }
        let (mut fitting, mut not) = (0.0, 1.0);
        for _ in 0..HALVINGS {
            let half = (fitting + not) * 0.5;
            match fits(half) == Some(true) {
                true => fitting = half,
                false => not = half,
            }
        }
        fitting
    }

    fn runs_away_from(&self, sides: [CutSide; 2], corner: PointId) -> Option<[Run; 2]> {
        Some([
            self.run_away_from(sides[0], corner)?,
            self.run_away_from(sides[1], corner)?,
        ])
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

/// The fillet of `radius` between two sides leaving `corner`, worked out on
/// its own — nothing when the sides leave the corner straight on or the same
/// way.
///
/// When no circle of that radius touches both near the corner, or it would
/// have to stand inside a curve bending tighter than itself, the fillet fits
/// nowhere, and is drawn where the sides' tangents alone would put it.
fn fillet_between(corner: DVec2, runs: [Run; 2], radius: f64, tolerance: f64) -> Option<Worked> {
    let Found {
        leaving,
        opening,
        off,
        at,
    } = found(corner, runs, radius)?;
    let back = radius / (opening * 0.5).tan();
    let room = runs.map(|run| run.room(tolerance));
    let Some(at) = at else {
        let centre =
            corner + (leaving[0] + leaving[1]).normalize() * (radius / (opening * 0.5).sin());
        let touches = leaving.map(|away| corner + away * back);
        return Some(Worked {
            touches,
            curve: CutCurve::Arc(short_way_round(centre, touches)),
            takes: runs.map(|run| run.along(run.first_guess(back))),
            room,
            fits_alone: false,
        });
    };
    let touches = [runs[0].path.place(at[0]), runs[1].path.place(at[1])];
    let takes = [runs[0].along(at[0]), runs[1].along(at[1])];
    Some(Worked {
        touches,
        curve: CutCurve::Arc(short_way_round(
            runs[0].path.off_by(at[0], off[0]).0,
            touches,
        )),
        takes,
        room,
        fits_alone: takes.iter().all(|take| *take >= 0.0),
    })
}

/// A fillet between two sides as the search finds it: which way each side
/// leaves the corner, how wide they open there, how far off each side's path
/// its centre stands — outwards, or inwards when negative — and where on each
/// path it touches, when it touches anywhere.
struct Found {
    leaving: [DVec2; 2],
    opening: f64,
    off: [f64; 2],
    at: Option<[f64; 2]>,
}

/// The fillet of `radius` between two sides leaving `corner`, as both its
/// drawing and what hangs on it read it — nothing when the sides leave the
/// corner straight on or the same way.
fn found(corner: DVec2, runs: [Run; 2], radius: f64) -> Option<Found> {
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
    Some(Found {
        leaving,
        opening,
        off,
        at: meeting(corner, runs, off, opening),
    })
}

/// Where on each side's path a fillet touches it whose centre stands `off`
/// each path, between sides opening by `opening` at the corner — followed out
/// of the corner from a fillet small enough for the tangents to say where it
/// touches, its radius doubled up to the one asked. Nothing when a search
/// fails on the way, or the fillet would have to stand inside a curve bending
/// tighter than itself.
fn meeting(corner: DVec2, runs: [Run; 2], off: [f64; 2], opening: f64) -> Option<[f64; 2]> {
    let radius = off[0].abs();
    let tightest = runs
        .iter()
        .map(|run| run.path.bend(run.from))
        .fold(f64::INFINITY, f64::min);
    let back_of = |radius: f64| radius / (opening * 0.5).tan();
    let doublings = (back_of(radius) / (tightest * GUESSED))
        .log2()
        .ceil()
        .clamp(0.0, f64::from(DOUBLINGS)) as i32;
    let mut at = runs.map(|run| run.first_guess(back_of(radius * 0.5_f64.powi(doublings))));
    for doubling in (0..=doublings).rev() {
        let share = 0.5_f64.powi(doubling);
        at = searched(corner, runs, off.map(|off| off * share), at)?;
    }
    let tight = (0..2).any(|rank| off[rank] < 0.0 && radius >= runs[rank].path.bend(at[rank]));
    (!tight).then_some(at)
}

/// Where on each side's path the places standing `off` it meet, by Newton's
/// method from `at` — nothing when they do not come near enough.
fn searched(corner: DVec2, runs: [Run; 2], off: [f64; 2], mut at: [f64; 2]) -> Option<[f64; 2]> {
    let centre_from = |rank: usize, at: f64| runs[rank].path.off_by(at, off[rank]);
    let miss = |at: [f64; 2]| centre_from(1, at[1]).0 - centre_from(0, at[0]).0;
    let met = MET * (corner.length() + off[0].abs());
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
    (miss(at).length() <= met).then_some(at)
}
