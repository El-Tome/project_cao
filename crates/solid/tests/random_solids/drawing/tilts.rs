//! Tools drawn on planes leaning off the planes of the origin (#552): what a
//! plane meeting a cylinder or a cone at a slant cuts along a conic.

use glam::DVec3;

use super::super::Plane;

impl Plane {
    /// A plane through `origin` holding the world axis `holds`, leaning by
    /// `degrees` off the plane of the origin it starts from: XY about X or
    /// Y, XZ about Z.
    pub fn leaning(holds: usize, degrees: f64, origin: DVec3) -> Plane {
        let turn = match holds {
            0 => DVec3::new(degrees, 0.0, 0.0),
            1 => DVec3::new(0.0, degrees, 0.0),
            _ => DVec3::new(90.0, degrees, 0.0),
        };
        Plane::Tilted { origin, turn }
    }
}
