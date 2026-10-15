//! The faces, edges and vertices of a turned profile.
//!
//! A partial turn has a corner where each corner of the profile off the axis
//! starts and where it ends, and the arc between them; one corner where a
//! corner on the axis stays; a straight edge where each run starts and where
//! it ends; and the two planes the turn ends on. A whole turn is its limit,
//! where the two ends close onto each other: a whole circle for each corner
//! off the axis, and no corner at all.

mod partial;

use std::f64::consts::TAU;

use glam::DVec3;

use super::Placed;
use crate::brep::Declined;
use crate::brep::laying::{Laying, reversed};
use crate::brep::piece::{Named, Piece};
use crate::brep::surface::{Plane, Surface};
use crate::brep::topology::{Body, Coedge, Face, SurfaceId};
use partial::partial;

/// The surface a piece of the profile turns into and whether the matter lies
/// on the side its normal points to; none for a piece on the axis.
type Swept = Option<(SurfaceId, bool)>;

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
            apex: None,
        });
        faces.push(Face {
            surface: closing,
            flipped: closing_flipped,
            loops: closings,
            numbers: vec![runs + 1],
            apex: None,
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
/// Any other run is slanted, and turns into a cone whose matter, on the
/// left of the run as the contour turns, lies on the side its normal points
/// to where the run goes away from the axis — read along the cone's own
/// axis, which a turn backwards points against the turn's.
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
        let cone = placed.cone(from, to);
        let flipped = (along.y > 0.0) == (cone.axis.dot(placed.axis) > 0.0);
        Ok(Some((
            laying.surface(Surface::Cone(cone), &corners),
            flipped,
        )))
    }
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
            apex: None,
        });
    }
    faces
}
