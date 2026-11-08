//! Several triangles around a line of edges: what no two of them show alone,
//! a surface passing through itself along the line.

use glam::DVec3;

use super::{Facet, Triangle, clear, cross, sides, sliver};

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
    let held = |by: usize, on: usize| holds(&facets[by], &facets[on], room);
    let holding: Vec<usize> = (1..triangles.len()).filter(|by| held(*by, 0)).collect();
    let kept: Vec<Triangle> = (0..triangles.len())
        .filter(|at| {
            *at == 0 || held(0, *at) || holding.iter().any(|by| by == at || held(*by, *at))
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
        self.share_edges(one, other, room);
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
