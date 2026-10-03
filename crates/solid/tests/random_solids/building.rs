//! A leaf handed to the kernel, and what the kernel was promised for it.

use cao_solid::{Body, Loop};
use glam::DVec2;

use super::{CIRCLE_STEPS, Leaf, Outline};

/// The loops a prism is raised from, and the triangles its two ends are
/// filled with.
struct Area {
    outline: Vec<DVec2>,
    outline_curves: Vec<Option<usize>>,
    holes: Vec<(Vec<DVec2>, Vec<Option<usize>>)>,
    triangles: Vec<[DVec2; 3]>,
}

fn circle(center: DVec2, radius: f64, from: f64) -> Vec<DVec2> {
    (0..CIRCLE_STEPS)
        .map(|step| {
            let angle =
                from.to_radians() + std::f64::consts::TAU * step as f64 / CIRCLE_STEPS as f64;
            center + DVec2::from_angle(angle) * radius
        })
        .collect()
}

fn fan_from(center: DVec2, corners: &[DVec2]) -> Vec<[DVec2; 3]> {
    (0..corners.len())
        .map(|index| [center, corners[index], corners[(index + 1) % corners.len()]])
        .collect()
}

fn area_of(outline: &Outline) -> Area {
    let straight = |points: &[DVec2]| vec![None; points.len()];
    let curved = |points: &[DVec2]| vec![Some(0); points.len()];
    match outline {
        Outline::Rectangle { low, high } => {
            let corners = vec![
                *low,
                DVec2::new(high.x, low.y),
                *high,
                DVec2::new(low.x, high.y),
            ];
            Area {
                triangles: vec![
                    [corners[0], corners[1], corners[2]],
                    [corners[0], corners[2], corners[3]],
                ],
                outline_curves: straight(&corners),
                outline: corners,
                holes: Vec::new(),
            }
        }
        Outline::Circle {
            center,
            radius,
            from,
        } => {
            let corners = circle(*center, *radius, *from);
            Area {
                triangles: fan_from(*center, &corners),
                outline_curves: curved(&corners),
                outline: corners,
                holes: Vec::new(),
            }
        }
        Outline::Star { center, corners } => Area {
            triangles: fan_from(*center, corners),
            outline_curves: straight(corners),
            outline: corners.clone(),
            holes: Vec::new(),
        },
        Outline::Ring {
            center,
            outer,
            inner,
        } => {
            let (rim, bore) = (circle(*center, *outer, 0.0), circle(*center, *inner, 0.0));
            let triangles = (0..CIRCLE_STEPS)
                .flat_map(|index| {
                    let next = (index + 1) % CIRCLE_STEPS;
                    [
                        [bore[index], rim[index], rim[next]],
                        [bore[index], rim[next], bore[next]],
                    ]
                })
                .collect();
            Area {
                triangles,
                outline_curves: curved(&rim),
                outline: rim,
                holes: vec![(bore.clone(), curved(&bore))],
            }
        }
    }
}

/// Whether every corner of a star is seen from its centre turning the same way
/// round: what makes a fan from the centre fill it. A shrunk case can drop the
/// corner that kept the centre inside.
fn seen_whole_from(center: DVec2, corners: &[DVec2]) -> bool {
    let turns: Vec<f64> = fan_from(center, corners)
        .iter()
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a))
        .collect();
    corners.len() >= 3
        && (turns.iter().all(|turn| *turn > 0.0) || turns.iter().all(|turn| *turn < 0.0))
}

fn signed_area(points: &[DVec2]) -> f64 {
    (0..points.len())
        .map(|index| points[index].perp_dot(points[(index + 1) % points.len()]))
        .sum::<f64>()
        * 0.5
}

impl Leaf {
    /// Whether the leaf describes a solid at all: an area that encloses
    /// something, pushed or turned by something, and turned about an axis it
    /// keeps to one side of — bar a hair, which is a profile meant to touch
    /// the axis. A shrunk case can round a rectangle flat, and a solid of no
    /// volume is no failure of the kernel's.
    pub fn is_solid(&self) -> bool {
        match self {
            Leaf::Prism {
                outline, height, ..
            } => {
                *height != 0.0
                    && match outline {
                        Outline::Rectangle { low, high } => low.x < high.x && low.y < high.y,
                        Outline::Circle { radius, .. } => *radius > 0.0,
                        Outline::Star { center, corners } => seen_whole_from(*center, corners),
                        Outline::Ring { outer, inner, .. } => 0.0 < *inner && inner < outer,
                    }
            }
            Leaf::Revolution {
                low, high, degrees, ..
            } => {
                let hair = 1e-3 * (high.x - low.x);
                low.x < high.x
                    && low.y < high.y
                    && *degrees != 0.0
                    && (low.x >= -hair || high.x <= hair)
            }
        }
    }

    /// The solid the kernel raises for this leaf, or `None` when the leaf is
    /// no solid at all or the kernel declined to raise one.
    pub fn solid(&self) -> Option<Body> {
        if !self.is_solid() {
            return None;
        }
        match self {
            Leaf::Prism {
                plane,
                outline,
                height,
            } => {
                let area = area_of(outline);
                let holes: Vec<Loop<'_>> = area
                    .holes
                    .iter()
                    .map(|(points, curves)| Loop { points, curves })
                    .collect();
                Some(Body::prism(
                    Loop {
                        points: &area.outline,
                        curves: &area.outline_curves,
                    },
                    &holes,
                    &area.triangles,
                    |point| plane.to_world(point),
                    plane.normal() * *height,
                ))
            }
            Leaf::Revolution {
                plane,
                low,
                high,
                degrees,
            } => {
                let area = area_of(&Outline::Rectangle {
                    low: *low,
                    high: *high,
                });
                Body::revolution(
                    Loop::straight(&area.outline),
                    &[],
                    &area.triangles,
                    |point| plane.to_world(point),
                    DVec2::ZERO,
                    DVec2::Y,
                    degrees.to_radians(),
                )
            }
        }
    }

    /// The volume the operation promised for this leaf, from arithmetic alone,
    /// and how far below it the kernel may land because it lays flats where
    /// the surface curves.
    ///
    /// A prism is promised its area times its height exactly: the kernel was
    /// handed the flats a circle was sampled into, and those are what it
    /// promised to raise. A revolution is promised Pappus's volume, which the
    /// flats laid around the turn stand inside of.
    pub fn promise(&self) -> (f64, f64) {
        match self {
            Leaf::Prism {
                outline, height, ..
            } => {
                let area = area_of(outline);
                let enclosed = signed_area(&area.outline).abs()
                    - area
                        .holes
                        .iter()
                        .map(|(points, _)| signed_area(points).abs())
                        .sum::<f64>();
                (enclosed * height.abs(), 0.0)
            }
            Leaf::Revolution {
                low, high, degrees, ..
            } => {
                let (near, far) = (low.x.abs().min(high.x.abs()), low.x.abs().max(high.x.abs()));
                let pappus = degrees.abs().to_radians() / 2.0
                    * (far * far - near * near)
                    * (high.y - low.y).abs();
                (pappus, pappus * FACETING)
            }
        }
    }
}

/// How much of a curved surface's volume its flats may lose: at sixty-four
/// flats a turn they stand inside the curve and give up 0.16 %. A fifth of a
/// percent covers that, and a kernel meshing a true curve to a hundredth of a
/// unit, and nothing like a face gone missing.
pub const FACETING: f64 = 2e-3;
