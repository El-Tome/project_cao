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
