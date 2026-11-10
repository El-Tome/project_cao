//! The narrowest cap of the unit sphere holding a piece's ways from the
//! centre: its axis is the way to the point of their hull nearest the
//! centre, and the cosine of its half-angle that point's distance. No axis
//! does better: along any axis, that point stands no further than its
//! distance, and being a blend of the ways, one of them stands no further
//! either.

use glam::DVec3;

/// How many points the walk to the nearest point of the hull may take:
/// it takes one more of the ways each step and ends on three at most, so a
/// handful is all a piece of sphere asks.
const STEPS: usize = 64;

/// How close, as a share of its distance, the nearest point found must
/// stand to the nearest point of the hull for the walk to end.
const CLOSE: f64 = 1e-12;

/// The axis of the narrowest cap holding every one of `ways`: the way to
/// the point of their hull nearest the origin, walked to as the distance
/// between two convex sets is (Gilbert, Johnson and Keerthi), from the first
/// way. None when that point is the origin.
pub(super) fn axis(ways: &[DVec3]) -> Option<DVec3> {
    let beyond = |near: DVec3| {
        ways.iter()
            .copied()
            .min_by(|one, other| one.dot(near).total_cmp(&other.dot(near)))
    };
    let mut simplex = vec![*ways.first()?];
    let mut near = simplex[0];
    for _ in 0..STEPS {
        let far = beyond(near)?;
        let square = near.length_squared();
        if square - near.dot(far) <= CLOSE * square || simplex.contains(&far) {
            break;
        }
        simplex.push(far);
        (near, simplex) = nearest(&simplex);
    }
    near.try_normalize()
}

/// The point of the hull of `simplex` nearest the origin, and the corners
/// of the face it stands inside: the nearest of the feet that fall inside
/// their face.
fn nearest(simplex: &[DVec3]) -> (DVec3, Vec<DVec3>) {
    let mut best = (simplex[0], vec![simplex[0]]);
    for mask in 1..1u32 << simplex.len() {
        if mask.count_ones() > 3 {
            continue;
        }
        let corners: Vec<DVec3> = (0..simplex.len())
            .filter(|at| mask >> at & 1 == 1)
            .map(|at| simplex[at])
            .collect();
        if let Some(foot) = foot(&corners)
            && foot.length_squared() < best.0.length_squared()
        {
            best = (foot, corners);
        }
    }
    best
}

/// The foot of the origin on the point, line or plane of `corners`, where
/// it falls strictly inside them.
fn foot(corners: &[DVec3]) -> Option<DVec3> {
    match *corners {
        [a] => Some(a),
        [a, b] => {
            let along = b - a;
            let share = -a.dot(along) / along.length_squared();
            (share > 0.0 && share < 1.0).then(|| a + along * share)
        }
        [a, b, c] => {
            let normal = (b - a).cross(c - a);
            let square = normal.length_squared();
            if square == 0.0 {
                return None;
            }
            let foot = normal * (a.dot(normal) / square);
            [(a, b), (b, c), (c, a)]
                .iter()
                .all(|&(from, to)| (to - from).cross(foot - from).dot(normal) > 0.0)
                .then_some(foot)
        }
        _ => None,
    }
}
