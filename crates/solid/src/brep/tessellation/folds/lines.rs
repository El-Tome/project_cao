//! Two faces bounding the sliver beside a line two walls a hair off parallel
//! cross along, cut alike there (#706).
//!
//! Sampled at its ends alone, the line leans a hair one way in one wall's
//! chart and the other way in the other's, and the sweep cuts the cell
//! beside it from whichever end comes first: one face rests its chord on a
//! sample of its own rim, the other on one of the curve the two share at the
//! line's foot, and both are drawn, lying on each other across the line.
//! Each face's triangle on the chord is cut the other way, where it is not
//! already, to take the sample beside that foot along the shared curve: the
//! two then stand on the same three samples, a skin of no thickness
//! ([`super::super::skins`]).

use super::{Drawn, Folds};
use crate::brep::curve::{Curve, Wall};
use crate::brep::relation::BAND;
use crate::brep::tessellation::sampling::Samples;
use crate::brep::topology::{Body, EdgeId, FaceId};

/// Cuts alike, beside every line two cylinders leaning within #552's band
/// cross along and sampled at its ends alone, the two faces it bounds where
/// they also share a curve leaving one of its ends.
pub(in crate::brep::tessellation) fn cut_alike(
    drawn: &mut [Drawn],
    body: &Body,
    samples: &Samples,
    eps: f64,
    tolerance: f64,
) {
    let mut folds = Folds::of(drawn, samples, eps, tolerance);
    for id in body.edge_ids() {
        let Some(([foot, head], beside, faces)) = line(body, samples, id) else {
            continue;
        };
        for face in faces {
            let Some(at) = folds.drawn.iter().position(|drawn| drawn.face == face) else {
                continue;
            };
            let drawn = &folds.drawn[at];
            let sample = |corner: usize| drawn.outline.samples[corner];
            let resting = |corners: &[usize; 3]| {
                corners.iter().any(|c| sample(*c) == foot)
                    && corners.iter().any(|c| sample(*c) == head)
            };
            let Some(chord) = drawn.triangles.iter().position(resting) else {
                continue;
            };
            let corners = drawn.triangles[chord];
            let third = corners
                .into_iter()
                .find(|c| sample(*c) != foot && sample(*c) != head);
            let Some(third) = third.filter(|c| sample(*c) != beside) else {
                continue;
            };
            let base = corners.into_iter().find(|c| sample(*c) == foot);
            let partner = drawn.triangles.iter().position(|other| {
                other.contains(&third)
                    && base.is_some_and(|base| other.contains(&base))
                    && other.iter().any(|c| sample(*c) == beside)
            });
            if let Some(partner) = partner {
                folds.flip((at, chord), (at, partner));
            }
        }
    }
}

/// The ends of edge `id`, foot first, the sample beside the foot along a
/// curve leaving it, and the two faces the edge and that curve both bound,
/// when it is a line two cylinders leaning within the band cross along,
/// sampled at its ends alone.
fn line(body: &Body, samples: &Samples, id: EdgeId) -> Option<([usize; 2], usize, [FaceId; 2])> {
    let edge = body.edge(id);
    let Curve::Meet(meet) = body.curve(edge.curve) else {
        return None;
    };
    let (Wall::Cylinder(one), Wall::Cylinder(other)) = (meet.first, meet.second) else {
        return None;
    };
    let &[from, to] = samples.edge(id) else {
        return None;
    };
    if one.axis.cross(other.axis).length() >= BAND {
        return None;
    }
    let [first, second] = match body.uses(id).as_slice() {
        [(one, _), (other, _)] if one != other => [*one, *other],
        _ => return None,
    };
    let bounds = |edge: EdgeId| {
        let uses = body.uses(edge);
        uses.iter().any(|(face, _)| *face == first) && uses.iter().any(|(face, _)| *face == second)
    };
    let vertices = edge.ends?;
    vertices.into_iter().find_map(|vertex| {
        let point = body.vertex(vertex).point;
        let foot = [from, to]
            .into_iter()
            .find(|sample| samples.point(*sample) == point)?;
        let head = if foot == from { to } else { from };
        body.edge_ids()
            .filter(|curve| *curve != id && bounds(*curve))
            .find_map(|curve| {
                let ends = body.edge(curve).ends?;
                let taken = samples.edge(curve);
                let beside = if ends[0] == vertex {
                    taken.get(1)
                } else if ends[1] == vertex {
                    taken.len().checked_sub(2).and_then(|at| taken.get(at))
                } else {
                    None
                }?;
                Some(([foot, head], *beside, [first, second]))
            })
    })
}
