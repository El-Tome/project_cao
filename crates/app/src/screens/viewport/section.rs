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
/// pointer was, and the distance the section read then.
struct Held {
    line: (DVec3, DVec3),
    taken_at: f64,
    offset: f64,
}

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

    pub(crate) fn choose_plane(&mut self, plane: WorkPlane) {
        self.plane = plane;
        self.waiting_for_a_face = false;
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

    /// Takes hold of the handle when the ray passes within `reach` of it.
    pub(crate) fn grab(
        &mut self,
        body: &Body,
        origin: DVec3,
        direction: DVec3,
        reach: f64,
    ) -> bool {
        let handle = self.handle(body);
        let direction = direction.normalize_or_zero();
        let along = (handle - origin).dot(direction);
        if (origin + direction * along - handle).length() > reach {
            return false;
        }
        let line = (handle, self.plane.normal());
        self.held = closest_along(line, (origin, direction)).map(|taken_at| Held {
            line,
            taken_at,
            offset: self.offset,
        });
        self.held.is_some()
    }

    pub(crate) fn is_held(&self) -> bool {
        self.held.is_some()
    }

    /// Moves the cut by as much as the pointer moved along the normal since the
    /// handle was taken.
    pub(crate) fn pull(&mut self, origin: DVec3, direction: DVec3) {
        let Some(held) = &self.held else {
            return;
        };
        if let Some(at) = closest_along(held.line, (origin, direction)) {
            self.offset = held.offset + at - held.taken_at;
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
