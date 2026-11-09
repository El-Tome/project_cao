//! An arc of a circle far wider than the part, drawn and written within the
//! tolerance asked rather than in the steps the cap cuts a whole turn into.
//!
//! cargo-mutants leaves one mutant of `steps::most` alive, `>` for `>=`
//! against 1024: at 1024 both branches return 1024, so it is equivalent.
//!
//! Closes #777.
//! - A wall raised on the arc of radius 3437 over half a degree, on a part 30
//!   wide, is drawn within `DRAWN` × reach of its arc, closed and uncrossed,
//!   and its triangles enclose its volume within the drawing tolerance —
//!   `a_wall_on_the_arc_of_radius_3437_over_half_a_degree_is_drawn_within_the_tolerance_closed_uncrossed_and_whole`
//! - The same at radius 1719 over one degree —
//!   `a_wall_on_the_arc_of_radius_1719_over_one_degree_is_drawn_within_the_tolerance_closed_uncrossed_and_whole`
//! - #697's near-flat sweep (`a_fillet_on_an_edge_nearly_flat_or_not_…`)
//!   checks what its triangles enclose, at every radius, including the
//!   largest — no test: the sweep is #697's and stays in
//!   `a_fillet_rounds_one_straight_edge.rs`, whose own `Closes #777` names it
//! - A circle no wider than the part is cut into the same number of steps as
//!   today —
//!   `a_circle_no_wider_than_the_part_is_drawn_and_written_in_as_many_triangles_as_before`
//! - On such a wide arc, an STL export at a fineness that needs more than 1024
//!   steps a turn holds that fineness; on an ordinary circle (no wider than
//!   the part), a fineness past the cap is declined as today (`Uncovered`),
//!   never written off the tolerance —
//!   `a_wall_on_an_arc_far_wider_than_the_part_written_to_stl_holds_a_fineness_past_a_thousand_and_twenty_four_steps_a_turn`,
//!   the same wall drawn far from the origin by
//!   `a_wall_on_an_arc_far_wider_than_the_part_drawn_far_from_the_origin_written_to_stl_holds_the_same_fineness`;
//!   `a_circle_no_wider_than_the_part_asked_finer_than_a_thousand_and_twenty_four_steps_a_turn_is_declined_as_uncovered`,
//!   and the same on a plane tilted so that no coordinate reaches the
//!   circle's radius by
//!   `a_disc_no_wider_than_its_part_on_a_tilted_plane_asked_finer_than_a_thousand_and_twenty_four_steps_a_turn_is_declined_as_uncovered`,
//!   an elliptic wall by
//!   `an_elliptic_post_no_wider_than_its_part_asked_finer_than_a_thousand_and_twenty_four_steps_a_turn_is_declined_as_uncovered`,
//!   and a wide arc asked finer than even its raised cap holds by
//!   `a_wall_on_an_arc_far_wider_than_the_part_asked_finer_than_even_its_raised_cap_is_declined_as_uncovered`
//!
//! The fifth criterion is read as the comment of 2026-11-09 on the issue
//! decided it with the human: the issue's own wording asked an ordinary circle
//! to hold a fineness past the cap too, and the cap now stays on it.

use std::f64::consts::TAU;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::stl::{self, Uncovered};
use cao_solid::{Body, Loop};
use glam::{DVec2, DVec3};

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far up the wall's part runs from its arc, and how high it is raised.
const DEPTH: f64 = 10.0;
const HEIGHT: f64 = 5.0;

/// The number the arc's wall answers to: the floor is nought, the top one,
/// and the walls from two, one for each run of the outline, the arc first.
const ARC_WALL: usize = 2;

const GROUND: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

/// The arc of a circle of `radius` about (0, `radius`) from the origin over
/// `degrees`, and the part above it.
struct OnAnArc {
    radius: f64,
    degrees: f64,
}

impl OnAnArc {
    fn center(&self) -> DVec2 {
        DVec2::new(0.0, self.radius)
    }

    fn turn(&self) -> f64 {
        self.degrees.to_radians()
    }

    fn end(&self) -> DVec2 {
        let turn = self.turn();
        DVec2::new(
            self.radius * turn.sin(),
            2.0 * self.radius * (turn / 2.0).sin().powi(2),
        )
    }

    /// The arc from the origin to its end, then straight up by [`DEPTH`],
    /// back across to over the origin, and down to it.
    fn outline(&self) -> Contour {
        let end = self.end();
        Contour {
            corners: vec![
                DVec2::ZERO,
                end,
                DVec2::new(end.x, DEPTH),
                DVec2::new(0.0, DEPTH),
            ],
            runs: vec![
                Run::Round {
                    center: self.center(),
                    turn: self.turn(),
                },
                Run::Straight,
                Run::Straight,
                Run::Straight,
            ],
        }
    }

    /// The outline raised by [`HEIGHT`], joined to an empty part as
    /// `cao_part` builds a step.
    fn raised(&self) -> Body {
        self.raised_at(DVec3::ZERO)
    }

    /// [`Self::raised`], drawn on the ground moved to `origin`.
    fn raised_at(&self, origin: DVec3) -> Body {
        let frame = Frame { origin, ..GROUND };
        on_a_part(raised(self.outline(), frame, HEIGHT))
    }

    /// The quadrilateral of the outline's corners, and the segment between
    /// the arc and its chord, which bows away from the center and adds to
    /// it: `r²/2 · (θ − sin θ)`.
    fn volume(&self) -> f64 {
        let end = self.end();
        let corners = [
            DVec2::ZERO,
            end,
            DVec2::new(end.x, DEPTH),
            DVec2::new(0.0, DEPTH),
        ];
        let quadrilateral = (0..4)
            .map(|index| corners[index].perp_dot(corners[(index + 1) % 4]))
            .sum::<f64>()
            / 2.0;
        let turn = self.turn();
        let segment = self.radius * self.radius / 2.0 * (turn - turn.sin());
        (quadrilateral + segment) * HEIGHT
    }

    fn wall_area(&self) -> f64 {
        self.radius * self.turn() * HEIGHT
    }

    fn off_the_arc(&self, point: DVec3) -> f64 {
        (point.truncate().distance(self.center()) - self.radius).abs()
    }

    /// How far a point stands from the nearest of the part's surfaces: its
    /// floor, its top, its arc and its three flat walls.
    fn off_every_surface(&self, point: DVec3) -> f64 {
        let end = self.end();
        [
            point.z,
            point.z - HEIGHT,
            point.x - end.x,
            point.y - DEPTH,
            point.x,
        ]
        .map(f64::abs)
        .into_iter()
        .fold(self.off_the_arc(point), f64::min)
    }
}

/// An outline drawn on `frame` and raised along its normal by `height`, as
/// the drawing hands it to the exact kernel.
fn raised(outline: Contour, frame: Frame, height: f64) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    Body::default()
        .tool_raised(&profile, frame, frame.normal() * height)
        .expect("the exact kernel raises it")
}

/// The tool joined to an empty part, as `cao_part` builds a step.
fn on_a_part(tool: Body) -> Body {
    Body::default()
        .union(&tool)
        .expect("the part takes the tool")
}

fn disc(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// A cylinder of radius 20 standing from 0 to 10, bored through on its axis
/// with a radius of 5: two circles, the wider as wide as the part.
fn bored_cylinder() -> Body {
    let below = Frame {
        origin: DVec3::NEG_Z,
        ..GROUND
    };
    on_a_part(raised(disc(DVec2::ZERO, 20.0), GROUND, 10.0))
        .difference(&raised(disc(DVec2::ZERO, 5.0), below, 12.0))
        .expect("the exact kernel bores it")
}

/// A cylinder of radius 2 m standing 10 mm high, as wide as the part: at
/// 0.01 mm a turn of it needs 996 steps, just under the 1024 the cap allows,
/// and at « Fine », 0.005 mm, 1405, past it.
fn wide_disc() -> Body {
    on_a_part(raised(disc(DVec2::ZERO, 2000.0), GROUND, 10.0))
}

/// The triangles the view draws and the STL writes, read from the code
/// before #777, for the cases of
/// `a_circle_no_wider_than_the_part_is_drawn_and_written_in_as_many_triangles_as_before`:
/// the disc's 3980 are 996 steps a turn, its 636 in the view 160.
const BEFORE: [[usize; 2]; 4] = [[960, 192], [960, 864], [636, 1260], [636, 3980]];

/// How many triangles a binary STL holds, read from its count.
fn written(bytes: &[u8]) -> usize {
    u32::from_le_bytes(bytes[80..84].try_into().expect("a count after the header")) as usize
}

/// The corners of every triangle of a binary STL, as a slicer reads them.
fn read(bytes: &[u8]) -> Vec<[DVec3; 3]> {
    let number = |at: usize| f64::from(f32::from_le_bytes(bytes[at..at + 4].try_into().unwrap()));
    let corner = |at: usize| DVec3::new(number(at), number(at + 4), number(at + 8));
    (0..written(bytes))
        .map(|index| {
            let at = 84 + 50 * index + 12;
            [corner(at), corner(at + 12), corner(at + 24)]
        })
        .collect()
}

/// The ground tilted square to (1, 1, 1), through the origin.
const TILTED: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::new(
        std::f64::consts::FRAC_1_SQRT_2,
        -std::f64::consts::FRAC_1_SQRT_2,
        0.0,
    ),
    v: DVec3::new(
        0.408_248_290_463_863,
        0.408_248_290_463_863,
        -0.816_496_580_927_726,
    ),
};

/// How far rounding every number of a binary STL to an `f32` may move a
/// point of its triangles, on a part reaching `reach` from the origin: half
/// an ulp on each coordinate, √3 of it across three.
fn written_rounding(reach: f64) -> f64 {
    f64::from(f32::EPSILON) * reach
}

/// Holds that the wall, drawn on the ground moved to `origin`, is written at
/// `gap` and every point of every triangle read back stands within `gap` of
/// one of its surfaces.
fn written_within(wall: &OnAnArc, origin: DVec3, gap: f64) {
    let what = format!(
        "the wall on the arc of {} over {}° drawn at {origin}, at {gap} mm",
        wall.radius, wall.degrees
    );
    let body = wall.raised_at(origin);
    let rounding = written_rounding(reach_of(&body));

    let bytes = stl::of(&body, gap, 1.0)
        .unwrap_or_else(|declined| panic!("{what}: {declined:?}, where it is written"));

    let triangles = read(&bytes);
    assert!(!triangles.is_empty(), "{what}: nothing was written");
    let mut furthest = 0.0f64;
    for corners in &triangles {
        for one in 0..=8 {
            for other in 0..=8 - one {
                let (a, b) = (one as f64 / 8.0, other as f64 / 8.0);
                let point = corners[0] * (1.0 - a - b) + corners[1] * a + corners[2] * b;
                furthest = furthest.max(wall.off_every_surface(point - origin));
            }
        }
    }
    assert!(
        furthest <= gap + rounding,
        "{what}: a triangle stands {furthest:e} off every surface"
    );
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// How far the furthest of a set of triangles stands from the arc, sampled on
/// a grid of eighths over each, which holds the middle of every side.
fn furthest_off(triangles: &[[DVec3; 3]], wall: &OnAnArc) -> f64 {
    let mut furthest = 0.0f64;
    for corners in triangles {
        for one in 0..=8 {
            for other in 0..=8 - one {
                let (a, b) = (one as f64 / 8.0, other as f64 / 8.0);
                let point = corners[0] * (1.0 - a - b) + corners[1] * a + corners[2] * b;
                furthest = furthest.max(wall.off_the_arc(point));
            }
        }
    }
    furthest
}

/// Holds that the wall is exact, closed, uncrossed and holds the matter
/// arithmetic promised; that its arc is drawn within the drawing's tolerance
/// of the arc; and that its triangles enclose that matter within the
/// tolerance over the arc's area.
fn drawn_within_the_tolerance(wall: &OnAnArc) {
    let what = format!(
        "the wall on the arc of {} over {}°",
        wall.radius, wall.degrees
    );
    let body = wall.raised();
    assert!(body.is_exact(), "{what}: the exact kernel computed it");
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    let promised = wall.volume();
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "{what}: the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );

    let tolerance = DRAWN * reach_of(&body);
    let arc: Vec<[DVec3; 3]> = body.triangles_of(ARC_WALL).collect();
    assert!(!arc.is_empty(), "{what}: the arc's wall is drawn");
    let off = furthest_off(&arc, wall);
    assert!(
        off <= tolerance,
        "{what}: its arc is drawn {off:e} off the arc, where {tolerance:e} is allowed"
    );

    let enclosed = enclosed(&triangles);
    let allowed = tolerance * wall.wall_area();
    assert!(
        (enclosed - promised).abs() <= allowed,
        "{what}: its triangles enclose {enclosed}, where arithmetic promised {promised}, within \
         {allowed:e}"
    );
}

#[test]
fn a_wall_on_the_arc_of_radius_3437_over_half_a_degree_is_drawn_within_the_tolerance_closed_uncrossed_and_whole()
 {
    drawn_within_the_tolerance(&OnAnArc {
        radius: 3437.0,
        degrees: 0.5,
    });
}

#[test]
fn a_wall_on_the_arc_of_radius_1719_over_one_degree_is_drawn_within_the_tolerance_closed_uncrossed_and_whole()
 {
    drawn_within_the_tolerance(&OnAnArc {
        radius: 1719.0,
        degrees: 1.0,
    });
}

#[test]
fn a_circle_no_wider_than_the_part_is_drawn_and_written_in_as_many_triangles_as_before() {
    let cases = [
        ("the bored cylinder", bored_cylinder(), 0.1),
        ("the bored cylinder", bored_cylinder(), 0.005),
        ("the disc 4 m across", wide_disc(), 0.1),
        ("the disc 4 m across", wide_disc(), 0.01),
    ];
    let counts = cases.each_ref().map(|(_, body, gap)| {
        let bytes = stl::of(body, *gap, 1.0).expect("the body is written");
        [body.triangles().len(), written(&bytes)]
    });

    assert_eq!(
        counts,
        BEFORE,
        "drawn and written, {:?}",
        cases
            .each_ref()
            .map(|(what, _, gap)| format!("{what} at {gap} mm"))
    );
}

#[test]
fn a_circle_no_wider_than_the_part_asked_finer_than_a_thousand_and_twenty_four_steps_a_turn_is_declined_as_uncovered()
 {
    let declined = stl::of(&wide_disc(), 0.005, 1.0).err();

    assert_eq!(
        declined,
        Some(Uncovered),
        "the disc 4 m across, as wide as the part, asked within 0.005 mm"
    );
}

#[test]
fn a_wall_on_an_arc_far_wider_than_the_part_written_to_stl_holds_a_fineness_past_a_thousand_and_twenty_four_steps_a_turn()
 {
    for wall in [
        OnAnArc {
            radius: 3437.0,
            degrees: 0.5,
        },
        OnAnArc {
            radius: 1719.0,
            degrees: 1.0,
        },
    ] {
        written_within(&wall, DVec3::ZERO, 0.005);
    }
}

#[test]
fn a_wall_on_an_arc_far_wider_than_the_part_drawn_far_from_the_origin_written_to_stl_holds_the_same_fineness()
 {
    let wall = OnAnArc {
        radius: 3437.0,
        degrees: 0.5,
    };

    written_within(&wall, DVec3::new(5000.0, 0.0, 0.0), 0.005);
}

#[test]
fn a_disc_no_wider_than_its_part_on_a_tilted_plane_asked_finer_than_a_thousand_and_twenty_four_steps_a_turn_is_declined_as_uncovered()
 {
    let radius = 2000.0;
    let disc = on_a_part(raised(disc(DVec2::ZERO, radius), TILTED, 10.0));
    let finest_under_the_cap = 0.01;
    let past_the_cap = 0.0075;
    let needed = |gap: f64| std::f64::consts::PI / (1.0 - gap / radius).acos();
    assert!(needed(finest_under_the_cap) < 1024.0 && needed(past_the_cap) > 1024.0);

    let written = stl::of(&disc, finest_under_the_cap, 1.0).map(|bytes| written(&bytes));
    let declined = stl::of(&disc, past_the_cap, 1.0).err();

    assert!(
        written.is_ok(),
        "the disc 4 m across on a tilted plane is written within {finest_under_the_cap} mm"
    );
    assert_eq!(
        declined,
        Some(Uncovered),
        "the disc 4 m across on a tilted plane, reaching {:.0} from the origin, asked within \
         {past_the_cap} mm: {:.0} steps a turn",
        reach_of(&disc),
        needed(past_the_cap)
    );
}

#[test]
fn a_wall_on_an_arc_far_wider_than_the_part_asked_finer_than_even_its_raised_cap_is_declined_as_uncovered()
 {
    let wall = OnAnArc {
        radius: 3437.0,
        degrees: 0.5,
    };
    let gap = 2e-7;
    let needed = std::f64::consts::PI / (1.0 - gap / wall.radius).acos();

    let declined = stl::of(&wall.raised(), gap, 1.0).err();

    assert_eq!(
        declined,
        Some(Uncovered),
        "the wall on the arc of 3437, its part some 16 across its half-diagonal, asked within \
         {gap} mm: {needed:.0} steps a turn"
    );
}

#[test]
fn an_elliptic_post_no_wider_than_its_part_asked_finer_than_a_thousand_and_twenty_four_steps_a_turn_is_declined_as_uncovered()
 {
    let post = on_a_part(raised(
        Contour {
            corners: vec![DVec2::new(15.0, 0.0)],
            runs: vec![Run::Oval {
                center: DVec2::ZERO,
                first: DVec2::new(15.0, 0.0),
                second: 8.0,
                turn: TAU,
            }],
        },
        GROUND,
        10.0,
    ));
    let (under_the_cap, past_the_cap) = (1e-3, 5e-5);

    let written = stl::of(&post, under_the_cap, 1.0).is_ok();
    let declined = stl::of(&post, past_the_cap, 1.0).err();

    assert!(
        written,
        "the elliptic post is written within {under_the_cap} mm"
    );
    assert_eq!(
        declined,
        Some(Uncovered),
        "the elliptic post of semi-axes 15 and 8 asked within {past_the_cap} mm"
    );
}
