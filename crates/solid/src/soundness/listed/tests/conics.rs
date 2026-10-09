//! Listings bounded by a conic, each cut by the kernel's own construction
//! and put together by hand: a shaft cut by a slanted plane along an
//! ellipse, a point's tip cut off at a slant along another, a flat milled on
//! a chamfer along a hyperbola, and a point cut parallel to its slope along a
//! parabola — each read by soundness's own formula, and found out when it
//! stands off what it bounds.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::super::conic;
use super::{Mislisted, REACH, listed, turned_round, with_sides};
use crate::brep::{
    Arm, Circle, Cone, Conic, Curve, Cylinder, Line, ListedEdge, ListedFace, Listing, Plane, Round,
    Scale, Surface,
};

/// The shaft's radius and height, and the point's: its tip stands 20 over a
/// base of radius 10.
const RADIUS: f64 = 5.0;
const TIP: f64 = 20.0;
const BASE: f64 = 10.0;

/// The one conic a plane cuts a wall along, as the kernel builds it.
fn cut(plane: &Plane, round: Round, reach: f64) -> Conic {
    let [conic] = Conic::cut(plane, &round, Scale::of(reach))[..] else {
        panic!("one conic where {plane:?} cuts {round:?}");
    };
    conic
}

fn face(surface: Surface, outward: bool, loops: Vec<Vec<(usize, bool)>>) -> ListedFace {
    ListedFace {
        surface,
        outward,
        loops,
        apex: None,
    }
}

fn edge(curve: Curve, from: f64, to: f64, ends: Option<[usize; 2]>) -> ListedEdge {
    ListedEdge {
        curve,
        from,
        to,
        ends,
        sides: Vec::new(),
    }
}

/// A shaft standing on the XY plane, its top cut by the plane through
/// `(0, 0, 10)` leaning along X by `lean`: a wall between a ring and a whole
/// ellipse, neither with a vertex.
fn shaft_cut_at_a_slant(lean: f64) -> Listing {
    let cylinder = Cylinder::about(DVec3::ZERO, DVec3::Z, RADIUS);
    let (slant, turned) = Plane::through(DVec3::Z * 10.0, DVec3::new(lean, 0.0, 1.0));
    let ellipse = cut(&slant, Round::Cylinder(cylinder), REACH);
    let period = TAU / ellipse.k.sqrt();
    let (foot, _) = Plane::through(DVec3::ZERO, DVec3::Z);
    with_sides(Listing {
        faces: vec![
            face(Surface::Plane(slant), !turned, vec![vec![(1, true)]]),
            face(Surface::Plane(foot), false, vec![vec![(0, false)]]),
            face(
                Surface::Cylinder(cylinder),
                true,
                vec![vec![(0, true)], vec![(1, false)]],
            ),
        ],
        edges: vec![
            edge(
                Curve::Circle(Circle::on(&cylinder, 0.0)),
                -TAU / 2.0,
                TAU / 2.0,
                None,
            ),
            edge(Curve::Conic(ellipse), 0.0, period, None),
        ],
        vertices: Vec::new(),
    })
}

/// A point about Z, its base of radius 10 on the XY plane, its tip at
/// height 20.
fn point() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, BASE), DVec2::new(TIP, 0.0)],
    )
}

/// The point's tip above the plane `z = 12 + lean·x`, less steep than its
/// slope: its wall holds its apex within it and goes round from the
/// ellipse the plane cuts.
fn tip_cut_at_a_slant(lean: f64) -> Listing {
    let cone = point();
    let (slant, turned) = Plane::through(DVec3::Z * 12.0, DVec3::new(lean, 0.0, -1.0));
    let ellipse = cut(&slant, Round::Cone(cone), TIP);
    let period = TAU / ellipse.k.sqrt();
    let mut wall = face(Surface::Cone(cone), false, vec![vec![(0, true)]]);
    wall.apex = Some(0);
    with_sides(Listing {
        faces: vec![
            face(Surface::Plane(slant), !turned, vec![vec![(0, false)]]),
            wall,
        ],
        edges: vec![edge(Curve::Conic(ellipse), 0.0, period, None)],
        vertices: vec![cone.apex()],
    })
}

/// A cone standing on the XY plane, milled down by the plane through
/// `flat` looking along `outward`, which cuts it along an open conic and
/// leaves a chord on the foot: the wall between the foot's long arc and the
/// conic, `ring` closing it round its top where it has one.
fn milled(cone: Cone, flat: DVec3, outward: DVec3, ring: Option<f64>, reach: f64) -> Listing {
    let (plane, turned) = Plane::through(flat, outward);
    let (foot, _) = Plane::through(DVec3::ZERO, DVec3::Z);
    let rim = cone.circle(0.0, cone.section(0.0));
    let half = (rim.radius * rim.radius - flat.x * flat.x).sqrt();
    let corners = [-half, half].map(|y| DVec3::new(flat.x, y, 0.0));
    let [left, right] = corners;
    let conic = cut(&plane, Round::Cone(cone), reach);
    let [from, to] = corners.map(|corner| conic.parameter(corner));
    let (bend, way) = if from < to {
        (edge(Curve::Conic(conic), from, to, Some([0, 1])), true)
    } else {
        (edge(Curve::Conic(conic), to, from, Some([1, 0])), false)
    };
    let start = rim.parameter(right);
    let arc = edge(
        Curve::Circle(rim),
        start,
        start + (rim.parameter(left) - start).rem_euclid(TAU),
        Some([1, 0]),
    );
    let line = Line::through(left, right - left);
    let chord = edge(
        Curve::Line(line),
        line.parameter(left),
        line.parameter(right),
        Some([0, 1]),
    );
    let mut faces = vec![
        face(Surface::Cone(cone), false, vec![vec![(0, true), (2, way)]]),
        face(
            Surface::Plane(plane),
            !turned,
            vec![vec![(1, true), (2, !way)]],
        ),
        face(
            Surface::Plane(foot),
            false,
            vec![vec![(1, false), (0, false)]],
        ),
    ];
    let mut edges = vec![arc, chord, bend];
    if let Some(height) = ring {
        let top = cone.circle(height, cone.section(height));
        edges.push(edge(Curve::Circle(top), 0.0, TAU, None));
        faces[0].loops.push(vec![(3, false)]);
        let (cap, _) = Plane::through(DVec3::Z * height, DVec3::Z);
        faces.push(face(Surface::Plane(cap), true, vec![vec![(3, true)]]));
    }
    with_sides(Listing {
        faces,
        edges,
        vertices: corners.to_vec(),
    })
}

/// A chamfer — a frustum about Z from radius 10 on the XY plane to radius
/// 5 at height 20 — with a flat milled down it at `x = 6`, parallel to its
/// axis, along a hyperbola.
fn chamfer_with_a_flat() -> Listing {
    let cone = Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, 10.0), DVec2::new(20.0, 5.0)],
    );
    milled(cone, DVec3::X * 6.0, DVec3::X, Some(20.0), 20.0)
}

/// The point cut by the plane `2x + z = 14`, parallel to the ruling its
/// slope runs down along X, along a parabola from `(7, ±√51, 0)` round the
/// back of the axis through `(−1.5, 0, 17)`.
fn point_cut_parallel_to_its_slope() -> Listing {
    milled(
        point(),
        DVec3::X * 7.0,
        DVec3::new(2.0, 0.0, 1.0),
        None,
        TIP,
    )
}

/// Every stretch of a conic a listing holds, with its rank.
fn conics(listing: &Listing) -> Vec<(usize, Conic, f64, f64)> {
    listing
        .edges
        .iter()
        .enumerate()
        .filter_map(|(rank, edge)| match edge.curve {
            Curve::Conic(conic) => Some((rank, conic, edge.from, edge.to)),
            _ => None,
        })
        .collect()
}

/// Every listing above, bounded by an ellipse, a hyperbola or a parabola,
/// and the reach it is read at.
fn listings() -> Vec<(Listing, f64)> {
    let mut listings: Vec<(Listing, f64)> = [0.1, -0.25, 0.45, 1.999]
        .map(|lean| (shaft_cut_at_a_slant(lean), REACH))
        .into();
    listings.extend([0.3, -0.9, 1.5, 1.9].map(|lean| (tip_cut_at_a_slant(lean), TIP)));
    listings.push((chamfer_with_a_flat(), 20.0));
    listings.push((point_cut_parallel_to_its_slope(), TIP));
    listings
}

#[test]
fn a_conic_is_read_by_a_formula_of_soundness_s_own_that_agrees_with_the_kernel_s() {
    let mut kinds = [0; 3];
    for (listing, _) in listings() {
        for (_, conic, from, to) in conics(&listing) {
            let kind = if conic.k > 0.0 {
                0
            } else if conic.k < 0.0 {
                2
            } else {
                1
            };
            kinds[kind] += 1;
            let far = conic.anchor.length() + (to - from).abs() * conic.bend.length().max(1.0);
            for piece in 0..=256 {
                let at = from + (to - from) * piece as f64 / 256.0;
                let (own, kernel) = (conic::point(&conic, at), conic.point(at));
                assert!(
                    own.distance(kernel) <= 64.0 * f64::EPSILON * far,
                    "at {at} of {conic:?}: {own} against the kernel's {kernel}"
                );
            }
        }
    }
    assert!(kinds.iter().all(|read| *read > 0), "{kinds:?}");
}

#[test]
fn a_conic_that_is_no_number_is_read_as_no_place() {
    let [(_, conic, ..)] = conics(&shaft_cut_at_a_slant(0.25))[..] else {
        panic!("one ellipse round the shaft");
    };
    for k in [f64::NAN, f64::INFINITY] {
        let place = conic::point(&Conic { k, ..conic }, 1.0);
        assert!(!place.is_finite(), "{place}");
    }
    let place = conic::point(&conic, f64::NAN);
    assert!(!place.is_finite(), "{place}");
}

#[test]
fn a_conic_s_listing_holds_to_its_own_formula() {
    for (listing, reach) in listings() {
        assert_eq!(listed(&listing, reach), Ok(()), "{listing:?}");
    }
}

/// The listing with the conic of edge `rank` changed by `change`.
fn with_conic(mut listing: Listing, rank: usize, change: impl Fn(Conic) -> Conic) -> Listing {
    let Curve::Conic(conic) = listing.edges[rank].curve else {
        panic!("edge {rank} is a conic");
    };
    listing.edges[rank].curve = Curve::Conic(change(conic));
    listing
}

#[test]
fn a_conic_edge_off_its_plane_is_mislisted() {
    let raised = |hair: f64| {
        with_conic(shaft_cut_at_a_slant(0.25), 1, |conic| Conic {
            anchor: conic.anchor + DVec3::Z * hair,
            ..conic
        })
    };
    let found = listed(&raised(1e-6), REACH);
    let Err(Mislisted::OffFace {
        edge: 1,
        face: 0,
        distance,
        ..
    }) = found
    else {
        panic!("{found:?}");
    };
    let across = 1e-6 / 0.25f64.hypot(1.0);
    assert!(
        (distance - across).abs() <= 1e-13,
        "{distance} against {across}"
    );
    assert_eq!(listed(&raised(1e-10), REACH), Ok(()));
}

#[test]
fn an_ellipse_a_hair_off_its_cylinder_is_found_off_it() {
    let slid = |hair: f64| {
        with_conic(shaft_cut_at_a_slant(0.25), 1, |conic| Conic {
            anchor: conic.anchor + DVec3::Y * hair,
            ..conic
        })
    };
    let found = listed(&slid(1e-6), REACH);
    assert!(
        matches!(found, Err(Mislisted::OffFace { edge: 1, face: 2, distance, .. }) if distance > 1e-8),
        "{found:?}"
    );
    assert_eq!(listed(&slid(1e-10), REACH), Ok(()));
}

/// The chamfer with its hyperbola bowed `hair` further along its bend at
/// its vertex, its ends left where they were: its stretch runs as far
/// before its anchor as after it, and the bend's share of a place is even
/// in the parameter.
fn chamfer_bowed(hair: f64) -> Listing {
    let listing = chamfer_with_a_flat();
    let (from, to) = (listing.edges[2].from, listing.edges[2].to);
    assert!((from + to).abs() <= 1e-12, "from {from} to {to}");
    with_conic(listing, 2, |conic| {
        let share = (conic.point(to) - conic.anchor).dot(conic.bend) / conic.bend.length_squared();
        let bowed = conic.bend.normalize() * hair;
        Conic {
            anchor: conic.anchor + bowed,
            bend: conic.bend - bowed / share,
            ..conic
        }
    })
}

#[test]
fn a_hyperbola_a_hair_off_its_cone_is_found_off_it() {
    let found = listed(&chamfer_bowed(1e-6), 20.0);
    assert!(
        matches!(found, Err(Mislisted::OffFace { edge: 2, face: 0, distance, .. }) if distance > 1e-9 * 20.0),
        "{found:?}"
    );
    assert_eq!(listed(&chamfer_bowed(1e-11), 20.0), Ok(()));
}

#[test]
fn a_conic_edge_running_onto_the_other_nappe_is_mislisted() {
    let lean = 0.3;
    let (mirrored, _) = Plane::through(DVec3::Z * 28.0, DVec3::new(lean, 0.0, -1.0));
    let above = Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(2.0 * TIP, BASE), DVec2::new(TIP, 0.0)],
    );
    let ellipse = cut(&mirrored, Round::Cone(above), TIP);
    let period = TAU / ellipse.k.sqrt();
    for piece in 0..=16 {
        let place = ellipse.point(period * piece as f64 / 16.0);
        let radius = place.truncate().length();
        assert!(
            (radius - (place.z - TIP).abs() * BASE / TIP).abs() <= 1e-12,
            "{place} on the point's other nappe"
        );
    }
    let mut listing = with_conic(tip_cut_at_a_slant(lean), 0, |_| ellipse);
    listing.edges[0].to = period;
    listing.faces[0].surface = Surface::Plane(mirrored);
    let found = listed(&listing, TIP);
    assert!(
        matches!(found, Err(Mislisted::OffFace { edge: 0, face: 1, distance, .. }) if distance > 1.0),
        "{found:?}"
    );
}

#[test]
fn a_whole_ellipse_is_an_edge_with_no_vertex_and_any_other_conic_is_not() {
    let listing = shaft_cut_at_a_slant(0.25);
    let period = listing.edges[1].to;
    for to in [period * (1.0 + 1e-12), period * (1.0 - 1e-12)] {
        let mut whole = listing.clone();
        whole.edges[1].to = to;
        assert_eq!(listed(&whole, REACH), Ok(()), "round to {to}");
    }
    let endless = Err(Mislisted::Endless { edge: 1 });
    for to in [period * (1.0 - 1e-6), period / 2.0, 2.0 * period] {
        let mut short = listing.clone();
        short.edges[1].to = to;
        assert_eq!(listed(&short, REACH), endless, "round to {to}");
    }
    for k in [0.0, -1.0] {
        let open = with_conic(listing.clone(), 1, |conic| Conic {
            k: k * conic.k,
            ..conic
        });
        assert_eq!(listed(&open, REACH), endless, "k {k}");
    }
    let split = with_conic(listing, 1, |conic| Conic {
        arm: Arm::Split { side: 1 },
        ..conic
    });
    assert_eq!(listed(&split, REACH), endless);
}

#[test]
fn a_parabola_s_face_turns_the_right_way() {
    let listing = point_cut_parallel_to_its_slope();
    assert_eq!(listed(&listing, TIP), Ok(()));
    let found = listed(&turned_round(listing.clone()), TIP);
    assert!(
        matches!(found, Err(Mislisted::Backwards { .. })),
        "{found:?}"
    );
    for face in 0..listing.faces.len() {
        let mut inside_out = listing.clone();
        inside_out.faces[face].outward = !inside_out.faces[face].outward;
        assert_eq!(
            listed(&inside_out, TIP),
            Err(Mislisted::Backwards { face, lap: 0 }),
            "face {face}"
        );
    }
}
