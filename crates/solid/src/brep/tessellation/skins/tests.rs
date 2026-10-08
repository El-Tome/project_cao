use super::cancelled;
use crate::brep::topology::FaceId;

#[test]
fn two_faces_drawn_on_one_triangle_facing_opposite_ways_are_left_out() {
    let mut oriented = vec![
        (FaceId(0), [1, 2, 3]),
        (FaceId(1), [2, 1, 3]),
        (FaceId(1), [3, 4, 5]),
    ];
    cancelled(&mut oriented);
    assert_eq!(oriented, vec![(FaceId(1), [3, 4, 5])]);
}

#[test]
fn two_faces_drawn_on_one_triangle_facing_the_same_way_are_kept() {
    let mut oriented = vec![(FaceId(0), [1, 2, 3]), (FaceId(1), [2, 3, 1])];
    cancelled(&mut oriented);
    assert_eq!(oriented.len(), 2);
}

#[test]
fn one_face_drawn_twice_on_one_triangle_is_kept() {
    let mut oriented = vec![(FaceId(0), [1, 2, 3]), (FaceId(0), [3, 2, 1])];
    cancelled(&mut oriented);
    assert_eq!(oriented.len(), 2);
}
