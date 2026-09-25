//! What pulls the cursor, and how hard.
//!
//! Aiming with a mouse is never exact, so the drawing pulls the cursor onto
//! what it is nearly on. Which of those magnets wins is a rule about the
//! drawing — a point already there is worth more than a curve, and a curve more
//! than the grid behind it — and it belongs with the drawing rather than with
//! the interface showing it. How far each one reaches is the other way round:
//! the interface measures that in pixels and hands it over in world units.

use glam::DVec2;

use crate::constraints::SketchAxis;
use crate::edges::off_by;
use crate::sketch::Sketch;

/// How far each magnet reaches, in the drawing's own units.
///
/// `grid_step` says both whether the grid pulls at all and how coarsely: no
/// step, no grid magnet.
#[derive(Clone, Copy, Debug)]
pub struct SnapSettings {
    pub point_reach: f64,
    pub curve_reach: f64,
    pub grid_step: Option<f64>,
    pub grid_reach: f64,
}

impl SnapSettings {
    /// The grid point nearest a place, when the grid pulls and the place is
    /// within its reach.
    pub fn node_near(&self, place: DVec2) -> Option<DVec2> {
        let step = self.grid_step.filter(|step| *step > 0.0)?;
        let node = DVec2::new(
            (place.x / step).round() * step,
            (place.y / step).round() * step,
        );
        (node.distance(place) <= self.grid_reach).then_some(node)
    }
}

/// What the cursor has been pulled onto, when it is worth saying so.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Snap {
    Point,
    /// Where two curves run through the same place without the drawing having
    /// a point there. It carries its position for the same reason a midpoint
    /// does: at a shallow angle nothing on screen tells a crossing from a near
    /// miss.
    Crossing(DVec2),
    /// The middle of a line, which needs a mark of its own: nothing else on
    /// screen says the cursor is exactly halfway along.
    Midpoint(DVec2),
    /// Somewhere along a drawn curve, its ends included.
    OnCurve(DVec2),
}

impl Sketch {
    /// Pulls the cursor onto whatever it is near: an existing point first, then
    /// a crossing, then the middle of a trait, then any drawn curve, then the
    /// grid.
    ///
    /// The grid magnet is what makes drawing on the origin, or a right angle by
    /// following the lines, a matter of aiming roughly rather than exactly. It
    /// only bites within its own reach, so a deliberate free position is still
    /// possible.
    pub fn magnetise(&self, cursor: DVec2, settings: &SnapSettings) -> (DVec2, Option<Snap>) {
        if let Some(point) = self.nearest_point(cursor, settings.point_reach) {
            return (self.point(point), Some(Snap::Point));
        }

        // A crossing ranks under a drawn point and over a midpoint: the drawing
        // names the one and merely happens to make the other, and between the
        // two of them the crossing is the place a person was aiming at.
        if let Some(at) = self.nearest_crossing(cursor, settings.point_reach) {
            return (at, Some(Snap::Crossing(at)));
        }

        // A curve already drawn pulls harder than the grid, and a trait's middle
        // harder still: joining the middle of a side is a thing one aims at, and
        // landing a hair off it leaves geometry that only looks joined.
        if let Some((_, middle)) = self.nearest_midpoint(cursor, settings.curve_reach) {
            return (middle, Some(Snap::Midpoint(middle)));
        }
        if let Some(at) = self.nearest_on_curve(cursor, settings.curve_reach) {
            return (at, Some(Snap::OnCurve(at)));
        }

        (settings.node_near(cursor).unwrap_or(cursor), None)
    }

    /// The place on a drawn curve nearest the cursor, whichever kind of curve
    /// it turns out to be: between a trait, a circle, an arc, an ellipse and
    /// the plane's own axes, the nearer one wins.
    ///
    /// The axes pull like anything else drawn: they are lines of the sketch a
    /// point can be laid on and held to, and a half-shape closed against one
    /// has to be able to land on it whether the grid is on or off.
    fn nearest_on_curve(&self, cursor: DVec2, reach: f64) -> Option<DVec2> {
        let on_traits = self.nearest_on_segment(cursor, reach).map(|(_, at)| at);
        let on_circles = self
            .live_circles()
            .filter_map(|(_, circle)| onto_rim(cursor, self.point(circle.center), circle.radius));
        let on_arcs = self
            .live_arcs()
            .map(|(id, _)| self.place_on_arc(id, cursor));
        let on_ellipses = self
            .live_ellipses()
            .map(|(id, _)| self.place_on_ellipse(id, cursor));
        let on_axes = axes().map(|axis| {
            let along = axis.direction();
            along * cursor.dot(along)
        });
        on_traits
            .into_iter()
            .chain(on_circles)
            .chain(on_arcs)
            .chain(on_ellipses)
            .chain(on_axes)
            .filter(|at| at.distance(cursor) <= reach)
            .min_by(|left, right| {
                left.distance_squared(cursor)
                    .total_cmp(&right.distance_squared(cursor))
            })
    }

    fn nearest_crossing(&self, cursor: DVec2, reach: f64) -> Option<DVec2> {
        self.crossings()
            .into_iter()
            .chain(self.crossed_axes())
            .filter(|at| at.distance(cursor) <= reach)
            .min_by(|left, right| {
                left.distance_squared(cursor)
                    .total_cmp(&right.distance_squared(cursor))
            })
    }
}

impl Sketch {
    /// Where the curves of the drawing cross the axes of the plane.
    ///
    /// The axes are no part of the drawing — nothing cuts them, and no area is
    /// bounded by them — so they stay out of `crossings`, which is what a
    /// division reads. A magnet is another matter: a trait running down to the
    /// axis is a place one aims at.
    fn crossed_axes(&self) -> Vec<DVec2> {
        let mut places = Vec::new();
        for axis in axes() {
            let along = axis.direction();
            let across = DVec2::new(-along.y, along.x);
            for (id, _) in self.live_segments() {
                let (start, end) = self.endpoints(id);
                let (from, to) = (start.dot(across), end.dot(across));
                if (from - to).abs() < 1e-12 {
                    continue;
                }
                let fraction = from / (from - to);
                if fraction > 0.0 && fraction < 1.0 {
                    places.push(start.lerp(end, fraction));
                }
            }
            for (_, circle) in self.live_circles() {
                places.extend(met_along(self.point(circle.center), circle.radius, along));
            }
            for (id, arc) in self.live_arcs() {
                let met = met_along(self.point(arc.center), self.arc_radius(id), along);
                places.extend(
                    met.into_iter()
                        .filter(|place| self.distance_to_arc(id, *place) <= off_by(*place)),
                );
            }
        }
        places
    }
}

/// Where a line through the origin, running that way, meets a circle.
fn met_along(centre: DVec2, radius: f64, along: DVec2) -> Vec<DVec2> {
    let middle = centre.dot(along);
    let across = (centre - along * middle).length();
    let half_chord = radius * radius - across * across;
    if half_chord < 0.0 {
        return Vec::new();
    }
    let half = half_chord.sqrt();
    match half < 1e-9 {
        true => vec![along * middle],
        false => vec![along * (middle - half), along * (middle + half)],
    }
}

/// The two axes of the plane, which pull like the curves drawn on it.
fn axes() -> impl Iterator<Item = SketchAxis> {
    [SketchAxis::U, SketchAxis::V].into_iter()
}

/// Where a place lands when pulled straight onto a rim of that centre and
/// radius. Dead on the centre it lands nowhere: every place on the rim is as
/// near as every other, and picking one of them would be inventing a direction.
pub(crate) fn onto_rim(from: DVec2, centre: DVec2, radius: f64) -> Option<DVec2> {
    let reach = from - centre;
    (reach.length() > 1e-9).then(|| centre + reach.normalize() * radius)
}

#[cfg(test)]
mod tests;
