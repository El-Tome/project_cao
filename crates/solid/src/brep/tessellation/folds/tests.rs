use glam::DVec3;

use super::through;

const CLEAR: f64 = 1e-10;

fn at(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(x, y, z)
}

#[test]
fn two_triangles_threaded_through_each_other_pass_through() {
    let flat = [at(0.0, 0.0, 0.0), at(2.0, 0.0, 0.0), at(0.0, 2.0, 0.0)];
    let standing = [at(0.5, 0.5, -1.0), at(0.5, 0.5, 1.0), at(-1.0, 0.5, 0.0)];
    assert!(through(flat, standing, CLEAR));
    assert!(through(standing, flat, CLEAR));
}

#[test]
fn two_triangles_leaving_a_shared_corner_either_side_of_their_planes_line_do_not() {
    let one = [at(0.0, 0.0, 0.0), at(1.0, 0.0, -1.0), at(1.0, 0.0, 1.0)];
    let other = [at(0.0, 0.0, 0.0), at(-1.0, -1.0, 0.0), at(-1.0, 1.0, 0.0)];
    assert!(!through(one, other, CLEAR));
}

#[test]
fn a_triangle_resting_a_corner_on_another_within_the_clearance_does_not_pass_through_it() {
    let flat = [at(0.0, 0.0, 0.0), at(2.0, 0.0, 0.0), at(0.0, 2.0, 0.0)];
    let resting = [at(0.5, 0.5, -1e-12), at(0.5, 0.5, 1.0), at(1.0, 0.2, 1.0)];
    assert!(!through(flat, resting, CLEAR));
}

#[test]
fn a_chord_sinking_two_billionths_under_a_corner_beside_it_passes_through_the_face_there() {
    let ceiling = [
        at(1.75000001, 3.5, 9.50000001),
        at(1.9413417161825448, 4.461939766255643, 9.50000001),
        at(1.941341716182545, 3.5380602337443565, 9.50000001),
    ];
    let chord = [
        at(2.0, 3.5380602337443565, 9.691341716182546),
        at(1.75, 3.5, 9.5),
        at(2.0, 3.5, 9.5),
    ];
    assert!(through(chord, ceiling, CLEAR));
}
