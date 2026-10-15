//! The faces of a contour turned part of the way: a corner where the turn
//! opens and one where it closes for each corner of the profile, the arc
//! between them, and the edges left on the two ends — straight for a
//! straight piece, a meridian arc for an arc of the profile (#576).

use glam::DVec2;

use super::super::Placed;
use super::Swept;
use super::round::{self, Round};
use crate::brep::laying::{Laying, reversed};
use crate::brep::piece::Named;
use crate::brep::topology::{Coedge, Face, SurfaceId, VertexId};

/// Where a corner of a partial turn went: a corner where the turn opens and
/// one where it closes, and the arc between them; or a single corner, on the
/// axis.
#[derive(Clone, Copy)]
struct Turned {
    opening: VertexId,
    closing: VertexId,
    arc: Option<Coedge>,
}

/// The surfaces either side of each corner of a contour: those of the piece
/// ending there and of the piece starting there, the axis having none.
fn beside(swept: &[Swept], index: usize) -> Vec<SurfaceId> {
    let count = swept.len();
    [swept[(index + count - 1) % count], swept[index]]
        .into_iter()
        .flatten()
        .map(|(surface, _)| surface)
        .collect()
}

/// The faces of one contour turned part of the way, and the edges it leaves
/// on the end the turn opens on and on the end it closes on, in the
/// contour's order.
pub(super) fn partial(
    laying: &mut Laying,
    placed: &Placed,
    contour: &[Named],
    swept: &[Swept],
    [(opening, _), (closing, _)]: [(SurfaceId, bool); 2],
) -> (Vec<Face>, Vec<Coedge>, Vec<Coedge>) {
    if let ([named], [Some(surface)]) = (contour, swept)
        && let Some(ring) = Round::of(&named.piece).filter(|_| named.piece.is_ring())
    {
        let (face, at_opening, at_closing) =
            round::ring(laying, placed, &ring, *surface, named.numbers.clone());
        return (vec![face], vec![at_opening], vec![at_closing]);
    }
    let turned: Vec<Turned> = contour
        .iter()
        .enumerate()
        .map(|(index, named)| {
            let corner = named.piece.from();
            let beside = beside(swept, index);
            let on = |end: SurfaceId| -> Vec<SurfaceId> {
                std::iter::once(end).chain(beside.iter().copied()).collect()
            };
            turned(laying, placed, corner, [on(opening), on(closing)])
        })
        .collect();
    let count = contour.len();
    let (mut faces, mut openings, mut closings) = (Vec::new(), Vec::new(), Vec::new());
    for (index, named) in contour.iter().enumerate() {
        let [start, end] = [turned[index], turned[(index + 1) % count]];
        let arc = Round::of(&named.piece);
        let at_opening = match &arc {
            Some(arc) => round::edge(laying, placed, arc, [start.opening, end.opening], false),
            None => laying.line(start.opening, end.opening),
        };
        openings.push(at_opening);
        let Some((surface, flipped)) = swept[index] else {
            closings.push(at_opening);
            continue;
        };
        let at_closing = match &arc {
            Some(arc) if placed.half && arc.is_sphere() => laying.round_beside(
                at_opening,
                [start.closing, end.closing],
                arc.sweep,
                placed.closing_outward(),
            ),
            Some(arc) => round::edge(laying, placed, arc, [start.closing, end.closing], true),
            None => {
                let square = named.piece.from().x == named.piece.to().x;
                if placed.half && square && start.arc.is_none() != end.arc.is_none() {
                    laying.line_on(at_opening, start.closing, end.closing)
                } else {
                    laying.line(start.closing, end.closing)
                }
            }
        };
        closings.push(at_closing);
        let lap = std::iter::once(at_opening)
            .chain(end.arc)
            .chain(std::iter::once(reversed(at_closing)))
            .chain(start.arc.map(reversed))
            .collect();
        faces.push(Face {
            surface,
            flipped,
            loops: vec![lap],
            numbers: named.numbers.clone(),
            poles: Vec::new(),
        });
    }
    (faces, openings, closings)
}

/// A corner of the profile turned part of the way: on the axis, one corner on
/// both ends and the surfaces beside it; off it, a corner on each end and the
/// arc between them, on the circle the corner turns along.
fn turned(laying: &mut Laying, placed: &Placed, corner: DVec2, on: [Vec<SurfaceId>; 2]) -> Turned {
    if corner.y == 0.0 {
        let both: Vec<SurfaceId> = on.concat();
        let vertex = laying.vertex(placed.center(corner), &both);
        return Turned {
            opening: vertex,
            closing: vertex,
            arc: None,
        };
    }
    let opening = laying.vertex(placed.opening(corner), &on[0]);
    let closing = laying.vertex(placed.closing(corner), &on[1]);
    let arc = laying.arc(
        &placed.cylinder(corner),
        placed.center(corner),
        [opening, closing],
        placed.angle,
        placed.axis,
    );
    Turned {
        opening,
        closing,
        arc: Some(arc),
    }
}
