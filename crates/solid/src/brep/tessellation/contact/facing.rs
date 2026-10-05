//! Whether two parallel walls stand face to face at a place: both hold a face
//! there, at some height they share.
//!
//! Only there can their triangles lie on each other where the walls all but
//! meet, and only there are the steps of the grid beside the lines they meet
//! along withheld. Where one of them is gone — a union keeps each wall where
//! the other is not, two blocks stacked keep each wall at its own heights —
//! the place is one wall's alone, and withholding its steps would only
//! lengthen its chords past the tolerance.

use std::cell::OnceCell;
use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::gap::Gap;
use super::{APART, Wall, bears, wall_of};
use crate::brep::curve::Curve;
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::topology::{Body, FaceId, SurfaceId};

/// Two parallel walls, and once asked, their faces and the heights they are
/// compared at: one between every two heights where a circle or a vertex of
/// either stands, so that each is a height where no face of either begins or
/// ends.
pub(in crate::brep::tessellation) struct Facing<'a> {
    body: &'a Body,
    walls: [Wall; 2],
    gap: Gap,
    known: OnceCell<Known>,
}

impl<'a> Facing<'a> {
    pub(super) fn of(body: &'a Body, one: &Wall, other: &Wall) -> Facing<'a> {
        Facing {
            body,
            walls: [*one, *other],
            gap: Gap::of(&one.1, &other.1, body.scale().eps()),
            known: OnceCell::new(),
        }
    }

    /// Whether `point`, on `wall`, is crowded by the other wall.
    pub(super) fn crowds(&self, wall: &Cylinder, point: DVec3) -> bool {
        self.gap.crowds(wall, point)
    }

    /// How near each other the two walls stand, as a place of either is
    /// told crowded.
    pub(super) fn gap(&self) -> Gap {
        self.gap
    }

    fn known(&self) -> &Known {
        self.known.get_or_init(|| {
            let body = self.body;
            let eps = body.scale().eps();
            let axis = self.walls[0].1.axis;
            let walls = self.walls;
            let faces = walls.map(|(id, _)| {
                body.face_ids()
                    .filter(|face| body.face(*face).surface == id)
                    .collect()
            });
            let arcs = walls.map(|(own, _)| {
                body.edge_ids()
                    .filter_map(|id| {
                        let edge = body.edge(id);
                        match body.curve(edge.curve) {
                            Curve::Circle(circle)
                                if wall_of(circle, &walls, eps)
                                    .is_some_and(|wall| wall.0 == own) =>
                            {
                                let (from, to) = match edge.ends {
                                    Some(_) => (edge.from.min(edge.to), edge.from.max(edge.to)),
                                    None => (0.0, TAU),
                                };
                                Some([circle.center.dot(axis), from, to])
                            }
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>()
            });
            let circles = arcs.iter().flatten().map(|[level, _, _]| *level);
            let vertices = body
                .vertex_ids()
                .map(|id| body.vertex(id))
                .filter(|vertex| walls.iter().any(|wall| bears(body, vertex, wall)))
                .map(|vertex| vertex.point.dot(axis));
            let mut levels: Vec<f64> = circles.chain(vertices).collect();
            levels.sort_by(f64::total_cmp);
            levels.dedup_by(|higher, lower| *higher - *lower <= eps);
            Known {
                faces,
                levels,
                arcs,
            }
        })
    }

    /// The heights the circles of `wall` stand at, each a stretch of none.
    pub(super) fn rims(&self, wall: SurfaceId) -> Vec<[f64; 2]> {
        let side = usize::from(self.walls[0].0 != wall);
        self.known().arcs[side]
            .iter()
            .map(|[level, _, _]| [*level, *level])
            .collect()
    }

    /// Whether both walls hold a face at the angle of `point`, seen from
    /// each one's axis, at a height they share.
    pub(super) fn at(&self, point: DVec3) -> bool {
        !self.over(point).is_empty()
    }

    /// The stretches of height, each between two levels where a circle or a
    /// vertex of either wall stands, over which both walls hold a face at
    /// the angle of `point`, seen from each one's axis: only the circles
    /// bounding such a stretch face the other wall there. A circle of one
    /// wall standing where the other holds no face — the rim of a disc left
    /// whole above a crescent two walls bound below it — faces nothing, and
    /// is sampled on its whole grid. Every height is taken when that cannot
    /// be told: a wall standing at one height alone, or a face the kernel
    /// cannot locate a place against. A wall holding no face — its circles
    /// printed on a cap, its wall gone — faces the other at the height of
    /// each of its circles alone: a circle printed round a hole it touches
    /// inside is sampled in common with the hole's rim on that cap, and the
    /// hole's other rims are not.
    ///
    /// So too, for two walls of radii apart decided to touch, the height
    /// where a circle of each stands at the angle of `point`: the two bound
    /// one cap there, a pocket's floor round a hole touching its wall
    /// inside, though the walls stand at heights apart. Not for walls
    /// crossing or all but one: a union keeps both circles on its caps all
    /// round, and withholding their steps would lengthen the walls' chords.
    pub(super) fn over(&self, point: DVec3) -> Vec<[f64; 2]> {
        let known = self.known();
        let eps = self.body.scale().eps();
        if known.levels.len() < 2 {
            return vec![[f64::NEG_INFINITY, f64::INFINITY]];
        }
        let angles = [0, 1].map(|side| self.walls[side].1.parameters(point).x);
        let at_level = |side: usize, level: f64| {
            known.arcs[side].iter().any(|[height, from, to]| {
                (height - level).abs() <= eps && from + (angles[side] - from).rem_euclid(TAU) <= *to
            })
        };
        let mut over: Vec<[f64; 2]> = known
            .levels
            .windows(2)
            .filter(|pair| self.both(point, (pair[0] + pair[1]) / 2.0))
            .map(|pair| [pair[0], pair[1]])
            .collect();
        for side in [0, 1] {
            if known.faces[side].is_empty() {
                over.extend(
                    known.arcs[side]
                        .iter()
                        .map(|[level, _, _]| *level)
                        .filter(|level| at_level(side, *level))
                        .map(|level| [level, level]),
                );
            }
        }
        if self.gap.touch() && (self.walls[0].1.radius - self.walls[1].1.radius).abs() > eps {
            over.extend(
                known.arcs[0]
                    .iter()
                    .map(|[level, _, _]| *level)
                    .filter(|level| at_level(0, *level) && at_level(1, *level))
                    .map(|level| [level, level]),
            );
        }
        over
    }

    /// The stretches of height over which `wall` withholds a place of its
    /// grid at `point`, on it, where the two stand within their room: those
    /// it faces the other over there — or every height, unless the other
    /// stands there on the hollow side of `wall`, further inside it than the
    /// rules tell apart. A circle of `wall` facing nothing is sampled
    /// densely, and its triangles reach down the wall to the circles that
    /// face the other, which are not: they sag less than the other's chords
    /// beside the line the two meet along. With the other on the hollow
    /// side, that keeps them out of it; with the other on the bulging side —
    /// a bore inside a boss it touches, a cylinder beside one it touches —
    /// or within what the rules tell apart, they would pass through it.
    pub(super) fn withheld(&self, wall: SurfaceId, point: DVec3) -> Vec<[f64; 2]> {
        let over = self.over(point);
        let [own, other] = if self.walls[0].0 == wall {
            [self.walls[0].1, self.walls[1].1]
        } else {
            [self.walls[1].1, self.walls[0].1]
        };
        let from = point - other.origin;
        let out = (from - other.axis * other.axis.dot(from)).normalize_or_zero();
        let nearest = point - out * other.distance(point);
        let hollow = own.distance(nearest) < -self.body.scale().eps() * APART / 2.0;
        if over.is_empty() || hollow {
            over
        } else {
            vec![[f64::NEG_INFINITY, f64::INFINITY]]
        }
    }

    /// Whether both walls hold a face at the angle of `point` and `height`,
    /// which stands between two levels further apart than the kernel's
    /// tolerance: a face ending at either level is told from one going on
    /// past it to what the rules tell apart, not to that tolerance — a skirt
    /// a hair high, below the floor of the other wall, faces nothing of it.
    fn both(&self, point: DVec3, height: f64) -> bool {
        let faces = &self.known().faces;
        let near = self.body.scale().eps() * APART / 2.0;
        let holds = |side: usize| {
            let angle = self.walls[side].1.parameters(point).x;
            faces[side].iter().any(|face| {
                !matches!(
                    self.body.locate(*face, DVec2::new(angle, height), near),
                    Ok(Location::Outside)
                )
            })
        };
        holds(0) && holds(1)
    }
}

/// The faces of two walls, the levels between which they are compared, and
/// the arcs of their circles: the height each stands at and the angles it
/// spans.
struct Known {
    faces: [Vec<FaceId>; 2],
    levels: Vec<f64>,
    arcs: [Vec<[f64; 3]>; 2],
}

/// Every two parallel walls of a body, and where they face each other.
pub(in crate::brep::tessellation) struct Zones<'a> {
    pairs: Vec<Facing<'a>>,
}

impl<'a> Zones<'a> {
    pub(in crate::brep::tessellation) fn of(body: &'a Body, walls: &[Wall]) -> Zones<'a> {
        let mut pairs = Vec::new();
        for (at, one) in walls.iter().enumerate() {
            for other in &walls[at + 1..] {
                if one.1.axis.cross(other.1.axis).length() <= Scale::RELATIVE {
                    pairs.push(Facing::of(body, one, other));
                }
            }
        }
        Zones { pairs }
    }

    /// Where two parallel walls, by id, face each other.
    pub(super) fn between(&self, one: SurfaceId, other: SurfaceId) -> Option<&Facing<'a>> {
        self.pairs.iter().find(|facing| {
            let [first, second] = facing.walls.map(|(id, _)| id);
            (first, second) == (one, other) || (first, second) == (other, one)
        })
    }

    /// Whether `point`, on `cylinder`, stands within the room of a parallel
    /// wall that faces that one there: a sample there would be as good as on
    /// the other wall.
    pub(in crate::brep::tessellation) fn crowded(&self, cylinder: &Cylinder, point: DVec3) -> bool {
        self.pairs.iter().any(|facing| {
            let [one, other] = facing.walls;
            if one.1 != *cylinder && other.1 != *cylinder {
                return false;
            }
            facing.crowds(cylinder, point) && facing.at(point)
        })
    }
}
