//! What a chamfer (#696) and a fillet (#697) share: the straight edge between
//! two flat faces they take off where the faces meet outwards and fill in
//! where they meet inwards (#778), read off the body with how far each face
//! runs from it, the cut or the join of a prism raised along it, and the
//! check that it left the one face the step names.

mod chain;
mod closing;
mod corners;
mod end;
mod extent;
mod fill;
mod flush;
mod odd;
mod overlap;
mod pairs;
mod reach;
mod reading;
mod section;
mod set_backs;
mod slope;
mod sphere;
mod step;
mod torus;
mod triangle;

use std::collections::BTreeSet;

use glam::DVec3;

use super::{Exact, PIECES, root};
use crate::body::caught::caught;
use crate::brep::{self, Declined, EdgeId, EdgeName, FaceId, Scale, VertexId};
use crate::profile::{Contour, Frame};

pub(super) use closing::Closing;
use flush::{Flush, Stop};
pub(super) use odd::Around;
use overlap::OVERLAPS;
pub(super) use pairs::Together;
pub(super) use section::Side;
pub(super) use sphere::{hollowed_across, rounded_across};
use step::Kind;
pub(super) use triangle::{cut_across, joined_across};

/// A straight edge between two flat faces: a point of it, the way it runs, its
/// two faces and the numbers they answer to on the body it was read on, for
/// each the way it faces and the way into it, square to the edge, how far the
/// nearer face runs that way from the edge — the most a chamfer can set back —
/// and whether the faces meet inwards, the way into either running the way the
/// other faces: a corner to fill in rather than cut off.
pub(super) struct Wedge {
    pub point: DVec3,
    pub along: DVec3,
    pub faces: [FaceId; 2],
    pub numbers: [Vec<u32>; 2],
    pub outward: [DVec3; 2],
    pub into: [DVec3; 2],
    pub reach: f64,
    pub inwards: bool,
}

/// What a step raises to cut an edge off or fill it in: the section, square
/// to the edge, whose first run lays the face the step names, the frame it
/// stands in, and the travel along the edge.
pub(super) type Prism = (Contour, Frame, DVec3);

/// What a kind lays where the edges of one step meet: two whose set-backs
/// meet across the face they share, as one, where `together` lays them so —
/// any other two each laid alone — a corner three meet at, closed, and one
/// of mixed sense, gone round its odd edge as `around` says.
#[derive(Clone, Copy)]
pub(super) struct Meeting {
    pub together: Together,
    pub closing: Closing,
    pub around: Around,
}

impl Exact {
    /// The body with the edge each name finds cut off, or filled in where its
    /// faces meet inwards, by the prism `tool` raises over its wedge for the
    /// one value taken, in the order the names come; that value: `asked`, or
    /// the most every edge gives — what `most` says each edge's two faces give
    /// alone, and, for two parallel edges facing each other across a face they
    /// share, what keeps the set-backs they make on it from crossing, by the
    /// set-back of a value of one `set_back` says, and, for any two edges not
    /// meeting at a corner, what keeps their set-backs from reaching one point
    /// of a straight edge of the body bounding a face of each — the edge where
    /// the face one ends on meets a face of the other, or the edge between two
    /// that are not parallel on a face they share — each read on the body as
    /// given. Set-backs that would meet only on a curved edge are not measured:
    /// the laying declines them where they meet. The pairs whose set-backs meet
    /// across their face that `meeting` lays together, the earlier first, are each laid as one, the face they shared
    /// gone — by the prism it raises, or as one slope at the value their
    /// set-backs meet at, as [`Exact::sloped`] lays it; every other edge is
    /// laid alone. Two edges meeting at a convex corner are laid in turn as any
    /// two are, each prism carried past the corner along its edge's line, so
    /// that the later trims the earlier's face there: two flats or two rounds
    /// meet in a mitre, and the later edge is laid from its wedge on the body
    /// as given, its prism cut from what the earlier cuts left, as
    /// [`Exact::laid_as`] lays it, so that the matter of every edge's prism
    /// goes whatever the order — and an edge bounding the face two cuts laid
    /// together meet across, meeting both at a corner, meets their one prism
    /// the same way, at their value, as [`pairs::at_their_ends`] reads it, the
    /// two read on the body as given. Two edges of one sense meeting at a
    /// corner beside an edge of the other sense left out of the list, as
    /// [`corners::admit`] admits it — two cuts beside a concave edge, two fills
    /// beside a convex one — each stop flush there, and the later takes with it
    /// what both prisms hold past both faces they stop on, as [`Flush::beside`]
    /// reads it, cut with a cut and joined with a fill: their flats or rounds
    /// run on into a mitre, down to the third edge or outside it, left sharp.
    /// Two fills meeting where the third edge is concave too, at a pocket's
    /// bottom corner, are each trimmed flush on the face the other runs along
    /// and meet in a valley, the third edge left sharp one value short. A
    /// corner three of the edges meet at, all convex or all concave, is closed
    /// once all are laid, as `meeting` closes it by the way they meet, read
    /// where each two's set-backs cross on the face they share, as
    /// [`Exact::closed`] closes it, its face numbered past the edges'. A corner
    /// three of them meet at, two concave and the convex upright above them,
    /// is gone round its odd edge as `meeting` says, as
    /// [`Exact::round_the_odd`] lays it — the two others following the outline
    /// it leaves, with the foot of its face or through a piece of torus about
    /// its round — what closes it numbered past the edges'. The wall an edge's
    /// prism lays is numbered where the steps' numbers ended, plus the edge's
    /// rank in the list; a pair is laid at its later edge's turn, so
    /// that every number laid before stands below those a prism lays, as
    /// [`Exact::cut_along`] reads them. A value shorter than the kernel can set
    /// a cut back from an edge is declined first; a list of no edge
    /// `Unsupported`; an edge its name no longer finds `Lost`; an edge that is
    /// not a wedge, one edge two names find, two edges meeting at a corner that
    /// is neither convex — every edge ending there a wedge whose faces meet
    /// outwards — nor two of one sense beside an edge of the other left out of
    /// the list, nor two concave beside a third concave or the convex upright
    /// above them, more than three edges meeting at a corner, or three where
    /// another edge ends too, where `meeting` lays two together an edge
    /// meeting two others, or one of two laid together that meets another
    /// edge of the list at a corner any other way, a cut or a fill that left
    /// something else than its one face, as
    /// [`Exact::cut_along`] reads it, a face of an overlap no edge covered, and
    /// a corner gone round its odd edge as [`Exact::round_the_odd`] declines
    /// it, are declined `Unsupported`; a prism the tool cannot raise, as the
    /// tool says.
    pub(super) fn cut_off(
        &self,
        names: &[EdgeName],
        asked: f64,
        most: impl Fn(&Wedge) -> f64,
        set_back: impl Fn(&Wedge) -> f64,
        tool: impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
        meeting: Meeting,
    ) -> Result<(Exact, f64, Vec<pairs::Pair>), Declined> {
        if too_short(asked, self.brep.scale()) {
            return Err(Declined::Distance);
        }
        if names.is_empty() {
            return Err(Declined::Unsupported);
        }
        caught(|| {
            let kind = Kind {
                most: &most,
                set_back: &set_back,
                tool: &tool,
                meeting,
            };
            let step = self.step(names, &kind)?;
            let taken = step.bound(&self.brep, asked, &kind);
            let odd = odd::corners(&self.brep, &step)?;
            if odd.is_empty() {
                self.in_turn(names, step, taken, &kind)
            } else {
                self.round_the_odd(names, &step.wedges, &odd, taken, &kind)
            }
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
    /// read on the body as given. `mitred` numbers the faces laid before it at
    /// a corner it shares, which its own face meets in a mitre, and `open`
    /// holds where it stops at a corner an edge laid after it completes, as
    /// [`flush::open`] reads them.
    fn laid(
        &self,
        name: &EdgeName,
        taken: f64,
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
        mitred: &[u32],
        open: &[Stop],
    ) -> Result<Exact, Declined> {
        let (edge, wedge) = self.wedge(name, Wedge::read)?;
        let flush = Flush::at(&self.brep, edge);
        self.laid_as(&flush, &wedge, taken, tool, mitred, open)
    }

    /// The body with the edge of `wedge` cut off, or filled in, as
    /// [`Exact::laid`] lays it, its ends read where `flush` says `wedge` was
    /// read: on this body, or on the body as given where an earlier edge of
    /// the step was laid at a corner it shares, its prism then cut from, or
    /// joined to, what the earlier edges left — the matter of every edge's
    /// prism goes or comes whatever the order.
    fn laid_as(
        &self,
        flush: &Flush,
        wedge: &Wedge,
        taken: f64,
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
        mitred: &[u32],
        open: &[Stop],
    ) -> Result<Exact, Declined> {
        let cut = self.cut_or_filled(flush, wedge, taken, tool)?;
        if cut.cut_along(&[wedge], self, 1, mitred, open) {
            Ok(cut)
        } else {
            Err(Declined::Unsupported)
        }
    }

    /// The body with the edge of `wedge` cut off, or filled in, by the prism
    /// `tool` raises over it for `taken`, its wall numbered where the steps'
    /// numbers ended, as [`Exact::laid_as`] lays it but for the check, its
    /// ends read as `flush` says, as [`Exact::laid_flush`] lays it.
    fn cut_or_filled(
        &self,
        flush: &Flush,
        wedge: &Wedge,
        taken: f64,
        tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
    ) -> Result<Exact, Declined> {
        let bounds = self.brep.bounds().ok_or(Declined::Unsupported)?;
        let prism = tool(wedge, taken, self.brep.scale(), bounds)?;
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
        self.laid_flush(flush, wedge, &prism, tool)
    }

    /// Whether the prism left its face alone on the body, numbered where the
    /// steps' numbers of `before` ended and answering to the `laid` numbers
    /// from there, in one piece, every corner of it on one of the edges' faces
    /// — where the faces met at the edges' ends trim it — or a corner of a
    /// face numbered in `mitred`, where a mitre with it ends — at a corner
    /// that is not square, on a face of the other edge, short of the third —
    /// or on the plane of one of `open`, where an edge laid after it completes
    /// it, and every face `before` held but the edges' still there: no face,
    /// nor piece of a face, of the prism but that one — but an overlap's,
    /// numbered from [`OVERLAPS`], which an edge laid after it may cover, and
    /// [`Exact::overlaps_covered`] holds once every edge is laid.
    fn cut_along(
        &self,
        wedges: &[&Wedge],
        before: &Exact,
        laid: u32,
        mitred: &[u32],
        open: &[Stop],
    ) -> bool {
        let made = before.next;
        let eps = self.brep.scale().eps();
        let numbers = |body: &brep::Body, face: FaceId| body.numbers(face).to_vec();
        let stray = self
            .brep
            .face_ids()
            .flat_map(|face| numbers(&self.brep, face))
            .any(|number| number >= made + laid && number < OVERLAPS);
        let parted = self
            .roots
            .values()
            .any(|replaced| (made..PIECES).contains(replaced));
        let corners = |number: u32| {
            self.named(number as usize)
                .flat_map(|face| self.brep.face(face).loops.iter().flatten())
                .flat_map(|coedge| self.brep.edge(coedge.edge).ends.into_iter().flatten())
        };
        let mitres: BTreeSet<VertexId> =
            mitred.iter().flat_map(|&number| corners(number)).collect();
        let on_a_side = corners(made).all(|corner| {
            let point = self.brep.vertex(corner).point;
            mitres.contains(&corner)
                || open.iter().any(|stop| stop.holds(point, eps))
                || wedges.iter().any(|wedge| {
                    let at = point - wedge.point;
                    wedge.outward.iter().any(|side| side.dot(at).abs() <= eps)
                })
        });
        let consumable: BTreeSet<u32> = (wedges.iter())
            .flat_map(|wedge| wedge.numbers.iter().flatten().copied())
            .collect();
        let kept = before
            .brep
            .face_ids()
            .flat_map(|face| numbers(&before.brep, face))
            .filter(|number| !consumable.contains(number) && !(OVERLAPS..PIECES).contains(number))
            .all(|number| self.has_face(number as usize));
        !stray && !parted && self.has_face(made as usize) && on_a_side && kept
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
