//! Two faces passing through each other along a line: no two of their
//! triangles cross, each only touches the other there, yet one face goes on
//! to either side of the other — what the rules read across several
//! triangles, a line of edges at a time.

use glam::DVec3;

use super::{At, Folds};
use crate::brep::crossing::Facet;

impl Folds<'_> {
    /// Whether the triangle at `at` and another face pass through each other
    /// along a line: a side of it both faces go on across, each to both sides
    /// of the plane of one of the other's triangles along it — either: two
    /// faces crossing at less than they bend there straddle one plane only —
    /// or a side of either lying inside a triangle of the other, its face
    /// going on to both sides of that triangle's plane there.
    pub(super) fn crossed_along(&self, at: At) -> bool {
        let Some(corners) = self.corners(at) else {
            return false;
        };
        let mine = self.facet(corners);
        let shared = sides(corners).into_iter().any(|edge| {
            let ours: Vec<Facet> = self
                .along(edge)
                .iter()
                .filter(|each| each.0 == at.0)
                .filter_map(|each| self.corners(*each))
                .map(|each| self.facet(each))
                .collect();
            self.along(edge).iter().any(|beside| {
                beside.0 != at.0
                    && self.corners(*beside).is_some_and(|theirs| {
                        self.severs(at.0, edge, &self.facet(theirs))
                            && ours.iter().any(|plane| self.severs(beside.0, edge, plane))
                    })
            })
        });
        shared
            || self.near(at.0, corners).any(|(other, theirs)| {
                let facet = self.facet(theirs);
                sides(theirs)
                    .into_iter()
                    .any(|edge| self.lies_inside(edge, &mine) && self.severs(other.0, edge, &mine))
                    || sides(corners).into_iter().any(|edge| {
                        self.lies_inside(edge, &facet) && self.severs(at.0, edge, &facet)
                    })
            })
    }

    fn facet(&self, corners: [usize; 3]) -> Facet {
        Facet::of(&corners.map(|id| self.samples.point(id)))
    }

    fn along(&self, [one, other]: [usize; 2]) -> &[At] {
        self.edges
            .get(&[one.min(other), one.max(other)])
            .map_or(&[], Vec::as_slice)
    }

    /// Whether `edge` rests on the plane of `on`, inside it.
    fn lies_inside(&self, edge: [usize; 2], on: &Facet) -> bool {
        let points = edge.map(|id| self.samples.point(id));
        points
            .iter()
            .all(|point| height(on, *point).abs() <= self.room)
            && on.inside(points, self.room).is_some()
    }

    /// Whether `face` goes on across `edge` to both sides of the plane of
    /// `other`: its two triangles along the edge, one each side.
    fn severs(&self, face: usize, edge: [usize; 2], other: &Facet) -> bool {
        let wings: Vec<f64> = self
            .along(edge)
            .iter()
            .filter(|each| each.0 == face)
            .filter_map(|each| self.corners(*each))
            .filter_map(|corners| corners.into_iter().find(|id| !edge.contains(id)))
            .map(|wing| height(other, self.samples.point(wing)))
            .collect();
        let room = self.room;
        matches!(wings[..], [one, two] if one * two < 0.0 && one.abs() > room && two.abs() > room)
    }
}

fn sides([a, b, c]: [usize; 3]) -> [[usize; 2]; 3] {
    [[a, b], [b, c], [c, a]]
}

fn height(on: &Facet, point: DVec3) -> f64 {
    on.normal.dot(point - on.corners[0])
}
