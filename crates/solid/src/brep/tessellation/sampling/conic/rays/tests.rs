use std::f64::consts::TAU;

use glam::{DQuat, DVec2, DVec3};

use super::{ADRIFT, drift, leaving_together};
use crate::brep::conic::{Conic, Round};
use crate::brep::curve::Curve;
use crate::brep::topology::{Body, Edge};
use crate::profile::{Contour, Frame, Run};

const LOW: f64 = 10.0;
const HIGH: f64 = 60.0;
const RADIUS: f64 = 3.5;

/// An elliptic shaft about the line x = 2, y = -1, of half-axes 5 along X
/// and `RADIUS`, from `LOW` to `HIGH`, bored by a round hole of `RADIUS`
/// leaning `lean` degrees about Y round a point of that line one above
/// `HIGH`: on the top cap the two rims leave the tips of the crescents
/// together, a few thousandths off the minor axis, and the axes part by
/// `(HIGH - LOW) * sin(lean)` over the shaft.
fn shaft_bored_leaning(lean: f64) -> Body {
    let center = DVec2::new(2.0, -1.0);
    let shaft = Contour {
        corners: vec![center + DVec2::X * 5.0],
        runs: vec![Run::Oval {
            center,
            first: DVec2::X * 5.0,
            second: RADIUS,
            turn: TAU,
        }],
    };
    let ground = Frame {
        origin: DVec3::Z * LOW,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let body =
        Body::raised(&shaft, &[], ground, DVec3::Z * (HIGH - LOW)).expect("the shaft raises");
    let turn = DQuat::from_rotation_y(lean.to_radians());
    let hinge = DVec3::new(2.0, -1.0, HIGH + 1.0);
    let leaning = Frame {
        origin: hinge + turn * (DVec3::NEG_Z * (HIGH + 1.0 - LOW + 20.0)),
        u: turn * DVec3::X,
        v: turn * DVec3::Y,
    };
    let bore = Contour {
        corners: vec![DVec2::X * RADIUS],
        runs: vec![Run::Round {
            center: DVec2::ZERO,
            turn: TAU,
        }],
    };
    let tool = Body::raised(&bore, &[], leaning, leaning.normal() * (HIGH - LOW + 41.0))
        .expect("the bore raises");
    body.cut_by(&tool).expect("the bore is cut")
}

/// Each elliptic rim of the top cap with each bore's rim it shares a tip
/// with, once for every tip they share.
fn tips_on_the_top_cap(body: &Body) -> Vec<([&Edge; 2], [&Conic; 2], DVec3)> {
    let conics: Vec<(&Edge, &Conic)> = body
        .edge_ids()
        .filter_map(|id| {
            let edge = body.edge(id);
            match body.curve(edge.curve) {
                Curve::Conic(conic) => Some((edge, conic)),
                Curve::Line(_) | Curve::Circle(_) | Curve::Meet(_) => None,
            }
        })
        .collect();
    let mut tips = Vec::new();
    for &(edge, conic) in &conics {
        let Round::Elliptic(_) = conic.round else {
            continue;
        };
        for &(other, theirs) in &conics {
            let Round::Cylinder(_) = theirs.round else {
                continue;
            };
            let (Some(ends), Some(their_ends)) = (edge.ends, other.ends) else {
                continue;
            };
            for end in ends.iter().filter(|end| their_ends.contains(end)) {
                let at = body.vertex(*end).point;
                if (at.z - HIGH).abs() < 1e-6 {
                    tips.push(([edge, other], [conic, theirs], at));
                }
            }
        }
    }
    tips
}

#[test]
fn an_elliptic_rim_and_a_bore_s_leaving_a_tip_together_hand_each_other_samples_where_they_drift_past_a_tenth_of_the_coarsest_step()
 {
    let body = shaft_bored_leaning(0.2);
    let tips = tips_on_the_top_cap(&body);
    assert_eq!(
        tips.len(),
        4,
        "two rims on the top cap, each pair sharing both tips"
    );
    for ([edge, other], [conic, theirs], at) in tips {
        let drifted = drift(
            &body,
            [&conic.round, &theirs.round],
            at,
            0.2_f64.to_radians().sin(),
        );
        assert!(
            drifted > 1.2 * ADRIFT,
            "the rims drift {drifted}, past {ADRIFT}"
        );
        assert!(leaving_together(&body, [edge, other], [conic, theirs]));
        assert!(leaving_together(&body, [other, edge], [theirs, conic]));
    }
}

#[test]
fn an_elliptic_rim_and_a_bore_s_leaving_a_tip_together_keep_their_own_samples_where_they_drift_under_a_tenth_of_the_coarsest_step()
 {
    let body = shaft_bored_leaning(0.12);
    let tips = tips_on_the_top_cap(&body);
    assert_eq!(
        tips.len(),
        4,
        "two rims on the top cap, each pair sharing both tips"
    );
    for ([edge, other], [conic, theirs], at) in tips {
        let drifted = drift(
            &body,
            [&conic.round, &theirs.round],
            at,
            0.12_f64.to_radians().sin(),
        );
        assert!(
            drifted < 0.8 * ADRIFT,
            "the rims drift {drifted}, under {ADRIFT}"
        );
        assert!(!leaving_together(&body, [edge, other], [conic, theirs]));
        assert!(!leaving_together(&body, [other, edge], [theirs, conic]));
    }
}

#[test]
fn the_drift_of_an_elliptic_rim_and_a_bore_s_is_how_far_their_axes_part_over_the_shaft_against_the_bore_s_radius()
 {
    let lean = 0.2_f64.to_radians();
    let body = shaft_bored_leaning(0.2);
    let tips = tips_on_the_top_cap(&body);
    assert!(!tips.is_empty(), "a tip on the top cap");
    let expected = (HIGH - LOW) * lean.sin() / RADIUS;
    for (_, [conic, theirs], at) in tips {
        let drifted = drift(&body, [&conic.round, &theirs.round], at, lean.sin());
        assert!(
            (drifted - expected).abs() <= 1e-9 * expected,
            "the rims drift {drifted}, expected {expected}"
        );
    }
}
