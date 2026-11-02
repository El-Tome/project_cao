//! The arcs of a profile laid against its axis (#576): the centre of each
//! laid like a corner — on the axis within the band, else on a level within
//! the tolerance — or at the middle of a half circle ending tangent to the
//! run leaving it, its radius read again from the laid centre to the laid
//! start, its end laid on that circle along the coordinate its next run
//! leaves free, and its turn read again between the laid corners. So a ball
//! end, a rounded shoulder and a round-bottom groove are tangent by
//! construction where the laid numbers say so, and the circle a corner turns
//! along is one circle for the arc and the run beside it.
//!
//! An arc joins no level, and the rules that re-read a laid profile column by
//! column are not taught arcs: an arc that crosses a run, that a corner
//! comes within the tolerance of, or that reaches the axis anywhere but at
//! its ends — a pinch — is not laid, and the body declines the profile.

use std::f64::consts::TAU;

use glam::DVec2;

/// The run leaving a corner, as laid: the centre of its circle in `(h, r)`,
/// `r` nought on the axis and negative across it, and how far round it
/// turns, in radians, anticlockwise in `(h, r)` positive.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bend {
    pub center: DVec2,
    pub turn: f64,
}

/// An arc as the drawing hands it, read in `(h, r)`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Drawn {
    pub center: DVec2,
    pub turn: f64,
}

/// A run, by the corners it goes between.
type Run = (usize, usize);

/// How many roundings of a radius a corner may stand off its circle and
/// still be on it.
const ROUNDINGS: f64 = 16.0;

/// Each run's bend, the arcs' end corners moved onto their circles; `None`
/// where an arc cannot be laid. `raw` holds the corners as drawn, `arcs`
/// the drawn arc of each run, by the corner it leaves; `band` is how close
/// to the axis a centre counts as on it. An end may stand off its circle by
/// the tolerance and by what laying moved its centre and itself, no more.
/// An arc leaving a pole whose end stands within that of tangent to the
/// level its next run keeps — a ball end a hair larger or smaller than its
/// shaft — is laid tangent, the pole slid along the axis onto the circle,
/// since the shaft's level is shared and the pole's height is nobody
/// else's. A corner laid onto one of the arc's ends is no corner within the
/// tolerance of it. A half circle ending tangent to the run leaving it has
/// its centre laid at the middle of its ends ([`diameter`]).
pub(super) fn laid(
    laid: &mut [DVec2],
    raw: &[DVec2],
    arcs: &[Option<Drawn>],
    runs: &[Run],
    band: f64,
    tolerance: f64,
) -> Option<Vec<Option<Bend>>> {
    let heights: Vec<f64> = laid.iter().map(|corner| corner.x).collect();
    let radii: Vec<f64> = laid.iter().map(|corner| corner.y).collect();
    let straight = |run: &Run| arcs[run.0].is_none();
    let mut bends = vec![None; runs.len()];
    for &(from, to) in runs {
        let Some(drawn) = arcs[from] else {
            continue;
        };
        let next = runs.iter().find(|run| run.0 == to)?;
        let free = free(laid, next, straight(next));
        let center = diameter(
            laid[from],
            laid[to],
            DVec2::new(
                level(drawn.center.x, &heights, tolerance),
                if drawn.center.y.abs() <= band {
                    0.0
                } else {
                    level(drawn.center.y, &radii, tolerance)
                },
            ),
            &free,
        );
        let mut radius = (laid[from] - center).x.hypot((laid[from] - center).y);
        let end = laid[to];
        let moved = drawn.center.distance(center) + raw[to].distance(end);
        if ((end - center).length() - radius).abs() > tolerance + moved {
            return None;
        }
        if laid[from].y == 0.0
            && let Some(tangent) = tangent(end, center, &free)
            && tangent != radius
            && (tangent - radius).abs() <= tolerance + moved
        {
            radius = tangent;
            let start = laid[from];
            slid(laid, start, landed(start, center, radius, Free::Height)?);
        }
        slid(laid, end, landed(end, center, radius, free)?);
        let (start, end) = (laid[from] - center, laid[to] - center);
        let turned = start.perp_dot(end).atan2(start.dot(end));
        let turn = turned + TAU * ((drawn.turn - turned) / TAU).round();
        if (turn - drawn.turn).abs() * radius > 4.0 * (tolerance + moved) {
            return None;
        }
        let arc = Arc {
            center,
            radius,
            start: start.y.atan2(start.x),
            turn,
            room: tolerance / radius,
        };
        let crossed = runs
            .iter()
            .filter(|run| straight(run))
            .any(|run| arc.crosses(laid[run.0], laid[run.1]));
        if arc.pinches(tolerance) || crossed {
            return None;
        }
        let near = laid.iter().enumerate().any(|(corner, &at)| {
            corner != from
                && corner != to
                && at != laid[from]
                && at != laid[to]
                && arc.distance(at) <= tolerance
        });
        if near {
            return None;
        }
        bends[from] = Some(Bend { center, turn });
    }
    Some(bends)
}

/// The centre of a half circle whose end lands tangent to the run leaving
/// it — a groove's half round bottom between two walls, a tube's wall
/// rounded whole between its two shafts — laid at the middle of its two
/// ends, which stand on its level: they are the ends of a diameter, and the
/// arc is tangent at both however the runs' levels were laid, where a
/// centre kept as drawn stands a hair off that middle about an axis
/// leaning a hair (#633). Any other centre stays as laid: a ball's poles
/// land along the axis, where nothing is tangent.
fn diameter(start: DVec2, end: DVec2, center: DVec2, free: &Free) -> DVec2 {
    match free {
        Free::Radius if start != end && start.y == center.y && end.y == center.y => {
            DVec2::new((start.x + end.x) / 2.0, center.y)
        }
        Free::Height if start != end && start.x == center.x && end.x == center.x => {
            DVec2::new(center.x, (start.y + end.y) / 2.0)
        }
        Free::Height | Free::Radius | Free::Both => center,
    }
}

/// The value laid on the level nearest it within `tolerance`, or left as it
/// is.
fn level(value: f64, levels: &[f64], tolerance: f64) -> f64 {
    levels
        .iter()
        .copied()
        .filter(|level| (level - value).abs() <= tolerance)
        .min_by(|one, other| (one - value).abs().total_cmp(&(other - value).abs()))
        .unwrap_or(value)
}

/// Which coordinate of a corner the run leaving it keeps: the height of a
/// square run, the distance of a parallel one or of a corner on the axis,
/// neither for a slant or an arc.
fn free(laid: &[DVec2], next: &Run, straight: bool) -> Free {
    let (from, to) = (laid[next.0], laid[next.1]);
    if from.y == 0.0 || straight && from.y == to.y {
        Free::Height
    } else if straight && from.x == to.x {
        Free::Radius
    } else {
        Free::Both
    }
}

enum Free {
    Height,
    Radius,
    Both,
}

/// Every corner standing at `from` moved to `to`: a corner laid onto the
/// arc's end, the far end of a run laid to no length, goes where the end
/// goes, so that the run stays laid to nothing.
fn slid(laid: &mut [DVec2], from: DVec2, to: DVec2) {
    for corner in laid.iter_mut().filter(|corner| **corner == from) {
        *corner = to;
    }
}

/// The radius at which an arc would end tangent to the level its end stands
/// on, the shaft a ball end joins, where the next run keeps that level:
/// none for a slant, and none on the axis, where an arc ends at a pole
/// rather than tangent to anything.
fn tangent(end: DVec2, center: DVec2, free: &Free) -> Option<f64> {
    match free {
        Free::Height if end.y > 0.0 => Some((end.y - center.y).abs()),
        Free::Radius => Some((end.x - center.x).abs()),
        Free::Height | Free::Both => None,
    }
}

/// The corner moved onto the circle along what it leaves free: the nearer
/// of the two places along a line, the foot of the radius otherwise; none
/// where the line misses the circle. A corner laid on the centre's level,
/// within roundings of the radius from the centre, stays: its line is
/// tangent to the circle there, a groove's wall or a shoulder's shaft, and
/// whether it misses or crosses by an ulp is the rounding of a lean, which
/// moved the corner by the root of it (#633).
fn landed(corner: DVec2, center: DVec2, radius: f64, free: Free) -> Option<DVec2> {
    let from = corner - center;
    let nearer = |fixed: f64, moving: f64| {
        if moving == 0.0 && (radius - fixed.abs()).abs() <= ROUNDINGS * f64::EPSILON * radius {
            return Some(0.0);
        }
        let reach = (radius - fixed) * (radius + fixed);
        if reach < 0.0 {
            return None;
        }
        let half = reach.sqrt();
        Some(if moving >= 0.0 { half } else { -half })
    };
    match free {
        Free::Height => Some(DVec2::new(center.x + nearer(from.y, from.x)?, corner.y)),
        Free::Radius => Some(DVec2::new(corner.x, center.y + nearer(from.x, from.y)?)),
        Free::Both => Some(center + from.normalize() * radius),
    }
}

/// An arc as laid: its circle, where it starts on it, how far it turns, and
/// the tolerance as an angle on it.
struct Arc {
    center: DVec2,
    radius: f64,
    start: f64,
    turn: f64,
    room: f64,
}

impl Arc {
    /// How far round from the start an angle of the circle stands, the way
    /// the arc turns, in `[0, 2π)`.
    fn along(&self, angle: f64) -> f64 {
        ((angle - self.start) * self.turn.signum()).rem_euclid(TAU)
    }

    /// Whether an angle of the circle stands on the arc, its ends included
    /// within the room.
    fn holds(&self, angle: f64) -> bool {
        let along = self.along(angle);
        along <= self.turn.abs() + self.room || along >= TAU - self.room
    }

    /// Whether an angle of the circle stands on the arc away from its ends.
    fn inside(&self, angle: f64) -> bool {
        let along = self.along(angle);
        along > self.room && along < self.turn.abs() - self.room
    }

    /// Whether the arc reaches the axis anywhere but at its ends: an arc
    /// crossing it, or one tangent to it within `tolerance` where the arc
    /// passes the point of tangency.
    fn pinches(&self, tolerance: f64) -> bool {
        let (center, radius) = (self.center.y, self.radius);
        if center.abs() > radius + tolerance {
            return false;
        }
        if center.abs() >= radius - tolerance {
            let tangent = (-center.signum()).asin();
            return self.holds(tangent);
        }
        let low = (-center / radius).asin();
        [low, std::f64::consts::PI - low]
            .into_iter()
            .any(|angle| self.inside(angle))
    }

    /// Whether the arc crosses the run from `from` to `to` away from the
    /// ends of both: where the run's line meets the circle, between the
    /// run's corners and on the arc.
    fn crosses(&self, from: DVec2, to: DVec2) -> bool {
        let along = to - from;
        let length = along.length();
        if length == 0.0 {
            return false;
        }
        let start = from - self.center;
        let half = start.dot(along) / length;
        let reach = (self.radius - start.perp_dot(along) / length)
            * (self.radius + start.perp_dot(along) / length);
        if reach < 0.0 {
            return false;
        }
        let room = self.room * self.radius / length;
        [-half - reach.sqrt(), -half + reach.sqrt()]
            .into_iter()
            .map(|at| at / length)
            .filter(|at| *at > room && *at < 1.0 - room)
            .any(|at| {
                let hit = start + along * at;
                self.inside(hit.y.atan2(hit.x))
            })
    }

    /// How far a point stands from the arc: across the circle where the
    /// point faces the arc, from the nearer end otherwise.
    fn distance(&self, point: DVec2) -> f64 {
        let from = point - self.center;
        if self.holds(from.y.atan2(from.x)) {
            return (from.length() - self.radius).abs();
        }
        let end = self.start + self.turn;
        [self.start, end]
            .into_iter()
            .map(|angle| point.distance(self.center + DVec2::from_angle(angle) * self.radius))
            .fold(f64::INFINITY, f64::min)
    }
}
