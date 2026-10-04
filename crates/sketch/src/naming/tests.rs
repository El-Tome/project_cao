//! What naming an area by the curves that bound it is held to.
//!
//! Closes #345.
//! - an area whose drawing is dragged raises the same matter as before —
//!   `a_name_holds_though_the_drawing_is_pulled_about`
//! - an area one of whose bounding traits is rounded raises the same matter as
//!   before — `an_area_is_found_though_rounding_a_corner_gave_it_a_curve`
//! - an area one of whose bounding traits is erased raises nothing —
//!   `a_name_no_area_answers_to_is_lost`
//! - the two halves of a circle a chord cuts are told apart, though the same
//!   two curves bound them both —
//!   `the_two_halves_of_a_cut_circle_are_told_apart`
//! - the place clicked is read before the closest fit, so a name that answers
//!   exactly elsewhere does not take the matter away from where the user
//!   pointed — `an_area_holding_the_place_clicked_wins_over_a_closer_fit`
//! - a border divided is one border still, held by either piece —
//!   `a_border_divided_is_held_by_either_of_its_pieces`
//!
//! Closes #493.
//! - an area whose border runs along another curve keeps its name when either
//!   of the two is erased — `a_stretch_two_sides_lie_along_answers_to_either_side`;
//!   the stretch is one border both curves name —
//!   `a_stretch_two_sides_lie_along_is_one_border_named_by_both`,
//!   `a_circle_drawn_twice_is_one_border_named_by_both`

use super::*;
use crate::plane::WorkPlane;
use crate::sketch::{Element, PointId, Sketch};

fn rectangle(sketch: &mut Sketch, min: DVec2, max: DVec2) -> Vec<SegmentId> {
    let corners = [
        sketch.add_point(min),
        sketch.add_point(DVec2::new(max.x, min.y)),
        sketch.add_point(max),
        sketch.add_point(DVec2::new(min.x, max.y)),
    ];
    (0..4)
        .map(|index| sketch.add_segment(corners[index], corners[(index + 1) % 4]))
        .collect()
}

#[test]
fn an_area_is_named_by_every_trait_that_borders_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));

    let regions = sketch.regions();

    let mut named: Vec<Vec<CurveId>> = drawn
        .into_iter()
        .map(|id| vec![CurveId::Segment(id)])
        .collect();
    named.sort_unstable();
    assert_eq!(regions[0].bounds(), named);
}

#[test]
fn a_circle_nothing_cuts_is_named_by_itself() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let circle = sketch.add_circle(centre, 5.0);

    let regions = sketch.regions();

    assert_eq!(regions[0].bounds(), [vec![CurveId::Circle(circle)]]);
}

#[test]
fn a_name_holds_though_the_drawing_is_pulled_about() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
    let regions = sketch.regions();
    let area = Area::of(&regions[0], DVec2::new(5.0, 2.0));

    sketch.move_point(PointId(2), DVec2::new(40.0, 30.0));
    sketch.move_point(PointId(3), DVec2::new(0.0, 30.0));
    let pulled = sketch.regions();

    assert_eq!(
        area.found_in(&pulled),
        Some(0),
        "the place first clicked is nowhere near the shape now, and the \
         curves bounding it never changed",
    );
    assert!(
        !pulled[0].contains(area.inside),
        "a drawing the click still falls inside would prove nothing",
    );
}

#[test]
fn a_name_no_area_answers_to_is_lost() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
    rectangle(&mut sketch, DVec2::new(20.0, 0.0), DVec2::new(30.0, 4.0));
    let regions = sketch.regions();
    let area = Area::of(&regions[0], DVec2::new(5.0, 2.0));

    sketch.erase(Element::Segment(drawn[0]));
    let left = sketch.regions();

    assert_eq!(left.len(), 1, "the second rectangle is still drawn");
    assert_eq!(
        area.found_in(&left),
        None,
        "an area that is gone answers to nothing, rather than to whatever \
         area is left",
    );
}

#[test]
fn an_area_is_found_though_rounding_a_corner_gave_it_a_curve() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
    let inside = DVec2::new(5.0, 2.0);

    let rounded = sketch.fillet(drawn[0], drawn[1], 1.0).expect("a corner");
    let regions = sketch.regions();

    // What the descent will hand over: the two sides as they are left, and
    // the two traits the rounding never touched. The curve now standing where
    // the corner was descends from neither side, and is not in the name.
    let mut carried: Vec<CurveId> = rounded.pieces.into_iter().map(CurveId::Segment).collect();
    carried.extend([CurveId::Segment(drawn[2]), CurveId::Segment(drawn[3])]);
    carried.sort_unstable();
    let area = Area {
        bounds: carried.into_iter().map(|curve| vec![curve]).collect(),
        inside,
    };

    assert!(
        regions[0].runs_along(CurveId::Arc(rounded.arc)),
        "the rounding really did put a curve in the boundary",
    );
    assert_eq!(area.found_in(&regions), Some(0));
}

#[test]
fn the_two_halves_of_a_cut_circle_are_told_apart() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, 5.0);
    let left = sketch.add_point(DVec2::new(-5.0, 0.0));
    let right = sketch.add_point(DVec2::new(5.0, 0.0));
    sketch.add_segment(left, right);

    let regions = sketch.regions();
    let above = area_under(&regions, DVec2::new(0.0, 2.0)).expect("the upper half");
    let below = area_under(&regions, DVec2::new(0.0, -2.0)).expect("the lower half");

    assert_eq!(
        regions[above].bounds(),
        regions[below].bounds(),
        "the chord and the circle bound them both, which is why the place \
         clicked has to settle it",
    );
    assert_eq!(
        Area::of(&regions[above], DVec2::new(0.0, 2.0)).found_in(&regions),
        Some(above),
    );
    assert_eq!(
        Area::of(&regions[below], DVec2::new(0.0, -2.0)).found_in(&regions),
        Some(below),
    );
}

#[test]
fn a_name_holding_nothing_answers_to_no_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));

    let nameless = Area {
        bounds: Vec::new(),
        inside: DVec2::new(5.0, 2.0),
    };

    assert_eq!(
        nameless.found_in(&sketch.regions()),
        None,
        "every area is bounded by all of nothing, so an empty name would \
         otherwise take the first one it met",
    );
}

#[test]
fn an_area_holding_the_place_clicked_wins_over_a_closer_fit() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    sketch.add_circle(centre, 5.0);
    let chord = |sketch: &mut Sketch, height: f64| {
        let reach = (25.0 - height * height).sqrt();
        let left = sketch.add_point(DVec2::new(-reach, height));
        let right = sketch.add_point(DVec2::new(reach, height));
        sketch.add_segment(left, right);
    };
    chord(&mut sketch, 0.0);
    let clicked = DVec2::new(0.0, 1.0);
    let area = Area::of(
        &sketch.regions()[area_under(&sketch.regions(), clicked).expect("a half")],
        clicked,
    );

    chord(&mut sketch, 2.0);
    let regions = sketch.regions();

    let found = area.found_in(&regions).expect("an area");
    assert!(
        regions[found].contains(clicked),
        "the far half of the circle answers to the name exactly, while the \
         half that was clicked answers with the new chord to spare — and the \
         matter belongs where the user pointed",
    );
}

#[test]
fn a_border_divided_is_held_by_either_of_its_pieces() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));

    // As the descent hands it over once the bottom has been cut in two: one
    // border, holding both pieces. Only one of them borders the area.
    let mut borders: Vec<Vec<CurveId>> = vec![vec![
        CurveId::Segment(drawn[0]),
        CurveId::Segment(SegmentId(99)),
    ]];
    borders.extend(drawn[1..].iter().map(|id| vec![CurveId::Segment(*id)]));
    let standing = Standing::new(borders, DVec2::new(5.0, 2.0));

    assert_eq!(
        standing.found_in(&sketch.regions()),
        Some(0),
        "demanding both pieces would lose an area that kept one of them, \
         which is what dividing a trait two areas share does",
    );
}

fn sides(ids: &[SegmentId]) -> Vec<CurveId> {
    ids.iter().copied().map(CurveId::Segment).collect()
}

fn borders(of: &[Vec<CurveId>]) -> Vec<Vec<CurveId>> {
    let mut borders: Vec<Vec<CurveId>> = of
        .iter()
        .map(|border| {
            let mut curves = border.clone();
            curves.sort_unstable();
            curves
        })
        .collect();
    borders.sort_unstable();
    borders
}

#[test]
fn a_stretch_two_sides_lie_along_is_one_border_named_by_both() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first = sides(&rectangle(&mut sketch, DVec2::ZERO, DVec2::new(1.0, 2.0)));
    let second = sides(&rectangle(
        &mut sketch,
        DVec2::new(1.0, 1.0),
        DVec2::new(2.0, 3.0),
    ));

    let regions = sketch.regions();
    let named = |place: DVec2| {
        let rank = area_under(&regions, place).expect("an area under the place");
        regions[rank].bounds().to_vec()
    };
    let shared = vec![first[1], second[3]];

    assert_eq!(
        named(DVec2::new(0.5, 1.0)),
        borders(&[
            vec![first[0]],
            vec![first[1]],
            vec![first[2]],
            vec![first[3]],
            shared.clone(),
        ]),
    );
    assert_eq!(
        named(DVec2::new(1.5, 2.0)),
        borders(&[
            vec![second[0]],
            vec![second[1]],
            vec![second[2]],
            vec![second[3]],
            shared,
        ]),
    );
}

/// A short rectangle against a long one, its left side lying wholly along the
/// long one's right side, and its name as it is clicked.
fn a_short_side_along_a_long_one() -> (Sketch, Vec<SegmentId>, Vec<SegmentId>, Area) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let long = rectangle(&mut sketch, DVec2::ZERO, DVec2::new(1.0, 3.0));
    let short = rectangle(&mut sketch, DVec2::new(1.0, 1.0), DVec2::new(2.0, 2.0));
    let clicked = DVec2::new(1.5, 1.5);
    let regions = sketch.regions();
    let area = Area::of(
        &regions[area_under(&regions, clicked).expect("the short rectangle")],
        clicked,
    );
    (sketch, long, short, area)
}

#[test]
fn a_stretch_two_sides_lie_along_answers_to_either_side() {
    let (mut sketch, long, short, area) = a_short_side_along_a_long_one();
    assert!(
        area.bounds
            .contains(&vec![CurveId::Segment(long[1]), CurveId::Segment(short[3])]),
        "the short side lies wholly along the long one, and runs along no \
         stretch of its own: {:?}",
        area.bounds,
    );
    for side in &long {
        sketch.erase(Element::Segment(*side));
    }
    let left = sketch.regions();
    assert_eq!(left.len(), 1, "the short rectangle is all that is left");
    assert_eq!(
        area.found_in(&left),
        Some(0),
        "its own side bounds it now where the long one did",
    );

    let (mut sketch, _, short, area) = a_short_side_along_a_long_one();
    sketch.erase(Element::Segment(short[3]));
    let left = sketch.regions();
    let rank = area.found_in(&left).expect("the long side still closes it");
    assert!(
        left[rank].contains(DVec2::new(1.5, 1.5)) && !left[rank].contains(DVec2::new(0.5, 1.5)),
        "the short rectangle, closed by the long one's side, and not the long one",
    );
}

#[test]
fn a_circle_drawn_twice_is_one_border_named_by_both() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let circles: Vec<CurveId> = (0..2)
        .map(|_| {
            let centre = sketch.add_point(DVec2::new(8.0, 4.0));
            CurveId::Circle(sketch.add_circle(centre, 2.0))
        })
        .collect();

    let regions = sketch.regions();

    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].bounds(), [circles]);
}
