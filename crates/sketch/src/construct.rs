//! Where the centre of a circle lands, for the ways of drawing one that do not
//! give it straight away.
//!
//! Pure geometry, kept apart from the sketch itself: these are the answers to
//! "which circle did the user mean", and they are worth testing on their own.

use glam::DVec2;

/// A straight line, as two points it passes through.
pub type Line = (DVec2, DVec2);

/// The centre of a circle through `a` and `b`, as near as possible to where the
/// user is pointing.
///
/// Every such centre sits on the perpendicular bisector of the two points, so
/// the click is simply brought back onto it: the user says roughly where, the
/// geometry says exactly where.
pub fn centre_through(a: DVec2, b: DVec2, towards: DVec2) -> Option<DVec2> {
    let span = b - a;
    let length = span.length();
    if length < 1e-9 {
        return None;
    }
    let middle = (a + b) * 0.5;
    let along = DVec2::new(-span.y, span.x) / length;
    Some(middle + along * (towards - middle).dot(along))
}

/// A circle found by what it touches, and the point its size grows from.
///
/// `anchor` is where the two lines meet: the radius grows in step with the
/// distance from it, which is what lets a size typed by hand be turned back
/// into a place for the centre. Two lines running alongside each other have no
/// such point — and no choice of size either.
pub struct Touching {
    pub centre: DVec2,
    pub radius: f64,
    pub anchor: Option<DVec2>,
}

/// The centre of a circle touching two lines, as near as possible to the click.
///
/// Such a centre is equally far from both lines, which puts it on one of their
/// two bisectors — or, when the lines never meet, on the line halfway between
/// them. The click picks which and where along it.
pub fn centre_touching_two(first: Line, second: Line, towards: DVec2) -> Option<Touching> {
    let (u, v) = (
        (first.1 - first.0).normalize_or_zero(),
        (second.1 - second.0).normalize_or_zero(),
    );
    if u == DVec2::ZERO || v == DVec2::ZERO {
        return None;
    }

    let crossing = u.perp_dot(v);
    let mut anchor = None;
    let centre = if crossing.abs() < 1e-9 {
        // Parallel: the centre runs down the middle, wherever the click says.
        let normal = DVec2::new(-u.y, u.x);
        let gap = (second.0 - first.0).dot(normal) * 0.5;
        let middle = first.0 + normal * gap;
        middle + u * (towards - middle).dot(u)
    } else {
        let meeting = first.0 + u * (second.0 - first.0).perp_dot(v) / crossing;
        // The bisector the click is nearer to, then the click brought onto it.
        let reach = towards - meeting;
        let bisector = [u + v, u - v]
            .into_iter()
            .map(DVec2::normalize_or_zero)
            .filter(|direction| *direction != DVec2::ZERO)
            .max_by(|a, b| reach.dot(*a).abs().total_cmp(&reach.dot(*b).abs()))?;
        anchor = Some(meeting);
        meeting + bisector * reach.dot(bisector)
    };

    Some(Touching {
        centre,
        radius: distance_to(centre, first),
        anchor,
    })
}

/// The same circle, resized to the radius asked for.
///
/// The centre slides along the bisector it already sits on: a circle told to be
/// bigger has to move away from the corner to stay touching both lines.
pub fn resize_touching(found: Touching, radius: f64) -> Touching {
    let (Some(anchor), true) = (found.anchor, found.radius > 1e-9) else {
        return found;
    };
    Touching {
        centre: anchor + (found.centre - anchor) * (radius / found.radius),
        radius,
        anchor: Some(anchor),
    }
}

/// The centre of a circle of a given size through `a` and `b`, on the side the
/// user is pointing at. Nothing when the circle would be too small to reach
/// both points.
pub fn centre_through_at(a: DVec2, b: DVec2, towards: DVec2, radius: f64) -> Option<DVec2> {
    let span = b - a;
    let half = span.length() * 0.5;
    if half < 1e-9 || radius < half {
        return None;
    }
    let middle = (a + b) * 0.5;
    let along = DVec2::new(-span.y, span.x) / (half * 2.0);
    let reach = (radius * radius - half * half).sqrt();
    let side = match (towards - middle).dot(along) < 0.0 {
        true => -1.0,
        false => 1.0,
    };
    Some(middle + along * reach * side)
}

/// The circle touching all three lines: the one inside the triangle they make.
///
/// The centre is the triangle's incentre, which is the corners weighted by the
/// lengths of the sides facing them.
pub fn circle_touching_three(first: Line, second: Line, third: Line) -> Option<(DVec2, f64)> {
    let a = crossing(second, third)?;
    let b = crossing(third, first)?;
    let c = crossing(first, second)?;

    let (side_a, side_b, side_c) = (b.distance(c), c.distance(a), a.distance(b));
    let perimeter = side_a + side_b + side_c;
    if perimeter < 1e-9 {
        return None;
    }
    let centre = (a * side_a + b * side_b + c * side_c) / perimeter;
    Some((centre, distance_to(centre, first)))
}

/// The centre of the circle running through all three points exactly — the
/// point every one of their perpendicular bisectors agrees on.
///
/// Built from the same `crossing` two lines already use to meet: each
/// bisector is carried as the two points that describe it, so no new way of
/// intersecting anything is needed.
pub fn circumcentre(a: DVec2, b: DVec2, c: DVec2) -> Option<DVec2> {
    crossing(perpendicular_bisector(a, b)?, perpendicular_bisector(b, c)?)
}

/// The perpendicular bisector of two points, as a line through its midpoint.
fn perpendicular_bisector(a: DVec2, b: DVec2) -> Option<Line> {
    let span = b - a;
    if span.length() < 1e-9 {
        return None;
    }
    let middle = (a + b) * 0.5;
    Some((middle, middle + DVec2::new(-span.y, span.x)))
}

/// Where two lines meet, or nothing when they run alongside each other.
fn crossing(first: Line, second: Line) -> Option<DVec2> {
    let (u, v) = (first.1 - first.0, second.1 - second.0);
    let crossing = u.perp_dot(v);
    if crossing.abs() < 1e-12 {
        return None;
    }
    Some(first.0 + u * (second.0 - first.0).perp_dot(v) / crossing)
}

fn distance_to(point: DVec2, line: Line) -> f64 {
    let span = line.1 - line.0;
    let length = span.length();
    match length < 1e-9 {
        true => point.distance(line.0),
        false => (span.perp_dot(point - line.0) / length).abs(),
    }
}

/// How a circle is being drawn.
///
/// Every one of them ends the same way — a centre and a radius — but what the
/// user points at to get there differs, and so does what is known after each
/// click.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CircleMode {
    /// The centre, then a point of the rim.
    #[default]
    Center,
    /// Two opposite points of the rim.
    TwoPoints,
    /// Two points of the rim, then the centre — which can only sit on their
    /// perpendicular bisector, so the click is brought back onto it.
    ThreePoints,
    /// Two traits it must touch, then the centre on their bisector.
    TwoTangents,
    /// Three traits it must touch: the circle inscribed between them, with
    /// nothing left to choose.
    ThreeTangents,
}

impl CircleMode {
    /// Whether it is drawn by pointing at traits rather than at places.
    pub fn touches_traits(self) -> bool {
        matches!(self, Self::TwoTangents | Self::ThreeTangents)
    }

    /// How many things it needs before the circle is settled.
    pub fn wants(self) -> usize {
        match self {
            Self::Center | Self::TwoPoints => 2,
            Self::ThreePoints | Self::TwoTangents | Self::ThreeTangents => 3,
        }
    }
}

#[cfg(test)]
mod tests;
