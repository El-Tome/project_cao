//! A fillet on a corner one of whose sides is an arc, worked out in closed
//! form every time it is read.
//!
//! The fillet stands in the angle under half a turn that the two sides'
//! tangents make at the corner. Each side is moved off by the radius towards
//! that angle — a trait to the line beside it, an arc to the circle a radius
//! further in when its centre is on that side, further out otherwise — and the
//! fillet's centre is where the two meet nearest the corner. It touches each
//! side where the side is nearest that centre, and fits when both of those
//! places lie on what is drawn of the sides.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::drawn::{CutCurve, DrawnCut, NOTHING, short_way_round};
use super::{CornerCut, CornerCutId, CutShape, CutSide};
use crate::arcing::sweep_of;
use crate::sketch::{PointId, Sketch};

/// How a side runs away from the corner.
#[derive(Clone, Copy, Debug)]
enum Run {
    /// Along a line, in the unit direction `along`, as far as `length`.
    Straight { along: DVec2, length: f64 },
    /// Round a circle, counter-clockwise when `ccw`, as far round as `sweep`.
    Round {
        centre: DVec2,
        radius: f64,
        ccw: bool,
        sweep: f64,
    },
}

/// A line or a circle a fillet's centre stands on.
#[derive(Clone, Copy, Debug)]
enum Carrier {
    Line { through: DVec2, along: DVec2 },
    Circle { centre: DVec2, radius: f64 },
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
        let (centre, touches) = fillet_between(corner, runs, radius)?;
        let fits = runs
            .iter()
            .zip(touches)
            .all(|(run, touch)| run.holds(corner, touch, tolerance));
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
        match side {
            CutSide::Trait(_) => {
                let (from, to) = (self.point(corner), self.point(far));
                Some(Run::Straight {
                    along: (to - from).try_normalize()?,
                    length: from.distance(to),
                })
            }
            CutSide::Arc(arc) => {
                let drawn = self.arc_draft(arc);
                Some(Run::Round {
                    centre: drawn.centre,
                    radius: self.arc_radius(arc),
                    ccw: self.arc(arc).start == corner,
                    sweep: sweep_of(drawn),
                })
            }
            CutSide::ArcOfEllipse(_) => None,
        }
    }
}

/// The centre of the fillet of `radius` between two sides leaving `corner`,
/// and where it touches each — nothing when the sides leave it straight on
/// or the same way, or no circle of that radius touches both.
fn fillet_between(corner: DVec2, runs: [Run; 2], radius: f64) -> Option<(DVec2, [DVec2; 2])> {
    let leaving = runs.map(|run| run.leaving(corner));
    let turn = leaving[0].perp_dot(leaving[1]);
    let opening = turn.abs().atan2(leaving[0].dot(leaving[1]));
    if radius <= NOTHING || opening <= NOTHING || opening >= PI - NOTHING {
        return None;
    }
    let inwards = [
        leaving[0].perp() * turn.signum(),
        -leaving[1].perp() * turn.signum(),
    ];
    let carriers = [
        runs[0].moved_off(corner, inwards[0], radius)?,
        runs[1].moved_off(corner, inwards[1], radius)?,
    ];
    let centre = crossings(carriers[0], carriers[1])
        .into_iter()
        .min_by(|one, other| one.distance(corner).total_cmp(&other.distance(corner)))?;
    Some((centre, runs.map(|run| run.touch(corner, centre))))
}

impl Run {
    /// The unit direction the side leaves the corner in.
    fn leaving(self, corner: DVec2) -> DVec2 {
        match self {
            Self::Straight { along, .. } => along,
            Self::Round { centre, ccw, .. } => {
                let across = (corner - centre).normalize_or_zero().perp();
                match ccw {
                    true => across,
                    false => -across,
                }
            }
        }
    }

    /// The line or circle standing `by` off the side, towards `inwards`.
    fn moved_off(self, corner: DVec2, inwards: DVec2, by: f64) -> Option<Carrier> {
        match self {
            Self::Straight { along, .. } => Some(Carrier::Line {
                through: corner + inwards * by,
                along,
            }),
            Self::Round { centre, radius, .. } => {
                let radius = match (centre - corner).dot(inwards) > 0.0 {
                    true => radius - by,
                    false => radius + by,
                };
                (radius > NOTHING).then_some(Carrier::Circle { centre, radius })
            }
        }
    }

    /// Where a circle centred on `centre` touches the side.
    fn touch(self, corner: DVec2, centre: DVec2) -> DVec2 {
        match self {
            Self::Straight { along, .. } => corner + along * (centre - corner).dot(along),
            Self::Round {
                centre: round,
                radius,
                ..
            } => round + (centre - round).normalize_or_zero() * radius,
        }
    }

    /// Whether a place of the side's line or circle lies on what is drawn of
    /// it, from the corner on.
    fn holds(self, corner: DVec2, place: DVec2, tolerance: f64) -> bool {
        match self {
            Self::Straight { along, length } => {
                (0.0..=length + tolerance).contains(&(place - corner).dot(along))
            }
            Self::Round {
                centre,
                radius,
                ccw,
                sweep,
            } => {
                let turned = (place - centre).to_angle() - (corner - centre).to_angle();
                let round = match ccw {
                    true => turned,
                    false => -turned,
                };
                round.rem_euclid(TAU) <= sweep + tolerance / radius
            }
        }
    }
}

/// Where two lines or circles cross.
fn crossings(one: Carrier, other: Carrier) -> Vec<DVec2> {
    match (one, other) {
        (
            Carrier::Line {
                through: a,
                along: u,
            },
            Carrier::Line {
                through: b,
                along: v,
            },
        ) => {
            let across = u.perp_dot(v);
            match across.abs() > NOTHING {
                true => vec![a + u * ((b - a).perp_dot(v) / across)],
                false => Vec::new(),
            }
        }
        (Carrier::Line { through, along }, Carrier::Circle { centre, radius })
        | (Carrier::Circle { centre, radius }, Carrier::Line { through, along }) => {
            let foot = through + along * (centre - through).dot(along);
            either_way(foot, along, radius * radius - foot.distance_squared(centre))
        }
        (
            Carrier::Circle {
                centre: a,
                radius: ra,
            },
            Carrier::Circle {
                centre: b,
                radius: rb,
            },
        ) => {
            let apart = a.distance(b);
            if apart <= NOTHING {
                return Vec::new();
            }
            let towards = (b - a) / apart;
            let along = (ra * ra - rb * rb + apart * apart) / (2.0 * apart);
            either_way(a + towards * along, towards.perp(), ra * ra - along * along)
        }
    }
}

/// The two places as far as the root of `reach_squared` either way from
/// `foot` along `along`, none when it is negative.
fn either_way(foot: DVec2, along: DVec2, reach_squared: f64) -> Vec<DVec2> {
    if reach_squared < 0.0 {
        return Vec::new();
    }
    let reach = reach_squared.sqrt();
    vec![foot - along * reach, foot + along * reach]
}
