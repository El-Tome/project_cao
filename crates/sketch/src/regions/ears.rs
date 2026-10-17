//! A closed outline cut into triangles by clipping its ears.

use glam::DVec2;

use super::side::{Side, side};

/// Every ear of the outline: repeatedly take a corner no other corner sits
/// in, and snip it off — or what was cut before the clipper wedged itself,
/// which the caller holds against the outline's measure.
pub(super) fn clipped_ears(outline: &[DVec2]) -> Vec<[DVec2; 3]> {
    let mut remaining: Vec<DVec2> = outline.to_vec();
    let mut triangles = Vec::new();
    let mut guard = remaining.len() * remaining.len();
    while remaining.len() > 3 && guard > 0 {
        guard -= 1;
        let count = remaining.len();
        let mut clipped = false;
        for index in 0..count {
            let (a, b, c) = (
                remaining[(index + count - 1) % count],
                remaining[index],
                remaining[(index + 1) % count],
            );
            if side(c, a, b) != Side::Left {
                continue;
            }
            let clear = remaining
                .iter()
                .enumerate()
                .filter(|(other, _)| {
                    *other != index
                        && *other != (index + count - 1) % count
                        && *other != (index + 1) % count
                })
                // A corner of the ear itself, met a second time, is the seam of
                // a corridor cut out to a hole: it sits on the ear by
                // construction and must not be read as blocking it.
                .filter(|(_, point)| {
                    point.distance_squared(a) > EPSILON
                        && point.distance_squared(b) > EPSILON
                        && point.distance_squared(c) > EPSILON
                })
                .all(|(_, point)| !in_triangle(*point, a, b, c));
            if clear {
                triangles.push([a, b, c]);
                remaining.remove(index);
                clipped = true;
                break;
            }
        }
        if !clipped {
            break;
        }
    }
    if remaining.len() == 3 {
        triangles.push([remaining[0], remaining[1], remaining[2]]);
    }
    triangles
}

/// How close two positions have to be to count as the same corner.
const EPSILON: f64 = 1e-12;

fn in_triangle(point: DVec2, a: DVec2, b: DVec2, c: DVec2) -> bool {
    [(a, b), (b, c), (c, a)]
        .into_iter()
        .all(|(from, to)| side(point, from, to) != Side::Right)
}
