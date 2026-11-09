//! The piece of sphere three fillets at a convex corner are closed with
//! (#699): centred one radius from the corner's three faces, where the three
//! rounds' axes cross, it meets each round along the round's end section —
//! the arc square to its edge through the centre — and touches each face at
//! one point. No boolean lays it: the sphere is tangent to each round along
//! that arc, and two of the rounds turn about another axis than any one
//! sphere's chart. The rounds are laid whole, the corner is cut back to the
//! three planes through the centre square to the edges, which leaves each
//! round ending on its arc and three quarter discs meeting at the centre, and
//! those three are replaced by the piece of sphere their arcs bound.

use glam::{DVec2, DVec3};

use super::closing::Corner;
use crate::body::drawn::Drawn;
use crate::body::exact::Exact;
use crate::brep::{self, Coedge, Curve, Declined, Face, FaceId, Sphere, Surface, SurfaceId};
use crate::profile::{Contour, Frame};

/// The body with the corner closed by a piece of sphere of the value's
/// radius, numbered where the steps' numbers end. Declined `Unsupported`
/// where the rounds' axes do not cross at one point, a plane through it
/// square to an edge does not stand between the corner and the edge, the
/// corner cut back leaves anything but three faces meeting at the centre,
/// bounded towards the rounds by three arcs about it, or the piece of
/// sphere left anything but its face, its every corner a corner of one of
/// the three `rounds`, as [`Exact::cut_along`] reads it; `Undrawn` where the
/// piece cannot be cut into triangles.
pub(in crate::body::exact) fn rounded_across(
    body: &Exact,
    corner: &Corner,
    rounds: [u32; 3],
) -> Result<Exact, Declined> {
    let radius = corner.value;
    let eps = body.brep.scale().eps();
    let centres = [0, 1, 2].map(|face| corner.crossings[face] - corner.facing[face] * radius);
    let centre = centres[0];
    if centres.iter().any(|other| other.distance(centre) > eps)
        || corner
            .along
            .iter()
            .any(|way| way.dot(centre - corner.at) <= eps)
    {
        return Err(Declined::Unsupported);
    }
    let cut = body.cut_by(&Exact {
        next: 1,
        ..short_of(corner, centre)?
    })?;
    let walls = meeting_at(&cut.brep, centre, eps);
    let lap = bounding(&cut.brep, &walls, centre, radius, eps).ok_or(Declined::Unsupported)?;
    let sphere = Sphere::through(centre, axis(corner.along), 0.0, radius);
    let face = Face {
        surface: SurfaceId(0),
        flipped: false,
        loops: vec![lap],
        numbers: vec![body.next],
        poles: Vec::new(),
    };
    let brep = cut.brep.replaced(&walls, Surface::Sphere(sphere), face)?;
    let drawn = Drawn::of(&brep);
    if !drawn.is_whole() {
        return Err(Declined::Undrawn);
    }
    let closed = Exact::of(brep, cut.next, cut.pieces, cut.roots);
    let _ = closed.drawn.set(drawn);
    if closed.cut_along(&[], body, 1, &rounds) {
        Ok(closed)
    } else {
        Err(Declined::Unsupported)
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
    let slab = |way: DVec3, from: DVec3, half: f64| {
        let u = way.any_orthonormal_vector();
        let frame = Frame {
            origin: from,
            u,
            v: way.cross(u),
        };
        let square = Contour::rectangle(DVec2::splat(-half), DVec2::splat(half));
        Exact::raised(&square, &[], frame, way * 2.0 * half)
    };
    let [.., last] = corner.along;
    let mut kept = slab(last, centre - last * half, half)?;
    for way in corner.along {
        kept = kept.cut_by(&slab(way, centre, 2.0 * half)?)?;
    }
    Ok(kept)
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
