//! Where two edges touch between their vertices: a circle whose wall a plane
//! touches, its rim meeting the line the plane cuts on another wall at one
//! point, or two curves grazing each other at a place of both grids. The
//! kernel puts no vertex there, the two edges crossing nowhere, but their
//! samples stand within rounding of each other — or of the line — and the
//! faces holding both cross themselves there unless that place is one
//! sample of both.

use std::collections::HashMap;

use glam::DVec3;

/// The samples placed so far, found by where they stand.
pub(super) struct Places {
    near: f64,
    cells: HashMap<[i64; 3], Vec<(usize, usize)>>,
}

impl Places {
    /// Samples standing within `near` of each other are one place.
    pub(super) fn new(near: f64) -> Places {
        Places {
            near,
            cells: HashMap::new(),
        }
    }

    fn cell(&self, point: DVec3) -> [i64; 3] {
        (point / self.near).floor().as_i64vec3().to_array()
    }

    /// The sample of another edge standing within rounding of `point`.
    pub(super) fn found(&self, points: &[DVec3], point: DVec3, edge: usize) -> Option<usize> {
        let [x, y, z] = self.cell(point);
        (-1..=1)
            .flat_map(|dx| (-1..=1).flat_map(move |dy| (-1..=1).map(move |dz| [dx, dy, dz])))
            .filter_map(|[dx, dy, dz]| self.cells.get(&[x + dx, y + dy, z + dz]))
            .flatten()
            .filter(|(_, owner)| *owner != edge)
            .map(|(id, _)| *id)
            .find(|id| (points[*id] - point).length() <= self.near)
    }

    pub(super) fn place(&mut self, point: DVec3, id: usize, edge: usize) {
        let cell = self.cell(point);
        self.cells.entry(cell).or_default().push((id, edge));
    }
}

/// The samples standing on the line from `start` to `end` within `near`,
/// further than `eps` from either end, in their order along it.
pub(super) fn on_the_line(
    points: &[DVec3],
    [start, end]: [usize; 2],
    near: f64,
    eps: f64,
) -> Vec<usize> {
    let (from, to) = (points[start], points[end]);
    let length = (to - from).length();
    if length <= 2.0 * eps {
        return Vec::new();
    }
    let along = (to - from) / length;
    let mut on: Vec<(f64, usize)> = points
        .iter()
        .enumerate()
        .filter(|(id, _)| *id != start && *id != end)
        .filter_map(|(id, point)| {
            let t = (*point - from).dot(along);
            let off = (*point - from - along * t).length();
            (t > eps && t < length - eps && off <= near).then_some((t, id))
        })
        .collect();
    on.sort_by(|one, other| one.0.total_cmp(&other.0));
    on.into_iter().map(|(_, id)| id).collect()
}
