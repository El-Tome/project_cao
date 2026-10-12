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
                        let mut straighter = turn;
                        straighter[axis] = 0.0;
                        simpler.push(Plane::Tilted {
                            origin,
                            turn: straighter,
                        });
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
