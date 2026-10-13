//! Two walls square to each other, an elliptic one among them (#571): a
//! round hole drilled from the side through an elliptic boss, an elliptic
//! hole through a round shaft or an elliptic boss, and bars crossing a boss
//! partly, touching it inside at a node, of its half-width through its axis,
//! and touching it outside at a point — then each a hair off its touch. Each
//! held to its listing, to the arithmetic's volume and drawn closed and
//! uncrossed from a fine tolerance to a coarse one.
//!
//! The arithmetic: a bar along X crossing a wall standing on Z cuts, at each
//! height `y` both reach, a chord of the wall's section `2α√(w² − y²)` long,
//! `w` how far the section reaches along Y and `α = AB/w²` — a circle's
//! chord when `A = B = w` — and the bar's chord across it likewise. The
//! matter the two share is `α α'` times what two circular walls of radii
//! `w` and `w'` share.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::Meet;
use crate::brep::surface::EllipticCylinder;
use crate::brep::tessellation::tests::across::common;
use crate::soundness::{closed, uncrossed};

/// An elliptic section, about its centre, `first` its first half-axis, the
/// other `second` long.
#[derive(Clone, Copy)]
struct Section {
    center: DVec2,
    first: DVec2,
    second: f64,
}

impl Section {
    fn round(center: [f64; 2], radius: f64) -> Section {
        Section {
            center: DVec2::from(center),
            first: DVec2::X * radius,
            second: radius,
        }
    }

    fn oval(center: [f64; 2], first: DVec2, second: f64) -> Section {
        Section {
            center: DVec2::from(center),
            first,
            second,
        }
    }

    fn contour(&self) -> Contour {
        let run = if self.first.length() == self.second {
            Run::Round {
                center: self.center,
                turn: TAU,
            }
        } else {
            Run::Oval {
                center: self.center,
                first: self.first,
                second: self.second,
                turn: TAU,
            }
        };
        Contour {
            corners: vec![self.center + self.first],
            runs: vec![run],
        }
    }

    fn area(&self) -> f64 {
        PI * self.first.length() * self.second
    }

    /// How far the section reaches along `way`, one of its two coordinates,
    /// and what a unit of a chord's root across it is worth: `w` and `α`.
    fn across(&self, way: DVec2) -> (f64, f64) {
        let (major, minor) = (self.first.length(), self.second);
        let along = self.first / major;
        let reach = (major * along.dot(way)).hypot(minor * along.perp_dot(way));
        (reach, major * minor / (reach * reach))
    }
}

/// A prism of `section` drawn on the ground and raised from `from` to `to`.
fn standing_up(section: Section, from: f64, to: f64) -> Body {
    Body::raised(
        &section.contour(),
        &[],
        ground(from),
        DVec3::Z * (to - from),
    )
    .expect("a wall stands")
}

/// A bar of `section`, drawn in `(y, z)` on the plane `x = from` and raised
/// along X to `x = to`.
fn lying(section: Section, from: f64, to: f64) -> Body {
    let frame = Frame {
        origin: DVec3::X * from,
        u: DVec3::Y,
        v: DVec3::Z,
    };
    Body::raised(&section.contour(), &[], frame, DVec3::X * (to - from)).expect("a bar lies")
}

/// What a bar of `bar`, lying along X right through a wall of `wall`
/// standing on Z, shares with it.
fn shared(wall: Section, bar: Section) -> f64 {
    let (reach, scale) = wall.across(DVec2::Y);
    let (bar_reach, bar_scale) = bar.across(DVec2::X);
    let offset = bar.center.x - wall.center.y;
    scale * bar_scale * common(reach, bar_reach, offset)
}

fn held(body: &Body, volume: f64) {
    assert!(
        (body.volume() - volume).abs() <= 1e-9 * volume,
        "the body holds {}, the arithmetic {volume}",
        body.volume()
    );
    drawn(body);
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

/// The curves the body's edges run along where two walls meet square, an
/// elliptic one among them.
fn meets(body: &Body) -> Vec<(Meet, Option<[VertexId; 2]>)> {
    body.edges
        .iter()
        .filter_map(|edge| match body.curve(edge.curve) {
            Curve::Meet(meet) => Some((*meet, edge.ends)),
            _ => None,
        })
        .collect()
}

fn elliptic(meet: &Meet) -> bool {
    use crate::brep::curve::Wall;
    matches!(meet.first, Wall::Elliptic(_)) || matches!(meet.second, Wall::Elliptic(_))
}

/// The boss every case below crosses: half-axes 7 along X and 4 along Y,
/// turned by `turned`, from the ground up to `height`.
fn boss(turned: f64) -> Section {
    Section::oval([0.0, 0.0], DVec2::from_angle(turned) * 7.0, 4.0)
}

const HEIGHT: f64 = 10.0;

fn boss_volume(section: Section, height: f64) -> f64 {
    section.area() * height
}

#[test]
fn a_round_hole_drilled_from_the_side_through_an_elliptic_boss_is_cut_along_two_loops() {
    for turned in [0.0, 0.4] {
        let wall = boss(turned);
        let hole = Section::round([1.0, 5.0], 2.0);
        let drilled = standing_up(wall, 0.0, HEIGHT)
            .cut_by(&lying(hole, -10.0, 10.0))
            .expect("the hole is drilled");
        held(&drilled, boss_volume(wall, HEIGHT) - shared(wall, hole));
        let loops = meets(&drilled);
        assert_eq!(loops.len(), 2, "{turned}: {loops:?}");
        assert!(
            loops
                .iter()
                .all(|(meet, ends)| ends.is_none() && elliptic(meet))
        );
    }
}

#[test]
fn an_elliptic_hole_drilled_from_the_side_through_a_round_shaft_holds_its_volume() {
    let shaft = Section::round([0.0, 0.0], 5.0);
    for (center, first, second) in [
        ([0.5, 6.0], DVec2::from_angle(0.5) * 3.0, 1.5),
        ([-1.0, 6.0], DVec2::Y * 3.5, 2.0),
    ] {
        let hole = Section::oval(center, first, second);
        let drilled = standing_up(shaft, 0.0, 12.0)
            .cut_by(&lying(hole, -8.0, 8.0))
            .expect("the hole is drilled");
        held(&drilled, boss_volume(shaft, 12.0) - shared(shaft, hole));
        assert_eq!(meets(&drilled).len(), 2);
    }
}

#[test]
fn an_elliptic_hole_drilled_from_the_side_through_an_elliptic_boss_holds_its_volume() {
    let wall = boss(0.3);
    let hole = Section::oval([0.5, 5.0], DVec2::from_angle(-0.7) * 2.5, 1.2);
    let drilled = standing_up(wall, 0.0, HEIGHT)
        .cut_by(&lying(hole, -10.0, 10.0))
        .expect("the hole is drilled");
    held(&drilled, boss_volume(wall, HEIGHT) - shared(wall, hole));
    assert_eq!(meets(&drilled).len(), 2);
}

#[test]
fn a_round_bar_crossing_an_elliptic_boss_partly_meets_it_along_one_loop() {
    let wall = boss(0.0);
    let bar = Section::round([3.0, 5.0], 2.0);
    let column = standing_up(wall, 0.0, HEIGHT);
    let rod = lying(bar, -10.0, 10.0);
    let cut = column.cut_by(&rod).expect("the bar is cut away");
    held(&cut, boss_volume(wall, HEIGHT) - shared(wall, bar));
    let joined = column.joined(&rod).expect("the bar is joined");
    held(
        &joined,
        boss_volume(wall, HEIGHT) + bar.area() * 20.0 - shared(wall, bar),
    );
    assert_eq!(meets(&joined).len(), 1);
}

#[test]
fn a_round_bar_touching_an_elliptic_boss_from_inside_meets_it_along_a_figure_of_eight() {
    let wall = boss(0.0);
    for d in [2.0, -2.0] {
        let bar = Section::round([d, 5.0], 2.0);
        let column = standing_up(wall, 0.0, HEIGHT);
        let rod = lying(bar, -10.0, 10.0);
        let cut = column.cut_by(&rod).expect("the bar is cut away");
        held(&cut, boss_volume(wall, HEIGHT) - shared(wall, bar));
        let joined = column.joined(&rod).expect("the bar is joined");
        held(
            &joined,
            boss_volume(wall, HEIGHT) + bar.area() * 20.0 - shared(wall, bar),
        );
    }
}

#[test]
fn a_round_bar_of_an_elliptic_boss_s_half_width_through_its_axis_meets_it_along_two_ellipses() {
    let wall = boss(0.0);
    let bar = Section::round([0.0, 10.0], 4.0);
    let column = standing_up(wall, 0.0, 20.0);
    let joined = column
        .joined(&lying(bar, -10.0, 10.0))
        .expect("the bar is joined");
    held(
        &joined,
        boss_volume(wall, 20.0) + bar.area() * 20.0 - shared(wall, bar),
    );
}

#[test]
fn a_round_bar_of_an_elliptic_boss_s_half_width_through_its_axis_cuts_it_down_to_two_nodes() {
    let wall = boss(0.0);
    let bar = Section::round([0.0, 10.0], 4.0);
    let cut = standing_up(wall, 0.0, 20.0)
        .cut_by(&lying(bar, -10.0, 10.0))
        .expect("the bar is cut away");
    held(&cut, boss_volume(wall, 20.0) - shared(wall, bar));
}

#[test]
fn an_elliptic_bar_crossing_an_elliptic_boss_partly_or_touching_it_inside_holds_its_volume() {
    let wall = boss(0.0);
    for (center, first, second) in [
        ([2.5, 5.0], DVec2::X * 2.0, 1.5),
        ([1.5, 5.0], DVec2::from_angle(0.6) * 3.0, 2.0),
        ([2.0, 5.0], DVec2::Y * 2.5, 2.0),
    ] {
        let bar = Section::oval(center, first, second);
        let column = standing_up(wall, 0.0, HEIGHT);
        let rod = lying(bar, -10.0, 10.0);
        let cut = column.cut_by(&rod).expect("the bar is cut away");
        held(&cut, boss_volume(wall, HEIGHT) - shared(wall, bar));
        let joined = column.joined(&rod).expect("the bar is joined");
        held(
            &joined,
            boss_volume(wall, HEIGHT) + bar.area() * 20.0 - shared(wall, bar),
        );
    }
}

#[test]
fn a_pocket_drilled_from_the_side_up_to_an_elliptic_boss_s_axis_takes_half_of_what_a_hole_does() {
    let wall = boss(0.0);
    for pocket in [
        Section::round([1.0, 5.0], 2.0),
        Section::oval([-0.5, 5.0], DVec2::X * 2.5, 1.5),
    ] {
        let drilled = standing_up(wall, 0.0, HEIGHT)
            .cut_by(&lying(pocket, -10.0, 0.0))
            .expect("the pocket is drilled");
        held(
            &drilled,
            boss_volume(wall, HEIGHT) - shared(wall, pocket) / 2.0,
        );
    }
}

#[test]
fn two_pockets_drilled_from_either_side_of_an_elliptic_boss_to_one_plane_take_what_one_hole_does() {
    for (turned, end) in [(0.0, 6.0), (0.4, 5.0), (0.4, -2.0)] {
        let wall = boss(turned);
        let hole = Section::round([1.0, 5.0], 2.0);
        let drilled = standing_up(wall, 0.0, HEIGHT)
            .cut_by(&lying(hole, -10.0, end))
            .and_then(|once| once.cut_by(&lying(hole, end, 10.0)))
            .expect("the pockets are drilled");
        held(&drilled, boss_volume(wall, HEIGHT) - shared(wall, hole));
    }
}

#[test]
fn a_bar_lying_on_a_boss_s_floor_a_hair_inside_a_tangent_of_its_rim_is_joined_and_cut() {
    let shaft = Section::round([0.0, 0.0], 5.0);
    let pairs = [0.0, 0.4]
        .map(|turned| {
            let wall = boss(turned);
            let (reach, _) = wall.across(DVec2::Y);
            (wall, Section::round([reach - 1e-5, 2.0], 2.0))
        })
        .into_iter()
        .chain([(shaft, Section::oval([5.0 - 1e-5, 1.5], DVec2::X * 2.5, 1.5))]);
    for (wall, bar) in pairs {
        let column = standing_up(wall, 0.0, HEIGHT);
        let rod = lying(bar, -10.0, 10.0);
        let cut = column.cut_by(&rod).expect("the bar is cut away");
        held(&cut, boss_volume(wall, HEIGHT) - shared(wall, bar));
        let joined = column.joined(&rod).expect("the bar is joined");
        held(
            &joined,
            boss_volume(wall, HEIGHT) + bar.area() * 20.0 - shared(wall, bar),
        );
    }
}

#[test]
fn a_round_bar_touching_an_elliptic_boss_from_outside_meets_it_at_one_point() {
    let wall = boss(0.0);
    let bar = Section::round([6.0, 5.0], 2.0);
    let column = standing_up(wall, 0.0, HEIGHT);
    let rod = lying(bar, -10.0, 10.0);
    let joined = column.joined(&rod).expect("the bar is joined");
    held(&joined, boss_volume(wall, HEIGHT) + bar.area() * 20.0);
    let cut = column.cut_by(&rod).expect("the bar is cut away");
    held(&cut, boss_volume(wall, HEIGHT));
}

#[test]
fn the_same_walls_a_hair_off_each_touch_are_built_on_the_touch() {
    let hair = Scale::of(20.0).eps() / 3.0;
    let wall = boss(0.0);
    let column = standing_up(wall, 0.0, 20.0);
    for (bar, joined_volume) in [
        (Section::round([2.0 + hair, 10.0], 2.0), None),
        (Section::round([-2.0 - hair, 10.0], 2.0), None),
        (Section::round([hair, 10.0], 4.0 + hair), None),
        (Section::round([0.0, 10.0], 4.0 - hair), None),
        (
            Section::round([6.0 - hair, 10.0], 2.0),
            Some(boss_volume(wall, 20.0) + PI * 4.0 * 20.0),
        ),
        (
            Section::round([6.0 + hair, 10.0], 2.0),
            Some(boss_volume(wall, 20.0) + PI * 4.0 * 20.0),
        ),
    ] {
        let rod = lying(bar, -10.0, 10.0);
        let promised = joined_volume
            .unwrap_or(boss_volume(wall, 20.0) + bar.area() * 20.0 - shared(wall, bar));
        let joined = column.joined(&rod).expect("the bar is joined");
        assert!(
            (joined.volume() - promised).abs() <= 1e-8 * promised,
            "{}: {} against {promised}",
            bar.center,
            joined.volume()
        );
        drawn(&joined);
        let cut = column.cut_by(&rod).expect("the bar is cut away");
        drawn(&cut);
    }
}

#[test]
fn an_elliptic_wall_square_to_another_is_related_along_the_curve_they_meet_along() {
    let column = standing_up(boss(0.4), 0.0, HEIGHT);
    let rod = lying(Section::oval([0.5, 5.0], DVec2::X * 2.0, 1.0), -10.0, 10.0);
    let walls = |body: &Body| {
        body.surfaces
            .iter()
            .copied()
            .find(|surface| matches!(surface, Surface::Elliptic(_)))
            .expect("an elliptic wall")
    };
    let scale = Scale::of(20.0);
    let met = relation(&walls(&column), &walls(&rod), scale);
    let Relation::Meet(meeting) = met else {
        panic!("{met:?}")
    };
    assert_eq!(meeting.components.len(), 2);
    let Surface::Elliptic(far) = walls(&rod) else {
        unreachable!("the rod is elliptic")
    };
    let apart = Surface::Elliptic(EllipticCylinder {
        origin: far.origin + DVec3::Y * 20.0,
        ..far
    });
    assert_eq!(relation(&walls(&column), &apart, scale), Relation::Apart);
}
