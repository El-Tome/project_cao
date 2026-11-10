use cao_sketch::PlaneKind;

use crate::lang::Catalogue;

/// The only place a work plane is turned into a name.
///
/// `cao_sketch` decides which plane it is; this decides how that reads. What
/// the geometry can only call "not through the origin" reads as a face of the
/// part: a sketch laid on a section's cut lands there too, and nothing in the
/// plane tells the two apart.
pub fn label(lang: &Catalogue, kind: PlaneKind) -> String {
    lang.t(match kind {
        PlaneKind::OffOrigin => "plane.off_origin",
        PlaneKind::OriginXY => "plane.origin_xy",
        PlaneKind::OriginXZ => "plane.origin_xz",
        PlaneKind::OriginYZ => "plane.origin_yz",
        PlaneKind::OriginSlanted => "plane.origin_slanted",
    })
}

/// The same, in the few letters a row of the Part tree has room for.
pub fn short(lang: &Catalogue, kind: PlaneKind) -> String {
    lang.t(match kind {
        PlaneKind::OffOrigin => "plane.short.off_origin",
        PlaneKind::OriginXY => "plane.short.origin_xy",
        PlaneKind::OriginXZ => "plane.short.origin_xz",
        PlaneKind::OriginYZ => "plane.short.origin_yz",
        PlaneKind::OriginSlanted => "plane.short.origin_slanted",
    })
}

#[cfg(test)]
mod tests;
