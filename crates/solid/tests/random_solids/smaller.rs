//! The cases a failing case could shrink into.
//!
//! Every change offered here makes a case simpler and never undoes another:
//! a step removed, a curve made straight, a plane put back square to the axes,
//! a number rounded. That is what makes the shrinking finish — no two changes
//! lead back to where the first started.

use glam::{DVec2, DVec3};

use super::{Case, Leaf, Mode, Outline, Plane, Step};

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
            Outline::Rectangle { low, high } => (*low, *high),
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
            Outline::Ring { center, outer, .. } => {
                simpler.push(Outline::Circle {
                    center: *center,
                    radius: *outer,
                    from: 0.0,
                });
                simpler.push(Outline::Rectangle { low, high });
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
