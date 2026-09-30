//! A surface that never passes through itself.

use glam::DVec3;

use super::{Flaw, NEAR, Triangle, reach};

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

/// Whether a triangle's corners all lie within `room` of one line, as a fan
/// leaves across three corners in a row: nothing inside it to cross.
fn sliver(corners: &Triangle, room: f64) -> bool {
    let [a, b, c] = *corners;
    let longest = a.distance(b).max(b.distance(c)).max(c.distance(a));
    (b - a).cross(c - a).length() <= room * longest
}

/// `one × other` by Kahan's difference of products, each coordinate rounded
/// once: across a face a few billionths wide the plain product tilts the plane
/// enough to lift one end of a parallel neighbour and sink the other.
fn cross(one: DVec3, other: DVec3) -> DVec3 {
    let difference = |a: f64, b: f64, c: f64, d: f64| a.mul_add(b, -c * d) - c.mul_add(d, -c * d);
    let across = |(i, j)| difference(one[i], other[j], one[j], other[i]);
    DVec3::from_array([(1, 2), (2, 0), (0, 1)].map(across))
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
        let resting = |heights: [f64; 3]| heights.iter().filter(|high| high.abs() <= room).count();
        let clear = |heights| resting(heights) == 0 && sides(heights, room) != [true; 2];
        if clear(one.heights(other)) || clear(other.heights(one)) {
            return;
        }
        let (over_one, over_other) = (one.heights(other), other.heights(one));
        self.meet(one, other, over_one, over_other, room);
        self.meet(other, one, over_other, over_one, room);
        if resting(over_one) > 1 && resting(over_other) > 1 {
            self.share_edges(one, other, room);
        }
    }

    /// Notes how `by`, its corners `heights` above `on`, meets it; `back` is
    /// the other way round. Faces cross only where each has corners more than
    /// `room` away on both sides of the other: a corner sunk by less rests on
    /// the face, and a thin face's plane, which rounding tilts, is not enough.
    fn meet(&mut self, on: &Face, by: &Face, heights: [f64; 3], back: [f64; 3], room: f64) {
        let [above, below] = sides(heights, room);
        let rests = |at: usize| heights[at].abs() <= room;
        let edge_down = (0..3).find(|at| rests(*at) && rests((at + 1) % 3));
        if !above && !below {
            if on.overlaps(by, room) {
                self.crossings.push((on.index, by.index));
            }
        } else if let (Some(at), false) = (edge_down, above && below) {
            let edge = [by.corners[at], by.corners[(at + 1) % 3]];
            let inside = on.inside(edge, room);
            if let Some(span) = inside.and_then(|along| stretch(edge, along, room)) {
                self.note(3 * by.index + at, on.index, span, None);
            }
        } else if above && below && sides(back, room) == [true; 2] {
            let mut meeting = Vec::with_capacity(2);
            for here in 0..3 {
                let (next, height) = ((here + 1) % 3, heights[here]);
                if rests(here) {
                    meeting.push(by.corners[here]);
                } else if !rests(next) && (height > 0.0) != (heights[next] > 0.0) {
                    let along = height / (height - heights[next]);
                    meeting.push(by.corners[here].lerp(by.corners[next], along));
                }
            }
            if on.inside([meeting[0], meeting[1]], room).is_some() {
                self.crossings.push((on.index, by.index));
            }
        }
    }

    /// Notes each stretch of edge the two faces run along together.
    fn share_edges(&mut self, one: &Face, other: &Face, room: f64) {
        for (this, that) in [(one, other), (other, one)] {
            for (mine, theirs) in (0..9).map(|pair| (pair / 3, pair % 3)) {
                let edge = [this.corners[mine], this.corners[(mine + 1) % 3]];
                let [from, to] = [that.corners[theirs], that.corners[(theirs + 1) % 3]];
                if let Some(span) = stretch(edge, [from, to], room) {
                    let wing = that.corners[(theirs + 2) % 3];
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

/// Whether some corner stands more than `room` above a plane, and some below.
fn sides(heights: [f64; 3], room: f64) -> [bool; 2] {
    [1.0, -1.0].map(|sign| heights.iter().any(|height| sign * height > room))
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

/// A triangle with its plane, and the edges bounding it within that plane.
#[derive(Default)]
struct Face {
    index: usize,
    corners: Triangle,
    normal: DVec3,
    /// Each edge as a plane square to the face: the way in, and how far along it.
    walls: [(DVec3, f64); 3],
    bounds: [DVec3; 2],
    /// The fan round each corner this face belongs to, from one; none is nought.
    fans: [usize; 3],
}

impl Face {
    fn of(index: usize, corners: &Triangle) -> Self {
        let [a, b, c] = *corners;
        let normal = cross(b - a, c - a).normalize_or_zero();
        let walls = [0, 1, 2].map(|at| {
            let [start, end] = [corners[at], corners[(at + 1) % 3]];
            let inward = normal.cross(end - start).normalize_or_zero();
            (inward, inward.dot(start))
        });
        let (corners, bounds) = (*corners, [a.min(b).min(c), a.max(b).max(c)]);
        Self {
            index,
            corners,
            normal,
            walls,
            bounds,
            ..Self::default()
        }
    }

    fn span(&self, way: DVec3) -> [f64; 2] {
        let [a, b, c] = self.corners.map(|corner| corner.dot(way));
        [a.min(b).min(c), a.max(b).max(c)]
    }

    /// How far above this face's plane each corner of `other` stands.
    fn heights(&self, other: &Face) -> [f64; 3] {
        let base = self.corners[0];
        other.corners.map(|corner| self.normal.dot(corner - base))
    }

    /// What of `points` — a segment, or a polygon in this face's plane —
    /// stands over this face, deeper inside it than `margin` from each edge.
    fn clip(&self, points: &[DVec3], margin: f64) -> Vec<DVec3> {
        let mut kept = points.to_vec();
        for (inward, level) in self.walls {
            let depth = |point: DVec3| inward.dot(point) - level - margin;
            let mut cut = Vec::with_capacity(kept.len() + 1);
            for (at, here) in kept.iter().enumerate() {
                let next = kept[(at + 1) % kept.len()];
                let (deep, onward) = (depth(*here), depth(next));
                cut.extend((deep >= 0.0).then_some(*here));
                let crossed = (deep >= 0.0) != (onward >= 0.0);
                cut.extend(crossed.then(|| here.lerp(next, deep / (deep - onward))));
            }
            kept = cut;
        }
        kept
    }

    /// The part of a segment standing over this face more than `room` inside
    /// its edges, when that part is longer than `room`.
    fn inside(&self, segment: [DVec3; 2], room: f64) -> Option<[DVec3; 2]> {
        let along = |point: &DVec3| point.dot(segment[1] - segment[0]);
        let mut kept = self.clip(&segment, room);
        kept.sort_by(|one, other| along(one).total_cmp(&along(other)));
        let (from, to) = (*kept.first()?, *kept.last()?);
        (from.distance(to) > room).then_some([from, to])
    }

    /// Whether `other`, lying in this face's plane, covers it thicker than `room`
    /// — twice the area shared over its outline, as a strip's thickness is.
    fn overlaps(&self, other: &Face, room: f64) -> bool {
        let kept = self.clip(&other.corners, 0.0);
        let (mut doubled_area, mut outline) = (0.0, 0.0);
        for (at, here) in kept.iter().enumerate() {
            let next = kept[(at + 1) % kept.len()];
            doubled_area += (*here - kept[0]).cross(next - kept[0]).dot(self.normal);
            outline += here.distance(next);
        }
        doubled_area.abs() > room * outline
    }
}

#[cfg(test)]
mod tests;
