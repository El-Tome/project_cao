//! A surface that never passes through itself.

use glam::DVec3;

use super::{Flaw, NEAR, Triangle, reach};
use crate::brep::crossing::{Facet, Lines, Pair, clear, measured, sliver};

mod pairs;

use pairs::{fans, for_each_pair_near};

/// Whether no face of a surface crosses another, or lies on another.
///
/// Faces may meet along the edges and corners they share, and a corner may lie
/// partway along a neighbour's edge. What they may not do is meet anywhere
/// else: a face whose inside is crossed by another face — whichever triangle
/// the crossing lands on — and two faces lying on each other over some area.
/// Nor may the surface pass through itself along a line of edges, which no two
/// faces show on their own: see `brep::crossing::Lines::passed`.
///
/// Within `NEAR` of the solid's reach counts as meeting: a crossing shorter
/// than that, or an overlap thinner, is what rounding leaves where a cut
/// passed, not a surface passing through itself.
pub fn uncrossed(triangles: &[Triangle]) -> Result<(), Flaw> {
    let room = NEAR * reach(triangles);
    let mut faces = Vec::with_capacity(triangles.len());
    for (index, corners) in triangles.iter().enumerate() {
        if !sliver(corners, room) {
            faces.push(Face::of(index, corners));
        }
    }
    let mut found = Found::default();
    for (one, other) in fans(&mut faces, room) {
        found
            .lines
            .share_edges(faces[one].indexed(), faces[other].indexed(), room);
    }
    for_each_pair_near(&faces, room, |one, other| found.weigh(one, other, room));
    let Some((one, other)) = found.first(triangles, room) else {
        return Ok(());
    };
    Err(Flaw::Crossing {
        first: Box::new(triangles[one]),
        second: Box::new(triangles[other]),
    })
}

/// Faces found crossing, and faces lying along another's edge, left for later.
#[derive(Default)]
struct Found {
    crossings: Vec<Pair>,
    lines: Lines,
}

impl Found {
    /// Weighs two faces against each other, both ways round. Faces with every
    /// corner more than `room` to one side of the other's plane cannot meet.
    fn weigh(&mut self, one: &Face, other: &Face, room: f64) {
        let (over_one, over_other) = (
            one.facet.heights(&other.facet),
            other.facet.heights(&one.facet),
        );
        if clear(over_one, room) || clear(over_other, room) {
            return;
        }
        if measured(&one.facet, &other.facet, over_one, over_other, room) {
            self.crossings.push((one.index, other.index));
        }
        let over = [over_one, over_other];
        self.lines.weigh(one.indexed(), other.indexed(), over, room);
    }

    /// The first two faces, by index, found to cross.
    fn first(mut self, triangles: &[Triangle], room: f64) -> Option<Pair> {
        let ordered = |(one, other): Pair| (one.min(other), one.max(other));
        let passed = self.lines.passed(triangles, room);
        self.crossings.into_iter().chain(passed).map(ordered).min()
    }
}

/// A triangle by its index among the faces, with its box and its fans.
struct Face {
    index: usize,
    facet: Facet,
    bounds: [DVec3; 2],
    /// The fan round each corner this face belongs to, from one; none is nought.
    fans: [usize; 3],
}

impl Face {
    fn of(index: usize, corners: &Triangle) -> Self {
        let [a, b, c] = *corners;
        Self {
            index,
            facet: Facet::of(corners),
            bounds: [a.min(b).min(c), a.max(b).max(c)],
            fans: [0; 3],
        }
    }

    fn indexed(&self) -> (usize, &Facet) {
        (self.index, &self.facet)
    }

    fn span(&self, way: DVec3) -> [f64; 2] {
        let [a, b, c] = self.facet.corners.map(|corner| corner.dot(way));
        [a.min(b).min(c), a.max(b).max(c)]
    }
}

#[cfg(test)]
mod tests;
