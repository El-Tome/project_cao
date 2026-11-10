//! The triangle three chamfers at a convex corner are closed with (#699):
//! its corners where the set-backs of each two of the corner's edges cross on
//! the face the two share, the corner cut off flat through them, across the
//! point the three flats would otherwise meet at.

use glam::{DVec2, DVec3};

use super::Prism;
use super::closing::Corner;
use crate::body::exact::Exact;
use crate::brep::Declined;
use crate::profile::{Contour, Frame};

/// How much wider than the triangle the prism that cuts the corner off
/// stands, about the triangle's middle: the point the three flats meet at
/// stands over the triangle at a square corner, and off its middle at one
/// that is not.
const SPREAD: f64 = 3.0;

/// How far past the corner that prism reaches, as a share of how high the
/// corner stands over the triangle.
const PAST: f64 = 0.125;

/// The body with the corner cut off flat through the crossings by the prism
/// [`across`] raises, the prism's floor the face it lays. Declined
/// `Unsupported` where the cut left anything but that face, every corner of
/// it a corner of one of the three `flats`, as [`Exact::cut_along`] reads it.
pub(in crate::body::exact) fn cut_across(
    body: &Exact,
    corner: &Corner,
    flats: [u32; 3],
) -> Result<Exact, Declined> {
    let (outline, frame, travel) = across(corner.crossings, corner.at);
    let tool = Exact::raised(&outline, &[], frame, travel)?;
    let cut = body.cut_by(&Exact { next: 1, ..tool })?;
    if cut.cut_along(&[], body, 1, &flats, &[]) {
        Ok(cut)
    } else {
        Err(Declined::Unsupported)
    }
}

/// The prism a corner is cut off flat with through `crossings`: raised from
/// their plane toward `corner`, over the triangle they make spread
/// [`SPREAD`] times about its middle, to [`PAST`] past the corner. Its floor
/// lies on that plane; what stands on the corner's side of it near the
/// corner is the point the three flats meet at.
fn across(crossings: [DVec3; 3], corner: DVec3) -> Prism {
    let [first, second, third] = crossings;
    let middle = (first + second + third) / 3.0;
    let u = (second - first).normalize();
    let normal = u.cross(third - first).normalize();
    let v = normal.cross(u);
    let outline = crossings.map(|at| {
        let off = (at - middle) * SPREAD;
        DVec2::new(off.dot(u), off.dot(v))
    });
    let frame = Frame {
        origin: middle,
        u,
        v,
    };
    let travel = normal * normal.dot(corner - middle) * (1.0 + PAST);
    (Contour::straight(outline.to_vec()), frame, travel)
}
