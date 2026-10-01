//! How near each other two parallel walls stand at a place of either, and
//! whether no sample is taken there for it.

use glam::DVec3;

use super::APART;
use crate::brep::surface::Cylinder;

/// Two parallel walls, whether they were decided to touch — inside or side
/// by side — and the room no sample of either is taken within.
#[derive(Clone, Copy, Debug)]
pub(in crate::brep::tessellation) struct Gap {
    walls: [Cylinder; 2],
    touch: bool,
    room: f64,
}

impl Gap {
    pub(super) fn of(one: &Cylinder, other: &Cylinder, eps: f64) -> Gap {
        let between = other.origin - one.origin;
        let apart = (between - one.axis * one.axis.dot(between)).length();
        let [near, far] = [(one.radius - other.radius).abs(), one.radius + other.radius];
        Gap {
            walls: [*one, *other],
            touch: (apart - far).abs() <= eps || ((apart - near).abs() <= eps && near > eps),
            room: eps * APART,
        }
    }

    /// Whether the two were decided to touch.
    pub(super) fn touch(&self) -> bool {
        self.touch
    }

    /// The wall of the two that is not `wall`.
    pub(in crate::brep::tessellation) fn other(&self, wall: &Cylinder) -> Cylinder {
        if self.walls[0] == *wall {
            self.walls[1]
        } else {
            self.walls[0]
        }
    }

    /// Whether `point`, on `wall`, is crowded by the other: closer to it than
    /// a fifth of the kernel's tolerance, twice what the rules tell apart,
    /// either side for walls crossing or all but one; on the wrong side at
    /// all, or closer than that on the right one, for walls decided to
    /// touch. The kernel may leave those overlapping by up to its tolerance
    /// round the line they touch along, one poking through the other, and a
    /// place there would put a sample of a cap's arc on the wrong side of
    /// the other; but where they part as decided, a fifth of it is enough,
    /// and the whole of it would withhold a wide arc of two walls nearly
    /// concentric.
    pub(in crate::brep::tessellation) fn crowds(&self, wall: &Cylinder, point: DVec3) -> bool {
        let clear = self.clearance(wall, point);
        if self.touch {
            clear < self.room
        } else {
            clear.abs() < self.room
        }
    }

    /// How far `point`, on `wall`, stands from the other wall: positive on
    /// the side the two were decided to stand on, where they touch. Where
    /// the smaller stands about an axis inside the larger, the two are
    /// sampled on common rays from that axis, and the gap is measured along
    /// the ray through `point`, from the smaller wall out to the larger:
    /// the same sum whichever of the two the place is taken on, so that a
    /// ray is crowded for both or for neither.
    fn clearance(&self, wall: &Cylinder, point: DVec3) -> f64 {
        let [first, second] = self.walls;
        let (outer, inner) = if first.radius >= second.radius {
            (first, second)
        } else {
            (second, first)
        };
        let axis = outer.axis;
        let flat = |vector: DVec3| vector - axis * axis.dot(vector);
        let offset = flat(inner.origin - outer.origin);
        let outside = offset.length_squared() - outer.radius * outer.radius;
        if outside < 0.0 {
            if *wall == inner {
                let way = flat(point - inner.origin).normalize_or_zero();
                let along = offset.dot(way);
                return -along + (along * along - outside).sqrt() - inner.radius;
            }
            return flat(point - inner.origin).length() - inner.radius;
        }
        let other = if *wall == first { second } else { first };
        other.distance(point)
    }
}
