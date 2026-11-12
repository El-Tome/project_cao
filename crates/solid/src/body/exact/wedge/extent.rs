//! Where the set-back of an edge of a step runs on one of its faces (#829):
//! the half-spaces that hold it, between the faces its edge ends on or the
//! mitres the list cuts it to, and the span of a straight edge of the body
//! within them. A blend reaches only there, so that a face's corner, or
//! another edge's set-back, standing on past where it stops is never read as
//! met: the far edge of a lower block a foot's line would cross, a block
//! beside a foot corner past the mitre of the fill along a flat's foot —
//! while the foot of what stands on the face crossing into the sweep, a low
//! wall in front of a foot, still is, where it enters.

use glam::DVec3;

use super::Wedge;
use super::corners::Beside;
use super::end::end_face;
use super::flush::stops;
use crate::brep::{self, EdgeId, Surface};

/// A half-space, `normal · p ≤ offset`: one side of where a set-back ends.
pub(super) type Within = (DVec3, f64);

/// A half-space holding a set-back at one end of its edge, and whether it is
/// a mitre's rather than the plane of the face the edge ends on: the plane
/// through the corner the edge mitres on there with another edge of the list
/// sharing its face — past which that edge's blend takes over — or, at a
/// corner of mixed sense the edge goes round, the plane square to it through
/// the corner. Past that one no edge of the list need take over: the corner
/// is the patch's, which the chamfer's second laying reads through the edge
/// on the odd edge's face, laid again lower where that falls short, and which
/// the fillet's guards against a filling swallowing, or a clearing cutting, a
/// face of the body decline.
pub(super) type End = (Within, bool);

/// A listed edge by its rank, and the side of its wedge a face is on.
pub(super) type Ranked = (usize, usize);

/// The shares of the way along the straight edge from `ends[0]` to `ends[1]`
/// that every half-space of `held` holds, within `eps`: from the first and to
/// the last. None where none is.
pub(super) fn span(held: &[Within], ends: [DVec3; 2], eps: f64) -> Option<[f64; 2]> {
    (held.iter())
        .map(|&(normal, offset)| ends.map(|at| normal.dot(at) - offset))
        .try_fold([0.0, 1.0], |shares, past| narrowed(shares, past, eps))
}

/// The shares of the way along a straight edge `shares` holds, less those
/// where a half-space's form, `past` at the edge's two ends and in proportion
/// between, stands above `eps`. None where none is left.
fn narrowed([low, high]: [f64; 2], past: [f64; 2], eps: f64) -> Option<[f64; 2]> {
    let rising = past[1] - past[0];
    let level = (eps - past[0]) / rising;
    let [low, high] = if rising > 0.0 {
        [low, high.min(level)]
    } else if rising < 0.0 {
        [low.max(level), high]
    } else if past[0] <= eps {
        [low, high]
    } else {
        return None;
    };
    (low <= high).then_some([low, high])
}

/// Where the set-back of the listed edge `ranked` runs on its face on that
/// side once the edge of rank `later` is laid, as the half-spaces that hold
/// it, each on the side the middle of the edge stands on, beyond `eps` of
/// it. At an end one other edge of the list meets, sharing that face, where
/// that edge's set-back crosses its own at every value, the plane through the
/// corner the two mitre on: where the other is laid before that one, or
/// where the two meet beside an edge of the other sense, as `beside` holds
/// them, and the edge stops flush there, as [`stops`] reads it — whichever
/// is laid later then takes their overlap and carries this one on past the
/// face it stops on into the mitre, in any order. At any other end, the
/// plane of the flat face it ends on there, which trims its blend until an
/// edge laid later mitres it, back from that plane at a convex corner — and
/// where several edges of the list meet it there, at a corner whose three
/// edges are named, which every laying cuts back within that plane or mitres
/// short of it. At an end where it is one of the two edges of one sense at a
/// corner of mixed sense, which go round the odd edge once it is laid (#829),
/// the plane through the corner square to the edge, past which their blends
/// never run: they mitre past it with the edge where the odd edge's face
/// meets the one they share (#863). Nothing holds it at an end where the
/// face it ends on is curved or not one face, or where the one edge mitring
/// it there does not share its face. Each half-space says whether it is a
/// mitre's, as [`End`] reads it.
pub(super) fn runs(
    body: &brep::Body,
    (edges, wedges): (&[EdgeId], &[Wedge]),
    beside: &[Beside],
    (rank, side): Ranked,
    later: usize,
    set_back: &impl Fn(&Wedge) -> f64,
) -> Vec<End> {
    let wedge = &wedges[rank];
    let eps = body.scale().eps();
    let stopping = stops(body, edges[rank], wedge);
    let Some(ends) = body.edge(edges[rank]).ends else {
        return Vec::new();
    };
    let middle = (body.vertex(ends[0]).point + body.vertex(ends[1]).point) / 2.0;
    let depth = |wedge: &Wedge, side: usize| {
        let per = set_back(wedge);
        (
            wedge.into[side] / per,
            wedge.into[side].dot(wedge.point) / per,
        )
    };
    let mut held = Vec::new();
    for end in ends {
        let meeting: Vec<usize> = (0..edges.len())
            .filter(|&other| other != rank)
            .filter(|&other| (body.edge(edges[other]).ends).is_some_and(|at| at.contains(&end)))
            .collect();
        let trimmed = || {
            let face = end_face(body, end, wedge.faces)?;
            let Surface::Plane(plane) = body.surface(body.face(face).surface) else {
                return None;
            };
            Some((plane.normal, plane.normal.dot(body.vertex(end).point)))
        };
        let carried = |neighbour: usize| {
            beside.contains(&(end, [rank.min(neighbour), rank.max(neighbour)]))
                && stopping.iter().any(|stop| stop.end == end)
        };
        let gone_round = |others: [usize; 2]| {
            let [mine, one, other] = [rank, others[0], others[1]].map(|at| wedges[at].inwards);
            (one != other) && (mine == one || mine == other)
        };
        let half = match meeting[..] {
            [one, other] if gone_round([one, other]) => {
                let at = body.vertex(end).point;
                Some(((wedge.along, wedge.along.dot(at)), true))
            }
            [neighbour] if neighbour < later || carried(neighbour) => {
                let theirs = &wedges[neighbour];
                let on = theirs
                    .faces
                    .iter()
                    .position(|&face| face == wedge.faces[side]);
                on.map(|on| {
                    let [(mine, own), (their, other)] = [depth(wedge, side), depth(theirs, on)];
                    ((their - mine, other - own), true)
                })
            }
            _ => trimmed().map(|plane| (plane, false)),
        };
        if let Some(((normal, offset), mitre)) = half {
            let inside = normal.dot(middle) - offset;
            if inside.abs() > eps {
                let toward = -inside.signum();
                held.push(((normal * toward, offset * toward), mitre));
            }
        }
    }
    held
}
