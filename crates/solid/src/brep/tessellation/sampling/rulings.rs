//! Two edges of a cone's face standing within twice the tolerance of each
//! other, sampled on the same rulings of the cone (#810).
//!
//! Each edge is sampled on its own angles: the cone's grid, and the grid,
//! rays and chord splits of the wall on its other side. Two walls a hair
//! apart, or a face tangent to a bore, leave two edges of the cone's face a
//! hair apart with places a hair apart, and a place one has and the other
//! lacks can stand beyond the other's chord: in the cone's `(θ, l)` the two
//! chords cross, and the face is left uncut. Such an edge takes the place
//! where the ruling through that one crosses it, within twice the tolerance
//! of it: both then stand on that ruling in the order of their curves, and
//! two chords that keep their order on every ruling either ends on keep it
//! between. An edge keeps every place it had, so the face on its other side
//! loses none; a place within the kernel's tolerance of one it has, or
//! within a hair of one of its ends, is that one, and a place within
//! rounding of a vertex, or of the place it was taken from, is that sample,
//! as `Samples::through` has it.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::{Samples, TOLD};
use crate::brep::Scale;
use crate::brep::curve::Curve;
use crate::brep::surface::{Cone, Surface};
use crate::brep::topology::{Body, EdgeId};

/// How many times the edges are read again: a place taken bends the chord
/// it falls on, which another place of the other edge may then stand beyond.
const ROUNDS: usize = 8;

/// A place an edge takes: the rank of the chord it falls on, how far along
/// it, the point, and the sample of the other edge it was taken from.
type Place = (usize, f64, DVec3, usize);

/// An edge of a cone's face: its samples in the cone's `(θ, l)`, and the
/// box they stand in.
struct Charted {
    edge: EdgeId,
    chart: Vec<DVec2>,
    low: DVec3,
    high: DVec3,
}

impl Samples {
    /// Every two edges of a cone's face given a place on each ruling through
    /// a place of the other that stands beyond its chord, within twice
    /// `tolerance` of it.
    pub(super) fn on_common_rulings(&mut self, body: &Body, tolerance: f64) {
        let mut faces: Vec<(Cone, Vec<EdgeId>)> = Vec::new();
        for face in body.face_ids() {
            let face = body.face(face);
            let Surface::Cone(cone) = body.surface(face.surface) else {
                continue;
            };
            let mut edges: Vec<EdgeId> = face
                .loops
                .iter()
                .flatten()
                .map(|coedge| coedge.edge)
                .filter(|edge| !matches!(body.curve(body.edge(*edge).curve), Curve::Line(_)))
                .collect();
            edges.sort();
            edges.dedup();
            if edges.len() > 1 {
                faces.push((*cone, edges));
            }
        }
        let reach = 2.0 * tolerance;
        for _ in 0..ROUNDS {
            let mut taken: BTreeMap<EdgeId, Vec<Place>> = BTreeMap::new();
            for (cone, edges) in &faces {
                let charted: Vec<Charted> =
                    edges.iter().map(|edge| self.charted(*edge, cone)).collect();
                for to in &charted {
                    let (low, high) = (to.low - reach - tolerance, to.high + reach + tolerance);
                    for from in charted.iter().filter(|from| from.edge != to.edge) {
                        if from.low.cmpgt(high).any() || from.high.cmplt(low).any() {
                            continue;
                        }
                        for (&sample, &read) in self.edge(from.edge).iter().zip(&from.chart) {
                            let at = self.points[sample];
                            if at.cmplt(low).any() || at.cmpgt(high).any() {
                                continue;
                            }
                            let near = self
                                .beyond(body, to, cone, read)
                                .into_iter()
                                .filter(|(_, _, point)| point.distance(at) <= reach)
                                .map(|(rank, along, point)| (rank, along, point, sample));
                            taken.entry(to.edge).or_default().extend(near);
                        }
                    }
                }
            }
            if !self.took(taken, body.scale().eps()) {
                break;
            }
        }
    }

    /// The samples of `edge` read on `cone`, and their box.
    fn charted(&self, edge: EdgeId, cone: &Cone) -> Charted {
        let points = self.edge(edge).iter().map(|id| self.points[*id]);
        let (low, high) = points.clone().fold(
            (DVec3::INFINITY, DVec3::NEG_INFINITY),
            |(low, high), point| (low.min(point), high.max(point)),
        );
        Charted {
            edge,
            chart: points.map(|point| cone.parameters(point)).collect(),
            low,
            high,
        }
    }

    /// Each edge given the places `taken` holds for it, in its order, but
    /// those it has already; whether any was.
    fn took(&mut self, taken: BTreeMap<EdgeId, Vec<Place>>, eps: f64) -> bool {
        let mut any = false;
        for (edge, mut places) in taken {
            places.sort_by(|one, other| one.0.cmp(&other.0).then(one.1.total_cmp(&other.1)));
            let run = self.edges[edge.0 as usize].clone();
            let mut kept: Vec<Vec<usize>> = vec![Vec::new(); run.len()];
            let mut last: Option<DVec3> = None;
            for (rank, _, point, source) in places {
                let held = run.iter().any(|&sample| {
                    let room = if self.is_vertex(sample) {
                        eps * Scale::HAIR
                    } else {
                        eps
                    };
                    self.points[sample].distance(point) <= room
                });
                if held || last.is_some_and(|last| last.distance(point) <= eps) {
                    continue;
                }
                last = Some(point);
                let id = self
                    .vertex_at(point, eps * TOLD)
                    .or_else(|| {
                        (self.points[source].distance(point) <= eps * TOLD).then_some(source)
                    })
                    .unwrap_or_else(|| {
                        self.points.push(point);
                        self.points.len() - 1
                    });
                if kept[rank].last() != Some(&id) {
                    kept[rank].push(id);
                }
                any = true;
            }
            self.edges[edge.0 as usize] = run
                .iter()
                .zip(kept)
                .flat_map(|(&sample, after)| std::iter::once(sample).chain(after))
                .collect();
        }
        any
    }

    /// Where the ruling of `cone` at `read` crosses the edge `to` between
    /// two of its places whose chord stands on the other side of `read`
    /// from the curve, or through it: the rank of the chord, how far along
    /// it, and the point.
    fn beyond(
        &self,
        body: &Body,
        to: &Charted,
        cone: &Cone,
        read: DVec2,
    ) -> Vec<(usize, f64, DVec3)> {
        let curve = body.curve(body.edge(to.edge).curve);
        let off = |theta: f64| (theta - read.x + PI).rem_euclid(TAU) - PI;
        let run = self.edge(to.edge);
        let ring = body.edge(to.edge).ends.is_none();
        let chords = if ring { run.len() } else { run.len() - 1 };
        let mut places = Vec::new();
        for rank in 0..chords {
            let next = (rank + 1) % run.len();
            let [before, after] = [to.chart[rank].x, to.chart[next].x].map(off);
            if before == 0.0 || after == 0.0 || before.signum() == after.signum() {
                continue;
            }
            if (before - after).abs() >= PI {
                continue;
            }
            let [from, end] = [run[rank], run[next]].map(|id| self.points[id]);
            let low = curve.parameter(from);
            let mut high = curve.parameter(end);
            if let Some(period) = curve.period() {
                high = low + (high - low + period / 2.0).rem_euclid(period) - period / 2.0;
            }
            let (mut inside, mut outside) = (low, high);
            for _ in 0..100 {
                let middle = (inside + outside) / 2.0;
                if middle == inside || middle == outside {
                    break;
                }
                if off(cone.parameters(curve.point(middle)).x).signum() == before.signum() {
                    inside = middle;
                } else {
                    outside = middle;
                }
            }
            let t = (inside + outside) / 2.0;
            let point = curve.point(t);
            let [chord_from, chord_to] = [to.chart[rank].y, to.chart[next].y];
            let chord = chord_from + (chord_to - chord_from) * before / (before - after);
            let curve_side = read.y - cone.parameters(point).y;
            if curve_side == 0.0 || (read.y - chord) * curve_side > 0.0 {
                continue;
            }
            let along = if high == low {
                0.0
            } else {
                (t - low) / (high - low)
            };
            places.push((rank, along, point));
        }
        places
    }
}

#[cfg(test)]
mod tests;
