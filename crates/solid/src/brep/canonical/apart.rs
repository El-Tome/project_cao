//! The pairs of surfaces no place lies on both of, read off decision 2: two
//! the relation of their pair finds apart, and two it finds one within the
//! tolerance though the boolean kept them two.
//!
//! Those are surfaces an operand decided apart when it was made — two walls
//! of a slit a hair wide — at a tolerance finer than this operation's, which
//! the larger reach of the other operand may have grown past the hair. The
//! body need not carry the pairs it decided apart: two surfaces it keeps
//! distinct were decided apart, whatever tolerance a later operation brings,
//! and that operation reads it here rather than deciding again.
//!
//! Beside them, the pairs decision 2 finds touching: two such surfaces stand
//! within the tolerance of each other over a band far wider than it, so that
//! a place on both is not for that on the line they touch along — and the
//! line, laid on one of them, may stand a hair off the other: a place within
//! the tolerance of the line is on it only where it is within the tolerance
//! of both surfaces too, which are kept here to be measured. So do two walls
//! of one radius whose axes stand a hair apart, beyond decision 8's: they
//! cross along two lines at an angle the hair over the radius, and stand
//! within the tolerance of each other far from either, and the pair is
//! decided to graze once, here. And the points
//! where two perpendicular cylinders touch — the node of the curve they meet
//! along, or the one point they share — and a cone's apex where a plane
//! holding its axis or a surface touching it there meets it, where a line on
//! one passing through the point only touches the other.

use std::collections::{BTreeMap, BTreeSet};

use glam::DVec3;

use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;

#[derive(Clone, Debug, Default, PartialEq)]
pub(in crate::brep) struct Apart {
    pairs: BTreeSet<[SurfaceId; 2]>,
    touching: BTreeSet<[SurfaceId; 2]>,
    grazing: BTreeSet<[SurfaceId; 2]>,
    pointed: BTreeSet<[SurfaceId; 2]>,
    points: BTreeMap<[SurfaceId; 2], Vec<DVec3>>,
    surfaces: Vec<Surface>,
}

impl Apart {
    /// Every pair decided at the scale `scale_of` gives it: an operand's own
    /// tolerance for a pair it alone carries, which it decided when it was
    /// made — two walls of a crescent a hair wide crossing along two rulings
    /// still cross, whatever tolerance a later leaf brings.
    pub fn of(surfaces: &[Surface], scale_of: impl Fn([SurfaceId; 2]) -> Scale) -> Apart {
        let mut pairs = BTreeSet::new();
        let mut touching = BTreeSet::new();
        let mut grazing = BTreeSet::new();
        let mut pointed = BTreeSet::new();
        let mut points = BTreeMap::new();
        for (one, first) in surfaces.iter().enumerate() {
            for (other, second) in surfaces.iter().enumerate().skip(one + 1) {
                let pair = [SurfaceId(one as u32), SurfaceId(other as u32)];
                let scale = scale_of(pair);
                match relation(first, second, scale) {
                    Relation::Same { .. } | Relation::Apart => {
                        pairs.insert(pair);
                    }
                    Relation::Tangent(_) => {
                        touching.insert(pair);
                    }
                    Relation::Lines([line, _])
                        if grazes(first, second, line.origin, scale.eps()) =>
                    {
                        grazing.insert(pair);
                    }
                    Relation::Meet(meeting) => {
                        let at: Vec<DVec3> = meeting
                            .nodes
                            .iter()
                            .map(|node| node.point)
                            .chain(meeting.contact)
                            .collect();
                        if !at.is_empty() {
                            points.insert(pair, at);
                        }
                    }
                    Relation::Rulings {
                        apex: Some(apex), ..
                    } => {
                        points.insert(pair, vec![apex]);
                    }
                    Relation::Apex(apex) => {
                        pointed.insert(pair);
                        points.insert(pair, vec![apex]);
                    }
                    Relation::Line(_)
                    | Relation::Lines(_)
                    | Relation::Circle(_)
                    | Relation::Rulings { apex: None, .. }
                    | Relation::Unsupported => {}
                }
            }
        }
        Apart {
            pairs,
            touching,
            grazing,
            pointed,
            points,
            surfaces: surfaces.to_vec(),
        }
    }

    pub fn pair(&self, one: SurfaceId, other: SurfaceId) -> bool {
        self.pairs.contains(&[one.min(other), one.max(other)])
    }

    /// Whether two surfaces were decided to touch along a line.
    pub fn touch(&self, one: SurfaceId, other: SurfaceId) -> bool {
        self.touching.contains(&[one.min(other), one.max(other)])
    }

    /// Whether two walls were decided to cross along lines at a grazing
    /// angle, standing within the tolerance of each other far from them.
    pub fn graze(&self, one: SurfaceId, other: SurfaceId) -> bool {
        self.grazing.contains(&[one.min(other), one.max(other)])
    }

    /// Whether a surface of `one` and a surface of `other` were decided to
    /// meet at a cone's apex alone, so that no line lies on both: a ruling
    /// of one, read as a whole line, may still be one of the other's, two
    /// cones of one apex each the other's mirror.
    pub fn pointed(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        one.iter().any(|&first| {
            other.iter().any(|&second| {
                self.pointed
                    .contains(&[first.min(second), first.max(second)])
            })
        })
    }

    /// The points where two perpendicular cylinders, or a cone at its apex
    /// and another surface, were decided to touch.
    pub fn points(&self, one: SurfaceId, other: SurfaceId) -> &[DVec3] {
        self.points
            .get(&[one.min(other), one.max(other)])
            .map_or(&[], Vec::as_slice)
    }

    /// Whether a place stands within `eps` of a surface.
    pub fn near(&self, surface: SurfaceId, point: DVec3, eps: f64) -> bool {
        self.surfaces
            .get(surface.0 as usize)
            .is_none_or(|surface| surface.distance(point).abs() <= eps)
    }

    /// Whether a surface of `one` is apart from a surface of `other`.
    pub fn across(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        one.iter()
            .any(|&first| other.iter().any(|&second| self.pair(first, second)))
    }
}

/// The sine of the widest angle two walls crossing along lines meet at for
/// the crossing to be grazing: within the tolerance of each other a hundred
/// tolerances from the lines, and more.
const GRAZING: f64 = 1e-2;

/// Whether two parallel walls of one radius crossing along a line through
/// `on` meet there at a grazing angle: their axes a hair apart, the crescent
/// between them a cusp at each line.
fn grazes(one: &Surface, other: &Surface, on: DVec3, eps: f64) -> bool {
    let (one, other) = match (one, other) {
        (Surface::Cylinder(one), Surface::Cylinder(other)) => (one, other),
        (Surface::Plane(_) | Surface::Cone(_), _) | (_, Surface::Plane(_) | Surface::Cone(_)) => {
            return false;
        }
    };
    if (one.radius - other.radius).abs() > eps {
        return false;
    }
    let [first, second] = [one, other].map(|cylinder| cylinder.radial(cylinder.parameters(on).x));
    first.cross(second).length() <= GRAZING
}
