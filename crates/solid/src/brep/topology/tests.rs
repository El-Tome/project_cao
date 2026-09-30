use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::*;
use crate::profile::{Contour, Frame, Run};

fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

#[test]
fn a_point_of_an_edge_is_given_the_parameter_its_edge_runs_through() {
    let slot = Contour {
        corners: vec![
            DVec2::new(-10.0, -5.0),
            DVec2::new(10.0, -5.0),
            DVec2::new(10.0, 5.0),
            DVec2::new(-10.0, 5.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(10.0, 0.0),
                turn: PI,
            },
            Run::Straight,
            Run::Round {
                center: DVec2::new(-10.0, 0.0),
                turn: PI,
            },
        ],
    };
    let disc = Contour::circle(DVec2::new(3.0, 1.0), 4.0);
    for outline in [slot, disc] {
        let body = Body::raised(&outline, &[], ground(), DVec3::Z * 10.0).expect("it raises");
        for edge in body.edge_ids() {
            let stretch = body.edge(edge);
            for share in [0.0, 1e-9, 0.25, 0.5, 0.75, 1.0 - 1e-9, 1.0] {
                let t = stretch.from + (stretch.to - stretch.from) * share;
                let found = body.parameter_on(edge, body.point_on(edge, t));
                let turn = body.curve(stretch.curve).period().unwrap_or(f64::INFINITY);
                let whole = stretch.ends.is_none();
                let same = (found - t).abs() <= 1e-12 * (1.0 + t.abs())
                    || (whole && ((found - t).abs() - turn).abs() <= 1e-12);
                assert!(
                    same,
                    "edge {edge:?} over {}..{} gives {found} for {t}",
                    stretch.from, stretch.to
                );
            }
        }
    }
}
