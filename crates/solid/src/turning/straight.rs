//! A profile of straight runs and arcs laid against its axis: every run
//! within the tolerance of parallel or square to the axis laid so, exactly,
//! every other slanted as drawn, and every arc about a centre laid like a
//! corner ([`bends`]), ready for the exact kernel to turn.

mod bends;
mod bounded;
mod contacts;
mod levels;
mod slivers;

use std::ops::Range;

use glam::DVec2;

use super::Turn;
use crate::brep::Scale;
use crate::profile::{Contour, Frame, Run};
pub use bends::Bend;

/// A profile whose every run is parallel, square or slanted to its axis,
/// read along the axis and away from it.
#[derive(Clone, Debug, PartialEq)]
pub struct Straight {
    /// `+1.0` or `-1.0`: a corner's distance from the axis, signed as
    /// [`super::Axis::side`] reads it, is `side` times its distance away.
    pub side: f64,
    /// The outline, then each hole in the profile's order, as the corners
    /// they run through; a run laid to no length is left out, and so is a
    /// hole laid to no area. None at all when the profile is laid to no area:
    /// turned, it is nothing.
    pub contours: Vec<Vec<Corner>>,
    /// How many runs the profile has, the outline's first and then each
    /// hole's: what its faces are numbered by.
    pub runs: u32,
    /// The highest number of a run not lying on the axis.
    pub last_off_the_axis: Option<u32>,
}

/// A corner of a profile laid against its axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corner {
    /// How far along the axis, and how far away from it, never below nought.
    pub at: DVec2,
    /// The number of the run leaving the corner.
    pub run: u32,
    /// The run leaving the corner where it is an arc, as laid (#576).
    pub bend: Option<Bend>,
}

impl Straight {
    /// The profile laid against the axis of `turn`, or `None` when a run is
    /// an arc of ellipse, or the profile cannot be laid as the exact kernel
    /// turns it: the flats turn it then.
    ///
    /// A corner within the turn's band of the axis is laid on it. Every run
    /// within the tolerance of parallel or square to the axis is laid so,
    /// exactly, and levels closer than the tolerance anywhere in the profile
    /// are one; a run leaning further slants between the levels of its two
    /// corners, its angle never snapped. The tolerance is the drawing's
    /// resolution, never finer than the kernel can tell apart over the
    /// profile turned and `part_reach`. A profile is not straight when laying
    /// it would move a corner by more than twice the tolerance. Where laying
    /// makes it touch itself where it does not, a wall or a gap thinner than
    /// the tolerance is not there: the profile is the matter it bounds as
    /// laid, nothing when it bounds none, declined when that is not one
    /// piece or touches itself at a corner. Nor is a sliver between a slanted
    /// run and the run bringing its end back to within the tolerance of it
    /// there: the slant ends where the run back does. Any other corner within
    /// the tolerance of a slanted run it does not end, a run across a slant,
    /// or a corner on the axis between two runs that both leave it is not
    /// straight. A run laid to no length keeps its number and names no face.
    ///
    /// An arc is laid about a centre laid like a corner ([`bends`]), joins
    /// no level, and is not re-read column by column: a profile whose arcs
    /// cannot be laid so goes to the flats.
    pub fn of(
        outline: &Contour,
        holes: &[Contour],
        frame: Frame,
        turn: &Turn,
        part_reach: f64,
    ) -> Option<Straight> {
        let contours: Vec<&Contour> = std::iter::once(outline).chain(holes).collect();
        if contours.iter().any(|contour| {
            contour
                .runs
                .iter()
                .any(|run| matches!(run, Run::Oval { .. }))
        }) {
            return None;
        }
        let (side, read, raw) = read(&contours, turn)?;
        let arcs = arcs(&contours, turn, side);
        let bent: Vec<bool> = arcs.iter().map(Option::is_some).collect();
        let rings = rings(&contours);
        let runs: Vec<(usize, usize)> = rings
            .iter()
            .flat_map(|ring| ring.clone().map(|corner| (corner, next(ring, corner))))
            .collect();
        let straight_runs: Vec<(usize, usize)> = runs
            .iter()
            .copied()
            .filter(|&(from, _)| !bent[from])
            .collect();
        let scale = scale(&contours, frame, &read, part_reach);
        let tolerance = turn.resolution.max(Scale::HAIR * scale.eps());
        let mut laid = levels::laid(&read, &straight_runs, tolerance)?;
        slivers::cut_short(&mut laid, &rings, tolerance, &bent);
        let bends = bends::laid(&mut laid, &raw, &arcs, &runs, turn.on_the_axis, tolerance)?;

        let mut kept: Vec<Vec<usize>> = rings
            .iter()
            .map(|ring| {
                ring.clone()
                    .filter(|&corner| bent[corner] || laid[corner] != laid[next(ring, corner)])
                    .collect()
            })
            .collect();
        let bounds =
            |corners: &Vec<usize>| corners.len() >= 3 || corners.iter().any(|&corner| bent[corner]);
        if !bounds(&kept[0]) {
            kept.clear();
        }
        kept.retain(bounds);
        let laid_runs: Vec<Vec<contacts::Run>> = kept
            .iter()
            .map(|corners| {
                (0..corners.len())
                    .map(|index| (corners[index], corners[(index + 1) % corners.len()]))
                    .collect()
            })
            .collect();
        let drawn = contacts::are_drawn(&laid, &read, &laid_runs, &bent, scale.eps(), tolerance);
        let contours = if drawn {
            kept.iter()
                .map(|corners| {
                    corners
                        .iter()
                        .map(|&corner| Corner {
                            at: laid[corner],
                            run: corner as u32,
                            bend: bends[corner],
                        })
                        .collect()
                })
                .collect()
        } else if bent.iter().any(|bent| *bent) {
            return None;
        } else {
            bounded::contours(&laid, &laid_runs, tolerance)?
        };
        if contacts::pinched(&contours) {
            return None;
        }

        let last_off_the_axis = runs
            .iter()
            .filter(|&&(from, to)| bent[from] || laid[from].y != 0.0 || laid[to].y != 0.0)
            .map(|&(from, _)| from as u32)
            .max();
        Some(Straight {
            side,
            contours,
            runs: read.len() as u32,
            last_off_the_axis,
        })
    }

    /// How many numbers a turn of the profile as laid names: one per run,
    /// and the two ends of a partial turn. A whole turn counts up to its last
    /// run off the axis, as the flats always have. The body counts a whole
    /// turn on the profile as drawn rather than as laid when it can, since a
    /// run within the band was counted before turns were exact.
    pub fn numbers(&self, whole: bool) -> u32 {
        if whole {
            self.last_off_the_axis.map_or(0, |run| run + 1)
        } else {
            self.runs + 2
        }
    }
}

/// Every corner read as `(along, away)` from the axis, those within the
/// band laid on it, and as drawn; and the side the profile stands on, read
/// off the corners beyond the band, or off the arcs' middles where every
/// corner stands on the axis, as a ball's do. `None` when they stand on
/// both sides, or none stands off the axis.
fn read(contours: &[&Contour], turn: &Turn) -> Option<(f64, Vec<DVec2>, Vec<DVec2>)> {
    let along = turn.axis.direction.normalize_or_zero();
    let corners = contours
        .iter()
        .flat_map(|contour| contour.corners.iter().copied());
    let read: Vec<(f64, f64)> = corners
        .map(|corner| (along.dot(corner - turn.axis.origin), turn.axis.side(corner)))
        .collect();
    let beyond = |side: f64| side.abs() > turn.on_the_axis;
    let bulges: Vec<f64> = contours
        .iter()
        .flat_map(|contour| contour.corners.iter().zip(&contour.runs))
        .filter_map(|(&corner, run)| match *run {
            Run::Round {
                center,
                turn: swept,
            } => Some(center + DVec2::from_angle(swept / 2.0).rotate(corner - center)),
            Run::Straight | Run::Oval { .. } => None,
        })
        .map(|middle| turn.axis.side(middle))
        .collect();
    let sides = read.iter().map(|(_, side)| *side).chain(bulges);
    let sign = sides.clone().find(|side| beyond(*side))?.signum();
    if sides
        .filter(|side| beyond(*side))
        .any(|side| side * sign < 0.0)
    {
        return None;
    }
    let raw = read
        .iter()
        .map(|&(along, side)| DVec2::new(along, side * sign))
        .collect();
    let read = read
        .into_iter()
        .map(|(along, side)| DVec2::new(along, if beyond(side) { side * sign } else { 0.0 }))
        .collect();
    Some((sign, read, raw))
}

/// Every run's arc, by the corner it leaves, read as the corners are: its
/// centre as `(along, away)`, away signed with `side` so that it is negative
/// across the axis, and its turn signed as it goes in `(h, r)`.
fn arcs(contours: &[&Contour], turn: &Turn, side: f64) -> Vec<Option<bends::Drawn>> {
    let along = turn.axis.direction.normalize_or_zero();
    contours
        .iter()
        .flat_map(|contour| contour.runs.iter())
        .map(|run| match *run {
            Run::Round {
                center,
                turn: swept,
            } => Some(bends::Drawn {
                center: DVec2::new(
                    along.dot(center - turn.axis.origin),
                    turn.axis.side(center) * side,
                ),
                turn: swept * side,
            }),
            Run::Straight | Run::Oval { .. } => None,
        })
        .collect()
}

/// The corners of each contour, numbered through the profile.
fn rings(contours: &[&Contour]) -> Vec<Range<usize>> {
    let mut start = 0;
    contours
        .iter()
        .map(|contour| {
            let ring = start..start + contour.corners.len();
            start = ring.end;
            ring
        })
        .collect()
}

fn next(ring: &Range<usize>, corner: usize) -> usize {
    if corner + 1 == ring.end {
        ring.start
    } else {
        corner + 1
    }
}

fn previous(ring: &Range<usize>, corner: usize) -> usize {
    if corner == ring.start {
        ring.end - 1
    } else {
        corner - 1
    }
}

/// The kernel's scale over what the turn can reach, the part's included: no
/// corner of the profile stands further from the origin than its place in
/// the frame and its distance from the axis.
fn scale(contours: &[&Contour], frame: Frame, read: &[DVec2], part_reach: f64) -> Scale {
    let furthest = read.iter().map(|corner| corner.y).fold(0.0, f64::max);
    let placed = contours
        .iter()
        .flat_map(|contour| contour.corners.iter())
        .map(|&corner| frame.at(corner).abs().max_element())
        .fold(0.0, f64::max);
    Scale::of(part_reach.max(placed + furthest))
}

#[cfg(test)]
mod tests;
