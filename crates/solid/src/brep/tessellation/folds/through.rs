//! Whether two triangles of different faces pass through each other.

use glam::DVec3;

/// Whether two triangles pass through each other: each stands across the
/// other's plane, and the stretches they cut each other's plane along
/// overlap by more than `clear`, as two sharing a corner do not where
/// they leave it on either side of the line their planes meet along.
pub(super) fn through(one: [DVec3; 3], other: [DVec3; 3], clear: f64) -> bool {
    let normal = |[a, b, c]: [DVec3; 3]| (b - a).cross(c - a).try_normalize();
    let (Some(mine), Some(theirs)) = (normal(one), normal(other)) else {
        return false;
    };
    let Some(along) = mine.cross(theirs).try_normalize() else {
        return false;
    };
    let stretch = |corners: [DVec3; 3], normal: DVec3, base: DVec3| {
        let heights = corners.map(|corner| normal.dot(corner - base));
        let above = heights.iter().any(|height| *height > clear);
        let below = heights.iter().any(|height| *height < -clear);
        if !(above && below) {
            return None;
        }
        let mut reach: Vec<f64> = Vec::with_capacity(3);
        for here in 0..3 {
            let next = (here + 1) % 3;
            let (height, onward) = (heights[here], heights[next]);
            if height.abs() <= clear {
                reach.push(along.dot(corners[here]));
            } else if onward.abs() > clear && (height > 0.0) != (onward > 0.0) {
                let point = corners[here].lerp(corners[next], height / (height - onward));
                reach.push(along.dot(point));
            }
        }
        let low = reach.iter().copied().fold(f64::INFINITY, f64::min);
        let high = reach.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Some((low, high))
    };
    let (Some(first), Some(second)) =
        (stretch(other, mine, one[0]), stretch(one, theirs, other[0]))
    else {
        return false;
    };
    first.1.min(second.1) - first.0.max(second.0) > clear
}
