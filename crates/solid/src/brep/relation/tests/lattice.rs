//! The campaign's lattice: planes of the origin moved along their normal,
//! cylinders along X, Y and Z at lattice positions, every pair held against
//! arithmetic done axis by axis, exactly and moved by a hair.

use std::collections::BTreeMap;

use super::*;

#[derive(Clone, Copy, Debug)]
enum Piece {
    Flat {
        normal: usize,
        offset: f64,
    },
    Round {
        axis: usize,
        at: [f64; 2],
        radius: f64,
    },
}

fn others(axis: usize) -> [usize; 2] {
    [(axis + 1) % 3, (axis + 2) % 3]
}

/// Where a cylinder's axis stands along a world axis square to it.
fn coordinate(axis: usize, at: [f64; 2], along: usize) -> f64 {
    if others(axis)[0] == along {
        at[0]
    } else {
        at[1]
    }
}

impl Piece {
    fn surface(self) -> Surface {
        match self {
            Piece::Flat { normal, offset } => {
                plane(DVec3::AXES[normal] * offset, DVec3::AXES[normal])
            }
            Piece::Round { axis, at, radius } => {
                let mut point = DVec3::ZERO;
                for (index, along) in others(axis).into_iter().enumerate() {
                    point[along] = at[index];
                }
                cylinder(point, DVec3::AXES[axis], radius)
            }
        }
    }

    /// The piece moved by `by` each way it can move.
    fn moved(self, by: f64) -> Vec<Piece> {
        match self {
            Piece::Flat { normal, offset } => vec![Piece::Flat {
                normal,
                offset: offset + by,
            }],
            Piece::Round { axis, at, radius } => vec![
                Piece::Round {
                    axis,
                    at: [at[0] + by, at[1]],
                    radius,
                },
                Piece::Round {
                    axis,
                    at: [at[0], at[1] + by],
                    radius,
                },
                Piece::Round {
                    axis,
                    at,
                    radius: radius + by,
                },
            ],
        }
    }
}

fn touching(gap: f64, eps: f64) -> &'static str {
    if gap > eps {
        "apart"
    } else if gap >= -eps {
        "tangent"
    } else {
        "lines"
    }
}

fn expected(one: Piece, other: Piece, eps: f64) -> &'static str {
    match (one, other) {
        (
            Piece::Flat {
                normal: one_normal,
                offset: one_offset,
            },
            Piece::Flat {
                normal: other_normal,
                offset: other_offset,
            },
        ) => {
            if one_normal != other_normal {
                "line"
            } else if (one_offset - other_offset).abs() <= eps {
                "same"
            } else {
                "apart"
            }
        }
        (Piece::Flat { normal, offset }, Piece::Round { axis, at, radius })
        | (Piece::Round { axis, at, radius }, Piece::Flat { normal, offset }) => {
            if normal == axis {
                "circle"
            } else {
                touching((coordinate(axis, at, normal) - offset).abs() - radius, eps)
            }
        }
        (
            Piece::Round {
                axis: one_axis,
                at: one_at,
                radius: one_radius,
            },
            Piece::Round {
                axis: other_axis,
                at: other_at,
                radius: other_radius,
            },
        ) => {
            if one_axis == other_axis {
                let distance = (one_at[0] - other_at[0]).hypot(one_at[1] - other_at[1]);
                if distance <= eps && (one_radius - other_radius).abs() <= eps {
                    return "same";
                }
                let outside = touching(distance - (one_radius + other_radius), eps);
                let inside = touching((one_radius - other_radius).abs() - distance, eps);
                if outside == "apart" || inside == "apart" {
                    "apart"
                } else if outside == "tangent" || inside == "tangent" {
                    "tangent"
                } else {
                    "lines"
                }
            } else {
                let third = 3 - one_axis - other_axis;
                let across = (coordinate(one_axis, one_at, third)
                    - coordinate(other_axis, other_at, third))
                .abs();
                if across - (one_radius + other_radius) > eps {
                    "apart"
                } else {
                    "meet"
                }
            }
        }
    }
}

fn lattice() -> Vec<Piece> {
    let mut pieces = Vec::new();
    for normal in 0..3 {
        for offset in [-20.0, -10.0, 0.0, 10.0, 20.0] {
            pieces.push(Piece::Flat { normal, offset });
        }
    }
    let places = [-15.0, 0.0, 5.0, 15.0];
    for axis in 0..3 {
        for first in places {
            for second in places {
                for radius in [5.0, 20.0] {
                    pieces.push(Piece::Round {
                        axis,
                        at: [first, second],
                        radius,
                    });
                }
            }
        }
    }
    pieces
}

#[test]
fn every_corner_of_a_smaller_lattice_lies_on_its_three_surfaces() {
    let eps = scale().eps();
    let mut surfaces = Vec::new();
    for axis in 0..3 {
        for offset in [-10.0, 0.0, 10.0] {
            surfaces.push(
                Piece::Flat {
                    normal: axis,
                    offset,
                }
                .surface(),
            );
        }
        for at in [[-5.0, 0.0], [0.0, 0.0], [5.0, 5.0]] {
            for radius in [5.0, 10.0] {
                surfaces.push(Piece::Round { axis, at, radius }.surface());
            }
        }
    }
    let mut corners = 0;
    for (index, one) in surfaces.iter().enumerate() {
        for other in &surfaces[index + 1..] {
            for curve in relation(one, other, scale()).curves() {
                for third in &surfaces {
                    let Crossings::At(found) = crossings(&curve, third, scale()) else {
                        continue;
                    };
                    for crossing in found {
                        let within = if crossing.tangent { eps } else { 1e-12 * REACH };
                        for surface in [one, other] {
                            assert!(surface.distance(crossing.point).abs() <= eps);
                        }
                        assert!(
                            third.distance(crossing.point).abs() <= within,
                            "{curve:?} against {third:?}: {crossing:?}"
                        );
                        corners += 1;
                    }
                }
            }
        }
    }
    assert!(corners > 1000, "{corners}");
}

#[test]
fn every_pair_of_the_lattice_meets_as_arithmetic_says_exactly_and_a_hair_away() {
    let eps = scale().eps();
    let pieces = lattice();
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for (index, &one) in pieces.iter().enumerate() {
        for &other in &pieces[index..] {
            let mut variants = vec![(other, 0.0)];
            for by in [-2.0 * eps, -eps / 2.0, eps / 2.0, 2.0 * eps] {
                variants.extend(other.moved(by).into_iter().map(|piece| (piece, by)));
            }
            for (other, by) in variants {
                let (surface_one, surface_other) = (one.surface(), other.surface());
                let found = relation(&surface_one, &surface_other, scale());
                let wanted = expected(one, other, eps);
                assert_eq!(
                    class(&found),
                    wanted,
                    "{one:?} against {other:?}: {found:?}"
                );
                assert_eq!(
                    relation(&surface_other, &surface_one, scale()),
                    found,
                    "{one:?} against {other:?} the other way"
                );
                let within = if by.abs() < eps { eps } else { 1e-12 * REACH };
                assert_on_both(&found, &surface_one, &surface_other, within);
                *seen.entry(wanted).or_default() += 1;
            }
        }
    }
    for class in [
        "apart", "same", "line", "lines", "tangent", "circle", "meet",
    ] {
        assert!(
            seen.get(class).is_some_and(|&count| count > 0),
            "{class}: {seen:?}"
        );
    }
}
