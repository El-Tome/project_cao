//! A point and an arc, a circle or an ellipse made to coincide in the order
//! they were clicked (#554): where on the curve the point lands, and how the
//! curve comes to a point clicked first.
//!
//! A curve holds a point on the whole of itself — an arc on its whole circle —
//! but a point laid on it lands on what is drawn of it, and clear of its ends:
//! a point on top of an end reads as a corner that is not one, and one made
//! one with the end is a merge nobody asked for. A curve coming to a point
//! clicked first moves whole, its size and the way it is turned kept, until it
//! passes through the point; when the point then falls beyond what is drawn,
//! the curve lengthens a little past it, an end slid round the curve. Where it
//! cannot — half an ellipse drawn by its ends ends on its own axis, which no
//! end slides off without turning the curve, and an arc's shape may not follow
//! its end — it moves until what is drawn reaches the point instead.

use std::f64::consts::{PI, TAU};

use glam::DVec2;

use super::super::{PointId, Sketch};
use crate::resizing::Curved;
use crate::sketch::Element;

/// How far round a curve a point laid on it stands from either end of what is
/// drawn, and how far past the point a curve lengthened to it reaches: enough
/// to read as two points, never one on top of the other. A turn in radians —
/// about the centre on an arc, in the curve's own measure on an ellipse.
pub const CLEAR_OF_AN_END: f64 = PI / 36.0;

impl Sketch {
    /// Where a point laid on a curve lands: the nearest place on what is drawn
    /// of it, clear of its ends.
    pub(super) fn landing_on(&self, curve: Curved, place: DVec2) -> DVec2 {
        let turn = self.turn_nearest(curve, place);
        let turn = self
            .drawn_run(curve)
            .and_then(|run| clear_of_the_ends(turn, run))
            .unwrap_or(turn);
        self.place_round(curve, turn)
    }

    /// Whether a place stands on what is drawn of a curve, its ends
    /// included: where a point laid on it may land, whichever way it came.
    pub(super) fn on_what_is_drawn(&self, curve: Curved, place: DVec2) -> bool {
        let off = match curve {
            Curved::Circle(_) => self
                .place_round(curve, self.turn_nearest(curve, place))
                .distance(place),
            Curved::Arc(arc) => self.distance_to_arc(arc, place),
            Curved::Ellipse(ellipse) => self.distance_to_ellipse(ellipse, place),
        };
        off <= self.drawing_size() * crate::solver::TOLERANCE
    }

    /// Carries a curve whole until it passes through a point, its size and the
    /// way it is turned kept — the curve alone, the rest of its shape left to
    /// follow, or its whole shape with it. When the point falls beyond what is
    /// drawn, the curve is lengthened past it, or — when it is not to be, or
    /// its end cannot slide — carried until what is drawn reaches it. What was
    /// moved, to be held while the drawing settles; nothing when the curve
    /// cannot come: it, or its shape, stands on the origin, on a fixed point,
    /// or on the point itself.
    pub(super) fn bring_curve(
        &mut self,
        curve: Curved,
        point: PointId,
        with_its_shape: bool,
        lengthened_past: bool,
    ) -> Option<Vec<PointId>> {
        let element = curve_element(curve);
        let own = self.points_it_leans_on(element);
        // The rule being laid already holds the point on the curve.
        let mut held_on = self.points_held_on(element);
        held_on.retain(|held| *held != point);
        let pinned = self.pinned_points();
        let stuck = |stands: &PointId| pinned[stands.0] || self.is_held(Element::Point(*stands));
        if own.contains(&point) || own.iter().chain(&held_on).any(stuck) {
            return None;
        }
        let place = self.point(point);
        let turn = self.turn_nearest(curve, place);
        let (meets, slid) = match self.drawn_run(curve) {
            None => (turn, None),
            Some(run) => match clear_of_the_ends(turn, run) {
                None => (turn, None),
                Some(clear) => match self.end_toward(curve, turn, run) {
                    Some(end) if lengthened_past => (turn, Some((end, run))),
                    _ => (clear, None),
                },
            },
        };
        let step = place - self.place_round(curve, meets);
        let moved = match with_its_shape {
            true => self.shape_carried(&own, &[point], (point, own[0]), step)?,
            false => self.curve_carried(&own, &held_on, point, step)?,
        };
        if let Some((end, run)) = slid {
            let past = lengthened(turn, run);
            self.move_point(end, self.place_round(curve, past));
        }
        Some(moved)
    }

    /// The curve standing on `own` carried by `step` with the points held on
    /// it and every curve turning about its centre, as a drag of the centre
    /// carries them. Nothing, and nothing moved, when one of them cannot go.
    fn curve_carried(
        &mut self,
        own: &[PointId],
        held_on: &[PointId],
        point: PointId,
        step: DVec2,
    ) -> Option<Vec<PointId>> {
        let centre = own[0];
        let mut carried = match step == DVec2::ZERO {
            true => own
                .iter()
                .chain(held_on)
                .copied()
                .filter(|stands| *stands != centre)
                .collect(),
            false => self.carry_curves_about(centre, step, [point, point]),
        };
        if !held_on
            .iter()
            .chain(own)
            .all(|stands| *stands == centre || carried.contains(stands))
        {
            for moved in &carried {
                self.move_point(*moved, self.point(*moved) - step);
            }
            return None;
        }
        self.move_point(centre, self.point(centre) + step);
        carried.push(centre);
        Some(carried)
    }

    /// The whole shape some points belong to carried by `step`: nothing when
    /// one of `apart` is in it — what stays while it comes — or something in
    /// it cannot move. The shape is read without the rule being laid, which
    /// joins the point to the curve's centre: the two are apart until it has
    /// landed.
    pub(super) fn shape_carried(
        &mut self,
        of: &[PointId],
        apart: &[PointId],
        laid: (PointId, PointId),
        step: DVec2,
    ) -> Option<Vec<PointId>> {
        let pinned = self.pinned_points();
        let pairs: Vec<(PointId, PointId)> = self
            .joined_pairs()
            .into_iter()
            .filter(|pair| *pair != laid && *pair != (laid.1, laid.0))
            .collect();
        let mut shape = of.to_vec();
        for point in of.iter().filter(|point| !pinned[point.0]) {
            let steps = self.steps_from(*point, &pairs);
            shape.extend(
                (0..steps.len())
                    .filter(|rank| steps[*rank].is_some())
                    .map(PointId),
            );
        }
        shape.sort_by_key(|point| point.0);
        shape.dedup();
        if shape.iter().any(|stands| apart.contains(stands))
            || shape
                .iter()
                .any(|stands| pinned[stands.0] || self.is_held(Element::Point(*stands)))
        {
            return None;
        }
        for stands in &shape {
            self.move_point(*stands, self.point(*stands) + step);
        }
        Some(shape)
    }

    /// The end of what is drawn of a curve a turn falls nearest to, when that
    /// end can slide round the curve: an arc's own, or one a cut left on an
    /// ellipse. Nothing for an end of an ellipse's own axis, nor a fixed one.
    fn end_toward(&self, curve: Curved, turn: f64, (from, sweep): (f64, f64)) -> Option<PointId> {
        let ahead = wrapped(turn - (from + sweep / 2.0)) >= 0.0;
        let (start, end) = match curve {
            Curved::Circle(_) => return None,
            Curved::Arc(arc) => {
                let curve = self.arc(arc);
                (curve.start, curve.end)
            }
            Curved::Ellipse(ellipse) => self.ellipse_ends(ellipse)?,
        };
        let end = match ahead {
            true => end,
            false => start,
        };
        let an_axis = match curve {
            Curved::Ellipse(ellipse) => self.ellipse_points(ellipse).contains(&end),
            Curved::Circle(_) | Curved::Arc(_) => false,
        };
        (!an_axis && !self.is_held(Element::Point(end))).then_some(end)
    }

    /// The turn round a curve nearest a place: about the centre on a circle or
    /// an arc, in its own measure on an ellipse.
    fn turn_nearest(&self, curve: Curved, place: DVec2) -> f64 {
        match curve {
            Curved::Circle(_) | Curved::Arc(_) => {
                let out = place - self.centre_of(curve);
                match out.length() > 1e-12 {
                    true => out.to_angle(),
                    false => self
                        .drawn_run(curve)
                        .map_or(0.0, |(from, sweep)| from + sweep / 2.0),
                }
            }
            Curved::Ellipse(ellipse) => self.ellipse_draft(ellipse).turn_nearest(place),
        }
    }

    /// Where a turn round a curve lands.
    fn place_round(&self, curve: Curved, turn: f64) -> DVec2 {
        match curve {
            Curved::Circle(circle) => {
                self.centre_of(curve) + DVec2::from_angle(turn) * self.circle(circle).radius
            }
            Curved::Arc(arc) => {
                self.centre_of(curve) + DVec2::from_angle(turn) * self.arc_radius(arc)
            }
            Curved::Ellipse(ellipse) => self.ellipse_draft(ellipse).at(turn),
        }
    }

    /// Where what is drawn of a curve starts and how far round it runs, as
    /// turns; nothing for a whole circle or ellipse.
    fn drawn_run(&self, curve: Curved) -> Option<(f64, f64)> {
        match curve {
            Curved::Circle(_) => None,
            Curved::Arc(arc) => {
                let drawn = self.arc_draft(arc);
                Some(((drawn.start - drawn.centre).to_angle(), self.arc_sweep(arc)))
            }
            Curved::Ellipse(ellipse) => {
                self.ellipse_ends(ellipse)?;
                Some(self.ellipse_run(ellipse))
            }
        }
    }
}

/// The piece of the drawing a curve is.
pub(super) fn curve_element(curve: Curved) -> Element {
    match curve {
        Curved::Circle(circle) => Element::Circle(circle),
        Curved::Arc(arc) => Element::Arc(arc),
        Curved::Ellipse(ellipse) => Element::Ellipse(ellipse),
    }
}

/// A turn taken onto what is drawn, clear of its ends by `CLEAR_OF_AN_END` —
/// or as far as the middle, for a curve drawn shorter than twice that.
/// Nothing when it is there already.
fn clear_of_the_ends(turn: f64, (from, sweep): (f64, f64)) -> Option<f64> {
    let half = sweep / 2.0;
    let room = half - CLEAR_OF_AN_END.min(half);
    let off = wrapped(turn - (from + half));
    (off.abs() > room).then(|| from + half + off.clamp(-room, room))
}

/// Where the end nearest a turn goes for what is drawn to reach a little past
/// it: `CLEAR_OF_AN_END` past it, or halfway to the other end when less is
/// left between them.
fn lengthened(turn: f64, (from, sweep): (f64, f64)) -> f64 {
    let half = sweep / 2.0;
    let off = wrapped(turn - (from + half));
    let left = TAU - half - off.abs();
    let past = CLEAR_OF_AN_END.min(left / 2.0);
    match off >= 0.0 {
        true => turn + past,
        false => turn - past,
    }
}

/// A turn brought between half a turn back and half a turn on.
fn wrapped(turn: f64) -> f64 {
    (turn + PI).rem_euclid(TAU) - PI
}
