//! A leaf handed to the kernel, and what the kernel was promised for it.

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::turning::{self, Turn};
use cao_solid::{Body, Declined, Loop};
use glam::DVec2;

use super::{Axis, CIRCLE_STEPS, Leaf, Outline, Section, Turned};

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
        Outline::Rounded { low, high, .. } => convex(outline, (*low + *high) / 2.0),
        Outline::Slot { from, to, .. } => convex(outline, (*from + *to) / 2.0),
    }
}

/// An outline of straight runs and arcs, which is convex: its flats sampled
/// from its contour, and a fan from a point inside to fill it.
fn convex(outline: &Outline, inside: DVec2) -> Area {
    let contour = outline.contour().expect("an outline of runs and arcs");
    let (corners, curves) = super::outlines::sampled(&contour);
    Area {
        triangles: fan_from(inside, &corners),
        outline_curves: curves,
        outline: corners,
        holes: Vec::new(),
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
                        Outline::Rounded { low, high, radius } => {
                            low.x < high.x
                                && low.y < high.y
                                && 0.0 < *radius
                                && 2.0 * radius <= (*high - *low).min_element()
                        }
                        Outline::Slot { from, to, radius } => {
                            0.0 < *radius && from != to && (from.x == to.x || from.y == to.y)
                        }
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
            Leaf::Turned {
                section, degrees, ..
            } => section.is_solid() && 0.0 < degrees.abs() && degrees.abs() <= 360.0,
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
                let (origin, u, v) = plane.frame();
                let profile = Profile {
                    exact: None,
                    sampled: Loop {
                        points: &area.outline,
                        curves: &area.outline_curves,
                    },
                    sampled_holes: holes,
                    triangles: &area.triangles,
                };
                Body::default()
                    .tool_raised(&profile, Frame { origin, u, v }, plane.normal() * *height)
                    .ok()
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
                let profile = Profile {
                    exact: None,
                    sampled: Loop::straight(&area.outline),
                    sampled_holes: Vec::new(),
                    triangles: &area.triangles,
                };
                let axis = turning::Axis {
                    origin: DVec2::ZERO,
                    direction: DVec2::Y,
                };
                let turn = Turn::of(axis, degrees.to_radians(), 0.0, &profile);
                let (origin, u, v) = plane.frame();
                Body::default()
                    .tool_turned(&profile, Frame { origin, u, v }, &turn)
                    .ok()
                    .filter(|body| !body.is_empty())
            }
            Leaf::Turned { .. } => self.as_turned()?.tool(false).ok(),
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
            Leaf::Turned { .. } => {
                let volume = self.as_turned().map_or(0.0, |turned| turned.volume());
                (volume, volume * FACETING)
            }
        }
    }
}

/// How much of a curved surface's volume its flats may lose: at sixty-four
/// flats a turn they stand inside the curve and give up 0.16 %. A fifth of a
/// percent covers that, and a kernel meshing a true curve to a hundredth of a
/// unit, and nothing like a face gone missing.
pub const FACETING: f64 = 2e-3;

/// One side of a turned leaf as the kernels are handed it, in the plane's
/// own coordinates: its outline and its holes, and the triangles its ends are
/// filled with, two to every rectangle of the section.
pub struct Drawn {
    pub outline: Vec<DVec2>,
    pub holes: Vec<Vec<DVec2>>,
    pub triangles: Vec<[DVec2; 3]>,
}

impl Drawn {
    fn of(axis: &Axis, section: &Section) -> Drawn {
        let (outline, holes) = section.corners();
        let triangles = section
            .rectangles()
            .iter()
            .flat_map(|&[from, to, low, high]| {
                let corners = [
                    DVec2::new(from, low),
                    DVec2::new(to, low),
                    DVec2::new(to, high),
                    DVec2::new(from, high),
                ]
                .map(|corner| axis.at(corner));
                [
                    [corners[0], corners[1], corners[2]],
                    [corners[0], corners[2], corners[3]],
                ]
            })
            .collect();
        Drawn {
            outline: outline.iter().map(|corner| axis.at(*corner)).collect(),
            holes: holes
                .iter()
                .map(|hole| hole.iter().map(|corner| axis.at(*corner)).collect())
                .collect(),
            triangles,
        }
    }

    /// The outline and the holes as runs, every one of them straight.
    pub fn contours(&self) -> (Contour, Vec<Contour>) {
        (
            Contour::straight(self.outline.clone()),
            self.holes
                .iter()
                .map(|hole| Contour::straight(hole.clone()))
                .collect(),
        )
    }

    /// The side as the application hands an area over: both ways, or, when
    /// not `exact`, sampled alone, which keeps it on the flats.
    pub fn profile(&self, exact: bool) -> Profile<'_> {
        Profile {
            exact: exact.then(|| self.contours()),
            sampled: Loop::straight(&self.outline),
            sampled_holes: self.holes.iter().map(|hole| Loop::straight(hole)).collect(),
            triangles: &self.triangles,
        }
    }
}

impl Turned {
    pub fn frame(&self) -> Frame {
        let (origin, u, v) = self.plane.frame();
        Frame { origin, u, v }
    }

    /// Each side of the section as the kernels are handed it: the section
    /// itself when it lies on one side of its axis, its two sides otherwise.
    pub fn drawn(&self) -> Vec<Drawn> {
        self.section
            .sides()
            .iter()
            .map(|side| Drawn::of(&self.axis, side))
            .collect()
    }

    /// The turn the kernels are handed, at the drawing's `resolution`, how
    /// close to the axis counts as on it read off the whole section.
    pub fn turn(&self, resolution: f64) -> Turn {
        let whole = Drawn::of(&self.axis, &self.section);
        Turn::of(
            self.axis.line(),
            self.degrees.to_radians(),
            resolution,
            &whole.profile(true),
        )
    }

    /// The leaf turned through the application's body, each side apart and
    /// the two joined: by the exact kernel, or by the flats when not `exact`.
    pub fn tool(&self, exact: bool) -> Result<Body, Declined> {
        let (frame, turn) = (self.frame(), self.turn(0.0));
        self.drawn()
            .iter()
            .map(|side| Body::default().tool_turned(&side.profile(exact), frame, &turn))
            .try_fold(None, |joined: Option<Body>, side| {
                let side = side?;
                Ok(Some(match joined {
                    Some(joined) => joined.union(&side)?,
                    None => side,
                }))
            })
            .map(Option::unwrap_or_default)
    }

    /// Whether a hole of the section stands a hair from an edge of its band
    /// or from the axis: a wall the exact kernel cannot lay square without
    /// bringing the hole onto the outline, so it may decline the section as
    /// no profile it reads, and the application turns it on the flats.
    pub fn has_a_wall_a_hair_thin(&self) -> bool {
        let Some((low, high)) = self.bounds() else {
            return false;
        };
        let hair = A_HAIR_THIN * low.abs().max(high.abs()).max_element();
        let ends = self.section.ends();
        self.section.holes.iter().any(|[hole_low, hole_high]| {
            let band = (0..self.section.bands.len())
                .find(|&index| ends[index] < hole_low.x && hole_high.x < ends[index + 1]);
            band.is_some_and(|index| {
                let [_, low, high] = self.section.bands[index];
                [
                    hole_low.x - ends[index],
                    ends[index + 1] - hole_high.x,
                    hole_low.y - low,
                    high - hole_high.y,
                    hole_low.y.abs(),
                    hole_high.y.abs(),
                ]
                .iter()
                .any(|gap| *gap <= hair)
            })
        })
    }
}

/// How thin a wall the draw draws a hair thin is, as a share of how far the
/// leaf reaches: its hairs stand below it and its lattice far above, and the
/// tolerance the exact kernel lays a turned leaf at is under a twentieth of
/// it.
const A_HAIR_THIN: f64 = 1e-6;
