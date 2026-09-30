//! How many times a body wraps a point, and where a line passes through it:
//! an exact ray cast through its faces, for the boolean's winding (step 6 of
//! `docs/exact-kernel.md`) and for the campaign's spans.
//!
//! A ray is cast along the first of a fixed list of directions none of the
//! planes of the origin can hold; one that grazes a face, lands near an edge
//! or starts on a face is cast again along the next.

mod hits;

use glam::DVec3;

use super::Declined;
use super::domain::Location;
use super::topology::{Body, FaceId};
use hits::{Hits, hits};

/// Directions with no simple ratio between their components, so that no
/// plane of the origin, nor one at a simple slant to it, holds any of them.
const DIRECTIONS: [[f64; 3]; 8] = [
    [0.414_213_562_37, 0.732_050_807_57, 0.236_067_977_5],
    [-0.141_592_653_59, 0.718_281_828_46, -0.645_751_311_06],
    [0.316_624_790_36, -0.605_551_275_46, 0.123_105_625_62],
    [-0.605_551_275_46, -0.316_624_790_36, 0.414_213_562_37],
    [0.236_067_977_5, 0.141_592_653_59, -0.732_050_807_57],
    [0.718_281_828_46, -0.414_213_562_37, -0.316_624_790_36],
    [-0.123_105_625_62, -0.236_067_977_5, -0.718_281_828_46],
    [0.645_751_311_06, 0.605_551_275_46, 0.141_592_653_59],
];

/// A crossing whose direction stands closer than this to the face's plane,
/// as a cosine, is a graze: the next direction is tried.
const GRAZING: f64 = 1e-3;

/// What one face makes of a ray: a crossing with the way it goes through,
/// nothing, or a doubt that only another direction settles.
enum Met {
    Through { at: f64, step: i32 },
    Missed,
    Doubtful,
}

impl Body {
    /// The number of times the body wraps `point`: one inside its matter,
    /// nought outside. A point on a face has no answer; one within `eps` of
    /// a face is taken on whichever side it stands.
    pub fn winding(&self, point: DVec3, eps: f64) -> Result<i32, Declined> {
        for direction in DIRECTIONS.map(|direction| DVec3::from(direction).normalize()) {
            if let Some(winding) = self.cast(point, direction, eps)? {
                return Ok(winding);
            }
        }
        Err(Declined::Tie)
    }

    /// Every place the line through `origin` along `direction` passes through
    /// a face, by its parameter, with one for going into the matter and minus
    /// one for coming out of it, sorted along the line. Declined where the line
    /// grazes a face or passes through an edge.
    pub fn crossings_along(
        &self,
        origin: DVec3,
        direction: DVec3,
        eps: f64,
    ) -> Result<Vec<(f64, i32)>, Declined> {
        let unit = direction.length();
        let mut found = Vec::new();
        for face in self.face_ids() {
            for met in self.met(face, origin, direction / unit, f64::NEG_INFINITY, eps)? {
                match met {
                    Met::Through { at, step } => found.push((at / unit, -step)),
                    Met::Missed => {}
                    Met::Doubtful => return Err(Declined::Tie),
                }
            }
        }
        found.sort_by(|one, other| one.0.total_cmp(&other.0));
        Ok(found)
    }

    /// The winding along one ray, or none when the ray is doubtful.
    fn cast(&self, origin: DVec3, direction: DVec3, eps: f64) -> Result<Option<i32>, Declined> {
        let mut winding = 0;
        for face in self.face_ids() {
            for met in self.met(face, origin, direction, -eps, eps)? {
                match met {
                    Met::Through { at, .. } if at <= eps => return Ok(None),
                    Met::Through { step, .. } => winding += step,
                    Met::Missed => {}
                    Met::Doubtful => return Ok(None),
                }
            }
        }
        Ok(Some(winding))
    }

    /// Where the line through `origin` along the unit `direction` meets a
    /// face from the parameter `from` on, each crossing with one for coming
    /// out of the matter there.
    fn met(
        &self,
        face: FaceId,
        origin: DVec3,
        direction: DVec3,
        from: f64,
        eps: f64,
    ) -> Result<Vec<Met>, Declined> {
        let lying = self.face(face);
        let surface = self.surface(lying.surface);
        let found = match hits(surface, origin, direction, eps) {
            Hits::Along => return Ok(vec![Met::Doubtful]),
            Hits::At(found) => found,
        };
        let mut met = Vec::new();
        for (at, normal) in found.into_iter().filter(|(at, _)| *at >= from) {
            let point = origin + direction * at;
            let located = self.locate(face, surface.parameters(point), eps)?;
            let outward = if lying.flipped { -normal } else { normal };
            let across = outward.dot(direction);
            met.push(match located {
                Location::Outside => Met::Missed,
                Location::Boundary => Met::Doubtful,
                Location::Inside if across.abs() < GRAZING => Met::Doubtful,
                Location::Inside => Met::Through {
                    at,
                    step: if across > 0.0 { 1 } else { -1 },
                },
            });
        }
        Ok(met)
    }
}

#[cfg(test)]
mod tests;
