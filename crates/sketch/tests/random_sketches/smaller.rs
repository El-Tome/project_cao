//! The drawings a failing drawing could shrink into.
//!
//! Every change offered here makes a drawing simpler and never undoes
//! another: a gesture dropped, a curve made straight, a corner cut rather than
//! rounded, a copy made fewer times, construction drawn plain, a number
//! rounded. That is what makes the shrinking finish — no two changes lead back
//! to where the first started.

use glam::DVec2;

use super::{Axis, Gesture};

/// The drawings one change simpler than `gestures`.
pub fn smaller(gestures: &[Gesture]) -> Vec<Vec<Gesture>> {
    let mut smaller = Vec::new();
    for index in (0..gestures.len()).rev() {
        let mut fewer = gestures.to_vec();
        fewer.remove(index);
        smaller.push(fewer);
    }
    for (index, gesture) in gestures.iter().enumerate() {
        for simpler in gesture.simpler() {
            let mut changed = gestures.to_vec();
            changed.splice(index..=index, simpler);
            smaller.push(changed);
        }
    }
    smaller.retain(|candidate| candidate.as_slice() != gestures);
    smaller
}

fn rounded(value: f64, step: f64) -> f64 {
    (value / step).round() * step
}

fn rounded_pair(pair: [f64; 2], step: f64) -> [f64; 2] {
    pair.map(|value| rounded(value, step))
}

fn rounded_all(places: &[[f64; 2]], step: f64) -> Vec<[f64; 2]> {
    places.iter().map(|at| rounded_pair(*at, step)).collect()
}

fn fewer(places: &[[f64; 2]]) -> Vec<Vec<[f64; 2]>> {
    if places.len() < 2 {
        return Vec::new();
    }
    (0..places.len())
        .map(|index| {
            let mut fewer = places.to_vec();
            fewer.remove(index);
            fewer
        })
        .collect()
}

impl Axis {
    fn rounded(self, step: f64) -> Axis {
        match self {
            Axis::Trait(at) => Axis::Trait(rounded_pair(at, step)),
            other => other,
        }
    }
}

impl Gesture {
    /// What the gesture could become one step simpler: each as the gestures
    /// standing in its place.
    fn simpler(&self) -> Vec<Vec<Gesture>> {
        let mut simpler: Vec<Vec<Gesture>> =
            self.straighter().into_iter().map(|one| vec![one]).collect();
        simpler.extend(self.fewer().into_iter().map(|one| vec![one]));
        for step in [1.0, 0.5] {
            simpler.push(vec![self.rounded(step)]);
        }
        simpler.retain(|candidate| candidate.as_slice() != std::slice::from_ref(self));
        simpler
    }

    /// The gesture with a curve made straight, a corner cut rather than
    /// rounded, or construction drawn as plain geometry.
    fn straighter(&self) -> Vec<Gesture> {
        let plain = |gesture: &Gesture| -> Option<Gesture> {
            let mut plain = gesture.clone();
            match &mut plain {
                Gesture::Chain { construction, .. }
                | Gesture::Rectangle { construction, .. }
                | Gesture::Circle { construction, .. }
                | Gesture::Arc { construction, .. }
                | Gesture::Ellipse { construction, .. }
                | Gesture::HalfEllipse { construction, .. }
                    if *construction =>
                {
                    *construction = false;
                    Some(plain)
                }
                _ => None,
            }
        };
        let mut straighter: Vec<Gesture> = plain(self).into_iter().collect();
        match self {
            Gesture::Circle {
                centre,
                radius,
                construction,
            } => straighter.push(Gesture::Rectangle {
                corner: (DVec2::from(*centre) - *radius).to_array(),
                opposite: (DVec2::from(*centre) + *radius).to_array(),
                construction: *construction,
            }),
            Gesture::Arc {
                centre,
                start,
                degrees,
                construction,
            } => {
                let (centre, start) = (DVec2::from(*centre), DVec2::from(*start));
                let end = centre + DVec2::from_angle(degrees.to_radians()).rotate(start - centre);
                straighter.push(Gesture::Chain {
                    through: vec![start.to_array(), end.to_array()],
                    closed: false,
                    construction: *construction,
                });
            }
            Gesture::Ellipse {
                centre,
                reach,
                across,
                construction,
            } => {
                let centre = DVec2::from(*centre);
                let first = DVec2::from(*reach) - centre;
                let second = first.perp().normalize_or_zero() * *across;
                let half = DVec2::new(first.x.hypot(second.x), first.y.hypot(second.y));
                straighter.push(Gesture::Rectangle {
                    corner: (centre - half).to_array(),
                    opposite: (centre + half).to_array(),
                    construction: *construction,
                });
            }
            Gesture::HalfEllipse {
                from,
                to,
                construction,
                ..
            } => straighter.push(Gesture::Chain {
                through: vec![*from, *to],
                closed: false,
                construction: *construction,
            }),
            Gesture::Fillet { at, radius } => straighter.push(Gesture::Chamfer {
                at: *at,
                length: *radius,
            }),
            _ => {}
        }
        straighter
    }

    /// The gesture with a place fewer, or a copy made fewer times.
    fn fewer(&self) -> Vec<Gesture> {
        match self {
            Gesture::Chain {
                through,
                closed,
                construction,
            } => {
                let mut fewer: Vec<Gesture> = fewer(through)
                    .into_iter()
                    .filter(|through| through.len() >= 2)
                    .map(|through| Gesture::Chain {
                        through,
                        closed: *closed,
                        construction: *construction,
                    })
                    .collect();
                if *closed {
                    fewer.push(Gesture::Chain {
                        through: through.clone(),
                        closed: false,
                        construction: *construction,
                    });
                }
                fewer
            }
            Gesture::Mirror { of, axis } => fewer(of)
                .into_iter()
                .map(|of| Gesture::Mirror { of, axis: *axis })
                .collect(),
            Gesture::PatternAround {
                of,
                centre,
                degrees,
                count,
            } => {
                let mut fewer: Vec<Gesture> = fewer(of)
                    .into_iter()
                    .map(|of| Gesture::PatternAround {
                        of,
                        centre: *centre,
                        degrees: *degrees,
                        count: *count,
                    })
                    .collect();
                if *count > 2 {
                    fewer.push(Gesture::PatternAround {
                        of: of.clone(),
                        centre: *centre,
                        degrees: *degrees,
                        count: count - 1,
                    });
                }
                fewer
            }
            Gesture::PatternAlong {
                of,
                axis,
                along,
                across,
            } => {
                let mut fewer: Vec<Gesture> = fewer(of)
                    .into_iter()
                    .map(|of| Gesture::PatternAlong {
                        of,
                        axis: *axis,
                        along: *along,
                        across: *across,
                    })
                    .collect();
                if along.1 > 2 {
                    fewer.push(Gesture::PatternAlong {
                        of: of.clone(),
                        axis: *axis,
                        along: (along.0, along.1 - 1),
                        across: *across,
                    });
                }
                if across.1 > 1 {
                    fewer.push(Gesture::PatternAlong {
                        of: of.clone(),
                        axis: *axis,
                        along: *along,
                        across: (across.0, 1),
                    });
                }
                fewer
            }
            _ => Vec::new(),
        }
    }

    /// The gesture with every number it holds rounded to a multiple of `step`.
    fn rounded(&self, step: f64) -> Gesture {
        let pair = |at: &[f64; 2]| rounded_pair(*at, step);
        let number = |value: &f64| rounded(*value, step);
        match self {
            Gesture::Chain {
                through,
                closed,
                construction,
            } => Gesture::Chain {
                through: rounded_all(through, step),
                closed: *closed,
                construction: *construction,
            },
            Gesture::Rectangle {
                corner,
                opposite,
                construction,
            } => Gesture::Rectangle {
                corner: pair(corner),
                opposite: pair(opposite),
                construction: *construction,
            },
            Gesture::Circle {
                centre,
                radius,
                construction,
            } => Gesture::Circle {
                centre: pair(centre),
                radius: number(radius),
                construction: *construction,
            },
            Gesture::Arc {
                centre,
                start,
                degrees,
                construction,
            } => Gesture::Arc {
                centre: pair(centre),
                start: pair(start),
                degrees: *degrees,
                construction: *construction,
            },
            Gesture::Ellipse {
                centre,
                reach,
                across,
                construction,
            } => Gesture::Ellipse {
                centre: pair(centre),
                reach: pair(reach),
                across: number(across),
                construction: *construction,
            },
            Gesture::HalfEllipse {
                from,
                to,
                rise,
                construction,
            } => Gesture::HalfEllipse {
                from: pair(from),
                to: pair(to),
                rise: number(rise),
                construction: *construction,
            },
            Gesture::Point { at } => Gesture::Point { at: pair(at) },
            Gesture::Fillet { at, radius } => Gesture::Fillet {
                at: pair(at),
                radius: number(radius),
            },
            Gesture::Chamfer { at, length } => Gesture::Chamfer {
                at: pair(at),
                length: number(length),
            },
            Gesture::Mirror { of, axis } => Gesture::Mirror {
                of: rounded_all(of, step),
                axis: axis.rounded(step),
            },
            Gesture::PatternAround {
                of,
                centre,
                degrees,
                count,
            } => Gesture::PatternAround {
                of: rounded_all(of, step),
                centre: pair(centre),
                degrees: *degrees,
                count: *count,
            },
            Gesture::PatternAlong {
                of,
                axis,
                along,
                across,
            } => Gesture::PatternAlong {
                of: rounded_all(of, step),
                axis: axis.rounded(step),
                along: (number(&along.0), along.1),
                across: (number(&across.0), across.1),
            },
            Gesture::Divide { at } => Gesture::Divide { at: pair(at) },
            Gesture::Trim { at } => Gesture::Trim { at: pair(at) },
            Gesture::Erase { at } => Gesture::Erase { at: pair(at) },
        }
    }
}
