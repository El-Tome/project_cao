//! The bodies a fillet leaves (#697), and those it leaves filling a concave
//! edge in (#778), held to the one rule of `soundness` the public body cannot
//! be asked about, since it needs the exact body itself: what it lists itself
//! as made of stands on its geometry; and the floor its radius is held to,
//! which is read at the part's own tolerance. The rest of the fillet's tests
//! are `tests/a_fillet_rounds_one_straight_edge.rs` and
//! `tests/a_chamfer_and_a_fillet_fill_a_concave_edge.rs`.

use std::collections::BTreeSet;

use super::*;
use crate::EdgeName;
use crate::brep::Scale;
use crate::soundness::{closed, listed, uncrossed};

/// An outline on the ground raised by `height` and joined to the matter so
/// far, as `cao_part` builds a step: the outline's faces numbered past the
/// part's.
fn joined(part: &Body, outline: Contour, height: f64) -> Body {
    let tool = raised(part, &exact(outline, vec![]), height);
    part.union(&tool).expect("the kernel joins it")
}

/// The name of the one edge between the faces numbered `first` and `second`.
fn between(body: &Body, first: &[u32], second: &[u32]) -> EdgeName {
    let sides = [first.to_vec(), second.to_vec()];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(named.len(), 1, "one edge between {sides:?}: {named:?}");
    named[0].clone()
}

fn block([length, breadth, height]: [f64; 3]) -> Body {
    joined(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        height,
    )
}

/// The leaning trapezoid of the fillet's tests: a front 40 long, a back 37
/// long at Y = 9, and a slant rising 3 for every 1 it comes back, meeting
/// the front at an acute corner and the back at an obtuse one; the front
/// two, the slant three, the back four.
fn leaning_trapezoid() -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            DVec2::new(37.0, 9.0),
            DVec2::new(0.0, 9.0),
        ]),
        5.0,
    )
}

/// The trapezoid of the chamfer's tests: a front 40 long, a back 34 long at
/// Y = 8, and a slant between them; the top one, the front two, the slant
/// three, the back four.
fn trapezoid() -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            DVec2::new(34.0, 8.0),
            DVec2::new(0.0, 8.0),
        ]),
        5.0,
    )
}

/// A regular hexagon of side 10 raised 5, its front the side along the
/// bottom: the top one, the front two.
fn hexagon() -> Body {
    let half = 5.0 * 3.0f64.sqrt();
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::new(-5.0, -half),
            DVec2::new(5.0, -half),
            DVec2::new(10.0, 0.0),
            DVec2::new(5.0, half),
            DVec2::new(-5.0, half),
            DVec2::new(-10.0, 0.0),
        ]),
        5.0,
    )
}

/// A slot 4 wide raised 20, its front 28 long, its round end of half a turn
/// about `(28 + off, 2)`: turning in from the front at a few degrees once
/// moved back.
fn slot(off: f64) -> Body {
    let (end, centre) = (DVec2::new(28.0, 0.0), DVec2::new(28.0 + off, 2.0));
    joined(
        &Body::default(),
        Contour {
            corners: vec![DVec2::ZERO, end, 2.0 * centre - end, DVec2::new(0.0, 4.0)],
            runs: vec![
                Run::Straight,
                Run::Round {
                    center: centre,
                    turn: PI,
                },
                Run::Straight,
                Run::Straight,
            ],
        },
        20.0,
    )
}

/// Two blocks 15 by 10 by 10 side by side, the second `from` along: apart,
/// or abutting so that their tops and their fronts merge.
fn two_blocks(from: f64) -> Body {
    joined(
        &block([15.0, 10.0, 10.0]),
        Contour::rectangle(DVec2::new(from, 0.0), DVec2::new(from + 15.0, 10.0)),
        10.0,
    )
}

/// The L of the fillet's tests: an arm 30 long and 6 deep, and at its left
/// end a stem 12 wide reaching back to 14, raised 10; the top one, the
/// stem's back six.
fn ell() -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(12.0, 14.0),
            DVec2::new(0.0, 14.0),
        ]),
        10.0,
    )
}

/// A block 30 by 10 raised 5 whose front bends back by `turn` halfway along:
/// its first half two, from the origin to (15, 0), and its second half
/// three, 15 long, meeting at 180 degrees less `turn`.
fn bent_front(turn: f64) -> Body {
    joined(
        &Body::default(),
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(15.0, 0.0),
            DVec2::new(15.0, 0.0) + DVec2::from_angle(turn) * 15.0,
            DVec2::new(30.0, 10.0),
            DVec2::new(0.0, 10.0),
        ]),
        5.0,
    )
}

/// The [`ell`] with its top cut away down to the plane z = 8 − x / 5 + y / 10
/// by a box raised off a sketch lying on it, as a part's next step cuts it:
/// the leaning face eight, the inner corner's top end on it.
fn leaning_ell() -> Body {
    let ell = ell();
    let facing = DVec3::new(0.2, -0.1, 1.0).normalize();
    let u = DVec3::new(1.0, 0.0, -0.2).normalize();
    let on_the_lean = Frame {
        origin: DVec3::Z * 8.0,
        u,
        v: facing.cross(u),
    };
    let wider = Contour::rectangle(DVec2::new(-4.0, -4.0), DVec2::new(36.0, 20.0));
    let tool = ell
        .tool_raised(&exact(wider, vec![]), on_the_lean, facing * 10.0)
        .expect("the kernel raises it");
    ell.difference(&tool).expect("the kernel cuts it")
}

/// A plate 40 by 30 and 4 thick and a rib joined on its top as a part's next
/// step is, from (18, 5) to (22, 22) and 10 high: the plate's top one, the
/// rib's right side nine, its front eight and its back ten. Its foot ends
/// short of the plate's edges, where filling it rejoins the edges either side
/// of each end.
fn ribbed() -> Body {
    let plate = block([40.0, 30.0, 4.0]);
    let on_the_plate = Frame {
        origin: DVec3::Z * 4.0,
        ..FLAT
    };
    let rib = Contour::rectangle(DVec2::new(18.0, 5.0), DVec2::new(22.0, 22.0));
    let tool = plate
        .tool_raised(&exact(rib, vec![]), on_the_plate, DVec3::Z * 10.0)
        .expect("the kernel raises it");
    plate.union(&tool).expect("the kernel joins it")
}

#[test]
fn every_body_a_fillet_leaves_lists_what_it_is_made_of_as_its_geometry_stands() {
    let block = block([30.0, 10.0, 20.0]);
    let leaning = leaning_trapezoid();
    let (trapezoid, hexagon) = (trapezoid(), hexagon());
    let inwards = slot(-0.1);
    let abutting = two_blocks(15.0);
    let first_block_s = between(&two_blocks(20.0), &[1], &[2]);
    let ell = ell();
    let bent_by_30 = bent_front(30f64.to_radians());
    let bent_by_a_tenth = bent_front(0.1f64.to_radians());
    let ribbed = ribbed();
    let leaning_ell = leaning_ell();
    let cases = [
        (
            "the L's inner corner filled by 3",
            &ell,
            between(&ell, &[4], &[5]),
            3.0,
        ),
        (
            "the L's inner corner filled by 50, the stem's right side taken whole",
            &ell,
            between(&ell, &[4], &[5]),
            50.0,
        ),
        (
            "the rib's foot filled by 3, the edges round its ends rejoined",
            &ribbed,
            between(&ribbed, &[1], &[9]),
            3.0,
        ),
        (
            "the leaning L's inner corner filled by 3, trimmed by its leaning top",
            &leaning_ell,
            between(&leaning_ell, &[4], &[5]),
            3.0,
        ),
        (
            "a front bent to 150 degrees, by 5",
            &bent_by_30,
            between(&bent_by_30, &[2], &[3]),
            5.0,
        ),
        (
            "a front bent to 179.9 degrees, by 1000, its round standing clear of both halves",
            &bent_by_a_tenth,
            between(&bent_by_a_tenth, &[2], &[3]),
            1000.0,
        ),
        (
            "the block's top front edge by 50, the whole top taken",
            &block,
            between(&block, &[1], &[2]),
            50.0,
        ),
        (
            "the leaning trapezoid's acute edge by 50, the whole slant taken",
            &leaning,
            between(&leaning, &[2], &[3]),
            50.0,
        ),
        (
            "the edge a join bridged from two blocks' by 2",
            &abutting,
            first_block_s,
            2.0,
        ),
        (
            "the L's stem's back top edge by 2",
            &ell,
            between(&ell, &[1], &[6]),
            2.0,
        ),
        (
            "the block's top front edge by 3",
            &block,
            between(&block, &[1], &[2]),
            3.0,
        ),
        (
            "the leaning trapezoid's acute edge by 2",
            &leaning,
            between(&leaning, &[2], &[3]),
            2.0,
        ),
        (
            "the leaning trapezoid's obtuse edge by 2",
            &leaning,
            between(&leaning, &[3], &[4]),
            2.0,
        ),
        (
            "the hexagon's front top edge by 1, running on past both its ends",
            &hexagon,
            between(&hexagon, &[1], &[2]),
            1.0,
        ),
        (
            "the trapezoid's slanted top edge by 1, trimmed by the front and the back",
            &trapezoid,
            between(&trapezoid, &[1], &[3]),
            1.0,
        ),
        (
            "the slot's front, its round turning in at a few degrees, by 1",
            &inwards,
            between(&inwards, &[1], &[2]),
            1.0,
        ),
    ];

    for (what, part, name, radius) in cases {
        let filleted = part
            .fillet(&name, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let body = filleted.body.brep().expect("a fillet leaves an exact body");
        assert_eq!(
            listed(&body.listing(), body.scale().reach()),
            Ok(()),
            "{what}"
        );
    }
}

/// How many `Scale::HAIR` tolerances, at the part's scale, a fillet's round
/// must stand off the edge's two faces, at the middle of its turn, not to be
/// declined as too short: the floor `docs/exact-kernel.md` and
/// `Declined::Distance` give it, the chamfer's own.
const FLOOR_IN_HAIRS: f64 = 4.0;

/// Every number some face of the body answers to.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// An edge to fillet, and the round a radius lays on it: how far a round of
/// radius one stands off the two faces, `1 − sin(α/2)`; the matter a round
/// of radius one takes, the section between the faces and the round times the
/// edge's length; and whether a point lies on the round of a radius, its
/// cylinder's axis read off the faces.
struct Edge {
    what: &'static str,
    part: Body,
    sides: [u32; 2],
    standing: f64,
    taken_per_square: f64,
    on_the_round: fn(DVec3, f64, f64) -> bool,
}

/// The radius whose round stands off the edge's two faces by exactly the
/// floor, at the part's scale.
fn floor_radius(edge: &Edge) -> f64 {
    let scale = edge
        .part
        .brep()
        .expect("the exact kernel computed the part")
        .scale();
    FLOOR_IN_HAIRS * Scale::HAIR * scale.eps() / edge.standing
}

/// What is wrong with the body a fillet of `radius` left on the edge, when
/// it is no sound round of its own: one face more, answering to the number
/// the steps' numbers ended at and to nothing else, curved, every corner of
/// its triangles on the round within the part's tolerance; the edge gone,
/// every other face kept, the body closed, uncrossed, and holding what
/// arithmetic says.
fn fault_of_the_round(edge: &Edge, name: &EdgeName, radius: f64) -> Option<String> {
    let part = &edge.part;
    let filleted = match part.fillet(name, radius) {
        Ok(filleted) => filleted,
        Err(declined) => return Some(format!("declined {declined:?}")),
    };
    let body = &filleted.body;
    let Some(brep) = body.brep() else {
        return Some("left flats".to_string());
    };
    let eps = part.brep().expect("the part is exact").scale().eps();
    let made = part.faces_end();
    let triangles = body.triangles();
    let faces: Vec<_> = brep
        .face_ids()
        .filter(|&face| brep.numbers(face).contains(&(made as u32)))
        .collect();
    let (before, after) = (numbers(part), numbers(body));
    let kept: BTreeSet<usize> = before
        .iter()
        .copied()
        .filter(|number| !edge.sides.contains(&(*number as u32)))
        .collect();
    let mut made_and_kept = before.clone();
    made_and_kept.insert(made);
    let promised = part.volume() - edge.taken_per_square * radius * radius;
    if (filleted.taken - radius).abs() > 1e-9 * radius {
        Some(format!("took {}", filleted.taken))
    } else if closed(&triangles).is_err() || uncrossed(&triangles).is_err() {
        Some("left a body not closed, or with a face through another".to_string())
    } else if faces.is_empty()
        || faces
            .iter()
            .any(|&face| brep.numbers(face) != [made as u32])
        || body.is_flat(made)
        || body.plane_of(made).is_some()
    {
        Some(format!("left no curved face answering to {made} alone"))
    } else if body
        .triangles_of(made)
        .flatten()
        .any(|corner| !(edge.on_the_round)(corner, radius, eps))
    {
        Some("drew its round off the cylinder of the radius".to_string())
    } else if body.lost(name).is_none() {
        Some("left the edge where it was".to_string())
    } else if !kept.is_subset(&after) || !after.is_subset(&made_and_kept) {
        Some(format!(
            "left the faces {after:?}, where the part had {before:?}"
        ))
    } else if (body.volume() - promised).abs() > 1e-9 * promised {
        Some(format!(
            "holds {}, where arithmetic promised {promised}",
            body.volume()
        ))
    } else {
        None
    }
}

#[test]
fn a_fillet_whose_round_stands_just_under_four_hair_tolerances_off_its_faces_is_declined_and_just_over_is_laid()
 {
    let turn = 1f64.to_radians();
    let edges = [
        Edge {
            what: "a block's square top front edge, 30 long",
            part: block([30.0, 10.0, 20.0]),
            sides: [1, 2],
            standing: 1.0 - std::f64::consts::FRAC_1_SQRT_2,
            taken_per_square: 30.0 * (1.0 - PI / 4.0),
            on_the_round: |corner, radius, eps| {
                let axis = DVec2::new(radius, 20.0 - radius);
                ((DVec2::new(corner.y, corner.z) - axis).length() - radius).abs() <= eps
                    && (-eps..=30.0 + eps).contains(&corner.x)
            },
        },
        Edge {
            what: "a front bent to 179 degrees halfway along, 5 high",
            part: bent_front(turn),
            sides: [2, 3],
            standing: 2.0 * (turn / 4.0).sin().powi(2),
            taken_per_square: ((turn / 2.0).tan() - turn / 2.0) * 5.0,
            on_the_round: |corner, radius, eps| {
                let turn = 1f64.to_radians();
                let axis = DVec2::new(15.0 - radius * (turn / 2.0).tan(), radius);
                (corner.truncate().distance(axis) - radius).abs() <= eps
                    && (-eps..=5.0 + eps).contains(&corner.z)
            },
        },
    ];

    for edge in &edges {
        let what = edge.what;
        let name = between(&edge.part, &[edge.sides[0]], &[edge.sides[1]]);
        let floor = floor_radius(edge);
        let (under, over) = (0.99 * floor, 1.01 * floor);

        let declined = edge.part.fillet(&name, under).err();
        let fault = fault_of_the_round(edge, &name, over);

        assert_eq!(
            declined,
            Some(Declined::Distance),
            "{what}: {under:e} asked, its round standing {:e} off the faces, under the floor of \
             {:e}",
            under * edge.standing,
            floor * edge.standing
        );
        assert_eq!(
            fault,
            None,
            "{what}: {over:e} asked, its round standing {:e} off the faces, over the floor of \
             {:e}, is a sound round of its own",
            over * edge.standing,
            floor * edge.standing
        );
    }
}
