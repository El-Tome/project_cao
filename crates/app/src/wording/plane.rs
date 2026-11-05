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

/// The plane a sketch was laid on: off the origin and held by no face of the
/// part, it is a cutting plane.
pub fn of_sketch(lang: &Catalogue, kind: PlaneKind, on_a_face: bool) -> String {
    match (kind, on_a_face) {
        (PlaneKind::OffOrigin, false) => lang.t("plane.cutting"),
        _ => label(lang, kind),
    }
}

#[cfg(test)]
mod tests;
