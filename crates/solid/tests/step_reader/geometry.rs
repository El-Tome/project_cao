//! The curves and surfaces of ISO 10303-42 a part's file uses, read from their
//! instances and evaluated the way the standard parameterises them.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::DVec3;

use super::text::{Exchange, Record, Value};

impl Exchange {
    pub fn records(&self, number: usize) -> Result<&[Record], String> {
        self.instances
            .get(&number)
            .map(Vec::as_slice)
            .ok_or_else(|| format!("#{number} is not in the file"))
    }

    /// The one record of a simple instance, which must be named one of `names`.
    pub fn simple(&self, number: usize, names: &[&str]) -> Result<&Record, String> {
        match self.records(number)? {
            [record] if names.contains(&record.name.as_str()) => Ok(record),
            records => Err(format!(
                "#{number} should be one of {names:?}, it is {:?}",
                records
                    .iter()
                    .map(|record| &record.name)
                    .collect::<Vec<_>>()
            )),
        }
    }

    /// The record of an instance, simple or complex, named `name`.
    pub fn part(&self, number: usize, name: &str) -> Option<&Record> {
        self.instances
            .get(&number)?
            .iter()
            .find(|record| record.name == name)
    }

    pub fn point(&self, number: usize) -> Result<DVec3, String> {
        let record = self.simple(number, &["CARTESIAN_POINT"])?;
        triple(at(record, 1)?, number)
    }

    pub fn direction(&self, number: usize) -> Result<DVec3, String> {
        let record = self.simple(number, &["DIRECTION"])?;
        let direction = triple(at(record, 1)?, number)?;
        if direction.length() < 1e-12 {
            return Err(format!("#{number} is a direction of no length"));
        }
        Ok(direction.normalize())
    }

    pub fn placement(&self, number: usize) -> Result<Placement, String> {
        let record = self.simple(number, &["AXIS2_PLACEMENT_3D"])?;
        let origin = self.point(reference(at(record, 1)?)?)?;
        let z = match at(record, 2)? {
            Value::Unset => DVec3::Z,
            value => self.direction(reference(value)?)?,
        };
        let asked = match at(record, 3)? {
            Value::Unset => DVec3::X,
            value => self.direction(reference(value)?)?,
        };
        let x = asked - z * asked.dot(z);
        if x.length() < 1e-9 {
            return Err(format!(
                "#{number}: its reference direction lies along its axis"
            ));
        }
        let x = x.normalize();
        Ok(Placement {
            origin,
            x,
            y: z.cross(x),
            z,
        })
    }

    pub fn curve(&self, number: usize) -> Result<Curve, String> {
        if self.part(number, "B_SPLINE_CURVE_WITH_KNOTS").is_some() {
            return self.spline(number);
        }
        let record = self.simple(
            number,
            &[
                "LINE",
                "CIRCLE",
                "ELLIPSE",
                "PARABOLA",
                "HYPERBOLA",
                "SURFACE_CURVE",
                "SEAM_CURVE",
            ],
        )?;
        let placement = || self.placement(reference(at(record, 1)?)?);
        let length = |index| positive(at(record, index)?, number);
        Ok(match record.name.as_str() {
            "LINE" => {
                let origin = self.point(reference(at(record, 1)?)?)?;
                let vector = self.simple(reference(at(record, 2)?)?, &["VECTOR"])?;
                let along = self.direction(reference(at(vector, 1)?)?)?;
                Curve::Line {
                    origin,
                    along: along * positive(at(vector, 2)?, number)?,
                }
            }
            "CIRCLE" => Curve::Ellipse {
                placement: placement()?,
                first: length(2)?,
                second: length(2)?,
            },
            "ELLIPSE" => Curve::Ellipse {
                placement: placement()?,
                first: length(2)?,
                second: length(3)?,
            },
            "PARABOLA" => Curve::Parabola {
                placement: placement()?,
                focal: number_of(at(record, 2)?)?,
            },
            "HYPERBOLA" => Curve::Hyperbola {
                placement: placement()?,
                first: length(2)?,
                second: length(3)?,
            },
            _ => self.curve(reference(at(record, 1)?)?)?,
        })
    }

    fn spline(&self, number: usize) -> Result<Curve, String> {
        let knotted = self.part(number, "B_SPLINE_CURVE_WITH_KNOTS").unwrap();
        let (curve, offset, knot_values) = match self.part(number, "B_SPLINE_CURVE") {
            Some(curve) => (curve, 0, knotted.values.as_slice()),
            None => (knotted, 1, knotted.values.get(6..).unwrap_or_default()),
        };
        let degree = number_of(at(curve, offset)?)? as usize;
        let poles = list(at(curve, offset + 1)?)?
            .iter()
            .map(|pole| self.point(reference(pole)?))
            .collect::<Result<Vec<_>, _>>()?;
        let closed = boolean(at(curve, offset + 3)?)?;
        let [multiplicities, knots, ..] = knot_values else {
            return Err(format!("#{number}: a spline without its knots"));
        };
        let multiplicities = list(multiplicities)?;
        let knots = list(knots)?;
        if multiplicities.len() != knots.len() {
            return Err(format!(
                "#{number}: as many multiplicities as knots are needed"
            ));
        }
        let mut flat = Vec::new();
        for (multiplicity, knot) in multiplicities.iter().zip(knots) {
            let knot = number_of(knot)?;
            for _ in 0..number_of(multiplicity)? as usize {
                flat.push(knot);
            }
        }
        if flat.len() != poles.len() + degree + 1 || flat.windows(2).any(|pair| pair[1] < pair[0]) {
            return Err(format!(
                "#{number}: {} knots for {} poles of degree {degree}",
                flat.len(),
                poles.len()
            ));
        }
        let weights = match self.part(number, "RATIONAL_B_SPLINE_CURVE") {
            Some(rational) => list(at(rational, 0)?)?
                .iter()
                .map(number_of)
                .collect::<Result<Vec<_>, _>>()?,
            None => vec![1.0; poles.len()],
        };
        if weights.len() != poles.len() || weights.iter().any(|weight| *weight <= 0.0) {
            return Err(format!(
                "#{number}: a weight for each pole, every one positive"
            ));
        }
        Ok(Curve::Spline(Spline {
            degree,
            poles,
            weights,
            knots: flat,
            closed,
        }))
    }

    pub fn surface(&self, number: usize, radians: f64) -> Result<Surface, String> {
        let record = self.simple(
            number,
            &[
                "PLANE",
                "CYLINDRICAL_SURFACE",
                "CONICAL_SURFACE",
                "SURFACE_OF_LINEAR_EXTRUSION",
                "SPHERICAL_SURFACE",
                "TOROIDAL_SURFACE",
                "DEGENERATE_TOROIDAL_SURFACE",
            ],
        )?;
        let placement = || self.placement(reference(at(record, 1)?)?);
        let length = |index| positive(at(record, index)?, number);
        Ok(match record.name.as_str() {
            "PLANE" => Surface::Plane(placement()?),
            "CYLINDRICAL_SURFACE" => Surface::Cylinder {
                placement: placement()?,
                radius: length(2)?,
            },
            "CONICAL_SURFACE" => {
                let radius = number_of(at(record, 2)?)?;
                let angle = number_of(at(record, 3)?)? * radians;
                if radius < 0.0 || angle <= 0.0 || angle >= FRAC_PI_2 {
                    return Err(format!(
                        "#{number}: a cone wants a radius of zero or more and a half-angle \
                         strictly between 0 and a right angle, it has {radius} and {angle} rad"
                    ));
                }
                Surface::Cone {
                    placement: placement()?,
                    radius,
                    angle,
                }
            }
            "SURFACE_OF_LINEAR_EXTRUSION" => {
                let swept = self.curve(reference(at(record, 1)?)?)?;
                let Curve::Ellipse { .. } = swept else {
                    return Err(format!(
                        "#{number}: this reader sweeps circles and ellipses only"
                    ));
                };
                let vector = self.simple(reference(at(record, 2)?)?, &["VECTOR"])?;
                let along = self.direction(reference(at(vector, 1)?)?)?;
                Surface::Extrusion {
                    swept,
                    along: along * positive(at(vector, 2)?, number)?,
                }
            }
            "SPHERICAL_SURFACE" => Surface::Sphere {
                placement: placement()?,
                radius: length(2)?,
            },
            "TOROIDAL_SURFACE" => Surface::Torus {
                placement: placement()?,
                major: length(2)?,
                minor: length(3)?,
                outer: None,
            },
            _ => {
                let (major, minor) = (length(2)?, length(3)?);
                if major >= minor {
                    return Err(format!(
                        "#{number}: a degenerate torus has its major radius below its minor"
                    ));
                }
                Surface::Torus {
                    placement: placement()?,
                    major,
                    minor,
                    outer: Some(boolean(at(record, 4)?)?),
                }
            }
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Placement {
    pub origin: DVec3,
    pub x: DVec3,
    pub y: DVec3,
    pub z: DVec3,
}

impl Placement {
    fn local(&self, point: DVec3) -> DVec3 {
        let offset = point - self.origin;
        DVec3::new(offset.dot(self.x), offset.dot(self.y), offset.dot(self.z))
    }

    fn around(&self, angle: f64) -> DVec3 {
        self.x * angle.cos() + self.y * angle.sin()
    }

    fn across(&self, angle: f64) -> DVec3 {
        self.y * angle.cos() - self.x * angle.sin()
    }
}

#[derive(Clone, Debug)]
pub struct Spline {
    degree: usize,
    poles: Vec<DVec3>,
    weights: Vec<f64>,
    knots: Vec<f64>,
    closed: bool,
}

impl Spline {
    pub fn range(&self) -> (f64, f64) {
        (
            self.knots[self.degree],
            self.knots[self.knots.len() - self.degree - 1],
        )
    }

    fn point(&self, t: f64) -> DVec3 {
        let (low, high) = self.range();
        let t = t.clamp(low, high);
        let last = self.poles.len() - 1;
        let span =
            (self.knots[..=last].partition_point(|knot| *knot <= t) - 1).clamp(self.degree, last);
        let span = (self.degree..=span)
            .rev()
            .find(|&span| self.knots[span] < self.knots[span + 1])
            .unwrap_or(self.degree);
        let mut points: Vec<(DVec3, f64)> = (0..=self.degree)
            .map(|index| {
                let pole = span - self.degree + index;
                (self.poles[pole] * self.weights[pole], self.weights[pole])
            })
            .collect();
        for level in 1..=self.degree {
            for index in (level..=self.degree).rev() {
                let pole = span - self.degree + index;
                let below = self.knots[pole];
                let above = self.knots[pole + self.degree + 1 - level];
                let share = if above > below {
                    (t - below) / (above - below)
                } else {
                    0.0
                };
                let (one, other) = (points[index - 1], points[index]);
                points[index] = (
                    one.0 * (1.0 - share) + other.0 * share,
                    one.1 * (1.0 - share) + other.1 * share,
                );
            }
        }
        let (point, weight) = points[self.degree];
        point / weight
    }

    /// The knots inside the range, ends included, where the curve may bend
    /// at a seam of its polynomials.
    fn breaks(&self) -> Vec<f64> {
        let (low, high) = self.range();
        let mut breaks: Vec<f64> = self
            .knots
            .iter()
            .copied()
            .filter(|knot| (low..=high).contains(knot))
            .collect();
        breaks.dedup();
        breaks
    }
}

#[derive(Clone, Debug)]
pub enum Curve {
    Line {
        origin: DVec3,
        along: DVec3,
    },
    /// A circle is an ellipse of two equal semi-axes.
    Ellipse {
        placement: Placement,
        first: f64,
        second: f64,
    },
    Parabola {
        placement: Placement,
        focal: f64,
    },
    Hyperbola {
        placement: Placement,
        first: f64,
        second: f64,
    },
    Spline(Spline),
}

impl Curve {
    pub fn point(&self, t: f64) -> DVec3 {
        match self {
            Curve::Line { origin, along } => *origin + *along * t,
            Curve::Ellipse {
                placement,
                first,
                second,
            } => placement.origin + placement.x * first * t.cos() + placement.y * second * t.sin(),
            Curve::Parabola { placement, focal } => {
                placement.origin + (placement.x * t * t + placement.y * 2.0 * t) * *focal
            }
            Curve::Hyperbola {
                placement,
                first,
                second,
            } => {
                placement.origin + placement.x * first * t.cosh() + placement.y * second * t.sinh()
            }
            Curve::Spline(spline) => spline.point(t),
        }
    }

    pub fn period(&self) -> Option<f64> {
        match self {
            Curve::Ellipse { .. } => Some(TAU),
            Curve::Spline(spline) if spline.closed => {
                let (low, high) = spline.range();
                Some(high - low)
            }
            _ => None,
        }
    }

    /// The parameter of the point of the curve nearest `point`.
    pub fn parameter(&self, point: DVec3) -> f64 {
        match self {
            Curve::Line { origin, along } => (point - *origin).dot(*along) / along.length_squared(),
            Curve::Ellipse {
                placement,
                first,
                second,
            } => {
                let local = placement.local(point);
                (local.y / second).atan2(local.x / first)
            }
            Curve::Parabola { placement, focal } => placement.local(point).y / (2.0 * focal),
            Curve::Hyperbola {
                placement, second, ..
            } => (placement.local(point).y / second).asinh(),
            Curve::Spline(spline) => {
                let (low, high) = spline.range();
                let samples = 4096;
                let best = (0..=samples)
                    .map(|index| low + (high - low) * index as f64 / samples as f64)
                    .min_by(|one, other| {
                        spline
                            .point(*one)
                            .distance(point)
                            .total_cmp(&spline.point(*other).distance(point))
                    })
                    .unwrap();
                let step = (high - low) / samples as f64;
                nearest(|t| spline.point(t).distance(point), best, step).clamp(low, high)
            }
        }
    }

    /// Where the parameter range between `from` and `to` is cut for
    /// quadrature: each span between a spline's knots in two, any other
    /// curve in sixty-four.
    pub fn pieces(&self, from: f64, to: f64) -> Vec<f64> {
        let Curve::Spline(spline) = self else {
            return (0..=64)
                .map(|index| from + (to - from) * index as f64 / 64.0)
                .collect();
        };
        let (low, high) = (from.min(to), from.max(to));
        let mut cuts = vec![from];
        let mut inner: Vec<f64> = spline
            .breaks()
            .into_iter()
            .filter(|knot| *knot > low && *knot < high)
            .collect();
        if to < from {
            inner.reverse();
        }
        cuts.extend(inner);
        cuts.push(to);
        let mut pieces = vec![from];
        for pair in cuts.windows(2) {
            for index in 1..=2 {
                pieces.push(pair[0] + (pair[1] - pair[0]) * index as f64 / 2.0);
            }
        }
        pieces
    }
}

/// A local minimum of `distance` near `guess`, searched over `reach` on each
/// side by a golden section after a coarse scan.
fn nearest(distance: impl Fn(f64) -> f64, guess: f64, reach: f64) -> f64 {
    let samples = 256;
    let step = 2.0 * reach / samples as f64;
    let best = (0..=samples)
        .map(|index| guess - reach + step * index as f64)
        .min_by(|one, other| distance(*one).total_cmp(&distance(*other)))
        .unwrap();
    let (mut low, mut high) = (best - step, best + step);
    let golden = (5f64.sqrt() - 1.0) / 2.0;
    for _ in 0..200 {
        let one = high - golden * (high - low);
        let other = low + golden * (high - low);
        if distance(one) < distance(other) {
            high = other;
        } else {
            low = one;
        }
    }
    (low + high) / 2.0
}

#[derive(Clone, Debug)]
pub enum Surface {
    Plane(Placement),
    Cylinder {
        placement: Placement,
        radius: f64,
    },
    Cone {
        placement: Placement,
        radius: f64,
        angle: f64,
    },
    /// Only a circle or an ellipse is swept.
    Extrusion {
        swept: Curve,
        along: DVec3,
    },
    Sphere {
        placement: Placement,
        radius: f64,
    },
    /// `outer` is `Some` on a degenerate torus, its major radius below its
    /// minor: whether the part selected is the outer one, `R + r cos v ≥ 0`,
    /// or the inner, `R + r cos v ≤ 0`, the one parametrisation of
    /// ISO 10303-42 giving both the point and the normal `S_u × S_v`.
    Torus {
        placement: Placement,
        major: f64,
        minor: f64,
        outer: Option<bool>,
    },
}

/// A point of a surface and its two partial derivatives, `u` then `v`.
pub struct Local {
    pub point: DVec3,
    pub along_u: DVec3,
    pub along_v: DVec3,
}

impl Surface {
    pub fn at(&self, u: f64, v: f64) -> Local {
        match self {
            Surface::Plane(placement) => Local {
                point: placement.origin + placement.x * u + placement.y * v,
                along_u: placement.x,
                along_v: placement.y,
            },
            Surface::Cylinder { placement, radius } => Local {
                point: placement.origin + placement.around(u) * *radius + placement.z * v,
                along_u: placement.across(u) * *radius,
                along_v: placement.z,
            },
            Surface::Cone {
                placement,
                radius,
                angle,
            } => {
                let reach = radius + v * angle.tan();
                Local {
                    point: placement.origin + placement.around(u) * reach + placement.z * v,
                    along_u: placement.across(u) * reach,
                    along_v: placement.around(u) * angle.tan() + placement.z,
                }
            }
            Surface::Extrusion { swept, along } => {
                let Curve::Ellipse {
                    placement,
                    first,
                    second,
                } = swept
                else {
                    unreachable!("only ellipses are swept")
                };
                Local {
                    point: swept.point(u) + *along * v,
                    along_u: placement.y * second * u.cos() - placement.x * first * u.sin(),
                    along_v: *along,
                }
            }
            Surface::Sphere { placement, radius } => Local {
                point: placement.origin
                    + (placement.around(u) * v.cos() + placement.z * v.sin()) * *radius,
                along_u: placement.across(u) * v.cos() * *radius,
                along_v: (placement.z * v.cos() - placement.around(u) * v.sin()) * *radius,
            },
            Surface::Torus {
                placement,
                major,
                minor,
                ..
            } => Local {
                point: placement.origin
                    + placement.around(u) * (major + minor * v.cos())
                    + placement.z * minor * v.sin(),
                along_u: placement.across(u) * (major + minor * v.cos()),
                along_v: (placement.z * v.cos() - placement.around(u) * v.sin()) * *minor,
            },
        }
    }

    /// The parameters of a point on the surface, `v` of a sphere or a torus
    /// in `(-π, π]`.
    pub fn parameters(&self, point: DVec3) -> (f64, f64) {
        match self {
            Surface::Plane(placement) => {
                let local = placement.local(point);
                (local.x, local.y)
            }
            Surface::Cylinder { placement, .. } => {
                let local = placement.local(point);
                (local.y.atan2(local.x), local.z)
            }
            Surface::Cone {
                placement,
                radius,
                angle,
            } => {
                let local = placement.local(point);
                let turn = local.y.atan2(local.x);
                let turn = if radius + local.z * angle.tan() < 0.0 {
                    turn + PI
                } else {
                    turn
                };
                (turn, local.z)
            }
            Surface::Extrusion { swept, along } => {
                let Curve::Ellipse {
                    placement,
                    first,
                    second,
                } = swept
                else {
                    unreachable!("only ellipses are swept")
                };
                let height = (point - placement.origin).dot(placement.z) / along.dot(placement.z);
                let local = placement.local(point - *along * height);
                ((local.y / second).atan2(local.x / first), height)
            }
            Surface::Sphere { placement, .. } => {
                let local = placement.local(point);
                (
                    local.y.atan2(local.x),
                    local.z.atan2(local.truncate().length()),
                )
            }
            Surface::Torus {
                placement,
                major,
                outer: Some(false),
                ..
            } => {
                let local = placement.local(point);
                let across = local.truncate().length();
                (
                    local.y.atan2(local.x) + PI,
                    local.z.atan2(-across - major).rem_euclid(TAU),
                )
            }
            Surface::Torus {
                placement, major, ..
            } => {
                let local = placement.local(point);
                let across = local.truncate().length();
                (local.y.atan2(local.x), local.z.atan2(across - major))
            }
        }
    }

    pub fn distance(&self, point: DVec3) -> f64 {
        match self {
            Surface::Plane(placement) => placement.local(point).z.abs(),
            Surface::Cylinder { placement, radius } => {
                (placement.local(point).truncate().length() - radius).abs()
            }
            Surface::Cone {
                placement,
                radius,
                angle,
            } => {
                let local = placement.local(point);
                let across = local.truncate().length();
                let off =
                    |across: f64| ((across - radius) * angle.cos() - local.z * angle.sin()).abs();
                off(across).min(off(-across))
            }
            Surface::Extrusion { along, .. } => {
                let (u, v) = self.parameters(point);
                let unit = along.normalize();
                let off = |u: f64| {
                    let offset = point - self.at(u, v).point;
                    (offset - unit * offset.dot(unit)).length()
                };
                off(nearest(off, u, 0.5))
            }
            Surface::Sphere { placement, radius } => {
                (point.distance(placement.origin) - radius).abs()
            }
            Surface::Torus {
                placement,
                major,
                minor,
                ..
            } => {
                let local = placement.local(point);
                let across = local.truncate().length();
                let off = |center: f64| ((across - center).hypot(local.z) - minor).abs();
                off(*major).min(off(-major))
            }
        }
    }

    /// Whether `u` turns: every surface but the plane.
    pub fn turns(&self) -> bool {
        !matches!(self, Surface::Plane(_))
    }

    /// Where the surface folds to a point, as the parameter `v` there, the
    /// lower first: a sphere's two poles, a spindle's two apexes — about
    /// `v = 0` on the outer part of a degenerate torus, about `v = π` on the
    /// inner. A cone's apex is where its integrals start, and needs no fold.
    pub fn poles(&self) -> Vec<f64> {
        match self {
            Surface::Sphere { .. } => vec![-FRAC_PI_2, FRAC_PI_2],
            Surface::Torus {
                major,
                minor,
                outer: Some(outer),
                ..
            } => {
                let apex = (-major / minor).acos();
                if *outer {
                    vec![-apex, apex]
                } else {
                    vec![apex, TAU - apex]
                }
            }
            _ => Vec::new(),
        }
    }

    pub fn pole_point(&self, v: f64) -> DVec3 {
        self.at(0.0, v).point
    }

    /// How wide a piece of `v` one quadrature takes exactly, or nearly.
    pub fn span(&self) -> f64 {
        match self {
            Surface::Sphere { .. } | Surface::Torus { .. } => PI / 32.0,
            _ => f64::INFINITY,
        }
    }
}

pub fn at(record: &Record, index: usize) -> Result<&Value, String> {
    record
        .values
        .get(index)
        .ok_or_else(|| format!("{} has no parameter {index}", record.name))
}

pub fn reference(value: &Value) -> Result<usize, String> {
    match value {
        Value::Reference(number) => Ok(*number),
        other => Err(format!("a reference was expected, not {other:?}")),
    }
}

pub fn number_of(value: &Value) -> Result<f64, String> {
    match value {
        Value::Number(number) => Ok(*number),
        Value::Typed(_, inner) if inner.len() == 1 => number_of(&inner[0]),
        other => Err(format!("a number was expected, not {other:?}")),
    }
}

pub fn list(value: &Value) -> Result<&[Value], String> {
    match value {
        Value::List(values) => Ok(values),
        other => Err(format!("a list was expected, not {other:?}")),
    }
}

pub fn boolean(value: &Value) -> Result<bool, String> {
    match value {
        Value::Enumeration(name) if name == "T" => Ok(true),
        Value::Enumeration(name) if name == "F" => Ok(false),
        other => Err(format!(".T. or .F. was expected, not {other:?}")),
    }
}

fn positive(value: &Value, number: usize) -> Result<f64, String> {
    let length = number_of(value)?;
    if length > 0.0 && length.is_finite() {
        Ok(length)
    } else {
        Err(format!(
            "#{number}: a positive length was expected, not {length}"
        ))
    }
}

fn triple(value: &Value, number: usize) -> Result<DVec3, String> {
    match list(value)? {
        [x, y, z] => Ok(DVec3::new(number_of(x)?, number_of(y)?, number_of(z)?)),
        other => Err(format!("#{number}: three coordinates, not {}", other.len())),
    }
}
