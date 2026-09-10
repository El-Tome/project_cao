use glam::DVec2;

/// Whether a closed loop of points never crosses its own boundary.
///
/// The face walk that builds an outline only reads the segment graph's
/// vertices and their angular order around each one — it has no notion of
/// two edges meeting in space without sharing a vertex, the bowtie a plain
/// drag can produce with no tangency or dimension involved. Such a loop is
/// combinatorially closed and simple by the graph's own account, yet draws
/// no area a fill or an extrusion could make sense of.
pub(crate) fn is_simple(outline: &[DVec2]) -> bool {
    let count = outline.len();
    for i in 0..count {
        let (a1, a2) = (outline[i], outline[(i + 1) % count]);
        for j in (i + 1)..count {
            let shares_a_vertex = j == i + 1 || (i == 0 && j == count - 1);
            if shares_a_vertex {
                continue;
            }
            let (b1, b2) = (outline[j], outline[(j + 1) % count]);
            if segments_cross(a1, a2, b1, b2) {
                return false;
            }
        }
    }
    true
}

fn segments_cross(a1: DVec2, a2: DVec2, b1: DVec2, b2: DVec2) -> bool {
    let straddles = |from: DVec2, to: DVec2, first: DVec2, second: DVec2| {
        let span = to - from;
        (span.perp_dot(first - from) > 0.0) != (span.perp_dot(second - from) > 0.0)
    };
    straddles(a1, a2, b1, b2) && straddles(b1, b2, a1, a2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_square_walked_in_order_is_simple() {
        let square = [
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(10.0, 10.0),
            DVec2::new(0.0, 10.0),
        ];
        assert!(is_simple(&square));
    }

    #[test]
    fn two_non_adjacent_sides_swapped_makes_a_bowtie_that_is_not_simple() {
        let bowtie = [
            DVec2::new(10.0, 0.0),
            DVec2::new(20.0, 5.0),
            DVec2::new(0.0, 10.0),
            DVec2::new(10.0, 10.0),
        ];
        assert!(!is_simple(&bowtie));
    }

    #[test]
    fn a_triangle_is_always_simple() {
        let triangle = [
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(5.0, 8.0),
        ];
        assert!(is_simple(&triangle));
    }
}
