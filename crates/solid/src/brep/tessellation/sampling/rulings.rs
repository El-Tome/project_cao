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
//! within a hair of one of its ends, is that one.

use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::Samples;
use crate::brep::Scale;
use crate::brep::curve::Curve;
use crate::brep::surface::{Cone, Surface};
use crate::brep::topology::{Body, EdgeId};

/// How many times the edges are read again: a place taken bends the chord
/// it falls on, which another place of the other edge may then stand beyond.
const ROUNDS: usize = 8;

/// A place an edge takes: the rank of the chord it falls on, how far along
/// it, and the point.
type Place = (usize, f64, DVec3);

impl Samples {
    /// Every two edges of a cone's face given a place on each ruling through
    /// a place of the other that stands beyond its chord, within twice
    /// `tolerance` of it.
    pub(super) fn on_common_rulings(&mut self, body: &Body, tolerance: f64) {
        let mut pairs: Vec<(Cone, EdgeId, EdgeId)> = Vec::new();
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
            for &to in &edges {
                for &from in edges.iter().filter(|from| **from != to) {
                    pairs.push((*cone, from, to));
                }
            }
        }
        for _ in 0..ROUNDS {
            let mut taken: BTreeMap<EdgeId, Vec<Place>> = BTreeMap::new();
            for (cone, from, to) in &pairs {
                for &sample in self.edge(*from) {
                    let at = self.points[sample];
                    let near = self
                        .beyond(body, *to, cone, at)
                        .into_iter()
                        .filter(|(_, _, point)| point.distance(at) <= 2.0 * tolerance);
                    taken.entry(*to).or_default().extend(near);
                }
            }
            if !self.took(taken, body.scale().eps()) {
                break;
            }
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
            for (rank, _, point) in places {
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
                kept[rank].push(self.points.len());
                self.points.push(point);
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

    /// Where the ruling of `cone` through `at` crosses the edge `edge`
    /// between two of its places whose chord stands on the other side of
    /// `at` from the curve, or through it: the rank of the chord, how far
    /// along it, and the point.
    fn beyond(&self, body: &Body, edge: EdgeId, cone: &Cone, at: DVec3) -> Vec<Place> {
        let curve = body.curve(body.edge(edge).curve);
        let [theta, height] = cone.parameters(at).to_array();
        let off = |point: DVec3| (cone.parameters(point).x - theta + PI).rem_euclid(TAU) - PI;
        let run = self.edge(edge);
        let ring = body.edge(edge).ends.is_none();
        let chords = if ring { run.len() } else { run.len() - 1 };
        let mut places = Vec::new();
        for rank in 0..chords {
            let [from, to] = [run[rank], run[(rank + 1) % run.len()]].map(|id| self.points[id]);
            let [before, after] = [off(from), off(to)];
            if before == 0.0 || after == 0.0 || before.signum() == after.signum() {
                continue;
            }
            if (before - after).abs() >= PI {
                continue;
            }
            let low = curve.parameter(from);
            let mut high = curve.parameter(to);
            if let Some(period) = curve.period() {
                high = low + (high - low + period / 2.0).rem_euclid(period) - period / 2.0;
            }
            let (mut inside, mut outside) = (low, high);
            for _ in 0..100 {
                let middle = (inside + outside) / 2.0;
                if middle == inside || middle == outside {
                    break;
                }
                if off(curve.point(middle)).signum() == before.signum() {
                    inside = middle;
                } else {
                    outside = middle;
                }
            }
            let t = (inside + outside) / 2.0;
            let point = curve.point(t);
            let [chord_from, chord_to] = [from, to].map(|end| cone.parameters(end).y);
            let chord = chord_from + (chord_to - chord_from) * before / (before - after);
            let curve_side = height - cone.parameters(point).y;
            if curve_side == 0.0 || (height - chord) * curve_side > 0.0 {
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
