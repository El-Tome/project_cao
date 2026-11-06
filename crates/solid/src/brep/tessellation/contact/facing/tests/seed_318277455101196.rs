use std::f64::consts::PI;

use glam::DVec3;

use crate::brep::conic::{Arm, Conic, Round};
use crate::brep::curve::{Circle, Curve, Meet, Wall};
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::tessellation::tests::fixtures;
use crate::brep::topology::{Body, Coedge};

/// The body the exact kernel builds for the walls draw's seed 318277455101196
/// once two cylinders closer than a chord's sag are no longer declined, as
/// it listed it: a shaft of radius 60, cut by another of its radius leaning
/// 1e-4° whose axis crosses its own beyond the part, then bored at 30° by one
/// of radius 45.
pub(super) fn body() -> Body {
    let reach = 381.0001047197553;
    let mut build = fixtures::Build::new();
    let surfaces = [
        build.surface(Surface::Plane(Plane {
            origin: DVec3::new(-0.0, -30.0, -0.0),
            normal: DVec3::new(0.0, 1.0, 0.0),
            u: DVec3::new(1.0, 0.0, 0.0),
            v: DVec3::new(0.0, 0.0, -1.0),
        })),
        build.surface(Surface::Plane(Plane {
            origin: DVec3::new(0.0, 105.0, 0.0),
            normal: DVec3::new(0.0, 1.0, 0.0),
            u: DVec3::new(1.0, 0.0, 0.0),
            v: DVec3::new(0.0, 0.0, -1.0),
        })),
        build.surface(Surface::Cylinder(Cylinder {
            origin: DVec3::new(-60.0, 0.0, 30.0),
            axis: DVec3::new(0.0, 1.0, 0.0),
            radius: 60.0,
            u: DVec3::new(1.0, 0.0, 0.0),
            v: DVec3::new(0.0, 0.0, -1.0),
        })),
        build.surface(Surface::Cylinder(Cylinder {
            origin: DVec3::new(
                -60.000664970262235,
                0.00010472091554447616,
                30.000000000000057,
            ),
            axis: DVec3::new(
                1.745329251993444e-6,
                0.9999999999984771,
                -1.5259917864581123e-16,
            ),
            radius: 60.0,
            u: DVec3::new(2.6633581032070766e-22, 1.5259917864557883e-16, 1.0),
            v: DVec3::new(
                0.9999999999984771,
                -1.745329251993444e-6,
                -4.70197740328915e-38,
            ),
        })),
        build.surface(Surface::Cylinder(Cylinder {
            origin: DVec3::new(-45.0, 16.39393884397515, 9.465045004647408),
            axis: DVec3::new(0.0, -0.4999999999999999, 0.8660254037844388),
            radius: 45.0,
            u: DVec3::new(1.0, 0.0, 0.0),
            v: DVec3::new(-0.0, 0.8660254037844388, 0.4999999999999999),
        })),
    ];
    let vertices = [
        build.vertex(DVec3::new(-60.00035867040724, -30.0, -29.99999999892796)),
        build.vertex(DVec3::new(-60.000240850190785, 105.0, 89.9999999995166)),
        build.vertex(DVec3::new(-60.0002408606827, 105.0, -29.999999999516554)),
        build.vertex(DVec3::new(0.0, 4.538077049611484, 30.0)),
        build.vertex(DVec3::new(-4.171037548973025, -30.0, 51.980149035932854)),
        build.vertex(DVec3::new(
            -4.1713033191213285,
            -30.000000000000018,
            51.97900205139862,
        )),
        build.vertex(DVec3::new(
            -60.000315998198175,
            18.88672677110192,
            89.99999999916787,
        )),
        build.vertex(DVec3::new(
            -60.000341051799225,
            -9.810562376889925,
            -29.9999999990307,
        )),
        build.vertex(DVec3::new(
            -60.00025554868978,
            88.16878375187913,
            -29.999999999455795,
        )),
    ];
    let edges = [
        build.edge(
            Curve::Circle(Circle {
                center: DVec3::new(-60.0, -30.0, 30.0),
                axis: DVec3::new(0.0, 1.0, 0.0),
                radius: 60.0,
                u: DVec3::new(1.0, 0.0, 0.0),
                v: DVec3::new(0.0, 0.0, -1.0),
            }),
            Some([vertices[4], vertices[0]]),
            -0.375068015591994,
            1.5708023046350172,
        ),
        build.edge(
            Curve::Circle(Circle {
                center: DVec3::new(-60.0, 105.0, 30.0),
                axis: DVec3::new(0.0, 1.0, 0.0),
                radius: 60.0,
                u: DVec3::new(1.0, 0.0, 0.0),
                v: DVec3::new(0.0, 0.0, -1.0),
            }),
            Some([vertices[1], vertices[2]]),
            -1.570800340964743,
            1.5708003411396083,
        ),
        build.edge(
            Curve::Conic(Conic {
                plane: Plane {
                    origin: DVec3::new(-0.0, -30.0, -0.0),
                    normal: DVec3::new(0.0, 1.0, 0.0),
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                },
                round: Round::Cylinder(Cylinder {
                    origin: DVec3::new(
                        -60.000664970262235,
                        0.00010472091554447616,
                        30.000000000000057,
                    ),
                    axis: DVec3::new(
                        1.745329251993444e-6,
                        0.9999999999984771,
                        -1.5259917864581123e-16,
                    ),
                    radius: 60.0,
                    u: DVec3::new(2.6633581032070766e-22, 1.5259917864557883e-16, 1.0),
                    v: DVec3::new(
                        0.9999999999984771,
                        -1.745329251993444e-6,
                        -4.70197740328915e-38,
                    ),
                }),
                anchor: DVec3::new(-0.0007173302311731468, -30.0, 29.999999994754102),
                tangent: DVec3::new(-8.743266649228343e-11, -1.7449585382585184e-22, -1.0),
                bend: DVec3::new(
                    -0.016666666666692052,
                    -3.5615964814406186e-40,
                    1.4572111082069424e-12,
                ),
                k: 0.00027777777777777783,
                arm: Arm::Whole,
            }),
            Some([vertices[5], vertices[0]]),
            -22.502848268856233,
            94.24742094253251,
        ),
        build.edge(
            Curve::Conic(Conic {
                plane: Plane {
                    origin: DVec3::new(0.0, 105.0, 0.0),
                    normal: DVec3::new(0.0, 1.0, 0.0),
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                },
                round: Round::Cylinder(Cylinder {
                    origin: DVec3::new(
                        -60.000664970262235,
                        0.00010472091554447616,
                        30.000000000000057,
                    ),
                    axis: DVec3::new(
                        1.745329251993444e-6,
                        0.9999999999984771,
                        -1.5259917864581123e-16,
                    ),
                    radius: 60.0,
                    u: DVec3::new(2.6633581032070766e-22, 1.5259917864557883e-16, 1.0),
                    v: DVec3::new(
                        0.9999999999984771,
                        -1.745329251993444e-6,
                        -4.70197740328915e-38,
                    ),
                }),
                anchor: DVec3::new(-0.000481710782153673, 105.0, 29.99999999475408),
                tangent: DVec3::new(-8.743266649228343e-11, -1.7449585382585184e-22, -1.0),
                bend: DVec3::new(
                    -0.016666666666692052,
                    -3.5615964814406186e-40,
                    1.4572111082069424e-12,
                ),
                k: 0.00027777777777777783,
                arm: Arm::Whole,
            }),
            Some([vertices[1], vertices[2]]),
            -94.247538752257,
            94.247538752257,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(
                        -60.000664970262235,
                        0.00010472091554447616,
                        30.000000000000057,
                    ),
                    axis: DVec3::new(
                        1.745329251993444e-6,
                        0.9999999999984771,
                        -1.5259917864581123e-16,
                    ),
                    radius: 60.0,
                    u: DVec3::new(2.6633581032070766e-22, 1.5259917864557883e-16, 1.0),
                    v: DVec3::new(
                        0.9999999999984771,
                        -1.745329251993444e-6,
                        -4.70197740328915e-38,
                    ),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-60.0, 0.0, 30.0),
                    axis: DVec3::new(0.0, 1.0, 0.0),
                    radius: 60.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                }),
                component: 0,
            }),
            Some([vertices[1], vertices[6]]),
            3.141596667847073,
            3.1415979203138624,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(
                        -60.000664970262235,
                        0.00010472091554447616,
                        30.000000000000057,
                    ),
                    axis: DVec3::new(
                        1.745329251993444e-6,
                        0.9999999999984771,
                        -1.5259917864581123e-16,
                    ),
                    radius: 60.0,
                    u: DVec3::new(2.6633581032070766e-22, 1.5259917864557883e-16, 1.0),
                    v: DVec3::new(
                        0.9999999999984771,
                        -1.745329251993444e-6,
                        -4.70197740328915e-38,
                    ),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-60.0, 0.0, 30.0),
                    axis: DVec3::new(0.0, 1.0, 0.0),
                    radius: 60.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                }),
                component: 0,
            }),
            Some([vertices[0], vertices[7]]),
            6.283179329426898,
            6.283179623070366,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(
                        -60.000664970262235,
                        0.00010472091554447616,
                        30.000000000000057,
                    ),
                    axis: DVec3::new(
                        1.745329251993444e-6,
                        0.9999999999984771,
                        -1.5259917864581123e-16,
                    ),
                    radius: 60.0,
                    u: DVec3::new(2.6633581032070766e-22, 1.5259917864557883e-16, 1.0),
                    v: DVec3::new(
                        0.9999999999984771,
                        -1.745329251993444e-6,
                        -4.70197740328915e-38,
                    ),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-60.0, 0.0, 30.0),
                    axis: DVec3::new(0.0, 1.0, 0.0),
                    radius: 60.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                }),
                component: 0,
            }),
            Some([vertices[8], vertices[2]]),
            6.28318104812219,
            6.283181292922307,
        ),
        build.edge(
            Curve::Conic(Conic {
                plane: Plane {
                    origin: DVec3::new(-0.0, -30.0, -0.0),
                    normal: DVec3::new(0.0, 1.0, 0.0),
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                },
                round: Round::Cylinder(Cylinder {
                    origin: DVec3::new(-45.0, 16.39393884397515, 9.465045004647408),
                    axis: DVec3::new(0.0, -0.4999999999999999, 0.8660254037844388),
                    radius: 45.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(-0.0, 0.8660254037844388, 0.4999999999999999),
                }),
                anchor: DVec3::new(-45.00000000000001, -30.0, -0.1782957543443331),
                tangent: DVec3::new(1.0, -5.3028761936245346e-17, -2.7554552980815453e-16),
                bend: DVec3::new(
                    4.082155997157844e-18,
                    -1.443144523732664e-34,
                    0.04444444444444446,
                ),
                k: 0.0004938271604938271,
                arm: Arm::Whole,
            }),
            Some([vertices[5], vertices[4]]),
            51.15726569976151,
            51.157897781363985,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-60.0, 0.0, 30.0),
                    axis: DVec3::new(0.0, 1.0, 0.0),
                    radius: 60.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-45.0, 16.39393884397515, 9.465045004647408),
                    axis: DVec3::new(0.0, -0.4999999999999999, 0.8660254037844388),
                    radius: 45.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(-0.0, 0.8660254037844388, 0.4999999999999999),
                }),
                component: 0,
            }),
            Some([vertices[6], vertices[3]]),
            1.230951969335824,
            PI,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-60.0, 0.0, 30.0),
                    axis: DVec3::new(0.0, 1.0, 0.0),
                    radius: 60.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-45.0, 16.39393884397515, 9.465045004647408),
                    axis: DVec3::new(0.0, -0.4999999999999999, 0.8660254037844388),
                    radius: 45.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(-0.0, 0.8660254037844388, 0.4999999999999999),
                }),
                component: 0,
            }),
            Some([vertices[3], vertices[7]]),
            PI,
            5.052233928792479,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-60.0, 0.0, 30.0),
                    axis: DVec3::new(0.0, 1.0, 0.0),
                    radius: 60.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-45.0, 16.39393884397515, 9.465045004647408),
                    axis: DVec3::new(0.0, -0.4999999999999999, 0.8660254037844388),
                    radius: 45.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(-0.0, 0.8660254037844388, 0.4999999999999999),
                }),
                component: 0,
            }),
            Some([vertices[8], vertices[3]]),
            7.514138701326606,
            9.42477796076938,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-60.0, 0.0, 30.0),
                    axis: DVec3::new(0.0, 1.0, 0.0),
                    radius: 60.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(0.0, 0.0, -1.0),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-45.0, 16.39393884397515, 9.465045004647408),
                    axis: DVec3::new(0.0, -0.4999999999999999, 0.8660254037844388),
                    radius: 45.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(-0.0, 0.8660254037844388, 0.4999999999999999),
                }),
                component: 0,
            }),
            Some([vertices[3], vertices[4]]),
            9.42477796076938,
            9.858732114645079,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(
                        -60.000664970262235,
                        0.00010472091554447616,
                        30.000000000000057,
                    ),
                    axis: DVec3::new(
                        1.745329251993444e-6,
                        0.9999999999984771,
                        -1.5259917864581123e-16,
                    ),
                    radius: 60.0,
                    u: DVec3::new(2.6633581032070766e-22, 1.5259917864557883e-16, 1.0),
                    v: DVec3::new(
                        0.9999999999984771,
                        -1.745329251993444e-6,
                        -4.70197740328915e-38,
                    ),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-45.0, 16.39393884397515, 9.465045004647408),
                    axis: DVec3::new(0.0, -0.4999999999999999, 0.8660254037844388),
                    radius: 45.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(-0.0, 0.8660254037844388, 0.4999999999999999),
                }),
                component: 0,
            }),
            Some([vertices[6], vertices[8]]),
            1.2309551162565082,
            5.052228766104081,
        ),
        build.edge(
            Curve::Meet(Meet {
                first: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(
                        -60.000664970262235,
                        0.00010472091554447616,
                        30.000000000000057,
                    ),
                    axis: DVec3::new(
                        1.745329251993444e-6,
                        0.9999999999984771,
                        -1.5259917864581123e-16,
                    ),
                    radius: 60.0,
                    u: DVec3::new(2.6633581032070766e-22, 1.5259917864557883e-16, 1.0),
                    v: DVec3::new(
                        0.9999999999984771,
                        -1.745329251993444e-6,
                        -4.70197740328915e-38,
                    ),
                }),
                second: Wall::Cylinder(Cylinder {
                    origin: DVec3::new(-45.0, 16.39393884397515, 9.465045004647408),
                    axis: DVec3::new(0.0, -0.4999999999999999, 0.8660254037844388),
                    radius: 45.0,
                    u: DVec3::new(1.0, 0.0, 0.0),
                    v: DVec3::new(-0.0, 0.8660254037844388, 0.4999999999999999),
                }),
                component: 0,
            }),
            Some([vertices[7], vertices[5]]),
            7.5141438631714665,
            9.858711028108953,
        ),
    ];
    build.face(
        surfaces[0],
        true,
        vec![vec![
            Coedge {
                edge: edges[7],
                forward: false,
            },
            Coedge {
                edge: edges[2],
                forward: true,
            },
            Coedge {
                edge: edges[0],
                forward: false,
            },
        ]],
    );
    build.face(
        surfaces[1],
        false,
        vec![vec![
            Coedge {
                edge: edges[1],
                forward: true,
            },
            Coedge {
                edge: edges[3],
                forward: false,
            },
        ]],
    );
    build.face(
        surfaces[2],
        false,
        vec![vec![
            Coedge {
                edge: edges[0],
                forward: true,
            },
            Coedge {
                edge: edges[5],
                forward: true,
            },
            Coedge {
                edge: edges[9],
                forward: false,
            },
            Coedge {
                edge: edges[11],
                forward: true,
            },
        ]],
    );
    build.face(
        surfaces[2],
        false,
        vec![vec![
            Coedge {
                edge: edges[1],
                forward: false,
            },
            Coedge {
                edge: edges[4],
                forward: true,
            },
            Coedge {
                edge: edges[8],
                forward: true,
            },
            Coedge {
                edge: edges[10],
                forward: false,
            },
            Coedge {
                edge: edges[6],
                forward: true,
            },
        ]],
    );
    build.face(
        surfaces[3],
        true,
        vec![vec![
            Coedge {
                edge: edges[13],
                forward: false,
            },
            Coedge {
                edge: edges[5],
                forward: false,
            },
            Coedge {
                edge: edges[2],
                forward: false,
            },
        ]],
    );
    build.face(
        surfaces[3],
        true,
        vec![vec![
            Coedge {
                edge: edges[6],
                forward: false,
            },
            Coedge {
                edge: edges[12],
                forward: false,
            },
            Coedge {
                edge: edges[4],
                forward: false,
            },
            Coedge {
                edge: edges[3],
                forward: true,
            },
        ]],
    );
    build.face(
        surfaces[4],
        true,
        vec![vec![
            Coedge {
                edge: edges[11],
                forward: false,
            },
            Coedge {
                edge: edges[9],
                forward: true,
            },
            Coedge {
                edge: edges[13],
                forward: true,
            },
            Coedge {
                edge: edges[7],
                forward: true,
            },
        ]],
    );
    build.face(
        surfaces[4],
        true,
        vec![vec![
            Coedge {
                edge: edges[12],
                forward: true,
            },
            Coedge {
                edge: edges[10],
                forward: true,
            },
            Coedge {
                edge: edges[8],
                forward: false,
            },
        ]],
    );
    build.finish(reach)
}
