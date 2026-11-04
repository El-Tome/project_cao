//! The numbered instances of a file's data section, its lengths scaled to
//! millimetres, and the file they make.

use glam::DVec3;

pub struct Instances {
    lines: Vec<String>,
    millimetres_per_unit: f64,
}

impl Instances {
    pub fn new(millimetres_per_unit: f64) -> Instances {
        Instances {
            lines: Vec::new(),
            millimetres_per_unit,
        }
    }

    pub fn add(&mut self, entity: String) -> usize {
        self.lines.push(entity);
        self.lines.len()
    }

    pub fn length(&self, value: f64) -> String {
        real(value * self.millimetres_per_unit)
    }

    pub fn point(&mut self, at: DVec3) -> usize {
        let [x, y, z] = at.to_array().map(|coordinate| self.length(coordinate));
        self.add(format!("CARTESIAN_POINT('',({x},{y},{z}))"))
    }

    pub fn direction(&mut self, along: DVec3) -> usize {
        let [x, y, z] = along.to_array().map(real);
        self.add(format!("DIRECTION('',({x},{y},{z}))"))
    }

    pub fn placement(&mut self, origin: DVec3, z: DVec3, x: DVec3) -> usize {
        let (origin, z, x) = (self.point(origin), self.direction(z), self.direction(x));
        self.add(format!("AXIS2_PLACEMENT_3D('',#{origin},#{z},#{x})"))
    }

    pub fn file(self) -> String {
        let data: String = self
            .lines
            .iter()
            .enumerate()
            .map(|(index, line)| format!("#{}={line};\n", index + 1))
            .collect();
        format!(
            "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\n\
             FILE_NAME('','',(''),(''),'','','');\n\
             FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));\nENDSEC;\nDATA;\n\
             {data}ENDSEC;\nEND-ISO-10303-21;\n"
        )
    }
}

/// A real as the standard writes one, its point never left out, and as
/// close as the number itself.
pub fn real(value: f64) -> String {
    let written = format!("{value:?}");
    match written.split_once('e') {
        Some((mantissa, exponent)) if mantissa.contains('.') => format!("{mantissa}E{exponent}"),
        Some((mantissa, exponent)) => format!("{mantissa}.E{exponent}"),
        None => written,
    }
}

pub fn flag(value: bool) -> &'static str {
    if value { ".T." } else { ".F." }
}
