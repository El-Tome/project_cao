//! The full round (#698): two edges either side of a face rounded at the
//! radius whose set-backs meet across it. The two rounds touch the face
//! along one line from the same side, so their centres are one point of
//! every section: one round about one axis, from one edge's other face over
//! to the other's, where the face was.

use glam::{DVec2, DVec3};

use crate::body::exact::wedge::{Prism, Wedge};
use crate::brep::{Declined, Scale};
use crate::profile::{Contour, Run};

impl Wedge {
    /// The prism the edge and a `later` one, whose set-backs for `radius`
    /// meet across the face they share, are rounded off with as one: raised
    /// along the edge over a section square to it whose first two runs are
    /// the round about the centre both rounds share, from where it touches
    /// the later edge's other face to where it touches the shared face, then
    /// on to where it touches this edge's other face; then square off that
    /// face to the margin outside it, round both edges that margin off the
    /// three faces, as [`Wedge::rim`] closes each, and square back to the
    /// start. Declined `Unsupported` where the two edges share no face.
    pub(in crate::body::exact) fn rounded_with(
        &self,
        later: &Wedge,
        radius: f64,
        scale: Scale,
        bounds: (DVec3, DVec3),
    ) -> Result<Prism, Declined> {
        let shared = |wedge: &Wedge| {
            let mut sides = wedge.faces.iter();
            sides.position(|face| self.faces.contains(face) && later.faces.contains(face))
        };
        let (Some(own), Some(theirs)) = (shared(self), shared(later)) else {
            return Err(Declined::Unsupported);
        };
        let margin = [self, later]
            .map(|wedge| {
                let (across, facing) = wedge.square();
                wedge.margin(across, facing, radius / wedge.half_tangent(), scale)
            })
            .into_iter()
            .fold(0.0, f64::max);
        let [other, beyond] = [self.outward[1 - own], later.outward[1 - theirs]];
        let centre = self.into[1 - own] * radius / self.half_tangent() - other * radius;
        let touching = |outward: DVec3| centre + outward * radius;
        let mut own_rim = self.rim(margin);
        if own == 0 {
            own_rim.reverse();
        }
        let mut later_rim = later.rim(margin);
        if theirs == 1 {
            later_rim.reverse();
        }
        let offset = later.point - self.point;
        let start = touching(beyond);
        let end = touching(other);
        let mut corners = vec![
            start,
            touching(self.outward[own]),
            end,
            end + other * margin,
        ];
        corners.extend(own_rim);
        corners.extend(later_rim.into_iter().map(|at| at + offset));
        corners.push(start + beyond * margin);
        let (across, _) = self.square();
        let (frame, travel) = self.raised_along(across, margin, bounds);
        let flat = |at: DVec3| DVec2::new(at.dot(frame.u), at.dot(frame.v));
        let corners: Vec<DVec2> = corners.into_iter().map(flat).collect();
        let about = flat(centre);
        let round = |from: DVec2, to: DVec2| {
            let (start, end) = (from - about, to - about);
            Run::Round {
                center: about,
                turn: start.perp_dot(end).atan2(start.dot(end)),
            }
        };
        let mut runs = vec![Run::Straight; corners.len()];
        runs[0] = round(corners[0], corners[1]);
        runs[1] = round(corners[1], corners[2]);
        Ok((Contour { corners, runs }, frame, travel))
    }
}
