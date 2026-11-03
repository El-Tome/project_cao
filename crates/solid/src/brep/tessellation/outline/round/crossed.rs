//! A face of a ring whose loops run through the cut of the chart along
//! meridians that are not whole loops of their own (#623): a meridian a rim
//! parts, a strip a rim slits across, a ring turned part of the way and
//! notched by a coaxial turn of another extent.
//!
//! Laid as two whole meridians are ([`Outline::wound`]): each loop is broken
//! at every sample where it crosses the cut, the piece reaching the cut
//! ending at the window's top or bottom, the next starting at the other, and
//! the strips close the face along the cut on lattice points of it, shared
//! by the bottom of the window and its top. A loop that never reaches the
//! cut is laid closed. A face reaching round the axis — a whole ring with
//! holes cut in its tube, one of them through the cut or none, a band the
//! cut runs through — is walled as a whole ring is, at an angle of the grid
//! no loop reaches and a turn on, and a latitude circle winding round the
//! axis runs across it from wall to wall; the walls stand only where the
//! face does, from the top of the window down to the first such circle, from
//! the next to the one after, and on to the bottom.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::super::Outline;
use super::super::cylinder::CLEAR;
use super::super::strips::Lattice;
use super::{Standing, on_a_level};
use crate::brep::tessellation::sampling::{Rounded, Samples};

/// A sample as a loop lays it: where it stands, and whether it is a sample
/// of a meridian at the cut.
type Read = (DVec2, usize, bool);

impl Outline {
    /// Lays out `laps` as the module says. None where a sample at the cut
    /// stands on no meridian or also on a latitude circle, where a loop runs
    /// round the axis on a face that is not walled, or where it reaches the
    /// cut and turns back.
    pub(super) fn crossed(
        &mut self,
        samples: &mut Samples,
        laps: &[Vec<usize>],
        standing: &BTreeMap<usize, Standing>,
        lattice: &mut Lattice,
    ) -> Option<()> {
        let (round, steps, room) = (lattice.round, lattice.steps, lattice.room);
        let top = round.cut()?;
        let bottom = top - TAU;
        let middle = top - PI;
        let mut read: Vec<Vec<Read>> = Vec::with_capacity(laps.len());
        for lap in laps {
            let mut laid = Vec::with_capacity(lap.len());
            for &id in lap {
                let raw = round.parameters(samples.point(id));
                let stands = standing.get(&id).copied().unwrap_or_default();
                let at_cut = ((raw.y - top + PI).rem_euclid(TAU) - PI).abs() <= room;
                if at_cut && (stands.theta.is_none() || stands.phi.is_some()) {
                    return None;
                }
                let theta = stands.theta.unwrap_or_else(|| {
                    let rank = (raw.x / (TAU / steps as f64)).round() as i64;
                    let on_grid = TAU * rank as f64 / steps as f64;
                    if (raw.x - on_grid).abs() <= CLEAR {
                        on_grid
                    } else {
                        raw.x
                    }
                });
                let phi = if at_cut {
                    top
                } else {
                    stands.phi.unwrap_or_else(|| on_a_level(raw.y, lattice))
                };
                laid.push((DVec2::new(theta, phi), id, at_cut));
            }
            read.push(laid);
        }
        let meridians: Vec<f64> = laps
            .iter()
            .flatten()
            .filter_map(|id| standing.get(id).and_then(|stands| stands.theta))
            .collect();
        let rim = |lap: &[Read]| {
            !lap.iter().any(|(_, _, at_cut)| *at_cut)
                && turned(&lap.iter().map(|(at, id, _)| (*at, *id)).collect::<Vec<_>>()) != 0
        };
        let angles = || {
            read.iter()
                .filter(|lap| !rim(lap))
                .flatten()
                .map(|(at, _, _)| at.x)
        };
        let mut crossings: Vec<(f64, bool)> = Vec::new();
        for lap in &read {
            let count = lap.len();
            for at in 0..count {
                let (place, _, at_cut) = lap[at];
                if at_cut {
                    let before = lap[(at + count - 1) % count].0.y;
                    crossings.push((place.x.rem_euclid(TAU), before <= middle));
                }
            }
        }
        let walls = |start: f64| crossings.is_empty() || reaches(&crossings, start.rem_euclid(TAU));
        let (rank, start) = opening(angles(), steps)
            .or_else(|| {
                between_meridians(angles(), &meridians, steps).filter(|(_, at)| !walls(*at))
            })
            .or_else(|| opening(meridians.iter().copied(), steps))
            .unwrap_or((0, 0.0));
        for lap in &mut read {
            for (at, _, _) in lap.iter_mut() {
                if at.x < start {
                    at.x += TAU;
                }
            }
        }
        let walled = walls(start);
        let mut chains: Vec<Vec<(DVec2, usize)>> = Vec::new();
        let mut closed: Vec<Vec<(DVec2, usize)>> = Vec::new();
        let mut rims: Vec<(f64, usize)> = Vec::new();
        for lap in &read {
            let count = lap.len();
            let Some(first) = lap.iter().position(|(_, _, at_cut)| *at_cut) else {
                let placed: Vec<(DVec2, usize)> =
                    lap.iter().map(|(at, id, _)| (*at, *id)).collect();
                if turned(&placed) != 0 {
                    if !walled {
                        return None;
                    }
                    let chain = across(&placed, rank, start, steps)?;
                    rims.push((chain[0].0.y, chain[0].1));
                    chains.push(chain);
                } else if runs_round(&placed, true) {
                    return None;
                } else {
                    closed.push(placed);
                }
                continue;
            };
            let mut piece: Vec<(DVec2, usize)> = Vec::new();
            for step in 0..=count {
                let (at, id, at_cut) = lap[(first + step) % count];
                if !at_cut {
                    piece.push((at, id));
                    continue;
                }
                let before = lap[(first + step + count - 1) % count].0.y;
                let after = lap[(first + step + 1) % count].0.y;
                let (arriving, leaving) = (before > middle, after > middle);
                if arriving == leaving {
                    return None;
                }
                if step > 0 {
                    piece.push((DVec2::new(at.x, if arriving { top } else { bottom }), id));
                    if runs_round(&piece, false) {
                        return None;
                    }
                    chains.push(std::mem::take(&mut piece));
                }
                if step < count {
                    piece.push((DVec2::new(at.x, if leaving { top } else { bottom }), id));
                }
            }
        }
        if walled {
            let place = lattice.sample(samples, rank, top);
            rims.sort_by(|one, other| other.0.total_cmp(&one.0));
            let mut stops = vec![(top, place)];
            stops.extend(rims);
            stops.push((bottom, place));
            if !stops.len().is_multiple_of(2) {
                return None;
            }
            for pair in stops.chunks(2) {
                let [(high, upper), (low, lower)] = pair else {
                    return None;
                };
                for (theta, down) in [(start, true), (start + TAU, false)] {
                    let wall = [
                        (DVec2::new(theta, *high), *upper),
                        (DVec2::new(theta, *low), *lower),
                    ];
                    self.chain(&if down { wall } else { [wall[1], wall[0]] });
                    self.walls.push(self.segments.len() - 1);
                }
            }
        }
        for chain in &chains {
            self.chain(chain);
        }
        for lap in &closed {
            self.close(lap);
        }
        Some(())
    }
}

/// The angle about the axis the face is laid from, and its rank on the grid
/// of `steps` angles a turn: the grid's angle nearest the middle of the
/// widest gap between the angles its samples stand at, read in `(−π, π]`,
/// so that no loop is laid across it, the angles below it moved a turn on.
/// None where no angle of the grid stands clear inside the gap — a band
/// whose latitude circles are sampled at every angle of the grid, which is
/// then walled between its meridians, or at nought where it has none.
fn opening(angles: impl Iterator<Item = f64>, steps: usize) -> Option<(i64, f64)> {
    let (gap, from) = widest(angles)?;
    let step = TAU / steps as f64;
    let rank = ((from + gap / 2.0) / step).round() as i64;
    let at = step * rank as f64;
    if at <= from + CLEAR || at >= from + gap - CLEAR {
        return None;
    }
    let rank = halved(rank, steps);
    Some((rank, TAU * rank as f64 / steps as f64))
}

/// The middle of the one gap between neighbouring angles a face's samples
/// stand at whose two sides are meridians of the face: a ring turned a
/// tenth of a degree short of a whole turn leaves a gap narrower than a
/// step of the grid, which no angle of the grid stands clear inside, and
/// was laid from the widest gap between its two meridians instead — the
/// other way round, through the face (5585602707). Its rank is the grid's
/// angle nearest it, which a face walled there would be walled at instead,
/// so the gap is taken only where the face is not walled. None where no
/// such gap, or more than one, is found.
fn between_meridians(
    angles: impl Iterator<Item = f64>,
    meridians: &[f64],
    steps: usize,
) -> Option<(i64, f64)> {
    let mut angles: Vec<f64> = angles.map(|angle| angle.rem_euclid(TAU)).collect();
    angles.sort_by(f64::total_cmp);
    angles.dedup();
    let on_a_meridian = |angle: f64| {
        meridians
            .iter()
            .any(|meridian| ((meridian - angle + PI).rem_euclid(TAU) - PI).abs() <= CLEAR)
    };
    let count = angles.len();
    let mut gaps = (0..count).filter_map(|rank| {
        let (from, to) = (angles[rank], angles[(rank + 1) % count]);
        let to = if rank + 1 == count { to + TAU } else { to };
        (to - from > CLEAR && on_a_meridian(from) && on_a_meridian(to)).then_some((from, to))
    });
    let (from, to) = gaps.next()?;
    if gaps.next().is_some() {
        return None;
    }
    let middle = ((from + to) / 2.0 + PI).rem_euclid(TAU) - PI;
    let rank = halved((middle / (TAU / steps as f64)).round() as i64, steps);
    Some((rank, middle))
}

/// The widest gap between `angles` round the axis, and the angle it starts
/// at, both in `[0, 2π)`.
fn widest(angles: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let mut angles: Vec<f64> = angles.map(|angle| angle.rem_euclid(TAU)).collect();
    angles.sort_by(f64::total_cmp);
    angles.dedup();
    let first = *angles.first()?;
    let mut widest = (
        first + TAU - angles[angles.len() - 1],
        angles[angles.len() - 1],
    );
    for pair in angles.windows(2) {
        if pair[1] - pair[0] > widest.0 {
            widest = (pair[1] - pair[0], pair[0]);
        }
    }
    Some(widest)
}

/// A rank of the grid of `steps` angles a turn, read in `(−steps/2, steps/2]`.
fn halved(rank: i64, steps: usize) -> i64 {
    let rank = rank.rem_euclid(steps as i64);
    if 2 * rank > steps as i64 {
        rank - steps as i64
    } else {
        rank
    }
}

/// Whether the face reaches the cut at the angle `at`, in `[0, 2π)`: where
/// the last crossing of the cut before it, going round the axis the way the
/// angle grows, starts the face — a loop running the angle along the
/// meridian down through the cut keeps the face on the side the angle grows
/// towards.
fn reaches(crossings: &[(f64, bool)], at: f64) -> bool {
    crossings
        .iter()
        .max_by(|one, other| {
            let behind = |angle: f64| (at - angle).rem_euclid(TAU);
            behind(other.0).total_cmp(&behind(one.0))
        })
        .is_some_and(|(_, starts)| *starts)
}

/// How many times a closed loop of points turns round the axis.
fn turned(placed: &[(DVec2, usize)]) -> i64 {
    let count = placed.len();
    let turned: f64 = (0..count)
        .map(|rank| {
            let gap = placed[(rank + 1) % count].0.x - placed[rank].0.x;
            (gap + PI).rem_euclid(TAU) - PI
        })
        .sum();
    (turned / TAU).round() as i64
}

/// A loop winding round the axis — a latitude circle bounding a face walled
/// at `start` — laid as one chain across the face from wall to wall, the
/// way it runs: from its sample on the grid's angle `rank` at the one wall
/// to the same sample at the other. None where it has no sample there.
fn across(
    placed: &[(DVec2, usize)],
    rank: i64,
    start: f64,
    steps: usize,
) -> Option<Vec<(DVec2, usize)>> {
    let step = TAU / steps as f64;
    let wall = rank.rem_euclid(steps as i64);
    let count = placed.len();
    let at = placed.iter().position(|(point, _)| {
        ((point.x / step).round() as i64).rem_euclid(steps as i64) == wall
    })?;
    let (phi, id) = (placed[at].0.y, placed[at].1);
    let forward = turned(placed) > 0;
    let (from, to) = if forward {
        (start, start + TAU)
    } else {
        (start + TAU, start)
    };
    let mut chain = vec![(DVec2::new(from, phi), id)];
    chain.extend((1..count).map(|offset| placed[(at + offset) % count]));
    chain.push((DVec2::new(to, phi), id));
    Some(chain)
}

/// Whether a face of a ring holds the whole cut of the chart, its region on
/// the left of its loops: it lies above the highest latitude circle bounding
/// it, where that circle runs the angle about the axis up, and above a rim
/// it touches along there, which runs both ways — a whole ring with holes cut
/// in its tube, a band the cut runs through. The rule the domain reads the
/// face by (`domain/floor/round/ring.rs`), read on the samples.
pub(super) fn holds_the_cut(
    samples: &Samples,
    round: &Rounded,
    laps: &[Vec<usize>],
    standing: &BTreeMap<usize, Standing>,
) -> bool {
    let phi = |id: &usize| standing.get(id).and_then(|stands| stands.phi);
    let mut latitudes: Vec<(f64, bool)> = Vec::new();
    for lap in laps {
        let count = lap.len();
        for at in 0..count {
            let (one, other) = (lap[at], lap[(at + 1) % count]);
            let (Some(level), Some(next)) = (phi(&one), phi(&other)) else {
                continue;
            };
            if level == next {
                let gap = round.parameters(samples.point(other)).x
                    - round.parameters(samples.point(one)).x;
                latitudes.push((level, (gap + PI).rem_euclid(TAU) - PI > 0.0));
            }
        }
    }
    let highest = latitudes
        .iter()
        .map(|(level, _)| *level)
        .fold(f64::NEG_INFINITY, f64::max);
    latitudes.iter().any(|&(level, up)| level == highest && up)
}

/// Whether two consecutive points of `placed`, closed back on its start
/// where `closed`, stand half a turn or more apart about the axis: the loop
/// runs round it.
fn runs_round(placed: &[(DVec2, usize)], closed: bool) -> bool {
    let count = placed.len();
    let pairs = if closed {
        count
    } else {
        count.saturating_sub(1)
    };
    (0..pairs).any(|rank| (placed[(rank + 1) % count].0.x - placed[rank].0.x).abs() >= PI)
}
