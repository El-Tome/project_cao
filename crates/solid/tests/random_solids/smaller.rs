//! The cases a failing case could shrink into.
//!
//! Every change offered here makes a case simpler and never undoes another:
//! a step removed, a curve made straight, a plane put back square to the axes,
//! a number rounded. That is what makes the shrinking finish — no two changes
//! lead back to where the first started.

use glam::{DVec2, DVec3};

use super::{Axis, Case, Leaf, Mode, Outline, Plane, Section, Step};

impl Case {
    pub fn smaller(&self) -> Vec<Case> {
        let mut smaller = Vec::new();
        for index in (0..self.steps.len()).rev() {
            let mut fewer = self.clone();
            fewer.steps.remove(index);
            smaller.push(fewer);
        }
        if !self.steps.is_empty() {
            for leaf in self.leaves() {
                smaller.push(Case::new(leaf.clone(), Vec::new()));
            }
        }
        if let Some(first) = self.steps.first().filter(|step| step.mode == Mode::Add) {
            smaller.push(Case::new(first.tool.clone(), self.steps[1..].to_vec()));
        }
        for simpler in self.start.simpler() {
            smaller.push(Case::new(simpler, self.steps.clone()));
        }
        for (index, step) in self.steps.iter().enumerate() {
            for simpler in step.tool.simpler() {
                let mut changed = self.clone();
                changed.steps[index] = Step {
                    mode: step.mode,
                    tool: simpler,
                };
                smaller.push(changed);
            }
        }
        smaller.retain(|candidate| candidate != self);
        smaller
    }
}

fn rounded(value: f64, step: f64) -> f64 {
    (value / step).round() * step
}

fn rounded_point(point: DVec2, step: f64) -> DVec2 {
    DVec2::new(rounded(point.x, step), rounded(point.y, step))
}

impl Leaf {
    fn simpler(&self) -> Vec<Leaf> {
        let mut simpler = Vec::new();
        match self {
            Leaf::Prism {
                plane,
                outline,
                height,
            } => {
                for plane in plane.simpler() {
                    simpler.push(Leaf::prism(plane, outline.clone(), *height));
                }
                for outline in outline.simpler() {
                    simpler.push(Leaf::prism(*plane, outline, *height));
                }
                for height in [height.abs(), rounded(*height, 1.0), rounded(*height, 0.5)] {
                    if height != 0.0 {
                        simpler.push(Leaf::prism(*plane, outline.clone(), height));
                    }
                }
            }
            Leaf::Revolution {
                plane,
                low,
                high,
                degrees,
            } => {
                let turned =
                    |plane: Plane, low: DVec2, high: DVec2, degrees: f64| Leaf::Revolution {
                        plane,
                        low,
                        high,
                        degrees,
                    };
                for plane in plane.simpler() {
                    simpler.push(turned(plane, *low, *high, *degrees));
                }
                simpler.push(turned(*plane, *low, *high, 360.0));
                for step in [1.0, 0.5] {
                    simpler.push(turned(
                        *plane,
                        rounded_point(*low, step),
                        rounded_point(*high, step),
                        *degrees,
                    ));
                }
            }
            Leaf::Turned {
                plane,
                axis,
                section,
                degrees,
            } => {
                let turned = Leaf::turned;
                for plane in plane.simpler() {
                    simpler.push(turned(plane, *axis, section.clone(), *degrees));
                }
                for axis in axis.simpler() {
                    simpler.push(turned(*plane, axis, section.clone(), *degrees));
                }
                let quarters = (degrees / 90.0).round();
                let quarter = if quarters == 0.0 {
                    90.0f64.copysign(*degrees)
                } else {
                    quarters * 90.0
                };
                for degrees in [360.0, quarter, degrees.abs()] {
                    simpler.push(turned(*plane, *axis, section.clone(), degrees));
                }
                for section in section.simpler() {
                    simpler.push(turned(*plane, *axis, section, *degrees));
                }
            }
        }
        simpler.retain(|candidate| candidate != self);
        simpler
    }
}

impl Plane {
    fn simpler(&self) -> Vec<Plane> {
        match *self {
            Plane::Tilted { origin, turn } => {
                let mut simpler = vec![Plane::Xy(origin.z)];
                for axis in 0..3 {
                    if turn[axis] != 0.0 {
                        for straight in [0.0, rounded(turn[axis], 15.0), rounded(turn[axis], 45.0)]
                        {
                            let mut straighter = turn;
                            straighter[axis] = straight;
                            simpler.push(Plane::Tilted {
                                origin,
                                turn: straighter,
                            });
                        }
                    }
                }
                let rounded_origin = DVec3::new(
                    rounded(origin.x, 1.0),
                    rounded(origin.y, 1.0),
                    rounded(origin.z, 1.0),
                );
                simpler.push(Plane::Tilted {
                    origin: rounded_origin,
                    turn,
                });
                simpler
            }
            Plane::Xz(offset) | Plane::Yz(offset) => vec![Plane::Xy(offset)],
            Plane::Xy(offset) => vec![Plane::Xy(rounded(offset, 1.0))],
        }
    }
}

impl Outline {
    fn bounds(&self) -> (DVec2, DVec2) {
        match self {
            Outline::Rectangle { low, high } | Outline::Rounded { low, high, .. } => (*low, *high),
            Outline::Slot { from, to, radius } => {
                (from.min(*to) - *radius, from.max(*to) + *radius)
            }
            Outline::Circle { center, radius, .. }
            | Outline::Ring {
                center,
                outer: radius,
                ..
            } => (*center - *radius, *center + *radius),
            Outline::Star { corners, .. } => corners.iter().fold(
                (DVec2::splat(f64::MAX), DVec2::splat(f64::MIN)),
                |(low, high), corner| (low.min(*corner), high.max(*corner)),
            ),
            Outline::Ellipse { .. }
            | Outline::Chorded { .. }
            | Outline::OvalSlot { .. }
            | Outline::OvalRing { .. } => self.oval_box().expect("an outline an ellipse bounds"),
        }
    }

    fn simpler(&self) -> Vec<Outline> {
        let (low, high) = self.bounds();
        let mut simpler = Vec::new();
        match self {
            Outline::Rectangle { low, high } => {
                for step in [1.0, 0.5] {
                    let (low, high) = (rounded_point(*low, step), rounded_point(*high, step));
                    if low.x < high.x && low.y < high.y {
                        simpler.push(Outline::Rectangle { low, high });
                    }
                }
            }
            Outline::Circle {
                center,
                radius,
                from,
            } => {
                simpler.push(Outline::Rectangle { low, high });
                simpler.push(Outline::Circle {
                    center: *center,
                    radius: *radius,
                    from: 0.0,
                });
                for step in [1.0, 0.5] {
                    let radius = rounded(*radius, step);
                    if radius > 0.0 {
                        simpler.push(Outline::Circle {
                            center: rounded_point(*center, step),
                            radius,
                            from: *from,
                        });
                    }
                }
            }
            Outline::Ring {
                center,
                outer,
                inner,
            } => {
                simpler.push(Outline::Circle {
                    center: *center,
                    radius: *outer,
                    from: 0.0,
                });
                simpler.push(Outline::Rectangle { low, high });
                for step in [1.0, 0.5] {
                    let (outer, inner) = (rounded(*outer, step), rounded(*inner, step));
                    if 0.0 < inner && inner < outer {
                        simpler.push(Outline::Ring {
                            center: rounded_point(*center, step),
                            outer,
                            inner,
                        });
                    }
                }
            }
            Outline::Rounded { radius, .. } => {
                simpler.push(Outline::Rectangle { low, high });
                for step in [1.0, 0.5] {
                    let (low, high) = (rounded_point(low, step), rounded_point(high, step));
                    let radius = rounded(*radius, step).min((high - low).min_element() / 2.0);
                    if low.x < high.x && low.y < high.y && radius > 0.0 {
                        simpler.push(Outline::Rounded { low, high, radius });
                    }
                }
            }
            Outline::Slot { from, to, radius } => {
                simpler.push(Outline::Circle {
                    center: *from,
                    radius: *radius,
                    from: 0.0,
                });
                simpler.push(Outline::Rectangle { low, high });
                for step in [1.0, 0.5] {
                    let (from, to) = (rounded_point(*from, step), rounded_point(*to, step));
                    let radius = rounded(*radius, step);
                    if from != to && radius > 0.0 {
                        simpler.push(Outline::Slot { from, to, radius });
                    }
                }
            }
            Outline::Ellipse { .. }
            | Outline::Chorded { .. }
            | Outline::OvalSlot { .. }
            | Outline::OvalRing { .. } => {
                simpler.push(Outline::Rectangle { low, high });
                simpler.extend(self.simpler_oval());
            }
            Outline::Star { center, corners } => {
                simpler.push(Outline::Rectangle { low, high });
                if corners.len() > 3 {
                    for index in 0..corners.len() {
                        let mut fewer = corners.clone();
                        fewer.remove(index);
                        simpler.push(Outline::Star {
                            center: *center,
                            corners: fewer,
                        });
                    }
                }
            }
        }
        simpler
    }
}

/// The direction of the plane's axis nearest `direction`, either way.
fn on_an_axis(direction: DVec2) -> DVec2 {
    if direction.x.abs() >= direction.y.abs() {
        DVec2::X * direction.x.signum()
    } else {
        DVec2::Y * direction.y.signum()
    }
}

/// The same direction, its angle on a step of `degrees`: on a plane's axis
/// exactly when it comes to one, rather than a rounding of the cosine off
/// it, which `on_an_axis` would put back.
fn turned_round(direction: DVec2, degrees: f64) -> DVec2 {
    let turned =
        DVec2::from_angle(rounded(direction.to_angle().to_degrees(), degrees).to_radians());
    let rounding = |value: f64| if value.abs() < 1e-12 { 0.0 } else { value };
    DVec2::new(rounding(turned.x), rounding(turned.y))
}

impl Outline {
    /// What an outline an ellipse bounds shrinks into: a circle, the whole
    /// ellipse without its traits or its hole, its ends round, its axes on
    /// the plane's, its half-axes made one, and every number on round ones.
    fn simpler_oval(&self) -> Vec<Outline> {
        let mut simpler = Vec::new();
        match self {
            Outline::OvalSlot {
                from,
                to,
                across,
                along,
            } => {
                let way = (*to - *from).normalize();
                let length = from.distance(*to);
                simpler.push(Outline::Slot {
                    from: *from,
                    to: *to,
                    radius: *across,
                });
                simpler.push(Outline::Ellipse {
                    center: *from,
                    first: way * *along,
                    second: *across,
                    from: 0.0,
                });
                let slot = |to: DVec2, across: f64, along: f64| Outline::OvalSlot {
                    from: *from,
                    to,
                    across,
                    along,
                };
                simpler.push(slot(*to, *across, *across));
                simpler.push(slot(*from + on_an_axis(way) * length, *across, *along));
                for step in [1.0, 0.5] {
                    let (start, end) = (rounded_point(*from, step), rounded_point(*to, step));
                    let (across, along) = (rounded(*across, step), rounded(*along, step));
                    if start != end && across > 0.0 && along > 0.0 {
                        simpler.push(Outline::OvalSlot {
                            from: start,
                            to: end,
                            across,
                            along,
                        });
                    }
                }
            }
            _ => {
                let Some(oval) = self.oval() else {
                    return simpler;
                };
                let (center, first, second) = (oval.center, oval.first, oval.second);
                let length = first.length();
                simpler.push(Outline::Circle {
                    center,
                    radius: oval.minor(),
                    from: 0.0,
                });
                simpler.push(Outline::Ellipse {
                    center,
                    first,
                    second,
                    from: 0.0,
                });
                let mut reshaped = |first: DVec2, second: f64| {
                    if let Some(outline) = self.with_oval(center, first, second) {
                        simpler.push(outline);
                    }
                };
                reshaped(on_an_axis(first) * length, second);
                reshaped(turned_round(first, 15.0) * length, second);
                reshaped(first, length);
                for step in [1.0, 0.5] {
                    let (length, second) = (rounded(length, step), rounded(second, step));
                    if length > 0.0 && second > 0.0 {
                        let moved = rounded_point(center, step);
                        if let Some(outline) =
                            self.with_oval(moved, first.normalize() * length, second)
                        {
                            simpler.push(outline);
                        }
                    }
                }
                match self {
                    Outline::Chorded { arcs, .. } => {
                        if arcs.len() > 1 {
                            for arc in arcs {
                                simpler.push(Outline::Chorded {
                                    center,
                                    first,
                                    second,
                                    arcs: vec![*arc],
                                });
                            }
                        }
                        for step in [15.0, 45.0] {
                            simpler.push(Outline::Chorded {
                                center,
                                first,
                                second,
                                arcs: arcs
                                    .iter()
                                    .map(|arc| arc.map(|end| rounded(end, step)))
                                    .collect(),
                            });
                        }
                    }
                    Outline::OvalRing { hole, .. } => {
                        let least = hole.min_element();
                        simpler.push(Outline::Ring {
                            center,
                            outer: oval.minor(),
                            inner: least,
                        });
                        simpler.push(Outline::OvalRing {
                            center,
                            first,
                            second,
                            hole: DVec2::splat(least),
                        });
                        for step in [1.0, 0.5] {
                            simpler.push(Outline::OvalRing {
                                center,
                                first,
                                second,
                                hole: rounded_point(*hole, step),
                            });
                        }
                    }
                    Outline::Ellipse { .. } => {}
                    _ => unreachable!("only outlines an ellipse bounds"),
                }
            }
        }
        simpler.retain(|candidate| differs(candidate, self));
        simpler
    }

    /// The same outline drawn on another ellipse: a ring's hole kept, a
    /// chorded ellipse's arcs kept.
    fn with_oval(&self, center: DVec2, first: DVec2, second: f64) -> Option<Outline> {
        match self {
            Outline::Ellipse { from, .. } => Some(Outline::Ellipse {
                center,
                first,
                second,
                from: *from,
            }),
            Outline::Chorded { arcs, .. } => Some(Outline::Chorded {
                center,
                first,
                second,
                arcs: arcs.clone(),
            }),
            Outline::OvalRing { hole, .. } => Some(Outline::OvalRing {
                center,
                first,
                second,
                hole: *hole,
            }),
            _ => None,
        }
    }
}

/// Whether two outlines differ by more than the rounding of the numbers
/// they are drawn with. A semi-axis rebuilt from its direction and its
/// length, or turned onto the step its angle already stands on, moves by a
/// last bit or two; offered as simpler, two such changes would lead back
/// and forth between the same two outlines for ever.
fn differs(one: &Outline, other: &Outline) -> bool {
    let near =
        |one: f64, other: f64| (one - other).abs() <= 1e-12 * one.abs().max(other.abs()).max(1.0);
    let near_pair = |one: DVec2, other: DVec2| near(one.x, other.x) && near(one.y, other.y);
    let same = match (one, other) {
        (
            Outline::Ellipse {
                center,
                first,
                second,
                from,
            },
            Outline::Ellipse {
                center: other_center,
                first: other_first,
                second: other_second,
                from: other_from,
            },
        ) => {
            near_pair(*center, *other_center)
                && near_pair(*first, *other_first)
                && near(*second, *other_second)
                && near(*from, *other_from)
        }
        (
            Outline::Chorded {
                center,
                first,
                second,
                arcs,
            },
            Outline::Chorded {
                center: other_center,
                first: other_first,
                second: other_second,
                arcs: other_arcs,
            },
        ) => {
            near_pair(*center, *other_center)
                && near_pair(*first, *other_first)
                && near(*second, *other_second)
                && arcs.len() == other_arcs.len()
                && arcs
                    .iter()
                    .zip(other_arcs)
                    .all(|(arc, other)| near(arc[0], other[0]) && near(arc[1], other[1]))
        }
        (
            Outline::OvalSlot {
                from,
                to,
                across,
                along,
            },
            Outline::OvalSlot {
                from: other_from,
                to: other_to,
                across: other_across,
                along: other_along,
            },
        ) => {
            near_pair(*from, *other_from)
                && near_pair(*to, *other_to)
                && near(*across, *other_across)
                && near(*along, *other_along)
        }
        (
            Outline::OvalRing {
                center,
                first,
                second,
                hole,
            },
            Outline::OvalRing {
                center: other_center,
                first: other_first,
                second: other_second,
                hole: other_hole,
            },
        ) => {
            near_pair(*center, *other_center)
                && near_pair(*first, *other_first)
                && near(*second, *other_second)
                && near_pair(*hole, *other_hole)
        }
        _ => one == other,
    };
    !same
}

impl Axis {
    /// The sketch's own axis, run forwards and leaning off nothing, and the
    /// line moved onto round numbers.
    fn simpler(&self) -> Vec<Axis> {
        let mut simpler = vec![
            Axis {
                across: 0.0,
                ..*self
            },
            Axis {
                backwards: false,
                ..*self
            },
            Axis { lean: 0.0, ..*self },
        ];
        for step in [1.0, 0.5] {
            simpler.push(Axis {
                across: rounded(self.across, step),
                ..*self
            });
        }
        simpler.retain(|candidate| candidate != self);
        simpler
    }
}

impl Section {
    /// The section with a band levelled, or every band, a band or a hole
    /// fewer, its edges nearest the axis laid on it, gathered into the one
    /// level band that bounds it, and on round numbers. None of them slopes
    /// more bands than the section did, which is what keeps levelling and
    /// the rest from undoing each other.
    fn simpler(&self) -> Vec<Section> {
        let mut simpler = Vec::new();
        let sloping = (0..self.bands.len()).filter(|&index| self.slopes_at(index));
        for index in sloping.clone() {
            let mut level = self.clone();
            let [_, low, high] = level.bands[index];
            level.sloping_to[index] = [low, high];
            simpler.push(level.canonical());
        }
        if sloping.count() > 1 {
            simpler.push(Section {
                sloping_to: Vec::new(),
                ..self.clone()
            });
        }
        if self.bands.len() > 1 {
            for index in 0..self.bands.len() {
                let mut fewer = self.clone();
                fewer.bands.remove(index);
                if fewer.slopes() {
                    fewer.sloping_to.remove(index);
                }
                simpler.push(fewer.canonical());
            }
        }
        for index in 0..self.holes.len() {
            let mut fewer = self.clone();
            fewer.holes.remove(index);
            simpler.push(fewer);
        }
        let mut on_the_axis = self.clone();
        let nearest = match self.side() {
            Some(side) if side > 0.0 => Some(1),
            Some(_) => Some(2),
            None => None,
        };
        if let Some(nearest) = nearest {
            for band in &mut on_the_axis.bands {
                band[nearest] = 0.0;
            }
            for end in &mut on_the_axis.sloping_to {
                end[nearest - 1] = 0.0;
            }
            simpler.push(on_the_axis.canonical());
        }
        if self.bands.len() > 1 || !self.holes.is_empty() {
            let length = self.bands.iter().map(|band| band[0]).sum();
            let edges = |edge: usize| {
                (0..self.bands.len()).flat_map(move |index| {
                    let (start, end) = self.edges(index);
                    [start[edge], end[edge]]
                })
            };
            let low = edges(0).fold(f64::MAX, f64::min);
            let high = edges(1).fold(f64::MIN, f64::max);
            simpler.push(Section::bands(self.from, &[[length, low, high]]));
        }
        for step in [1.0, 0.5] {
            simpler.push(
                Section {
                    from: rounded(self.from, step),
                    bands: self
                        .bands
                        .iter()
                        .map(|band| band.map(|value| rounded(value, step)))
                        .collect(),
                    sloping_to: self
                        .sloping_to
                        .iter()
                        .map(|end| end.map(|value| rounded(value, step)))
                        .collect(),
                    holes: self
                        .holes
                        .iter()
                        .map(|hole| hole.map(|corner| rounded_point(corner, step)))
                        .collect(),
                }
                .canonical(),
            );
        }
        simpler.retain(|candidate| candidate != self);
        simpler
    }
}
