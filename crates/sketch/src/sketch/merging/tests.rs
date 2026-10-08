use glam::DVec2;

use crate::constraints::DimensionTarget;
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
