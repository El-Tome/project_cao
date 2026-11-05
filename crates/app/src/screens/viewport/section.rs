//! The section view: a plane through the part, and how the matter on each of
//! its sides is drawn. A way of looking — nothing of it enters the document.

use cao_render::MatterInFront;
use cao_sketch::WorkPlane;

/// The plane moved `offset` along its normal; behind is against the normal,
/// in front along it, whichever side the eye is on.
pub(crate) struct Section {
    pub plane: WorkPlane,
    pub offset: f64,
    pub behind: MatterInFront,
    pub in_front: MatterInFront,
}

// The section panel, still to come, is what opens and inverts it.
#[cfg_attr(not(test), expect(dead_code))]
impl Section {
    pub(crate) fn new(plane: WorkPlane) -> Self {
        Self {
            plane,
            offset: 0.0,
            behind: MatterInFront::Opaque,
            in_front: MatterInFront::Hidden,
        }
    }

    pub(crate) fn invert(&mut self) {
        std::mem::swap(&mut self.behind, &mut self.in_front);
    }
}
