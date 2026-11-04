//! What sketch · corner_cut/points.rs is held to.

use glam::DVec2;

use super::*;
use crate::WorkPlane;
use crate::sketch::{PointId, SegmentId};

/// A corner at (10, 20) opening `degrees` wide, its first side running right
/// for 100 and its second leaving at that angle for 80.
fn a_corner(degrees: f64) -> (Sketch, [PointId; 3], [SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let at = DVec2::new(10.0, 20.0);
    let corner = sketch.add_point(at);
    let right = sketch.add_point(at + DVec2::new(100.0, 0.0));
    let leaning = sketch.add_point(at + DVec2::from_angle(degrees.to_radians()) * 80.0);
    let sides = [
        sketch.add_segment(corner, right),
        sketch.add_segment(corner, leaning),
    ];
    (sketch, [corner, right, leaning], sides)
}

const SHAPES: [CutShape; 4] = [
    CutShape::Fillet { radius: 6.0 },
    CutShape::Chamfer(Chamfer::Equal(7.0)),
    CutShape::Chamfer(Chamfer::Sided {
        first: 5.0,
        second: 9.0,
    }),
    CutShape::Chamfer(Chamfer::Angled {
        along: 8.0,
        degrees: 50.0,
    }),
];

const POINTS: [CutPoint; 3] = [CutPoint::Centre, CutPoint::First, CutPoint::Second];

#[test]
fn the_rule_finds_each_point_where_the_cut_is_drawn() {
    for degrees in [30.0, 90.0, 135.0, 170.0] {
        for shape in SHAPES {
            let (mut sketch, _, [first, second]) = a_corner(degrees);
            let cut = sketch.cut_corner(first, second, shape, false, 0).unwrap();
            let Some(places) = sketch.cut_places(cut) else {
                assert_eq!(
                    sketch.drawn_cut(cut),
                    None,
                    "{shape:?} at {degrees}°: drawn where the rule finds no place",
                );
                continue;
            };

            for point in POINTS {
                let drawn = sketch.cut_point(cut, point);
                let worked_out = places.at(point).map(|place| place.value());
                match (drawn, worked_out) {
                    (Some(drawn), Some(worked_out)) => assert!(
                        drawn.distance(worked_out) < 1e-9,
                        "{shape:?} at {degrees}°, {point:?}: drawn at {drawn}, worked out at {worked_out}",
                    ),
                    (None, None) => {}
                    _ => panic!(
                        "{shape:?} at {degrees}°, {point:?}: {drawn:?} drawn, {worked_out:?} worked out"
                    ),
                }
            }
        }
    }
}

#[test]
fn each_point_moves_with_the_corner_and_its_sides_as_the_rule_says() {
    for shape in SHAPES {
        let (sketch, stands_on, [first, second]) = a_corner(70.0);
        let mut sketch = sketch;
        let cut = sketch.cut_corner(first, second, shape, false, 0).unwrap();
        let places = sketch.cut_places(cut).unwrap();
        let step = 1e-6;

        for point in POINTS {
            let Some(place) = places.at(point) else {
                continue;
            };
            for (rank, moved) in stands_on.iter().enumerate() {
                for axis in [DVec2::X, DVec2::Y] {
                    let read = |by: DVec2| {
                        let mut nudged = sketch.clone();
                        nudged.move_point(*moved, sketch.point(*moved) + by);
                        nudged.cut_point(cut, point).unwrap()
                    };
                    let measured = (read(axis * step) - read(-axis * step)) / (2.0 * step);
                    let column = rank * 2 + usize::from(axis == DVec2::Y);
                    let carried = DVec2::new(place.x.slope[column], place.y.slope[column]);
                    assert!(
                        carried.distance(measured) < 1e-6,
                        "{shape:?}, {point:?}, coordinate {column}: the rule carries {carried} where the drawing moves {measured}",
                    );
                }
            }
        }
    }
}

/// The ellipse beside the corner of `fillets_beside_a_curve`, ten across and
/// five high round (40, 30): its centre, the ends of its first axis, then of
/// its second, then the corner on it and the far end of the trait rising from
/// there — all turned about its centre by `degrees`.
fn the_ellipse_turned(degrees: f64) -> [DVec2; 7] {
    let centre = DVec2::new(40.0, 30.0);
    let turn = DVec2::from_angle(degrees.to_radians());
    [
        (40.0, 30.0),
        (30.0, 30.0),
        (50.0, 30.0),
        (40.0, 25.0),
        (40.0, 35.0),
        (46.0, 34.0),
        (46.0, 54.0),
    ]
    .map(|(x, y)| centre + turn.rotate(DVec2::new(x, y) - centre))
}

/// A fillet beside a curve each way `cut_places` reads one: beside an arc
/// bulging towards the corner, which leaves from it; beside an arc hollow
/// towards it, which ends there; at the tip of a lens, between two arcs; and
/// beside an arc of ellipse, level and turned, which ends on the end of an
/// axis — a point the cut stands on twice, as the far end of its side and as
/// what the ellipse is drawn about.
fn fillets_beside_a_curve() -> Vec<(Sketch, CornerCutId, String)> {
    let mut fillets = Vec::new();

    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let far = sketch.add_point(DVec2::new(50.0, 10.0));
    let along = sketch.add_segment(corner, far);
    let centre = sketch.add_point(DVec2::new(2.0, 10.0));
    let top = sketch.add_point(DVec2::new(2.0, 18.0));
    let arc = sketch.add_arc(centre, corner, top);
    let cut = sketch
        .cut_corner(along, arc, CutShape::Fillet { radius: 5.0 }, false, 0)
        .unwrap();
    fillets.push((
        sketch,
        cut,
        "beside an arc bulging towards the corner".into(),
    ));

    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let far = sketch.add_point(DVec2::new(24.0, 10.0));
    let along = sketch.add_segment(corner, far);
    let centre = sketch.add_point(DVec2::new(28.0, 10.0));
    let start = sketch.add_point(DVec2::new(28.0, 28.0));
    let arc = sketch.add_arc(centre, start, corner);
    let cut = sketch
        .cut_corner(along, arc, CutShape::Fillet { radius: 5.0 }, false, 0)
        .unwrap();
    fillets.push((
        sketch,
        cut,
        "beside an arc hollow towards the corner".into(),
    ));

    let mut sketch = Sketch::new(WorkPlane::XY);
    let tip = sketch.add_point(DVec2::new(0.0, 16.0));
    let [east, low, high, west] = [(12.0, 0.0), (-8.0, 0.0), (8.0, 0.0), (-12.0, 0.0)]
        .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let arcs = [
        sketch.add_arc(east, tip, low),
        sketch.add_arc(west, high, tip),
    ];
    let cut = sketch
        .cut_corner(arcs[0], arcs[1], CutShape::Fillet { radius: 5.0 }, false, 0)
        .unwrap();
    fillets.push((sketch, cut, "at the tip of a lens".into()));

    for degrees in [0.0, 35.0] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let [centre, back, front, low, high, corner, up] =
            the_ellipse_turned(degrees).map(|place| sketch.add_point(place));
        let rising = sketch.add_segment(corner, up);
        let ellipse = sketch.add_ellipse(centre, [back, front], [low, high]);
        sketch.draw_the_stretch(ellipse, corner, back);
        let cut = sketch
            .cut_corner(rising, ellipse, CutShape::Fillet { radius: 3.0 }, false, 0)
            .unwrap();
        fillets.push((
            sketch,
            cut,
            format!("beside an arc of ellipse turned {degrees}°"),
        ));
    }

    fillets
}

#[test]
fn the_rule_finds_each_point_of_a_fillet_beside_a_curve_where_it_is_drawn() {
    for (sketch, cut, what) in fillets_beside_a_curve() {
        let places = sketch
            .cut_places(cut)
            .unwrap_or_else(|| panic!("{what}: the rule finds no place for the fillet"));

        for point in POINTS {
            let drawn = sketch
                .cut_point(cut, point)
                .unwrap_or_else(|| panic!("{what}: the fillet is drawn with no {point:?}"));
            let worked_out = places
                .at(point)
                .unwrap_or_else(|| panic!("{what}: the rule works out no {point:?}"))
                .value();
            assert!(
                drawn.distance(worked_out) < 1e-9,
                "{what}, {point:?}: drawn at {drawn}, worked out at {worked_out}",
            );
        }
    }
}

#[test]
fn each_point_of_a_fillet_beside_a_curve_moves_with_what_it_stands_on_as_the_rule_says() {
    let step = 1e-6;
    for (sketch, cut, what) in fillets_beside_a_curve() {
        let places = sketch.cut_places(cut).unwrap();
        assert_eq!(places.stands_on, sketch.cut_stands_on(cut), "{what}");

        for point in POINTS {
            let place = places.at(point).unwrap();
            for column in 2 * places.stands_on.len()..COORDINATES {
                assert_eq!(
                    [place.x.slope[column], place.y.slope[column]],
                    [0.0, 0.0],
                    "{what}, {point:?}: the place leans on coordinate {column}, past what the \
                     cut stands on",
                );
            }
            for moved in (0..sketch.points().len()).map(PointId) {
                for axis in [DVec2::X, DVec2::Y] {
                    let read = |by: DVec2| {
                        let mut nudged = sketch.clone();
                        nudged.move_point(moved, sketch.point(moved) + by);
                        nudged.cut_point(cut, point).unwrap()
                    };
                    let measured = (read(axis * step) - read(-axis * step)) / (2.0 * step);
                    let lean = usize::from(axis == DVec2::Y);
                    let carried = places
                        .stands_on
                        .iter()
                        .enumerate()
                        .filter(|(_, stand)| **stand == moved)
                        .map(|(rank, _)| {
                            let column = rank * 2 + lean;
                            DVec2::new(place.x.slope[column], place.y.slope[column])
                        })
                        .sum::<DVec2>();
                    assert!(
                        carried.distance(measured) < 1e-6,
                        "{what}, {point:?}, {moved:?} moved along {axis}: the rule carries \
                         {carried} where the drawing moves {measured}",
                    );
                }
            }
        }
    }
}

#[test]
fn a_chamfer_offers_no_centre_and_a_fillet_offers_three_points() {
    let (mut sketch, _, [first, second]) = a_corner(90.0);
    let round = sketch
        .cut_corner(first, second, SHAPES[0], false, 0)
        .unwrap();
    assert_eq!(sketch.cut_points().len(), 3);

    sketch.reshape_cuts(round, SHAPES[1]);

    let offered: Vec<CutPoint> = sketch
        .cut_points()
        .iter()
        .map(|(_, point, _)| *point)
        .collect();
    assert_eq!(offered, vec![CutPoint::First, CutPoint::Second]);
    assert_eq!(sketch.cut_point(round, CutPoint::Centre), None);
}

#[test]
fn a_cut_that_does_not_fit_still_offers_its_points_where_it_is_drawn() {
    let (mut sketch, _, [first, second]) = a_corner(90.0);
    let cut = sketch
        .cut_corner(first, second, CutShape::Fillet { radius: 90.0 }, false, 0)
        .unwrap();

    assert!(!sketch.drawn_cut(cut).unwrap().fits);
    let touch = sketch.cut_point(cut, CutPoint::Second).unwrap();
    assert!((touch.distance(DVec2::new(10.0, 20.0)) - 90.0).abs() < 1e-9);
    assert_eq!(
        sketch.nearest_cut_point(touch, 0.5),
        Some((cut, CutPoint::Second))
    );
}

#[test]
fn the_nearest_point_of_a_cut_is_found_within_reach_and_not_beyond() {
    let (mut sketch, _, [first, second]) = a_corner(90.0);
    let cut = sketch
        .cut_corner(first, second, SHAPES[0], false, 0)
        .unwrap();
    let centre = sketch.cut_point(cut, CutPoint::Centre).unwrap();

    assert_eq!(
        sketch.nearest_cut_point(centre + DVec2::new(0.3, 0.0), 0.5),
        Some((cut, CutPoint::Centre))
    );
    assert_eq!(
        sketch.nearest_cut_point(centre + DVec2::new(0.6, 0.0), 0.5),
        None
    );
}
