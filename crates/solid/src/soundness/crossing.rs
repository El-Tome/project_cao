//! A surface that never passes through itself.

use glam::DVec3;

use super::{Flaw, NEAR, Triangle, reach};
use crate::brep::crossing::{Facet, clear, cross, crossing, resting, sides, sliver};

mod pairs;

use pairs::{fans, for_each_pair_near};

/// Two faces, by their index among the triangles.
type Pair = (usize, usize);

/// Whether no face of a surface crosses another, or lies on another.
///
/// Faces may meet along the edges and corners they share, and a corner may lie
/// partway along a neighbour's edge. What they may not do is meet anywhere
/// else: a face whose inside is crossed by another face — whichever triangle
/// the crossing lands on — and two faces lying on each other over some area.
/// Nor may the surface pass through itself along a line of edges, which no two
/// faces show on their own: see `Found::passed`.
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
        found.share_edges(&faces[one], &faces[other], room);
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
    lines: Vec<Alongside>,
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
        if crossing(&one.facet, &other.facet, room) {
            self.crossings.push((one.index, other.index));
        }
        self.lean(one, other, over_one, room);
        self.lean(other, one, over_other, room);
        if resting(over_one, room) > 1 && resting(over_other, room) > 1 {
            self.share_edges(one, other, room);
        }
    }

    /// Notes the stretch of an edge of `by`, its corners `heights` above `on`,
    /// that rests on `on` inside it while its third corner stands to one side.
    fn lean(&mut self, on: &Face, by: &Face, heights: [f64; 3], room: f64) {
        let rests = |at: usize| heights[at].abs() <= room;
        let Some(at) = (0..3).find(|at| rests(*at) && rests((at + 1) % 3)) else {
            return;
        };
        let [above, below] = sides(heights, room);
        if above == below {
            return;
        }
        let edge = [by.facet.corners[at], by.facet.corners[(at + 1) % 3]];
        let inside = on.facet.inside(edge, room);
        if let Some(span) = inside.and_then(|along| stretch(edge, along, room)) {
            self.note(3 * by.index + at, on.index, span, None);
        }
    }

    /// Notes each stretch of edge the two faces run along together.
    fn share_edges(&mut self, one: &Face, other: &Face, room: f64) {
        for (this, that) in [(one, other), (other, one)] {
            for (mine, theirs) in (0..9).map(|pair| (pair / 3, pair % 3)) {
                let edge = [this.facet.corners[mine], this.facet.corners[(mine + 1) % 3]];
                let [from, to] = [
                    that.facet.corners[theirs],
                    that.facet.corners[(theirs + 1) % 3],
                ];
                if let Some(span) = stretch(edge, [from, to], room) {
                    let wing = that.facet.corners[(theirs + 2) % 3];
                    let ahead = (edge[1] - edge[0]).dot(to - from) > 0.0;
                    self.note(3 * this.index + mine, that.index, span, Some((wing, ahead)));
                }
            }
        }
    }

    fn note(&mut self, anchor: usize, face: usize, span: [f64; 2], ends: Option<(DVec3, bool)>) {
        self.lines.push(Alongside {
            anchor,
            face,
            span,
            ends,
        });
    }

    /// The first two faces, by index, found to cross.
    fn first(mut self, triangles: &[Triangle], room: f64) -> Option<Pair> {
        let ordered = |(one, other): Pair| (one.min(other), one.max(other));
        let passed = self.passed(triangles, room);
        self.crossings.into_iter().chain(passed).map(ordered).min()
    }

    /// The faces a surface passes through itself between along a line of
    /// edges, which no two faces show alone: stretch by stretch along each edge.
    ///
    /// A face the line lies inside, with faces ending on it from both sides,
    /// is passed through; from one side only, they are a ridge resting on it.
    /// Faces all ending on the line: a skin wraps matter, so going round, each
    /// runs along it the other way from the next. Two skins crossing, cut along
    /// the line and every piece kept, break that. Faces not running one way as
    /// often as the other leave the skin open, and that is for `closed`.
    fn passed(&mut self, triangles: &[Triangle], room: f64) -> Vec<Pair> {
        self.lines.sort_by_key(|each| (each.anchor, each.face));
        self.lines.dedup_by_key(|each| (each.anchor, each.face));
        let mut passed = Vec::new();
        for around in self.lines.chunk_by(|one, other| one.anchor == other.anchor) {
            let (index, edge) = (around[0].anchor / 3, around[0].anchor % 3);
            let [start, end, wing] = [0, 1, 2].map(|step| triangles[index][(edge + step) % 3]);
            let out = (wing - start).reject_from(end - start).normalize_or_zero();
            let up = (end - start).normalize_or_zero().cross(out);
            let angle = |point: DVec3| up.dot(point - start).atan2(out.dot(point - start));
            let mut cuts = vec![0.0, start.distance(end)];
            for alongside in around {
                cuts.extend(alongside.span.map(|at| at.clamp(0.0, start.distance(end))));
            }
            cuts.sort_by(f64::total_cmp);
            for stretch in cuts.windows(2).filter(|pair| pair[1] - pair[0] > room) {
                let middle = (stretch[0] + stretch[1]) / 2.0;
                let covers = |each: &&Alongside| each.span[0] < middle && middle < each.span[1];
                let mut round = vec![(0.0, true, index, wing)];
                for each in around.iter().filter(covers) {
                    if let Some((wing, ahead)) = each.ends {
                        round.push((angle(wing), ahead, each.face, wing));
                    }
                }
                round.sort_by(|one, other| one.0.total_cmp(&other.0).then(one.2.cmp(&other.2)));
                let balanced = 2 * round.iter().filter(|face| face.1).count() == round.len();
                for (one, next) in round.iter().zip(round.iter().cycle().skip(1)) {
                    if balanced && one.1 == next.1 {
                        passed.push((one.2, next.2));
                    }
                }
                let inside = |each: &&Alongside| covers(each) && each.ends.is_none();
                for inside in around.iter().filter(inside) {
                    let [a, b, c] = triangles[inside.face];
                    let normal = cross(b - a, c - a).normalize_or_zero();
                    let height = |point: DVec3| normal.dot(point - a);
                    let side = |sign: f64| round.iter().find(|one| sign * height(one.3) > room);
                    if let (Some(above), Some(below)) = (side(1.0), side(-1.0)) {
                        passed.extend([(inside.face, above.2), (inside.face, below.2)]);
                    }
                }
            }
        }
        passed
    }
}

/// A face along a `span` of `anchor`, another face's edge counted three to a
/// face. One that `ends` on the line reaches a corner out from it and runs
/// ahead, the anchor's way, or not; one the line lies inside does neither.
struct Alongside {
    anchor: usize,
    face: usize,
    span: [f64; 2],
    ends: Option<(DVec3, bool)>,
}

/// How far along `edge`, from its start, a segment lying on the same line
/// runs: when it does lie on that line, and shares more than `room` of it.
fn stretch([start, end]: [DVec3; 2], other: [DVec3; 2], room: f64) -> Option<[f64; 2]> {
    let (length, along) = (start.distance(end), (end - start).normalize_or_zero());
    let off = |point: &DVec3| (*point - start).reject_from_normalized(along).length() > room;
    let [one, two] = other.map(|point| along.dot(point - start));
    let span = [one.min(two), one.max(two)];
    let shared = span[1].min(length) - span[0].max(0.0) > room;
    (shared && !other.iter().any(off)).then_some(span)
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

    fn span(&self, way: DVec3) -> [f64; 2] {
        let [a, b, c] = self.facet.corners.map(|corner| corner.dot(way));
        [a.min(b).min(c), a.max(b).max(c)]
    }
}

#[cfg(test)]
mod tests;
