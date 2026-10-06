//! A body's own account of what it is made of, held to the geometry behind
//! it.
//!
//! Written apart from the kernel: every surface and every curve is evaluated
//! here by its own formula, so that a listing the kernel got wrong is not
//! checked by the same mistake. The curve two perpendicular cylinders meet
//! along is the one exception, evaluated by its own `point`: it has no
//! formula short enough to write twice.

use std::f64::consts::TAU;

use crate::brep::{Curve, Listing};

mod lying;
mod turning;

/// What a listing says that its geometry does not bear out, and where: faces,
/// edges and vertices by their rank in the listing, a loop by its rank in its
/// face, a use by its rank in its loop.
#[derive(Clone, Debug, PartialEq)]
pub enum Mislisted {
    /// A loop naming an edge the listing does not hold.
    NoSuchEdge { face: usize, edge: usize },
    /// An edge naming a vertex the listing does not hold.
    NoSuchVertex { edge: usize, vertex: usize },
    /// A face holding within it a vertex the listing does not hold.
    NoSuchApex { face: usize, vertex: usize },
    /// An edge naming, beside it, a face the listing does not hold.
    NoSuchFace { edge: usize, face: usize },
    /// An edge whose sides are not the faces whose loops use it.
    Sides { edge: usize },
    /// An edge run more times one way than the other: a face missing beside
    /// it, or one laid twice.
    Unbalanced {
        edge: usize,
        forward: usize,
        backward: usize,
    },
    /// An edge no face uses, which bounds nothing.
    Unused { edge: usize },
    /// A face with no loop round it: a whole plane or a whole cylinder, which
    /// no solid is bounded by.
    Unbounded { face: usize },
    /// A use that does not start where the one before it in its loop ends,
    /// or a whole closed curve sharing its loop with another use.
    Unclosed { face: usize, lap: usize, at: usize },
    /// An edge with no vertex that is not a whole closed curve.
    Endless { edge: usize },
    /// A vertex standing off the surface of a face around it, or of the face
    /// holding it within it.
    VertexOffFace {
        vertex: usize,
        face: usize,
        distance: f64,
    },
    /// An edge whose curve, at the start of its stretch (`end` nought) or at
    /// its end (one), is not at the vertex it names there.
    EndAway {
        edge: usize,
        end: usize,
        vertex: usize,
        distance: f64,
    },
    /// An edge whose curve, at its parameter `at`, stands off the surface of
    /// a face beside it.
    OffFace {
        edge: usize,
        face: usize,
        at: f64,
        distance: f64,
    },
    /// A loop keeping its face on its right seen from outside the matter:
    /// run the wrong way round, or on a face said to look the wrong way.
    Backwards { face: usize, lap: usize },
}

/// Whether a listing holds together and stands on its geometry, to within a
/// billionth of how far the body reaches: the kernel's own tolerance, under
/// which it merges two places into one.
pub fn listed(listing: &Listing, reach: f64) -> Result<(), Mislisted> {
    ranked(listing)?;
    let uses = uses(listing);
    for (rank, edge) in listing.edges.iter().enumerate() {
        let mut listed = edge.sides.clone();
        let mut found = uses[rank].clone();
        listed.sort_unstable();
        found.sort_unstable();
        if listed != found {
            return Err(Mislisted::Sides { edge: rank });
        }
        let forward = found.iter().filter(|(_, forward)| *forward).count();
        let backward = found.len() - forward;
        if forward != backward {
            return Err(Mislisted::Unbalanced {
                edge: rank,
                forward,
                backward,
            });
        }
        if found.is_empty() {
            return Err(Mislisted::Unused { edge: rank });
        }
    }
    closed(listing)?;
    whole(listing, reach)?;
    lying::lying(listing, &uses, ON * reach.max(1.0))?;
    turning::turning(listing, ON * reach.max(1.0))
}

/// Every rank the listing names is one it holds.
fn ranked(listing: &Listing) -> Result<(), Mislisted> {
    for (face, listed) in listing.faces.iter().enumerate() {
        if let Some(&(edge, _)) = listed
            .loops
            .iter()
            .flatten()
            .find(|(edge, _)| *edge >= listing.edges.len())
        {
            return Err(Mislisted::NoSuchEdge { face, edge });
        }
        if let Some(vertex) = listed
            .apex
            .filter(|vertex| *vertex >= listing.vertices.len())
        {
            return Err(Mislisted::NoSuchApex { face, vertex });
        }
    }
    for (edge, listed) in listing.edges.iter().enumerate() {
        if let Some(&vertex) = listed
            .ends
            .iter()
            .flatten()
            .find(|vertex| **vertex >= listing.vertices.len())
        {
            return Err(Mislisted::NoSuchVertex { edge, vertex });
        }
        if let Some(&(face, _)) = listed
            .sides
            .iter()
            .find(|(face, _)| *face >= listing.faces.len())
        {
            return Err(Mislisted::NoSuchFace { edge, face });
        }
    }
    Ok(())
}

/// For every edge, the faces whose loops use it and which way, in the order
/// the faces come.
fn uses(listing: &Listing) -> Vec<Vec<(usize, bool)>> {
    let mut uses = vec![Vec::new(); listing.edges.len()];
    for (face, listed) in listing.faces.iter().enumerate() {
        for &(edge, forward) in listed.loops.iter().flatten() {
            uses[edge].push((face, forward));
        }
    }
    uses
}

/// Every face has a loop, and every loop closes: each use starts at the
/// vertex the one before it ends at, and a whole closed curve, which has no
/// vertex, stands alone.
fn closed(listing: &Listing) -> Result<(), Mislisted> {
    let ends = |(edge, forward): (usize, bool)| {
        listing.edges[edge]
            .ends
            .map(|[from, to]| if forward { [from, to] } else { [to, from] })
    };
    for (face, listed) in listing.faces.iter().enumerate() {
        if listed.loops.is_empty() {
            return Err(Mislisted::Unbounded { face });
        }
        for (lap, uses) in listed.loops.iter().enumerate() {
            let open = Err(Mislisted::Unclosed { face, lap, at: 0 });
            let Some(&last) = uses.last() else {
                return open;
            };
            if uses.len() == 1 && ends(last).is_none() {
                continue;
            }
            let mut before = ends(last);
            for (at, &used) in uses.iter().enumerate() {
                let now = ends(used);
                match (before, now) {
                    (Some([_, end]), Some([start, _])) if end == start => {}
                    _ => return Err(Mislisted::Unclosed { face, lap, at }),
                }
                before = now;
            }
        }
    }
    Ok(())
}

/// Every edge with no vertex runs round a whole closed curve.
fn whole(listing: &Listing, reach: f64) -> Result<(), Mislisted> {
    let room = ON * reach.max(1.0);
    for (rank, edge) in listing.edges.iter().enumerate() {
        if edge.ends.is_some() {
            continue;
        }
        let round = match edge.curve {
            Curve::Line(_) => false,
            Curve::Circle(_) => ((edge.to - edge.from).abs() - TAU).abs() <= ON * TAU,
            Curve::Meet(meet) => {
                edge.from != edge.to
                    && (meet.point(edge.from) - meet.point(edge.to)).length() <= room
            }
        };
        if !round {
            return Err(Mislisted::Endless { edge: rank });
        }
    }
    Ok(())
}

/// How far from a surface a place may stand and still lie on it, as a
/// fraction of the reach.
const ON: f64 = 1e-9;

#[cfg(test)]
mod tests;
