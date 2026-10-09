//! Bodies bounded by a conic, built by hand, drawn: each closed, uncrossed,
//! every triangle of a wall within the tolerance of it, and enclosing the
//! arithmetic's volume within what the chords take off the walls.

use std::f64::consts::PI;

use super::fixtures::{self, HEIGHT, STOCK_RADIUS};
use super::slanted;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::tessellation::Cut;
use crate::brep::topology::Body;
use crate::soundness::{closed, enclosed, uncrossed};

/// A thousandth of the stock's radius, the tolerance the application draws
/// at, and finer and coarser.
const TOLERANCES: [f64; 4] = [1e-4, 0.02, 0.5, 3.0];

fn held(body: &Body, tolerance: f64, exact: f64) -> Cut {
    let cut = body.triangles_by_face(tolerance);
    assert!(cut.uncut.is_empty(), "faces left open: {:?}", cut.uncut);
    closed(&cut.triangles).expect("the triangles close");
    uncrossed(&cut.triangles).expect("no triangle crosses another");
    let mut curved = 0.0;
    for (corners, face) in cut.triangles.iter().zip(&cut.faces) {
        let surface = body.surface(body.face(*face).surface);
        if let Surface::Plane(_) = surface {
            continue;
        }
        curved += (corners[1] - corners[0])
            .cross(corners[2] - corners[0])
            .length()
            / 2.0;
        for one in 0..=8 {
            for other in 0..=8 - one {
                let (a, b) = (one as f64 / 8.0, other as f64 / 8.0);
                let point = corners[0] * (1.0 - a - b) + corners[1] * a + corners[2] * b;
                let off = surface.distance(point).abs();
                assert!(
                    off <= tolerance * (1.0 + 1e-9) + 1e-12,
                    "a triangle of {face:?} stands {off} off its surface, within {tolerance}"
                );
            }
        }
    }
    let volume = enclosed(&cut.triangles);
    assert!(
        (volume - exact).abs() <= tolerance * curved * 1.01 + 1e-9 * exact,
        "the triangles enclose {volume}, the body {exact}, within {tolerance}"
    );
    cut
}

#[test]
fn a_shaft_cut_by_a_slanted_plane_is_drawn_closed_and_uncrossed() {
    let exact = PI * STOCK_RADIUS * STOCK_RADIUS * HEIGHT;
    for lean in [0.1, -0.25, 0.45] {
        for tolerance in TOLERANCES {
            held(&fixtures::stock_cut_at_a_slant(lean), tolerance, exact);
        }
    }
}

#[test]
fn a_shaft_cut_by_a_slanted_plane_a_hair_above_its_foot_s_rim_is_drawn_closed_and_uncrossed() {
    let exact = PI * STOCK_RADIUS * STOCK_RADIUS * HEIGHT;
    let eps = Scale::of(STOCK_RADIUS).eps();
    for hair in [2.0 * eps, 30.0 * eps, 1e3 * eps, 1e-4, 1e-2] {
        let lean = (HEIGHT - hair) / STOCK_RADIUS;
        for tolerance in TOLERANCES {
            held(&fixtures::stock_cut_at_a_slant(lean), tolerance, exact);
        }
    }
}

#[test]
fn a_shaft_cut_by_a_slanted_plane_a_hair_from_tangent_is_drawn_closed_and_uncrossed() {
    for tilt in [0.1, 1e-3, 1e-5, 1e-7] {
        let body = slanted::stock_with_a_sliver_cut_at_a_slant(tilt);
        let exact = slanted::stock_with_a_sliver_cut_at_a_slant_volume(tilt);
        for tolerance in TOLERANCES {
            held(&body, tolerance, exact);
        }
    }
}

#[test]
fn a_point_s_tip_cut_off_at_a_slant_is_drawn_closed_and_uncrossed() {
    for lean in [0.3, -0.9, 1.5, 1.9] {
        let body = slanted::tip_cut_at_a_slant(lean);
        for tolerance in TOLERANCES {
            held(&body, tolerance, slanted::tip_cut_at_a_slant_volume(lean));
        }
    }
}

#[test]
fn a_point_cut_at_a_slant_below_a_narrow_rim_is_drawn_closed_and_uncrossed() {
    for lean in [0.3, -0.9, 1.5, 1.9] {
        let body = slanted::frustum_cut_at_a_slant(lean);
        for tolerance in TOLERANCES {
            held(
                &body,
                tolerance,
                slanted::frustum_cut_at_a_slant_volume(lean),
            );
        }
    }
}

#[test]
fn a_flat_on_a_chamfer_is_drawn_closed_and_uncrossed() {
    let body = slanted::chamfer_with_a_flat(6.0);
    for tolerance in TOLERANCES {
        held(&body, tolerance, slanted::chamfer_with_a_flat_volume());
    }
}

#[test]
fn two_holes_square_into_one_slanted_face_are_drawn_closed_and_uncrossed() {
    for apart in [8.0, 0.5, 1e-3] {
        let body = slanted::slanted_block_with_two_holes(apart);
        for tolerance in TOLERANCES {
            held(
                &body,
                tolerance,
                slanted::slanted_block_with_two_holes_volume(apart),
            );
        }
    }
}
