//! A chamfer and a fillet on concave straight edges of an exact body that
//! meet at a corner: the feet of a block standing on another, where its walls
//! meet the lower block's top. Each fill is laid as one fills alone (#778),
//! trimmed flush at both ends on the planes of the faces its edge runs into.
//! Where two feet meet at a corner whose third edge, the standing block's
//! upright, meets outwards, the two fill prisms carried past the corner hold
//! matter in common beyond both walls, and that piece is joined too: the two
//! fills run on past the corner into a mitre outside the upright, which
//! stays sharp. An operation of the kernel on [`Body`], as #699's corners
//! are.
//!
//! Part of #801, its second slice: two concave edges with a convex one left
//! sharp. Its first slice, two convex edges beside a concave one, is held in
//! `a_chamfer_and_a_fillet_stop_and_mitre_beside_a_concave_edge.rs`; its
//! third, the three concave edges of a pocket's bottom corner, comes on a
//! branch of its own, and the issue stays open until it lands.
//!
//! - Two concave edges meeting at a corner whose third edge is convex and
//!   left sharp leave a closed, sound body when chamfered or filleted at one
//!   value. This holds on two feet of a block standing on another, at one of
//!   its corners. Each fill ends flush on the wall the other stands against.
//!   The two fills run on past the corner and meet in a mitre outside the
//!   convex edge: a straight line among flats, a planar ellipse arc among
//!   rounds. The volume is exact, and the body is the same in either order —
//!   `a_chamfer_and_a_fillet_on_two_feet_of_a_standing_block_fill_them_into_a_mitre_outside_its_upright_left_sharp`,
//!   on its front and right feet and on its right and back feet. The four
//!   feet mitre the four corners —
//!   `a_chamfer_and_a_fillet_on_the_four_feet_of_a_standing_block_mitre_its_four_corners`.
//!   "Each fill ends flush on the wall the other stands against" is read as
//!   what the design lays — each fill trimmed flush there, then the piece
//!   past both walls joined — and held by what the body shows of it: the
//!   matter it holds, each fill's face going no farther past the corner than
//!   the plane halfway between the two edges, and each fill trimmed flush on
//!   the wall at its other end, that wall grown by the fill's end section and
//!   keeping its number
//! - The same holds at a corner that is not square: a standing block whose
//!   outline is not a rectangle —
//!   `a_chamfer_and_a_fillet_on_two_feet_at_a_corner_of_a_standing_block_that_is_not_square_fill_them_into_a_mitre`,
//!   a trapezoid whose right wall leans, on the two feet at its obtuse corner,
//!   the two at its acute one, and its four feet
//! - What must not break: as slice 1 — #699's corner tests, the concave fill
//!   of one edge (`a_chamfer_and_a_fillet_fill_a_concave_edge`) and slice 1's
//!   own tests; and the decline test
//!   `a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported`
//!   loses the feet, which become the body tests above, and keeps the others
//!   — no test: each is held by its own file, left as it was but for the
//!   three lists of feet the decline test gives up
//!
//! The volumes are arithmetic. The lower block is 30 by 20 by 10 and the
//! block standing on it 10 by 10 by 10, 7000 together. Every foot lies
//! between two faces square to each other, and every wall it ends on stands
//! square to it. A flat set back `d` along both faces fills a right triangle
//! of legs `d`, `d²/2`, along the foot's length; a round of radius `r` the
//! square of side `r` less the quarter of the disc, `r²(1 − π/4)`. At a corner
//! where two feet meet, measured from it `c` up from the lower block's top
//! and `p` and `q` out past the two walls, both fill prisms carried past the
//! corner hold what has `c < d − max(p, q)` among flats, and what lies below
//! both rounds among rounds, `c < w(max(p, q))` with
//! `w(u) = r − √(r² − (r − u)²)`; the two fills' faces meet where `p = q`.
//! Over the square of side the value past both walls that is
//! `2∫₀ᵈ u(d − u) du = d³/3` and `2∫₀ʳ u·w(u) du = r³(5/3 − π/2)`, the
//! amounts #699 gives back at a convex corner and slice 1 takes beside a
//! concave one, here joined. Two feet of 10 hold `7000 + 10d² + d³/3` and
//! `7000 + 20r²(1 − π/4) + r³(5/3 − π/2)`; the four,
//! `7000 + 20d² + 4d³/3` and `7000 + 40r²(1 − π/4) + 4r³(5/3 − π/2)`. Held
//! another way, by the booleans, in the tests themselves: the part joined to
//! each fill's prism carried well past both ends, clipped at a mitred end by
//! the plane halfway between the two edges on the other's side, and at
//! another end by the plane of the face it runs into, on the far side.
//!
//! The block standing at a corner that is not square is a trapezoid, 10 at
//! its front, 14 at its back and 10 deep, its right wall leaning 4 in 10:
//! 7200 with the block below. Its walls stand upright on a level top, so
//! each fill still takes its section square to its foot; what leans is the
//! wall a foot ends on. A line along the foot `a` through the section,
//! standing `o` out from it, runs from the plane of the face at one end to the
//! plane at the other, `f₀` and `f₁` the ways they face:
//! `L − o·(f₁/(a·f₁) − f₀/(a·f₀))` long. The fill adds the section times `L`
//! less its first moment about either face, `d³/6` for a flat and
//! `r³(5/6 − π/4)` for a round, times `(n₁ + n₂)·(f₁/(a·f₁) − f₀/(a·f₀))`:
//! on the front foot, ending on the leaning wall `0.4` nearer for each unit
//! out, `10S − 0.4M`; on the back foot, `14S + 0.4M`; on the right foot, whose
//! two end walls are parallel, `√116·S`; on the left, `10S`. At a corner the
//! two fills hold besides the square corner's amount stretched by `1/sin φ`,
//! `φ` the angle between the two walls, a unit out past one by a unit out
//! past the other covering `1/sin φ` of the lower block's top; at both right
//! corners `sin φ = 10/√116`. The mitre still runs from where the two
//! set-backs cross on the lower block's top, the value out from both walls,
//! up the upright the value: both walls stand upright on it.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DMat3, DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane:
/// each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The number a raised part's top answers to.
const TOP: usize = 1;

/// The values every list is filled at.
const VALUES: [f64; 2] = [1.0, 2.5];

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

/// An outline drawn on the ground raised from `from` by `height` and joined
/// to the part as `cao_part` builds a step: the outline's faces numbered past
/// the part's, its floor first, then its top, then its walls, one for each
/// run of the outline.
fn joined(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    let tool = part
        .tool_raised(
            &profile(outline),
            Frame {
                origin: DVec3::Z * from,
                ..ground()
            },
            DVec3::Z * height,
        )
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// A block 30 by 20 by 10 with another, 10 by 10 by 10, standing on the
/// middle of its top: the upper block's top numbered seven, its walls, front,
/// right, back and left, eight to eleven, its floor gone into the lower
/// block's top. Its feet, where its walls meet that top, meet inwards; its
/// uprights outwards.
fn stacked() -> Body {
    joined(
        &joined(
            &Body::default(),
            Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
            0.0,
            10.0,
        ),
        Contour::rectangle(DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)),
        10.0,
        10.0,
    )
}
const STACKED_VOLUME: f64 = 30.0 * 20.0 * 10.0 + 10.0 * 10.0 * 10.0;
const STANDING_FRONT: usize = 8;
const STANDING_RIGHT: usize = 9;
const STANDING_BACK: usize = 10;
const STANDING_LEFT: usize = 11;

/// A concave edge between two faces: the faces it lies between, the way each
/// faces, out of the matter, and its two ends. Where two edges are mitred,
/// the first face of each is the one they share, and each runs into the
/// other's second face.
struct Concave {
    called: &'static str,
    between: [usize; 2],
    facing: [DVec3; 2],
    ends: [End; 2],
}

/// An end of an edge: where it is, the face the edge runs into there — the
/// one face at that corner that is neither of the edge's two, flat — the way
/// that face faces, out of the matter, and whether matter lies past it along
/// the edge's line, or air.
struct End {
    at: DVec3,
    on: usize,
    facing: DVec3,
    matter_past: bool,
}

/// The point that stands `offsets` from the planes through `through` that
/// face the three ways `facing`, each offset signed the way its plane faces.
fn standing(through: DVec3, facing: [DVec3; 3], offsets: [f64; 3]) -> DVec3 {
    let [one, two, three] = facing;
    through + DMat3::from_cols(one, two, three).transpose().inverse() * DVec3::from(offsets)
}

impl Concave {
    fn length(&self) -> f64 {
        self.ends[0].at.distance(self.ends[1].at)
    }

    fn along(&self) -> DVec3 {
        (self.ends[1].at - self.ends[0].at).normalize()
    }

    /// Where a fill of `value` crosses the face an end runs into: on the
    /// edge's first face, the value out from its second, then on its second,
    /// the value out from its first.
    fn set_back(&self, end: &End, value: f64) -> [DVec3; 2] {
        let [first, second] = self.facing;
        [
            standing(end.at, [second, end.facing, first], [value, 0.0, 0.0]),
            standing(end.at, [first, end.facing, second], [value, 0.0, 0.0]),
        ]
    }

    /// Whether a point lies on the curve a round of `radius` ends in on the
    /// face an end runs into: on that face's plane, a radius from the round's
    /// axis, which stands the radius out from both faces, and on the edge's
    /// side of the axis, between the two set-backs.
    fn on_the_round_s_end(&self, end: &End, radius: f64, point: DVec3) -> bool {
        let [first, second] = self.facing;
        let axis = standing(end.at, [first, second, self.along()], [radius, radius, 0.0]);
        let off = point - axis;
        let across = off - self.along() * off.dot(self.along());
        let least = radius * first.dot(second);
        (point - end.at).dot(end.facing).abs() < PLACE
            && (across.length() - radius).abs() < PLACE
            && across.dot(-first) >= least - PLACE
            && across.dot(-second) >= least - PLACE
    }

    /// What a fill of `value` adds along the edge alone, each line along the
    /// edge through its section running from the plane of the face at one end
    /// to the plane of the face at the other: the section times the edge's
    /// length, less, where a face at an end leans across the edge, the
    /// section's first moment about the edge times how far the lines shrink a
    /// unit out from it.
    fn added(&self, cut: Cut, value: f64) -> f64 {
        let along = self.along();
        let [start, end] = &self.ends;
        let lean = end.facing / along.dot(end.facing) - start.facing / along.dot(start.facing);
        let [first, second] = self.facing;
        cut.section(value) * self.length() - cut.moment(value) * (first + second).dot(lean)
    }

    /// The ways along the edge's two faces from it, square to it, each across
    /// the air the two faces leave between them.
    fn into(&self) -> [DVec3; 2] {
        let along = self.along();
        let [first, second] = self.facing;
        let into = |own: DVec3, other: DVec3| {
            let way = own.cross(along).normalize();
            if way.dot(other) > 0.0 { way } else { -way }
        };
        [into(first, second), into(second, first)]
    }
}

/// The standing block's four feet, each running between the two walls beside
/// its own, with air past both.
fn feet() -> [Concave; 4] {
    let [front_left, front_right, back_right, back_left] = [
        DVec3::new(10.0, 5.0, 10.0),
        DVec3::new(20.0, 5.0, 10.0),
        DVec3::new(20.0, 15.0, 10.0),
        DVec3::new(10.0, 15.0, 10.0),
    ];
    let free = |at: DVec3, on: usize, facing: DVec3| End {
        at,
        on,
        facing,
        matter_past: false,
    };
    [
        Concave {
            called: "the standing block's front foot",
            between: [TOP, STANDING_FRONT],
            facing: [DVec3::Z, -DVec3::Y],
            ends: [
                free(front_left, STANDING_LEFT, -DVec3::X),
                free(front_right, STANDING_RIGHT, DVec3::X),
            ],
        },
        Concave {
            called: "the standing block's right foot",
            between: [TOP, STANDING_RIGHT],
            facing: [DVec3::Z, DVec3::X],
            ends: [
                free(front_right, STANDING_FRONT, -DVec3::Y),
                free(back_right, STANDING_BACK, DVec3::Y),
            ],
        },
        Concave {
            called: "the standing block's back foot",
            between: [TOP, STANDING_BACK],
            facing: [DVec3::Z, DVec3::Y],
            ends: [
                free(back_right, STANDING_RIGHT, DVec3::X),
                free(back_left, STANDING_LEFT, -DVec3::X),
            ],
        },
        Concave {
            called: "the standing block's left foot",
            between: [TOP, STANDING_LEFT],
            facing: [DVec3::Z, -DVec3::X],
            ends: [
                free(back_left, STANDING_BACK, DVec3::Y),
                free(front_left, STANDING_FRONT, -DVec3::Y),
            ],
        },
    ]
}

/// The two cuts a step lays on an edge, here filling it in.
#[derive(Clone, Copy, PartialEq)]
enum Cut {
    Flat,
    Round,
}

/// The body a step left, the value it was asked and the value it took.
struct Laid {
    body: Body,
    asked: f64,
    taken: f64,
}

impl Cut {
    fn called(self) -> &'static str {
        match self {
            Cut::Flat => "chamfer",
            Cut::Round => "fillet",
        }
    }

    /// The step on the edges the names find, in one step, at `value`.
    fn laid_on(self, part: &Body, edges: &[EdgeName], value: f64) -> Result<Laid, Declined> {
        match self {
            Cut::Flat => part.chamfer_edges(edges, value).map(|step| Laid {
                body: step.body,
                asked: step.asked,
                taken: step.taken,
            }),
            Cut::Round => part.fillet_edges(edges, value).map(|step| Laid {
                body: step.body,
                asked: step.asked,
                taken: step.taken,
            }),
        }
    }

    /// The section the fill adds along an edge whose faces meet square.
    fn section(self, value: f64) -> f64 {
        match self {
            Cut::Flat => value * value / 2.0,
            Cut::Round => value * value * (1.0 - PI / 4.0),
        }
    }

    /// The first moment of that section about either of the edge's faces.
    fn moment(self, value: f64) -> f64 {
        match self {
            Cut::Flat => value.powi(3) / 6.0,
            Cut::Round => value.powi(3) * (5.0 / 6.0 - PI / 4.0),
        }
    }

    /// What two fills' prisms carried past a square corner both hold past
    /// the faces their edges end on there, as the module's documentation
    /// derives it.
    fn past_the_corner(self, value: f64) -> f64 {
        match self {
            Cut::Flat => value.powi(3) / 3.0,
            Cut::Round => value.powi(3) * (5.0 / 3.0 - PI / 2.0),
        }
    }
}

/// The names of every edge between the faces numbered `first` and `second`.
fn edges_between(body: &Body, first: usize, second: usize) -> Vec<EdgeName> {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    body.edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
        .collect()
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let named = edges_between(body, first, second);
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

/// The numbers of the faces a step made: those the body answers to that the
/// part it was given did not.
fn faces_made(part: &Body, body: &Body) -> Vec<usize> {
    numbers(body).difference(&numbers(part)).copied().collect()
}

/// The sides of the faces the face numbered `face` meets, one for each edge
/// it has, in order.
fn sides_met(body: &Body, face: usize) -> Vec<Vec<u32>> {
    let own = vec![face as u32];
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
/// [`sides_met`] lists them.
fn sides(faces: &[usize]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces.iter().map(|&face| vec![face as u32]).collect();
    sorted.sort();
    sorted
}

/// The sides a face met on the part, each face numbered `from` in `renamed`
/// met now as the face numbered `to`, in the order [`sides_met`] lists them.
fn renamed(met: Vec<Vec<u32>>, renamed: &[(usize, usize)]) -> Vec<Vec<u32>> {
    let mut sides: Vec<Vec<u32>> = met
        .into_iter()
        .map(|side| {
            renamed
                .iter()
                .find(|(from, _)| side == [*from as u32])
                .map_or(side, |&(_, to)| vec![to as u32])
        })
        .collect();
    sides.sort();
    sides
}

/// Holds that a face's corners are the points expected, each of them a
/// corner, and every other corner on the curve `on` says it is on.
fn corners_are_or_on(
    body: &Body,
    face: usize,
    expected: &[DVec3],
    on: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            on(*corner) || expected.iter().any(|point| point.distance(*corner) < PLACE),
            "{what}: a corner at {corner}, where the corners are {expected:?}"
        );
    }
    for point in expected {
        assert!(
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
}

/// Holds that a flat face faces the way `toward` points.
fn faces_toward(body: &Body, face: usize, toward: DVec3, what: &str) {
    let facing = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"))
        .normal;
    assert!(
        facing.distance(toward.normalize()) < PLACE,
        "{what}: face {face} faces {facing}, where it faces {}",
        toward.normalize()
    );
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

/// The edge of `edges` other than `edge` that ends where the end `end` of
/// `edge` is, the two meeting there in a mitre, and which of its ends that
/// is; none where no other edge of the list ends there.
fn mitred_with(edges: &[Concave], edge: usize, end: usize) -> Option<(usize, usize)> {
    let at = edges[edge].ends[end].at;
    (0..edges.len())
        .filter(|&other| other != edge)
        .find_map(|other| {
            edges[other]
                .ends
                .iter()
                .position(|far| far.at.distance(at) < PLACE)
                .map(|far| (other, far))
        })
}

/// The way an edge runs from its end `end` to its other end.
fn from_end(edge: &Concave, end: usize) -> DVec3 {
    (edge.ends[1 - end].at - edge.ends[end].at).normalize()
}

/// Every corner two of `edges` meet at, once: the lower edge, its end there,
/// the other edge and its end there.
fn mitres(edges: &[Concave]) -> Vec<(usize, usize, usize, usize)> {
    (0..edges.len())
        .flat_map(|edge| {
            (0..2).filter_map(move |end| {
                mitred_with(edges, edge, end).map(|(other, far)| (edge, end, other, far))
            })
        })
        .filter(|&(edge, _, other, _)| edge < other)
        .collect()
}

/// The round a fillet lays in a concave corner: a cylinder of `radius` about
/// the line from `start` along `along` for `length`, standing in the air the
/// two faces leave between them, the round facing its axis, from the way the
/// edge's first face faces to the way its second does.
struct Hollow {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Hollow {
    /// The round of `radius` on `edge`: its axis the radius out from both of
    /// the edge's faces, from the edge's first end to its last.
    fn on(edge: &Concave, radius: f64) -> Hollow {
        let [first, second] = edge.facing;
        let along = edge.along();
        Hollow {
            start: standing(
                edge.ends[0].at,
                [first, second, along],
                [radius, radius, 0.0],
            ),
            along,
            length: edge.length(),
            radius,
            facing: edge.facing,
        }
    }

    /// The point of the axis `share` of the way along it.
    fn axis_at(&self, share: f64) -> DVec3 {
        self.start + self.along * self.length * share
    }

    /// The way the round faces `share` of the way round it, from its first
    /// face's way to its second's: toward its axis.
    fn facing_at(&self, share: f64) -> DVec3 {
        let [first, second] = self.facing;
        let turn = first.dot(second).acos() * share;
        let across = (second - first * first.dot(second)).normalize();
        first * turn.cos() + across * turn.sin()
    }

    /// Whether a point lies on the round's cylinder, on the edge's side of
    /// its axis, between its two sides; how far along it the round runs is
    /// for the faces at its ends to say.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let across = off - self.along * off.dot(self.along);
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && across.dot(-first) >= least - PLACE
            && across.dot(-second) >= least - PLACE
    }

    /// The part of the round from `from` along its axis to `to`.
    fn part(&self, from: f64, to: f64) -> Hollow {
        Hollow {
            start: self.start + self.along * from,
            length: to - from,
            ..*self
        }
    }
}

/// How far along the edge the plane of the face at its end `end` stands off
/// square across `radius` of it: nothing where that face is square to the
/// edge.
fn leaning_by(edge: &Concave, end: usize, radius: f64) -> f64 {
    let cos = edge.along().dot(edge.ends[end].facing).abs();
    radius * (1.0 - cos * cos).sqrt() / cos
}

/// How far a round of `radius` on the edge `edge` of `edges` can stand past
/// each end of the edge's own length, or short of it, along the edge: at a
/// mitre, as far as the plane halfway between the two edges reaches across
/// the round, the radius over the tangent of half their angle; elsewhere as
/// far as the face at that end leans across it.
fn overhang(edges: &[Concave], edge: usize, radius: f64) -> [f64; 2] {
    let concave = &edges[edge];
    [0, 1].map(|end| match mitred_with(edges, edge, end) {
        Some((other, far)) => {
            let cos = from_end(concave, end).dot(from_end(&edges[other], far));
            radius * (1.0 + cos) / (1.0 - cos * cos).sqrt()
        }
        None => leaning_by(concave, end, radius),
    })
}

/// How far in from its end `end` the round of `radius` on the edge `edge` of
/// `edges` is met first from its axis: past a mitre at a corner whose third
/// edge meets inwards too, where the other round takes over, as far as the
/// plane halfway between the two edges reaches across it; elsewhere as far
/// as the face at that end leans.
fn clear_of(edges: &[Concave], edge: usize, end: usize, radius: f64) -> f64 {
    match mitred_with(edges, edge, end) {
        Some(_) if edges[edge].ends[end].matter_past => overhang(edges, edge, radius)[end],
        _ => leaning_by(&edges[edge], end, radius),
    }
}

/// What the triangles of a fillet on `edges` may enclose apart from its
/// matter: the drawing's tolerance at the part's own reach, over the rounds,
/// each counted along its edge and as far past its ends as it can stand.
fn rounds_drawn_within(part: &Body, edges: &[Concave], radius: f64) -> f64 {
    let area: f64 = (0..edges.len())
        .map(|edge| {
            let [before, after] = overhang(edges, edge, radius);
            PI / 2.0 * radius * (edges[edge].length() + before + after)
        })
        .sum();
    DRAWN * reach_of(part) * area
}

/// The two corners a fill of `value` on the edge `edge` of `edges` has at its
/// end `end`, on the face the edges share and on the edge's other face. Where
/// it is mitred, the first is where the two fills' set-backs cross on the
/// face they share, and the second where they meet the third edge between
/// the two other faces, the value from the face they share; elsewhere, where
/// the fill crosses the face its edge runs into.
fn fill_corners(edges: &[Concave], edge: usize, end: usize, value: f64) -> [DVec3; 2] {
    let concave = &edges[edge];
    let [shared, own] = concave.facing;
    match mitred_with(edges, edge, end) {
        Some((other, _)) => {
            let facing = [own, edges[other].facing[1], shared];
            let at = concave.ends[end].at;
            [
                standing(at, facing, [value, value, 0.0]),
                standing(at, facing, [0.0, 0.0, value]),
            ]
        }
        None => concave.set_back(&concave.ends[end], value),
    }
}

/// What a part holding `volume` keeps once a fill of `value` on `edges` in
/// one step is laid: its volume and what each fill adds alone, trimmed flush
/// at its ends; and at each corner two of them meet at, plus what both
/// prisms carried past the corner hold beyond both faces they end on where
/// the corner's third edge meets outwards, or less what the two fills both
/// hold there where it meets inwards — the square corner's amount stretched
/// by one over the sine of the angle between the edges' other faces, as the
/// module's documentation derives it.
fn kept(volume: f64, edges: &[Concave], cut: Cut, value: f64) -> f64 {
    let alone: f64 = edges.iter().map(|edge| edge.added(cut, value)).sum();
    let at_the_corners: f64 = mitres(edges)
        .into_iter()
        .map(|(edge, end, other, _)| {
            let sine = edges[edge].facing[1].cross(edges[other].facing[1]).length();
            let joined = if edges[edge].ends[end].matter_past {
                -1.0
            } else {
                1.0
            };
            joined * cut.past_the_corner(value) / sine
        })
        .sum();
    volume + alone + at_the_corners
}

/// What a part keeps once a fill of `value` on `edges` in one step is laid,
/// held another way, by the booleans and nothing of the step: the part
/// joined, for each edge, to the prism over the fill's section carried well
/// past both its ends, clipped at a mitred end by the plane halfway between
/// the two edges, on the other's side, and at another end by the plane of the
/// face it runs into, on the far side from the edge. The section is drawn
/// from the ways along the edge's two faces, square or not: the flat's
/// corners the value along each, the round's where it touches them, the
/// radius from both.
fn by_booleans(part: &Body, edges: &[Concave], cut: Cut, value: f64) -> f64 {
    let mut body = part.clone();
    for (index, edge) in edges.iter().enumerate() {
        let along = edge.along();
        let [u, into_second] = edge.into();
        let v = (into_second - u * u.dot(into_second)).normalize();
        let angle = u.dot(into_second).acos();
        let set_back = match cut {
            Cut::Flat => value,
            Cut::Round => value / (angle / 2.0).tan(),
        };
        let corners = vec![
            DVec2::ZERO,
            DVec2::new(set_back, 0.0),
            DVec2::new(angle.cos(), angle.sin()) * set_back,
        ];
        let outline = match cut {
            Cut::Flat => Contour::straight(corners),
            Cut::Round => Contour {
                corners,
                runs: vec![
                    Run::Straight,
                    Run::Round {
                        center: DVec2::new(set_back, value),
                        turn: angle - PI,
                    },
                    Run::Straight,
                ],
            },
        };
        let [start, stop] = [
            edge.ends[0].at - along * 6.0 * value,
            edge.ends[1].at + along * 6.0 * value,
        ];
        let (origin, travel) = if u.cross(v).dot(along) > 0.0 {
            (start, stop - start)
        } else {
            (stop, start - stop)
        };
        let mut prism = body
            .tool_raised(&profile(outline), Frame { origin, u, v }, travel)
            .expect("the prism raises");
        for end in 0..2 {
            let at = &edge.ends[end];
            let inward = from_end(edge, end);
            let away = match mitred_with(edges, index, end) {
                Some((other, far)) => from_end(&edges[other], far) - inward,
                None if inward.dot(at.facing) < 0.0 => at.facing,
                None => -at.facing,
            };
            prism = prism
                .difference(&beyond(&prism, at.at, away, 12.0 * value))
                .expect("the prism is clipped");
        }
        body = body.union(&prism).expect("the clipped prism joins");
    }
    body.volume()
}

/// The slab beyond the plane through `through` square to `away`, on the side
/// `away` points to: `size` each way across it, and `size` deep.
fn beyond(part: &Body, through: DVec3, away: DVec3, size: f64) -> Body {
    let away = away.normalize();
    let u = away.any_orthonormal_vector();
    part.tool_raised(
        &profile(Contour::rectangle(DVec2::splat(-size), DVec2::splat(size))),
        Frame {
            origin: through,
            u,
            v: away.cross(u),
        },
        away * size,
    )
    .expect("the slab raises")
}

/// Holds that every ray aimed from the round's axis, a tenth to nine tenths
/// of the way along it, out to the round meets the face numbered `face`
/// first, where it faces back along the ray, toward the axis.
fn met_from_its_axis(body: &Body, face: usize, hollow: &Hollow, what: &str) {
    for along in [0.11, 0.47, 0.83] {
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = hollow.facing_at(share);
            let from = hollow.axis_at(along);
            let hit = body
                .ray_hit(from, -facing)
                .unwrap_or_else(|| panic!("{what}: a ray from {from} on the axis meets nothing"));
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} on the axis meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 toward its axis",
                hit.normal
            );
        }
    }
}

/// A step laid on a list of edges: the order the list names them in, the cut,
/// the value, what the step gave, and what to call it.
struct Case<'a> {
    order: &'a [usize],
    cut: Cut,
    value: f64,
    step: Laid,
    what: String,
}

impl Case<'_> {
    /// The number the face laid on the edge `edge` answers to: where the
    /// steps' numbers of `part` ended, plus the edge's rank in the list.
    fn laid_on(&self, part: &Body, edge: usize) -> usize {
        part.faces_end()
            + self
                .order
                .iter()
                .position(|&named| named == edge)
                .expect("every edge is named")
    }
}

/// Holds that `edges`, named in each of `orders` and filled by each of `cuts`
/// at every one of [`VALUES`] in one step, leave what [`mitred`] holds, the
/// part holding `volume` before the step and keeping what [`kept`] says after
/// it — which the booleans give too, [`by_booleans`] — and the same body
/// whatever the order, as [`the_same_body`] holds it. Every list is laid
/// before any body is read, so that a decline names every case that declines.
fn mitre_in_every_order(
    part: &Body,
    volume: f64,
    edges: &[Concave],
    orders: &[&[usize]],
    cuts: &[Cut],
    called: &str,
) {
    assert!(
        (part.volume() - volume).abs() <= EXACT * volume,
        "{called}: the part holds {}, where arithmetic gives {volume}",
        part.volume()
    );
    for &cut in cuts {
        for value in VALUES {
            let arithmetic = kept(volume, edges, cut, value);
            let booleans = by_booleans(part, edges, cut, value);
            assert!(
                (arithmetic - booleans).abs() <= EXACT * volume,
                "{called}, the {} of {value}: the arithmetic keeps {arithmetic}, the booleans \
                 {booleans}",
                cut.called()
            );
        }
    }
    let names: Vec<EdgeName> = edges
        .iter()
        .map(|edge| edge_between(part, edge.between[0], edge.between[1]))
        .collect();
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for &order in orders {
        let list: Vec<EdgeName> = order.iter().map(|&edge| names[edge].clone()).collect();
        let named: Vec<&str> = order.iter().map(|&edge| edges[edge].called).collect();
        for &cut in cuts {
            for value in VALUES {
                let what = format!(
                    "the {} of {value} on {called}, named {named:?}",
                    cut.called()
                );
                match cut.laid_on(part, &list, value) {
                    Ok(step) => laid.push(Case {
                        order,
                        cut,
                        value,
                        step,
                        what,
                    }),
                    Err(reason) => declined.push(format!("{what}: {reason:?}")),
                }
            }
        }
    }
    assert!(
        declined.is_empty(),
        "{called}: the edges filled in one step, mitred where two of them meet; declined:\n{}",
        declined.join("\n")
    );
    for case in &laid {
        let promised = kept(volume, edges, case.cut, case.value);
        mitred(part, edges, case, promised);
    }
    for case in &laid {
        let first = laid
            .iter()
            .find(|other| other.cut == case.cut && other.value == case.value)
            .expect("the case itself is among them");
        if !std::ptr::eq(first, case) {
            the_same_body(part, edges, first, case);
        }
    }
}

/// Holds that two steps naming the same edges in different orders left the
/// same body, the face laid on each edge answering to the number its rank
/// gives it in each: the same matter; the same faces, each flat or curved
/// alike and a flat one with the same corners; and the same edges between
/// the same faces.
fn the_same_body(part: &Body, edges: &[Concave], first: &Case, second: &Case) {
    let what = format!("{} and {}", first.what, second.what);
    let [one, other] = [&first.step.body, &second.step.body];
    let to_first: Vec<(usize, usize)> = (0..edges.len())
        .map(|edge| (second.laid_on(part, edge), first.laid_on(part, edge)))
        .collect();
    let as_first = |number: usize| {
        to_first
            .iter()
            .find(|(from, _)| *from == number)
            .map_or(number, |&(_, to)| to)
    };
    assert!(
        (one.volume() - other.volume()).abs() <= EXACT * one.volume(),
        "{what}: the two hold {} and {}",
        one.volume(),
        other.volume()
    );
    let renumbered: BTreeSet<usize> = numbers(other).into_iter().map(as_first).collect();
    assert_eq!(numbers(one), renumbered, "{what}: the same faces");
    for number in numbers(other) {
        let face = as_first(number);
        assert_eq!(
            one.is_flat(face),
            other.is_flat(number),
            "{what}: face {face} is flat in one and not in the other"
        );
        let (Some(mine), Some(theirs)) = (one.plane_of(face), other.plane_of(number)) else {
            continue;
        };
        let within = |corners: &[DVec3], corner: &DVec3| {
            corners.iter().any(|point| point.distance(*corner) < PLACE)
        };
        assert!(
            mine.corners
                .iter()
                .all(|corner| within(&theirs.corners, corner))
                && theirs
                    .corners
                    .iter()
                    .all(|corner| within(&mine.corners, corner)),
            "{what}: face {face} has the corners {:?} in one and {:?} in the other",
            mine.corners,
            theirs.corners
        );
    }
    let between = |body: &Body, number: &dyn Fn(usize) -> usize| {
        let mut sides: Vec<[usize; 2]> = body
            .edge_names()
            .into_iter()
            .map(|name| {
                let [first, second] = name.between.map(|side| number(side[0] as usize));
                [first.min(second), first.max(second)]
            })
            .collect();
        sides.sort();
        sides
    };
    assert_eq!(
        between(one, &|number| number),
        between(other, &as_first),
        "{what}: the same edges between the same faces"
    );
}

/// Holds what a step's fill of its value on `edges`, named in its order,
/// leaves of a part, the edges' first faces one face they share, and two of
/// the edges mitred at a corner wherever two of them end at one point:
///
/// - the value taken whole; a sound body holding `promised`;
/// - one face laid for each edge, numbered in the order the list names them,
///   and none at a corner;
/// - each face laid meeting its edge's two faces along its sides, and at each
///   end the face laid on the edge it is mitred with there, or the face its
///   edge runs into, one edge each;
/// - a round on its cylinder, facing its axis, from end to end, as far as
///   the plane halfway between two mitred edges and no farther, where the
///   other round meets it along an ellipse, and nowhere past the face its
///   edge runs into at another end;
/// - a flat running from end to end, its mitre from where the two flats
///   cross on the face they share to where they meet the third edge, the
///   value from that face;
/// - the face the edges share giving up to the fills their footprints, its
///   corner at each mitre where the two set-backs cross, and meeting the
///   faces laid in place of the edges' other faces;
/// - each edge's other face filled up by the value along it, meeting the
///   face laid on it in place of the face they share, and the third edge at
///   each corner still one sharp edge, ending where the two fills meet it;
/// - each face an edge runs into at an end not mitred grown by the fill's
///   end section in place of the corner the edge ended at, and meeting both
///   the edge's faces along one edge still, for a name to find.
fn mitred(part: &Body, edges: &[Concave], case: &Case, promised: f64) {
    let Case {
        cut,
        value,
        step,
        what,
        ..
    } = case;
    let (cut, value) = (*cut, *value);
    taken_as_asked(step.asked, step.taken, value, what);
    let body = &step.body;
    let laid: Vec<usize> = (0..edges.len())
        .map(|edge| case.laid_on(part, edge))
        .collect();
    let drawn = match cut {
        Cut::Flat => EXACT * promised,
        Cut::Round => rounds_drawn_within(part, edges, value),
    };
    sound_drawn_within(body, promised, drawn, what);
    let mut made = laid.clone();
    made.sort();
    assert_eq!(
        faces_made(part, body),
        made,
        "{what}: one face for each edge, numbered in the order the step names them, and no \
         face laid at a corner: {:?}",
        numbers(body)
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    let shared = edges[0].between[0];
    let shared_facing = edges[0].facing[0];
    let corners_of = |face: usize| {
        part.plane_of(face)
            .unwrap_or_else(|| panic!("{what}: the part offers no plane on face {face}"))
            .corners
    };
    let end_at = |edge: usize, corner: DVec3| {
        (0..2).find(|&end| edges[edge].ends[end].at.distance(corner) < PLACE)
    };
    for (index, edge) in edges.iter().enumerate() {
        let face = laid[index];
        let [_, own] = edge.between;
        let [_, own_facing] = edge.facing;
        let partners = [0, 1].map(|end| mitred_with(edges, index, end));
        let [met_first, met_last] =
            [0, 1].map(|end| partners[end].map_or(edge.ends[end].on, |(other, _)| laid[other]));
        assert_eq!(
            sides_met(body, face),
            sides(&[shared, own, met_first, met_last]),
            "{what}: the face laid on {} meets its edge's two faces along its sides, and at each \
             end the face laid on the edge it is mitred with, or the face it runs into, by one \
             edge each",
            edge.called
        );
        match cut {
            Cut::Flat => {
                assert!(
                    body.is_flat(face),
                    "{what}: the chamfer lays a flat face on {}",
                    edge.called
                );
                let corners: Vec<DVec3> = [0, 1]
                    .into_iter()
                    .flat_map(|end| fill_corners(edges, index, end, value))
                    .collect();
                corners_are_or_on(
                    body,
                    face,
                    &corners,
                    |_| false,
                    &format!(
                        "{what}: the flat on {}, from end to end, its mitre at a corner \
                         running from where the two flats cross on the face they share to \
                         where they meet the third edge",
                        edge.called
                    ),
                );
                faces_toward(
                    body,
                    face,
                    shared_facing + own_facing,
                    &format!("{what}: the flat on {}", edge.called),
                );
            }
            Cut::Round => {
                let hollow = Hollow::on(edge, value);
                let which = format!("{what}: the round on {}", edge.called);
                assert!(
                    !body.is_flat(face) && body.plane_of(face).is_none(),
                    "{which} is curved, and offers no plane to draw on"
                );
                let drawn: Vec<DVec3> = body.triangles_of(face).flatten().collect();
                assert!(!drawn.is_empty(), "{which} is drawn");
                for point in drawn {
                    assert!(
                        hollow.holds(point),
                        "{which} is drawn through {point}, off its cylinder of {value} about the \
                         line from {} along {}",
                        hollow.start,
                        hollow.along
                    );
                    for (end, partner) in partners.iter().enumerate() {
                        let at = &edge.ends[end];
                        let inward = from_end(edge, end);
                        match *partner {
                            Some((other, far)) => assert!(
                                (point - at.at).dot(inward)
                                    >= (point - at.at).dot(from_end(&edges[other], far)) - PLACE,
                                "{which} is drawn through {point}, past the plane halfway between \
                                 it and {}, where the other round meets it in the mitre",
                                edges[other].called
                            ),
                            None => assert!(
                                (point - at.at).dot(at.facing) * inward.dot(at.facing).signum()
                                    >= -PLACE,
                                "{which} is drawn through {point}, past face {}, which its edge \
                                 runs into there",
                                at.on
                            ),
                        }
                    }
                }
                let [before, after] = [0, 1].map(|end| clear_of(edges, index, end, value));
                met_from_its_axis(
                    body,
                    face,
                    &hollow.part(before, edge.length() - after),
                    &format!("{which}, between its edge's ends"),
                );
            }
        }
        let filled_up: Vec<DVec3> = corners_of(own)
            .into_iter()
            .map(|corner| {
                end_at(index, corner)
                    .map_or(corner, |end| fill_corners(edges, index, end, value)[1])
            })
            .collect();
        corners_are_or_on(
            body,
            own,
            &filled_up,
            |_| false,
            &format!(
                "{what}: face {own}, the other face of {}, filled up by the value along it",
                edge.called
            ),
        );

        assert_eq!(
            sides_met(body, own),
            renamed(sides_met(part, own), &[(shared, face)]),
            "{what}: face {own}, the other face of {}, meets the face laid on it in place of face \
             {shared}, and every other face it met",
            edge.called
        );
        for (end, at) in edge.ends.iter().enumerate() {
            if partners[end].is_some() {
                continue;
            }
            let ended_on = at.on;
            let kept_corners = corners_of(ended_on)
                .into_iter()
                .filter(|corner| corner.distance(at.at) >= PLACE);
            let expected: Vec<DVec3> = kept_corners.chain(edge.set_back(at, value)).collect();
            corners_are_or_on(
                body,
                ended_on,
                &expected,
                |corner| cut == Cut::Round && edge.on_the_round_s_end(at, value, corner),
                &format!(
                    "{what}: face {ended_on}, which {} runs into, grown by the fill's end \
                     section in place of the corner the edge ended at",
                    edge.called
                ),
            );

            for side in edge.between {
                let meeting = edges_between(body, ended_on, side);
                assert_eq!(
                    meeting.len(),
                    1,
                    "{what}: face {ended_on}, which {} runs into, meets face {side} along one \
                     edge still: {meeting:?}",
                    edge.called
                );
            }
        }
    }
    let given_up: Vec<DVec3> = corners_of(shared)
        .into_iter()
        .map(|corner| {
            (0..edges.len())
                .find_map(|edge| {
                    end_at(edge, corner).map(|end| fill_corners(edges, edge, end, value)[0])
                })
                .unwrap_or(corner)
        })
        .collect();
    corners_are_or_on(
        body,
        shared,
        &given_up,
        |_| false,
        &format!(
            "{what}: face {shared}, which the edges share, giving up to the fills their \
             footprints, its corner at each mitre where the two set-backs cross"
        ),
    );
    let replaced: Vec<(usize, usize)> = edges
        .iter()
        .zip(&laid)
        .map(|(edge, &face)| (edge.between[1], face))
        .collect();
    assert_eq!(
        sides_met(body, shared),
        renamed(sides_met(part, shared), &replaced),
        "{what}: face {shared}, which the edges share, meets the faces laid in place of the \
         edges' other faces, and every other face it met"
    );
    for (edge, end, other, _) in mitres(edges) {
        let [one, two] = [edges[edge].between[1], edges[other].between[1]];
        assert_eq!(
            edges_between(body, one, two).len(),
            1,
            "{what}: the third edge at the corner {}, between faces {one} and {two}, is still one \
             sharp edge",
            edges[edge].ends[end].at
        );
    }
}

/// The two orders a list can name two edges in.
const BOTH_ORDERS: [&[usize]; 2] = [&[0, 1], &[1, 0]];

/// Two of the orders a list can name the standing block's four feet in:
/// round the block, and the front and the back first.
const ROUND_AND_ACROSS: [&[usize]; 2] = [&[0, 1, 2, 3], &[0, 2, 1, 3]];

#[test]
fn a_chamfer_and_a_fillet_on_two_feet_of_a_standing_block_fill_them_into_a_mitre_outside_its_upright_left_sharp()
 {
    let part = stacked();
    let [front, right, ..] = feet();
    mitre_in_every_order(
        &part,
        STACKED_VOLUME,
        &[front, right],
        &BOTH_ORDERS,
        &[Cut::Flat, Cut::Round],
        "the standing block's front and right feet, at its front right corner",
    );
    let [_, right, back, _] = feet();
    mitre_in_every_order(
        &part,
        STACKED_VOLUME,
        &[right, back],
        &BOTH_ORDERS,
        &[Cut::Flat, Cut::Round],
        "the standing block's right and back feet, at its back right corner",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_the_four_feet_of_a_standing_block_mitre_its_four_corners() {
    mitre_in_every_order(
        &stacked(),
        STACKED_VOLUME,
        &feet(),
        &ROUND_AND_ACROSS,
        &[Cut::Flat, Cut::Round],
        "the standing block's four feet",
    );
}

/// The lower block of [`stacked`] with a block standing on it whose outline
/// is not a rectangle: a trapezoid 10 high, its front from (10, 5) to
/// (20, 5), its right wall leaning out from there to (24, 15), its back
/// running from there to (10, 15) and its left wall upright, numbered as
/// [`stacked`]'s. Its corner at the front right foot holds 111.8° of matter,
/// the one at the back right 68.2°; the two on the left are square.
fn leaning_stacked() -> Body {
    joined(
        &joined(
            &Body::default(),
            Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
            0.0,
            10.0,
        ),
        Contour::straight(vec![
            DVec2::new(10.0, 5.0),
            DVec2::new(20.0, 5.0),
            DVec2::new(24.0, 15.0),
            DVec2::new(10.0, 15.0),
        ]),
        10.0,
        10.0,
    )
}

/// What [`leaning_stacked`] holds: the lower block, and the trapezoid 10 at
/// its front and 14 at its back, 10 deep, raised 10.
const LEANING_STACKED_VOLUME: f64 = 30.0 * 20.0 * 10.0 + (10.0 + 14.0) / 2.0 * 10.0 * 10.0;

/// The four feet of the block standing on [`leaning_stacked`], each running
/// between the two walls beside its own, with air past both: the front and
/// the back foot ending on the leaning right wall, the right foot running
/// along it.
fn leaning_feet() -> [Concave; 4] {
    let [front_left, front_right, back_right, back_left] = [
        DVec3::new(10.0, 5.0, 10.0),
        DVec3::new(20.0, 5.0, 10.0),
        DVec3::new(24.0, 15.0, 10.0),
        DVec3::new(10.0, 15.0, 10.0),
    ];
    let right = DVec3::new(10.0, -4.0, 0.0).normalize();
    let free = |at: DVec3, on: usize, facing: DVec3| End {
        at,
        on,
        facing,
        matter_past: false,
    };
    [
        Concave {
            called: "the leaning block's front foot",
            between: [TOP, STANDING_FRONT],
            facing: [DVec3::Z, -DVec3::Y],
            ends: [
                free(front_left, STANDING_LEFT, -DVec3::X),
                free(front_right, STANDING_RIGHT, right),
            ],
        },
        Concave {
            called: "the leaning block's right foot",
            between: [TOP, STANDING_RIGHT],
            facing: [DVec3::Z, right],
            ends: [
                free(front_right, STANDING_FRONT, -DVec3::Y),
                free(back_right, STANDING_BACK, DVec3::Y),
            ],
        },
        Concave {
            called: "the leaning block's back foot",
            between: [TOP, STANDING_BACK],
            facing: [DVec3::Z, DVec3::Y],
            ends: [
                free(back_right, STANDING_RIGHT, right),
                free(back_left, STANDING_LEFT, -DVec3::X),
            ],
        },
        Concave {
            called: "the leaning block's left foot",
            between: [TOP, STANDING_LEFT],
            facing: [DVec3::Z, -DVec3::X],
            ends: [
                free(back_left, STANDING_BACK, DVec3::Y),
                free(front_left, STANDING_FRONT, -DVec3::Y),
            ],
        },
    ]
}

#[test]
fn a_chamfer_and_a_fillet_on_two_feet_at_a_corner_of_a_standing_block_that_is_not_square_fill_them_into_a_mitre()
 {
    let part = leaning_stacked();
    let [front, right, ..] = leaning_feet();
    mitre_in_every_order(
        &part,
        LEANING_STACKED_VOLUME,
        &[front, right],
        &BOTH_ORDERS,
        &[Cut::Flat, Cut::Round],
        "the leaning block's front and right feet, at its front right corner, 111.8° of matter",
    );
    let [_, right, back, _] = leaning_feet();
    mitre_in_every_order(
        &part,
        LEANING_STACKED_VOLUME,
        &[right, back],
        &BOTH_ORDERS,
        &[Cut::Flat, Cut::Round],
        "the leaning block's right and back feet, at its back right corner, 68.2° of matter",
    );
    mitre_in_every_order(
        &part,
        LEANING_STACKED_VOLUME,
        &leaning_feet(),
        &ROUND_AND_ACROSS,
        &[Cut::Flat, Cut::Round],
        "the leaning block's four feet",
    );
}
