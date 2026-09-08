use cao_sketch::{PlaneKind, WorkPlane};

/// The only place a work plane is turned into a name.
///
/// `cao_sketch` decides which plane it is; this decides how that reads.
pub fn label(plane: &WorkPlane) -> &'static str {
    match plane.kind() {
        PlaneKind::PartFace => "Face de la pièce",
        PlaneKind::OriginXY => "Plan XY",
        PlaneKind::OriginXZ => "Plan XZ",
        PlaneKind::OriginYZ => "Plan YZ",
        PlaneKind::Slanted => "Plan d'esquisse",
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use super::*;

    #[test]
    fn each_plane_the_sketch_recognises_reads_differently() {
        let named = [
            (WorkPlane::XY, "Plan XY"),
            (WorkPlane::XZ, "Plan XZ"),
            (WorkPlane::YZ, "Plan YZ"),
            (
                WorkPlane::from_normal(DVec3::ZERO, DVec3::new(1.0, 1.0, 0.0)),
                "Plan d'esquisse",
            ),
            (
                WorkPlane::from_normal(DVec3::Z * 12.0, DVec3::Z),
                "Face de la pièce",
            ),
        ];

        for (plane, reads) in named {
            assert_eq!(label(&plane), reads, "{plane:?} reads {reads:?}");
        }
    }
}
