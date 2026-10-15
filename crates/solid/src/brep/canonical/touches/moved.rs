//! The surface a move onto a touch makes: `moving` set on the line it was
//! decided to touch `fixed` along, and a cylinder slid along its partner's
//! axis to keep a node.

use crate::brep::surface::{Cylinder, Plane, Surface};

/// A cylinder given its partner's radius at a node of two of one radius,
/// slid either way along the partner's axis by what its radius grew: the
/// node stands wherever along that axis the two meet, and the slide keeps
/// the cylinder touching a plane square to the axis as it touched it — a
/// post resting on a shaft's end, whose wall crosses the shaft at a node
/// but for a hair of radius (533609727). None for any other move.
pub(super) fn along_the_partner(
    before: &Surface,
    after: &Surface,
    partner: &Surface,
) -> Vec<Surface> {
    let (before, after, partner) = match (before, after, partner) {
        (Surface::Cylinder(before), Surface::Cylinder(after), Surface::Cylinder(partner)) => {
            (before, after, partner)
        }
        (Surface::Plane(_) | Surface::Cone(_) | Surface::Elliptic(_), _, _)
        | (_, Surface::Plane(_) | Surface::Cone(_) | Surface::Elliptic(_), _)
        | (_, _, Surface::Plane(_) | Surface::Cone(_) | Surface::Elliptic(_)) => {
            return Vec::new();
        }
    };
    let grown = after.radius - before.radius;
    if grown == 0.0 {
        return Vec::new();
    }
    [grown, -grown]
        .map(|by| {
            Surface::Cylinder(Cylinder::about(
                after.origin + partner.axis * by,
                after.axis,
                after.radius,
            ))
        })
        .to_vec()
}

/// `moving` moved onto the line it was decided to touch `fixed` along: a
/// cylinder or an elliptic wall along a plane's normal, a plane along its
/// own, a cylinder towards or away from a parallel one, by the gap the
/// relation found within the tolerance. None where the touch is exact
/// already.
pub(super) fn touching(fixed: &Surface, moving: &Surface) -> Option<Surface> {
    let moved = match (fixed, moving) {
        (Surface::Plane(plane), Surface::Elliptic(wall)) => {
            let away = plane.distance(wall.origin);
            let gap = away.abs() - wall.support(plane.normal);
            Surface::Elliptic(wall.moved(-plane.normal * away.signum() * gap))
        }
        (Surface::Elliptic(wall), Surface::Plane(plane)) => {
            let away = plane.distance(wall.origin);
            let gap = away.abs() - wall.support(plane.normal);
            let (plane, _) = Plane::through(
                plane.origin + plane.normal * away.signum() * gap,
                plane.normal,
            );
            Surface::Plane(plane)
        }
        (Surface::Plane(plane), Surface::Cylinder(cylinder)) => {
            let away = plane.distance(cylinder.origin);
            let gap = away.abs() - cylinder.radius;
            Surface::Cylinder(Cylinder::about(
                cylinder.origin - plane.normal * away.signum() * gap,
                cylinder.axis,
                cylinder.radius,
            ))
        }
        (Surface::Cylinder(cylinder), Surface::Plane(plane)) => {
            let away = plane.distance(cylinder.origin);
            let gap = away.abs() - cylinder.radius;
            let (plane, _) = Plane::through(
                plane.origin + plane.normal * away.signum() * gap,
                plane.normal,
            );
            Surface::Plane(plane)
        }
        (Surface::Cylinder(kept), Surface::Cylinder(cylinder)) => {
            let between = cylinder.origin - kept.origin;
            let across = between - kept.axis * kept.axis.dot(between);
            let distance = across.length();
            if distance == 0.0 {
                return None;
            }
            let outside = distance - (kept.radius + cylinder.radius);
            let inside = (kept.radius - cylinder.radius).abs() - distance;
            let by = if outside.abs() <= inside.abs() {
                -outside
            } else {
                inside
            };
            Surface::Cylinder(Cylinder::about(
                cylinder.origin + across / distance * by,
                cylinder.axis,
                cylinder.radius,
            ))
        }
        (Surface::Plane(_), Surface::Plane(_))
        | (Surface::Cone(_) | Surface::Elliptic(_), _)
        | (_, Surface::Cone(_) | Surface::Elliptic(_)) => {
            return None;
        }
    };
    (moved != *moving).then_some(moved)
}
