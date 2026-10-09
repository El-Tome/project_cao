//! The matter of bodies a plane cuts at a slant through a wall, built by
//! hand: their faces bounded by a conic, or by the rulings a plane through a
//! cone's tip cuts it along.

use super::*;
use crate::brep::conic::{Conic, Round};
use crate::brep::scale::Scale;
use crate::brep::surface::Cone;
use crate::brep::tessellation::tests::fixtures::{
    Build, HEIGHT, STOCK_RADIUS, stock_cut_at_a_slant,
};
use crate::brep::topology::Coedge;

const TIP: f64 = 10.0;
const BASE: f64 = 5.0;

/// The point of radius 5 about Z, its tip at height 10.
fn point() -> Cone {
    Cone::through(
        DVec3::ZERO,
        DVec3::Z,
        [DVec2::new(0.0, BASE), DVec2::new(TIP, 0.0)],
    )
}

/// A plane of the Y direction through the point `(foot, 0, 0)` of the base,
/// its normal `(x, 0, z)` pointing away from the axis: it keeps what stands
/// on that side of it.
#[derive(Clone, Copy)]
struct Cut {
    normal: DVec3,
    foot: f64,
}

impl Cut {
    /// Through the tip and the line `x = chord` of the base, moved `away`
    /// from the axis along its normal.
    fn beside_the_tip(chord: f64, away: f64) -> Cut {
        let normal = DVec3::new(TIP, 0.0, chord).normalize();
        Cut {
            normal,
            foot: (normal.z * TIP + away) / normal.x,
        }
    }

    /// Parallel to the ruling on the far side of the axis, through the line
    /// `x = foot` of the base.
    fn along_the_slope(foot: f64) -> Cut {
        Cut {
            normal: DVec3::new(TIP, 0.0, -BASE).normalize(),
            foot,
        }
    }

    /// Where the plane crosses the base's plane, at height `z`.
    fn threshold(self, z: f64) -> f64 {
        self.foot - self.normal.z / self.normal.x * z
    }
}

/// What `cut` keeps of the point: the slice of its wall over its base
/// segment, a flat face, and the segment. A cut through the tip bounds the
/// wall with the two rulings it holds; any other, with its conic.
fn point_cut(cut: Cut, through_the_tip: bool) -> Body {
    let tip = DVec3::Z * TIP;
    let cone = point();
    let mut build = Build::new();
    let wall = build.surface(Surface::Cone(cone));
    let (cutting, slant_flipped) = Plane::through(DVec3::X * cut.foot, -cut.normal);
    let slant = build.surface(Surface::Plane(cutting));
    let (base, base_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    let half = (BASE * BASE - cut.foot * cut.foot).sqrt();
    let corners = [-half, half].map(|y| DVec3::new(cut.foot, y, 0.0));
    let [left, right] = corners.map(|corner| build.vertex(corner));
    let rim = cone.circle(0.0, BASE);
    let arc = build.edge(
        Curve::Circle(rim),
        Some([left, right]),
        rim.parameter(corners[0]),
        rim.parameter(corners[1]),
    );
    let across = build.line(left, right);
    let use_of = |edge, forward| Coedge { edge, forward };
    let over = if through_the_tip {
        let apex = build.vertex(tip);
        let [up_left, up_right] = [left, right].map(|end| build.line(end, apex));
        let wall_loop = vec![
            build.leaving(up_left, apex),
            use_of(arc, true),
            build.leaving(up_right, right),
        ];
        build.face(wall, true, vec![wall_loop]);
        vec![build.leaving(up_left, left), build.leaving(up_right, apex)]
    } else {
        let [conic] = Conic::cut(&cutting, &Round::Cone(cone), Scale::of(TIP))[..] else {
            panic!("a plane off the tip cuts one conic");
        };
        let span = corners.map(|corner| conic.parameter(corner));
        let edge = build.edge(Curve::Conic(conic), Some([left, right]), span[0], span[1]);
        build.face(
            wall,
            true,
            vec![vec![use_of(arc, true), use_of(edge, false)]],
        );
        vec![use_of(edge, true)]
    };
    let mut slanted = over;
    slanted.push(build.leaving(across, right));
    build.face(slant, slant_flipped, vec![slanted]);
    let floor = vec![build.leaving(across, left), use_of(arc, false)];
    build.face(base, base_flipped, vec![floor]);
    build.finish(TIP)
}

/// What `cut` keeps, by its slices square to the axis: at each height a
/// segment of the point's section, `ρ² acos(c/ρ) − c √(ρ² − c²)`, summed by
/// Gauss–Legendre in the square root of the depth under the top of the cut,
/// where the segment closes as the depth to the power three halves.
fn sliced(cut: Cut) -> f64 {
    let radius = |z: f64| BASE * (1.0 - z / TIP);
    let top = (cut.foot - BASE) / (cut.normal.z / cut.normal.x - BASE / TIP);
    let segment = |z: f64| {
        let (rho, c) = (radius(z), cut.threshold(z));
        if c >= rho {
            return 0.0;
        }
        rho * rho * (c / rho).acos() - c * ((rho - c) * (rho + c)).sqrt()
    };
    const PANELS: usize = 400;
    let width = top.sqrt() / PANELS as f64;
    let mut total = 0.0;
    for panel in 0..PANELS {
        let middle = width * (panel as f64 + 0.5);
        for (node, weight) in GAUSS_LEGENDRE {
            for side in [-1.0, 1.0] {
                let w = middle + side * node * width / 2.0;
                total += weight * segment(top - w * w) * 2.0 * w * width / 2.0;
            }
        }
    }
    total
}

#[test]
fn a_shaft_cut_by_a_slanted_plane_encloses_pi_r_squared_times_its_height_at_the_axis() {
    for lean in [0.1, -0.25, 0.45] {
        assert_relative(
            stock_cut_at_a_slant(lean).volume(),
            PI * STOCK_RADIUS * STOCK_RADIUS * HEIGHT,
        );
    }
}

#[test]
fn a_point_cut_through_its_tip_at_a_slant_encloses_a_third_of_its_height_times_its_base_segment() {
    for chord in [2.0, -1.5, 4.5] {
        let segment =
            BASE * BASE * (chord / BASE).acos() - chord * (BASE * BASE - chord * chord).sqrt();
        let cut = Cut::beside_the_tip(chord, 0.0);
        assert_relative(point_cut(cut, true).volume(), TIP * segment / 3.0);
        assert_relative(sliced(cut), TIP * segment / 3.0);
    }
}

#[test]
fn a_point_cut_a_hair_off_its_tip_holds_its_volume() {
    let eps = Scale::of(TIP).eps();
    for chord in [2.0, -1.5] {
        for hairs in [2.0, 30.0, 1000.0] {
            let cut = Cut::beside_the_tip(chord, hairs * eps);
            assert_relative(point_cut(cut, false).volume(), sliced(cut));
        }
    }
}

#[test]
fn a_point_cut_parallel_to_its_slope_holds_its_volume() {
    for foot in [3.5, 1.0, -2.0] {
        let cut = Cut::along_the_slope(foot);
        assert_relative(point_cut(cut, false).volume(), sliced(cut));
    }
}
