//! A body as a binary STL, the file a slicer reads: an 80-byte header, the
//! count of triangles, then each triangle's normal and three corners as
//! little-endian `f32`, and two bytes of nothing.

use crate::Body;

/// The body could not be covered whole by triangles at the gap asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Uncovered;

/// The body's triangles standing within `gap` millimetres of its surfaces,
/// written in millimetres.
pub fn of(body: &Body, gap: f64, millimetres_per_unit: f64) -> Result<Vec<u8>, Uncovered> {
    let triangles = body
        .triangles_within(gap / millimetres_per_unit)
        .ok_or(Uncovered)?;
    let mut bytes = vec![0; 80];
    bytes.extend_from_slice(&(triangles.len() as u32).to_le_bytes());
    for triangle in triangles {
        let [a, b, c] = triangle.map(|corner| corner * millimetres_per_unit);
        let normal = (b - a).cross(c - a).normalize_or_zero();
        for vector in [normal, a, b, c] {
            for number in vector.to_array() {
                bytes.extend_from_slice(&(number as f32).to_le_bytes());
            }
        }
        bytes.extend_from_slice(&[0, 0]);
    }
    Ok(bytes)
}
