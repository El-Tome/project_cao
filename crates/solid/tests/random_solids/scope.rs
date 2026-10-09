//! The scope rule: which cases may ask the exact kernel for a curve it does
//! not build, read off the walls their leaves are bounded by, so that a
//! decline as unsupported is counted rather than failed.
//!
//! #552 splits it in two, the old rule kept beside them until the kernel
//! builds the conics: what asks beyond the conics — skew cylinders, a cone
//! against anything off its axis — and the conics a case's planes meet its
//! cylinders and cones along, told kind by kind. The two together answer
//! as the old rule did.

use glam::{DVec2, DVec3};

use super::{Case, Leaf, Outline};

impl Case {
    /// Whether the case may ask the exact kernel for a curve it does not
    /// build. A plane meeting a cylinder neither square to its axis nor
    /// along it, an ellipse, or two cylinders whose axes are neither
    /// parallel nor square, a skew meeting (#533): turned leaves bring them,
    /// with an end that stops off the quarter turns, or an axis slanted in
    /// its plane. And a cone (#536) meeting anything but a plane square to
    /// its axis, a plane holding its axis or parallel to it past where its
    /// leaf reaches, or a cylinder or a cone about the very same axis, where
    /// its leaf's box meets the other's: a cone a hair off coaxial asks too,
    /// and so does a plane tangent to its widest rim, or past it by less than
    /// the kernel tells a face clear (5365216226). The kernel may decline
    /// such a case as unsupported; it may also hold it, the leaves never
    /// meeting there.
    pub fn asks_for_a_conic(&self) -> bool {
        let walled: Vec<(Vec<Wall>, Spanned)> = self
            .leaves()
            .map(|leaf| {
                (
                    walls(leaf),
                    leaf.is_solid().then(|| leaf.bounds()).flatten(),
                )
            })
            .collect();
        let all = || walled.iter().flat_map(|(walls, _)| walls);
        let planes: Vec<DVec3> = all()
            .filter_map(|wall| match wall {
                Wall::Plane { normal, .. } => Some(*normal),
                _ => None,
            })
            .collect();
        let cylinders: Vec<DVec3> = all()
            .filter_map(|wall| match wall {
                Wall::Cylinder { axis, .. } => Some(*axis),
                _ => None,
            })
            .collect();
        let an_ellipse = cylinders.iter().any(|axis| {
            planes.iter().any(|normal| oblique(*normal, *axis))
                || cylinders.iter().any(|other| oblique(*other, *axis))
        });
        let reach = walled
            .iter()
            .filter_map(|(_, bounds)| *bounds)
            .map(|(low, high)| low.abs().max(high.abs()).max_element())
            .fold(1.0, f64::max);
        let near = |one: Spanned, other: Spanned| {
            let margin = 1e-6 * reach;
            match (one, other) {
                (Some((low, high)), Some((other_low, other_high))) => {
                    (low - margin).cmple(other_high).all() && (other_low - margin).cmple(high).all()
                }
                _ => false,
            }
        };
        let a_cone = walled.iter().any(|(walls, bounds)| {
            walls.iter().any(|wall| {
                let Wall::Cone {
                    axis,
                    point,
                    radius,
                } = *wall
                else {
                    return false;
                };
                walled
                    .iter()
                    .filter(|(_, others)| near(*bounds, *others))
                    .flat_map(|(others, _)| others)
                    .any(|other| other.asks_of_a_cone(axis, point, radius, reach))
            })
        });
        an_ellipse || a_cone
    }
}

/// The box a leaf spans, or nothing for a leaf that is no solid.
type Spanned = Option<(DVec3, DVec3)>;

/// A surface a leaf is bounded by, as the scope rule reads it: a plane by its
/// normal and a point of it, a cylinder or a cone by its axis and a point of
/// the axis, a cone besides by the furthest its leaf reaches from the axis.
enum Wall {
    Plane {
        normal: DVec3,
        point: DVec3,
    },
    Cylinder {
        axis: DVec3,
        point: DVec3,
    },
    Cone {
        axis: DVec3,
        point: DVec3,
        radius: f64,
    },
}

/// How close to parallel, square or on one line two walls are taken as so:
/// the rounding of a direction of unit length, and that share of the reach.
const ALIGNED: f64 = 1e-12;

/// How far past a cone's widest rim, as a share of the reach, a plane
/// parallel to its axis still asks: ten times the kernel's tolerance, which
/// tells a face clear of another only four tolerances off it
/// (`combine/clear.rs`). Nearer, the plane grazes the rim within what the
/// kernel can tell, and may cut it along a hyperbola it does not build.
const GRAZED: f64 = 1e-8;

impl Wall {
    /// Whether this wall meets a cone of `axis` through `point`, whose leaf
    /// reaches `radius` from the axis, in a curve the kernel does not build:
    /// anything but a plane square to the axis, holding it or parallel to it
    /// further off than the leaf reaches by more than the kernel tells
    /// clear, or a cylinder or a cone about the same line.
    fn asks_of_a_cone(&self, axis: DVec3, point: DVec3, radius: f64, reach: f64) -> bool {
        let axis = axis.normalize();
        match *self {
            Wall::Plane { normal, point: on } => {
                let normal = normal.normalize();
                let square = normal.cross(axis).length() <= ALIGNED;
                let off = normal.dot(point - on).abs();
                let parallel = normal.dot(axis).abs() <= ALIGNED;
                let clear = parallel && (off <= ALIGNED * reach || off > radius + GRAZED * reach);
                !square && !clear
            }
            Wall::Cylinder {
                axis: other,
                point: on,
            }
            | Wall::Cone {
                axis: other,
                point: on,
                ..
            } => {
                let parallel = other.normalize().cross(axis).length() <= ALIGNED;
                let off = (on - point) - axis * (on - point).dot(axis);
                !parallel || off.length() > ALIGNED * reach
            }
        }
    }
}

/// The planes a leaf is bounded by, the cylinders and the cones: a turn's
/// planes square to its axis at each end of its section and, short of a
/// whole turn, the two it starts and ends on; its cylinder, and its cone
/// when an edge of its section slopes; a prism's two ends, its sides and
/// the cylinders of its arcs.
fn walls(leaf: &Leaf) -> Vec<Wall> {
    if let Some(turned) = leaf.as_turned() {
        let swept = turned.swept(1.0);
        let (axis, point) = (swept.along, swept.origin);
        let mut walls: Vec<Wall> = turned
            .section
            .ends()
            .iter()
            .map(|end| Wall::Plane {
                normal: axis,
                point: point + axis * *end,
            })
            .collect();
        if !turned.is_whole() {
            let end = DVec2::from_angle(turned.angle());
            walls.push(Wall::Plane {
                normal: swept.onward,
                point,
            });
            walls.push(Wall::Plane {
                normal: swept.onward * end.x - swept.out * end.y,
                point,
            });
        }
        walls.push(Wall::Cylinder { axis, point });
        if turned.section.slopes() {
            let radius = turned
                .section
                .pieces()
                .iter()
                .map(|piece| piece.away[1].max(piece.ending[1]))
                .fold(0.0, f64::max);
            walls.push(Wall::Cone {
                axis,
                point,
                radius,
            });
        }
        return walls;
    }
    let Leaf::Prism {
        plane,
        outline,
        height,
    } = leaf
    else {
        return Vec::new();
    };
    let (base, u, v) = plane.frame();
    let normal = u.cross(v);
    let at = |place: DVec2| base + u * place.x + v * place.y;
    let mut walls = vec![
        Wall::Plane {
            normal,
            point: base,
        },
        Wall::Plane {
            normal,
            point: base + normal * *height,
        },
    ];
    let sides = |low: DVec2, high: DVec2| {
        [(u, low), (u, high), (v, low), (v, high)].map(|(normal, place)| Wall::Plane {
            normal,
            point: at(place),
        })
    };
    let round = |center: DVec2| Wall::Cylinder {
        axis: normal,
        point: at(center),
    };
    match outline {
        Outline::Rectangle { low, high } => walls.extend(sides(*low, *high)),
        Outline::Circle { center, .. } | Outline::Ring { center, .. } => walls.push(round(*center)),
        Outline::Rounded { low, high, radius } => {
            walls.extend(sides(*low, *high));
            walls.extend(
                [
                    *low + *radius,
                    *high - *radius,
                    DVec2::new(low.x + radius, high.y - radius),
                    DVec2::new(high.x - radius, low.y + radius),
                ]
                .map(round),
            );
        }
        Outline::Slot { from, to, radius } => {
            walls.extend(sides(from.min(*to) - *radius, from.max(*to) + *radius));
            walls.extend([*from, *to].map(round));
        }
        Outline::Star { corners, .. } => {
            walls.extend((0..corners.len()).map(|index| {
                let side = corners[(index + 1) % corners.len()] - corners[index];
                Wall::Plane {
                    normal: u * side.y - v * side.x,
                    point: at(corners[index]),
                }
            }));
        }
    }
    walls
}

/// Whether two directions are neither parallel nor square to each other,
/// by more than the rounding of a direction of unit length.
fn oblique(one: DVec3, other: DVec3) -> bool {
    let (one, other) = (one.normalize(), other.normalize());
    one.dot(other).abs() > ALIGNED && one.cross(other).length() > ALIGNED
}

/// How many pairs of a case's walls meet along each kind of conic, as the
/// scope rule reads them (#552): a plane neither square to a cylinder's axis
/// nor along it cuts an ellipse; a plane meeting a cone, near the box its
/// leaf spans, neither square to its axis nor holding it nor parallel to it
/// past its widest rim, cuts an ellipse, a parabola or a hyperbola, or
/// through its tip two lines, a ruling it touches along, or the tip alone,
/// as its angle with the axis is steeper than the cone's slope, as steep or
/// flatter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Conics {
    pub ellipses: usize,
    pub parabolas: usize,
    pub hyperbolas: usize,
    pub lines_through_an_apex: usize,
    pub apex_alone: usize,
    pub tangent_rulings: usize,
}

impl Conics {
    pub fn total(&self) -> usize {
        self.ellipses
            + self.parabolas
            + self.hyperbolas
            + self.lines_through_an_apex
            + self.apex_alone
            + self.tangent_rulings
    }
}

/// A cone of a turned leaf as the conics are told by: its tip, its axis of
/// unit length, and its slope off the axis in radians.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Slope {
    tip: DVec3,
    axis: DVec3,
    angle: f64,
}

impl Slope {
    /// The conic a plane through `on` square to `normal` cuts this cone
    /// along, a plane that neither stands square to its axis nor holds it.
    fn cut(&self, normal: DVec3, on: DVec3, reach: f64, conics: &mut Conics) {
        let normal = normal.normalize();
        let along = normal.dot(self.axis).abs();
        let with_the_axis = along.atan2(normal.cross(self.axis).length());
        let through_the_tip = normal.dot(self.tip - on).abs() <= ALIGNED * reach;
        let kind = if along <= ALIGNED {
            &mut conics.hyperbolas
        } else if (with_the_axis - self.angle).abs() <= ALIGNED {
            if through_the_tip {
                &mut conics.tangent_rulings
            } else {
                &mut conics.parabolas
            }
        } else if with_the_axis < self.angle {
            if through_the_tip {
                &mut conics.lines_through_an_apex
            } else {
                &mut conics.hyperbolas
            }
        } else if through_the_tip {
            &mut conics.apex_alone
        } else {
            &mut conics.ellipses
        };
        *kind += 1;
    }
}

/// The cones of a turned leaf: one for every slanted edge of its section,
/// read as drawn, either side of the axis.
fn slopes(leaf: &Leaf) -> Vec<Slope> {
    let Some(turned) = leaf.as_turned() else {
        return Vec::new();
    };
    let swept = turned.swept(1.0);
    let ends = turned.section.ends();
    let mut slopes: Vec<Slope> = Vec::new();
    for index in 0..turned.section.bands.len() {
        let (start, end) = turned.section.edges(index);
        let run = ends[index + 1] - ends[index];
        for edge in 0..2 {
            let (from, to) = (start[edge], end[edge]);
            if from == to || run <= 0.0 {
                continue;
            }
            let tip = ends[index] - from * run / (to - from);
            let slope = Slope {
                tip: swept.origin + swept.along * tip,
                axis: swept.along,
                angle: (to - from).abs().atan2(run),
            };
            if !slopes.contains(&slope) {
                slopes.push(slope);
            }
        }
    }
    slopes
}

/// A case's walls read leaf by leaf, with the box each leaf spans and its
/// cones, and how far the case reaches.
struct Walled {
    leaves: Vec<(Vec<Wall>, Spanned, Vec<Slope>)>,
    reach: f64,
}

impl Walled {
    fn of(case: &Case) -> Walled {
        let leaves: Vec<(Vec<Wall>, Spanned, Vec<Slope>)> = case
            .leaves()
            .map(|leaf| {
                (
                    walls(leaf),
                    leaf.is_solid().then(|| leaf.bounds()).flatten(),
                    slopes(leaf),
                )
            })
            .collect();
        let reach = leaves
            .iter()
            .filter_map(|(_, bounds, _)| *bounds)
            .map(|(low, high)| low.abs().max(high.abs()).max_element())
            .fold(1.0, f64::max);
        Walled { leaves, reach }
    }

    fn all(&self) -> impl Iterator<Item = &Wall> {
        self.leaves.iter().flat_map(|(walls, _, _)| walls)
    }

    fn cylinders(&self) -> Vec<DVec3> {
        self.all()
            .filter_map(|wall| match wall {
                Wall::Cylinder { axis, .. } => Some(*axis),
                _ => None,
            })
            .collect()
    }

    /// Whether two leaves' boxes meet, a millionth of the reach apart.
    fn near(&self, one: Spanned, other: Spanned) -> bool {
        let margin = 1e-6 * self.reach;
        match (one, other) {
            (Some((low, high)), Some((other_low, other_high))) => {
                (low - margin).cmple(other_high).all() && (other_low - margin).cmple(high).all()
            }
            _ => false,
        }
    }

    /// Every wall of a leaf whose box meets a cone's, with that cone's wall,
    /// its leaf's cones, and the wall met.
    fn against_cones(&self) -> Vec<(&Wall, &[Slope], &Wall)> {
        let mut met = Vec::new();
        for (walls, bounds, slopes) in &self.leaves {
            for cone in walls
                .iter()
                .filter(|wall| matches!(wall, Wall::Cone { .. }))
            {
                for (others, other_bounds, _) in &self.leaves {
                    if self.near(*bounds, *other_bounds) {
                        met.extend(others.iter().map(|other| (cone, slopes.as_slice(), other)));
                    }
                }
            }
        }
        met
    }
}

impl Case {
    /// Whether the case may ask the exact kernel for a curve that is no
    /// conic (#552): two cylinders whose axes are neither parallel nor
    /// square, a skew meeting; or a cone meeting a cylinder or a cone not
    /// about its very axis, where its leaf's box meets the other's. A plane
    /// never asks. The kernel may decline such a case as unsupported; it
    /// may also hold it, the leaves never meeting there.
    pub fn asks_beyond_the_conics(&self) -> bool {
        let walled = Walled::of(self);
        let cylinders = walled.cylinders();
        let skew = cylinders
            .iter()
            .any(|axis| cylinders.iter().any(|other| oblique(*other, *axis)));
        skew || walled.against_cones().into_iter().any(|(cone, _, other)| {
            let Wall::Cone {
                axis,
                point,
                radius,
            } = *cone
            else {
                return false;
            };
            !matches!(other, Wall::Plane { .. })
                && other.asks_of_a_cone(axis, point, radius, walled.reach)
        })
    }

    /// The conics the case's planes meet its cylinders and cones along, as
    /// `Conics` tells them.
    pub fn conics(&self) -> Conics {
        let walled = Walled::of(self);
        let mut conics = Conics::default();
        let cylinders = walled.cylinders();
        for wall in walled.all() {
            if let Wall::Plane { normal, .. } = wall {
                conics.ellipses += cylinders
                    .iter()
                    .filter(|axis| oblique(*normal, **axis))
                    .count();
            }
        }
        for (cone, slopes, other) in walled.against_cones() {
            let (
                Wall::Cone {
                    axis,
                    point,
                    radius,
                },
                Wall::Plane { normal, point: on },
            ) = (cone, other)
            else {
                continue;
            };
            if other.asks_of_a_cone(*axis, *point, *radius, walled.reach) {
                for slope in slopes {
                    slope.cut(*normal, *on, walled.reach, &mut conics);
                }
            }
        }
        conics
    }

    /// Whether some plane of the case meets a cylinder or a cone of it along
    /// a conic.
    pub fn meets_along_a_conic(&self) -> bool {
        self.conics().total() > 0
    }
}
