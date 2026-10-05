use super::*;
use crate::plane::WorkPlane;
use crate::regions::Leg;
use crate::sketch::PointId;

fn rectangle(sketch: &mut Sketch, min: DVec2, max: DVec2) {
    let corners = [
        sketch.add_point(min),
        sketch.add_point(DVec2::new(max.x, min.y)),
        sketch.add_point(max),
        sketch.add_point(DVec2::new(min.x, max.y)),
    ];
    for index in 0..4 {
        sketch.add_segment(corners[index], corners[(index + 1) % 4]);
    }
}

fn area(region: &Region) -> f64 {
    region
        .face_triangles()
        .iter()
        .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
        .sum()
}

fn areas(pieces: &[Region]) -> Vec<f64> {
    pieces.iter().map(area).collect()
}

fn assert_areas(pieces: &[Region], expected: &[f64]) {
    let found = areas(pieces);
    assert_eq!(
        found.len(),
        expected.len(),
        "pieces of areas {found:?}, expected {expected:?}"
    );
    for (found_one, expected_one) in found.iter().zip(expected) {
        assert!(
            (found_one - expected_one).abs() < 1e-9,
            "pieces of areas {found:?}, expected {expected:?}"
        );
    }
}

/// How far past the axis a corner of a piece may stand: where an arc is cut,
/// the crossing is worked out on its circle, a rounding away from the axis.
const ROUNDING: f64 = 1e-12;

fn assert_left_then_right_of_v(pieces: &[Region]) {
    assert_eq!(pieces.len(), 2);
    let left = &pieces[0].outline.points;
    let right = &pieces[1].outline.points;
    assert!(
        left.iter().all(|point| point.x <= ROUNDING),
        "the first piece stands left of the axis: {left:?}"
    );
    assert!(
        right.iter().all(|point| point.x >= -ROUNDING),
        "the second piece stands right of the axis: {right:?}"
    );
}

fn the_only_area(sketch: &Sketch) -> Region {
    let regions = sketch.regions();
    assert_eq!(regions.len(), 1, "the drawing encloses one area");
    regions.into_iter().next().unwrap()
}

#[test]
fn an_area_across_the_v_axis_is_cut_into_its_two_sides() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 2.0));
    let region = the_only_area(&sketch);

    let pieces = sketch.pieces_across(&region, DVec2::ZERO, DVec2::Y);

    assert_areas(&pieces, &[8.0, 10.0]);
    assert_left_then_right_of_v(&pieces);
}

#[test]
fn an_area_across_a_construction_line_is_cut_along_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 2.0));
    let below = sketch.add_point(DVec2::new(0.0, -1.0));
    let above = sketch.add_point(DVec2::new(0.0, 3.0));
    sketch.add_construction_segment(below, above);
    let region = the_only_area(&sketch);

    let pieces = sketch.pieces_across(&region, DVec2::new(0.0, -1.0), DVec2::new(0.0, 4.0));

    assert_areas(&pieces, &[8.0, 10.0]);
}

#[test]
fn an_area_across_a_segment_s_extension_is_cut_along_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 2.0));
    let low = sketch.add_point(DVec2::new(0.0, 5.0));
    let high = sketch.add_point(DVec2::new(0.0, 8.0));
    sketch.add_segment(low, high);
    let region = the_only_area(&sketch);

    let pieces = sketch.pieces_across(&region, DVec2::new(0.0, 5.0), DVec2::new(0.0, 3.0));

    assert_areas(&pieces, &[8.0, 10.0]);
}

#[test]
fn a_hole_across_the_axis_opens_into_each_side() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 6.0));
    rectangle(&mut sketch, DVec2::new(-1.0, 2.0), DVec2::new(2.0, 4.0));
    let region = sketch
        .regions()
        .into_iter()
        .find(|region| region.depth == 0)
        .unwrap();
    assert_eq!(region.holes.len(), 1);

    let pieces = sketch.pieces_across(&region, DVec2::ZERO, DVec2::Y);

    assert_areas(&pieces, &[22.0, 26.0]);
    assert!(pieces.iter().all(|piece| piece.holes.is_empty()));
}

#[test]
fn an_arc_across_the_axis_is_cut_into_two_arcs_that_keep_their_bend() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::new(-4.0, 0.0));
    let right = sketch.add_point(DVec2::new(4.0, 0.0));
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_segment(left, right);
    sketch.add_arc(centre, right, left);
    let region = the_only_area(&sketch);

    let pieces = sketch.pieces_across(&region, DVec2::ZERO, DVec2::Y);

    assert_eq!(pieces.len(), 2);
    for piece in &pieces {
        let rounds: Vec<(DVec2, f64)> = piece
            .outline
            .runs()
            .into_iter()
            .filter_map(|(_, leg)| match leg {
                Leg::Round { centre, turned } => Some((centre, turned)),
                _ => None,
            })
            .collect();
        assert_eq!(rounds.len(), 1, "one arc in each piece, found {rounds:?}");
        let (centre, turned) = rounds[0];
        assert!(
            centre.length() < 1e-12,
            "the arc keeps its centre, found {centre}"
        );
        assert!(
            (turned.abs() - std::f64::consts::FRAC_PI_2).abs() < 1e-9,
            "each arc turns a quarter of a turn, found {turned}"
        );
    }
    assert_left_then_right_of_v(&pieces);
}

fn polygon(sketch: &mut Sketch, corners: &[DVec2]) {
    let points: Vec<PointId> = corners
        .iter()
        .map(|corner| sketch.add_point(*corner))
        .collect();
    for index in 0..points.len() {
        sketch.add_segment(points[index], points[(index + 1) % points.len()]);
    }
}

#[test]
fn an_area_with_a_side_along_the_axis_is_cut_only_where_it_crosses() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    polygon(
        &mut sketch,
        &[
            DVec2::new(0.0, 0.0),
            DVec2::new(4.0, 0.0),
            DVec2::new(4.0, 4.0),
            DVec2::new(-3.0, 4.0),
            DVec2::new(-3.0, 2.0),
            DVec2::new(0.0, 2.0),
        ],
    );
    let region = the_only_area(&sketch);

    let pieces = sketch.pieces_across(&region, DVec2::ZERO, DVec2::Y);

    assert_areas(&pieces, &[6.0, 16.0]);
}

#[test]
fn an_area_wholly_on_one_side_of_the_axis_is_one_piece() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::new(0.0, 0.0), DVec2::new(5.0, 2.0));
    let region = the_only_area(&sketch);

    let pieces = sketch.pieces_across(&region, DVec2::ZERO, DVec2::Y);

    assert_areas(&pieces, &[10.0]);
}

#[test]
fn the_pieces_left_of_the_axis_come_first() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    polygon(
        &mut sketch,
        &[
            DVec2::new(-3.0, -2.0),
            DVec2::new(3.0, -2.0),
            DVec2::new(3.0, 4.0),
            DVec2::new(1.0, 4.0),
            DVec2::new(1.0, -1.0),
            DVec2::new(-1.0, -1.0),
            DVec2::new(-1.0, 4.0),
            DVec2::new(-3.0, 4.0),
        ],
    );
    let region = the_only_area(&sketch);

    let forwards = sketch.pieces_across(&region, DVec2::ZERO, DVec2::X);
    let backwards = sketch.pieces_across(&region, DVec2::ZERO, -DVec2::X);

    assert_areas(&forwards, &[8.0, 8.0, 10.0]);
    assert_areas(&backwards, &[10.0, 8.0, 8.0]);
}

#[test]
fn cutting_leaves_the_drawing_as_it_was() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 2.0));
    let region = the_only_area(&sketch);
    let (points, segments) = (sketch.points().to_vec(), sketch.segments().len());

    sketch.pieces_across(&region, DVec2::ZERO, DVec2::Y);

    assert_eq!(sketch.points(), points.as_slice());
    assert_eq!(sketch.segments().len(), segments);
    assert_areas(&sketch.regions(), &[18.0]);
}

#[test]
fn what_the_line_cuts_outside_the_area_is_left_out() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 6.0));
    rectangle(&mut sketch, DVec2::new(-2.0, 1.0), DVec2::new(2.0, 5.0));
    rectangle(&mut sketch, DVec2::new(-1.0, 2.0), DVec2::new(1.0, 3.0));
    rectangle(&mut sketch, DVec2::new(-2.0, 10.0), DVec2::new(2.0, 12.0));
    let region = sketch
        .regions()
        .into_iter()
        .find(|region| region.depth == 0 && region.contains(DVec2::new(-3.0, 3.0)))
        .unwrap();

    let pieces = sketch.pieces_across(&region, DVec2::ZERO, DVec2::Y);

    assert_areas(&pieces, &[16.0, 22.0]);
}

#[test]
fn a_hole_on_one_side_of_the_axis_stays_a_hole_of_that_side() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 6.0));
    rectangle(&mut sketch, DVec2::new(1.0, 2.0), DVec2::new(3.0, 4.0));
    let region = sketch
        .regions()
        .into_iter()
        .find(|region| region.depth == 0)
        .unwrap();

    let pieces = sketch.pieces_across(&region, DVec2::ZERO, DVec2::Y);

    assert_areas(&pieces, &[24.0, 26.0]);
    assert!(pieces[0].holes.is_empty());
    assert_eq!(pieces[1].holes.len(), 1);
}
