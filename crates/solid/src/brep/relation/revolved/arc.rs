//! A sphere or a torus against a surface of its axis (#576, decision 9): its
//! meridian circle, about `(h_c, R)` in the half-plane `(h, ρ)` and of
//! radius `r`, against a line of the half-plane — a plane square to the
//! axis, a cylinder, a cone — or against another circle, decided once where
//! they cross. A circle about the axis at each crossing off it, the point of
//! the axis where they meet on it, nothing where they cross on the mirror
//! lobe, at `ρ < 0`, which is not the surface. A touch is decided on the
//! signed gap between the circle and the line, or between the two circles,
//! against the tolerance before any root is taken, and built as the one
//! exact circle at the foot of the centre: no pair with a round grazes, and
//! none is moved onto a touch here — decision 12 moves one standing a hair
//! further off before (#626). Each root is written as a product, never as
//! the difference of two squares, so that a crossing a hair from a touch is
//! found where it stands. A plane keeps its height and a cylinder its
//! radius to the bit, so that the circle is the one either shares with every
//! other surface there. The crossings of a meridian circle with a coaxial
//! surface (decision 4, `crossing/round.rs`) are read off the same standing,
//! so that a corner is decided as the pair it lies on was.

use glam::DVec2;

use super::{Frame, Meridian, in_the_box};
use crate::brep::relation::{Relation, Rim};
use crate::brep::scale::Scale;

/// Where a circle of the half-plane stands against another curve of it.
pub(in crate::brep::relation) enum Met {
    /// One circle within the tolerance of the other all round.
    Same,
    /// Two circles about one centre within the tolerance, their radii a
    /// hair past it: within the tolerance of each other all round, with no
    /// direction to decide a touch along, as two parallel meridian lines.
    Unsupported,
    /// The foot of the circle's centre on the other curve, where the two
    /// touch if they do; the direction the two crossings part from it along;
    /// how far the circle stands from touching the curve, negative where the
    /// two cross; and half the distance between the crossings.
    Near {
        foot: DVec2,
        along: DVec2,
        gap: f64,
        half: f64,
    },
}

/// Where the circle about `center` of `radius` stands against `other`, both
/// read in one half-plane: against a line, at the foot of the centre; against
/// a circle, on the line of the two centres.
pub(in crate::brep::relation) fn met_in_the_half_plane(
    center: DVec2,
    radius: f64,
    other: &Meridian,
    eps: f64,
) -> Met {
    let (foot, along, away) = match *other {
        Meridian::Level(height) => (DVec2::new(height, center.y), DVec2::Y, height - center.x),
        Meridian::Wall(rho) => (DVec2::new(center.x, rho), DVec2::X, rho - center.y),
        Meridian::Nappe { foot, ruling } => {
            let normal = ruling.perp();
            let away = normal.dot(center - foot);
            (center - normal * away, ruling, away)
        }
        Meridian::Arc {
            center: theirs,
            radius: r,
        } => return circles((center, radius), (theirs, r), eps),
    };
    let away = away.abs();
    Met::Near {
        foot,
        along,
        gap: away - radius,
        half: ((radius - away) * (radius + away)).max(0.0).sqrt(),
    }
}

/// Two circles: one where their centres and radii stand within the
/// tolerance; unsupported where their centres do and their radii a hair past
/// it; else how they stand along the line of their centres, the gap the
/// larger of the gap to touching outside and the gap to touching inside, the
/// crossings at Heron's distance either side of the foot.
fn circles((one, r1): (DVec2, f64), (other, r2): (DVec2, f64), eps: f64) -> Met {
    let between = other - one;
    let d = between.length();
    let unequal = (r1 - r2).abs();
    if d <= eps {
        if unequal <= eps {
            return Met::Same;
        }
        if unequal - d <= eps {
            return Met::Unsupported;
        }
        return Met::Near {
            foot: one,
            along: DVec2::X,
            gap: unequal - d,
            half: 0.0,
        };
    }
    let e = between / d;
    let product = (r1 + r2 + d) * (r1 + r2 - d) * (d + r1 - r2) * (d - r1 + r2);
    Met::Near {
        foot: one + e * ((d * d + r1 * r1 - r2 * r2) / (2.0 * d)),
        along: e.perp(),
        gap: (d - (r1 + r2)).max(unequal - d),
        half: product.max(0.0).sqrt() / (2.0 * d),
    }
}

/// The relation of the round whose meridian circle is about `center` with
/// `radius`, and the surface `other` is the meridian of, both read in
/// `frame`.
pub(in crate::brep::relation) fn met(
    frame: &Frame,
    center: DVec2,
    radius: f64,
    other: Meridian,
    scale: Scale,
) -> Relation {
    let eps = scale.eps();
    match met_in_the_half_plane(center, radius, &other, eps) {
        Met::Same => Relation::Same { agree: true },
        Met::Unsupported => Relation::Unsupported,
        Met::Near { gap, .. } if gap > eps => Relation::Apart,
        Met::Near { foot, gap, .. } if gap >= -eps => rims(frame, [foot], true, scale),
        Met::Near {
            foot, along, half, ..
        } => rims(
            frame,
            [foot - along * half, foot + along * half],
            false,
            scale,
        ),
    }
}

/// What the crossings `found` in the half-plane make: a circle about the
/// axis at each that stands off it, in order of height then radius; else
/// the point of the axis one stands on within the tolerance, where it is in
/// the box; else apart. A crossing at `ρ < 0` is on the mirror lobe and
/// counts for nothing.
fn rims(
    frame: &Frame,
    found: impl IntoIterator<Item = DVec2>,
    touching: bool,
    scale: Scale,
) -> Relation {
    let eps = scale.eps();
    let mut circles: Vec<DVec2> = Vec::new();
    let mut on_the_axis = None;
    for at in found {
        if at.y > eps {
            circles.push(at);
        } else if at.y >= -eps && on_the_axis.is_none() {
            on_the_axis = Some(at.x);
        }
    }
    if !circles.is_empty() {
        circles.sort_by(|one, other| one.x.total_cmp(&other.x).then(one.y.total_cmp(&other.y)));
        return Relation::Rims(
            circles
                .into_iter()
                .map(|at| Rim {
                    circle: frame.circle(at.x, at.y),
                    touching,
                })
                .collect(),
        );
    }
    match on_the_axis {
        Some(h) if in_the_box(frame.point(h), scale) => Relation::Apex(frame.point(h)),
        Some(_) | None => Relation::Apart,
    }
}
