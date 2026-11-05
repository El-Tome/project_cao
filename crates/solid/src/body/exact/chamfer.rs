//! The cut a chamfer makes: a prism raised along the edge over the wedge
//! between the edge's two faces and the plane set back by the distance
//! along each, cut from the body. Its wall on that plane is the one face
//! the step names.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::{Exact, PIECES, root};
use crate::body::caught::caught;
use crate::brep::{self, Coedge, Curve, Declined, Edge, EdgeId, EdgeName, FaceId, Scale, Surface};
use crate::profile::{Contour, Frame};

/// A straight edge between two flat faces meeting outwards: a point of it,
/// the way it runs, its two faces, for each the way it faces and the way
/// into it, square to the edge, and how far the nearer face runs that way
/// from the edge — the most a chamfer can set back.
struct Wedge {
    point: DVec3,
    along: DVec3,
    faces: [FaceId; 2],
    outward: [DVec3; 2],
    into: [DVec3; 2],
    reach: f64,
}

impl Exact {
    /// The body with the edge a name finds cut off, set back by `distance`
    /// along each of its two faces or by the most they give, and the
    /// distance taken; the face the cut lays numbered where the steps'
    /// numbers ended, and trimmed, as a CAD tool trims it, by the faces it
    /// meets at the edge's ends — past the end corner where one of them
    /// leaves the edge at an obtuse angle, or a round leaves it at a few
    /// degrees either way. A distance the tolerance holds no length of is
    /// declined first. Unsupported: an edge that is not straight between two
    /// flat faces meeting outwards, one running on tangentially into the next
    /// edge of a face, and a cut that, carried along the edge's line, takes
    /// matter beyond the faces it meets there — a flat wall leaving the edge
    /// at a concave corner, which the cut would run on along — seen as a face
    /// of the prism but its chamfer left on the body, the chamfer in two
    /// pieces or with a corner on neither of the edge's faces, or a face the
    /// body held, but the edge's two, taken away whole.
    pub fn chamfered(&self, name: &EdgeName, distance: f64) -> Result<(Exact, f64), Declined> {
        if distance.is_nan() || distance < self.brep.scale().eps() {
            return Err(Declined::Distance);
        }
        caught(|| {
            let edge = self
                .brep
                .find_rooted(name, |number| root(&self.roots, number))
                .map_err(Declined::Lost)?;
            let wedge = Wedge::of(&self.brep, edge)?;
            let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
            let taken = distance.min(wedge.reach);
            let margin = (taken * MARGIN).max(2.0 * Scale::HAIR * self.brep.scale().eps());
            let (outline, frame, travel) = wedge.prism(taken, margin, bounds);
            let raised = Exact::raised(&outline, &[], frame, travel)?;
            let tool = Exact {
                brep: raised.brep.renumbered_by(|number| match number {
                    0 => CHAMFER,
                    CHAMFER => 0,
                    other => other,
                }),
                next: 1,
                ..raised
            };
            let cut = self.cut_by(&tool)?;
            if cut.cut_along(&wedge, self) {
                Ok((cut, taken))
            } else {
                Err(Declined::Unsupported)
            }
        })
    }

    /// Whether the prism left its chamfer alone on the body, numbered where
    /// the steps' numbers of `before` ended, in one piece, every corner of it
    /// on one of the edge's two faces — where the faces met at the edge's
    /// ends trim it — and every face `before` held but those two still there.
    fn cut_along(&self, wedge: &Wedge, before: &Exact) -> bool {
        let made = before.next;
        let eps = self.brep.scale().eps();
        let numbers = |body: &brep::Body, face: FaceId| body.numbers(face).to_vec();
        let stray = self
            .brep
            .face_ids()
            .flat_map(|face| numbers(&self.brep, face))
            .any(|number| number > made && number < PIECES);
        let parted = self.roots.values().any(|&replaced| replaced == made);
        let on_a_side = self
            .named(made as usize)
            .flat_map(|face| self.brep.face(face).loops.iter().flatten())
            .flat_map(|coedge| self.brep.edge(coedge.edge).ends.into_iter().flatten())
            .all(|corner| {
                let at = self.brep.vertex(corner).point - wedge.point;
                wedge.outward.iter().any(|side| side.dot(at).abs() <= eps)
            });
        let consumable: BTreeSet<u32> = wedge
            .faces
            .iter()
            .flat_map(|&face| numbers(&before.brep, face))
            .collect();
        let kept = before
            .brep
            .face_ids()
            .flat_map(|face| numbers(&before.brep, face))
            .filter(|number| !consumable.contains(number))
            .all(|number| self.has_face(number as usize));
        !stray && !parted && self.has_face(made as usize) && on_a_side && kept
    }
}

impl Wedge {
    fn of(body: &brep::Body, edge: EdgeId) -> Result<Wedge, Declined> {
        let stretch = body.edge(edge);
        let Curve::Line(line) = body.curve(stretch.curve) else {
            return Err(Declined::Unsupported);
        };
        let along = line.direction * (stretch.to - stretch.from).signum();
        let side = |face: FaceId, forward: bool| {
            let face = body.face(face);
            let Surface::Plane(plane) = body.surface(face.surface) else {
                return None;
            };
            let outward = if face.flipped {
                -plane.normal
            } else {
                plane.normal
            };
            let running = if forward { along } else { -along };
            Some((outward, outward.cross(running)))
        };
        let uses = body.uses(edge);
        let [(one, one_way), (other, other_way)] = uses[..] else {
            return Err(Declined::Unsupported);
        };
        let (Some((first, into_first)), Some((second, into_second))) =
            (side(one, one_way), side(other, other_way))
        else {
            return Err(Declined::Unsupported);
        };
        if into_first.dot(second) >= 0.0
            || [one, other]
                .into_iter()
                .any(|face| runs_on(body, face, edge, along))
        {
            return Err(Declined::Unsupported);
        }
        let reach_of = |face: FaceId, into: DVec3| reach(body, face, into, line.origin);
        let (Some(first_reach), Some(second_reach)) =
            (reach_of(one, into_first), reach_of(other, into_second))
        else {
            return Err(Declined::Unsupported);
        };
        Ok(Wedge {
            point: line.origin,
            along,
            faces: [one, other],
            outward: [first, second],
            into: [into_first, into_second],
            reach: first_reach.min(second_reach),
        })
    }

    /// The prism the chamfer is cut with, raised along the edge: in a
    /// section square to it, the wedge between the chamfer's plane and the
    /// edge's two faces, each face pushed out by `margin`, its first run on
    /// the chamfer's plane; carried over the body's box along the edge's
    /// line and `margin` past either end. Nothing of it but that run meets
    /// the two faces near the edge, and the part reaches little further.
    fn prism(
        &self,
        distance: f64,
        margin: f64,
        (low, high): (DVec3, DVec3),
    ) -> (Contour, Frame, DVec3) {
        let [first, second] = self.outward;
        let [on_first, on_second] = self.into.map(|into| into * distance);
        let across = (on_first - on_second).normalize();
        let corners = [
            on_second - across * (margin / -second.dot(across)),
            on_first + across * (margin / first.dot(across)),
            (first + second) * (margin / (1.0 + first.dot(second))),
        ];
        let middle = self.along.dot((low + high) / 2.0 - self.point);
        let half = self.along.abs().dot((high - low) / 2.0) + margin;
        let frame = Frame {
            origin: self.point + self.along * (middle - half),
            u: across,
            v: self.along.cross(across),
        };
        let outline = Contour::straight(
            corners
                .map(|corner| DVec2::new(corner.dot(frame.u), corner.dot(frame.v)))
                .to_vec(),
        );
        (outline, frame, self.along * 2.0 * half)
    }
}

/// The number a raise gives the wall of its profile's first run.
const CHAMFER: u32 = 2;

/// How far the prism stands off the edge's two faces, and past the body's
/// box along the edge, as a share of the distance taken: far enough that no
/// face of it lies nearly flush with one of the body's, near enough that
/// the part's reach, which every later step's tolerance is taken against,
/// grows by an eighth of the distance at a square edge. Never nearer than
/// twice the furthest the kernel moves one operand onto another's face
/// (`Scale::HAIR` tolerances), which would lay the two flush.
const MARGIN: f64 = 0.125;

/// How far a face runs along `into` from the edge's line through `point`:
/// to the nearest corner of its boundary off that line, or to the nearest
/// point where a round or a conic of its boundary turns back along `into`.
/// A whole closed edge counts where its turn starts as a corner, so that a
/// turning point standing there is not lost between the two ends of the
/// turn. None where its boundary holds a curve two walls meet along, which
/// turns back where no closed form says.
fn reach(body: &brep::Body, face: FaceId, into: DVec3, point: DVec3) -> Option<f64> {
    let mut far = Vec::new();
    for coedge in body.face(face).loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        match edge.ends {
            Some(ends) => far.extend(ends.map(|end| body.vertex(end).point)),
            None => far.push(body.point_on(coedge.edge, edge.from)),
        }
        far.extend(turning_back(body, edge, into)?);
    }
    let eps = body.scale().eps();
    Some(
        far.into_iter()
            .map(|at| into.dot(at - point))
            .filter(|&off| off > eps)
            .fold(f64::INFINITY, f64::min),
    )
}

/// The points strictly inside an edge where its curve turns back along
/// `into`: none on a line, every half turn from the phase of `into` on a
/// circle, the conic's own turning points.
fn turning_back(body: &brep::Body, edge: &Edge, into: DVec3) -> Option<Vec<DVec3>> {
    let curve = body.curve(edge.curve);
    let (low, high) = (edge.from.min(edge.to), edge.from.max(edge.to));
    let turns = match curve {
        Curve::Line(_) => Vec::new(),
        Curve::Circle(circle) => {
            let phase = into.dot(circle.v).atan2(into.dot(circle.u));
            let first = ((low - phase) / PI).ceil() as i64;
            let last = ((high - phase) / PI).floor() as i64;
            (first..=last)
                .map(|turn| phase + PI * turn as f64)
                .filter(|&at| at > low && at < high)
                .collect()
        }
        Curve::Conic(conic) => conic.turning(into, low, high),
        Curve::Meet(_) => return None,
    };
    Some(turns.into_iter().map(|at| curve.point(at)).collect())
}

/// The sine of the angle under which an edge running on into the next one
/// is a tangent chain. Well above what a sketch solved within a billionth of
/// its size leaves between a round and the run it was drawn tangent to —
/// some 1.5e-8 for a round of 2 in a drawing of 30, ten times that for a
/// round of 0.2 — and well below any corner a hand draws: a thousandth of
/// a degree is some 1.7e-5. That noise is a billionth of the drawing's size
/// over the round's radius, so this holds it up to a drawing a thousand
/// times the radius; past that, a round a hair off tangent reads as a
/// corner and is cut, soundly.
const TANGENT: f64 = 1e-6;

/// Whether, along a loop of `face`, the edge runs on into the edge after it,
/// or the edge before it runs on into it, along the edge's own way within
/// [`TANGENT`]: a tangent chain.
fn runs_on(body: &brep::Body, face: FaceId, edge: EdgeId, along: DVec3) -> bool {
    body.face(face).loops.iter().any(|lap| {
        let Some(at) = lap.iter().position(|coedge| coedge.edge == edge) else {
            return false;
        };
        let running = if lap[at].forward { along } else { -along };
        let [next, before] = [at + 1, at + lap.len() - 1].map(|rank| lap[rank % lap.len()]);
        let on = |way: DVec3| way.dot(running) > 0.0 && way.cross(running).length() <= TANGENT;
        on(ways(body, next)[0]) || on(ways(body, before)[1])
    })
}

/// The way a loop runs along a coedge where it starts and where it ends.
fn ways(body: &brep::Body, coedge: Coedge) -> [DVec3; 2] {
    let edge = body.edge(coedge.edge);
    let curve = body.curve(edge.curve);
    let sign = (edge.to - edge.from).signum();
    let [start, end] = [edge.from, edge.to].map(|at| curve.derivative(at).normalize() * sign);
    if coedge.forward {
        [start, end]
    } else {
        [-end, -start]
    }
}
