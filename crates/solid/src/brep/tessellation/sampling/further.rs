//! The sampling's rounds run again aside, then further, for
//! [`crate::brep::settling`]: copies of [`Samples::of`] and
//! [`Samples::through`] up to their last contacts, so that nothing they
//! hand on is touched.

use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::panic::AssertUnwindSafe;
use std::time::Instant;

use glam::DVec3;

use super::abreast::Abreast;
use super::axis::Axes;
use super::cone::{self, Grid};
use super::{Samples, beneath, conic, on_meets};
use crate::brep::curve::{Curve, Wall};
use crate::brep::settling::{self, CAP, Record, Settling};
use crate::brep::tessellation::contact::{self, Contact, Zones};
use crate::brep::topology::{Body, EdgeId, SurfaceId};

type Rays = BTreeMap<SurfaceId, Vec<DVec3>>;

struct State {
    meets: Vec<Vec<DVec3>>,
    rays: Rays,
}

impl State {
    fn of(meets: Vec<Vec<DVec3>>, contacts: &BTreeMap<SurfaceId, Contact>) -> State {
        let rays = contacts
            .iter()
            .map(|(id, contact)| (*id, contact.rays.clone()))
            .collect();
        State { meets, rays }
    }

    fn size(&self) -> (usize, usize) {
        (
            self.meets.iter().map(Vec::len).sum(),
            self.rays.values().map(Vec::len).sum(),
        )
    }

    fn apart(&self, other: &State, radii: &BTreeMap<SurfaceId, f64>) -> f64 {
        self.distances(other, radii, None).fold(0.0, f64::max)
    }

    fn within(&self, other: &State, radii: &BTreeMap<SurfaceId, f64>, eps: f64) -> bool {
        self.distances(other, radii, Some(eps))
            .all(|apart| apart <= eps)
    }

    /// How far each meet's samples, then each wall's rays, stand from
    /// `other`'s; with `bound`, only whether each is within it is exact.
    fn distances<'a>(
        &'a self,
        other: &'a State,
        radii: &'a BTreeMap<SurfaceId, f64>,
        bound: Option<f64>,
    ) -> impl Iterator<Item = f64> + 'a {
        let meets = self
            .meets
            .iter()
            .zip(&other.meets)
            .map(move |(one, two)| hausdorff(one, two, bound, |a, b| (a - b).length()));
        let walls = self.rays.keys().chain(other.rays.keys()).map(move |id| {
            let one = self.rays.get(id).map_or(&[][..], Vec::as_slice);
            let two = other.rays.get(id).map_or(&[][..], Vec::as_slice);
            match radii.get(id) {
                Some(radius) => hausdorff(one, two, bound, |a, b| {
                    radius * (a.normalize_or_zero() - b.normalize_or_zero()).length()
                }),
                None => hausdorff(one, two, bound, |a, b| (a - b).length()),
            }
        });
        meets.chain(walls)
    }

    /// The meets whose samples stand further than `eps` from `other`'s.
    fn moved(&self, other: &State, eps: f64) -> Vec<usize> {
        (0..self.meets.len())
            .filter(|at| {
                hausdorff(&self.meets[*at], &other.meets[*at], Some(eps), |a, b| {
                    (a - b).length()
                }) > eps
            })
            .collect()
    }

    fn same(&self, other: &State) -> bool {
        self.meets == other.meets && self.rays == other.rays
    }
}

/// The furthest a point of either stands from the nearest of the other; past
/// `bound`, any figure past it. The nearest is looked for from the same rank
/// first, where an unmoved state has it.
fn hausdorff(
    one: &[DVec3],
    two: &[DVec3],
    bound: Option<f64>,
    apart: impl Fn(DVec3, DVec3) -> f64,
) -> f64 {
    if one.is_empty() || two.is_empty() {
        return if one.len() == two.len() {
            0.0
        } else {
            f64::INFINITY
        };
    }
    let reach = |from: &[DVec3], to: &[DVec3]| {
        let mut furthest: f64 = 0.0;
        for (rank, a) in from.iter().enumerate() {
            let mut nearest = apart(*a, to[rank.min(to.len() - 1)]);
            if bound.is_none_or(|bound| nearest > bound) {
                for b in to {
                    nearest = nearest.min(apart(*a, *b));
                    if bound.is_some_and(|bound| nearest <= bound) {
                        break;
                    }
                }
            }
            furthest = furthest.max(nearest);
            if bound.is_some_and(|bound| furthest > bound) {
                return furthest;
            }
        }
        furthest
    };
    let there = reach(one, two);
    if bound.is_some_and(|bound| there > bound) {
        return there;
    }
    there.max(reach(two, one))
}

/// The rounds of `body`'s sampling measured, when this thread asked for it,
/// against `drawn`, what the sampling gave.
pub(in crate::brep::tessellation) fn measured(body: &Body, tolerance: f64, drawn: &Samples) {
    if !settling::armed() {
        return;
    }
    let mut hasher = DefaultHasher::new();
    format!("{body:?}").hash(&mut hasher);
    tolerance.to_bits().hash(&mut hasher);
    let key = hasher.finish();
    if settling::recalled(key) {
        return;
    }
    let measuring = std::panic::catch_unwind(AssertUnwindSafe(|| measure(body, tolerance, drawn)));
    settling::record(
        key,
        measuring.unwrap_or(Record {
            settling: Settling::Broke,
            sizes: Vec::new(),
            moved: f64::NAN,
            exact: false,
            rays: false,
            astray: 0,
            on_a_cone: false,
            chained: false,
            micros: [0; 2],
        }),
    );
}

fn measure(body: &Body, tolerance: f64, drawn: &Samples) -> Record {
    let started = Instant::now();
    let walls = contact::walls(body);
    let zones = Zones::of(body, &walls);
    let grids = cone::grids(body, tolerance);
    let first = Samples::through(body, &walls, &zones, &grids, &BTreeMap::new(), tolerance);
    let mut beneath = beneath::walls(body, &walls, &first, tolerance);
    beneath.extend(beneath::oval::walls(body, &first, tolerance));
    let ran = Ran::of(body, (&walls, &zones, &grids), &beneath, tolerance);
    let copied = started.elapsed();
    let eps = body.scale().eps();
    let astray = astray(body, drawn, &ran.meets, eps);
    let rays = ran
        .contacts
        .values()
        .any(|contact| !contact.rays.is_empty());
    let radii: BTreeMap<SurfaceId, f64> =
        walls.iter().map(|(id, wall)| (*id, wall.radius)).collect();
    let mut states = vec![State::of(ran.meets, &ran.contacts)];
    let mut contacts = ran.contacts;
    let mut settling = None;
    let mut moved = 0.0;
    let mut exact = true;
    let mut on_cones = [false; 2];
    if !ran.meeting {
        settling = Some(Settling::NoMeet);
    }
    for round in 1..=CAP {
        if settling.is_some() {
            break;
        }
        let meets = on_meets(body, &walls, &zones, &contacts, &grids, tolerance);
        contacts = contact::contacts(
            body, &walls, &zones, &meets, &beneath, &ran.given, tolerance,
        );
        let state = State::of(meets, &contacts);
        let last = &states[round - 1];
        if round == 1 {
            moved = state.apart(last, &radii);
            exact = state.same(last);
            on_cones = on_cones_of(body, &state.moved(last, eps));
        }
        if state.within(last, &radii, eps) {
            settling = Some(match round {
                1 => Settling::Settled,
                _ => Settling::Later(round - 1),
            });
        } else if let Some(earlier) = states[..round - 1]
            .iter()
            .position(|earlier| state.within(earlier, &radii, eps))
        {
            settling = Some(Settling::Cycles(round - earlier));
        }
        states.push(state);
    }
    let sizes: Vec<(usize, usize)> = states.iter().map(State::size).collect();
    let settling = settling.unwrap_or_else(|| {
        let total = |(samples, rays): (usize, usize)| samples + rays;
        let last = total(sizes[sizes.len() - 1]);
        if sizes[..sizes.len() - 1]
            .iter()
            .all(|size| total(*size) < last)
        {
            Settling::Grows
        } else {
            Settling::Drifts
        }
    });
    Record {
        settling,
        sizes,
        moved,
        exact,
        rays,
        astray,
        on_a_cone: on_cones[0],
        chained: on_cones[1],
        micros: [copied, started.elapsed() - copied].map(|spent| spent.as_micros() as u64),
    }
}

/// Whether any of the meets `moved` runs along a cone, and whether any runs
/// along a cone and a wall that also meets a wall that is no cone: the
/// curves `fix/761-last-rays-attempt-set-aside` sampled once more.
fn on_cones_of(body: &Body, moved: &[usize]) -> [bool; 2] {
    let held = |id: EdgeId| match body.curve(body.edge(id).curve) {
        Curve::Meet(meet) => Some(contact::held(body, id, meet)),
        Curve::Line(_) | Curve::Circle(_) | Curve::Conic(_) => None,
    };
    let cone = |wall: &Wall| matches!(wall, Wall::Cone(_));
    let chained: Vec<Wall> = body
        .edge_ids()
        .filter_map(held)
        .filter(|walls| !walls.iter().any(cone))
        .flatten()
        .collect();
    let mut found = [false; 2];
    for at in moved {
        let Some(walls) = held(EdgeId(*at as u32)) else {
            continue;
        };
        if walls.iter().any(cone) {
            found[0] = true;
            found[1] |= walls.iter().any(|wall| chained.contains(wall));
        }
    }
    found
}

/// What [`Samples::through`] leaves its rounds on: the samples of the meets,
/// the contacts, and what each wall was last given on its own.
struct Ran {
    meets: Vec<Vec<DVec3>>,
    contacts: BTreeMap<SurfaceId, Contact>,
    given: Rays,
    meeting: bool,
}

impl Ran {
    fn of(
        body: &Body,
        (walls, zones, grids): (&[contact::Wall], &Zones, &BTreeMap<SurfaceId, Grid>),
        beneath: &Rays,
        tolerance: f64,
    ) -> Ran {
        let abreast = Abreast::of(body, tolerance);
        let axes = Axes::of(body, grids, walls, tolerance);
        let given = abreast.joined(axes.given());
        let contacts_on = |meets: &[Vec<DVec3>], given: &Rays| {
            contact::contacts(body, walls, zones, meets, beneath, given, tolerance)
        };
        let mut meets = on_meets(body, walls, zones, &BTreeMap::new(), grids, tolerance);
        let mut contacts = contacts_on(&meets, &given);
        let meeting = body
            .edge_ids()
            .any(|id| matches!(body.curve(body.edge(id).curve), Curve::Meet(_)));
        if meeting {
            meets = on_meets(body, walls, zones, &contacts, grids, tolerance);
            contacts = contacts_on(&meets, &given);
        }
        let mut axes = axes.gathered(&contacts);
        let mut last = abreast.joined(axes.given());
        if last != given {
            contacts = contacts_on(&meets, &last);
            axes = axes.gathered(&contacts);
        }
        let along = conic::rays_along(body, walls, zones, grids, &contacts, tolerance);
        if !along.is_empty() {
            last = abreast.joined(axes.given());
            for (wall, rays) in along {
                last.entry(wall).or_default().extend(rays);
            }
            contacts = contacts_on(&meets, &last);
            if meeting {
                meets = on_meets(body, walls, zones, &contacts, grids, tolerance);
                contacts = contacts_on(&meets, &last);
            }
        }
        Ran {
            meets,
            contacts,
            given: last,
            meeting,
        }
    }
}

/// How many samples the sampling drew between the ends of a meet stand
/// further than a hair from every one of the copy's for it.
fn astray(body: &Body, drawn: &Samples, meets: &[Vec<DVec3>], eps: f64) -> usize {
    body.edge_ids()
        .filter(|id| matches!(body.curve(body.edge(*id).curve), Curve::Meet(_)))
        .map(|id: EdgeId| {
            let run = drawn.edge(id);
            let inner = match body.edge(id).ends {
                Some(_) if run.len() >= 2 => &run[1..run.len() - 1],
                Some(_) => &[][..],
                None => run,
            };
            inner
                .iter()
                .filter(|sample| !drawn.is_vertex(**sample))
                .filter(|sample| {
                    let point = drawn.point(**sample);
                    meets[id.0 as usize]
                        .iter()
                        .all(|copy| (point - *copy).length() > eps)
                })
                .count()
        })
        .sum()
}
