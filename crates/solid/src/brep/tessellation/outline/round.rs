//! A face of a sphere or a torus laid out in `(θ, φ)` (#576), as a cone's is
//! in `(θ, l)`, and cut in strips between the rows of its grid.
//!
//! Every edge of such a face is a latitude circle, at one angle along the
//! meridian, or a meridian arc, at one angle about the axis: the face is a
//! region bounded by lines of the chart. Each sample is laid where its edge
//! stands — a latitude circle's at the circle's own angle along the meridian,
//! a meridian's at the arc's own angle about the axis and on the row of the
//! grid it was sampled at — so that the boundaries are exactly square to the
//! chart, and the strips between the rows ([`super::strips`]) are columns.
//! The poles are laid as a cone's apex is: a loop passing through one runs
//! along its floor from the angle it comes in at to the angle it leaves at,
//! and a face holding one within it takes the whole floor as a loop of its
//! own, the other way round from its rims. A face of a ring winding round
//! the tube between two whole meridians, and a whole ring with no loop at
//! all, are closed along the cut of the chart; a whole ball along its two
//! floors; the walls closing a face cut open at an angle are the cone's.

mod floors;

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::super::sampling::{ON_A_ROW, Rounded, Samples, levels, midway};
use super::Outline;
use super::cone::Laid;
use super::cylinder::CLEAR;
use super::strips::Lattice;
use crate::brep::curve::Curve;
use crate::brep::topology::{Body, Face};
use floors::Floors;

/// A whole meridian circle bounding a face winding round the tube: the angle
/// about the axis it stands at, whether the face's loop runs the angle along
/// the meridian up along it, and its samples at their angles along the
/// meridian, the cut of the chart first and last.
type Meridian = (f64, bool, Vec<(f64, usize)>);

/// Where a sample stands in the chart, as its edges put it: the angle about
/// the axis of a meridian through it, the angle along the meridian of a
/// latitude circle through it.
#[derive(Clone, Copy, Default)]
struct Standing {
    theta: Option<f64>,
    phi: Option<f64>,
}

impl Outline {
    /// Lays out and cuts in strips the loops `laps` of `face`, on `round`,
    /// whose grid has `steps` angles a turn and `rows` rows a turn of the
    /// meridian. None when an edge is neither a latitude circle nor a
    /// meridian, when a loop cannot be read, or when the strips cannot be
    /// cut.
    pub(super) fn rounded(
        &mut self,
        body: &Body,
        samples: &mut Samples,
        face: &Face,
        round: &Rounded,
        laps: &[Vec<usize>],
        (steps, rows): (usize, usize),
    ) -> Option<()> {
        let eps = body.scale().eps();
        let levels = levels(body, face, round, eps);
        let mut lattice = Lattice::new(*round, steps, rows, &levels, eps / round.radius());
        let mut standing: BTreeMap<usize, Standing> = BTreeMap::new();
        let mut winding = Vec::new();
        for (rank, lap) in face.loops.iter().enumerate() {
            for coedge in lap {
                let edge = body.edge(coedge.edge);
                let Curve::Circle(circle) = body.curve(edge.curve) else {
                    return None;
                };
                if round.holds(circle, eps) {
                    let phi = round.latitude(circle);
                    for &id in samples.edge(coedge.edge) {
                        standing.entry(id).or_default().phi.get_or_insert(phi);
                    }
                } else if round.meridian(circle, eps) {
                    let (theta, _, _) = round.along_meridian(circle, midway(edge));
                    for &id in samples.edge(coedge.edge) {
                        standing.entry(id).or_default().theta.get_or_insert(theta);
                    }
                    if edge.ends.is_none() && lap.len() == 1 {
                        winding.push(rank);
                    }
                } else {
                    return None;
                }
            }
        }
        if !winding.is_empty() {
            if winding.len() != 2 || laps.len() != 2 {
                return None;
            }
            self.wound(samples, laps, &standing, &lattice)?;
        } else if laps.is_empty() && round.poles(eps).is_empty() {
            self.whole_ring(samples, round, &mut lattice)?;
        } else {
            self.laid_round(samples, face, laps, &standing, &lattice, eps)?;
        }
        self.stripped(samples, &mut lattice)
    }

    /// A face with loops through vertices laid out as a cone's face is, each
    /// sample on the line of its edge, the poles along their floors.
    fn laid_round(
        &mut self,
        samples: &Samples,
        face: &Face,
        laps: &[Vec<usize>],
        standing: &BTreeMap<usize, Standing>,
        lattice: &Lattice,
        eps: f64,
    ) -> Option<()> {
        let (round, steps, rows) = (&lattice.round, lattice.steps, lattice.rows);
        let mut read: BTreeMap<usize, DVec2> = BTreeMap::new();
        for lap in laps {
            let mut last: Option<f64> = None;
            for &id in lap {
                let raw = round.parameters(samples.point(id));
                let stands = standing.get(&id).copied().unwrap_or_default();
                let theta = match stands.theta {
                    Some(theta) => theta,
                    None => {
                        let rank = (raw.x / (TAU / steps as f64)).round() as i64;
                        let on_grid = TAU * rank as f64 / steps as f64;
                        if (raw.x - on_grid).abs() <= CLEAR && last != Some(on_grid) {
                            on_grid
                        } else {
                            raw.x
                        }
                    }
                };
                let phi = stands.phi.unwrap_or_else(|| on_a_row(raw.y, rows));
                last = Some(theta);
                read.insert(id, DVec2::new(theta, phi));
            }
        }
        let floors = Floors {
            samples,
            poles: round.poles(eps),
            read,
            eps,
            steps,
        };
        let mut laid: Vec<Laid> = laps
            .iter()
            .map(|lap| floors.laid(lap))
            .collect::<Option<_>>()?;
        let turns: i64 = laid.iter().map(|lap| lap.turns).sum();
        let held: Vec<(usize, usize)> = face
            .poles
            .iter()
            .map(|vertex| {
                let id = vertex.0 as usize;
                floors.tip(id).map(|pole| (pole, id))
            })
            .collect::<Option<_>>()?;
        match (held.is_empty(), turns) {
            (true, _) => {}
            (false, 1 | -1) => {
                for &(pole, id) in &held {
                    laid.push(floors.round(-turns, pole, id));
                }
            }
            (false, 0) => {
                for &(pole, id) in &held {
                    let way = if floors.poles[pole].high { -1 } else { 1 };
                    laid.push(floors.round(way, pole, id));
                }
            }
            (false, _) => return None,
        }
        let (raws, ids): (Vec<Vec<DVec2>>, Vec<Vec<usize>>) =
            laid.into_iter().map(|lap| (lap.raw, lap.ids)).unzip();
        self.round_laid(&raws, &ids, steps, |id| {
            !samples.is_vertex(id) || floors.tip(id).is_some()
        })
    }

    /// A face of a ring winding round the tube between two whole meridians,
    /// one running the angle along the meridian up and one down: the one
    /// running up is the wall on the right, since the face lies on its left,
    /// and the face is closed along the cut of the chart at the top and the
    /// bottom of the window, on lattice points of the cut the strips add.
    fn wound(
        &mut self,
        samples: &Samples,
        laps: &[Vec<usize>],
        standing: &BTreeMap<usize, Standing>,
        lattice: &Lattice,
    ) -> Option<()> {
        let (round, rows, room) = (&lattice.round, lattice.rows, lattice.room);
        let top = round.cut()?;
        let bottom = top - TAU;
        let mut walls: Vec<Meridian> = Vec::new();
        for lap in laps {
            let theta = standing.get(&lap[0])?.theta?;
            let mut along: Vec<(f64, usize)> = Vec::new();
            let mut cut = None;
            for &id in lap {
                let phi = round.parameters(samples.point(id)).y;
                if ((phi - top + PI).rem_euclid(TAU) - PI).abs() <= room {
                    cut = Some(id);
                } else {
                    along.push((on_a_row(phi, rows), id));
                }
            }
            let cut = cut?;
            let rising = along
                .windows(2)
                .filter(|pair| pair[1].0 > pair[0].0)
                .count();
            let up = 2 * rising + 1 > along.len();
            along.sort_by(|one, other| one.0.total_cmp(&other.0));
            along.insert(0, (bottom, cut));
            along.push((top, cut));
            walls.push((theta, up, along));
        }
        let [one, other] = &walls[..] else {
            return None;
        };
        if one.1 == other.1 {
            return None;
        }
        let (right, left) = if one.1 { (one, other) } else { (other, one) };
        let (right_theta, left_theta) = (right.0, right.0 - (right.0 - left.0).rem_euclid(TAU));
        if right_theta - left_theta <= CLEAR {
            return None;
        }
        let downward: Vec<(DVec2, usize)> = left
            .2
            .iter()
            .rev()
            .map(|&(phi, id)| (DVec2::new(left_theta, phi), id))
            .collect();
        let upward: Vec<(DVec2, usize)> = right
            .2
            .iter()
            .map(|&(phi, id)| (DVec2::new(right_theta, phi), id))
            .collect();
        self.chain(&downward);
        self.chain(&upward);
        Some(())
    }

    /// A whole ring, one face with no loop and no vertex: the whole window of
    /// the chart, walled at the angle nought and a turn on along lattice
    /// points, and closed along the cut of the chart by the strips.
    fn whole_ring(
        &mut self,
        samples: &mut Samples,
        round: &Rounded,
        lattice: &mut Lattice,
    ) -> Option<()> {
        let top = round.cut()?;
        let bottom = top - TAU;
        let (low, high) = (
            lattice.sample(samples, 0, bottom),
            lattice.sample(samples, 0, top),
        );
        for (theta, way) in [(0.0, -1.0), (TAU, 1.0)] {
            let wall = [
                (DVec2::new(theta, top), high),
                (DVec2::new(theta, bottom), low),
            ];
            let placed = if way < 0.0 { wall } else { [wall[1], wall[0]] };
            self.chain(&placed);
            self.walls.push(self.segments.len() - 1);
        }
        Some(())
    }
}

/// The row of a grid of `rows` rows a turn `phi` stands on, where it stands
/// within [`ON_A_ROW`] of one; `phi` itself otherwise.
fn on_a_row(phi: f64, rows: usize) -> f64 {
    let rank = (phi / (TAU / rows as f64)).round() as i64;
    let row = TAU * rank as f64 / rows as f64;
    if (phi - row).abs() <= ON_A_ROW {
        row
    } else {
        phi
    }
}
