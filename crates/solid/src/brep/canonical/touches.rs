//! Decision 2's moves onto a touch. Two surfaces, one of each operand, a
//! hair from touching are made to touch: two perpendicular cylinders,
//! inside, outside or at a node; a plane and a cylinder, or two parallel
//! cylinders, decided to touch along a line. The move is the surface's,
//! made here before any curve or corner is found on either, so that the
//! curve they share and the corners on it stand on both.
//!
//! Only the surface the second operand alone carries moves: a cylinder
//! carrying the first operand's corners would leave them on its old wall.
//! Every touch a surface has is read before it moves, and a move is made
//! only where no other touch of the surface ends further from exact than it
//! was: settled onto one touch, a cylinder in two would otherwise break the
//! other — but a cylinder grown onto a node may slide along its partner's
//! axis, which leaves the node where it is, to keep a plane it touched. A
//! move may run along an exact touch, but not on a wall its operand drew
//! corners on, which would leave them behind the line of
//! touch it takes along: such a slide is made before, on the operand, with
//! its corners (`combine/slid.rs`); nor take such a wall, a plane, further
//! than the tolerance off the line it crosses another plane along. A
//! cylinder touching two parallel planes
//! on opposite sides is moved midway between them, its radius half their
//! gap, which makes both exact; so is one touching one of them and standing
//! within decision 8's hair of the other, which left touching the one
//! stands a hair or two off the other, across a skin no ray tells the side
//! of.
//! A touch counts where faces on the two surfaces stand in boxes that meet:
//! a plane whose face is far away touches nothing.

use crate::brep::meet::{moved, moved_instead};
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::SurfaceId;

use super::surfaces::Surfaces;

/// Under this share of the tolerance, a touch a move leaves is as near
/// exact as it was.
const ROUNDING: f64 = 1e-3;

impl Surfaces {
    /// The moves onto a touch, of pairs of a surface `carried` by each
    /// operand whose faces stand `near` each other; a surface the second
    /// operand's corners lie on is `cornered`. Whether each surface was
    /// moved.
    pub fn snapped(
        &mut self,
        carried: impl Fn(usize, SurfaceId) -> bool,
        near: impl Fn(SurfaceId, SurfaceId) -> bool,
        cornered: impl Fn(SurfaceId) -> bool,
        scale: Scale,
    ) -> Vec<bool> {
        let count = self.list.len();
        let id = |rank: usize| SurfaceId(rank as u32);
        let own = |rank: usize| carried(1, id(rank)) && !carried(0, id(rank));
        let close = |one: usize, other: usize| near(id(one), id(other));
        let pinned = |rank: usize| cornered(id(rank));
        let mut moved = vec![false; count];
        for rank in (0..count).filter(|&rank| own(rank)) {
            let Some((planes, cylinder)) = self.between(rank, close, scale) else {
                continue;
            };
            let surface = Surface::Cylinder(cylinder);
            if self.keeps(rank, &surface, &planes, (close, pinned(rank)), scale) {
                self.list[rank] = surface;
                moved[rank] = true;
            }
        }
        let pairs: Vec<[usize; 2]> = (0..count)
            .flat_map(|one| (one + 1..count).map(move |other| [one, other]))
            .filter(|&[one, other]| {
                let [first, second] = [id(one), id(other)];
                (carried(0, first) && carried(1, second) || carried(1, first) && carried(0, second))
                    && close(one, other)
            })
            .collect();
        for pair in pairs {
            let Some((shifted, surface)) = self.onto(pair, own, scale) else {
                continue;
            };
            let partner = self.list[pair[usize::from(pair[0] == shifted)]];
            let kept = std::iter::once(surface)
                .chain(along_the_partner(&self.list[shifted], &surface, &partner))
                .find(|surface| {
                    self.keeps(shifted, surface, &pair, (close, pinned(shifted)), scale)
                });
            if let Some(surface) = kept {
                self.list[shifted] = surface;
                moved[shifted] = true;
            }
        }
        moved
    }

    /// Where the pair moves which of its two surfaces onto their touch: the
    /// one the second operand alone carries.
    fn onto(
        &self,
        [one, other]: [usize; 2],
        own: impl Fn(usize) -> bool,
        scale: Scale,
    ) -> Option<(usize, Surface)> {
        let [first, second] = [&self.list[one], &self.list[other]];
        match relation(first, second, scale) {
            Relation::Meet(_) => {
                let (first, second) = match (first, second) {
                    (Surface::Cylinder(first), Surface::Cylinder(second)) => (first, second),
                    (Surface::Plane(_) | Surface::Cone(_), _)
                    | (_, Surface::Plane(_) | Surface::Cone(_)) => return None,
                };
                let (rank, cylinder) = moved(first, second, scale)?;
                let (rank, cylinder) = if own([one, other][rank]) {
                    (rank, cylinder)
                } else {
                    moved_instead(first, second, scale)?
                };
                let shifted = [one, other][rank];
                own(shifted).then_some((shifted, Surface::Cylinder(cylinder)))
            }
            Relation::Tangent(_) if own(other) => {
                touching(first, second).map(|moved| (other, moved))
            }
            Relation::Tangent(_) if own(one) => touching(second, first).map(|moved| (one, moved)),
            Relation::Apart
            | Relation::Same { .. }
            | Relation::Line(_)
            | Relation::Lines(_)
            | Relation::Tangent(_)
            | Relation::Circle(_)
            | Relation::Rulings { .. }
            | Relation::Apex(_)
            | Relation::Unsupported => None,
        }
    }

    /// A cylinder touching two parallel planes on opposite sides, or one of
    /// them and within decision 8's hair of the other, those two, and the
    /// cylinder moved midway between them, its radius half their gap.
    fn between(
        &self,
        rank: usize,
        near: impl Fn(usize, usize) -> bool,
        scale: Scale,
    ) -> Option<([usize; 2], Cylinder)> {
        let cylinder = match self.list[rank] {
            Surface::Cylinder(cylinder) => cylinder,
            Surface::Plane(_) | Surface::Cone(_) => return None,
        };
        let planes: Vec<(usize, Plane, bool)> = (0..self.list.len())
            .filter(|&other| other != rank && near(rank, other))
            .filter_map(|other| match self.list[other] {
                Surface::Plane(plane)
                    if plane.normal.dot(cylinder.axis).abs() * 2.0 * scale.reach()
                        <= scale.eps()
                        && (plane.distance(cylinder.origin).abs() - cylinder.radius).abs()
                            <= Scale::HAIR * scale.eps() =>
                {
                    let touches = matches!(
                        relation(&self.list[other], &self.list[rank], scale),
                        Relation::Tangent(_)
                    );
                    Some((other, plane, touches))
                }
                Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) => None,
            })
            .collect();
        for (index, &(one, plane, touches)) in planes.iter().enumerate() {
            for &(other, facing, touched) in &planes[index + 1..] {
                let normal = plane.normal;
                if normal.cross(facing.normal).length() * 2.0 * scale.reach() > scale.eps()
                    || !touches && !touched
                {
                    continue;
                }
                let far = facing.offset() * normal.dot(facing.normal).signum();
                let level = normal.dot(cylinder.origin);
                if (level - plane.offset()) * (level - far) >= 0.0 {
                    continue;
                }
                let middle = (plane.offset() + far) / 2.0;
                let moved = Cylinder::about(
                    cylinder.origin + normal * (middle - level),
                    cylinder.axis,
                    (plane.offset() - far).abs() / 2.0,
                );
                return Some(([one, other], moved));
            }
        }
        None
    }

    /// Whether `surface`, put for the one of rank `rank`, leaves every touch
    /// it has with a surface near it but `besides` as near exact as it was;
    /// and, where corners of its operand lie on it, every exact touch where
    /// it was: slid along it, the line of touch would leave behind the
    /// corners drawn on it. Nor, then, may a plane cross another a tolerance
    /// further off the line they crossed along: moved a hair along its
    /// normal, a plane takes the line it crosses another along at a grazing
    /// angle that hair over the sine of the angle, away from the corners on
    /// it — a hundredth of a degree's turn, moved onto a round it grazes,
    /// put the line its two ends meet along ninety microns away (5365230254).
    fn keeps(
        &self,
        rank: usize,
        surface: &Surface,
        besides: &[usize],
        (near, pinned): (impl Fn(usize, usize) -> bool, bool),
        scale: Scale,
    ) -> bool {
        let room = ROUNDING * scale.eps();
        let before = &self.list[rank];
        (0..self.list.len())
            .filter(|&other| other != rank && !besides.contains(&other) && near(rank, other))
            .all(|other| {
                let other = &self.list[other];
                if pinned && crossed_elsewhere(before, surface, other, scale) {
                    return false;
                }
                match off(before, other, scale) {
                    Some(gap) => {
                        off(surface, other, scale).is_some_and(|after| after <= gap + room)
                            && !(pinned
                                && gap <= room
                                && moved_along(before, surface, other, scale) > room)
                    }
                    None => true,
                }
            })
    }
}

/// A cylinder given its partner's radius at a node of two of one radius,
/// slid either way along the partner's axis by what its radius grew: the
/// node stands wherever along that axis the two meet, and the slide keeps
/// the cylinder touching a plane square to the axis as it touched it — a
/// post resting on a shaft's end, whose wall crosses the shaft at a node
/// but for a hair of radius (533609727). None for any other move.
fn along_the_partner(before: &Surface, after: &Surface, partner: &Surface) -> Vec<Surface> {
    let (before, after, partner) = match (before, after, partner) {
        (Surface::Cylinder(before), Surface::Cylinder(after), Surface::Cylinder(partner)) => {
            (before, after, partner)
        }
        (Surface::Plane(_) | Surface::Cone(_), _, _)
        | (_, Surface::Plane(_) | Surface::Cone(_), _)
        | (_, _, Surface::Plane(_) | Surface::Cone(_)) => {
            return Vec::new();
        }
    };
    let grown = after.radius - before.radius;
    if grown == 0.0 {
        return Vec::new();
    }
    [grown, -grown]
        .map(|by| {
            Surface::Cylinder(Cylinder::about(
                after.origin + partner.axis * by,
                after.axis,
                after.radius,
            ))
        })
        .to_vec()
}

/// How far two surfaces decided to touch stand from touching exactly; none
/// where their pair is no touch. A cone touches nothing along a line.
fn off(one: &Surface, other: &Surface, scale: Scale) -> Option<f64> {
    match relation(one, other, scale) {
        Relation::Tangent(_) => match (one, other) {
            (Surface::Plane(plane), Surface::Cylinder(cylinder))
            | (Surface::Cylinder(cylinder), Surface::Plane(plane)) => {
                Some((plane.distance(cylinder.origin).abs() - cylinder.radius).abs())
            }
            (Surface::Cylinder(one), Surface::Cylinder(other)) => {
                let between = other.origin - one.origin;
                let across = (between - one.axis * one.axis.dot(between)).length();
                let outside = across - (one.radius + other.radius);
                let inside = (one.radius - other.radius).abs() - across;
                Some(outside.abs().min(inside.abs()))
            }
            (Surface::Plane(_), Surface::Plane(_)) => Some(0.0),
            (Surface::Cone(_), _) | (_, Surface::Cone(_)) => None,
        },
        Relation::Meet(meeting) if !meeting.nodes.is_empty() || meeting.contact.is_some() => {
            let (one, other) = match (one, other) {
                (Surface::Cylinder(one), Surface::Cylinder(other)) => (one, other),
                (Surface::Plane(_) | Surface::Cone(_), _)
                | (_, Surface::Plane(_) | Surface::Cone(_)) => return None,
            };
            Some(moved(one, other, scale).map_or(0.0, |(rank, to)| {
                let from = [one, other][rank];
                from.origin.distance(to.origin) + (from.radius - to.radius).abs()
            }))
        }
        Relation::Apart
        | Relation::Same { .. }
        | Relation::Line(_)
        | Relation::Lines(_)
        | Relation::Circle(_)
        | Relation::Meet(_)
        | Relation::Rulings { .. }
        | Relation::Apex(_)
        | Relation::Unsupported => None,
    }
}

/// Whether two planes crossing along a line, the first put for `before`,
/// cross along one standing further than the tolerance from it.
fn crossed_elsewhere(before: &Surface, after: &Surface, other: &Surface, scale: Scale) -> bool {
    match (
        relation(before, other, scale),
        relation(after, other, scale),
    ) {
        (Relation::Line(was), Relation::Line(is)) => {
            let from = is.origin - was.origin;
            (from - was.direction * was.direction.dot(from)).length() > scale.eps()
        }
        _ => false,
    }
}

/// How far the place two surfaces touch at moves when the first is put
/// for `before`: the line they touch along, or the points two
/// perpendicular cylinders touch at.
fn moved_along(before: &Surface, after: &Surface, other: &Surface, scale: Scale) -> f64 {
    let places = |surface: &Surface| match relation(surface, other, scale) {
        Relation::Tangent(line) => vec![line.origin],
        found => found.points(),
    };
    let (from, to) = (places(before), places(after));
    if from.len() != to.len() {
        return f64::INFINITY;
    }
    from.iter()
        .zip(&to)
        .map(|(one, other)| one.distance(*other))
        .fold(0.0, f64::max)
}

/// `moving` moved onto the line it was decided to touch `fixed` along: a
/// cylinder along a plane's normal, a plane along its own, a cylinder
/// towards or away from a parallel one, by the gap the relation found within
/// the tolerance. None where the touch is exact already.
fn touching(fixed: &Surface, moving: &Surface) -> Option<Surface> {
    let moved = match (fixed, moving) {
        (Surface::Plane(plane), Surface::Cylinder(cylinder)) => {
            let away = plane.distance(cylinder.origin);
            let gap = away.abs() - cylinder.radius;
            Surface::Cylinder(Cylinder::about(
                cylinder.origin - plane.normal * away.signum() * gap,
                cylinder.axis,
                cylinder.radius,
            ))
        }
        (Surface::Cylinder(cylinder), Surface::Plane(plane)) => {
            let away = plane.distance(cylinder.origin);
            let gap = away.abs() - cylinder.radius;
            let (plane, _) = Plane::through(
                plane.origin + plane.normal * away.signum() * gap,
                plane.normal,
            );
            Surface::Plane(plane)
        }
        (Surface::Cylinder(kept), Surface::Cylinder(cylinder)) => {
            let between = cylinder.origin - kept.origin;
            let across = between - kept.axis * kept.axis.dot(between);
            let distance = across.length();
            if distance == 0.0 {
                return None;
            }
            let outside = distance - (kept.radius + cylinder.radius);
            let inside = (kept.radius - cylinder.radius).abs() - distance;
            let by = if outside.abs() <= inside.abs() {
                -outside
            } else {
                inside
            };
            Surface::Cylinder(Cylinder::about(
                cylinder.origin + across / distance * by,
                cylinder.axis,
                cylinder.radius,
            ))
        }
        (Surface::Plane(_), Surface::Plane(_)) | (Surface::Cone(_), _) | (_, Surface::Cone(_)) => {
            return None;
        }
    };
    (moved != *moving).then_some(moved)
}
