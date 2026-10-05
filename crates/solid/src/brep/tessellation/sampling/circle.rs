//! Where a circle's edge is sampled between its ends.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::super::contact::{self, Contact, Gap};
use super::ends::Ends;
use super::planes::on_a_plane;
use super::{TOLD, divisions};
use crate::brep::curve::Circle;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::topology::Edge;

/// The points of a circle's edge between its ends, in the way the edge runs:
/// on the grid but for the steps its contact withholds, and along the rays it
/// adds. A place closer than `eps` to a grid angle is that angle, and to an
/// end that end — or, at an end a plane touches the wall at, a place so near
/// it that the circle there stands within a fifth of `eps` of the plane: the
/// sample would lie on the plane's edge. Nor is a ray taken, on any circle of
/// the wall, where the circle stands so near such a plane but not on the
/// line it touches along: a ray a partner passed on, through its own grid,
/// would lay a strip of the wall on the plane's face. So too beside a line
/// the wall touches or crosses another along: a ray is not taken where the
/// circle stands within the pair's room of the other wall a step or less
/// from an end standing so too. The vertex is the line's sample there, and a
/// second a hair round from it — through the other end of a line the kernel
/// laid leaning a hair — would stand as good as on the other wall's circle,
/// which ends at the same vertex. So too beside any surface an end lies on
/// and the circle does not, where the circle all but lies on it a step or
/// less from that end: a wall grazing the plane of the circle there. All
/// but lying on it is read at the place and halfway back to the end: a
/// circle crossing the surface again a step from the end stands on the line
/// of that second crossing, off the surface's face, and keeps the ray its
/// partner in contact takes too.
pub(super) fn on_circle(
    circle: &Circle,
    edge: &Edge,
    tolerance: f64,
    eps: f64,
    contact: &Contact,
    ends_of: &Ends,
    planes: &[Plane],
) -> Vec<DVec3> {
    let steps = divisions(circle.radius, tolerance);
    let step = TAU / steps as f64;
    let gap = eps / circle.radius;
    let level = circle.center.dot(circle.axis);
    let beside_end = gap.max((2.0 * eps * contact::APART / circle.radius).sqrt());
    let [at_from, at_to] = ends_of
        .touched
        .map(|touched| if touched { beside_end } else { gap });
    let whole = edge.ends.is_none();
    let (low, high) = if whole {
        (edge.from, edge.from + TAU)
    } else if edge.from <= edge.to {
        (edge.from + at_from, edge.to - at_to)
    } else {
        (edge.to + at_to, edge.from - at_from)
    };
    let inside = |at: f64| {
        if whole {
            low <= at && at < high
        } else {
            low < at && at < high
        }
    };
    let ends = if whole {
        Vec::new()
    } else {
        vec![edge.from, edge.to]
    };
    let beside = |at: f64, (own, gap): &(Cylinder, Gap)| gap.crowds(own, circle.point(at));
    let beside_a_plane = |at: f64| {
        let point = circle.point(at);
        planes
            .iter()
            .any(|plane| on_a_plane(plane, circle.center, circle.axis, point, eps))
    };
    let grazed = |at: f64, room: f64, closing: bool| {
        ends.iter().enumerate().any(|(side, end)| {
            let lies =
                |surface: &Surface, place: f64| surface.distance(circle.point(place)).abs() < room;
            (end - at).abs() <= step
                && ends_of.through[side].iter().any(|surface| {
                    lies(surface, at)
                        && lies(surface, (end + at) / 2.0)
                        && (!closing || ends_of.grazes(side, surface, circle.axis, eps))
                })
        })
    };
    let by_an_end = |at: f64| {
        grazed(at, eps * contact::APART, true)
            || contact.beside.iter().any(|other| {
                let partner = other.1.other(&other.0);
                beside(at, other)
                    && ends.iter().enumerate().any(|(side, end)| {
                        (end - at).abs() <= step
                            && beside(*end, other)
                            && ends_of.closes(side, &partner, eps)
                    })
            })
    };

    let on_the_turn = |at: f64| {
        let round = low + (at - low).rem_euclid(TAU);
        if round < high { round } else { low }
    };
    let at_rank = |rank: i64| {
        let at = rank as f64 * step;
        if whole { on_the_turn(at) } else { at }
    };
    let ranks = if whole {
        0..=steps as i64 - 1
    } else {
        (low / step).floor() as i64..=(high / step).ceil() as i64
    };
    let mut places: Vec<(f64, bool, f64)> = ranks
        .filter(|rank| inside(at_rank(*rank)) && !grazed(at_rank(*rank), eps * TOLD, false))
        .filter(|rank| {
            let step = rank.rem_euclid(steps as i64) as usize;
            !contact.withheld.iter().any(|(withheld, [low, high])| {
                *withheld == step && *low - eps <= level && level <= *high + eps
            })
        })
        .map(|rank| {
            let angle = TAU * rank.rem_euclid(steps as i64) as f64 / steps as f64;
            (at_rank(rank), true, angle)
        })
        .collect();
    for way in &contact.rays {
        let angle = way.dot(circle.v).atan2(way.dot(circle.u));
        let mut at = if whole {
            on_the_turn(angle)
        } else {
            angle + TAU * ((low - angle) / TAU).ceil()
        };
        while at < high {
            let kept = contact.beneath.contains(way) || !by_an_end(at);
            if inside(at) && kept && !beside_a_plane(at) {
                places.push((at, false, angle));
            }
            at += TAU;
        }
    }
    places.sort_by(|one, other| one.0.total_cmp(&other.0).then(other.1.cmp(&one.1)));
    let mut kept: Vec<(f64, bool, f64)> = Vec::with_capacity(places.len());
    for place in places {
        match kept.last_mut() {
            Some(last) if place.0 - last.0 <= gap => {
                if place.1 && !last.1 {
                    *last = place;
                }
            }
            _ => kept.push(place),
        }
    }
    if whole && kept.len() > 1 && kept[0].0 + TAU - kept[kept.len() - 1].0 <= gap {
        kept.pop();
    }
    if edge.to < edge.from {
        kept.reverse();
    }
    let apart = |one: f64, other: f64| ((one - other + PI).rem_euclid(TAU) - PI).abs();
    let anchored = |angle: f64| {
        contact
            .anchors
            .iter()
            .map(|point| {
                let from = *point - circle.center;
                let along = circle.axis.dot(from);
                (from - circle.axis * along, along.abs())
            })
            .filter(|(flat, _)| apart(flat.dot(circle.v).atan2(flat.dot(circle.u)), angle) <= gap)
            .min_by(|one, other| one.1.total_cmp(&other.1))
    };
    let mut points: Vec<DVec3> = kept
        .into_iter()
        .map(|(_, _, angle)| match anchored(angle) {
            Some((flat, _)) => circle.center + flat,
            None => circle.point(angle),
        })
        .collect();
    points.dedup();
    if whole && points.len() > 1 && points[0] == points[points.len() - 1] {
        points.pop();
    }
    points
}
