//! What a leaf holds along one line, by arithmetic alone.
//!
//! The flats kernel is held to its promise by measuring its inputs' own
//! triangles along the lines, which works because its result is made of
//! them. A kernel that lays a true circle afresh on every body cannot be held
//! that way: a line grazing a wall meets two different chords in the input and
//! in the result. What it can be held to is the leaf itself — a slab between
//! the two ends of a prism, against the area of its outline — and, at every
//! place the line enters or leaves, how squarely it crosses and whether what
//! it crosses curves: the room a mesh of that surface may take along the line.

use glam::{DVec2, DVec3};

use super::{Leaf, Outline};

/// Where a line crosses the boundary of a leaf: how far along it, the cosine
/// between the line and the normal of the surface crossed there, and whether
/// that surface curves.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crossing {
    pub at: f64,
    pub cosine: f64,
    pub curved: bool,
}

/// A stretch of the line inside a leaf, from where it enters to where it
/// leaves.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stretch {
    pub from: Crossing,
    pub to: Crossing,
}

/// The line as it runs in the plane of a profile: its place and its step
/// across the plane, and its height and rise along the plane's normal, per
/// unit of distance along the line.
struct Seen {
    start: DVec2,
    step: DVec2,
    level: f64,
    rise: f64,
    speed: f64,
}

impl Leaf {
    /// The stretches of the line through `origin` along `direction` that lie
    /// inside this leaf, in order, `t` along the line being the point
    /// `origin + direction * t`. `None` for a revolution, which no kernel
    /// held this way raises.
    pub fn along(&self, origin: DVec3, direction: DVec3) -> Option<Vec<Stretch>> {
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = self
        else {
            return None;
        };
        let (base, u, v) = plane.frame();
        let normal = u.cross(v);
        let from = origin - base;
        let seen = Seen {
            start: DVec2::new(from.dot(u), from.dot(v)),
            step: DVec2::new(direction.dot(u), direction.dot(v)),
            level: from.dot(normal),
            rise: direction.dot(normal),
            speed: direction.length(),
        };
        let areas = match outline {
            Outline::Rectangle { low, high } => rectangle(&seen, *low, *high),
            Outline::Circle { center, radius, .. } => disc(&seen, *center, *radius),
            Outline::Ring {
                center,
                outer,
                inner,
            } => ring(&seen, *center, *outer, *inner),
            Outline::Star { corners, .. } => polygon(&seen, corners),
        };
        let Some(slab) = slab(&seen, *height) else {
            return Some(Vec::new());
        };
        Some(
            areas
                .into_iter()
                .filter_map(|area| {
                    let from = if slab.from.at > area.from.at {
                        slab.from
                    } else {
                        area.from
                    };
                    let to = if slab.to.at < area.to.at {
                        slab.to
                    } else {
                        area.to
                    };
                    (from.at < to.at).then_some(Stretch { from, to })
                })
                .collect(),
        )
    }
}

/// Where the line lies between the two ends of the prism.
fn slab(seen: &Seen, height: f64) -> Option<Stretch> {
    let (low, high) = (height.min(0.0), height.max(0.0));
    if seen.rise == 0.0 {
        return (low <= seen.level && seen.level <= high).then_some(Stretch {
            from: far(f64::NEG_INFINITY),
            to: far(f64::INFINITY),
        });
    }
    let cosine = seen.rise.abs() / seen.speed;
    let [one, other] = [low, high].map(|end| Crossing {
        at: (end - seen.level) / seen.rise,
        cosine,
        curved: false,
    });
    Some(if one.at <= other.at {
        Stretch {
            from: one,
            to: other,
        }
    } else {
        Stretch {
            from: other,
            to: one,
        }
    })
}

fn far(at: f64) -> Crossing {
    Crossing {
        at,
        cosine: 0.0,
        curved: false,
    }
}

/// The line against a rectangle, one pair of sides at a time: Liang and
/// Barsky's clipping.
fn rectangle(seen: &Seen, low: DVec2, high: DVec2) -> Vec<Stretch> {
    let (mut from, mut to) = (far(f64::NEG_INFINITY), far(f64::INFINITY));
    for axis in 0..2 {
        let (start, step) = (seen.start[axis], seen.step[axis]);
        if step == 0.0 {
            if start < low[axis] || start > high[axis] {
                return Vec::new();
            }
            continue;
        }
        let cosine = step.abs() / seen.speed;
        let [one, other] = [low[axis], high[axis]].map(|side| (side - start) / step);
        let (enter, leave) = (one.min(other), one.max(other));
        if enter > from.at {
            from = Crossing {
                at: enter,
                cosine,
                curved: false,
            };
        }
        if leave < to.at {
            to = Crossing {
                at: leave,
                cosine,
                curved: false,
            };
        }
    }
    if from.at < to.at {
        vec![Stretch { from, to }]
    } else {
        Vec::new()
    }
}

/// The line against a disc: the two roots of a quadratic, worked out so that
/// neither loses its digits to the other.
fn disc(seen: &Seen, center: DVec2, radius: f64) -> Vec<Stretch> {
    let away = seen.start - center;
    let (a, b, c) = (
        seen.step.length_squared(),
        2.0 * seen.step.dot(away),
        away.length_squared() - radius * radius,
    );
    if a == 0.0 {
        return if c < 0.0 {
            vec![Stretch {
                from: far(f64::NEG_INFINITY),
                to: far(f64::INFINITY),
            }]
        } else {
            Vec::new()
        };
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant <= 0.0 {
        return Vec::new();
    }
    let root = discriminant.sqrt();
    let q = -(b + b.signum() * root) / 2.0;
    let (one, other) = if q == 0.0 {
        (-root / (2.0 * a), root / (2.0 * a))
    } else {
        (q / a, c / q)
    };
    let cosine = root / (2.0 * radius * seen.speed);
    let crossing = |at: f64| Crossing {
        at,
        cosine,
        curved: true,
    };
    vec![Stretch {
        from: crossing(one.min(other)),
        to: crossing(one.max(other)),
    }]
}

/// The line against a disc with a smaller one taken out of its middle.
fn ring(seen: &Seen, center: DVec2, outer: f64, inner: f64) -> Vec<Stretch> {
    let Some(&rim) = disc(seen, center, outer).first() else {
        return Vec::new();
    };
    let Some(&bore) = disc(seen, center, inner).first() else {
        return vec![rim];
    };
    [
        Stretch {
            from: rim.from,
            to: bore.from,
        },
        Stretch {
            from: bore.to,
            to: rim.to,
        },
    ]
    .into_iter()
    .filter(|stretch| stretch.from.at < stretch.to.at)
    .collect()
}

/// The line against a polygon: everywhere it crosses a side, and inside
/// between every other pair of crossings.
fn polygon(seen: &Seen, corners: &[DVec2]) -> Vec<Stretch> {
    let mut crossings: Vec<Crossing> = Vec::new();
    for (index, &from) in corners.iter().enumerate() {
        let side = corners[(index + 1) % corners.len()] - from;
        let across = seen.step.perp_dot(side);
        if across == 0.0 {
            continue;
        }
        let to_corner = from - seen.start;
        let share = to_corner.perp_dot(seen.step) / across;
        if (0.0..1.0).contains(&share) {
            crossings.push(Crossing {
                at: to_corner.perp_dot(side) / across,
                cosine: across.abs() / (side.length() * seen.speed),
                curved: false,
            });
        }
    }
    crossings.sort_by(|one, other| one.at.total_cmp(&other.at));
    let (pairs, _) = crossings.as_chunks::<2>();
    pairs
        .iter()
        .filter(|[from, to]| from.at < to.at)
        .map(|&[from, to]| Stretch { from, to })
        .collect()
}
