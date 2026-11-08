//! What a chamfer (#696) and a fillet (#697) share: the straight edge between
//! two flat faces they take off where the faces meet outwards and fill in
//! where they meet inwards (#778), read off the body with how far each face
//! runs from it, the cut or the join of a prism raised along it, and the
//! check that it left the one face the step names.

mod chain;
mod corners;
mod fill;
mod pairs;
mod section;

use std::collections::BTreeSet;
use std::f64::consts::PI;

use glam::DVec3;

use super::{Exact, PIECES, root};
use crate::body::caught::caught;
use crate::brep::{self, Curve, Declined, Edge, EdgeId, EdgeName, FaceId, Scale, Surface};
use crate::profile::{Contour, Frame};

pub(super) use pairs::Together;
pub(super) use section::Side;

/// A straight edge between two flat faces: a point of it, the way it runs,
/// its two faces, for each the way it faces and the way into it, square to
/// the edge, how far the nearer face runs that way from the edge — the most
/// a chamfer can set back — and whether the faces meet inwards, the way into
/// either running the way the other faces: a corner to fill in rather than
/// cut off.
pub(super) struct Wedge {
    pub point: DVec3,
    pub along: DVec3,
    pub faces: [FaceId; 2],
    pub outward: [DVec3; 2],
    pub into: [DVec3; 2],
    pub reach: f64,
    pub inwards: bool,
}

/// What a step raises to cut an edge off or fill it in: the section, square
/// to the edge, whose first run lays the face the step names, the frame it
/// stands in, and the travel along the edge.
pub(super) type Prism = (Contour, Frame, DVec3);

impl Exact {
    /// The body with the edge each name finds cut off, or filled in where its
    /// faces meet inwards, by the prism `tool` raises over its wedge for the
    /// one value taken, in the order the names come; that value: `asked`, or
    /// the most every edge gives — what `most` says each edge's two faces give
    /// alone, and, for two parallel edges facing each other across a face they
    /// share, what keeps the set-backs they make on it from crossing, by the
    /// set-back of a value of one `set_back` says — each read on the body as
    /// given. Rounds or flats that would meet anywhere else, across a face two
    /// edges share that are not parallel or on a face one of them ends on, are
    /// not measured: the laying declines them where they meet, until #792.
    /// Where `together` is some, the pairs whose set-backs meet across their
    /// face, the earlier first, are each taken off as one by the prism it
    /// raises, the face they shared gone; where it is none, each edge is laid
    /// alone. Two edges meeting at a convex corner are laid in turn as any two
    /// are, each prism carried past the corner along its edge's line, so that
    /// the later trims the earlier's face there: two flats or two rounds meet
    /// in a mitre.
    /// The wall an edge's prism lays is numbered where the steps' numbers
    /// ended, plus the edge's rank in the list; a pair is laid at its later
    /// edge's turn, so that every number laid before stands below those a
    /// prism lays, as [`Exact::cut_along`] reads them. A value shorter than the
    /// kernel can set a cut back from an edge is declined first; a list of no
    /// edge `Unsupported`; an edge its name no longer finds `Lost`; an edge
    /// that is not a wedge, one edge two names find, two edges meeting at a
    /// corner that is not convex — an edge ending there that is no wedge, or
    /// whose faces meet inwards —, where `together` is some an edge meeting
    /// two others, and a cut or a fill that left something else than its one
    /// face, as [`Exact::cut_along`] reads it, are declined `Unsupported`; a
    /// prism the tool cannot raise, as the tool says.
    pub(super) fn cut_off(
        &self,
        names: &[EdgeName],
        asked: f64,
        most: impl Fn(&Wedge) -> f64,
        set_back: impl Fn(&Wedge) -> f64,
        tool: impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
        together: Option<Together>,
    ) -> Result<(Exact, f64, Vec<pairs::Pair>), Declined> {
        let scale = self.brep.scale();
        if too_short(asked, scale) {
            return Err(Declined::Distance);
        }
        if names.is_empty() {
            return Err(Declined::Unsupported);
        }
        caught(|| {
            let found = names
                .iter()
                .map(|name| self.wedge(name, Wedge::of))
                .collect::<Result<Vec<(EdgeId, Wedge)>, Declined>>()?;
            let edges: Vec<EdgeId> = found.iter().map(|&(edge, _)| edge).collect();
            let once = edges.iter().collect::<BTreeSet<_>>().len() == edges.len();
            if !once || !corners::convex(&self.brep, &edges) {
                return Err(Declined::Unsupported);
            }
            let wedges: Vec<Wedge> = found.into_iter().map(|(_, wedge)| wedge).collect();
            let facing = pairs::facing(&wedges);
            let alone = wedges
                .iter()
                .fold(asked, |taken, wedge| taken.min(most(wedge)));
            let taken = alone.min(pairs::bound(&wedges, &facing, &set_back));
            let pairs = match together {
                Some(_) => pairs::meeting(&wedges, &facing, taken, &set_back, scale)?,
                None => Vec::new(),
            };
            let start = self.next;
            let mut cut = self.clone();
            for (rank, name) in names.iter().enumerate() {
                if pairs.iter().any(|&(earlier, _)| earlier == rank) {
                    continue;
                }
                cut.next = start + rank as u32;
                let partner = pairs.iter().find(|&&(_, later)| later == rank);
                cut = match (partner, together) {
                    (Some(&(earlier, _)), Some(together)) => cut.laid_together(
                        [&names[earlier], name],
                        start + earlier as u32,
                        together,
                    )?,
                    _ => cut.laid(name, taken, &tool)?,
                };
            }
            cut.next = start + names.len() as u32;
            Ok((cut, taken, pairs))
        })
    }

    /// The edge a name finds, and its wedge as `read` reads it. Declined
    /// `Lost` where the name no longer finds it, `Unsupported` where it is not
    /// a wedge.
    fn wedge(
        &self,
        name: &EdgeName,
        read: fn(&brep::Body, EdgeId) -> Result<Wedge, Declined>,
    ) -> Result<(EdgeId, Wedge), Declined> {
        let edge = self
            .brep
            .find_rooted(name, |number| root(&self.roots, number))
            .map_err(Declined::Lost)?;
        Ok((edge, read(&self.brep, edge)?))
    }

    /// The body with the edge a name finds cut off, or filled in, by the
    /// prism `tool` raises over its wedge for `taken`, its wall numbered
    /// where the steps' numbers ended, declined as [`Exact::cut_off`] says.
    /// The wedge is read again on the body the edges before it were laid on,
    /// as [`Wedge::read`] reads it: whether the edge is on a tangent chain was
    /// read on the body as given.
    fn laid(
        &self,
        name: &EdgeName,
        taken: f64,
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
    ) -> Result<Exact, Declined> {
        let (edge, wedge) = self.wedge(name, Wedge::read)?;
        let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
        let prism = tool(&wedge, taken, self.brep.scale(), bounds)?;
        let (outline, frame, travel) = &prism;
        let raised = Exact::raised(outline, &[], *frame, *travel)?;
        let tool = Exact {
            brep: raised.brep.renumbered_by(|number| match number {
                0 => MADE,
                MADE => 0,
                other => other,
            }),
            ..raised
        };
        let cut = if wedge.inwards {
            self.filled(&[(edge, &wedge)], &prism, tool)?
        } else {
            self.cut_by(&Exact { next: 1, ..tool })?
        };
        if cut.cut_along(&[&wedge], self, 1) {
            Ok(cut)
        } else {
            Err(Declined::Unsupported)
        }
    }

    /// Whether the prism left its face alone on the body, numbered where the
    /// steps' numbers of `before` ended and answering to the `laid` numbers
    /// from there, in one piece, every corner of it on one of the edges' faces
    /// — where the faces met at the edges' ends trim it — and every face
    /// `before` held but the edges' still there: no face, nor piece of a
    /// face, of the prism but that one.
    fn cut_along(&self, wedges: &[&Wedge], before: &Exact, laid: u32) -> bool {
        let made = before.next;
        let eps = self.brep.scale().eps();
        let numbers = |body: &brep::Body, face: FaceId| body.numbers(face).to_vec();
        let stray = self
            .brep
            .face_ids()
            .flat_map(|face| numbers(&self.brep, face))
            .any(|number| number >= made + laid && number < PIECES);
        let parted = self
            .roots
            .values()
            .any(|replaced| (made..PIECES).contains(replaced));
        let on_a_side = self
            .named(made as usize)
            .flat_map(|face| self.brep.face(face).loops.iter().flatten())
            .flat_map(|coedge| self.brep.edge(coedge.edge).ends.into_iter().flatten())
            .all(|corner| {
                let point = self.brep.vertex(corner).point;
                wedges.iter().any(|wedge| {
                    let at = point - wedge.point;
                    wedge.outward.iter().any(|side| side.dot(at).abs() <= eps)
                })
            });
        let consumable: BTreeSet<u32> = wedges
            .iter()
            .flat_map(|wedge| wedge.faces)
            .flat_map(|face| numbers(&before.brep, face))
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
    /// The wedge of an edge straight between two flat faces, whether or not
    /// it runs on tangentially into the next edge: a round laid before it at a
    /// corner they share does.
    fn read(body: &brep::Body, edge: EdgeId) -> Result<Wedge, Declined> {
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
        let towards = into_first.dot(second);
        if towards == 0.0 {
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
            inwards: towards > 0.0,
        })
    }
}

/// The shortest distance a chamfer is set back, in `Scale::HAIR`
/// tolerances: the kernel moves an operand that far to lay it on another,
/// and reads a plane or a corner that near an edge or a corner onto it, so
/// a chamfer within a few such moves of its edge is read back onto it. On
/// every edge of a block, a hexagonal prism and a trapezoid, chamfers of
/// one and a half came back declined or unsound, and from two and a half
/// every edge was cut; four leaves room.
const SHORTEST: f64 = 4.0;

/// Whether a length is none the kernel can set a cut back by, at the scale
/// of the part: not a number, or shorter than [`SHORTEST`] `Scale::HAIR`
/// tolerances.
pub(super) fn too_short(length: f64, scale: Scale) -> bool {
    length.is_nan() || length < SHORTEST * Scale::HAIR * scale.eps()
}

/// The number a raise gives the wall of its profile's first run.
const MADE: u32 = 2;

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
