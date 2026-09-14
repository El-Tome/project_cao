//! What the trim tool makes of an arc: the stretch clicked goes, and the two
//! ends of the curve stay as arcs around the same centre.

use cao_sketch::{ArcId, Constraint, DimensionTarget, PointId, Sketch, WorkPlane};
use glam::DVec2;

const REACH: f64 = 10.0;
const TOLERANCE: f64 = 1e-9;

fn on_the_rim(sketch: &mut Sketch, degrees: f64) -> PointId {
    sketch.add_point(DVec2::from_angle(degrees.to_radians()) * REACH)
}

/// A half turn from due east round to due west, with two points sitting on it
/// a third and two thirds of the way along.
fn a_half_turn() -> (Sketch, ArcId, [PointId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let east = on_the_rim(&mut sketch, 0.0);
    let west = on_the_rim(&mut sketch, 180.0);
    let arc = sketch.add_arc(Sketch::ORIGIN, east, west);
    let first = on_the_rim(&mut sketch, 60.0);
    let second = on_the_rim(&mut sketch, 120.0);
    (sketch, arc, [first, second])
}

#[test]
fn a_cut_between_two_points_on_an_arc_leaves_the_two_ends_of_it() {
    let (mut sketch, arc, [first, second]) = a_half_turn();

    let trimmed = sketch
        .trim_arc(arc, first, second)
        .expect("a cut that can be made");

    assert!(sketch.is_erased_arc(arc), "the arc cut into pieces is gone");
    assert_eq!(trimmed.pieces.len(), 2, "both ends of the curve stayed");
    for piece in trimmed.pieces {
        let sweep = sketch.arc_sweep(piece).to_degrees();
        assert!(
            (sweep - 60.0).abs() < TOLERANCE,
            "a third of the half turn expected, got {sweep}",
        );
        let reach = sketch.arc_radius(piece);
        assert!(
            (reach - REACH).abs() < TOLERANCE,
            "a cut takes nothing off the reach, got {reach}",
        );
    }
}

#[test]
fn a_click_on_an_arc_nothing_sits_on_takes_the_whole_curve_away() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let east = on_the_rim(&mut sketch, 0.0);
    let west = on_the_rim(&mut sketch, 180.0);
    let arc = sketch.add_arc(Sketch::ORIGIN, east, west);

    let (from, to) = sketch
        .arc_stretch_at(arc, DVec2::new(0.0, REACH))
        .expect("a stretch under the click");
    let trimmed = sketch
        .trim_arc(arc, from, to)
        .expect("a cut that can be made");

    assert_eq!(trimmed.pieces, Vec::new(), "nothing of the curve is left");
    assert!(sketch.is_erased_arc(arc));
}

#[test]
fn a_click_on_an_arc_takes_out_the_stretch_it_fell_in_and_no_other() {
    let (sketch, arc, [first, second]) = a_half_turn();

    let clicked = sketch
        .arc_stretch_at(arc, DVec2::from_angle(30f64.to_radians()) * REACH)
        .expect("a stretch under the click");

    assert_eq!(clicked, (sketch.arc(arc).start, first));
    let further = sketch
        .arc_stretch_at(arc, DVec2::from_angle(150f64.to_radians()) * REACH)
        .expect("a stretch under the click");
    assert_eq!(further, (second, sketch.arc(arc).end));
}

#[test]
fn a_reach_typed_on_an_arc_is_read_again_on_both_pieces_and_a_sweep_on_neither() {
    let (mut sketch, arc, [first, second]) = a_half_turn();
    sketch.set_dimension(DimensionTarget::ArcRadius(arc), 10.0, false);
    sketch.set_dimension(DimensionTarget::ArcSweep(arc), 180.0, false);

    let trimmed = sketch
        .trim_arc(arc, first, second)
        .expect("a cut that can be made");

    for piece in &trimmed.pieces {
        assert!(
            sketch
                .dimension_of(DimensionTarget::ArcRadius(*piece))
                .is_some(),
            "a cut takes nothing off the reach",
        );
    }
    assert_eq!(
        trimmed.values_dropped, 1,
        "the sweep went, the reach followed both pieces",
    );
}

#[test]
fn two_arcs_held_the_same_reach_stay_so_on_every_piece_a_cut_leaves() {
    let (mut sketch, arc, [first, second]) = a_half_turn();
    let north = sketch.add_point(DVec2::new(40.0, 10.0));
    let south = sketch.add_point(DVec2::new(40.0, -10.0));
    let centre = sketch.add_point(DVec2::new(40.0, 0.0));
    let other = sketch.add_arc(centre, south, north);
    sketch.add_constraint(Constraint::EqualRadiusArc {
        first: arc,
        second: other,
    });

    let trimmed = sketch
        .trim_arc(arc, first, second)
        .expect("a cut that can be made");

    for piece in &trimmed.pieces {
        assert!(
            sketch.constraints().contains(
                &Constraint::EqualRadiusArc {
                    first: *piece,
                    second: other,
                }
                .normalised()
            ),
            "the piece lost the reach it shared",
        );
    }
    assert_eq!(trimmed.rules_dropped, 0, "nothing was lost to the cut");
}

#[test]
fn a_line_brushing_an_arc_goes_on_brushing_the_piece_it_touches() {
    let (mut sketch, arc, [first, _]) = a_half_turn();
    let left = sketch.add_point(DVec2::new(-20.0, REACH));
    let right = sketch.add_point(DVec2::new(20.0, REACH));
    let grazing = sketch.add_segment(left, right);
    sketch.add_constraint(Constraint::ArcTangent {
        arc,
        segment: grazing,
        at: None,
    });

    let trimmed = sketch
        .trim_arc(arc, sketch.arc(arc).start, first)
        .expect("a cut that can be made");

    let far = *trimmed.pieces.last().expect("the far end of the curve");
    assert!(
        sketch.constraints().contains(&Constraint::ArcTangent {
            arc: far,
            segment: grazing,
            at: None,
        }),
        "the line touches the curve at the top, which the far piece still holds",
    );
    assert_eq!(trimmed.rules_dropped, 0, "nothing was lost to the cut");
}

#[test]
fn a_cut_counts_the_brush_it_took_away_with_the_stretch_it_touched() {
    let (mut sketch, arc, [first, second]) = a_half_turn();
    let left = sketch.add_point(DVec2::new(-20.0, REACH));
    let right = sketch.add_point(DVec2::new(20.0, REACH));
    let grazing = sketch.add_segment(left, right);
    sketch.add_constraint(Constraint::ArcTangent {
        arc,
        segment: grazing,
        at: None,
    });

    let trimmed = sketch
        .trim_arc(arc, first, second)
        .expect("a cut that can be made");

    assert_eq!(
        trimmed.rules_dropped, 1,
        "the line touched the top of the curve, which is the stretch that went",
    );
}

#[test]
fn naming_the_same_point_twice_cuts_an_arc_in_two_and_takes_nothing_away() {
    let (mut sketch, arc, [first, _]) = a_half_turn();

    let trimmed = sketch
        .trim_arc(arc, first, first)
        .expect("a cut that can be made");

    assert_eq!(trimmed.pieces.len(), 2, "both halves of the curve stayed");
    let swept: f64 = trimmed
        .pieces
        .iter()
        .map(|piece| sketch.arc_sweep(*piece).to_degrees())
        .sum();
    assert!(
        (swept - 180.0).abs() < TOLERANCE,
        "the two pieces run the whole half turn, got {swept}",
    );
}

#[test]
fn a_cut_that_would_hand_the_whole_arc_back_is_no_cut_at_all() {
    let (mut sketch, arc, _) = a_half_turn();
    let end = sketch.arc(arc).end;

    assert_eq!(sketch.trim_arc(arc, end, end), None);
}

#[test]
fn cutting_a_guide_arc_leaves_guides() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let east = on_the_rim(&mut sketch, 0.0);
    let west = on_the_rim(&mut sketch, 180.0);
    let guide = sketch.add_construction_arc(Sketch::ORIGIN, east, west);
    let first = on_the_rim(&mut sketch, 60.0);
    let second = on_the_rim(&mut sketch, 120.0);

    let trimmed = sketch
        .trim_arc(guide, first, second)
        .expect("a cut that can be made");

    for piece in trimmed.pieces {
        assert!(sketch.arc(piece).construction, "a guide became a drawn arc");
    }
}
