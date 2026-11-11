//! The edges of one step on the body they are laid on: what its names find
//! there and the corners they meet at, what a kind lays them with, the one
//! value they take, and the edges laid in turn at that value — two whose
//! set-backs meet across the face they share as one, a later edge mitred
//! against the earlier ones it meets at a corner, and the corners three of
//! them meet at closed once all are laid.

use std::collections::BTreeSet;

use glam::DVec3;

use super::corners::{self, Beside, Three};
use super::extent::runs;
use super::flush::{self, Flush, Held};
use super::reach::reach_within;
use super::{Meeting, Prism, Together, Wedge, closing, pairs, set_backs};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, EdgeName, Scale};

/// What a step's names find on the body it is laid on: each edge, once, and
/// its wedge, by its rank in the list; each two facing each other across a
/// face they share, as [`pairs::facing`] reads them; the corners two of them
/// meet at beside a third that is not convex with them, as
/// [`corners::admit`] takes them; the corners three of them meet at, as
/// [`corners::of_three`] reads them; and the ends of cuts and fills held on
/// a plane of their own, none unless a step going round an odd edge holds
/// them (#846).
pub(super) struct Step {
    pub edges: Vec<EdgeId>,
    pub wedges: Vec<Wedge>,
    facing: Vec<(usize, usize, f64)>,
    beside: Vec<Beside>,
    pub three: Vec<Three>,
    pub held: Vec<Held>,
}

impl Exact {
    /// The step the names make on this body, each edge's reach read only
    /// where its blend runs once it is laid, as [`runs`] holds it for the
    /// kind's `set_back`, with [`reach_within`]. Declined `Lost` where a name
    /// no longer finds its edge, and `Unsupported` where an edge is not a
    /// wedge, two names find one edge, or the corners are declined as
    /// [`corners::admit`] and [`corners::of_three`] decline them.
    pub(super) fn step(&self, names: &[EdgeName], kind: &Kind) -> Result<Step, Declined> {
        let found = names
            .iter()
            .map(|name| self.wedge(name, Wedge::of))
            .collect::<Result<Vec<(EdgeId, Wedge)>, Declined>>()?;
        let edges: Vec<EdgeId> = found.iter().map(|&(edge, _)| edge).collect();
        let once = edges.iter().collect::<BTreeSet<_>>().len() == edges.len();
        if !once {
            return Err(Declined::Unsupported);
        }
        let beside = corners::admit(&self.brep, &edges)?;
        let three = corners::of_three(&self.brep, &edges)?;
        let mut wedges: Vec<Wedge> = found.into_iter().map(|(_, wedge)| wedge).collect();
        let reached = |rank: usize, side: usize| {
            let ends = runs(
                &self.brep,
                (&edges, &wedges),
                &beside,
                (rank, side),
                rank,
                &kind.set_back,
            );
            let wedge = &wedges[rank];
            let from = (wedge.into[side], wedge.point);
            reach_within(&self.brep, wedge.faces[side], from, &ends)
        };
        let reaches = (0..wedges.len())
            .map(|rank| Some(reached(rank, 0)?.min(reached(rank, 1)?)))
            .collect::<Option<Vec<f64>>>()
            .ok_or(Declined::Unsupported)?;
        for (wedge, reach) in wedges.iter_mut().zip(reaches) {
            wedge.reach = reach;
        }
        let facing = pairs::facing(&wedges);
        Ok(Step {
            edges,
            wedges,
            facing,
            beside,
            three,
            held: Vec::new(),
        })
    }
}

/// What a kind lays a step's edges with, as [`Exact::cut_off`] takes it:
/// how far each edge's two faces let it go alone, the set-back along them a
/// value of one makes, the prism raised over an edge for a value, and how its
/// edges meet.
#[derive(Clone, Copy)]
pub(super) struct Kind<'a> {
    pub most: &'a dyn Fn(&Wedge) -> f64,
    pub set_back: &'a dyn Fn(&Wedge) -> f64,
    pub tool: &'a Tool<'a>,
    pub meeting: Meeting,
}

/// What raises the prism over an edge's wedge for a value, at the body's
/// scale, within its box.
pub(super) type Tool<'a> =
    dyn Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined> + 'a;

impl Step {
    /// `asked`, or the most every edge gives, on `body`, as
    /// [`Exact::cut_off`] reads it: what the kind's `most` says each edge's
    /// two faces give alone, what keeps two facing each other across a face
    /// from crossing on it, and what keeps two not meeting at a corner from
    /// reaching one point of an edge of the body, each set back the kind's
    /// `set_back` of a value of one.
    pub fn bound(&self, body: &brep::Body, asked: f64, kind: &Kind) -> f64 {
        let alone = (self.wedges.iter()).fold(asked, |taken, wedge| taken.min((kind.most)(wedge)));
        let set_back = kind.set_back;
        alone
            .min(pairs::bound(&self.wedges, &self.facing, set_back))
            .min(set_backs::bound(
                body,
                &self.edges,
                &self.wedges,
                &self.beside,
                set_back,
            ))
    }
}

impl Exact {
    /// The body with the edges of `step`, which `names` found on it, laid in
    /// turn at `taken`, as [`Exact::cut_off`] lays them — with the pairs laid
    /// as one — each set back the kind's `set_back` of the value, by the
    /// prisms its `tool` raises, its `meeting` laying the pairs together and
    /// closing the corners three meet at, the faces numbered from where the
    /// steps' numbers ended by the edges' ranks, the closings' past them in
    /// the order of the step's corners. Declined as [`Exact::cut_off`] says.
    pub(super) fn in_turn(
        &self,
        names: &[EdgeName],
        step: Step,
        taken: f64,
        kind: &Kind,
    ) -> Result<(Exact, f64, Vec<pairs::Pair>), Declined> {
        let Kind {
            set_back,
            tool,
            meeting,
            ..
        } = *kind;
        let Step {
            edges,
            wedges,
            facing,
            beside,
            three,
            held,
        } = step;
        let scale = self.brep.scale();
        let pairs = pairs::meeting(&wedges, &facing, taken, set_back, meeting.together, scale)?;
        let ends = pairs::at_their_ends(&self.brep, &edges, &wedges, &pairs)?;
        let closes = closing::closings(
            three,
            &wedges,
            |w| taken * set_back(w),
            taken,
            meeting.closing,
        )?;
        let given: Vec<(EdgeId, &Wedge)> = edges.iter().copied().zip(&wedges).collect();
        let crossing = set_backs::crossing_on(&self.brep, &edges, &wedges, set_back);
        let start = self.next;
        let mut cut = self.clone();
        for rank in 0..names.len() {
            if pairs.iter().any(|&(earlier, _, _)| earlier == rank) {
                continue;
            }
            cut.next = start + rank as u32;
            let partner = pairs.iter().find(|&&(_, later, _)| later == rank);
            cut = match (partner, meeting.together) {
                (Some(&(earlier, _, _)), Together::Raised(raise)) => {
                    let pair = [given[earlier], given[rank]];
                    cut.laid_together(&self.brep, pair, start + earlier as u32, raise)?
                }
                (Some(&(earlier, _, value)), Together::Sloped) => cut.sloped(
                    [&names[earlier], &names[rank]],
                    start + earlier as u32,
                    value,
                    &tool,
                )?,
                (None, _) => {
                    let mitred: Vec<u32> = (0..rank)
                        .filter(|&earlier| corners::meet(&self.brep, [edges[earlier], edges[rank]]))
                        .map(|earlier| start + earlier as u32)
                        .collect();
                    let mut open = flush::open(&self.brep, rank, &given, &beside);
                    open.extend(flush::completing(&self.brep, rank, &given, &crossing));
                    let value = ends.get(&rank).copied().unwrap_or(taken);
                    if mitred.is_empty() {
                        cut.laid(&names[rank], value, &tool, (&mitred, &open), &held)?
                    } else {
                        let flush = Flush::beside(
                            &self.brep, rank, &given, &beside, value, &set_back, &tool,
                        )?
                        .holding(&held);
                        cut.laid_as(&flush, &wedges[rank], value, &tool, &mitred, &open)?
                    }
                }
            };
        }
        cut.overlaps_covered()?;
        cut.next = start + names.len() as u32;
        Ok((cut.closed(&closes, start)?, taken, pairs))
    }
}
