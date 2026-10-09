//! What planes fix by themselves: two across each other share one line, and
//! three whose normals span space share one place. Two curves found on the
//! same two such planes are one curve, and two corners found on the same
//! three one corner, however far apart they were found — each plane taken
//! for another within the tolerance by decision 1, their crossing moves by
//! more than the tolerance. Such a corner stands where the three meet, and
//! one on two across each other only on the line they share. Two nearer
//! parallel than that fix nothing, and where every corner found on either
//! where faces of both may meet stands within the tolerance of both, and
//! every curve crossing both crosses them within the tolerance of one
//! place, they graze: a place on one within the tolerance of the other is
//! on both, however far from the line they share. Decided once for the
//! pair, rather than corner by corner: two pairs of corners along the same
//! two arcs, one within the tolerance and the next a hair past it, would
//! make one arc of the two at one end and leave them two at the other
//! (5520267036, 5520167649).

use std::collections::BTreeSet;

use glam::DVec3;

use crate::brep::surface::Surface;
use crate::brep::topology::SurfaceId;

/// How far from each other the normals must stand for their planes to fix a
/// line or a place, in the sine of their angle or the volume they span: well
/// clear of planes the relation of their pair would take for parallel.
const ACROSS: f64 = 1e-3;

/// A corner candidate as [`Planes::grazing`] reads it: where it was found,
/// the surfaces it lies on — those it was found on and those of the curves
/// it was found along — and those curves.
pub(in crate::brep) struct Place {
    pub point: DVec3,
    pub on: Vec<SurfaceId>,
    pub along: Vec<usize>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(in crate::brep) struct Planes {
    /// Each surface's normal and offset, when it is a plane.
    planes: Vec<Option<(DVec3, f64)>>,
    /// The pairs of planes decided to graze.
    grazing: BTreeSet<[SurfaceId; 2]>,
}

impl Planes {
    pub fn of(surfaces: &[Surface]) -> Planes {
        Planes {
            planes: surfaces
                .iter()
                .map(|surface| match surface {
                    Surface::Plane(plane) => Some((plane.normal, plane.offset())),
                    Surface::Cylinder(_) | Surface::Cone(_) => None,
                })
                .collect(),
            grazing: BTreeSet::new(),
        }
    }

    /// The planes, each two of them crossing too near parallel to fix the
    /// line they share decided to graze where, of the corner candidates
    /// `places` within a box `meeting` gives for them — a box faces of both
    /// may meet in — every one on either stands within `eps` of both, and
    /// every two a curve crosses the two at, one on each, within `eps` of
    /// each other: each would be one corner on both. Two whose corners part
    /// by more there cross.
    pub fn grazing(
        mut self,
        meeting: impl Fn([SurfaceId; 2]) -> Vec<[DVec3; 2]>,
        places: &[Place],
        eps: f64,
    ) -> Planes {
        for (one, first) in self.planes.iter().enumerate() {
            for (other, second) in self.planes.iter().enumerate().skip(one + 1) {
                let (Some(first), Some(second)) = (first, second) else {
                    continue;
                };
                if first.0.cross(second.0).length() >= ACROSS {
                    continue;
                }
                let pair = [SurfaceId(one as u32), SurfaceId(other as u32)];
                let boxes = meeting(pair);
                let inside = |point: DVec3| {
                    boxes
                        .iter()
                        .any(|[low, high]| point.cmpge(*low).all() && point.cmple(*high).all())
                };
                let near = |point: DVec3| {
                    [first, second]
                        .iter()
                        .all(|(normal, offset)| (normal.dot(point) - offset).abs() <= eps)
                };
                let met: Vec<&Place> = places
                    .iter()
                    .filter(|place| {
                        place.on.iter().any(|surface| pair.contains(surface)) && inside(place.point)
                    })
                    .collect();
                let only = |place: &Place, own: SurfaceId, theirs: SurfaceId| {
                    place.on.contains(&own) && !place.on.contains(&theirs)
                };
                let one_place = |own: SurfaceId, theirs: SurfaceId| {
                    met.iter()
                        .filter(|place| only(place, own, theirs))
                        .all(|place| {
                            place.along.iter().all(|curve| {
                                let across: Vec<DVec3> = met
                                    .iter()
                                    .filter(|other| {
                                        only(other, theirs, own) && other.along.contains(curve)
                                    })
                                    .map(|other| other.point)
                                    .collect();
                                across.is_empty()
                                    || across.iter().any(|at| at.distance(place.point) <= eps)
                            })
                        })
                };
                if !boxes.is_empty()
                    && met.iter().all(|place| near(place.point))
                    && one_place(pair[0], pair[1])
                    && one_place(pair[1], pair[0])
                {
                    self.grazing.insert(pair);
                }
            }
        }
        self
    }

    /// Whether two planes were decided to graze.
    pub fn graze(&self, one: SurfaceId, other: SurfaceId) -> bool {
        self.grazing.contains(&[one.min(other), one.max(other)])
    }

    pub fn is_plane(&self, surface: SurfaceId) -> bool {
        self.planes
            .get(surface.0 as usize)
            .is_some_and(Option::is_some)
    }

    /// Whether both sets lie on two planes across each other.
    pub fn fix_a_line(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        let shared = self.shared(one, other);
        shared.iter().enumerate().any(|(index, first)| {
            shared[index + 1..]
                .iter()
                .any(|second| first.0.cross(second.0).length() >= ACROSS)
        })
    }

    /// Whether both sets lie on three planes whose normals span space.
    pub fn fix_a_place(&self, one: &[SurfaceId], other: &[SurfaceId]) -> bool {
        self.spanning(&self.shared(one, other)).is_some()
    }

    /// The place three planes of `support` whose normals span space meet at,
    /// of all such triples the one spanning the most; none where no three do,
    /// nor where `support` holds a cylinder that fixes the place as well:
    /// one `held` does not say the planes hold.
    pub fn place(&self, support: &[SurfaceId], held: impl Fn(SurfaceId) -> bool) -> Option<DVec3> {
        let planes = self.shared(support, support);
        if support
            .iter()
            .any(|&surface| !self.is_plane(surface) && !held(surface))
        {
            return None;
        }
        let [one, other, third] = self.spanning(&planes)?;
        let volume = one.0.dot(other.0.cross(third.0));
        Some(
            (other.0.cross(third.0) * one.1
                + third.0.cross(one.0) * other.1
                + one.0.cross(other.0) * third.1)
                / volume,
        )
    }

    /// The line two planes of `support` across each other share, as a point
    /// of it and its direction, of all such pairs the one across the most;
    /// none where no two are across, or where three span space and fix a
    /// place instead.
    pub fn line(&self, support: &[SurfaceId]) -> Option<(DVec3, DVec3)> {
        let planes = self.shared(support, support);
        if self.spanning(&planes).is_some() {
            return None;
        }
        let mut best: Option<(f64, [(DVec3, f64); 2])> = None;
        for first in 0..planes.len() {
            for second in first + 1..planes.len() {
                let across = planes[first].0.cross(planes[second].0).length();
                if across >= ACROSS && best.is_none_or(|(most, _)| across > most) {
                    best = Some((across, [planes[first], planes[second]]));
                }
            }
        }
        let (_, [(one, at_one), (other, at_other)]) = best?;
        let direction = one.cross(other);
        let point = (other.cross(direction) * at_one + direction.cross(one) * at_other)
            / direction.length_squared();
        Some((point, direction.normalize()))
    }

    /// The triple of planes whose normals span the most, when some span.
    fn spanning(&self, planes: &[(DVec3, f64)]) -> Option<[(DVec3, f64); 3]> {
        let mut best: Option<(f64, [(DVec3, f64); 3])> = None;
        for first in 0..planes.len() {
            for second in first + 1..planes.len() {
                for third in second + 1..planes.len() {
                    let triple = [planes[first], planes[second], planes[third]];
                    let volume = triple[0].0.dot(triple[1].0.cross(triple[2].0)).abs();
                    if volume >= ACROSS && best.is_none_or(|(most, _)| volume > most) {
                        best = Some((volume, triple));
                    }
                }
            }
        }
        best.map(|(_, triple)| triple)
    }

    /// The planes both sets lie on, as normals and offsets.
    fn shared(&self, one: &[SurfaceId], other: &[SurfaceId]) -> Vec<(DVec3, f64)> {
        one.iter()
            .filter(|surface| other.contains(surface))
            .filter_map(|surface| self.planes.get(surface.0 as usize).copied().flatten())
            .collect()
    }
}
