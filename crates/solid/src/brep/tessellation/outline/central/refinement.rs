//! The triangles of a piece of sphere swept on its central chart, refined
//! until each stands within the tolerance of the sphere (#800).
//!
//! A triangle whose corners lie on the sphere stands furthest from it at the
//! point of the triangle nearest the centre, measured exactly here. While a
//! triangle stands further than the tolerance, its longest side inside the
//! face is cut in two at its middle on the chart, lifted onto the sphere;
//! never a side on the boundary, whose samples the faces beyond share. After
//! every cut, a side is cut the other way wherever the point across it
//! stands beyond the plane of the triangle on this side: on the sphere, the
//! cut where no triangle's circle holds another point, which stands nearest
//! the sphere everywhere. A triangle on the boundary is then held by the
//! chord of its side there, which the sampling holds within the tolerance,
//! once the cuts have brought its corner inside the circle on that side as
//! its diameter.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use glam::{DVec2, DVec3};

use super::Chart;
use crate::orientation::turn;

/// How far beyond the plane of a triangle, as a share of the product of the
/// lengths from one of its corners, the point across a side must stand for
/// the side to be cut the other way: far over rounding, so that two cuts a
/// hair from a tie do not take each other back.
const TIE: f64 = 1e-9;

/// How many points a piece of sphere may be refined into, for each area of
/// its radius by the tolerance: far over the fifth of a point even triangles
/// standing the tolerance off the sphere take. A refinement that does not
/// end within it gives the face up.
const BUDGET: f64 = 16.0;

/// The triangles of the region `swept` cuts on the chart of `chart` out of
/// `points`, which stand for `places` seen from the centre, `segments`
/// bounding it, refined within `tolerance`: the points added on the chart,
/// numbered on past the outline's, and the triangles. None when the cut
/// does not end, or leaves a triangle that does not turn counterclockwise.
pub(super) fn refined(
    chart: &Chart,
    (points, places): (&[DVec2], &[DVec3]),
    segments: &[[usize; 2]],
    swept: Vec<[usize; 3]>,
    tolerance: f64,
) -> Option<(Vec<DVec2>, Vec<[usize; 3]>)> {
    let mut mesh = Mesh {
        chart,
        points: points.to_vec(),
        places: places.to_vec(),
        triangles: Vec::new(),
        sides: BTreeMap::new(),
        bounds: segments.iter().map(|&segment| sorted(segment)).collect(),
    };
    for corners in swept {
        mesh.add(corners);
    }
    if mesh.sides.len() != 3 * mesh.triangles.len() {
        return None;
    }
    let every: Vec<[usize; 2]> = mesh.sides.keys().copied().collect();
    mesh.legalize(every);
    let most = mesh.most(tolerance);
    loop {
        let sagging: Vec<usize> = (0..mesh.triangles.len())
            .filter(|&at| mesh.sags(at, tolerance))
            .collect();
        if sagging.is_empty() {
            break;
        }
        let mut cut = false;
        for at in sagging {
            if mesh.sags(at, tolerance) {
                cut |= mesh.refine(at);
            }
        }
        if !cut || mesh.points.len() > most {
            return None;
        }
    }
    if !mesh.triangles.iter().all(|&corners| mesh.turns(corners)) {
        return None;
    }
    Some((mesh.points[points.len()..].to_vec(), mesh.triangles))
}

struct Mesh<'a> {
    chart: &'a Chart,
    points: Vec<DVec2>,
    places: Vec<DVec3>,
    triangles: Vec<[usize; 3]>,
    /// Each side of a triangle, the way it runs round it, and the triangle.
    sides: BTreeMap<[usize; 2], usize>,
    /// The segments of the outline, lowest point first.
    bounds: BTreeSet<[usize; 2]>,
}

impl Mesh<'_> {
    /// Whether the triangle at `at` stands further from the sphere than
    /// `tolerance`, and than the chord of any of its sides on the boundary.
    fn sags(&self, at: usize, tolerance: f64) -> bool {
        let corners = self.triangles[at];
        let allowed = (0..3)
            .map(|side| [corners[side], corners[(side + 1) % 3]])
            .filter(|&side| self.bounds.contains(&sorted(side)))
            .map(|[one, other]| self.chart.radius - self.reach(one, other))
            .fold(tolerance, f64::max);
        self.chart.radius - self.nearest(corners) > allowed
    }

    /// How near the centre the triangle on `corners` comes.
    fn nearest(&self, corners: [usize; 3]) -> f64 {
        let [a, b, c] = corners.map(|id| self.places[id]);
        let normal = (b - a).cross(c - a);
        let square = normal.length_squared();
        if square > 0.0 {
            let foot = normal * (normal.dot(a) / square);
            let inside = [(a, b), (b, c), (c, a)]
                .iter()
                .all(|&(from, to)| (to - from).cross(foot - from).dot(normal) >= 0.0);
            if inside {
                return normal.dot(a).abs() / square.sqrt();
            }
        }
        (0..3)
            .map(|side| self.reach(corners[side], corners[(side + 1) % 3]))
            .fold(f64::INFINITY, f64::min)
    }

    /// How near the centre the chord between two points comes, the same
    /// whichever way it is asked.
    fn reach(&self, one: usize, other: usize) -> f64 {
        let [from, to] = [one.min(other), one.max(other)].map(|id| self.places[id]);
        let along = to - from;
        let square = along.length_squared();
        let share = if square > 0.0 {
            (-from.dot(along) / square).clamp(0.0, 1.0)
        } else {
            0.0
        };
        (from + along * share).length()
    }

    /// The most points the cut may reach: [`BUDGET`] for each area of the
    /// radius by `tolerance` the triangles swept cover, past the outline's,
    /// and a few more for a piece too small to need any.
    fn most(&self, tolerance: f64) -> usize {
        let area: f64 = self
            .triangles
            .iter()
            .map(|corners| {
                let [a, b, c] = corners.map(|id| self.places[id]);
                (b - a).cross(c - a).length() / 2.0
            })
            .sum();
        let room = (BUDGET * area / (self.chart.radius * tolerance)).ceil();
        self.points.len() + 64 + room.min(u32::MAX as f64) as usize
    }

    /// Cuts the triangle at `at` closer to the sphere: its longest side
    /// inside the face in two, or, where all three are on the boundary, the
    /// triangle in three about its middle. Whether it was cut.
    fn refine(&mut self, at: usize) -> bool {
        let corners = self.triangles[at];
        let length = |[one, other]: [usize; 2]| self.places[one].distance(self.places[other]);
        let inner = (0..3)
            .map(|side| [corners[side], corners[(side + 1) % 3]])
            .filter(|&side| !self.bounds.contains(&sorted(side)))
            .max_by(|one, other| length(*one).total_cmp(&length(*other)));
        match inner {
            Some(side) => self.split(side),
            None => self.split_inside(at),
        }
    }

    /// Cuts the side from `u` to `v` at its middle on the chart, and the two
    /// triangles along it in two each.
    fn split(&mut self, [u, v]: [usize; 2]) -> bool {
        let (Some(&one), Some(&other)) = (self.sides.get(&[u, v]), self.sides.get(&[v, u])) else {
            return false;
        };
        let w = third(self.triangles[one], [u, v]);
        let z = third(self.triangles[other], [u, v]);
        let middle = self.place((self.points[u] + self.points[v]) / 2.0);
        let made = [
            [u, middle, w],
            [middle, v, w],
            [v, middle, z],
            [middle, u, z],
        ];
        if !made.iter().all(|&corners| self.turns(corners)) {
            self.unplace();
            return false;
        }
        self.replace(one, made[0]);
        self.replace(other, made[2]);
        self.add(made[1]);
        self.add(made[3]);
        self.legalize(vec![[w, u], [v, w], [z, v], [u, z]]);
        true
    }

    /// Cuts the triangle at `at` in three about its middle on the chart.
    fn split_inside(&mut self, at: usize) -> bool {
        let [a, b, c] = self.triangles[at];
        let middle = self.place((self.points[a] + self.points[b] + self.points[c]) / 3.0);
        let made = [[a, b, middle], [b, c, middle], [c, a, middle]];
        if !made.iter().all(|&corners| self.turns(corners)) {
            self.unplace();
            return false;
        }
        self.replace(at, made[0]);
        self.add(made[1]);
        self.add(made[2]);
        self.legalize(vec![[a, b], [b, c], [c, a]]);
        true
    }

    /// Cuts the other way every side of `stack`, and every side around one
    /// cut, where the point across it stands beyond the plane of the
    /// triangle on this side by more than a [`TIE`]: each such cut makes the
    /// triangles hold more volume about the centre, so the cuts end.
    fn legalize(&mut self, mut stack: Vec<[usize; 2]>) {
        while let Some([a, b]) = stack.pop() {
            if self.bounds.contains(&sorted([a, b])) {
                continue;
            }
            let (Some(&one), Some(&other)) = (self.sides.get(&[a, b]), self.sides.get(&[b, a]))
            else {
                continue;
            };
            let x = third(self.triangles[one], [a, b]);
            let y = third(self.triangles[other], [a, b]);
            if !self.beyond([a, b, x], y) {
                continue;
            }
            let made = [[a, y, x], [y, b, x]];
            if !made.iter().all(|&corners| self.turns(corners)) {
                continue;
            }
            self.replace(one, made[0]);
            self.replace(other, made[1]);
            stack.extend([[a, y], [y, b], [b, x], [x, a]]);
        }
    }

    /// Whether `point` stands beyond the plane of the triangle on `corners`,
    /// away from the centre: within its circle on the sphere.
    fn beyond(&self, corners: [usize; 3], point: usize) -> bool {
        let [a, b, x, y] = [corners[0], corners[1], corners[2], point].map(|id| self.places[id]);
        let (along, across, to) = (b - a, x - a, y - a);
        along.cross(across).dot(to) > TIE * along.length() * across.length() * to.length()
    }

    fn turns(&self, [a, b, c]: [usize; 3]) -> bool {
        turn(self.points[a], self.points[b], self.points[c]) == Ordering::Greater
    }

    /// A point of the chart, with the point of the sphere it stands for.
    fn place(&mut self, at: DVec2) -> usize {
        self.points.push(at);
        self.places.push(self.chart.lifted(at));
        self.points.len() - 1
    }

    fn unplace(&mut self) {
        self.points.pop();
        self.places.pop();
    }

    fn add(&mut self, corners: [usize; 3]) {
        let at = self.triangles.len();
        self.triangles.push(corners);
        for side in sides(corners) {
            self.sides.insert(side, at);
        }
    }

    fn replace(&mut self, at: usize, corners: [usize; 3]) {
        for side in sides(self.triangles[at]) {
            if self.sides.get(&side) == Some(&at) {
                self.sides.remove(&side);
            }
        }
        self.triangles[at] = corners;
        for side in sides(corners) {
            self.sides.insert(side, at);
        }
    }
}

fn sides([a, b, c]: [usize; 3]) -> [[usize; 2]; 3] {
    [[a, b], [b, c], [c, a]]
}

fn sorted([one, other]: [usize; 2]) -> [usize; 2] {
    [one.min(other), one.max(other)]
}

/// The corner of a triangle that is neither end of a side.
fn third(corners: [usize; 3], [one, other]: [usize; 2]) -> usize {
    corners
        .into_iter()
        .find(|&corner| corner != one && corner != other)
        .unwrap_or(corners[0])
}
