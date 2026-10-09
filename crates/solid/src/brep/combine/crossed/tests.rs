use glam::{DVec2, DVec3};

use super::*;
use crate::brep::surface::{Cone, Cylinder, Plane};

fn rim() -> [Surface; 2] {
    let (floor, _) = Plane::through(DVec3::ZERO, DVec3::Z);
    let wall = Cylinder::about(DVec3::ZERO, DVec3::Z, 5.0);
    [Surface::Plane(floor), Surface::Cylinder(wall)]
}

#[test]
fn a_crossing_left_unsolved_on_a_pair_the_kernel_builds_declines_rather_than_being_skipped() {
    let scale = Scale::of(20.0);
    let support = rim();
    let support: Vec<(&Surface, Scale)> = support.iter().map(|own| (own, scale)).collect();
    let (side, _) = Plane::through(DVec3::new(1.0, 0.0, 0.0), DVec3::X);
    assert!(
        !left_to_a_declined_pair(&support, &Surface::Plane(side)),
        "a plane both surfaces of the rim are decided against owes the crossing"
    );
    let point = Cone::through(
        DVec3::new(0.0, 0.0, 2.0),
        DVec3::X,
        [DVec2::new(0.0, 3.0), DVec2::new(6.0, 0.0)],
    );
    assert!(
        left_to_a_declined_pair(&support, &Surface::Cone(point)),
        "a cone off the rim's axis is a pair the kernel declines unless clear"
    );
}
