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
pub(super) struct Seen {
    pub(super) start: DVec2,
    pub(super) step: DVec2,
    pub(super) level: f64,
    pub(super) rise: f64,
    pub(super) speed: f64,
}

impl Leaf {
    /// The stretches of the line through `origin` along `direction` that lie
    /// inside this leaf, in order, `t` along the line being the point
    /// `origin + direction * t`.
    pub fn along(&self, origin: DVec3, direction: DVec3) -> Option<Vec<Stretch>> {
        self.along_grown(origin, direction, 0.0)
    }

    /// The same, for the leaf grown by `by` all round — its outline pushed out
    /// by `by` and each of its ends moved out by `by` — or shrunk when `by` is
    /// negative. A prism rather than the rounded solid a ball rolled round the
    /// leaf would sweep: a little larger grown and a little smaller shrunk,
    /// which is the side a room for a tolerance may err on. A turn is held
    /// to the slabs of annuli it sweeps (`around.rs`). `None` for a star
    /// grown at all, whose offset no formula here draws.
    pub fn along_grown(&self, origin: DVec3, direction: DVec3, by: f64) -> Option<Vec<Stretch>> {
        if let Some(turned) = self.as_turned() {
            return Some(turned.along_grown(origin, direction, by));
        }
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
            Outline::Rectangle { low, high } => rectangle(&seen, *low - by, *high + by),
            Outline::Circle { center, radius, .. } => disc(&seen, *center, *radius + by),
            Outline::Ring {
                center,
                outer,
                inner,
            } => ring(&seen, *center, *outer + by, *inner - by),
            Outline::Star { corners, .. } if by == 0.0 => polygon(&seen, corners),
            Outline::Star { .. } => return None,
            Outline::Rounded { .. } | Outline::Slot { .. } => {
                let pieces = outline.pieces(by).expect("an outline of runs and arcs");
                let rectangles = pieces
                    .rectangles
                    .iter()
                    .flat_map(|band| clipped(&seen, band.low, band.high, Some(band.walled)));
                let discs = pieces
                    .discs
                    .iter()
                    .flat_map(|(center, radius)| disc(&seen, *center, *radius));
                union(rectangles.chain(discs).collect())
            }
        };
        let Some(slab) = slab(&seen, height.min(0.0) - by, height.max(0.0) + by) else {
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

impl Leaf {
    /// The box a leaf spans, from its outline and its two ends or from the
    /// turn it sweeps: its true circles rather than their flats.
    pub fn bounds(&self) -> Option<(DVec3, DVec3)> {
        if let Some(turned) = self.as_turned() {
            return turned.bounds();
        }
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = self
        else {
            return None;
        };
        let (base, u, v) = plane.frame();
        let travel = u.cross(v) * *height;
        let round = |center: DVec2, radius: f64| {
            let middle = base + u * center.x + v * center.y;
            let spread = (u * u + v * v).map(f64::sqrt) * radius;
            [middle - spread, middle + spread]
        };
        let corners: Vec<DVec3> = match outline {
            Outline::Rectangle { low, high } => [
                *low,
                DVec2::new(high.x, low.y),
                *high,
                DVec2::new(low.x, high.y),
            ]
            .map(|corner| base + u * corner.x + v * corner.y)
            .to_vec(),
            Outline::Circle { center, radius, .. } => round(*center, *radius).to_vec(),
            Outline::Ring { center, outer, .. } => round(*center, *outer).to_vec(),
            Outline::Star { corners, .. } => corners
                .iter()
                .map(|corner| base + u * corner.x + v * corner.y)
                .collect(),
            Outline::Rounded { .. } | Outline::Slot { .. } => outline
                .pieces(0.0)
                .expect("an outline of runs and arcs")
                .discs
                .iter()
                .flat_map(|(center, radius)| round(*center, *radius))
                .collect(),
        };
        corners
            .iter()
            .flat_map(|corner| [*corner, *corner + travel])
            .map(|corner| (corner, corner))
            .reduce(|(low, high), (other, _)| (low.min(other), high.max(other)))
    }
}

/// Where the line lies between the two ends of the prism, at the heights
/// `low` and `high` along the plane's normal.
pub(super) fn slab(seen: &Seen, low: f64, high: f64) -> Option<Stretch> {
    if low >= high {
        return None;
    }
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

pub(super) fn far(at: f64) -> Crossing {
    Crossing {
        at,
        cosine: 0.0,
        curved: false,
    }
}

/// The line against a rectangle, one pair of sides at a time: Liang and
/// Barsky's clipping.
fn rectangle(seen: &Seen, low: DVec2, high: DVec2) -> Vec<Stretch> {
    clipped(seen, low, high, None)
}

/// The same for a rectangle that is a piece of a larger area, only the pair
/// of sides square to the axis `walled` lying on its outline: a line through
/// a corner, where a straight run meets an arc, crosses that pair and not the
/// one inside the area.
fn clipped(seen: &Seen, low: DVec2, high: DVec2, walled: Option<usize>) -> Vec<Stretch> {
    if low.cmpge(high).any() {
        return Vec::new();
    }
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
        let walled = walled == Some(axis);
        if enter > from.at || walled && enter == from.at {
            from = Crossing {
                at: enter,
                cosine,
                curved: false,
            };
        }
        if leave < to.at || walled && leave == to.at {
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
    if radius <= 0.0 {
        return Vec::new();
    }
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

/// The stretches of the line inside any of several areas, those that overlap
/// or touch made one: where one ends inside another the line crosses no
/// boundary. Of two crossings at one place, the one crossed less squarely is
/// kept, and a curved one before a straight one.
pub(super) fn union(mut stretches: Vec<Stretch>) -> Vec<Stretch> {
    stretches.sort_by(|one, other| one.from.at.total_cmp(&other.from.at));
    let mut merged: Vec<Stretch> = Vec::new();
    for stretch in stretches {
        let Some(last) = merged
            .last_mut()
            .filter(|last| stretch.from.at <= last.to.at)
        else {
            merged.push(stretch);
            continue;
        };
        if stretch.from.at == last.from.at {
            last.from = shallower(last.from, stretch.from);
        }
        if stretch.to.at > last.to.at {
            last.to = stretch.to;
        } else if stretch.to.at == last.to.at {
            last.to = shallower(last.to, stretch.to);
        }
    }
    merged
}

fn shallower(one: Crossing, other: Crossing) -> Crossing {
    if (other.cosine, !other.curved) < (one.cosine, !one.curved) {
        other
    } else {
        one
    }
}

/// The line against a cone about the axis the line is seen across: where it
/// stands no further from the axis than an edge `radius` away from it at
/// `at` along it, growing by `slope` per unit along it, and that edge on the
/// near side of the cone's tip. One quadratic, its roots worked out as
/// `disc` works them, held to the half of the line where the edge stands
/// off the axis. The line crosses the cone as squarely as its direction
/// meets the cone's normal; through the tip, where the cone has none, at a
/// cosine of nought, as a line grazing a wall is.
///
/// The quadratic is worked out from where the line crosses the level `at`,
/// unless it runs nearly square to the axis: a cone a hair from square
/// grows by millions of units per unit along its axis, and its radius read
/// where the line starts, far from that level, would lose every digit that
/// matters.
pub(super) fn cone(seen: &Seen, at: f64, radius: f64, slope: f64) -> Vec<Stretch> {
    if seen.rise.abs() < 1e-3 * seen.speed {
        return cone_from_where_it_starts(seen, at, radius, slope);
    }
    let shift = (at - seen.level) / seen.rise;
    let moved = Seen {
        start: seen.start + seen.step * shift,
        step: seen.step,
        level: at,
        rise: seen.rise,
        speed: seen.speed,
    };
    let back = |crossing: Crossing| Crossing {
        at: crossing.at + shift,
        ..crossing
    };
    cone_from_where_it_starts(&moved, at, radius, slope)
        .into_iter()
        .map(|stretch| Stretch {
            from: back(stretch.from),
            to: back(stretch.to),
        })
        .collect()
}

fn cone_from_where_it_starts(seen: &Seen, at: f64, radius: f64, slope: f64) -> Vec<Stretch> {
    let base = radius + slope * (seen.level - at);
    let climb = slope * seen.rise;
    let crossing = |at: f64| Crossing {
        at,
        cosine: across_a_cone(seen, slope, at),
        curved: true,
    };
    let everywhere = || Stretch {
        from: far(f64::NEG_INFINITY),
        to: far(f64::INFINITY),
    };
    let inside = if seen.step == DVec2::ZERO {
        beyond_an_edge(seen.start.length(), base, climb, crossing(0.0))
    } else {
        let (a, b, c) = (
            seen.step.length_squared() - climb * climb,
            2.0 * (seen.start.dot(seen.step) - base * climb),
            seen.start.length_squared() - base * base,
        );
        if a == 0.0 {
            if b == 0.0 {
                if c <= 0.0 {
                    vec![everywhere()]
                } else {
                    Vec::new()
                }
            } else if b > 0.0 {
                vec![Stretch {
                    from: far(f64::NEG_INFINITY),
                    to: crossing(-c / b),
                }]
            } else {
                vec![Stretch {
                    from: crossing(-c / b),
                    to: far(f64::INFINITY),
                }]
            }
        } else {
            let discriminant = b * b - 4.0 * a * c;
            if discriminant <= 0.0 {
                if a > 0.0 {
                    Vec::new()
                } else {
                    vec![everywhere()]
                }
            } else {
                let root = discriminant.sqrt();
                let q = -(b + b.signum() * root) / 2.0;
                let (one, other) = if q == 0.0 {
                    (-root / (2.0 * a), root / (2.0 * a))
                } else {
                    (q / a, c / q)
                };
                let (near, far_root) = (one.min(other), one.max(other));
                if a > 0.0 {
                    vec![Stretch {
                        from: crossing(near),
                        to: crossing(far_root),
                    }]
                } else {
                    vec![
                        Stretch {
                            from: far(f64::NEG_INFINITY),
                            to: crossing(near),
                        },
                        Stretch {
                            from: crossing(far_root),
                            to: far(f64::INFINITY),
                        },
                    ]
                }
            }
        }
    };
    let tip = Crossing {
        at: 0.0,
        cosine: 0.0,
        curved: true,
    };
    meet(&inside, &beyond_an_edge(0.0, base, climb, tip))
}

/// Where an edge `base + climb·t` away from the axis stands further from it
/// than `away`, which is where it is crossed when it is crossed at all.
fn beyond_an_edge(away: f64, base: f64, climb: f64, crossed: Crossing) -> Vec<Stretch> {
    if climb == 0.0 {
        return if base >= away {
            vec![Stretch {
                from: far(f64::NEG_INFINITY),
                to: far(f64::INFINITY),
            }]
        } else {
            Vec::new()
        };
    }
    let crossing = Crossing {
        at: (away - base) / climb,
        ..crossed
    };
    vec![if climb > 0.0 {
        Stretch {
            from: crossing,
            to: far(f64::INFINITY),
        }
    } else {
        Stretch {
            from: far(f64::NEG_INFINITY),
            to: crossing,
        }
    }]
}

/// The cosine between the line and the normal of a cone of `slope` where
/// the line stands `at` along it: nought on the axis, where the cone has no
/// normal, or within the rounding of the place's own coordinates of it.
fn across_a_cone(seen: &Seen, slope: f64, at: f64) -> f64 {
    let place = seen.start + seen.step * at;
    let away = place.length();
    if away <= 1e-9 * (seen.start.length() + (seen.step * at).length()) {
        return 0.0;
    }
    (seen.step.dot(place) / away - slope * seen.rise).abs()
        / ((1.0 + slope * slope).sqrt() * seen.speed)
}

/// Where the line lies inside both of two sets of stretches.
pub(super) fn meet(one: &[Stretch], other: &[Stretch]) -> Vec<Stretch> {
    let mut met = Vec::new();
    for first in one {
        for second in other {
            let from = if second.from.at > first.from.at {
                second.from
            } else {
                first.from
            };
            let to = if second.to.at < first.to.at {
                second.to
            } else {
                first.to
            };
            if from.at < to.at {
                met.push(Stretch { from, to });
            }
        }
    }
    met
}

/// Where the line lies outside every one of some stretches that overlap
/// nowhere.
pub(super) fn outside(stretches: &[Stretch]) -> Vec<Stretch> {
    let mut sorted = stretches.to_vec();
    sorted.sort_by(|one, other| one.from.at.total_cmp(&other.from.at));
    let mut from = far(f64::NEG_INFINITY);
    let mut gaps = Vec::new();
    for stretch in sorted {
        gaps.push(Stretch {
            from,
            to: stretch.from,
        });
        from = stretch.to;
    }
    gaps.push(Stretch {
        from,
        to: far(f64::INFINITY),
    });
    gaps.retain(|gap| gap.from.at < gap.to.at);
    gaps
}

/// The line against a disc with a smaller one taken out of its middle.
pub(super) fn ring(seen: &Seen, center: DVec2, outer: f64, inner: f64) -> Vec<Stretch> {
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
/// between every other pair of crossings. A line square to the plane crosses
/// no side, and is inside all along or nowhere.
fn polygon(seen: &Seen, corners: &[DVec2]) -> Vec<Stretch> {
    if seen.step == DVec2::ZERO {
        return if encloses(corners, seen.start) {
            vec![Stretch {
                from: far(f64::NEG_INFINITY),
                to: far(f64::INFINITY),
            }]
        } else {
            Vec::new()
        };
    }
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

/// Whether a place lies inside a polygon: an odd number of its sides cross the
/// ray from it along the first axis, a side counted with its lower end and
/// not its upper, so that a ray through a corner counts it once.
fn encloses(corners: &[DVec2], place: DVec2) -> bool {
    let mut inside = false;
    for (index, &from) in corners.iter().enumerate() {
        let to = corners[(index + 1) % corners.len()];
        if (from.y > place.y) != (to.y > place.y) {
            let across = from.x + (place.y - from.y) / (to.y - from.y) * (to.x - from.x);
            if place.x < across {
                inside = !inside;
            }
        }
    }
    inside
}
