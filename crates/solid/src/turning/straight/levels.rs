//! The levels a profile's corners are laid at: every run parallel to the
//! axis brings its two corners to one distance from it, every run square to
//! it to one place along it, and levels a hair apart anywhere in the profile
//! are one.

/// Each corner, read as `(along, away)`, laid at its level, or `None` when a
/// run leans past `tolerance` or a corner would move further than twice it.
/// `runs` are the corners each run goes between.
pub(super) fn laid(
    read: &[glam::DVec2],
    runs: &[(usize, usize)],
    tolerance: f64,
) -> Option<Vec<glam::DVec2>> {
    let mut along = Classes::of(read.len());
    let mut away = Classes::of(read.len());
    for &(from, to) in runs {
        let step = (read[to] - read[from]).abs();
        let (square, parallel) = (step.x <= tolerance, step.y <= tolerance);
        if !square && !parallel {
            return None;
        }
        if square {
            along.join(from, to);
        }
        if parallel {
            away.join(from, to);
        }
    }
    let heights: Vec<f64> = read.iter().map(|corner| corner.x).collect();
    let radii: Vec<f64> = read.iter().map(|corner| corner.y).collect();
    let laid: Vec<glam::DVec2> = along
        .levels(&heights, tolerance, false)
        .into_iter()
        .zip(away.levels(&radii, tolerance, true))
        .map(|(height, radius)| glam::DVec2::new(height, radius))
        .collect();
    let moved = laid
        .iter()
        .zip(read)
        .any(|(laid, read)| (*laid - *read).abs().max_element() > 2.0 * tolerance);
    (!moved).then_some(laid)
}

/// Corners joined into classes by the runs between them.
struct Classes {
    parent: Vec<usize>,
}

impl Classes {
    fn of(count: usize) -> Classes {
        Classes {
            parent: (0..count).collect(),
        }
    }

    fn root(&mut self, corner: usize) -> usize {
        let mut root = corner;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        self.parent[corner] = root;
        root
    }

    fn join(&mut self, first: usize, second: usize) {
        let (first, second) = (self.root(first), self.root(second));
        self.parent[first.max(second)] = first.min(second);
    }

    /// One value per corner: its class's, and that of every class within
    /// `tolerance` of the next one up, all of them taking the mean of their
    /// corners. With `anchored`, a cluster holding a value of nought, a
    /// corner on the axis, stays at nought.
    fn levels(&mut self, values: &[f64], tolerance: f64, anchored: bool) -> Vec<f64> {
        let mut members: Vec<Vec<usize>> = vec![Vec::new(); values.len()];
        for corner in 0..values.len() {
            let root = self.root(corner);
            members[root].push(corner);
        }
        let mut classes: Vec<(f64, Vec<usize>)> = members
            .into_iter()
            .filter(|corners| !corners.is_empty())
            .map(|corners| (level(values, &corners, anchored), corners))
            .collect();
        classes.sort_by(|(first, of_first), (second, of_second)| {
            first.total_cmp(second).then(of_first[0].cmp(&of_second[0]))
        });
        let mut laid = vec![0.0; values.len()];
        let mut start = 0;
        while start < classes.len() {
            let mut end = start + 1;
            while end < classes.len() && classes[end].0 - classes[end - 1].0 <= tolerance {
                end += 1;
            }
            let mut corners: Vec<usize> = classes[start..end]
                .iter()
                .flat_map(|(_, corners)| corners.iter().copied())
                .collect();
            corners.sort_unstable();
            let value = level(values, &corners, anchored);
            for corner in corners {
                laid[corner] = value;
            }
            start = end;
        }
        laid
    }
}

/// The mean of the corners' values, taken in corner order from the first, so
/// that corners already level keep their value to the bit; nought when
/// `anchored` and one of them is.
fn level(values: &[f64], corners: &[usize], anchored: bool) -> f64 {
    if anchored && corners.iter().any(|&corner| values[corner] == 0.0) {
        return 0.0;
    }
    let first = values[corners[0]];
    let spread: f64 = corners.iter().map(|&corner| values[corner] - first).sum();
    first + spread / corners.len() as f64
}
