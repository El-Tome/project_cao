//! What render · geometry.rs is held to.
//!
//! Closes #358.
//! - the drawing's own axes are drawn at the origin its plane was given —
//!   `the_axes_of_a_plane_cross_at_its_origin`

use super::*;

#[test]
fn the_axes_of_a_plane_cross_at_its_origin() {
    let origin = Vec3::new(3.0, -7.0, 12.0);
    let mut vertices = Vec::new();

    push_plane_axes(
        &mut vertices,
        origin,
        Vec3::X,
        Vec3::Y,
        100.0,
        &AxisStyle::default(),
    );

    let segments = vertices.as_chunks::<2>().0;
    assert_eq!(segments.len(), 2);
    for segment in segments {
        let [start, end] = segment.map(|vertex| Vec3::from_array(vertex.position));
        assert!(
            ((start + end) * 0.5 - origin).length() < 1e-4,
            "an axis runs from {start:?} to {end:?}, not through {origin:?}",
        );
    }
}
