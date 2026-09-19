//! Where the surface of a solid turns a corner, and what names each one.
//!
//! A corner is named by the faces that meet at it. That name holds when the
//! part changes size — the faces are the same faces — and is lost exactly
//! when the corner really goes, which is what lets a drawing laid on the part
//! follow it without ever silently grabbing the neighbour.
//!
//! Not every place a polygon has a corner is one. Two of them are not:
//!
//! A boolean cuts a flat face into pieces, and the pieces keep numbers of
//! their own. Where two such pieces meet a wall, three faces meet and the
//! surface does not turn at all. Worse, the *same* three answer along the
//! whole seam, so two places would carry one name.
//!
//! A curved wall is sampled into flats. Along the rim where it meets a cap,
//! every sample has two flats of the wall and the cap meeting at it — three
//! directions, and only two faces. A rim is a curve, not a run of corners.
//!
//! So a corner is a place where **three faces** meet whose **normals do not
//! all lie in one plane**. Both halves are needed: the first throws out the
//! rim, the second throws out the seam.

use std::collections::BTreeSet;

use glam::DVec3;

use crate::mesh::Mesh;

/// A place where the surface of a solid turns, and the faces that meet there.
#[derive(Clone, Debug, PartialEq)]
pub struct Corner {
    pub at: DVec3,
    /// The faces meeting there, once each and in order. This is its name.
    pub faces: Vec<usize>,
}

/// How far apart two corners of two polygons can be and still be one place.
///
/// Corners come from the same arithmetic on both sides of an edge, so they
/// land on each other to the last bit or thereabouts; this only has to be
/// wider than that rounding and narrower than anything a part distinguishes.
const THE_SAME_PLACE: f64 = 1e-9;

/// How far off parallel two faces have to be to count as facing different
/// ways.
const THE_SAME_WAY: f64 = 0.9999;

/// How far a third face has to lean out of the plane two others span before
/// the surface counts as turning there.
const OUT_OF_THE_PLANE: f64 = 1e-6;

impl Mesh {
    /// Every corner of the surface, in no particular order.
    pub fn corners(&self) -> Vec<Corner> {
        let mut places: Vec<(DVec3, Vec<(usize, DVec3)>)> = Vec::new();
        for polygon in &self.polygons {
            let normal = polygon.normal();
            for corner in &polygon.corners {
                let known = places
                    .iter_mut()
                    .find(|(place, _)| place.distance(*corner) <= reach(*corner));
                match known {
                    Some((_, meeting)) => meeting.push((polygon.face, normal)),
                    None => places.push((*corner, vec![(polygon.face, normal)])),
                }
            }
        }

        places
            .into_iter()
            .filter_map(|(at, meeting)| turns(&meeting).map(|faces| Corner { at, faces }))
            .collect()
    }
}

/// The faces meeting at a place, when the surface really turns there.
fn turns(meeting: &[(usize, DVec3)]) -> Option<Vec<usize>> {
    let faces: BTreeSet<usize> = meeting.iter().map(|(face, _)| *face).collect();
    if faces.len() < 3 {
        return None;
    }
    let mut ways: Vec<DVec3> = Vec::new();
    for (_, normal) in meeting {
        if !ways.iter().any(|known| known.dot(*normal) > THE_SAME_WAY) {
            ways.push(*normal);
        }
    }
    // Two directions always lie in a plane. A third one turns the surface
    // only if it stands out of the plane those two span.
    let (plane, rest) = ways.split_first()?;
    let across = rest
        .iter()
        .map(|other| plane.cross(*other).normalize_or_zero())
        .find(|across| *across != DVec3::ZERO)?;
    let turns = ways
        .iter()
        .any(|way| across.dot(*way).abs() > OUT_OF_THE_PLANE);

    turns.then(|| faces.into_iter().collect())
}

/// Read against how far out the place stands, so a part can be measured in
/// anything.
fn reach(place: DVec3) -> f64 {
    THE_SAME_PLACE * (1.0 + place.abs().max_element())
}

#[cfg(test)]
mod tests;
