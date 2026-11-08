//! Whether triangles pass through each other or lie on each other, two at a
//! time or around a line of edges: the one definition the tessellation's folds
//! draw by, and the rules' check that no face of a solid crosses another
//! weighs its faces with.

use glam::DVec3;

type Triangle = [DVec3; 3];

/// Two points closer than this share a place, as a fraction of how far the
/// solid reaches: the room two triangles meet within rather than cross.
///
/// Relative rather than absolute for the reason `boolean.rs` gives: the noise
/// of a coordinate grows with its size. A ten-billionth is a million times the
/// noise a cut leaves on a corner written twice, and ten times finer than the
/// billionth [`super::scale::Scale::eps`] judges two faces one plane by — so a
/// crack that tolerance leaves is seen, rather than welded shut by the
/// definition looking for it. Two faces closer than this cannot be told from one face
/// laid twice, which is why the cases drawn never ask for a gap that fine.
pub const NEAR: f64 = 1e-10;

/// How far a solid reaches from the origin, never less than one: the scale
/// every tolerance here is taken against.
pub fn reach(triangles: &[Triangle]) -> f64 {
    triangles
        .iter()
        .flatten()
        .fold(1.0f64, |far, corner| far.max(corner.abs().max_element()))
}

/// Whether two faces cross: the inside of one crossed by the other, or the two
/// lying on each other over some area.
///
/// Within `room` counts as meeting: a crossing shorter than that, or an
/// overlap thinner, is what rounding leaves where a cut passed. A face whose
/// corners all lie within `room` of one line has nothing inside it to cross.
pub(crate) fn crossing(one: &Facet, other: &Facet, room: f64) -> bool {
    if sliver(&one.corners, room) || sliver(&other.corners, room) {
        return false;
    }
    let (over_one, over_other) = (one.heights(other), other.heights(one));
    !clear(over_one, room)
        && !clear(over_other, room)
        && measured(one, other, over_one, over_other, room)
}

/// [`crossing`] for two faces neither a sliver nor clear of the other, each's
/// corners already measured above the other's plane.
pub(crate) fn measured(
    one: &Facet,
    other: &Facet,
    over_one: [f64; 3],
    over_other: [f64; 3],
    room: f64,
) -> bool {
    meets(one, other, over_one, over_other, room) || meets(other, one, over_other, over_one, room)
}

/// Whether `by`, its corners `heights` above `on`, crosses it; `back` is the
/// other way round. Faces cross only where each has corners more than `room`
/// away on both sides of the other: a corner sunk by less rests on the face,
/// and a thin face's plane, which rounding tilts, is not enough.
fn meets(on: &Facet, by: &Facet, heights: [f64; 3], back: [f64; 3], room: f64) -> bool {
    let [above, below] = sides(heights, room);
    if !above && !below {
        return on.overlaps(by, room);
    }
    if !(above && below && sides(back, room) == [true; 2]) {
        return false;
    }
    let rests = |at: usize| heights[at].abs() <= room;
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
    on.inside([meeting[0], meeting[1]], room).is_some()
}

/// Whether a triangle's corners all lie within `room` of one line, as a fan
/// leaves across three corners in a row: nothing inside it to cross.
pub(crate) fn sliver(corners: &Triangle, room: f64) -> bool {
    let [a, b, c] = *corners;
    let longest = a.distance(b).max(b.distance(c)).max(c.distance(a));
    (b - a).cross(c - a).length() <= room * longest
}

/// How many corners, `heights` above a plane, rest on it within `room`.
pub(crate) fn resting(heights: [f64; 3], room: f64) -> usize {
    heights.iter().filter(|high| high.abs() <= room).count()
}

/// Whether a face, its corners `heights` above a plane, stands clear of it:
/// no corner resting on it, and every one to the same side.
pub(crate) fn clear(heights: [f64; 3], room: f64) -> bool {
    resting(heights, room) == 0 && sides(heights, room) != [true; 2]
}

/// Whether some corner stands more than `room` above a plane, and some below.
pub(crate) fn sides(heights: [f64; 3], room: f64) -> [bool; 2] {
    [1.0, -1.0].map(|sign| heights.iter().any(|height| sign * height > room))
}

/// `one × other` by Kahan's difference of products, each coordinate rounded
/// once: across a face a few billionths wide the plain product tilts the plane
/// enough to lift one end of a parallel neighbour and sink the other.
pub(crate) fn cross(one: DVec3, other: DVec3) -> DVec3 {
    let difference = |a: f64, b: f64, c: f64, d: f64| a.mul_add(b, -c * d) - c.mul_add(d, -c * d);
    let across = |(i, j)| difference(one[i], other[j], one[j], other[i]);
    DVec3::from_array([(1, 2), (2, 0), (0, 1)].map(across))
}

/// A triangle with its plane, and the edges bounding it within that plane.
pub(crate) struct Facet {
    pub(crate) corners: Triangle,
    pub(crate) normal: DVec3,
    /// Each edge as a plane square to the face: the way in, and how far along it.
    walls: [(DVec3, f64); 3],
}

impl Facet {
    pub(crate) fn of(corners: &Triangle) -> Self {
        let [a, b, c] = *corners;
        let normal = cross(b - a, c - a).normalize_or_zero();
        let walls = [0, 1, 2].map(|at| {
            let [start, end] = [corners[at], corners[(at + 1) % 3]];
            let inward = normal.cross(end - start).normalize_or_zero();
            (inward, inward.dot(start))
        });
        Self {
            corners: *corners,
            normal,
            walls,
        }
    }

    /// How far above this face's plane each corner of `other` stands.
    pub(crate) fn heights(&self, other: &Facet) -> [f64; 3] {
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
    pub(crate) fn inside(&self, segment: [DVec3; 2], room: f64) -> Option<[DVec3; 2]> {
        let along = |point: &DVec3| point.dot(segment[1] - segment[0]);
        let mut kept = self.clip(&segment, room);
        kept.sort_by(|one, other| along(one).total_cmp(&along(other)));
        let (from, to) = (*kept.first()?, *kept.last()?);
        (from.distance(to) > room).then_some([from, to])
    }

    /// Whether `other`, lying in this face's plane, covers it thicker than `room`
    /// — twice the area shared over its outline, as a strip's thickness is.
    fn overlaps(&self, other: &Facet, room: f64) -> bool {
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

/// Two faces, by their index among the triangles.
pub(crate) type Pair = (usize, usize);

/// A face by its index among the triangles.
type Indexed<'a> = (usize, &'a Facet);

/// Whether the first of `triangles` and another pass through each other along
/// a line of edges, as [`Lines::passed`] reads them all, weighing only the few
/// a line through it can gather: those an edge of the first holds along it,
/// those holding the first along an edge of theirs, and those such an edge
/// holds — every stretch, round and face the line lies inside that the first
/// is read with. Their order is kept, and with it every tie a round breaks.
pub(crate) fn passes_along(triangles: &[Triangle], room: f64) -> bool {
    let facets: Vec<Facet> = triangles.iter().map(Facet::of).collect();
    let holds = |by: usize, on: usize| holds(&facets[by], &facets[on], room);
    let holding: Vec<usize> = (1..triangles.len()).filter(|by| holds(*by, 0)).collect();
    let kept: Vec<Triangle> = (0..triangles.len())
        .filter(|at| {
            *at == 0 || holds(0, *at) || holding.iter().any(|by| by == at || holds(*by, *at))
        })
        .map(|at| triangles[at])
        .collect();
    let pairs = passed(&kept, room);
    pairs.into_iter().any(|(one, other)| one == 0 || other == 0)
}

/// The faces of `triangles` the surface passes through itself between along a
/// line of edges, every two weighed by the same [`Lines`] the rules' check
/// weighs faces near each other with.
fn passed(triangles: &[Triangle], room: f64) -> Vec<Pair> {
    let facets: Vec<Facet> = triangles.iter().map(Facet::of).collect();
    let kept: Vec<usize> = (0..triangles.len())
        .filter(|at| !sliver(&triangles[*at], room))
        .collect();
    let mut lines = Lines::default();
    for (at, &one) in kept.iter().enumerate() {
        for &other in &kept[at + 1..] {
            let (one, other) = ((one, &facets[one]), (other, &facets[other]));
            let over = [one.1.heights(other.1), other.1.heights(one.1)];
            if !clear(over[0], room) && !clear(over[1], room) {
                lines.weigh(one, other, over, room);
            }
        }
    }
    lines.passed(triangles, room)
}

/// Whether an edge of `by` could note `on` along it, whatever else holds:
/// an edge of `on` running along it, or the edge resting inside `on`.
fn holds(by: &Facet, on: &Facet, room: f64) -> bool {
    runs(by, on, room).next().is_some() || leaning(on, by, on.heights(by), room).is_some()
}

/// Each edge of `this` an edge of `that` runs along, that one, and the
/// stretch of the first they share.
fn runs(this: &Facet, that: &Facet, room: f64) -> impl Iterator<Item = (usize, usize, [f64; 2])> {
    let [this, that] = [this.corners, that.corners];
    (0..9).filter_map(move |pair| {
        let (mine, theirs) = (pair / 3, pair % 3);
        let edge = [this[mine], this[(mine + 1) % 3]];
        let span = stretch(edge, [that[theirs], that[(theirs + 1) % 3]], room)?;
        Some((mine, theirs, span))
    })
}

/// The edge of `by`, its corners `heights` above `on`, that rests on `on`
/// inside it while its third corner stands to one side, and the stretch of it
/// inside.
fn leaning(on: &Facet, by: &Facet, heights: [f64; 3], room: f64) -> Option<(usize, [f64; 2])> {
    let rests = |at: usize| heights[at].abs() <= room;
    let at = (0..3).find(|at| rests(*at) && rests((at + 1) % 3))?;
    let [above, below] = sides(heights, room);
    if above == below {
        return None;
    }
    let edge = [by.corners[at], by.corners[(at + 1) % 3]];
    let span = stretch(edge, on.inside(edge, room)?, room)?;
    Some((at, span))
}

/// Faces lying along another's edge, gathered two faces at a time and read a
/// line at a time by [`Self::passed`].
#[derive(Default)]
pub(crate) struct Lines(Vec<Alongside>);

impl Lines {
    /// Notes what two faces neither clear of the other show of a line, each's
    /// corners `over` the other's plane.
    pub(crate) fn weigh(&mut self, one: Indexed, other: Indexed, over: [[f64; 3]; 2], room: f64) {
        for (on, by, heights) in [(one, other, over[0]), (other, one, over[1])] {
            if let Some((at, span)) = leaning(on.1, by.1, heights, room) {
                self.note(3 * by.0 + at, on.0, span, None);
            }
        }
        if resting(over[0], room) > 1 && resting(over[1], room) > 1 {
            self.share_edges(one, other, room);
        }
    }

    /// Notes each stretch of edge the two faces run along together.
    pub(crate) fn share_edges(&mut self, one: Indexed, other: Indexed, room: f64) {
        for ((index, this), (face, that)) in [(one, other), (other, one)] {
            for (mine, theirs, span) in runs(this, that, room) {
                let edge = [this.corners[mine], this.corners[(mine + 1) % 3]];
                let [from, to] = [that.corners[theirs], that.corners[(theirs + 1) % 3]];
                let wing = that.corners[(theirs + 2) % 3];
                let ahead = (edge[1] - edge[0]).dot(to - from) > 0.0;
                self.note(3 * index + mine, face, span, Some((wing, ahead)));
            }
        }
    }

    fn note(&mut self, anchor: usize, face: usize, span: [f64; 2], ends: Option<(DVec3, bool)>) {
        self.0.push(Alongside {
            anchor,
            face,
            span,
            ends,
        });
    }

    /// The faces a surface passes through itself between along a line of
    /// edges, which no two faces show alone: stretch by stretch along each edge.
    ///
    /// A face the line lies inside, with faces ending on it from both sides,
    /// is passed through; from one side only, they are a ridge resting on it.
    /// Faces all ending on the line: a skin wraps matter, so going round, each
    /// runs along it the other way from the next. Two skins crossing, cut along
    /// the line and every piece kept, break that. Faces not running one way as
    /// often as the other leave the skin open, and that is for the rule that
    /// the surface is closed.
    pub(crate) fn passed(&mut self, triangles: &[Triangle], room: f64) -> Vec<Pair> {
        self.0.sort_by_key(|each| (each.anchor, each.face));
        self.0.dedup_by_key(|each| (each.anchor, each.face));
        let mut passed = Vec::new();
        for around in self.0.chunk_by(|one, other| one.anchor == other.anchor) {
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
