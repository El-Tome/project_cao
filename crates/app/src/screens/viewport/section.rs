//! The section view: a plane through the part, and how the matter on each of
//! its sides is drawn. A way of looking — nothing of it enters the document.

use cao_render::MatterInFront;
use cao_sketch::WorkPlane;
use cao_solid::Body;
use glam::DVec3;

/// The plane moved `offset` along its normal; behind is against the normal,
/// in front along it, whichever side the eye is on.
pub(crate) struct Section {
    pub plane: WorkPlane,
    pub offset: f64,
    pub behind: MatterInFront,
    pub in_front: MatterInFront,
    waiting_for_a_face: bool,
    held: Option<Held>,
}

/// Where the handle was taken: the line it slides along, how far along it the
/// pointer was, the distance the section read then, and how far the part
/// reaches either way along the normal.
struct Held {
    line: (DVec3, DVec3),
    taken_at: f64,
    offset: f64,
    reach: (f64, f64),
}

/// A ray closer than this to the normal runs nearly along the handle's line,
/// and the smallest move of the pointer would throw the cut across the part.
const GRAZING_DEGREES: f64 = 5.0;

impl Section {
    pub(crate) fn new(plane: WorkPlane) -> Self {
        Self {
            plane,
            offset: 0.0,
            behind: MatterInFront::Opaque,
            in_front: MatterInFront::Hidden,
            waiting_for_a_face: false,
            held: None,
        }
    }

    /// A section along `plane` through the middle of the part.
    pub(crate) fn through_the_middle(plane: WorkPlane, body: &Body) -> Self {
        let mut section = Self::new(plane);
        section.offset = to_the_middle(plane, body);
        section
    }

    pub(crate) fn invert(&mut self) {
        std::mem::swap(&mut self.behind, &mut self.in_front);
    }

    pub(crate) fn next_behind(&mut self) {
        self.behind = self.behind.next();
    }

    pub(crate) fn next_in_front(&mut self) {
        self.in_front = self.in_front.next();
    }

    pub(crate) fn wait_for_a_face(&mut self) {
        self.waiting_for_a_face = true;
    }

    pub(crate) fn is_waiting_for_a_face(&self) -> bool {
        self.waiting_for_a_face
    }

    pub(crate) fn choose_origin_plane(&mut self, plane: WorkPlane, body: &Body) {
        self.plane = plane;
        self.offset = to_the_middle(plane, body);
        self.waiting_for_a_face = false;
    }

    pub(crate) fn choose_face(&mut self, plane: WorkPlane) {
        self.plane = plane;
        self.offset = 0.0;
        self.waiting_for_a_face = false;
    }

    pub(crate) fn type_distance(&mut self, millimetres: f64, millimetres_per_unit: f64) {
        self.offset = millimetres / millimetres_per_unit;
    }

    pub(crate) fn distance_in_millimetres(&self, millimetres_per_unit: f64) -> f64 {
        self.offset * millimetres_per_unit
    }

    pub(crate) fn distance_text(&self, millimetres_per_unit: f64) -> String {
        crate::wording::dimension::short(self.distance_in_millimetres(millimetres_per_unit))
    }

    /// The plane the part is cut along: the one chosen, moved by the distance.
    pub(crate) fn cutting_plane(&self) -> WorkPlane {
        WorkPlane {
            origin: self.plane.origin + self.plane.normal() * self.offset,
            ..self.plane
        }
    }

    /// Where the handle stands: on the cutting plane, over the middle of the
    /// part.
    pub(crate) fn handle(&self, body: &Body) -> DVec3 {
        let cut = self.cutting_plane();
        let normal = cut.normal();
        let middle = middle(body, cut);
        middle - normal * normal.dot(middle - cut.origin)
    }

    /// Takes hold of the handle when the ray passes within `reach` of it, and
    /// does not graze the normal.
    pub(crate) fn grab(
        &mut self,
        body: &Body,
        origin: DVec3,
        direction: DVec3,
        reach: f64,
    ) -> bool {
        let handle = self.handle(body);
        let direction = direction.normalize_or_zero();
        let normal = self.plane.normal();
        if grazes(normal, direction) {
            return false;
        }
        let along = (handle - origin).dot(direction);
        if (origin + direction * along - handle).length() > reach {
            return false;
        }
        let line = (handle, normal);
        let reach = extent(body, self.plane);
        self.held = closest_along(line, (origin, direction)).map(|taken_at| Held {
            line,
            taken_at,
            offset: self.offset,
            reach,
        });
        self.held.is_some()
    }

    pub(crate) fn is_held(&self) -> bool {
        self.held.is_some()
    }

    /// Moves the cut by as much as the pointer moved along the normal since the
    /// handle was taken, no further than the part reaches.
    pub(crate) fn pull(&mut self, origin: DVec3, direction: DVec3) {
        let Some(held) = &self.held else {
            return;
        };
        if grazes(held.line.1, direction.normalize_or_zero()) {
            return;
        }
        if let Some(at) = closest_along(held.line, (origin, direction)) {
            let (low, high) = held.reach;
            self.offset = (held.offset + at - held.taken_at).clamp(low, high);
        }
    }

    pub(crate) fn let_go(&mut self) {
        self.held = None;
    }
}

/// How far along its normal `plane` has to move to pass the middle of the part.
pub(crate) fn to_the_middle(plane: WorkPlane, body: &Body) -> f64 {
    plane.normal().dot(middle(body, plane) - plane.origin)
}

fn grazes(normal: DVec3, direction: DVec3) -> bool {
    direction.dot(normal).abs() > GRAZING_DEGREES.to_radians().cos()
}

/// How far the part reaches along the normal of `plane`, from it, nearest and
/// furthest.
fn extent(body: &Body, plane: WorkPlane) -> (f64, f64) {
    let Some((low, high)) = body.bounds() else {
        return (0.0, 0.0);
    };
    let normal = plane.normal();
    (0..8)
        .map(|corner| {
            let pick = glam::BVec3::new(corner & 1 != 0, corner & 2 != 0, corner & 4 != 0);
            normal.dot(DVec3::select(pick, high, low) - plane.origin)
        })
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(near, far), at| {
            (near.min(at), far.max(at))
        })
}

/// The middle of the part's box, or the plane's own origin when it holds none.
fn middle(body: &Body, plane: WorkPlane) -> DVec3 {
    body.bounds()
        .map_or(plane.origin, |(low, high)| (low + high) / 2.0)
}

/// How far along a line, from its origin and in units of its direction, lies
/// the point nearest a ray; `None` when the two run side by side.
pub(crate) fn closest_along(line: (DVec3, DVec3), ray: (DVec3, DVec3)) -> Option<f64> {
    let ((from, along), (origin, towards)) = (line, ray);
    let (a, b, c) = (along.dot(along), along.dot(towards), towards.dot(towards));
    let between = from - origin;
    let (d, e) = (along.dot(between), towards.dot(between));
    let denominator = a * c - b * b;
    (denominator > 1e-12 * a * c).then(|| (b * e - c * d) / denominator)
}

#[cfg(test)]
mod tests;
