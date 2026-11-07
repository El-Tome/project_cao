//! Whether two triangles pass through each other or lie on each other: the one
//! definition the tessellation's folds draw by, and the rules' check that no
//! face of a solid crosses another weighs every two faces with.

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
