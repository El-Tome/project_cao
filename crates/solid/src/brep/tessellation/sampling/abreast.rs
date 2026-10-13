//! Two parallel walls, an elliptic one among them (#571), standing within
//! twice a chord's sag of each other where the distance between them turns
//! — one touching the other inside, or crossing it a hair from a touch —
//! with faces at heights that meet: the rays both take from a point inside
//! both, as two circular walls close to each other take theirs
//! (`contact/together.rs`).
//!
//! Each sampled on its own grid, the outer one's chords sag past the inner
//! one's samples about where the two touch, and the faces either bounds on
//! a cap they share, or the strips of the two walls, cross. Sampled on rays
//! from the axis of the one standing inside the other there — through the
//! samples of either's grid and through the vertices on either — both are
//! graphs of the angle round it, one further out than the other on every
//! ray between two the walls cross at, and chords between the same two
//! rays cannot cross. Neither takes a ray, nor a circle a step of
//! its grid, along which the two stand within the room of each other, as
//! an elliptic rim leaves out its places a hair from a surface its end lies
//! on: about a touch of a high order, an osculating circle at a vertex, a
//! sample there of one alone stands outside the other's chord.

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::super::contact::{self, Contact};
use super::divisions;
use crate::brep::relation::closest;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};

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
    /// one inside the other and within twice `tolerance` of it somewhere.
    pub(super) fn of(body: &Body, tolerance: f64) -> Abreast {
        let mut found = Abreast::default();
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
                if nearest > 2.0 * tolerance || !side_by_side([one, other], body) {
                    continue;
                }
                let Some([inner, outer]) = hubbed([one, other], body) else {
                    continue;
                };
                found.shared([inner, outer], body, tolerance);
                found.crowded([inner, outer], body, tolerance);
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

    /// Both walls sampled on the rays from the inner one's axis through the
    /// samples of either's grid and through the vertices on either.
    fn shared(&mut self, [inner, outer]: [SurfaceId; 2], body: &Body, tolerance: f64) {
        let (hub, axis) = axis_of(body.surface(inner));
        let flat = |way: DVec3| way - axis * axis.dot(way);
        let mut places: Vec<DVec3> = [inner, outer]
            .into_iter()
            .flat_map(|wall| grid(body.surface(wall), tolerance))
            .collect();
        places.extend(
            body.vertices
                .iter()
                .filter(|vertex| vertex.on.contains(&inner) || vertex.on.contains(&outer))
                .map(|vertex| vertex.point),
        );
        let room = body.scale().eps() * contact::APART;
        for place in places {
            let Some(way) = flat(place - hub).try_normalize() else {
                continue;
            };
            let hits = [inner, outer].map(|wall| hit(body.surface(wall), hub, way));
            let [Some((within, _)), Some((beyond, _))] = hits else {
                continue;
            };
            if within.distance(beyond) <= room {
                continue;
            }
            for (wall, (_, taken)) in [inner, outer].into_iter().zip(hits.into_iter().flatten()) {
                match taken {
                    Taken::Ray(ray) => self.rounds.entry(wall).or_default().push(ray),
                    Taken::Angle(angle) => self.ovals.entry(wall).or_default().push(angle),
                }
            }
        }
    }
}

/// What a wall takes for a ray: a circular wall the direction from its
/// axis, an elliptic one the eccentric angle.
#[derive(Clone, Copy)]
enum Taken {
    Ray(DVec3),
    Angle(f64),
}

/// Where the ray from `hub` along `way`, square to the wall's axis and
/// from inside it, leaves the wall, and what the wall takes for it.
fn hit(wall: &Surface, hub: DVec3, way: DVec3) -> Option<(DVec3, Taken)> {
    match wall {
        Surface::Cylinder(round) => {
            let flat = |v: DVec3| v - round.axis * round.axis.dot(v);
            let from = flat(hub - round.origin);
            let along = from.dot(way);
            let reach = along * along - (from.length_squared() - round.radius * round.radius);
            if reach < 0.0 {
                return None;
            }
            let point = hub + way * (reach.sqrt() - along);
            let ray = flat(point - round.origin).try_normalize()?;
            Some((point, Taken::Ray(ray)))
        }
        Surface::Elliptic(oval) => {
            let squashed =
                |v: DVec3| DVec2::new(v.dot(oval.u) / oval.major, v.dot(oval.v) / oval.minor);
            let (from, pace) = (squashed(hub - oval.origin), squashed(way));
            let t = beyond(from, pace)?;
            let at = from + pace * t;
            Some((hub + way * t, Taken::Angle(at.y.atan2(at.x))))
        }
        Surface::Plane(_) | Surface::Cone(_) => None,
    }
}

/// Whether faces of two parallel walls stand at heights along the axis
/// that meet: two walls whose faces stand one above the other never face
/// each other, though one stands inside the other seen along the axis — a
/// post under a boss — and a step one leaves out there would leave a chord
/// across two.
fn side_by_side([one, other]: [SurfaceId; 2], body: &Body) -> bool {
    let (_, axis) = axis_of(body.surface(one));
    let [first, second] = [one, other].map(|wall| heights(wall, axis, body));
    let eps = body.scale().eps();
    match (first, second) {
        (Some([low, high]), Some([from, to])) => from <= high + eps && low <= to + eps,
        _ => false,
    }
}

/// The least and the largest height along `axis` the ends of the edges
/// bounding a wall's faces stand at; none where it holds no face.
fn heights(wall: SurfaceId, axis: DVec3, body: &Body) -> Option<[f64; 2]> {
    let mut span: Option<[f64; 2]> = None;
    for face in body
        .face_ids()
        .filter(|face| body.face(*face).surface == wall)
    {
        for coedge in body.face(face).loops.iter().flatten() {
            let edge = body.edge(coedge.edge);
            for at in [edge.from, edge.to] {
                let height = axis.dot(body.point_on(coedge.edge, at));
                span = Some(span.map_or([height, height], |[low, high]| {
                    [low.min(height), high.max(height)]
                }));
            }
        }
    }
    span
}

/// The two walls of a pair, first the one whose axis stands inside the
/// other wall, the smaller where each stands inside the other's: a point
/// inside both, which every ray from it leaves once through each wall. None
/// where neither does.
fn hubbed([one, other]: [SurfaceId; 2], body: &Body) -> Option<[SurfaceId; 2]> {
    let holds = |wall: SurfaceId, of: SurfaceId| {
        let (hub, _) = axis_of(body.surface(wall));
        body.surface(of).distance(hub) < 0.0
    };
    let size = |wall: SurfaceId| match body.surface(wall) {
        Surface::Cylinder(round) => round.radius,
        Surface::Elliptic(oval) => oval.major,
        Surface::Plane(_) | Surface::Cone(_) => f64::INFINITY,
    };
    match (holds(one, other), holds(other, one)) {
        (true, true) if size(other) < size(one) => Some([other, one]),
        (true, _) => Some([one, other]),
        (false, true) => Some([other, one]),
        (false, false) => None,
    }
}

/// The points of a wall's grid at the height of its origin: a circle's
/// steps from its `u`, an ellipse's steps of eccentric angle from its major
/// half-axis, divided as a circle of that half-axis is.
fn grid(wall: &Surface, tolerance: f64) -> Vec<DVec3> {
    let steps = match wall {
        Surface::Cylinder(round) => divisions(round.radius, tolerance),
        Surface::Elliptic(oval) => divisions(oval.major, tolerance),
        Surface::Plane(_) | Surface::Cone(_) => return Vec::new(),
    };
    (0..steps)
        .map(|step| wall.point(DVec2::new(TAU * step as f64 / steps as f64, 0.0)))
        .collect()
}

/// A point of a wall's axis, and its direction.
fn axis_of(wall: &Surface) -> (DVec3, DVec3) {
    match wall {
        Surface::Cylinder(round) => (round.origin, round.axis),
        Surface::Elliptic(oval) => (oval.origin, oval.axis),
        Surface::Plane(plane) => (plane.origin, plane.normal),
        Surface::Cone(cone) => (cone.origin, cone.axis),
    }
}

/// How far along `pace` from `from` the unit circle is left, `from` inside
/// it.
fn beyond(from: DVec2, pace: DVec2) -> Option<f64> {
    let (a, b, c) = (
        pace.length_squared(),
        from.dot(pace),
        from.length_squared() - 1.0,
    );
    let reach = b * b - a * c;
    (a > 0.0 && reach >= 0.0).then(|| (reach.sqrt() - b) / a)
}
