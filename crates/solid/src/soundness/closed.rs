//! A surface with no hole in it.

use std::collections::HashMap;

use glam::{DVec3, I64Vec3};

use super::{Flaw, NEAR, Triangle, reach};

/// Whether a surface is closed: along every stretch of every edge, as many
/// faces run one way as the other — one each on an ordinary edge, two each
/// where two solids touch along an edge and nothing more.
///
/// A stretch rather than a whole edge, because a kernel that cuts faces leaves
/// T-junctions: the corner of one face lying partway along the edge of its
/// neighbour. Both are still one closed surface, and an edge is split at every
/// corner lying on it before the faces along it are counted.
///
/// The same split is what lets a face with three corners in a line through:
/// the triangle with no area it fans into runs along its own long side both
/// ways once that side is cut at the middle corner.
pub fn closed(triangles: &[Triangle]) -> Result<(), Flaw> {
    let mut places = Places::of(triangles);
    let mut edges = Vec::with_capacity(triangles.len() * 3);
    for triangle in triangles {
        let ids = triangle.map(|corner| places.weld(corner));
        for side in 0..3 {
            let (from, to) = (ids[side], ids[(side + 1) % 3]);
            if from != to {
                edges.push(Stretch::along(from, to, 1, 0));
            }
        }
    }

    let boxes = Boxes::of(&places.at, places.near);
    let mut stretches = Vec::with_capacity(edges.len());
    for edge in gathered(edges) {
        let (low, high) = edge.ends;
        for step in boxes.path(low, high).windows(2) {
            stretches.push(Stretch::along(step[0], step[1], edge.forth, edge.back));
        }
    }

    match gathered(stretches)
        .into_iter()
        .find(|stretch| stretch.forth != stretch.back)
    {
        None => Ok(()),
        Some(Stretch {
            ends: (low, high),
            forth,
            back,
        }) => {
            let (from, to) = if forth > back {
                (low, high)
            } else {
                (high, low)
            };
            Err(Flaw::Open {
                from: places.at[from],
                to: places.at[to],
                one_way: forth.max(back),
                other_way: forth.min(back),
            })
        }
    }
}

/// How many faces run along a stretch of edge each way, keyed by its two ends
/// with the lower number first.
struct Stretch {
    ends: (usize, usize),
    forth: usize,
    back: usize,
}

impl Stretch {
    /// A stretch run `forth` times from `from` to `to`, and `back` times the
    /// other way.
    fn along(from: usize, to: usize, forth: usize, back: usize) -> Self {
        if from < to {
            Self {
                ends: (from, to),
                forth,
                back,
            }
        } else {
            Self {
                ends: (to, from),
                forth: back,
                back: forth,
            }
        }
    }
}

/// Every stretch once, in the order of its ends, with the faces along it
/// summed. Sorted rather than hashed, so that which hole is reported first
/// depends on the triangles alone.
fn gathered(mut stretches: Vec<Stretch>) -> Vec<Stretch> {
    stretches.sort_unstable_by_key(|stretch| stretch.ends);
    let mut gathered: Vec<Stretch> = Vec::with_capacity(stretches.len());
    for stretch in stretches {
        match gathered.last_mut() {
            Some(last) if last.ends == stretch.ends => {
                last.forth += stretch.forth;
                last.back += stretch.back;
            }
            _ => gathered.push(stretch),
        }
    }
    gathered
}

/// The distinct places the corners of a surface stand at, each found again
/// from any corner closer to it than `near`.
///
/// Filed in cubes a few times `near` across, whatever the size of the
/// surface: a cube that small fills up only with places that are nearly one,
/// and a surface has few of those.
struct Places {
    at: Vec<DVec3>,
    near: f64,
    cells: HashMap<I64Vec3, Vec<usize>>,
}

/// The side of a cube, in `near`. Wider than twice `near`, so that a new place
/// is filed in a few cubes rather than nearly always eight.
const CUBE: f64 = 4.0;

impl Places {
    /// Room for the corners of `triangles`.
    ///
    /// A corner that is not a number is left out of the scale and never
    /// welded: it stands nowhere, and its edges are left without a partner.
    fn of(triangles: &[Triangle]) -> Self {
        let finite: Vec<Triangle> = triangles
            .iter()
            .filter(|triangle| triangle.iter().all(|corner| corner.is_finite()))
            .copied()
            .collect();
        Self {
            at: Vec::new(),
            near: NEAR * reach(&finite),
            cells: HashMap::new(),
        }
    }

    /// The number of the place `corner` stands at, made anew when no place
    /// is near enough to it.
    ///
    /// A new place is filed in every cube within `near` of it, so the one cube
    /// a later corner falls in is enough to find it.
    fn weld(&mut self, corner: DVec3) -> usize {
        if corner.is_finite()
            && let Some(known) = self.cells.get(&self.cube(corner)).and_then(|ids| {
                ids.iter()
                    .find(|id| self.at[**id].distance(corner) < self.near)
            })
        {
            return *known;
        }
        let id = self.at.len();
        self.at.push(corner);
        if corner.is_finite() {
            let (low, high) = (self.cube(corner - self.near), self.cube(corner + self.near));
            for x in low.x..=high.x {
                for y in low.y..=high.y {
                    for z in low.z..=high.z {
                        self.cells
                            .entry(I64Vec3::new(x, y, z))
                            .or_default()
                            .push(id);
                    }
                }
            }
        }
        id
    }

    fn cube(&self, point: DVec3) -> I64Vec3 {
        (point / (CUBE * self.near)).floor().as_i64vec3()
    }
}

/// The places of a surface sorted into boxes inside boxes, each box cut in
/// two at its middle place along its longest side until a handful are left.
///
/// Cut where the places are rather than on a grid of equal cubes: a grid sized
/// on the whole surface puts every corner of a fine part in one cube as soon
/// as something small stands far off, and every edge of the part then reads
/// them all. Looking along an edge opens only the boxes it passes, and a long
/// edge across a face with corners only round its rim opens few.
struct Boxes<'a> {
    at: &'a [DVec3],
    near: f64,
    order: Vec<usize>,
    boxes: Vec<Bounds>,
}

/// A box, the run of `order` held inside it, and where its second half is
/// when it is cut: its first half comes right after it.
struct Bounds {
    low: DVec3,
    high: DVec3,
    first: usize,
    end: usize,
    second: Option<usize>,
}

const HANDFUL: usize = 8;

impl<'a> Boxes<'a> {
    /// Every place that stands somewhere, sorted into boxes.
    fn of(at: &'a [DVec3], near: f64) -> Self {
        let order: Vec<usize> = (0..at.len()).filter(|id| at[*id].is_finite()).collect();
        let mut boxes = Self {
            at,
            near,
            boxes: Vec::new(),
            order,
        };
        if !boxes.order.is_empty() {
            boxes.cut(0, boxes.order.len());
        }
        boxes
    }

    fn cut(&mut self, first: usize, end: usize) {
        let at = self.at;
        let held = &mut self.order[first..end];
        let (low, high) = held
            .iter()
            .fold((DVec3::INFINITY, DVec3::NEG_INFINITY), |(low, high), id| {
                (low.min(at[*id]), high.max(at[*id]))
            });
        let index = self.boxes.len();
        self.boxes.push(Bounds {
            low,
            high,
            first,
            end,
            second: None,
        });
        if held.len() > HANDFUL {
            let axis = (high - low).max_position();
            let middle = held.len() / 2;
            held.select_nth_unstable_by(middle, |left, right| {
                at[*left][axis].total_cmp(&at[*right][axis])
            });
            self.cut(first, first + middle);
            self.boxes[index].second = Some(self.boxes.len());
            self.cut(first + middle, end);
        }
    }

    /// The places from `from` to `to` along the edge between them, both ends
    /// included, every place lying on the edge in between taken in order.
    ///
    /// A place lies on the edge when it is nearer the edge than `near`, and no
    /// nearer than that to either end: nearer, it would have been welded to
    /// that end. What is measured is the distance to the end itself, not how
    /// far along the edge the place stands: one just beside the edge, close to
    /// an end but not at it, is still on the edge.
    ///
    /// An edge whose length is not a number — an end that stands nowhere, or
    /// ends so far out that the length overflows — has no direction to look
    /// along, and is searched for nothing.
    fn path(&self, from: usize, to: usize) -> Vec<usize> {
        let (start, end) = (self.at[from], self.at[to]);
        let length = start.distance(end);
        let mut path = vec![from];
        if length.is_finite() {
            let direction = (end - start) / length;
            let mut on: Vec<(f64, usize)> = self
                .beside(start, end)
                .into_iter()
                .filter_map(|id| {
                    let place = self.at[id];
                    let run = (place - start).dot(direction);
                    let foot = start + direction * run.clamp(0.0, length);
                    (place.distance(foot) < self.near
                        && place.distance(start) >= self.near
                        && place.distance(end) >= self.near)
                        .then_some((run, id))
                })
                .collect();
            on.sort_unstable_by(|left, right| {
                left.0.total_cmp(&right.0).then(left.1.cmp(&right.1))
            });
            path.extend(on.into_iter().map(|(_, id)| id));
        }
        path.push(to);
        path
    }

    /// Every place held in a box the edge passes within `near` of: all those
    /// nearer the edge than `near`, and a few more.
    ///
    /// The boxes are widened by twice `near`, so that rounding in the test
    /// never loses a place standing on the side of a box.
    fn beside(&self, start: DVec3, end: DVec3) -> Vec<usize> {
        let margin = DVec3::splat(2.0 * self.near);
        let mut met = Vec::new();
        let mut open = vec![0];
        while let Some(index) = open.pop() {
            let Some(bounds) = self.boxes.get(index) else {
                continue;
            };
            if !passes(start, end, bounds.low - margin, bounds.high + margin) {
                continue;
            }
            match bounds.second {
                Some(second) => open.extend([second, index + 1]),
                None => met.extend(&self.order[bounds.first..bounds.end]),
            }
        }
        met
    }
}

/// Whether the segment from `start` to `end` passes through the box from `low`
/// to `high`: the stretches of it within the box along each axis overlap.
fn passes(start: DVec3, end: DVec3, low: DVec3, high: DVec3) -> bool {
    let way = end - start;
    let (mut enter, mut leave) = (0.0f64, 1.0f64);
    for axis in 0..3 {
        if way[axis] == 0.0 {
            if start[axis] < low[axis] || start[axis] > high[axis] {
                return false;
            }
        } else {
            let into = (low[axis] - start[axis]) / way[axis];
            let out = (high[axis] - start[axis]) / way[axis];
            enter = enter.max(into.min(out));
            leave = leave.min(into.max(out));
        }
    }
    enter <= leave
}

#[cfg(test)]
mod tests;
