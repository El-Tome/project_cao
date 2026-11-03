#[cfg(test)]
use cao_sketch::SegmentId;

use super::*;

fn parallel() -> Constraint {
    Constraint::Parallel {
        first: SegmentId(0),
        second: SegmentId(1),
    }
}

fn equal() -> Constraint {
    Constraint::Equal {
        first: SegmentId(0),
        second: SegmentId(1),
    }
}

fn marks_at(on_the_plane: Vec<(Constraint, DVec2)>) -> Marks {
    Marks {
        on_screen: Vec::new(),
        on_the_plane,
    }
}

#[test]
fn a_box_takes_each_rule_with_a_mark_inside_it_once() {
    let marks = marks_at(vec![
        (parallel(), DVec2::new(10.0, 10.0)),
        (parallel(), DVec2::new(12.0, 14.0)),
        (equal(), DVec2::new(40.0, 10.0)),
    ]);

    assert_eq!(
        marks.inside(DVec2::new(20.0, 0.0), DVec2::new(0.0, 20.0)),
        vec![Selection::Rule(parallel())],
        "both marks of the parallel are in the box, the equality's is not"
    );
}

#[test]
fn a_box_with_no_mark_inside_takes_no_rule() {
    let marks = marks_at(vec![(parallel(), DVec2::new(10.0, 10.0))]);

    assert!(
        marks
            .inside(DVec2::new(20.0, 20.0), DVec2::new(30.0, 30.0))
            .is_empty()
    );
}
