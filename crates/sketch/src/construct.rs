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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_circle_through_two_points_is_centred_where_it_can_be() {
        let (a, b) = (DVec2::new(0.0, 0.0), DVec2::new(40.0, 0.0));
        let centre = centre_through(a, b, DVec2::new(35.0, 30.0)).unwrap();

        assert!((centre.x - 20.0).abs() < 1e-9, "centre = {centre:?}");
        assert!((centre.distance(a) - centre.distance(b)).abs() < 1e-9);
    }

    #[test]
    fn a_circle_touching_two_lines_sits_on_their_bisector() {
        let first = (DVec2::new(0.0, 0.0), DVec2::new(100.0, 0.0));
        let second = (DVec2::new(0.0, 0.0), DVec2::new(0.0, 100.0));
        let found = centre_touching_two(first, second, DVec2::new(30.0, 20.0)).unwrap();

        let (centre, radius) = (found.centre, found.radius);
        assert!((centre.x - centre.y).abs() < 1e-9, "centre = {centre:?}");
        assert!((radius - centre.x).abs() < 1e-9);

        // Told to be 10 across, it slides down the same bisector.
        let bigger = resize_touching(found, 10.0);
        assert!((bigger.radius - 10.0).abs() < 1e-9);
        assert!(
            (bigger.centre.x - 10.0).abs() < 1e-9,
            "centre = {:?}",
            bigger.centre
        );
    }

    #[test]
    fn two_lines_that_never_meet_take_the_middle() {
        let first = (DVec2::new(0.0, 0.0), DVec2::new(100.0, 0.0));
        let second = (DVec2::new(0.0, 40.0), DVec2::new(100.0, 40.0));
        let found = centre_touching_two(first, second, DVec2::new(60.0, 5.0)).unwrap();

        assert!(
            (found.centre.y - 20.0).abs() < 1e-9,
            "centre = {:?}",
            found.centre
        );
        assert!((found.centre.x - 60.0).abs() < 1e-9);
        assert!((found.radius - 20.0).abs() < 1e-9);
        assert!(found.anchor.is_none(), "two parallels meet at no corner");
    }

    #[test]
    fn a_circle_of_a_given_size_through_two_points() {
        let (a, b) = (DVec2::new(0.0, 0.0), DVec2::new(6.0, 0.0));
        let centre = centre_through_at(a, b, DVec2::new(3.0, 10.0), 5.0).unwrap();

        assert!(
            centre.distance(DVec2::new(3.0, 4.0)) < 1e-9,
            "centre = {centre:?}"
        );
        assert!(centre_through_at(a, b, DVec2::new(3.0, 10.0), 2.0).is_none());
    }

    #[test]
    fn a_circle_touching_three_lines_is_the_one_inside_them() {
        // A 3-4-5 triangle, whose inscribed circle has a radius of exactly 1.
        let a = DVec2::new(0.0, 0.0);
        let b = DVec2::new(4.0, 0.0);
        let c = DVec2::new(0.0, 3.0);
        let (centre, radius) = circle_touching_three((a, b), (b, c), (c, a)).unwrap();

        assert!((radius - 1.0).abs() < 1e-9, "radius = {radius}");
        assert!(
            centre.distance(DVec2::new(1.0, 1.0)) < 1e-9,
            "centre = {centre:?}"
        );
    }
}
