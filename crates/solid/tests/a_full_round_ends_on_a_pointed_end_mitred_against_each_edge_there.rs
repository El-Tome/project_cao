//! A full round ending on a pointed end, seen from above, filleted in one step
//! with the two edges of the point. The plate is 30 long, 6 wide and 10 high,
//! its top (0, 0), (30, 0), the tip, (30, 6), (0, 6), raised as `cao_part`
//! builds a step: its floor numbered nought, its top one, then one wall for
//! each run of the outline in order — the front two, the point's front three,
//! the point's back four, the back five, and the square left end six. The
//! top's front and back edges, rounded at half the width, meet across the top
//! in one full round, as #698 lays it; each edge of the point meets one of
//! them at a corner, and the other edge of the point at the tip. Where #803
//! mitres a full round against the one edge across its end, here each point
//! edge's round is laid at the full round's radius and mitred against both its
//! neighbours on the vertical plane bisecting their walls, the three mitres
//! meeting at the crown, where the three cylinders touch. An operation of the
//! kernel on [`Body`], as #698's step on several edges is; the step of the
//! history that calls it comes with the application, in #701.
//!
//! Closes #843.
//! - The pointed end of this issue — tip (33,3) — filleted at 3 on its two
//!   long top edges and its two point edges, is built on the exact kernel and
//!   takes the value whole. The full round and each point round are cylinders
//!   of radius 3. Each point round meets the full round in the vertical plane
//!   through the corner (30,0) or (30,6) and the crown, and the two point
//!   rounds meet in the plane y = 3. The three mitres meet at the crown,
//!   (33 − 3√2, 3, 10). The body holds the volume arithmetic gives; its
//!   triangles are closed, uncrossed, and enclose that volume within the
//!   drawing's tolerance —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_mitres_each_point_round_against_its_neighbours_up_to_the_crown`
//! - The same body whatever the order the list names the four edges in —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_lays_the_same_body_whatever_the_order`.
//!   Read as: each of the 24 orders builds the body the first criterion
//!   holds, and the one the issue's order lays — the same matter, the same
//!   faces meeting by the same edges, each laid face read by the wall it was
//!   laid on, and the plate's flat faces at the same corners
//! - A sharper point, tip (39,3), with the crown at (39 − 3√10, 3, 10), and a
//!   blunter one, tip (32,3), with the crown at (32 − √13, 3, 10): built
//!   likewise —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_a_sharper_or_a_blunter_point_mitres_each_point_round_against_its_neighbours_up_to_the_crown`
//! - The same points chamfered at 3 stay built as on `main`, a hip whose flats
//!   meet at the crown —
//!   `a_chamfer_on_a_full_round_pair_s_edges_and_the_two_edges_of_a_pointed_end_stays_a_hip_whose_flats_meet_at_the_crown`
//! - No body of #698's, #699's or #803's tests changes — no test: those tests
//!   hold their own bodies, in
//!   `a_chamfer_and_a_fillet_take_several_edges_in_one_step.rs` and
//!   `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`, and run
//!   beside this file
//! - The ends of #894 are still built at the value `main` takes —
//!   `the_blunt_the_lopsided_and_the_bevelled_ends_are_still_filleted_at_the_value_main_takes`
//! - Decided on the way (2026-11-12): admitting an edge that meets one edge
//!   of the pair and a listed edge meeting the other also lets in a V-notched
//!   end, the point turned inwards — tips (27,3) and (28,3), the notch's two
//!   edges meeting at a concave corner, laid beside it as #801 lays two convex
//!   edges beside a concave one: the upright at the notch left sharp, each
//!   notch edge's round running on past its wall's end down to it, mitred
//!   against the other's in the plane y = 3, the crown at (27 − 3√2, 3, 10)
//!   and (28 − √13, 3, 10). `main` declines it in every order; kept, as the
//!   kernel should build every shape, and held in every order — built exact at
//!   3, one body, closed, uncrossed, its volume what arithmetic gives, its
//!   rounds and mitres held as the point's are —
//!   `a_fillet_on_a_full_round_and_the_two_edges_of_a_v_notched_end_mitres_each_notch_edge_s_round_against_its_neighbours_down_to_the_notch_whatever_the_order`
//! - Decided on the way (2026-11-12): a point whose tip's upright is named
//!   too, closed by a piece of sphere, builds in some orders and not others —
//!   no test: the issue pins it only if the order's fix makes every order lay
//!   one body, and otherwise leaves it to #837, the rounded tongue; settled
//!   once the order's criterion above is green
//!
//! The volume is arithmetic. Seen from above, the top is shared among the four
//! rounds, each share bounded by its wall, the mitres at the wall's two ends
//! and, for the front and the back, the middle of the left end: the front's
//! and the back's run from the left end to the mitre at their corner, and each
//! point edge's is the triangle its wall makes with the crown. Over its share,
//! a round lays the top at the height a quarter round of radius `r` gives at a
//! point's distance `t` from its wall, `H − r + √(r² − (r − t)²)`. Under the
//! rounds' axes, at `H − r`, the plate keeps its whole outline, of area
//! `2rL + ar` for a tip `a` past the ends of the front and back, both `L` long
//! — `a` negative for a notch, whose triangle is taken away. Over them the
//! matter is `∫ √(r² − (r − t)²)·ℓ(t) dt` from the walls, `t = 0`, up to the
//! axes, `t = r`, `ℓ(t)` the length of the lines at `t` from their walls
//! across the four shares: `L − kt` along the front and along the back, the
//! mitre at the corner standing back by `k = (s − a)/r` for each unit in from
//! the front, the cotangent of half the corner's angle; and `s(r − t)/r` along
//! each point edge, `s = √(a² + r²)` long, shrinking evenly to nothing at the
//! crown. So `ℓ(t)` runs linearly and the integral is
//! `ℓ(r)·πr²/4 + (ℓ(0) − ℓ(r))·r²/3`; `ℓ(r)` is twice the crown's distance
//! from the left end. On the tip (33, 3), `a = r = 3`:
//! `1305 + 36√2 + (66 − 6√2)·9π/4`, 1762.46; on the notch's tip (27, 3),
//! `1215 + 36√2 + (54 − 6√2)·9π/4`, 1587.64, and on (28, 3), 1618.14. At
//! `a = 0` the point is a square end, and the sum is #803's,
//! `(H − r)·2rL + πr²(L − r)/2 + 4r³/3`. The booleans give it too: the full
//! round's prism cut from the plate whole, then each point edge's, raised
//! along its wall's line past both its ends and kept on its own side of the
//! tip's mitre, `y = 3`.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::{PI, SQRT_2};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Filleted, Loop};
use glam::{DVec2, DVec3};

/// How far a point may stand from where the arithmetic puts it: each is placed
/// exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// How far inside a round's own share of the top a ray aims, clear of every
/// side of it, to meet that round's triangles rather than a neighbour's: well
/// over the drawing's tolerance on the plate, [`DRAWN`] times its reach.
const CLEAR_OF_THE_MITRE: f64 = 0.1;

/// The numbers the plate's faces answer to: its floor nought, its top one,
/// then its walls in the order its outline runs — the front, the point's
/// front, the point's back, the back and the left end.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const POINT_FRONT: usize = 3;
const POINT_BACK: usize = 4;
const BACK: usize = 5;
const LEFT_END: usize = 6;

/// The plate: its front and back 30 long, from the left end to the point, 6
/// wide and 10 high.
const LENGTH: f64 = 30.0;
const WIDTH: f64 = 6.0;
const HEIGHT: f64 = 10.0;

/// The value asked, half the width: the radius at which the rounds of the
/// top's front and back edges meet across the top in one full round.
const RADIUS: f64 = WIDTH / 2.0;

/// The four rounded walls' top edges in the order the issue names them: the
/// front, the point's front, the point's back and the back, as indices into
/// [`PointedPlate::walls`].
const AS_THE_ISSUE_NAMES_THEM: [usize; 4] = [0, 1, 2, 3];

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn profile(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// An outline drawn on the ground raised `height` as `cao_part` builds a
/// step: its floor numbered nought, its top one, then its walls, one for each
/// run of the outline.
fn raised(outline: Contour, height: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(&profile(outline), ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// Whether every point of `some` stands at a point of `others`, and every
/// point of `others` at one of `some`.
fn same_points(some: &[DVec3], others: &[DVec3]) -> bool {
    let within =
        |points: &[DVec3], point: &DVec3| points.iter().any(|other| other.distance(*point) < PLACE);
    some.iter().all(|point| within(others, point)) && others.iter().all(|point| within(some, point))
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    assert!(
        same_points(&plane.corners, expected),
        "{what}: face {face} has its corners at {:?}, where they are {expected:?}",
        plane.corners
    );
}

/// The sides of the faces the face answering to all of `numbers`, and to
/// nothing else, meets, one for each edge it has, in order.
fn sides_met_by(body: &Body, numbers: &[usize]) -> Vec<Vec<u32>> {
    let own: Vec<u32> = numbers.iter().map(|&number| number as u32).collect();
    let mut met: Vec<Vec<u32>> = body
        .edge_names()
        .into_iter()
        .filter_map(|name| {
            let [first, second] = name.between;
            if first == own {
                Some(second)
            } else if second == own {
                Some(first)
            } else {
                None
            }
        })
        .collect();
    met.sort();
    met
}

/// The sides a face is expected to meet, one edge each, in the order
/// [`sides_met_by`] lists them, each side every number the face across it
/// answers to: a full round answers to both its edges'.
fn sides_answering(faces: &[&[usize]]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces
        .iter()
        .map(|numbers| numbers.iter().map(|&number| number as u32).collect())
        .collect();
    sorted.sort();
    sorted
}

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, the matter it holds what
/// arithmetic promised, and the matter its triangles enclose within `drawn`
/// of it.
fn sound_drawn_within(body: &Body, promised: f64, drawn: f64, what: &str) {
    assert!(
        body.is_exact(),
        "{what}: the exact kernel computed the body"
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "{what}: the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );
    let enclosed = enclosed(&triangles);
    assert!(
        (enclosed - promised).abs() <= drawn,
        "{what}: its triangles enclose {enclosed}, where arithmetic promised {promised}"
    );
}

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// Holds that the numbers `laid` answer to one and the same face, which is
/// curved: drawn with the same triangles whichever is asked.
fn one_curved_face_answers_to(body: &Body, laid: &[usize], what: &str) {
    let drawn: Vec<Vec<[DVec3; 3]>> = laid
        .iter()
        .map(|&number| body.triangles_of(number).collect())
        .collect();
    assert!(
        !drawn[0].is_empty() && drawn.iter().all(|each| *each == drawn[0]),
        "{what}: faces {laid:?} are one face, drawn with the same triangles"
    );
    for &number in laid {
        assert!(
            !body.is_flat(number) && body.plane_of(number).is_none(),
            "{what}: face {number} is curved, and offers no plane to draw on"
        );
    }
}

/// Whether some corner of the triangles `drawn` stands at `point`.
fn drawn_through(drawn: &[DVec3], point: DVec3) -> bool {
    drawn.iter().any(|corner| corner.distance(point) < PLACE)
}

/// The round a fillet is expected to lay: a cylinder of `radius` about the
/// line from `start` along `along` for `length`, turning about it from facing
/// the way the edge's first face does to facing the way its second does.
/// Facing each face's way where it meets it is what tangent to both comes to.
struct Round {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Round {
    /// The point of the axis `share` of the way along it.
    fn axis_at(&self, share: f64) -> DVec3 {
        self.start + self.along * self.length * share
    }

    /// The way the round faces `share` of the way round it, from its first
    /// face's way to its second's.
    fn facing_at(&self, share: f64) -> DVec3 {
        let [first, second] = self.facing;
        let turn = first.dot(second).acos() * share;
        let across = (second - first * first.dot(second)).normalize();
        first * turn.cos() + across * turn.sin()
    }

    /// Whether a point lies on the round: on its cylinder, between its two
    /// ends, and between its two sides.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let across = off - self.along * along;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && across.dot(first) >= least - PLACE
            && across.dot(second) >= least - PLACE
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that every ray aimed square at the round's axis from outside, over
/// a grid along it and round it, meets the face numbered `face` first,
/// facing away from the axis along the ray, wherever `shows` says the round
/// shows at the point the ray aims at; and that some ray was aimed.
fn met_from_outside_where(
    body: &Body,
    face: usize,
    round: &Round,
    shows: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let mut aimed = 0;
    for tenth in 0..10 {
        let along = (f64::from(tenth) + 0.5) / 10.0;
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = round.facing_at(share);
            let aim = round.axis_at(along) + facing * round.radius;
            if !shows(aim) {
                continue;
            }
            aimed += 1;
            let from = aim + facing * 10.0;
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} toward the axis meets nothing")
            });
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} toward {aim} meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 away from its axis",
                hit.normal
            );
        }
    }
    assert!(aimed > 0, "{what}: some ray aims where the round shows");
}

/// A round's share of the top, seen from above: the corners of the polygon it
/// is laid over, anticlockwise.
struct Share {
    corners: Vec<DVec2>,
}

impl Share {
    /// How far a point stands inside the share, seen from above: its least
    /// distance in from the share's sides, negative outside.
    fn inside_by(&self, point: DVec3) -> f64 {
        let point = point.truncate();
        let next = self.corners.iter().cycle().skip(1);
        self.corners
            .iter()
            .zip(next)
            .map(|(&from, &to)| (to - from).normalize().perp_dot(point - from))
            .fold(f64::INFINITY, f64::min)
    }
}

/// A wall of the plate whose top edge the list names: its number, what its
/// top edge is called, and where it runs from and to, seen from above, the
/// way the outline runs round the plate.
struct Wall {
    number: usize,
    called: String,
    from: DVec2,
    to: DVec2,
}

impl Wall {
    /// The way the wall runs, level.
    fn along(&self) -> DVec3 {
        (self.to - self.from).normalize().extend(0.0)
    }

    /// The way out of the plate, square to the wall, level: the outline runs
    /// anticlockwise seen from above, the plate on its left.
    fn outward(&self) -> DVec3 {
        self.along().cross(DVec3::Z)
    }

    /// The top edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        edge_between(part, TOP, self.number)
    }

    /// The round a fillet of [`RADIUS`] lays on the top edge over `share`: a
    /// quarter round about the line the radius in from the wall and under the
    /// top, as far along as the share reaches — past the wall's end where the
    /// wall meets the next inwards, as a notch's two do — turning from the
    /// top's way to the wall's.
    fn round_over(&self, share: &Share) -> Round {
        let along = self.along();
        let reached = |corner: &DVec2| (*corner - self.from).dot(along.truncate());
        let first = share
            .corners
            .iter()
            .map(reached)
            .fold(f64::INFINITY, f64::min);
        let last = share
            .corners
            .iter()
            .map(reached)
            .fold(f64::NEG_INFINITY, f64::max);
        Round {
            start: self.from.extend(HEIGHT) - (self.outward() + DVec3::Z) * RADIUS + along * first,
            along,
            length: last - first,
            radius: RADIUS,
            facing: [DVec3::Z, self.outward()],
        }
    }

    /// The prism the round of the top edge takes away alone: all that stands
    /// over the edge outside its quarter round, raised along the wall's whole
    /// line and past both its ends.
    fn tool(&self, part: &Body) -> Body {
        let r = RADIUS;
        let far = 2.0 * r + 2.0;
        let foot = HEIGHT - r;
        let corners = vec![
            DVec2::new(0.0, foot),
            DVec2::new(r, HEIGHT),
            DVec2::new(r, HEIGHT + far),
            DVec2::new(-far, HEIGHT + far),
            DVec2::new(-far, foot),
        ];
        let mut runs = vec![Run::Straight; corners.len()];
        runs[0] = Run::Round {
            center: DVec2::new(r, foot),
            turn: -PI / 2.0,
        };
        let along = self.along();
        let past = r + 2.0;
        part.tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: self.from.extend(0.0) - along * past,
                u: DVec3::Z.cross(along),
                v: DVec3::Z,
            },
            along * (self.from.distance(self.to) + 2.0 * past),
        )
        .expect("the tool raises")
    }
}

/// Where two neighbouring rounds meet: their walls, as indices into
/// [`PointedPlate::walls`], and the corner of the outline those walls share,
/// seen from above, and what it is called.
struct Mitre {
    walls: [usize; 2],
    corner: DVec2,
    called: &'static str,
}

impl Mitre {
    /// How far a point stands from the vertical plane through the corner and
    /// `crown`.
    fn off(&self, crown: DVec3, point: DVec3) -> f64 {
        let across = (crown.truncate() - self.corner).perp().normalize();
        (point.truncate() - self.corner).dot(across).abs()
    }
}

/// A plate [`LENGTH`] long, [`WIDTH`] wide and [`HEIGHT`] high whose right end
/// comes to a point seen from above, from the ends of its front and back to
/// its tip on the middle line, `reach` past them — or, `reach` negative, is
/// notched back to it: the part, raised as `cao_part` builds a step.
struct PointedPlate {
    reach: f64,
    part: Body,
}

impl PointedPlate {
    /// The plate whose tip stands at `x` along, on the middle line.
    fn with_tip_at(x: f64) -> PointedPlate {
        let tip = DVec2::new(x, RADIUS);
        PointedPlate {
            reach: x - LENGTH,
            part: raised(
                Contour::straight(vec![
                    DVec2::ZERO,
                    DVec2::new(LENGTH, 0.0),
                    tip,
                    DVec2::new(LENGTH, WIDTH),
                    DVec2::new(0.0, WIDTH),
                ]),
                HEIGHT,
            ),
        }
    }

    /// The tip, seen from above.
    fn tip(&self) -> DVec2 {
        DVec2::new(LENGTH + self.reach, RADIUS)
    }

    /// The four walls whose top edges are rounded, in the order the outline
    /// runs: the front, the point's front, the point's back and the back.
    fn walls(&self) -> [Wall; 4] {
        let end = if self.reach < 0.0 { "notch" } else { "point" };
        let front_end = DVec2::new(LENGTH, 0.0);
        let back_end = DVec2::new(LENGTH, WIDTH);
        [
            Wall {
                number: FRONT,
                called: "the top's front edge".to_string(),
                from: DVec2::ZERO,
                to: front_end,
            },
            Wall {
                number: POINT_FRONT,
                called: format!("the {end}'s front edge"),
                from: front_end,
                to: self.tip(),
            },
            Wall {
                number: POINT_BACK,
                called: format!("the {end}'s back edge"),
                from: self.tip(),
                to: back_end,
            },
            Wall {
                number: BACK,
                called: "the top's back edge".to_string(),
                from: back_end,
                to: DVec2::new(0.0, WIDTH),
            },
        ]
    }

    /// The four rounds' shares of the top, as [`PointedPlate::walls`] lists
    /// their walls, the mitres meeting at `crown`: the front's and the back's
    /// from the left end, up to its middle, to the mitre at their corner, and
    /// each point edge's the triangle its wall makes with the crown.
    fn shares(&self, crown: DVec3) -> [Share; 4] {
        let crown = crown.truncate();
        let middle_of_the_left_end = DVec2::new(0.0, RADIUS);
        let [front, point_front, point_back, back] = self.walls();
        [
            vec![front.from, front.to, crown, middle_of_the_left_end],
            vec![point_front.from, point_front.to, crown],
            vec![point_back.from, point_back.to, crown],
            vec![back.from, back.to, middle_of_the_left_end, crown],
        ]
        .map(|corners| Share { corners })
    }

    /// The three mitres: the full round against the point's front edge's
    /// round at the front corner, the two point rounds against each other at
    /// the tip, and the full round against the point's back edge's round at
    /// the back corner.
    fn mitres(&self) -> [Mitre; 3] {
        [
            Mitre {
                walls: [0, 1],
                corner: DVec2::new(LENGTH, 0.0),
                called: "the front corner",
            },
            Mitre {
                walls: [1, 2],
                corner: self.tip(),
                called: "the tip",
            },
            Mitre {
                walls: [2, 3],
                corner: DVec2::new(LENGTH, WIDTH),
                called: "the back corner",
            },
        ]
    }

    /// The fillet at [`RADIUS`] on the four rounded walls' top edges, named
    /// in `order`, as indices into [`PointedPlate::walls`].
    fn filleted(&self, order: [usize; 4]) -> Result<Filleted, Declined> {
        let walls = self.walls();
        let names: Vec<EdgeName> = order
            .iter()
            .map(|&wall| walls[wall].name(&self.part))
            .collect();
        self.part.fillet_edges(&names, RADIUS)
    }

    /// What the fillet on the edges named in `order` is called.
    fn called(&self, order: [usize; 4]) -> String {
        let walls = self.walls();
        let called: Vec<&str> = order
            .iter()
            .map(|&wall| walls[wall].called.as_str())
            .collect();
        format!(
            "the fillet at {RADIUS} on {}, the tip at {}",
            called.join(", then "),
            self.tip()
        )
    }

    /// How long each edge of the point, or of the notch, is.
    fn point_edge(&self) -> f64 {
        self.reach.hypot(RADIUS)
    }

    /// How far back along the front, or the back, the mitre at its corner
    /// stands for each unit in from it: the cotangent of half the corner's
    /// angle.
    fn corner_drift(&self) -> f64 {
        (self.point_edge() - self.reach) / RADIUS
    }

    /// The area of the outline: the plate up to the ends of its front and
    /// back, and the point's triangle — or less the notch's, its reach
    /// negative.
    fn outline_area(&self) -> f64 {
        WIDTH * LENGTH + self.reach * RADIUS
    }

    /// The length of the lines at `t` from their walls across the four
    /// rounds' shares, seen from above: along the front and the back from the
    /// left end to the mitre at its corner, and along each point edge between
    /// its two mitres, shrinking evenly to nothing at the crown.
    fn shares_at(&self, t: f64) -> f64 {
        let r = RADIUS;
        2.0 * (LENGTH - self.corner_drift() * t) + 2.0 * self.point_edge() * (r - t) / r
    }

    /// What the plate keeps once its four rounds are laid at [`RADIUS`] and
    /// mitred, as the module's documentation derives it: the whole outline
    /// up to the rounds' axes, and over them a quarter round swept along the
    /// shares, from the walls up to the axes.
    fn rounded(&self) -> f64 {
        (HEIGHT - RADIUS) * self.outline_area()
            + quarter_round_swept_along(self.shares_at(0.0), self.shares_at(RADIUS))
    }
}

/// What a quarter round of [`RADIUS`] holds swept along lines whose length
/// runs evenly from `at_the_walls` up to `at_the_axes`: the quarter disc
/// along `at_the_axes`, and a third of the radius cubed for each unit the
/// line shrinks per unit climbed.
fn quarter_round_swept_along(at_the_walls: f64, at_the_axes: f64) -> f64 {
    let r = RADIUS;
    at_the_axes * PI * r * r / 4.0 + (at_the_walls - at_the_axes) / r * r.powi(3) / 3.0
}

/// The prism the full round takes away, raised whole along X past both ends
/// of the plate, the point included: all that stands over the front and the
/// back outside the half-cylinder about the line [`RADIUS`] under the middle
/// of the top.
fn full_round_tool(plate: &PointedPlate) -> Body {
    let far = 2.0 * RADIUS + 2.0;
    let foot = HEIGHT - RADIUS;
    let corners = vec![
        DVec2::new(-far, foot),
        DVec2::new(0.0, foot),
        DVec2::new(WIDTH, foot),
        DVec2::new(WIDTH + far, foot),
        DVec2::new(WIDTH + far, HEIGHT + far),
        DVec2::new(-far, HEIGHT + far),
    ];
    let mut runs = vec![Run::Straight; corners.len()];
    runs[1] = Run::Round {
        center: DVec2::new(RADIUS, foot),
        turn: -PI,
    };
    plate
        .part
        .tool_raised(
            &profile(Contour { corners, runs }),
            Frame {
                origin: DVec3::new(-2.0, 0.0, 0.0),
                u: DVec3::Y,
                v: DVec3::Z,
            },
            DVec3::X * (LENGTH + plate.reach.max(0.0) + 4.0),
        )
        .expect("the tool raises")
}

/// What the plate keeps once the full round and the point rounds are laid at
/// [`RADIUS`], computed apart from the step: the full round's prism, then
/// each point edge's, raised whole along its line and kept on its own side of
/// the tip's mitre, taken away by the booleans.
fn rounded_by_booleans(plate: &PointedPlate) -> f64 {
    let [_, point_front, point_back, _] = plate.walls();
    let rounded = plate
        .part
        .difference(&full_round_tool(plate))
        .expect("the full round's prism cuts the plate");
    [(point_front, DVec3::Y), (point_back, -DVec3::Y)]
        .iter()
        .fold(rounded, |cut, (wall, toward_its_side)| {
            let prism = wall
                .tool(&cut)
                .behind(*toward_its_side, toward_its_side.y * RADIUS);
            cut.difference(&prism)
                .expect("a point edge's round's prism cuts the plate")
        })
        .volume()
}

/// Every order the four rounded walls' top edges can be named in, as indices
/// into [`PointedPlate::walls`].
fn every_order() -> Vec<[usize; 4]> {
    let mut orders = Vec::new();
    for first in 0..4 {
        for second in 0..4 {
            for third in 0..4 {
                for fourth in 0..4 {
                    let order = [first, second, third, fourth];
                    if (0..4).all(|wall| order.contains(&wall)) {
                        orders.push(order);
                    }
                }
            }
        }
    }
    orders
}

/// Each face of a body and the sides of the faces it meets, one for each edge
/// it has, every face laid on the edges named in `order` read by the wall it
/// was laid on, as the rank the issue's order gives that wall: what tells two
/// bodies the same face for face, whatever the order their edges were named
/// in.
fn meetings(body: &Body, start: usize, order: [usize; 4]) -> BTreeMap<Vec<u32>, Vec<Vec<u32>>> {
    let as_the_issue_names_them = |side: Vec<u32>| -> Vec<u32> {
        let mut read: Vec<u32> = side
            .into_iter()
            .map(|number| {
                let number = number as usize;
                if number < start {
                    return number as u32;
                }
                let wall = order[number - start];
                let rank = AS_THE_ISSUE_NAMES_THEM
                    .iter()
                    .position(|&named| named == wall)
                    .expect("the issue names every wall");
                (start + rank) as u32
            })
            .collect();
        read.sort();
        read
    };
    let mut met: BTreeMap<Vec<u32>, Vec<Vec<u32>>> = BTreeMap::new();
    for name in body.edge_names() {
        let [first, second] = name.between.map(as_the_issue_names_them);
        met.entry(first.clone()).or_default().push(second.clone());
        met.entry(second).or_default().push(first);
    }
    for sides in met.values_mut() {
        sides.sort();
    }
    met
}

/// Holds that a body whose faces meet as those of `reference`, the one laid
/// with the edges named as the issue names them, is that body: the same
/// matter, and the plate's flat faces at the same corners.
fn same_body_as(plate: &PointedPlate, body: &Body, reference: &Body, what: &str) {
    assert!(
        (body.volume() - reference.volume()).abs() <= EXACT * reference.volume(),
        "{what}: the body holds {}, where the issue's order lays one of {}",
        body.volume(),
        reference.volume()
    );
    for face in numbers(&plate.part) {
        let [own, theirs] =
            [body, reference].map(|laid| laid.plane_of(face).map(|plane| plane.corners));
        let alike = match (&own, &theirs) {
            (Some(own), Some(theirs)) => same_points(own, theirs),
            (None, None) => true,
            _ => false,
        };
        assert!(
            alike,
            "{what}: face {face} has its corners at {own:?}, where the issue's order leaves them \
             at {theirs:?}"
        );
    }
}

/// Holds that the fillet at [`RADIUS`] on the top edges of the plate's four
/// rounded walls, named in `order` and laid as `filleted`, takes the value
/// whole and lays the full round and each point round at that radius, each
/// mitred against its neighbours up to `crown`: the body exact, closed,
/// uncrossed and drawn, holding what the arithmetic promises; the full round
/// answering to both its edges' numbers and each point round to its own, the
/// top gone and nothing laid at the crown; each round drawn on its own
/// cylinder over its own share of the top, and met there from outside; each
/// two neighbours drawn up to both ends of their mitre, from the corner their
/// walls share up to the crown, and meeting nowhere off the vertical plane
/// through the two; the three meeting at one vertex, the crown; and the walls
/// set back by the radius under them.
fn mitres_each_point_round_against_its_neighbours(
    plate: &PointedPlate,
    crown: DVec3,
    order: [usize; 4],
    filleted: &Filleted,
    what: &str,
) {
    let r = RADIUS;
    let part = &plate.part;
    let start = part.faces_end();
    let walls = plate.walls();
    let shares = plate.shares(crown);
    let rounds: Vec<Round> = walls
        .iter()
        .zip(&shares)
        .map(|(wall, share)| wall.round_over(share))
        .collect();

    taken_as_asked(filleted.asked, filleted.taken, r, what);
    let body = &filleted.body;
    let area: f64 = rounds.iter().map(Round::area).sum();
    sound_drawn_within(body, plate.rounded(), DRAWN * reach_of(part) * area, what);

    let rank_of = |wall: usize| {
        start
            + order
                .iter()
                .position(|&named| named == wall)
                .expect("every wall's top edge is named")
    };
    let mut full = [rank_of(0), rank_of(3)];
    full.sort();
    let side_of = |wall: usize| match wall {
        0 | 3 => full.to_vec(),
        other => vec![rank_of(other)],
    };
    let face_of = |wall: usize| side_of(wall)[0];
    let round_called = |wall: usize| match wall {
        0 | 3 => "the full round".to_string(),
        other => format!("the round of {}", walls[other].called),
    };
    let [point_front, point_back] = [rank_of(1), rank_of(2)];

    assert!(
        !body.has_face(TOP),
        "{what}: the top is gone, the full round over it"
    );
    let mut kept_and_laid = numbers(part);
    kept_and_laid.remove(&TOP);
    kept_and_laid.extend(start..start + 4);
    assert_eq!(
        numbers(body),
        kept_and_laid,
        "{what}: every face of the plate but its top, the full round answering to {full:?}, \
         each point edge's round to its own rank, {point_front} and {point_back}, and no face \
         laid at the crown"
    );
    assert_eq!(
        body.faces_end(),
        start + 4,
        "{what}: the next step numbers its faces past the four the list gave its edges"
    );
    one_curved_face_answers_to(body, &full, &format!("{what}: the full round"));
    for number in [point_front, point_back] {
        assert!(
            !body.is_flat(number) && body.plane_of(number).is_none(),
            "{what}: the point edge's round, face {number}, is curved, and offers no plane to \
             draw on"
        );
    }

    let drawn_of =
        |wall: usize| -> Vec<DVec3> { body.triangles_of(face_of(wall)).flatten().collect() };
    for own in [&[0, 3][..], &[1][..], &[2][..]] {
        let which = round_called(own[0]);
        let drawn = drawn_of(own[0]);
        assert!(!drawn.is_empty(), "{what}: {which} is drawn");
        for &point in &drawn {
            assert!(
                own.iter()
                    .any(|&wall| rounds[wall].holds(point)
                        && shares[wall].inside_by(point) >= -PLACE),
                "{what}: {which} is drawn through {point}, off the quarter round of {r} over \
                 the share of the top of each of its walls"
            );
        }
        for &wall in own {
            met_from_outside_where(
                body,
                face_of(wall),
                &rounds[wall],
                |aim| shares[wall].inside_by(aim) > CLEAR_OF_THE_MITRE,
                &format!(
                    "{what}: {which}, over the wall under {}, clear of its mitres",
                    walls[wall].called
                ),
            );
        }
    }

    for mitre in plate.mitres() {
        let [one, other] = mitre.walls;
        let [one_drawn, other_drawn] = [drawn_of(one), drawn_of(other)];
        let foot = mitre.corner.extend(HEIGHT - r);
        for (point, at) in [(foot, mitre.called), (crown, "the crown")] {
            for (drawn, which) in [
                (&one_drawn, round_called(one)),
                (&other_drawn, round_called(other)),
            ] {
                assert!(
                    drawn_through(drawn, point),
                    "{what}: {which} is drawn up to {at}, {point}, where the mitre at {} ends",
                    mitre.called
                );
            }
        }
        for &point in one_drawn
            .iter()
            .filter(|&&point| drawn_through(&other_drawn, point))
        {
            assert!(
                mitre.off(crown, point) < PLACE,
                "{what}: {} and {} meet at {point}, off the vertical plane through {} and the \
                 crown, {crown}",
                round_called(one),
                round_called(other),
                mitre.called
            );
        }
    }

    let mut at_the_crown: Vec<Vec<u32>> = [0, 1, 2]
        .map(|wall| {
            side_of(wall)
                .into_iter()
                .map(|number| number as u32)
                .collect()
        })
        .to_vec();
    at_the_crown.sort();
    let crowns = body
        .vertex_names()
        .into_iter()
        .filter(|vertex| vertex.faces == at_the_crown)
        .count();
    assert_eq!(
        crowns, 1,
        "{what}: the full round and the two point rounds meet at one vertex, the crown"
    );

    let full_side: &[usize] = &full;
    assert_eq!(
        sides_met_by(body, full_side),
        sides_answering(&[
            &[FRONT],
            &[BACK],
            &[LEFT_END],
            &[point_front],
            &[point_back]
        ]),
        "{what}: the full round meets the front and the back along its sides, the left end \
         across it, and each point edge's round along its mitre, by one edge each"
    );
    for (number, wall, beside) in [
        (point_front, POINT_FRONT, point_back),
        (point_back, POINT_BACK, point_front),
    ] {
        assert_eq!(
            sides_met_by(body, &[number]),
            sides_answering(&[&[wall], full_side, &[beside]]),
            "{what}: the round of face {number} meets its wall along its foot, the full round \
             along one mitre and the other point round along the other, by one edge each"
        );
    }
    for (at, wall) in walls.iter().enumerate() {
        let [from, to] = [wall.from, wall.to];
        corners_are(
            body,
            wall.number,
            &[
                from.extend(0.0),
                to.extend(0.0),
                to.extend(HEIGHT - r),
                from.extend(HEIGHT - r),
            ],
            &format!(
                "{what}: the wall under {}, set back by the radius from the top",
                wall.called
            ),
        );
        let before = if at == 0 {
            LEFT_END
        } else {
            walls[at - 1].number
        };
        let after = if at == 3 {
            LEFT_END
        } else {
            walls[at + 1].number
        };
        assert_eq!(
            sides_met_by(body, &[wall.number]),
            sides_answering(&[&[FLOOR], &[before], &[after], &side_of(at)]),
            "{what}: the wall under {} meets the floor, the walls on either side along its \
             uprights, and its round along its top",
            wall.called
        );
    }
}

/// What the faces of `body`, laid on the edges named in `order`, meet
/// otherwise than those of `reference`, laid on them named as the issue names
/// them, each laid face read by the wall it was laid on: one line for each
/// face that does.
fn met_otherwise(start: usize, order: [usize; 4], body: &Body, reference: &Body) -> Vec<String> {
    let own = meetings(body, start, order);
    let theirs = meetings(reference, start, AS_THE_ISSUE_NAMES_THEM);
    let faces: BTreeSet<&Vec<u32>> = own.keys().chain(theirs.keys()).collect();
    faces
        .into_iter()
        .filter(|face| own.get(*face) != theirs.get(*face))
        .map(|face| {
            format!(
                "face {face:?} meets {:?}, where in the issue's order it meets {:?}",
                own.get(face),
                theirs.get(face)
            )
        })
        .collect()
}

/// Holds, for each plate `cases` names by its tip and the crown its mitres
/// meet at, that the fillet at [`RADIUS`] on its four rounded walls' top
/// edges lays in each of `orders` the one body the issue's order lays, the
/// mitred rounds: every body whose faces meet as that order's do held whole
/// first, then every order declined, and every one whose faces meet
/// otherwise, listed together.
fn lays_one_mitred_body_in_each_order(cases: &[(f64, DVec3)], orders: &[[usize; 4]]) {
    let mut declined = Vec::new();
    let mut laid_otherwise = Vec::new();
    for &(tip, crown) in cases {
        let plate = PointedPlate::with_tip_at(tip);
        let start = plate.part.faces_end();
        let promised = plate.rounded();
        let by_booleans = rounded_by_booleans(&plate);
        assert!(
            (by_booleans - promised).abs() <= EXACT * promised,
            "the tip at {}: the full round's prism and the point rounds', cut from the plate by \
             the booleans, leave {by_booleans}, where arithmetic promises {promised}",
            plate.tip()
        );
        let reference = plate.filleted(AS_THE_ISSUE_NAMES_THEM);
        for &order in orders {
            let what = plate.called(order);
            let filleted = match plate.filleted(order) {
                Ok(filleted) => filleted,
                Err(reason) => {
                    declined.push(format!("{what}: declined, {reason:?}"));
                    continue;
                }
            };
            if let Ok(reference) = &reference {
                let otherwise = met_otherwise(start, order, &filleted.body, &reference.body);
                if !otherwise.is_empty() {
                    laid_otherwise.push(format!("{what}:\n  {}", otherwise.join("\n  ")));
                    continue;
                }
            }
            mitres_each_point_round_against_its_neighbours(&plate, crown, order, &filleted, &what);
            if let Ok(reference) = &reference {
                same_body_as(&plate, &filleted.body, &reference.body, &what);
            }
        }
    }
    assert!(
        declined.is_empty() && laid_otherwise.is_empty(),
        "the fillet at {RADIUS} on the top's front and back edges, a full round, and on the two \
         edges of the plate's end, each meeting one of them, lays one body whichever edge the \
         list names first, the one the issue's order lays; declined:\n{}\nits faces meeting \
         otherwise than in the issue's order:\n{}",
        declined.join("\n"),
        laid_otherwise.join("\n")
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_mitres_each_point_round_against_its_neighbours_up_to_the_crown()
 {
    lays_one_mitred_body_in_each_order(
        &[(33.0, DVec3::new(33.0 - 3.0 * SQRT_2, 3.0, 10.0))],
        &[AS_THE_ISSUE_NAMES_THEM],
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_two_edges_of_its_pointed_end_lays_the_same_body_whatever_the_order()
 {
    lays_one_mitred_body_in_each_order(
        &[(33.0, DVec3::new(33.0 - 3.0 * SQRT_2, 3.0, 10.0))],
        &every_order(),
    );
}

#[test]
fn a_fillet_on_a_full_round_and_the_two_edges_of_a_v_notched_end_mitres_each_notch_edge_s_round_against_its_neighbours_down_to_the_notch_whatever_the_order()
 {
    lays_one_mitred_body_in_each_order(
        &[
            (27.0, DVec3::new(27.0 - 3.0 * SQRT_2, 3.0, 10.0)),
            (28.0, DVec3::new(28.0 - 13f64.sqrt(), 3.0, 10.0)),
        ],
        &every_order(),
    );
}
