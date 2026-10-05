//! The faces, edges and vertices of a turned profile.
//!
//! A partial turn has a corner where each corner of the profile off the axis
//! starts and where it ends, and the arc between them; one corner where a
//! corner on the axis stays; a straight edge where each run starts and where
//! it ends; and the two planes the turn ends on. A whole turn is its limit,
//! where the two ends close onto each other: a whole circle for each corner
//! off the axis, and no corner at all.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::Placed;
use crate::brep::Declined;
use crate::brep::laying::{Laying, reversed};
use crate::brep::piece::{Named, Piece};
use crate::brep::surface::{Plane, Surface};
use crate::brep::topology::{Body, Coedge, Face, SurfaceId, VertexId};

/// The surface a piece of the profile turns into and whether the matter lies
/// on the side its normal points to; none for a piece on the axis.
type Swept = Option<(SurfaceId, bool)>;

/// Where a corner of a partial turn went: a corner where the turn opens and
/// one where it closes, and the arc between them; or a single corner, on the
/// axis.
#[derive(Clone, Copy)]
struct Turned {
    opening: VertexId,
    closing: VertexId,
    arc: Option<Coedge>,
}

/// The body's surfaces, edges and vertices, and its faces before they are
/// tidied. The outline turns anticlockwise and the holes clockwise in
/// `(h, r)`, so that the matter lies on the left of each piece.
pub(super) fn laid(
    placed: &Placed,
    contours: &[Vec<Named>],
    runs: u32,
    eps: f64,
) -> Result<(Body, Vec<Face>), Declined> {
    let mut laying = Laying::new(eps);
    let ends = (!placed.whole).then(|| ends(&mut laying, placed, contours));
    let mut faces = Vec::new();
    let (mut openings, mut closings) = (Vec::new(), Vec::new());
    for contour in contours {
        let swept = contour
            .iter()
            .map(|named| sweep(&mut laying, placed, &named.piece))
            .collect::<Result<Vec<Swept>, Declined>>()?;
        let laid = match ends {
            None => whole(&mut laying, placed, contour, &swept),
            Some(ends) => {
                let (laid, opening, closing) = partial(&mut laying, placed, contour, &swept, ends);
                openings.push(
                    opening
                        .iter()
                        .rev()
                        .map(|coedge| reversed(*coedge))
                        .collect(),
                );
                closings.push(closing);
                laid
            }
        };
        faces.extend(laid);
    }
    if let Some([(opening, opening_flipped), (closing, closing_flipped)]) = ends {
        faces.push(Face {
            surface: opening,
            flipped: opening_flipped,
            loops: openings,
            numbers: vec![runs],
        });
        faces.push(Face {
            surface: closing,
            flipped: closing_flipped,
            loops: closings,
            numbers: vec![runs + 1],
        });
    }
    Ok((laying.body, faces))
}

/// The planes a partial turn ends on, laid before every other surface so
/// that no plane of the profile is taken for one of them: the sketch's own
/// plane, and the plane the profile is turned onto. At a half turn they are
/// one plane.
fn ends(laying: &mut Laying, placed: &Placed, contours: &[Vec<Named>]) -> [(SurfaceId, bool); 2] {
    let corners = || contours.iter().flatten().map(|named| named.piece.from());
    let opening: Vec<DVec3> = corners().map(|corner| placed.opening(corner)).collect();
    let closing: Vec<DVec3> = corners().map(|corner| placed.closing(corner)).collect();
    let (plane, turned) = Plane::through(placed.origin, -placed.across);
    let first = (laying.surface(Surface::Plane(plane), &opening), turned);
    let (plane, turned) = Plane::through(placed.origin, placed.closing_outward());
    let second = (laying.surface(Surface::Plane(plane), &closing), turned);
    [first, second]
}

/// The surface a piece turns into, shared with one already laid that it lies
/// on within the tolerance: two shoulders at one height on one plane. Read
/// at its ends and halfway round, where no end of the turn stands.
///
/// Parallel, square and on the axis are read exactly: the profile was laid
/// so before it came, each run's corners given one height or one distance.
fn sweep(laying: &mut Laying, placed: &Placed, piece: &Piece) -> Result<Swept, Declined> {
    let Piece::Straight { from, to } = *piece else {
        return Err(Declined::Profile);
    };
    let corners = [
        placed.opening(from),
        placed.opening(to),
        placed.halfway(from),
        placed.halfway(to),
    ];
    let along = to - from;
    if from.y == 0.0 && to.y == 0.0 {
        Ok(None)
    } else if along.y == 0.0 {
        let cylinder = Surface::Cylinder(placed.cylinder(from));
        Ok(Some((laying.surface(cylinder, &corners), along.x > 0.0)))
    } else if along.x == 0.0 {
        let (plane, flipped) = Plane::through(placed.center(from), placed.axis * along.y.signum());
        Ok(Some((
            laying.surface(Surface::Plane(plane), &corners),
            flipped,
        )))
    } else {
        Err(Declined::Profile)
    }
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

/// The faces of one contour turned whole: each piece off the axis bounded by
/// the whole circles its two ends turn along, those on the axis bounding
/// nothing.
fn whole(laying: &mut Laying, placed: &Placed, contour: &[Named], swept: &[Swept]) -> Vec<Face> {
    let rings: Vec<Option<Coedge>> = contour
        .iter()
        .map(|named| {
            let corner = named.piece.from();
            (corner.y > 0.0).then(|| {
                laying.ring(
                    &placed.cylinder(corner),
                    placed.center(corner),
                    TAU,
                    placed.axis,
                )
            })
        })
        .collect();
    let count = contour.len();
    let mut faces = Vec::new();
    for (index, named) in contour.iter().enumerate() {
        let Some((surface, flipped)) = swept[index] else {
            continue;
        };
        let [start, end] = [rings[index], rings[(index + 1) % count]];
        faces.push(Face {
            surface,
            flipped,
            loops: [end, start.map(reversed)]
                .into_iter()
                .flatten()
                .map(|ring| vec![ring])
                .collect(),
            numbers: named.numbers.clone(),
        });
    }
    faces
}

/// The faces of one contour turned part of the way, and the edges it leaves
/// on the end the turn opens on and on the end it closes on, in the
/// contour's order.
fn partial(
    laying: &mut Laying,
    placed: &Placed,
    contour: &[Named],
    swept: &[Swept],
    [(opening, _), (closing, _)]: [(SurfaceId, bool); 2],
) -> (Vec<Face>, Vec<Coedge>, Vec<Coedge>) {
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
        let at_opening = laying.line(start.opening, end.opening);
        openings.push(at_opening);
        let Some((surface, flipped)) = swept[index] else {
            closings.push(at_opening);
            continue;
        };
        let at_closing = if placed.half && start.arc.is_none() != end.arc.is_none() {
            laying.line_on(at_opening, start.closing, end.closing)
        } else {
            laying.line(start.closing, end.closing)
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
