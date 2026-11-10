//! The piece of sphere three fillets at a convex corner are closed with
//! (#699): centred one radius from the corner's three faces, where the three
//! rounds' axes cross, it meets each round along the round's end section —
//! the arc square to its edge through the centre — and touches each face at
//! one point. No boolean lays it: the sphere is tangent to each round along
//! that arc, and two of the rounds turn about another axis than any one
//! sphere's chart. The rounds are laid whole, the corner is cut back to the
//! three planes through the centre square to the edges, which leaves each
//! round ending on its arc and three quarter discs meeting at the centre, and
//! those three are replaced by the piece of sphere their arcs bound. At a
//! concave corner, a pocket's bottom (#801), the mirror: the sphere centred
//! one radius out from the three faces, the fills laid trimmed flush, the
//! corner between them filled up to the three planes, and the three quarter
//! discs that leaves, facing the centre, replaced by the piece of sphere
//! facing it, the matter outside the ball.

use glam::DVec3;

use super::closing::{Corner, slab};
use crate::body::drawn::Drawn;
use crate::body::exact::Exact;
use crate::brep::{self, Coedge, Curve, Declined, Face, FaceId, Sphere, Surface, SurfaceId};

/// The body with the corner closed by a piece of sphere of the value's
/// radius, numbered where the steps' numbers end, its centre where the
/// rounds' axes cross, one radius in from the first crossing's face.
/// Declined `Unsupported` where the plane through the centre square to an
/// edge does not stand between the corner and the edge's other end — the
/// sphere would stand at or behind the corner along it —, the corner cut
/// back leaves anything but three faces meeting at the centre,
/// bounded towards the rounds by three arcs about it, or the piece of
/// sphere left anything but its face, its every corner a corner of one of
/// the three `rounds`, as [`Exact::cut_along`] reads it; `Undrawn` where the
/// piece cannot be cut into triangles.
pub(in crate::body::exact) fn rounded_across(
    body: &Exact,
    corner: &Corner,
    rounds: [u32; 3],
) -> Result<Exact, Declined> {
    let centre = corner.crossings[0] - corner.facing[0] * corner.value;
    ahead(body, corner, centre)?;
    let cut = body.cut_by(&Exact {
        next: 1,
        ..short_of(corner, centre)?
    })?;
    sphered(body, cut, corner, centre, rounds, false)
}

/// The body with a concave corner closed by a hollow piece of sphere of the
/// value's radius, facing its centre, numbered where the steps' numbers end:
/// its centre one radius out from the first crossing's face, where the fills'
/// axes cross, and the corner between the three fills filled up to the three
/// planes through the centre square to the edges, as [`filling`] holds it.
/// Declined as [`rounded_across`] declines.
pub(in crate::body::exact) fn hollowed_across(
    body: &Exact,
    corner: &Corner,
    rounds: [u32; 3],
) -> Result<Exact, Declined> {
    let centre = corner.crossings[0] + corner.facing[0] * corner.value;
    ahead(body, corner, centre)?;
    let joined = body.joined(&Exact {
        next: 1,
        ..filling(corner, centre)?
    })?;
    sphered(body, joined, corner, centre, rounds, true)
}

/// `body` with the three faces `laid` left meeting at `centre` replaced by the
/// piece of sphere about it their arcs bound, `flipped` to face its centre
/// where the matter lies outside the ball, as [`rounded_across`] lays it.
fn sphered(
    body: &Exact,
    laid: Exact,
    corner: &Corner,
    centre: DVec3,
    rounds: [u32; 3],
    flipped: bool,
) -> Result<Exact, Declined> {
    let radius = corner.value;
    let eps = body.brep.scale().eps();
    let walls = meeting_at(&laid.brep, centre, eps);
    let lap = bounding(&laid.brep, &walls, centre, radius, eps).ok_or(Declined::Unsupported)?;
    let sphere = Sphere::through(centre, axis(corner.along), 0.0, radius);
    let face = Face {
        surface: SurfaceId(0),
        flipped,
        loops: vec![lap],
        numbers: vec![body.next],
        poles: Vec::new(),
    };
    let brep = laid.brep.replaced(&walls, Surface::Sphere(sphere), face)?;
    let drawn = Drawn::of(&brep);
    if !drawn.is_whole() {
        return Err(Declined::Undrawn);
    }
    let closed = Exact::of(brep, laid.next, laid.pieces, laid.roots);
    let _ = closed.drawn.set(drawn);
    if closed.cut_along(&[], body, 1, &rounds, &[]) {
        Ok(closed)
    } else {
        Err(Declined::Unsupported)
    }
}

/// Declined `Unsupported` where the plane through `centre` square to an edge
/// does not stand between the corner and the edge's other end: the sphere
/// would stand at or behind the corner along it.
fn ahead(body: &Exact, corner: &Corner, centre: DVec3) -> Result<(), Declined> {
    let eps = body.brep.scale().eps();
    if (corner.along.iter()).any(|way| way.dot(centre - corner.at) <= eps) {
        Err(Declined::Unsupported)
    } else {
        Ok(())
    }
}

/// The axis the piece of sphere is read about: square to two of the edges,
/// and as near along the third as any such: its end sections then lie on
/// two meridians meeting at a pole, where the piece touches the face of the
/// two edges, and as near the equator as the corner lets the third — on it
/// where the third edge stands square to the other two, at a square corner
/// or the end of a prism.
fn axis(along: [DVec3; 3]) -> DVec3 {
    [[0, 1, 2], [1, 2, 0], [2, 0, 1]]
        .map(|[one, other, third]| {
            let axis = along[one].cross(along[other]).normalize();
            (axis.dot(along[third]).abs(), axis)
        })
        .into_iter()
        .max_by(|(one, _), (other, _)| one.total_cmp(other))
        .map_or(along[0], |(_, axis)| axis)
}

/// What of the corner stands on its side of the three planes through
/// `centre` square to its edges: a cube about the centre, its half side
/// twice the way from the centre to the corner, less what of it stands past
/// each plane.
fn short_of(corner: &Corner, centre: DVec3) -> Result<Exact, Declined> {
    let half = 2.0 * centre.distance(corner.at);
    let [.., last] = corner.along;
    let mut kept = slab(last, centre - last * half, half)?;
    for way in corner.along {
        kept = kept.cut_by(&slab(way, centre, 2.0 * half)?)?;
    }
    Ok(kept)
}

/// What of a concave corner stands on its side of the three planes through
/// `centre` square to its edges and no deeper into the matter than
/// [`Corner::sunk`] keeps it: what [`short_of`] holds, less what of it stands
/// deeper.
fn filling(corner: &Corner, centre: DVec3) -> Result<Exact, Declined> {
    corner.sunk(short_of(corner, centre)?, 4.0 * centre.distance(corner.at))
}

/// The faces with a corner at `centre`, within `eps`.
fn meeting_at(body: &brep::Body, centre: DVec3, eps: f64) -> Vec<FaceId> {
    body.face_ids()
        .filter(|&face| {
            (body.face(face).loops.iter().flatten())
                .flat_map(|coedge| body.edge(coedge.edge).ends.into_iter().flatten())
                .any(|end| body.vertex(end).point.distance(centre) <= eps)
        })
        .collect()
}

/// The loop the three `walls` make towards the rest of the body, the way
/// they run it: three arcs about `centre` of `radius`, each where a wall
/// meets a face that is none of them, joined end to end. None where there
/// are not three walls and three such arcs, or they close no loop.
fn bounding(
    body: &brep::Body,
    walls: &[FaceId],
    centre: DVec3,
    radius: f64,
    eps: f64,
) -> Option<Vec<Coedge>> {
    let mut arcs: Vec<Coedge> = (walls.iter())
        .flat_map(|&wall| body.face(wall).loops.iter().flatten().copied())
        .filter(|coedge| (body.uses(coedge.edge).iter()).any(|(face, _)| !walls.contains(face)))
        .collect();
    let about = |coedge: &Coedge| match body.curve(body.edge(coedge.edge).curve) {
        Curve::Circle(circle) => {
            circle.center.distance(centre) <= eps && (circle.radius - radius).abs() <= eps
        }
        _ => false,
    };
    if walls.len() != 3 || arcs.len() != 3 || !arcs.iter().all(about) {
        return None;
    }
    let ends = |coedge: &Coedge| {
        let [from, to] = body.edge(coedge.edge).ends?;
        Some(if coedge.forward {
            [from, to]
        } else {
            [to, from]
        })
    };
    let mut lap = vec![arcs.pop()?];
    while !arcs.is_empty() {
        let [_, end] = ends(lap.last()?)?;
        let next = (arcs.iter()).position(|arc| ends(arc).is_some_and(|[from, _]| from == end))?;
        lap.push(arcs.swap_remove(next));
    }
    let ([start, _], [_, end]) = (ends(&lap[0])?, ends(lap.last()?)?);
    (start == end).then_some(lap)
}
