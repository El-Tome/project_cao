//! A drawing's gestures laid on a sketch, each the way its tool lays it
//! through `cao_part` and the application: what it reuses, what it holds,
//! what it lays alongside — and nothing solved afterwards, since a value or a
//! rule moving the drawing is #501's to check.

use cao_sketch::{
    Chamfer, ChosenAxis, Crossing, Element, EllipseMode, PointId, Repeats, Sketch, SketchAxis,
    Support, WorkPlane, ellipse_from, rise_of,
};
use glam::DVec2;

use super::{Axis, Gesture};

/// Nearer than this to a point, a place lands on it, as a click lands on a
/// point the magnets pulled it onto. Far finer than the application's reach,
/// which is a click's: a drawing missing a point by a hair is one a campaign
/// draws on purpose.
const ON_A_POINT: f64 = 1e-9;

/// Nearer than this to something drawn, a place picks it.
const UNDER_A_PLACE: f64 = 1e-6;

fn reach(at: DVec2, by: f64) -> f64 {
    by * (1.0 + at.abs().max_element())
}

/// The sketch a drawing leaves, every gesture laid in turn.
pub fn laid(gestures: &[Gesture]) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    for gesture in gestures {
        gesture.lay(&mut sketch);
    }
    sketch
}

/// Where a click lands: on the point already standing there, or on a new one
/// held on whatever curves it falls on — at most two, a crossing's, as the
/// application holds them.
#[derive(Clone, Debug)]
enum Landing {
    On(PointId),
    New(DVec2, Vec<Support>),
}

fn landing(sketch: &Sketch, place: DVec2) -> Landing {
    match sketch.nearest_point(place, reach(place, ON_A_POINT)) {
        Some(point) => Landing::On(point),
        None => Landing::New(
            place,
            sketch.supports_at(place).into_iter().take(2).collect(),
        ),
    }
}

fn landed(sketch: &mut Sketch, landing: Landing) -> PointId {
    match landing {
        Landing::On(point) => point,
        Landing::New(place, on) => {
            let point = sketch.add_point(place);
            for support in on {
                sketch.add_constraint(support.holding(point));
            }
            point
        }
    }
}

/// What a click at a place picks, in the order a click picks it: a point
/// before a trait before a circle, an arc, an ellipse. Never the origin.
fn picked(sketch: &Sketch, at: DVec2) -> Option<Element> {
    let near = reach(at, UNDER_A_PLACE);
    if let Some(point) = sketch
        .nearest_point(at, near)
        .filter(|point| !sketch.is_origin(*point))
    {
        return Some(Element::Point(point));
    }
    if let Some(segment) = sketch.nearest_segment(at, near) {
        return Some(Element::Segment(segment));
    }
    if let Some(circle) = sketch.nearest_circle(at, near) {
        return Some(Element::Circle(circle));
    }
    if let Some(arc) = sketch.nearest_arc(at, near) {
        return Some(Element::Arc(arc));
    }
    sketch.nearest_ellipse(at, near).map(Element::Ellipse)
}

/// What the copy tools hold once each place is clicked in turn: a click on
/// something already held lets it go again, as the tools do.
fn picked_all(sketch: &Sketch, places: &[[f64; 2]]) -> Vec<Element> {
    let mut elements = Vec::new();
    for at in places {
        if let Some(element) = picked(sketch, DVec2::from(*at)) {
            match elements.iter().position(|held| *held == element) {
                Some(held) => {
                    elements.remove(held);
                }
                None => elements.push(element),
            }
        }
    }
    elements
}

fn chosen(sketch: &Sketch, axis: Axis) -> Option<ChosenAxis> {
    match axis {
        Axis::U => Some(ChosenAxis::Sketch(SketchAxis::U)),
        Axis::V => Some(ChosenAxis::Sketch(SketchAxis::V)),
        Axis::Trait(at) => {
            let at = DVec2::from(at);
            sketch
                .nearest_segment(at, reach(at, UNDER_A_PLACE))
                .map(ChosenAxis::Trait)
        }
    }
}

fn point_at(sketch: &Sketch, at: [f64; 2]) -> Option<PointId> {
    let at = DVec2::from(at);
    sketch.nearest_point(at, reach(at, UNDER_A_PLACE))
}

fn trait_between(sketch: &mut Sketch, from: PointId, to: PointId, construction: bool) {
    if from == to {
        return;
    }
    match construction {
        true => sketch.add_construction_segment(from, to),
        false => sketch.add_segment(from, to),
    };
}

impl Gesture {
    /// Lays the gesture on the sketch. A gesture with nothing to act on, or
    /// that its tool would refuse, leaves the sketch as it was.
    pub fn lay(&self, sketch: &mut Sketch) {
        match self {
            Gesture::Chain {
                through,
                closed,
                construction,
            } => chain(sketch, through, *closed, *construction),
            Gesture::Rectangle {
                corner,
                opposite,
                construction,
            } => rectangle(
                sketch,
                DVec2::from(*corner),
                DVec2::from(*opposite),
                *construction,
            ),
            Gesture::Circle {
                centre,
                radius,
                construction,
            } => circle(sketch, DVec2::from(*centre), *radius, *construction),
            Gesture::Arc {
                centre,
                start,
                degrees,
                construction,
            } => arc(
                sketch,
                DVec2::from(*centre),
                DVec2::from(*start),
                *degrees,
                *construction,
            ),
            Gesture::Ellipse {
                centre,
                reach,
                across,
                construction,
            } => ellipse(
                sketch,
                DVec2::from(*centre),
                DVec2::from(*reach),
                *across,
                *construction,
            ),
            Gesture::HalfEllipse {
                from,
                to,
                rise,
                construction,
            } => half_ellipse(
                sketch,
                DVec2::from(*from),
                DVec2::from(*to),
                *rise,
                *construction,
            ),
            Gesture::Point { at } => {
                let at = DVec2::from(*at);
                let on = sketch.supports_at(at).into_iter().take(2).collect();
                landed(sketch, Landing::New(at, on));
            }
            Gesture::Fillet { at, radius } => {
                if let Some((first, second)) =
                    point_at(sketch, *at).and_then(|point| sketch.corner_at(point))
                {
                    sketch.fillet(first, second, *radius);
                }
            }
            Gesture::Chamfer { at, length } => {
                if let Some((first, second)) =
                    point_at(sketch, *at).and_then(|point| sketch.corner_at(point))
                {
                    sketch.chamfer(first, second, Chamfer::Equal(*length));
                }
            }
            Gesture::Mirror { of, axis } => {
                let elements = picked_all(sketch, of);
                if let Some(axis) = chosen(sketch, *axis)
                    && !elements.is_empty()
                {
                    sketch.mirror(&elements, axis);
                }
            }
            Gesture::PatternAround {
                of,
                centre,
                degrees,
                count,
            } => {
                let elements = picked_all(sketch, of);
                if let Some(centre) = point_at(sketch, *centre)
                    && !elements.is_empty()
                {
                    sketch.pattern_around(&elements, centre, *degrees, *count);
                }
            }
            Gesture::PatternAlong {
                of,
                axis,
                along,
                across,
            } => {
                let elements = picked_all(sketch, of);
                if let Some(axis) = chosen(sketch, *axis)
                    && !elements.is_empty()
                {
                    let repeats = |(step, count): (f64, usize)| Repeats { step, count };
                    sketch.pattern_along(&elements, axis, repeats(*along), repeats(*across));
                }
            }
            Gesture::Divide { at } => divide(sketch, DVec2::from(*at)),
            Gesture::Trim { at } => trim(sketch, DVec2::from(*at)),
            Gesture::Erase { at } => {
                if let Some(element) = picked(sketch, DVec2::from(*at)) {
                    sketch.erase(element);
                }
            }
        }
    }
}

/// The line tool, click after click: each click lands against the drawing as
/// it stands, so a chain coming back to its first place reuses its first
/// point.
fn chain(sketch: &mut Sketch, through: &[[f64; 2]], closed: bool, construction: bool) {
    let Some(first) = through.first() else {
        return;
    };
    let ends = through.iter().skip(1).chain(closed.then_some(first));
    let start = landing(sketch, DVec2::from(*first));
    let mut last = landed(sketch, start);
    for place in ends {
        let end = landing(sketch, DVec2::from(*place));
        let next = landed(sketch, end);
        trait_between(sketch, last, next, construction);
        last = next;
    }
}

/// Nearer than this, the two clicks of a rectangle are one, and the tool lays
/// nothing (`two_click_shape`, `crates/app/src/screens/viewport/input/`).
const ONE_CLICK: f64 = 1e-6;

/// `crates/part/src/straight.rs`: the two corners clicked land as any click
/// does, the two others are always new points. A rectangle dragged flat along
/// a line is laid all the same, as the tool lays it.
fn rectangle(sketch: &mut Sketch, corner: DVec2, opposite: DVec2, construction: bool) {
    if corner.distance(opposite) < ONE_CLICK {
        return;
    }
    let (first, third) = (landing(sketch, corner), landing(sketch, opposite));
    let first = landed(sketch, first);
    let third = landed(sketch, third);
    let second = sketch.add_point(DVec2::new(opposite.x, corner.y));
    let fourth = sketch.add_point(DVec2::new(corner.x, opposite.y));
    let corners = [first, second, third, fourth];
    for index in 0..4 {
        trait_between(
            sketch,
            corners[index],
            corners[(index + 1) % 4],
            construction,
        );
    }
}

/// The circle tool by its centre: the centre lands, and the place the cursor
/// gave the radius at — east of the centre here — stays as a point held on
/// the rim.
fn circle(sketch: &mut Sketch, centre: DVec2, radius: f64, construction: bool) {
    if radius <= 0.0 {
        return;
    }
    let (middle, rim) = (
        landing(sketch, centre),
        landing(sketch, centre + DVec2::X * radius),
    );
    let middle = landed(sketch, middle);
    let circle = match construction {
        true => sketch.add_construction_circle(middle, radius),
        false => sketch.add_circle(middle, radius),
    };
    let rim = landed(sketch, rim);
    sketch.add_constraint(Support::Circle(circle).holding(rim));
}

fn arc(sketch: &mut Sketch, centre: DVec2, start: DVec2, degrees: f64, construction: bool) {
    if centre.distance(start) <= 0.0 || degrees <= 0.0 || degrees >= 360.0 {
        return;
    }
    let end = centre + DVec2::from_angle(degrees.to_radians()).rotate(start - centre);
    let places = [centre, start, end].map(|place| landing(sketch, place));
    let [centre, start, end] = places.map(|place| landed(sketch, place));
    match construction {
        true => sketch.add_construction_arc(centre, start, end),
        false => sketch.add_arc(centre, start, end),
    };
}

/// The ellipse tool by its centre: the centre and the end of the first axis
/// land, the three other ends of the axes are always new points.
fn ellipse(sketch: &mut Sketch, centre: DVec2, reach: DVec2, across: f64, construction: bool) {
    let first = reach - centre;
    if first.length() <= 0.0 || across <= 0.0 {
        return;
    }
    let side = first.perp().normalize() * across;
    let (middle, end) = (landing(sketch, centre), landing(sketch, reach));
    let middle = landed(sketch, middle);
    let end = landed(sketch, end);
    let first = [sketch.add_point(centre - first), end];
    let second = [
        sketch.add_point(centre - side),
        sketch.add_point(centre + side),
    ];
    match construction {
        true => sketch.add_construction_ellipse(middle, first, second),
        false => sketch.add_ellipse(middle, first, second),
    };
}

/// The ellipse tool by its ends, as half an ellipse: the two ends land, the
/// centre and the top of the rise are always new, and only the half on the
/// side of the rise is drawn.
fn half_ellipse(sketch: &mut Sketch, from: DVec2, to: DVec2, rise: f64, construction: bool) {
    let Some(way) = (to - from).try_normalize() else {
        return;
    };
    let cursor = (from + to) / 2.0 + way.perp() * rise;
    let Some(drawn) = ellipse_from(EllipseMode::ByEnds, &[from, to], cursor) else {
        return;
    };
    let rising = rise_of(drawn, cursor);
    let ends = [landing(sketch, from), landing(sketch, to)];
    let first = ends.map(|end| landed(sketch, end));
    let centre = sketch.add_point(drawn.centre);
    let top = sketch.add_point(drawn.centre + rising.reach(drawn));
    let id = match construction {
        true => sketch.add_construction_ellipse(centre, first, [centre, top]),
        false => sketch.add_ellipse(centre, first, [centre, top]),
    };
    let [start, end] = rising.between().map(|rank| first[rank]);
    sketch.draw_the_stretch(id, start, end);
}

/// The division tool: only where traits or arcs cross, never on a circle or
/// an ellipse, which it refuses.
fn divide(sketch: &mut Sketch, at: DVec2) {
    if let Some(Crossing::Curves { at, segments, arcs }) =
        sketch.crossing_at(at, reach(at, UNDER_A_PLACE))
    {
        sketch.split(&segments, &arcs, at);
    }
}

/// The trim tool, as the application reads a click (`trim.rs`, `cut_under`):
/// a trait first, then an arc, a circle, an ellipse.
fn trim(sketch: &mut Sketch, at: DVec2) {
    let near = reach(at, UNDER_A_PLACE);
    if let Some(segment) = sketch.nearest_segment(at, near)
        && let Some((from, to)) = sketch.stretch_at(segment, at)
    {
        sketch.trim(segment, from, to);
        return;
    }
    if let Some(arc) = sketch.nearest_arc(at, near)
        && let Some((from, to)) = sketch.arc_stretch_at(arc, at)
    {
        sketch.trim_arc(arc, from, to);
        return;
    }
    if let Some(circle) = sketch.nearest_circle(at, near) {
        let between = sketch.circle_stretch_at(circle, at);
        sketch.trim_circle(circle, between);
        return;
    }
    if let Some(ellipse) = sketch.nearest_ellipse(at, near) {
        let between = sketch.ellipse_stretch_at(ellipse, at);
        if between.is_some() || sketch.ellipse_ends(ellipse).is_none() {
            sketch.trim_ellipse(ellipse, between);
        }
    }
}
