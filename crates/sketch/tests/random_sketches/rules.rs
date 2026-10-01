//! What the areas of every drawing must satisfy, whatever is drawn.
//!
//! Nobody can write down the areas of a random drawing, so they have no
//! expected answer to be held against. What can be written down is what any
//! answer must keep: its tint covers what its outline encloses, every closed
//! place lies in one area and only one, nothing lies in an area where nothing
//! closes, and the same drawing laid again another way gives the same areas.
//!
//! Every rule here reads plain data — outlines, holes, triangles, places — and
//! never a `Sketch`, so that each can be caught out by a list of areas broken
//! on purpose. Whether a place is enclosed arrives already decided, by a
//! reckoning that does not walk the drawing the way the code under test does:
//! a rule checked by the code it checks would agree with every bug.

use std::time::Duration;

use glam::DVec2;

/// One area of a drawing, as the drawing hands it to whoever tints it.
#[derive(Clone, Debug, PartialEq)]
pub struct Area {
    /// The loop the area is walked round, as the steps it was sampled into.
    pub outline: Vec<DVec2>,
    /// The loops drawn directly inside it, which it is hollow of.
    pub holes: Vec<Vec<DVec2>>,
    /// What it is tinted with: the whole of its outline, holes included, so
    /// that a shape drawn inside another is tinted twice (`docs/sketch.md`,
    /// *Closed surfaces*).
    pub triangles: Vec<[DVec2; 3]>,
    /// What its outline encloses, read on the curves themselves rather than
    /// on the steps they were sampled into: `Region::area()` with the holes
    /// added back.
    pub measure: f64,
    /// How far the tint may stand from that measure because its curves were
    /// tinted as straight steps.
    pub sampling: f64,
}

/// A place of the drawing, and whether something closes around it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Place {
    pub at: DVec2,
    pub enclosed: bool,
}

/// How a drawing was laid again to be held against itself.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Relaying {
    /// The same points and curves added in another order, each trait drawn
    /// from its other end.
    Reordered,
    /// Every place moved by the same step.
    Moved(DVec2),
    /// Every place turned about the origin, by an angle in degrees.
    Turned(f64),
}

/// Which rule a drawing broke: what a shrunk case has to go on breaking to
/// still be the same failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rule {
    TintIsMeasure,
    NothingMissing,
    NothingExtra,
    OrderDoesNotMatter,
    Answers,
}

/// A rule a drawing broke, and where — enough for somebody to go and look.
#[derive(Clone, Debug, PartialEq)]
pub enum Flaw {
    /// The triangles of the area whose outline passes through `near` cover
    /// more or less than that outline encloses.
    Tint {
        near: DVec2,
        tinted: f64,
        measured: f64,
    },
    /// A place something closes around lies in no area.
    Missing { at: DVec2 },
    /// A place lies in more than one area, or in one while nothing closes
    /// around it.
    Extra {
        at: DVec2,
        areas: usize,
        enclosed: bool,
    },
    /// Laid again another way, the same drawing gives another count of areas,
    /// or puts `at` in an area of another size.
    Relaid {
        how: Relaying,
        areas: (usize, usize),
        at: Option<DVec2>,
    },
    /// Finding the areas gave no answer at all.
    NoAnswer(Silence),
}

/// How the drawing failed to answer.
#[derive(Clone, Debug, PartialEq)]
pub enum Silence {
    /// It panicked, with the message the panic carried if it carried one.
    Panicked(Option<String>),
    /// It was still working when the patience given ran out.
    Late(Duration),
}

impl Flaw {
    pub fn rule(&self) -> Rule {
        match self {
            Flaw::Tint { .. } => Rule::TintIsMeasure,
            Flaw::Missing { .. } => Rule::NothingMissing,
            Flaw::Extra { .. } => Rule::NothingExtra,
            Flaw::Relaid { .. } => Rule::OrderDoesNotMatter,
            Flaw::NoAnswer(_) => Rule::Answers,
        }
    }

    /// Whether two flaws are the same failure: the same rule broken, and for
    /// a drawing that gave no answer, the same kind of silence. A panic that
    /// shrinks into a drawing that never comes back is two bugs, and would be
    /// reported as neither.
    pub fn is_like(&self, other: &Flaw) -> bool {
        match (self, other) {
            (Flaw::NoAnswer(one), Flaw::NoAnswer(other)) => {
                std::mem::discriminant(one) == std::mem::discriminant(other)
            }
            _ => self.rule() == other.rule(),
        }
    }
}

/// How far two measures read off the same curves may differ: the rounding
/// of sums of products, against the square of how far the drawing reaches.
const ROUNDING: f64 = 1e-9;

/// How far the size of one area may move when its drawing is laid again, as a
/// share of how far the drawing reaches: the walk welds a point onto a curve
/// within a distance that grows with how far out the point stands
/// (`off_by`, `crates/sketch/src/edges.rs`), so a point standing near that
/// distance from a curve is welded in one laying and not in the other, and
/// the run it ends is sampled from a place that far off the curve.
const WELDED: f64 = 1e-6;

/// The triangles of every area cover what its outline encloses, give or take
/// what the sampling of its curves allows.
pub fn tint_is_measure(areas: &[Area]) -> Result<(), Flaw> {
    for area in areas {
        let tinted: f64 = area.triangles.iter().map(surface).sum();
        let reach = reach(&area.outline);
        if (tinted - area.measure).abs() > area.sampling + ROUNDING * reach * reach {
            return Err(Flaw::Tint {
                near: lowest(&area.outline),
                tinted,
                measured: area.measure,
            });
        }
    }
    Ok(())
}

/// Every place something closes around lies in an area.
pub fn nothing_missing(areas: &[Area], places: &[Place]) -> Result<(), Flaw> {
    match places
        .iter()
        .find(|place| place.enclosed && holding(areas, place.at).is_empty())
    {
        Some(place) => Err(Flaw::Missing { at: place.at }),
        None => Ok(()),
    }
}

/// No place lies in two areas, nor in one where nothing closes around it.
pub fn nothing_extra(areas: &[Area], places: &[Place]) -> Result<(), Flaw> {
    for place in places {
        let count = holding(areas, place.at).len();
        if count > 1 || (count == 1 && !place.enclosed) {
            return Err(Flaw::Extra {
                at: place.at,
                areas: count,
                enclosed: place.enclosed,
            });
        }
    }
    Ok(())
}

/// The areas of a drawing laid again are the areas it had: as many, and every
/// place — carried along by `carried` — in an area of the same size.
pub fn laid_alike(
    how: Relaying,
    first: &[Area],
    second: &[Area],
    places: &[DVec2],
    carried: impl Fn(DVec2) -> DVec2,
) -> Result<(), Flaw> {
    let areas = (first.len(), second.len());
    if areas.0 != areas.1 {
        return Err(Flaw::Relaid {
            how,
            areas,
            at: None,
        });
    }
    let reach = first
        .iter()
        .map(|area| reach(&area.outline))
        .fold(1.0, f64::max);
    let alike = |one: &[f64], other: &[f64]| {
        one.len() == other.len()
            && one
                .iter()
                .zip(other)
                .all(|(one, other)| (one - other).abs() <= WELDED * (1.0 + reach) * reach)
    };
    let sizes = |areas: &[Area]| {
        let mut sizes: Vec<f64> = areas.iter().map(|area| area.measure).collect();
        sizes.sort_by(f64::total_cmp);
        sizes
    };
    if !alike(&sizes(first), &sizes(second)) {
        return Err(Flaw::Relaid {
            how,
            areas,
            at: None,
        });
    }
    let held = |areas: &[Area], at: DVec2| {
        let mut sizes: Vec<f64> = holding(areas, at)
            .into_iter()
            .map(|area| area.measure)
            .collect();
        sizes.sort_by(f64::total_cmp);
        sizes
    };
    match places
        .iter()
        .find(|at| !alike(&held(first, **at), &held(second, carried(**at))))
    {
        Some(at) => Err(Flaw::Relaid {
            how,
            areas,
            at: Some(*at),
        }),
        None => Ok(()),
    }
}

/// The areas a place lies in: in their tint, and in none of their holes.
pub fn holding(areas: &[Area], at: DVec2) -> Vec<&Area> {
    areas
        .iter()
        .filter(|area| {
            area.triangles
                .iter()
                .any(|triangle| in_triangle(at, triangle))
                && !area.holes.iter().any(|hole| in_loop(at, hole))
        })
        .collect()
}

pub fn surface([a, b, c]: &[DVec2; 3]) -> f64 {
    (*b - *a).perp_dot(*c - *a).abs() / 2.0
}

/// How far outside a triangle a place may fall and still be in it, as a
/// share of the triangle itself: a place on the seam two triangles share is in
/// both, whichever side of it the rounding put it on.
const SEAM: f64 = 1e-9;

/// Under this surface, against the square of how far it reaches, a triangle
/// is a sliver laid along a line, and holds no place.
const SLIVER: f64 = 1e-12;

/// Whether a place lies in a triangle, read by how much of the triangle each
/// side leaves on the place's side of it: all three shares are positive
/// inside, whichever way round the triangle is wound.
fn in_triangle(at: DVec2, triangle: &[DVec2; 3]) -> bool {
    let [a, b, c] = *triangle;
    let whole = (b - a).perp_dot(c - a);
    let reach = reach(triangle);
    if whole.abs() <= SLIVER * reach * reach {
        return false;
    }
    [(b, c), (c, a), (a, b)]
        .iter()
        .all(|(from, to)| (*from - at).perp_dot(*to - at) / whole >= -SEAM)
}

/// Whether a place lies inside a closed run of places, by the parity of the
/// sides a ray from it crosses.
pub fn in_loop(at: DVec2, places: &[DVec2]) -> bool {
    let mut inside = false;
    for index in 0..places.len() {
        let (a, b) = (places[index], places[(index + 1) % places.len()]);
        if (a.y > at.y) != (b.y > at.y) {
            let crossing = a.x + (at.y - a.y) / (b.y - a.y) * (b.x - a.x);
            if crossing > at.x {
                inside = !inside;
            }
        }
    }
    inside
}

/// How far a run of places reaches from the origin, never less than one: the
/// scale a tolerance is taken against.
pub fn reach(places: &[DVec2]) -> f64 {
    places
        .iter()
        .fold(1.0f64, |far, place| far.max(place.abs().max_element()))
}

fn lowest(places: &[DVec2]) -> DVec2 {
    places
        .iter()
        .copied()
        .min_by(|a, b| a.y.total_cmp(&b.y).then(a.x.total_cmp(&b.x)))
        .unwrap_or_default()
}
