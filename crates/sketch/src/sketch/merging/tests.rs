use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget};
use crate::plane::WorkPlane;
use crate::sketch::Sketch;

#[test]
fn an_arc_end_made_one_with_its_centre_leaves_no_value_behind_whichever_end() {
    for start_goes in [true, false] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::new(20.0, 0.0));
        let start = sketch.add_point(DVec2::new(20.0, -5.0));
        let end = sketch.add_point(DVec2::new(25.0, 0.0));
        let arc = sketch.add_arc(centre, start, end);
        sketch.set_dimension(DimensionTarget::ArcRadius(arc), 5.0, true);

        sketch.merge_points(centre, if start_goes { start } else { end });

        assert!(
            sketch.is_erased_arc(arc),
            "an arc of no radius is still drawn"
        );
        assert!(
            sketch.dimensions().is_empty(),
            "the start going: {start_goes}, and a value was left behind: {:?}",
            sketch.dimensions()
        );
    }
}

#[test]
fn an_arc_of_ellipse_whose_two_ends_become_one_is_taken_away() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(0.0, 50.0));
    let [west, east] = [-30.0, 30.0].map(|x| sketch.add_point(DVec2::new(x, 50.0)));
    let [south, north] = [30.0, 70.0].map(|y| sketch.add_point(DVec2::new(0.0, y)));
    let oval = sketch.add_ellipse(centre, [west, east], [south, north]);
    let from = sketch.add_point(DVec2::new(21.213, 64.142));
    let to = sketch.add_point(DVec2::new(-21.213, 64.142));
    for point in [from, to] {
        sketch.add_constraint(Constraint::OnEllipse {
            point,
            ellipse: oval,
        });
    }
    sketch.draw_the_stretch(oval, to, from);

    sketch.merge_points(from, to);

    assert!(
        sketch.is_erased_ellipse(oval),
        "a stretch of no length is still drawn"
    );
}
