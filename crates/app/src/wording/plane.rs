use cao_sketch::PlaneKind;

use crate::lang::Catalogue;

/// The only place a work plane is turned into a name.
///
/// `cao_sketch` decides which plane it is; this decides how that reads. What
/// the geometry can only call "not through the origin" is a face of the part
/// as long as that is the only way to get one.
pub fn label(lang: &Catalogue, kind: PlaneKind) -> String {
    lang.t(match kind {
        PlaneKind::OffOrigin => "plane.off_origin",
        PlaneKind::OriginXY => "plane.origin_xy",
        PlaneKind::OriginXZ => "plane.origin_xz",
        PlaneKind::OriginYZ => "plane.origin_yz",
        PlaneKind::OriginSlanted => "plane.origin_slanted",
    })
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

    fn said(kind: PlaneKind) -> String {
        label(&Catalogue::french(), kind)
    }

    #[test]
    fn each_plane_the_sketch_recognises_reads_differently() {
        let read: BTreeSet<String> = EVERY_KIND.iter().map(|kind| said(*kind)).collect();

        assert_eq!(
            read.len(),
            EVERY_KIND.len(),
            "two planes read the same: {read:?}"
        );
    }

    #[test]
    fn a_plane_through_the_origin_is_named_after_its_axes_and_a_face_is_not() {
        assert_eq!(said(PlaneKind::OriginXY), "Plan XY");
        assert_eq!(said(PlaneKind::OriginXZ), "Plan XZ");
        assert_eq!(said(PlaneKind::OriginYZ), "Plan YZ");
        assert_eq!(said(PlaneKind::OriginSlanted), "Plan d'esquisse");
        assert_eq!(said(PlaneKind::OffOrigin), "Face de la pièce");
    }
}
