//! STEP files written by hand, entity by entity, to hold the reader to
//! solids whose every number is known before it reads them.

use glam::DVec3;

struct Draft {
    lines: Vec<String>,
}

impl Draft {
    fn new() -> Draft {
        Draft { lines: Vec::new() }
    }

    fn add(&mut self, entity: String) -> usize {
        self.lines.push(entity);
        self.lines.len()
    }

    fn point(&mut self, at: DVec3) -> usize {
        self.add(format!(
            "CARTESIAN_POINT('',({:?},{:?},{:?}))",
            at.x, at.y, at.z
        ))
    }

    fn direction(&mut self, along: DVec3) -> usize {
        self.add(format!(
            "DIRECTION('',({:?},{:?},{:?}))",
            along.x, along.y, along.z
        ))
    }

    fn placement(&mut self, origin: DVec3, z: DVec3, x: DVec3) -> usize {
        let (origin, z, x) = (self.point(origin), self.direction(z), self.direction(x));
        self.add(format!("AXIS2_PLACEMENT_3D('',#{origin},#{z},#{x})"))
    }

    fn vertex(&mut self, at: DVec3) -> usize {
        let point = self.point(at);
        self.add(format!("VERTEX_POINT('',#{point})"))
    }

    fn straight(&mut self, (start, from): (usize, DVec3), (end, to): (usize, DVec3)) -> usize {
        let origin = self.point(from);
        let along = self.direction(to - from);
        let vector = self.add(format!("VECTOR('',#{along},{:?})", (to - from).length()));
        let line = self.add(format!("LINE('',#{origin},#{vector})"));
        self.add(format!("EDGE_CURVE('',#{start},#{end},#{line},.T.)"))
    }

    fn face(&mut self, surface: usize, same_sense: bool, loops: &[&[(usize, bool)]]) -> usize {
        let bounds: Vec<String> = loops
            .iter()
            .enumerate()
            .map(|(index, edges)| {
                let oriented: Vec<String> = edges
                    .iter()
                    .map(|(edge, along)| {
                        let id =
                            self.add(format!("ORIENTED_EDGE('',*,*,#{edge},{})", flag(*along)));
                        format!("#{id}")
                    })
                    .collect();
                let edge_loop = self.add(format!("EDGE_LOOP('',({}))", oriented.join(",")));
                let kind = if index == 0 {
                    "FACE_OUTER_BOUND"
                } else {
                    "FACE_BOUND"
                };
                format!("#{}", self.add(format!("{kind}('',#{edge_loop},.T.)")))
            })
            .collect();
        self.add(format!(
            "ADVANCED_FACE('',({}),#{surface},{})",
            bounds.join(","),
            flag(same_sense)
        ))
    }

    fn circle(&mut self, center: DVec3, radius: f64) -> usize {
        let start = self.vertex(center + DVec3::X * radius);
        let placement = self.placement(center, DVec3::Z, DVec3::X);
        let circle = self.add(format!("CIRCLE('',#{placement},{radius:?})"));
        self.add(format!("EDGE_CURVE('',#{start},#{start},#{circle},.T.)"))
    }

    fn file(mut self, faces: &[usize]) -> String {
        let length = self.add("(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.))".into());
        let angle = self.add("(NAMED_UNIT(*)PLANE_ANGLE_UNIT()SI_UNIT($,.RADIAN.))".into());
        let solid_angle =
            self.add("(NAMED_UNIT(*)SI_UNIT($,.STERADIAN.)SOLID_ANGLE_UNIT())".into());
        let uncertainty = self.add(format!(
            "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#{length},\
             'distance_accuracy_value','')"
        ));
        let context = self.add(format!(
            "(GEOMETRIC_REPRESENTATION_CONTEXT(3)GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{uncertainty}))\
             GLOBAL_UNIT_ASSIGNED_CONTEXT((#{length},#{angle},#{solid_angle}))\
             REPRESENTATION_CONTEXT('',''))"
        ));
        let faces: Vec<String> = faces.iter().map(|face| format!("#{face}")).collect();
        let shell = self.add(format!("CLOSED_SHELL('',({}))", faces.join(",")));
        let brep = self.add(format!("MANIFOLD_SOLID_BREP('',#{shell})"));
        self.add(format!(
            "ADVANCED_BREP_SHAPE_REPRESENTATION('',(#{brep}),#{context})"
        ));
        let data: String = self
            .lines
            .iter()
            .enumerate()
            .map(|(index, line)| format!("#{}={line};\n", index + 1))
            .collect();
        format!(
            "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('a hand-written solid'),'2;1');\n\
             FILE_NAME('hand.step','2026-11-04T00:00:00',(''),(''),'','','');\n\
             FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));\nENDSEC;\nDATA;\n\
             {data}ENDSEC;\nEND-ISO-10303-21;\n"
        )
    }
}

fn flag(value: bool) -> &'static str {
    if value { ".T." } else { ".F." }
}

/// The box from (1, 1, 1) to (3, 4, 5), its top face's plane laid at height
/// `top`: at 5 the box is sound, anywhere else its top loop stands off it.
pub fn box_with_its_top_at(top: f64) -> String {
    let mut draft = Draft::new();
    let corners: Vec<DVec3> = (0..8)
        .map(|index: usize| {
            let x = if matches!(index % 4, 1 | 2) { 3.0 } else { 1.0 };
            let y = if index % 4 >= 2 { 4.0 } else { 1.0 };
            let z = if index >= 4 { 5.0 } else { 1.0 };
            DVec3::new(x, y, z)
        })
        .collect();
    let vertices: Vec<(usize, DVec3)> = corners
        .iter()
        .map(|corner| (draft.vertex(*corner), *corner))
        .collect();
    let pairs = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    let e: Vec<usize> = pairs
        .iter()
        .map(|(from, to)| draft.straight(vertices[*from], vertices[*to]))
        .collect();
    let mut plane = |origin: DVec3, z: DVec3, x: DVec3| {
        let placement = draft.placement(origin, z, x);
        draft.add(format!("PLANE('',#{placement})"))
    };
    let (x, y, z) = (DVec3::X, DVec3::Y, DVec3::Z);
    let planes = [
        plane(DVec3::ONE, -z, x),
        plane(DVec3::new(1.0, 1.0, top), z, x),
        plane(DVec3::ONE, -y, x),
        plane(DVec3::new(1.0, 4.0, 1.0), y, x),
        plane(DVec3::ONE, -x, y),
        plane(DVec3::new(3.0, 1.0, 1.0), x, y),
    ];
    let loops: [&[(usize, bool)]; 6] = [
        &[(e[3], false), (e[2], false), (e[1], false), (e[0], false)],
        &[(e[4], true), (e[5], true), (e[6], true), (e[7], true)],
        &[(e[0], true), (e[9], true), (e[4], false), (e[8], false)],
        &[(e[11], true), (e[6], false), (e[10], false), (e[2], true)],
        &[(e[8], true), (e[7], false), (e[11], false), (e[3], true)],
        &[(e[1], true), (e[10], true), (e[5], false), (e[9], false)],
    ];
    let faces: Vec<usize> = planes
        .iter()
        .zip(loops)
        .map(|(plane, edges)| draft.face(*plane, true, &[edges]))
        .collect();
    draft.file(&faces)
}

/// A cylinder of radius 2 standing on (1, 1, 0) from height 0 to 5: two
/// whole circles, one vertex each, and no seam.
pub fn cylinder() -> String {
    let mut draft = Draft::new();
    let foot = DVec3::new(1.0, 1.0, 0.0);
    let mut circle = |height: f64| {
        let start = foot + DVec3::new(2.0, 0.0, height);
        let vertex = draft.vertex(start);
        let placement = draft.placement(foot + DVec3::Z * height, DVec3::Z, DVec3::X);
        let circle = draft.add(format!("CIRCLE('',#{placement},2.)"));
        draft.add(format!("EDGE_CURVE('',#{vertex},#{vertex},#{circle},.T.)"))
    };
    let (low, high) = (circle(0.0), circle(5.0));
    let bottom = draft.placement(foot, DVec3::Z, DVec3::X);
    let bottom = draft.add(format!("PLANE('',#{bottom})"));
    let top = draft.placement(foot + DVec3::Z * 5.0, DVec3::Z, DVec3::X);
    let top = draft.add(format!("PLANE('',#{top})"));
    let wall = draft.placement(foot, DVec3::Z, DVec3::X);
    let wall = draft.add(format!("CYLINDRICAL_SURFACE('',#{wall},2.)"));
    let faces = [
        draft.face(bottom, false, &[&[(low, false)]]),
        draft.face(top, true, &[&[(high, true)]]),
        draft.face(wall, true, &[&[(low, true)], &[(high, false)]]),
    ];
    draft.file(&faces)
}

/// A cone of radius 3 on the ground and height 4, its apex up: the conical
/// surface placed at the apex, its axis pointing down.
pub fn cone() -> String {
    let mut draft = Draft::new();
    let rim = draft.circle(DVec3::ZERO, 3.0);
    let apex = draft.placement(DVec3::Z * 4.0, DVec3::NEG_Z, DVec3::X);
    let wall = draft.add(format!(
        "CONICAL_SURFACE('',#{apex},0.,{:?})",
        0.75f64.atan()
    ));
    let ground = draft.placement(DVec3::ZERO, DVec3::Z, DVec3::X);
    let ground = draft.add(format!("PLANE('',#{ground})"));
    let faces = [
        draft.face(wall, true, &[&[(rim, true)]]),
        draft.face(ground, false, &[&[(rim, false)]]),
    ];
    draft.file(&faces)
}

/// The upper half of the ball of radius 2 about the origin.
pub fn hemisphere() -> String {
    let mut draft = Draft::new();
    let equator = draft.circle(DVec3::ZERO, 2.0);
    let center = draft.placement(DVec3::ZERO, DVec3::Z, DVec3::X);
    let dome = draft.add(format!("SPHERICAL_SURFACE('',#{center},2.)"));
    let ground = draft.add(format!("PLANE('',#{center})"));
    let faces = [
        draft.face(dome, true, &[&[(equator, true)]]),
        draft.face(ground, false, &[&[(equator, false)]]),
    ];
    draft.file(&faces)
}

/// The ball of radius 2 about (1, 1, 1): one face, bounded by nothing but a
/// vertex at its north pole.
pub fn ball() -> String {
    let mut draft = Draft::new();
    let pole = draft.vertex(DVec3::new(1.0, 1.0, 3.0));
    let center = draft.placement(DVec3::ONE, DVec3::Z, DVec3::X);
    let sphere = draft.add(format!("SPHERICAL_SURFACE('',#{center},2.)"));
    let around = draft.add(format!("VERTEX_LOOP('',#{pole})"));
    let bound = draft.add(format!("FACE_OUTER_BOUND('',#{around},.T.)"));
    let face = draft.add(format!("ADVANCED_FACE('',(#{bound}),#{sphere},.T.)"));
    draft.file(&[face])
}
