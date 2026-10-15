//! Two parallel walls, an elliptic one among them (#571), that meet, or
//! stand within twice a chord's sag of each other where the distance
//! between them turns — one touching the other inside, crossing it a hair
//! from a touch, or crossing it anywhere — with faces at heights that meet:
//! the rays both take from a point inside both, as two circular walls close
//! to each other take theirs (`contact/together.rs`).
//!
//! Each sampled on its own grid, the outer one's chords sag past the inner
//! one's samples about where the two touch, or where two ellipses turned a
//! few degrees apart leave a crossing nearly along each other, and the
//! faces either bounds on a cap they share, or the strips of the two walls,
//! cross. Sampled on rays
//! from the axis of the one standing inside the other there — through the
//! samples of either's grid and through the vertices on either — both are
//! graphs of the angle round it, one further out than the other on every
//! ray between two the walls cross at, and chords between the same two
//! rays cannot cross. Neither takes a ray, nor a circle a step of
//! its grid, along which the two stand within the room of each other, as
//! an elliptic rim leaves out its places a hair from a surface its end lies
//! on: about a touch of a high order, an osculating circle at a vertex, a
//! sample there of one alone stands outside the other's chord.
//!
//! A wall abreast of two others takes the rays of both pairs; the walls
//! the pairs join into a group are all sampled from one point inside every
//! one of them, so that each takes every ray. Where no such point is, each
//! pair standing close is sampled from its own, and a pair that only
//! crosses keeps its grids.

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

mod groups;
mod rays;

use super::super::contact::{self, Contact};
use super::divisions;
use crate::brep::relation::{Relation, closest, relation};
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};
use groups::{common_hub, grouped, hubbed, side_by_side};
use rays::{Taken, alone, axis_of, grid, hit};

/// What the walls of a body close inside one another take besides their
/// grid: for a circular wall, the directions from its axis its circles are
/// also sampled along; for an elliptic one, the angles its rims are.
#[derive(Debug, Default)]
pub(super) struct Abreast {
    rounds: BTreeMap<SurfaceId, Vec<DVec3>>,
    ovals: BTreeMap<SurfaceId, Vec<f64>>,
    withheld: BTreeMap<SurfaceId, Vec<usize>>,
}

impl Abreast {
    /// Every pair of parallel walls of `body`, an elliptic one among them,
    /// that meet, or stand one inside the other within twice `tolerance` of
    /// it somewhere, sampled from a point inside every wall the pairs join
    /// into a group. Where no such point is, each pair standing that close
    /// is sampled from its own, and a pair that only crosses is left to its
    /// grids: rays from two points on one wall leave samples the third
    /// never takes (campaign seed 5571469258).
    pub(super) fn of(body: &Body, tolerance: f64) -> Abreast {
        let mut found = Abreast::default();
        let mut pairs = Vec::new();
        let walls: Vec<SurfaceId> = (0..body.surfaces.len() as u32)
            .map(SurfaceId)
            .filter(|id| {
                matches!(
                    body.surface(*id),
                    Surface::Cylinder(_) | Surface::Elliptic(_)
                )
            })
            .collect();
        for (rank, &one) in walls.iter().enumerate() {
            for &other in &walls[rank + 1..] {
                let pair = [body.surface(one), body.surface(other)];
                if !pair.iter().any(|wall| matches!(wall, Surface::Elliptic(_))) {
                    continue;
                }
                let Some(nearest) = closest(pair[0], pair[1], body.scale()) else {
                    continue;
                };
                let near = nearest <= 2.0 * tolerance;
                let meeting = !matches!(
                    relation(pair[0], pair[1], body.scale()),
                    Relation::Apart | Relation::Unsupported
                );
                if !(near || meeting) || !side_by_side([one, other], body) {
                    continue;
                }
                if let Some(pair) = hubbed([one, other], body) {
                    pairs.push((pair, near));
                }
            }
        }
        let joined: Vec<[SurfaceId; 2]> = pairs.iter().map(|(pair, _)| *pair).collect();
        for group in grouped(&joined) {
            if let Some(hub) = common_hub(&group, body) {
                found.shared(&group, hub, body, tolerance);
                continue;
            }
            for &([inner, outer], near) in &pairs {
                if near && group.contains(&inner) {
                    let (hub, _) = axis_of(body.surface(inner));
                    found.shared(&[inner, outer], hub, body, tolerance);
                }
            }
        }
        for &(pair, near) in &pairs {
            if near {
                found.crowded(pair, body, tolerance);
            }
        }
        found
    }

    /// The rays a wall's circles take on its own, with those given.
    pub(super) fn joined(
        &self,
        mut given: BTreeMap<SurfaceId, Vec<DVec3>>,
    ) -> BTreeMap<SurfaceId, Vec<DVec3>> {
        for (wall, rays) in &self.rounds {
            given.entry(*wall).or_default().extend(rays);
        }
        given
    }

    /// The steps of a circular wall's grid standing within the room of the
    /// wall it stands close to, which its circles leave out at every height
    /// as that wall's rims leave out their places there: a sample a hair
    /// from the line two walls touch along stands as good as on the other
    /// wall, and the first chords from the line reach out to where the two
    /// stand apart on both.
    pub(super) fn withhold(&self, contacts: &mut BTreeMap<SurfaceId, Contact>) {
        for (wall, steps) in &self.withheld {
            let withheld = &mut contacts.entry(*wall).or_default().withheld;
            for step in steps {
                withheld.push((*step, [f64::NEG_INFINITY, f64::INFINITY]));
            }
        }
    }

    /// The angles an elliptic wall's rims take besides their grid.
    pub(super) fn angles(&self, wall: SurfaceId) -> &[f64] {
        self.ovals.get(&wall).map_or(&[], Vec::as_slice)
    }

    /// The steps of each circular wall of the pair standing within the room
    /// of the other wall, a step or less from a corner on both: by a line
    /// the two touch along. Where no corner is, a face of either is gone
    /// there — a chord of an ellipse cut it away — and a step left out would
    /// leave a chord across two.
    fn crowded(&mut self, pair: [SurfaceId; 2], body: &Body, tolerance: f64) {
        let room = body.scale().eps() * contact::APART;
        for (wall, other) in [(pair[0], pair[1]), (pair[1], pair[0])] {
            let Surface::Cylinder(round) = body.surface(wall) else {
                continue;
            };
            let steps = divisions(round.radius, tolerance);
            let width = TAU / steps as f64;
            let touching: Vec<f64> = body
                .vertices
                .iter()
                .filter(|vertex| vertex.on.contains(&wall) && vertex.on.contains(&other))
                .map(|vertex| round.parameters(vertex.point).x)
                .collect();
            let near: Vec<usize> = (0..steps)
                .filter(|step| {
                    let angle = width * *step as f64;
                    let point = round.point(DVec2::new(angle, 0.0));
                    body.surface(other).distance(point).abs() <= room
                        && touching.iter().any(|at| {
                            ((angle - at + TAU / 2.0).rem_euclid(TAU) - TAU / 2.0).abs() <= width
                        })
                })
                .collect();
            if !near.is_empty() {
                self.withheld.entry(wall).or_default().extend(near);
            }
        }
    }

    /// The walls of a group sampled on the rays from `hub`, inside every
    /// one of them, through the samples of each one's grid, through the
    /// vertices on each, and through the places an elliptic wall's rims take
    /// on their own: where a plane along the axis reaches furthest past it,
    /// where a vertex stands just inside it. Taken by one rim alone, such a
    /// place of the outer wall beside a touch left a chord of it across the
    /// inner rim's (campaign seed 5571457963).
    fn shared(&mut self, group: &[SurfaceId], hub: DVec3, body: &Body, tolerance: f64) {
        let (_, axis) = axis_of(body.surface(group[0]));
        let flat = |way: DVec3| way - axis * axis.dot(way);
        let mut places: Vec<DVec3> = group
            .iter()
            .flat_map(|wall| grid(body.surface(*wall), tolerance))
            .chain(group.iter().flat_map(|wall| alone(body, *wall, tolerance)))
            .collect();
        places.extend(
            body.vertices
                .iter()
                .filter(|vertex| group.iter().any(|wall| vertex.on.contains(wall)))
                .map(|vertex| vertex.point),
        );
        let room = body.scale().eps() * contact::APART;
        for place in places {
            let Some(way) = flat(place - hub).try_normalize() else {
                continue;
            };
            let Some(hits) = group
                .iter()
                .map(|wall| hit(body.surface(*wall), hub, way))
                .collect::<Option<Vec<(DVec3, Taken)>>>()
            else {
                continue;
            };
            let crowded = hits.iter().enumerate().any(|(rank, (one, _))| {
                hits[rank + 1..]
                    .iter()
                    .any(|(other, _)| one.distance(*other) <= room)
            });
            if crowded {
                continue;
            }
            for (wall, (_, taken)) in group.iter().zip(hits) {
                match taken {
                    Taken::Ray(ray) => self.rounds.entry(*wall).or_default().push(ray),
                    Taken::Angle(angle) => self.ovals.entry(*wall).or_default().push(angle),
                }
            }
        }
    }
}
