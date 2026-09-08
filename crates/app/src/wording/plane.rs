use cao_sketch::PlaneKind;

/// The only place a work plane is turned into a name.
///
/// `cao_sketch` decides which plane it is; this decides how that reads. What
/// the geometry can only call "not through the origin" is a face of the part
/// as long as that is the only way to get one.
pub fn label(kind: PlaneKind) -> &'static str {
    match kind {
        PlaneKind::OffOrigin => "Face de la pièce",
        PlaneKind::OriginXY => "Plan XY",
        PlaneKind::OriginXZ => "Plan XZ",
        PlaneKind::OriginYZ => "Plan YZ",
        PlaneKind::OriginSlanted => "Plan d'esquisse",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    const EVERY_KIND: [PlaneKind; 5] = [
        PlaneKind::OffOrigin,
        PlaneKind::OriginXY,
        PlaneKind::OriginXZ,
        PlaneKind::OriginYZ,
        PlaneKind::OriginSlanted,
    ];

    #[test]
    fn each_plane_the_sketch_recognises_reads_differently() {
        let read: BTreeSet<&str> = EVERY_KIND.iter().map(|kind| label(*kind)).collect();

        assert_eq!(
            read.len(),
            EVERY_KIND.len(),
            "two planes read the same: {read:?}"
        );
    }

    #[test]
    fn a_plane_through_the_origin_is_named_after_its_axes_and_a_face_is_not() {
        assert_eq!(label(PlaneKind::OriginXY), "Plan XY");
        assert_eq!(label(PlaneKind::OriginXZ), "Plan XZ");
        assert_eq!(label(PlaneKind::OriginYZ), "Plan YZ");
        assert_eq!(label(PlaneKind::OriginSlanted), "Plan d'esquisse");
        assert_eq!(label(PlaneKind::OffOrigin), "Face de la pièce");
    }
}
