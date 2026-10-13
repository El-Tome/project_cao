//! Joins and cuts of two walls about parallel axes, an elliptic one among
//! them (#571): round holes and posts through and beside elliptic bosses,
//! crossing at two rulings or four, touching outside, touching inside and
//! crossing twice; two ellipses crossing or touching, and the same ellipse
//! a ten-thousandth off. Each held to its listing, to the area its sections
//! enclose times its height, and drawn closed and uncrossed from a fine
//! tolerance to a coarse one.
//!
//! The area is the arithmetic's, read off the two sections apart from the
//! kernel: where their curves cross, found by halving between samples, and
//! the area each arc of one inside the other sweeps about the origin, in
//! closed form.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::soundness::{closed, uncrossed};

/// A section seen square to Z: `center + first cos t + second sin t`,
/// `second` a quarter turn counter-clockwise from `first`.
#[derive(Clone, Copy, Debug)]
struct Section {
    center: DVec2,
    first: DVec2,
    second: DVec2,
}

impl Section {
    fn ellipse(center: [f64; 2], first: DVec2, other: f64) -> Section {
        Section {
            center: DVec2::from(center),
            first,
            second: first.perp().normalize() * other,
        }
    }

    fn circle(center: [f64; 2], radius: f64) -> Section {
        Section::ellipse(center, DVec2::X * radius, radius)
    }

    fn point(&self, t: f64) -> DVec2 {
        self.center + self.first * t.cos() + self.second * t.sin()
    }

    /// Below nought inside, above outside.
    fn implicit(&self, point: DVec2) -> f64 {
        let from = point - self.center;
        let x = from.dot(self.first) / self.first.length_squared();
        let y = from.dot(self.second) / self.second.length_squared();
        x * x + y * y - 1.0
    }

    fn area(&self) -> f64 {
        PI * self.first.length() * self.second.length()
    }

    /// The area the arc from `from` to `to` sweeps about the origin.
    fn swept(&self, from: f64, to: f64) -> f64 {
        let across = self.first.perp_dot(self.second);
        let chord = self.point(to) - self.point(from);
        (across * (to - from) + self.center.perp_dot(chord)) / 2.0
    }

    /// The parameter of the point of the curve `point` stands on, in
    /// `[0, 2π)`.
    fn parameter(&self, point: DVec2) -> f64 {
        let from = point - self.center;
        let x = from.dot(self.first) / self.first.length_squared();
        let y = from.dot(self.second) / self.second.length_squared();
        y.atan2(x).rem_euclid(TAU)
    }

    /// The points where this section's curve crosses `other`'s, where the
    /// other's equation changes sign between two samples, by halving:
    /// samples within rounding of the other curve, as about a touch, tell no
    /// side.
    fn crossings(&self, other: &Section) -> Vec<DVec2> {
        const SAMPLES: usize = 20_000;
        const ROUNDING: f64 = 1e-12;
        let side = |t: f64| other.implicit(self.point(t));
        let mut found = Vec::new();
        let mut last: Option<(f64, bool)> = None;
        for step in 0..=SAMPLES {
            let t = TAU * step as f64 / SAMPLES as f64;
            let value = side(t);
            if value.abs() <= ROUNDING {
                continue;
            }
            let outside = value > 0.0;
            if let Some((from, was)) = last
                && was != outside
            {
                let [mut low, mut high] = [from, t];
                for _ in 0..200 {
                    let middle = (low + high) / 2.0;
                    if (side(middle) > 0.0) == was {
                        low = middle;
                    } else {
                        high = middle;
                    }
                }
                found.push(self.point((low + high) / 2.0));
            }
            last = Some((t, outside));
        }
        found
    }

    /// The area of this section's arcs standing inside `other`, swept, the
    /// arcs ending at `crossings`.
    fn inside(&self, other: &Section, crossings: &[DVec2]) -> f64 {
        let mut at: Vec<f64> = crossings
            .iter()
            .map(|point| self.parameter(*point))
            .collect();
        at.sort_by(f64::total_cmp);
        if at.is_empty() {
            let deepest = (0..64)
                .map(|step| other.implicit(self.point(TAU * f64::from(step) / 64.0)))
                .max_by(|one, other| one.abs().total_cmp(&other.abs()))
                .unwrap_or(1.0);
            return if deepest < 0.0 { self.area() } else { 0.0 };
        }
        let mut swept = 0.0;
        for (index, &from) in at.iter().enumerate() {
            let to = at.get(index + 1).copied().unwrap_or(at[0] + TAU);
            let deepest = [0.25, 0.5, 0.75]
                .map(|share| other.implicit(self.point(from + (to - from) * share)))
                .into_iter()
                .max_by(|one, other| one.abs().total_cmp(&other.abs()))
                .unwrap_or(1.0);
            if deepest < 0.0 {
                swept += self.swept(from, to);
            }
        }
        swept
    }

    /// The area both sections enclose, both curves' arcs ending at the very
    /// points where they cross, so that the boundary closes.
    fn overlap(&self, other: &Section) -> f64 {
        let crossings = self.crossings(other);
        self.inside(other, &crossings) + other.inside(self, &crossings)
    }
}

/// A prism of a section from `from` to `to` along Z, handed to the kernel
/// as an ellipse or, where both half-axes are one, a circle.
fn raised(section: Section, from: f64, to: f64) -> Body {
    let (first, other) = (section.first.length(), section.second.length());
    let start = section.center + section.first;
    let runs = if first == other {
        vec![Run::Round {
            center: section.center,
            turn: TAU,
        }]
    } else {
        vec![Run::Oval {
            center: section.center,
            first: section.first,
            second: other,
            turn: TAU,
        }]
    };
    let outline = Contour {
        corners: vec![start],
        runs,
    };
    Body::raised(&outline, &[], ground(from), DVec3::Z * (to - from)).expect("a prism raises")
}

/// The body's listing, its volume against the arithmetic's, and its
/// triangles closed and uncrossed from a fine tolerance to a coarse one.
fn held(body: &Body, volume: f64) {
    assert!(
        (body.volume() - volume).abs() <= 1e-9 * volume,
        "the body holds {}, the arithmetic {volume}",
        body.volume()
    );
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    for share in [1e-4, 1e-3, 2e-2] {
        let triangles = body.triangles(share * body.scale().reach());
        assert_eq!(closed(&triangles).map_err(|_| share), Ok(()), "open");
        assert_eq!(uncrossed(&triangles).map_err(|_| share), Ok(()), "crossed");
    }
}

/// The ellipse every case below is drawn on: half-axes 7 and 3, the major
/// one turned 0.4 off X, about `(3, −1)`.
fn boss() -> Section {
    Section::ellipse([3.0, -1.0], DVec2::from_angle(0.4) * 7.0, 3.0)
}

/// A place of the plane, `(x, y)` along the boss's own half-axes from its
/// centre.
fn on_the_boss(x: f64, y: f64) -> [f64; 2] {
    let boss = boss();
    let place = boss.center + boss.first.normalize() * x + boss.second.normalize() * y;
    place.to_array()
}

/// The boss cut by a prism of `tool` through it, and joined to one.
fn cut_and_joined(tool: Section) {
    let boss = boss();
    let body = raised(boss, 0.0, 4.0);
    let cut = body
        .cut_by(&raised(tool, -1.0, 5.0))
        .expect("the tool is cut out of the boss");
    held(&cut, (boss.area() - boss.overlap(&tool)) * 4.0);
    let joined = body
        .joined(&raised(tool, 0.0, 4.0))
        .expect("the tool is joined to the boss");
    held(
        &joined,
        (boss.area() + tool.area() - boss.overlap(&tool)) * 4.0,
    );
}

#[test]
fn a_round_hole_crossing_an_elliptic_boss_s_wall_at_two_rulings_or_four_holds_its_volume() {
    cut_and_joined(Section::circle(on_the_boss(7.0, 0.0), 2.0));
    cut_and_joined(Section::circle([9.3, -1.0], 1.0));
    cut_and_joined(Section::circle(on_the_boss(0.3, 0.2), 5.0));
}

#[test]
fn a_circle_touching_an_elliptic_boss_outside_or_inside_at_a_major_vertex_and_crossing_it_twice_holds_its_volume()
 {
    cut_and_joined(Section::circle(on_the_boss(9.0, 0.0), 2.0));
    cut_and_joined(Section::circle(on_the_boss(4.0, 0.0), 3.0));
}

#[test]
fn two_ellipses_crossing_at_four_rulings_or_touching_outside_hold_their_volume() {
    let boss = boss();
    let u = boss.first.normalize();
    let v = boss.second.normalize();
    cut_and_joined(Section::ellipse(on_the_boss(0.0, 0.0), v * 6.0, 2.0));
    cut_and_joined(Section::ellipse(on_the_boss(9.0, 0.0), u * 2.0, 1.0));
}

#[test]
fn the_same_ellipse_a_ten_thousandth_off_is_built_crossing_it() {
    let boss = boss();
    let moved = Section {
        center: boss.center + DVec2::new(1e-4, 0.0),
        ..boss
    };
    cut_and_joined(moved);
}

#[test]
fn the_same_ellipse_a_hair_off_in_its_centre_its_half_axes_or_its_turn_is_one_wall() {
    let boss = boss();
    let stock = block([-10.0, -10.0, 0.0], [15.0, 10.0, 4.0]);
    let hole = stock
        .cut_by(&raised(boss, -1.0, 5.0))
        .expect("the hole is cut");
    let hair = 5.0 * Scale::of(10.0).eps();
    for again in [
        Section {
            center: boss.center + DVec2::new(hair, -hair),
            ..boss
        },
        Section::ellipse(
            boss.center.to_array(),
            boss.first.normalize() * (7.0 + hair),
            3.0 - hair,
        ),
        Section::ellipse(
            boss.center.to_array(),
            DVec2::from_angle(0.4 + hair / 7.0) * 7.0,
            3.0,
        ),
    ] {
        let twice = hole
            .cut_by(&raised(again, -2.0, 6.0))
            .expect("the same hole is cut again");
        held(&twice, 500.0 * 4.0 - boss.area() * 4.0);
        let walls = twice
            .surfaces
            .iter()
            .filter(|surface| matches!(surface, Surface::Elliptic(_)))
            .count();
        assert_eq!(walls, 1, "{again:?}");
        let joined = raised(boss, 0.0, 4.0)
            .joined(&raised(again, 0.0, 4.0))
            .expect("the same boss is joined again");
        held(&joined, boss.area() * 4.0);
    }
}

#[test]
fn a_round_pin_in_an_elliptic_pocket_touching_it_across_its_minor_axis_holds_its_volume() {
    let boss = boss();
    let stock = block([-10.0, -10.0, 0.0], [15.0, 10.0, 4.0]);
    let pocket = stock
        .cut_by(&raised(boss, 2.0, 5.0))
        .expect("the pocket is cut");
    let pin = Section::circle(on_the_boss(0.0, 0.0), 3.0);
    let pinned = pocket
        .joined(&raised(pin, 2.0, 4.0))
        .expect("the pin is joined in the pocket along the two rulings it touches");
    held(&pinned, 500.0 * 4.0 - boss.area() * 2.0 + pin.area() * 2.0);
}

#[test]
fn a_circle_or_an_ellipse_touching_an_elliptic_boss_from_inside_holds_its_volume() {
    let boss = boss();
    let curvature = 3.0 * 3.0 / 7.0;
    let tighter = curvature / 2.0;
    cut_and_joined(Section::circle(on_the_boss(7.0 - tighter, 0.0), tighter));
    cut_and_joined(Section::circle(
        on_the_boss(7.0 - curvature, 0.0),
        curvature,
    ));
    let u = boss.first.normalize();
    cut_and_joined(Section::ellipse(on_the_boss(3.5, 0.0), u * 3.5, 1.5));
}

#[test]
fn an_ellipse_a_hair_from_round_cut_by_the_circle_inscribed_in_it_leaves_two_crescents() {
    let oval = Section::ellipse([3.0, 7.0], DVec2::X * 2.5, 2.49975);
    let circle = Section::circle([3.0, 7.0], 2.49975);
    let cut = raised(oval, 8.0, 12.0)
        .cut_by(&raised(circle, 9.0, 17.0))
        .expect("the circle is cut out of the ellipse, touching it across its minor axis");
    held(&cut, oval.area() * 4.0 - circle.area() * 3.0);
}

#[test]
fn a_round_post_a_hair_off_touching_an_elliptic_boss_is_moved_onto_the_touch() {
    let boss = boss();
    let body = raised(boss, 0.0, 4.0);
    let eps = body.scale().joined(Scale::of(13.0)).eps();
    for hair in [eps / 3.0, -eps / 3.0] {
        let post = Section::circle(on_the_boss(9.0 + hair, 0.0), 2.0);
        let joined = body
            .joined(&raised(post, 0.0, 4.0))
            .expect("the post is joined along the ruling it touches");
        held(&joined, (boss.area() + post.area()) * 4.0);
        let [round, oval] = [
            |surface: &&Surface| matches!(surface, Surface::Cylinder(_)),
            |surface: &&Surface| matches!(surface, Surface::Elliptic(_)),
        ]
        .map(|kind| *joined.surfaces.iter().find(kind).expect("both walls stand"));
        let feet = crate::brep::relation::feet_of_touches(&round, &oval, joined.scale());
        let [[on_round, on_oval]] = feet[..] else {
            panic!("the two walls touch along one ruling, not {feet:?}");
        };
        let off = on_round.distance(on_oval);
        assert!(off <= 1e-12 * 13.0, "the touch stands {off} off exact");
    }
}

#[test]
fn a_post_under_an_elliptic_boss_touching_its_wall_seen_from_above_is_drawn_within_the_tolerance() {
    let boss = Section::ellipse([9.0, 0.0], DVec2::X * 5.0, 3.0);
    let post = Section::circle([9.0, 0.5], 2.5);
    let both = raised(boss, 0.0, 9.0)
        .joined(&raised(post, -4.0, -1.0))
        .expect("the two prisms are joined, standing apart");
    held(&both, boss.area() * 9.0 + post.area() * 3.0);
    let Some(Surface::Cylinder(round)) = both
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Cylinder(_)))
    else {
        panic!("the post stands on a circular wall");
    };
    for share in [1e-4, 1e-3, 2e-2] {
        let tolerance = share * both.scale().reach();
        for triangle in both.triangles(tolerance) {
            let across = (triangle[1] - triangle[0]).cross(triangle[2] - triangle[0]);
            if across.normalize().dot(DVec3::Z).abs() < 0.5
                && triangle
                    .iter()
                    .all(|corner| round.distance(*corner).abs() <= 1e-9)
            {
                for (one, other) in [(0, 1), (1, 2), (2, 0)] {
                    let middle = (triangle[one] + triangle[other]) / 2.0;
                    assert!(
                        round.distance(middle).abs() <= tolerance,
                        "a chord stands {} off the post at {share}",
                        round.distance(middle)
                    );
                }
            }
        }
    }
}

#[test]
fn a_circle_touching_an_elliptic_boss_at_a_vertex_and_crossing_it_near_there_holds_its_volume() {
    let curvature = 3.0 * 3.0 / 7.0;
    for radius in [curvature * 1.01, curvature * 1.1] {
        cut_and_joined(Section::circle(on_the_boss(7.0 - radius, 0.0), radius));
    }
}

/// The chords of the triangles a body's circular wall is drawn with, each
/// held within the tolerance of the wall, from a fine tolerance to a coarse
/// one.
fn round_drawn_within_the_tolerance(body: &Body) {
    let Some(Surface::Cylinder(round)) = body
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Cylinder(_)))
    else {
        panic!("the body stands on a circular wall");
    };
    for share in [1e-4, 1e-3, 2e-2] {
        let tolerance = share * body.scale().reach();
        let triangles = body.triangles(tolerance);
        assert_eq!(closed(&triangles).map_err(|_| share), Ok(()), "open");
        assert_eq!(uncrossed(&triangles).map_err(|_| share), Ok(()), "crossed");
        for triangle in triangles {
            let across = (triangle[1] - triangle[0]).cross(triangle[2] - triangle[0]);
            if across.normalize().dot(DVec3::Z).abs() < 0.5
                && triangle
                    .iter()
                    .all(|corner| round.distance(*corner).abs() <= 1e-9)
            {
                for (one, other) in [(0, 1), (1, 2), (2, 0)] {
                    let middle = (triangle[one] + triangle[other]) / 2.0;
                    assert!(
                        round.distance(middle).abs() <= tolerance,
                        "a chord stands {} off the round wall at {share}",
                        round.distance(middle)
                    );
                }
            }
        }
    }
}

#[test]
fn a_circle_inscribed_where_an_elliptic_boss_s_wall_was_cut_away_is_drawn_within_the_tolerance() {
    let boss = raised(Section::ellipse([0.0, 0.0], DVec2::X * 6.0, 3.0), 0.0, 4.0);
    let trimmed = boss
        .cut_by(&block([-10.0, 2.0, -1.0], [10.0, 10.0, 5.0]))
        .and_then(|body| body.cut_by(&block([-10.0, -10.0, -1.0], [10.0, -2.0, 5.0])))
        .expect("the boss is trimmed across its minor axis");
    let joined = trimmed
        .joined(&raised(Section::circle([0.0, 0.0], 3.0), 0.0, 4.0))
        .expect("the circle is joined where the wall it touches is gone");
    assert_eq!(listed(&joined.listing(), joined.scale().reach()), Ok(()));
    round_drawn_within_the_tolerance(&joined);
}

/// A cap of `oval`, the arc of its ellipse from the eccentric angle −2.5
/// round by 5 and the trait back, raised from `from` to `to`.
fn raised_cap(oval: Section, from: f64, to: f64) -> Body {
    let (first, other) = (oval.first, oval.second.length());
    let outline = Contour {
        corners: vec![oval.point(-2.5), oval.point(2.5)],
        runs: vec![
            Run::Oval {
                center: oval.center,
                first,
                second: other,
                turn: 5.0,
            },
            Run::Straight,
        ],
    };
    Body::raised(&outline, &[], ground(from), DVec3::Z * (to - from)).expect("a cap raises")
}

#[test]
fn an_arc_of_an_ellipse_a_hair_from_round_cut_by_a_circle_a_hair_off_it_is_declined_rather_than_kept()
 {
    let oval = Section::ellipse([120.0, 150.0], DVec2::Y * 15.0000015, 15.0);
    let circle = Section::circle([119.9999997, 150.0], 15.0000005);
    let body = raised_cap(oval, 15.0, 150.0);
    assert!(
        body.surfaces
            .iter()
            .any(|surface| matches!(surface, Surface::Elliptic(_))),
        "the arc is raised on an elliptic wall"
    );
    let cut = body.cut_by(&raised(circle, 15.0, 149.999994));
    assert_eq!(cut.map(|_| ()), Err(Declined::Unsupported));
}

/// The body `step` makes of a prism of `oval` and a prism of the circle
/// of its half-axes' mean, each the same: one is made as the other is,
/// with no elliptic wall left.
fn made_as_the_circle_of_its_mean(oval: Section, step: impl Fn(Body) -> Body) {
    let mean = (oval.first.length() + oval.second.length()) / 2.0;
    let round = step(raised(
        Section::circle(oval.center.to_array(), mean),
        0.0,
        4.0,
    ));
    let made = step(raised(oval, 0.0, 4.0));
    assert!(
        made.surfaces
            .iter()
            .all(|surface| !matches!(surface, Surface::Elliptic(_))),
        "an elliptic wall is left"
    );
    assert_eq!(made.faces.len(), round.faces.len());
    assert_eq!(made.edges.len(), round.edges.len());
    assert_eq!(made.vertices.len(), round.vertices.len());
    assert!(
        (made.volume() - round.volume()).abs() <= 1e-9 * round.volume(),
        "the body holds {}, two circles {}",
        made.volume(),
        round.volume()
    );
    held(&made, round.volume());
}

#[test]
fn a_whole_ellipse_a_hair_from_round_is_joined_and_cut_by_a_circle_of_its_size_as_two_circles_are()
{
    let eps = Scale::of(165.0).eps();
    let oval = Section::ellipse(
        [120.0, 150.0],
        DVec2::Y * (15.0 + 5.0 * eps),
        15.0 - 5.0 * eps,
    );
    let size = Section::circle([120.0, 150.0], 15.0);
    let off = Section::circle([119.9999997, 150.0], 15.0000005);
    made_as_the_circle_of_its_mean(oval, |body| {
        body.joined(&raised(size, 2.0, 6.0))
            .expect("the circle of its size is joined")
    });
    made_as_the_circle_of_its_mean(oval, |body| {
        body.cut_by(&raised(off, 1.0, 3.0))
            .expect("the circle a hair off is cut")
    });
}

#[test]
fn two_ellipses_turned_a_few_degrees_apart_crossing_at_four_rulings_hold_their_volume() {
    let boss = boss();
    for degrees in [3.5f64, 12.0] {
        let turned = DVec2::from_angle(0.4 + degrees.to_radians()) * 7.0;
        cut_and_joined(Section::ellipse(boss.center.to_array(), turned, 3.0));
    }
    let rounder = Section::ellipse(boss.center.to_array(), DVec2::from_angle(0.46) * 5.0, 4.0);
    let body = raised(
        Section::ellipse(boss.center.to_array(), DVec2::from_angle(0.4) * 5.0, 4.0),
        0.0,
        4.0,
    );
    let joined = body
        .joined(&raised(rounder, -1.0, 3.0))
        .expect("the turned ellipse is joined");
    let base = Section::ellipse(boss.center.to_array(), DVec2::from_angle(0.4) * 5.0, 4.0);
    held(
        &joined,
        base.area() * 4.0 + rounder.area() * 4.0 - base.overlap(&rounder) * 3.0,
    );
}

/// Both feet of the touches the walls of `body` meet along, exact.
fn touching_exactly_at_two(body: &Body) {
    let [round, oval] = [
        |surface: &&Surface| matches!(surface, Surface::Cylinder(_)),
        |surface: &&Surface| matches!(surface, Surface::Elliptic(_)),
    ]
    .map(|kind| *body.surfaces.iter().find(kind).expect("both walls stand"));
    let feet = crate::brep::relation::feet_of_touches(&round, &oval, body.scale());
    assert_eq!(feet.len(), 2, "the two walls touch along two rulings");
    for [on_round, on_oval] in feet {
        let off = on_round.distance(on_oval);
        assert!(
            off <= 1e-12 * body.scale().reach(),
            "a touch stands {off} off exact"
        );
    }
}

#[test]
fn an_elliptic_hole_a_hair_from_touching_a_round_bore_at_both_ends_of_its_major_axis_is_grown_onto_both()
 {
    let bore = Section::circle([6.0, 1.0], 2.0);
    let eps = Scale::of(10.0).eps();
    let way = DVec2::new(-0.92, -0.39).normalize();
    for hair in [-eps * 0.9, -eps / 3.0, eps / 3.0] {
        let hole = Section::ellipse([6.0, 1.0], way * (2.0 + hair), 1.5);
        let cut = raised(bore, 2.0, 10.0)
            .cut_by(&raised(hole, 2.0, 9.5))
            .expect("the hole is cut, touching the bore at both ends of its major axis");
        let grown = Section::ellipse([6.0, 1.0], way * 2.0, 1.5);
        held(&cut, bore.area() * 8.0 - grown.area() * 7.5);
        touching_exactly_at_two(&cut);
    }
}

#[test]
fn a_round_pin_a_hair_from_touching_an_elliptic_pocket_across_its_minor_axis_is_grown_onto_both_sides()
 {
    let pocket = Section::ellipse([3.0, 7.0], DVec2::from_angle(0.3) * 2.5, 1.5);
    let eps = Scale::of(17.0).eps();
    for hair in [-eps / 2.0, eps / 3.0] {
        let pin = Section::circle([3.0, 7.0], 1.5 + hair);
        let joined = raised(pocket, 8.0, 12.0)
            .joined(&raised(pin, 9.0, 17.0))
            .expect("the pin is joined, touching the pocket across its minor axis");
        let grown = Section::circle([3.0, 7.0], 1.5);
        held(&joined, pocket.area() * 4.0 + grown.area() * 5.0);
        touching_exactly_at_two(&joined);
    }
}

#[test]
fn an_ellipse_a_hair_from_round_turned_a_ten_thousandth_joins_itself_as_one_wall() {
    let oval = Section::ellipse([3.0, 7.0], DVec2::X * 5.001, 5.0);
    let turned = Section::ellipse([3.0, 7.0], DVec2::from_angle(1e-4) * 5.001, 5.0);
    let joined = raised(oval, 0.0, 4.0)
        .joined(&raised(turned, -1.0, 3.0))
        .expect("the turned ellipse is the same wall within the hair");
    held(&joined, oval.area() * 5.0);
    let walls = joined
        .surfaces
        .iter()
        .filter(|surface| matches!(surface, Surface::Elliptic(_)))
        .count();
    assert_eq!(walls, 1);
}

#[test]
fn an_ellipse_cut_by_itself_turned_a_ten_thousandth_of_a_turn_leaves_four_slivers() {
    let first = DVec2::new(1.9318516525781362, -0.5176380902050413);
    let oval = Section::ellipse([7.0, 10.0], first, 5.0);
    let turned = Section::ellipse(
        [7.0, 10.0],
        DVec2::new(1.9319034067278127, -0.517444902451915),
        5.0,
    );
    let cut = raised(oval, 8.0, 14.0)
        .cut_by(&raised(turned, 8.0, 14.0))
        .expect("the turned ellipse is cut out, crossing it at four rulings");
    held(&cut, (oval.area() - oval.overlap(&turned)) * 6.0);
}

#[test]
fn an_ellipse_cut_by_a_smaller_one_reaching_a_millionth_past_its_end_leaves_two_rims_crossing() {
    let oval = Section::ellipse([25.0, 10.0], DVec2::new(-12.5, 0.0), 22.5);
    let inner = Section::ellipse([25.0, 4.374999], DVec2::new(-9.375, 0.0), 16.875);
    let cut = raised(oval, 0.0, 47.5)
        .cut_by(&raised(inner, 0.0, 47.5))
        .expect("the smaller ellipse is cut out, crossing the other twice beside its end");
    held(&cut, (oval.area() - oval.overlap(&inner)) * 47.5);
}

#[test]
fn an_elliptic_pocket_touching_a_boss_s_end_inside_beside_a_block_reaching_both_walls_is_drawn_closed()
 {
    let boss = Section::ellipse(
        [322.5, 202.5],
        DVec2::new(-19.48557158514987, -11.250000000000002),
        67.5,
    );
    let pocket = Section::ellipse(
        [330.9375, 187.8858213111376],
        DVec2::new(-14.614178688862403, -8.437500000000002),
        50.625,
    );
    let body = block([300.0, 180.0, 90.0], [525.0, 225.0, 195.0])
        .joined(&raised(boss, 60.0, 105.0))
        .expect("the boss is joined to the block");
    let cut = body
        .cut_by(&raised(pocket, 30.00001, 60.00001))
        .expect("the pocket is cut, touching the boss's wall at the end of its major axis");
    drawn(&cut);
}

/// The body's listing, and its triangles closed and uncrossed from a fine
/// tolerance to a coarse one.
fn drawn(body: &Body) {
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    for share in [1e-4, 1e-3, 2e-2] {
        let triangles = body.triangles(share * body.scale().reach());
        assert_eq!(closed(&triangles).map_err(|_| share), Ok(()), "open");
        assert_eq!(uncrossed(&triangles).map_err(|_| share), Ok(()), "crossed");
    }
}

#[test]
fn two_ellipses_touching_at_both_ends_of_their_one_half_axis_stacked_under_a_round_cut_are_drawn_from_one_point()
 {
    let way = DVec2::new(-0.7764571353075626, 2.897777478867205);
    let upper = Section::ellipse([8.0, 1.0], way, 5.5);
    let lower = Section::ellipse([8.0, 1.0], way, 6.0);
    let round = Section::circle([6.0, 3.0], 3.0);
    let cut = raised(upper, 2.0, 12.0)
        .cut_by(&raised(round, 4.0, 12.0))
        .expect("the round is cut out of the upper ellipse");
    let joined = cut
        .joined(&raised(lower, -7.0, 2.0))
        .expect("the lower ellipse is joined under it, touching it at both ends of a half-axis");
    held(
        &joined,
        upper.area() * 10.0 - upper.overlap(&round) * 8.0 + lower.area() * 9.0,
    );
}

#[test]
fn a_circle_and_two_ellipses_no_point_stands_inside_all_three_of_are_drawn_closed() {
    let round = raised(Section::circle([5.0, 7.5], 2.0), 4.0, 5.0);
    let narrow = Section::ellipse(
        [3.5012387797481557, 7.5609442751146005],
        DVec2::new(-0.4995871400289, 0.020314761080168248),
        2.0,
    );
    let wide = Section::ellipse(
        [6.5, 7.5],
        DVec2::new(0.08125904432067299, 1.9983485601156),
        2.5,
    );
    let joined = round
        .joined(&raised(narrow, 3.99999999, 4.99999999))
        .and_then(|body| body.joined(&raised(wide, 3.99999989, 4.99999989)))
        .expect("the three prisms are joined");
    drawn(&joined);
}
