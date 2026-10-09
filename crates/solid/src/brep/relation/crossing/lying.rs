//! Where a line or a circle crosses a surface that one of the surfaces it
//! lies on was decided to cut, or touch, along lines parallel to an axis — a
//! plane read along a wall's axis, a wall parallel to the other (decision 4).
//!
//! The triple is solved from that pair: the corner is where those very lines
//! cross the curve's other plane, the call the lines make themselves, so
//! both curves give it alike. A plane leaning by less than the angle read
//! as along the axis is cut along lines drawn through the foot of the
//! wall's own origin, and stands a fraction of the tolerance off them where
//! the corner is; a hair inside the wall's ruling, that fraction moves where
//! a line in the plane crosses the wall by the tolerance over the slope the
//! wall meets the plane at — a hundred tolerances aside of the lines, two
//! corners where there is one (5520135751).
//!
//! So too for a plane at a slant decided through a cone's apex (#552): it
//! holds the two rulings it cuts, or touches the cone along one, and a rim
//! of the cone crosses it where those rulings cross the rim's plane. Crossed
//! on its own, a rim a third of a millimetre wide of the ruling a plane a
//! hair past the apex was decided to touch along crosses the plane twice
//! that far apart (5541100672).
//!
//! A line or a circle of a triple of two planes and a wall, where the line
//! the planes share grazes the wall, is crossed as `grazed.rs` decides.

use glam::DVec3;

use super::super::{Relation, at_a_slant};
use super::grazed::grazed;
use super::{Crossing, Crossings, Solved, Touches, crossings_given, line_and_plane};
use crate::brep::curve::{Curve, Line};
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Surface};

/// How steeply, in the cosine of their angle, lines parallel to an axis
/// must cross a plane for the place they cross it at to be fixed by the
/// two. Shallower, the place moves along them by the tolerance over that
/// cosine: a plane a thousandth of a degree off a wall's axis crosses the
/// rulings a second plane, a hair off the axis, cuts the wall along a
/// thousandth of the reach from where the line the planes share crosses
/// the wall, which a wall crossed square holds within rounding
/// (5524010446).
pub(super) const STEEP: f64 = 0.1;

/// A surface a curve lies on, with what it was decided to make with the
/// surface the curve crosses.
pub struct Lying<'a> {
    pub surface: &'a Surface,
    pub with: Relation,
}

/// Where a curve crosses `surface`, given the surfaces it lies on and what
/// each was decided to make with it, and what they were decided to touch.
pub fn crossings_lying(
    curve: &Curve,
    lying: &[Lying],
    surface: &Surface,
    scale: Scale,
    touching: &Touches,
) -> Crossings {
    through_lines(curve, lying, surface, scale).unwrap_or_else(|| {
        let found = crossings_given(curve, surface, scale, touching);
        let (Some((planes, wall)), Crossings::At(list)) = (triple(lying, surface), &found) else {
            return found;
        };
        match grazed(planes, wall, list, scale) {
            Some(decided) => on_the_curve(curve, decided, scale),
            None => found,
        }
    })
}

/// The two planes and the wall a curve lying on two surfaces makes a triple
/// of with `surface`, where it is one.
fn triple<'a>(lying: &[Lying<'a>], surface: &'a Surface) -> Option<([&'a Plane; 2], &'a Surface)> {
    let [one, other] = lying else {
        return None;
    };
    match (one.surface, other.surface, surface) {
        (Surface::Plane(first), Surface::Plane(second), wall)
        | (Surface::Plane(first), wall, Surface::Plane(second))
        | (wall, Surface::Plane(first), Surface::Plane(second)) => {
            (!matches!(wall, Surface::Plane(_))).then_some(([first, second], wall))
        }
        _ => None,
    }
}

/// Points decided for a curve, as its crossings: each kept where it stands
/// within the tolerance of the curve, sorted along it.
fn on_the_curve(curve: &Curve, decided: Vec<(DVec3, bool)>, scale: Scale) -> Crossings {
    let mut kept: Vec<Crossing> = decided
        .into_iter()
        .filter_map(|(point, tangent)| {
            let parameter = curve.parameter(point);
            (curve.point(parameter).distance(point) <= scale.eps()).then_some(Crossing {
                parameter,
                point,
                tangent,
            })
        })
        .collect();
    kept.sort_by(|one, other| one.parameter.total_cmp(&other.parameter));
    Crossings::At(kept)
}

/// Whether a plane read along a wall's axis, or a wall read parallel to
/// another, leans off it by more than rounding across the box: only then do
/// the lines the pair is cut along stand off the surfaces where a corner is,
/// and only then is a corner found off them. Along it, every curve finds the
/// corner within rounding, and a corner moved by rounding may be parted
/// from its twin where two planes cross at a grazing angle nearby
/// (5523010597). A plane through a cone's apex leans off its rulings where
/// it stands at a slant to it, and holds them where it holds the axis.
pub(super) fn leans(one: &Surface, other: &Surface, scale: Scale) -> bool {
    let lean = match (one, other) {
        (Surface::Plane(_), Surface::Cone(_)) | (Surface::Cone(_), Surface::Plane(_)) => {
            return at_a_slant(one, other, scale);
        }
        (Surface::Plane(plane), Surface::Cylinder(cylinder))
        | (Surface::Cylinder(cylinder), Surface::Plane(plane)) => {
            plane.normal.dot(cylinder.axis).abs()
        }
        (Surface::Cylinder(one), Surface::Cylinder(other)) => one.axis.cross(other.axis).length(),
        _ => return false,
    };
    lean * 2.0 * scale.reach() > scale.eps() * ROUNDING
}

/// Under this share of the tolerance, a lean across the box is rounding.
const ROUNDING: f64 = 1e-3;

/// The crossings through the lines a surface the curve lies on makes with
/// the one it crosses, each kept where it stands within the tolerance of the
/// curve; none where no such lines cross a plane the curve lies on, as those
/// along a line that runs along them do not. Of the curve's other planes,
/// the lines cross the one they cross most steeply: a line may lie on a
/// plane leaning a hair off them too, which they cross a billion reaches
/// away (5524005772).
fn through_lines(
    curve: &Curve,
    lying: &[Lying],
    surface: &Surface,
    scale: Scale,
) -> Option<Crossings> {
    if !matches!(curve, Curve::Line(_) | Curve::Circle(_)) {
        return None;
    }
    lying.iter().enumerate().find_map(|(index, one)| {
        let (lines, touch): (Vec<Line>, bool) = match &one.with {
            Relation::Lines(lines) | Relation::Rulings { lines, .. } => (lines.to_vec(), false),
            Relation::Tangent(line) | Relation::Ruling { line, .. } => (vec![*line], true),
            _ => return None,
        };
        if !leans(one.surface, surface, scale) {
            return None;
        }
        let steepness = |plane: &Plane| lines[0].direction.dot(plane.normal).abs();
        let across = lying
            .iter()
            .enumerate()
            .filter_map(|(other, own)| match own.surface {
                Surface::Plane(plane) if other != index => Some(plane),
                _ => None,
            })
            .max_by(|one, other| steepness(one).total_cmp(&steepness(other)))
            .filter(|plane| steepness(plane) >= STEEP)?;
        let mut kept = Vec::new();
        for line in lines {
            let Solved::At(found) = line_and_plane(&line, across, scale) else {
                continue;
            };
            for (at, _) in found {
                let point = line.point(at);
                let parameter = curve.parameter(point);
                if curve.point(parameter).distance(point) <= scale.eps() {
                    kept.push(Crossing {
                        parameter,
                        point,
                        tangent: touch,
                    });
                }
            }
        }
        kept.sort_by(|one, other| one.parameter.total_cmp(&other.parameter));
        Some(Crossings::At(kept))
    })
}
