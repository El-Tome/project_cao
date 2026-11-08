//! The prism a chamfer is cut with, raised along the edge over a section
//! that holds the corner the chamfer takes, or joined with, over the corner
//! it fills.

use glam::{DVec2, DVec3};

use crate::body::exact::wedge::{Prism, Side, Wedge};
use crate::brep::Scale;
use crate::profile::Contour;

impl Wedge {
    /// The prism, raised along the edge: in a section square to it, the
    /// corner between the chamfer's plane and the edge's two faces, each
    /// face pushed out by the margin, or by less where the chamfer's plane
    /// lies so near it that its run would end further than the distance, or
    /// the margin, past the set-back; capped as [`Side::top`] caps it. Its
    /// first run lies on the chamfer's plane. Carried over the body's box
    /// along the edge's line and a margin past either end. A fill's section
    /// runs on the chamfer's plane from set-back to set-back and no further,
    /// as [`Wedge::squared_off`] draws it: carried into the matter past a
    /// set-back, its run would come out past a face whose far end stands
    /// within the margin of it. None where a face lies within twice `Scale::HAIR`
    /// tolerances of flush with the chamfer's plane, too near to cut or fill
    /// honestly.
    pub(super) fn prism(
        &self,
        distance: f64,
        scale: Scale,
        bounds: (DVec3, DVec3),
    ) -> Option<Prism> {
        let [first, second] = self.outward;
        let facing = (first + second).normalize();
        let [on_first, on_second] = self.into.map(|into| into * distance);
        let across = (on_first - on_second).normalize();
        let height = -facing.dot(on_first);
        let margin = self.margin(across, facing, distance, scale);
        let flush = 2.0 * Scale::HAIR * scale.eps();
        let side = |normal: DVec3| {
            let (a, b) = (normal.dot(across), normal.dot(facing));
            let offset = margin.min(distance.max(margin) * a.abs());
            (offset >= flush).then_some(Side { a, b, offset })
        };
        let (first, second) = (side(first)?, side(second)?);
        if self.inwards {
            return Some(self.squared_off(distance, None, scale, bounds));
        }
        let mut section = vec![second.at(-height), first.at(-height)];
        section.extend(Side::top(first, second, margin));
        let (frame, travel) = self.raised_along(across, margin, bounds);
        let up = facing.dot(frame.v);
        let outline = Contour::straight(
            section
                .into_iter()
                .map(|at| DVec2::new(at.x, at.y * up))
                .collect(),
        );
        Some((outline, frame, travel))
    }
}
