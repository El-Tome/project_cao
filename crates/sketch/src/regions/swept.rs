//! A face cut into triangles by sweeping it in bands, counting how many times
//! its loops wind around each stretch.
//!
//! The ear clipper wants one simple loop, and a face is not always that: a
//! hole can touch the outline it is cut from, a face pinched where two curves
//! touch runs through one corner twice, and near such a touch the sampled
//! walls can even cross each other by less than their own sampling. The sweep
//! asks none of that. Every loop is read as it stands, the plane is cut into
//! bands at the corners' heights, and ground is matter wherever the loops wind
//! a net number of times around it. A touching hole or a pinched corner costs
//! nothing; walls that cross inside one band cost triangles overlapping by the
//! crossing's own size — a sliver of the sampling, never a bite of the face.

use glam::DVec2;

/// One edge's stand across a band: where it crosses the band's floor and
/// ceiling, and which way it winds.
struct Stand {
    floor: f64,
    ceiling: f64,
    winds: i32,
}

/// The triangles covering every place the loops wind a net number of times
/// around. The loops keep their own ways round: an outline one way and a hole
/// the other cancel where the hole is.
pub(super) fn swept(loops: &[&[DVec2]]) -> Vec<[DVec2; 3]> {
    let mut edges: Vec<(DVec2, DVec2)> = Vec::new();
    for points in loops {
        for index in 0..points.len() {
            let (from, to) = (points[index], points[(index + 1) % points.len()]);
            if from.y != to.y {
                edges.push((from, to));
            }
        }
    }
    let mut heights: Vec<f64> = edges.iter().flat_map(|(from, to)| [from.y, to.y]).collect();
    heights.sort_by(f64::total_cmp);
    heights.dedup();

    let mut triangles = Vec::new();
    for band in heights.windows(2) {
        let (floor, ceiling) = (band[0], band[1]);
        let mut stands: Vec<Stand> = edges
            .iter()
            .filter(|(from, to)| from.y.min(to.y) <= floor && from.y.max(to.y) >= ceiling)
            .map(|(from, to)| {
                let at =
                    |height: f64| from.x + (to.x - from.x) * (height - from.y) / (to.y - from.y);
                Stand {
                    floor: at(floor),
                    ceiling: at(ceiling),
                    winds: if to.y > from.y { 1 } else { -1 },
                }
            })
            .collect();
        stands
            .sort_by(|near, far| (near.floor + near.ceiling).total_cmp(&(far.floor + far.ceiling)));

        let mut winding = 0;
        for pair in stands.windows(2) {
            let (left, right) = (&pair[0], &pair[1]);
            winding += left.winds;
            if winding == 0 {
                continue;
            }
            let corners = [
                DVec2::new(left.floor, floor),
                DVec2::new(right.floor, floor),
                DVec2::new(right.ceiling, ceiling),
                DVec2::new(left.ceiling, ceiling),
            ];
            for triangle in [
                [corners[0], corners[1], corners[2]],
                [corners[0], corners[2], corners[3]],
            ] {
                let spans = (triangle[1] - triangle[0])
                    .perp_dot(triangle[2] - triangle[0])
                    .abs();
                if spans > f64::MIN_POSITIVE {
                    triangles.push(triangle);
                }
            }
        }
    }
    triangles
}

#[cfg(test)]
mod tests;
