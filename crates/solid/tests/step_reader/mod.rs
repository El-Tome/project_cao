//! A STEP file read back from its text alone, the way another CAD program
//! reads it, and held to what makes it one exact solid: every reference
//! resolves, the shell is closed, every loop lies on its face's surface, and
//! the matter its faces enclose is measured from the entities themselves.

pub mod hand_written;
mod measure;
mod text;

mod geometry;

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use glam::DVec3;

use geometry::{Curve, Surface, at, boolean, list, number_of, reference};
use text::{Exchange, Value, references};

/// What the file holds, in its own length unit.
#[derive(Debug)]
pub struct Solid {
    pub faces: usize,
    /// The matter enclosed, from the divergence theorem over every face.
    pub volume: f64,
    pub area: f64,
    /// The furthest a point of any loop, or any vertex, stands from the
    /// surface of its face or the curve of its edge.
    pub worst: f64,
    /// The distance the file declares two points one under.
    pub uncertainty: f64,
    pub metres_per_unit: f64,
    /// The box the vertices span.
    pub low: DVec3,
    pub high: DVec3,
    /// Points along every B-spline edge, thirty-two to a span between knots.
    pub along_splines: Vec<Vec<DVec3>>,
    /// Points along every parabola and every hyperbola edge, 129 each.
    pub along_parabolas: Vec<Vec<DVec3>>,
    pub along_hyperbolas: Vec<Vec<DVec3>>,
}

pub struct Vertex {
    pub id: usize,
    pub point: DVec3,
}

pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
    pub curve: Curve,
    /// The parameters of the curve at the start and at the end, in the
    /// order the edge runs.
    pub from: f64,
    pub to: f64,
}

/// An edge of a loop, and whether the loop runs along it.
pub struct Use {
    pub edge: usize,
    pub along: bool,
}

pub enum Bound {
    Edges(Vec<Use>),
    Vertex(DVec3),
}

pub struct Face {
    pub id: usize,
    pub surface: Surface,
    /// Whether the face's normal is its surface's.
    pub same_sense: bool,
    pub bounds: Vec<Bound>,
}

pub fn read(text: &str) -> Result<Solid, String> {
    let exchange = text::parse(text)?;
    let schema = exchange
        .header
        .iter()
        .find(|record| record.name == "FILE_SCHEMA")
        .ok_or("the header names no schema")?;
    if !format!("{:?}", schema.values)
        .to_ascii_uppercase()
        .contains("AUTOMOTIVE_DESIGN")
    {
        return Err(format!(
            "the schema is {:?}, not AP214's AUTOMOTIVE_DESIGN",
            schema.values
        ));
    }
    unresolved(&exchange)?;
    let breps: Vec<usize> = exchange
        .instances
        .iter()
        .filter(|(_, records)| {
            records
                .iter()
                .any(|record| record.name == "MANIFOLD_SOLID_BREP")
        })
        .map(|(number, _)| *number)
        .collect();
    let [brep] = breps[..] else {
        return Err(format!(
            "one MANIFOLD_SOLID_BREP was expected, the file holds {breps:?}"
        ));
    };
    let (metres_per_unit, radians, uncertainty) = units(&exchange, brep)?;
    let shell = reference(at(exchange.simple(brep, &["MANIFOLD_SOLID_BREP"])?, 1)?)?;
    let shell = exchange.simple(shell, &["CLOSED_SHELL"])?;
    let mut edges = BTreeMap::new();
    let faces = list(at(shell, 1)?)?
        .iter()
        .map(|face| face_of(&exchange, reference(face)?, radians, &mut edges))
        .collect::<Result<Vec<_>, _>>()?;
    closed(&faces)?;
    let worst = worst(&faces, &edges);
    let (volume, area) = measure::enclosed(&faces, &edges)?;
    let corners = edges
        .values()
        .flat_map(|edge| [edge.start.point, edge.end.point])
        .chain(faces.iter().flat_map(|face| {
            face.bounds.iter().filter_map(|bound| match bound {
                Bound::Vertex(point) => Some(*point),
                Bound::Edges(_) => None,
            })
        }));
    let (low, high) = corners.fold(
        (DVec3::INFINITY, DVec3::NEG_INFINITY),
        |(low, high), corner| (low.min(corner), high.max(corner)),
    );
    let along = |kind: fn(&Curve) -> bool| -> Vec<Vec<DVec3>> {
        edges
            .values()
            .filter(|edge| kind(&edge.curve))
            .map(|edge| {
                let pieces = edge.curve.pieces(edge.from, edge.to);
                pieces
                    .windows(2)
                    .flat_map(|pair| [pair[0], (pair[0] + pair[1]) / 2.0])
                    .chain(pieces.last().copied())
                    .map(|t| edge.curve.point(t))
                    .collect()
            })
            .collect()
    };
    let along_splines = along(|curve| matches!(curve, Curve::Spline(_)));
    let along_parabolas = along(|curve| matches!(curve, Curve::Parabola { .. }));
    let along_hyperbolas = along(|curve| matches!(curve, Curve::Hyperbola { .. }));
    Ok(Solid {
        faces: faces.len(),
        volume,
        area,
        worst,
        uncertainty,
        metres_per_unit,
        low,
        high,
        along_splines,
        along_parabolas,
        along_hyperbolas,
    })
}

fn unresolved(exchange: &Exchange) -> Result<(), String> {
    let mut missing = Vec::new();
    for (number, records) in &exchange.instances {
        for record in records {
            let mut named = Vec::new();
            references(&record.values, &mut named);
            missing.extend(
                named
                    .into_iter()
                    .filter(|target| !exchange.instances.contains_key(target))
                    .map(|target| format!("#{number} names #{target}")),
            );
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("references to nothing: {}", missing.join(", ")))
    }
}

/// Metres per length unit, radians per angle unit, and the distance under
/// which two points are one, in the length unit — all from the context of the
/// representation that holds `brep`.
fn units(exchange: &Exchange, brep: usize) -> Result<(f64, f64, f64), String> {
    let holder = exchange
        .instances
        .values()
        .flatten()
        .find(|record| {
            record.name.ends_with("REPRESENTATION")
                && record
                    .values
                    .get(1)
                    .is_some_and(|items| *items == Value::List(vec![Value::Reference(brep)]))
        })
        .ok_or("no representation holds the solid alone")?;
    let context = reference(at(holder, 2)?)?;
    let assigned = exchange
        .part(context, "GLOBAL_UNIT_ASSIGNED_CONTEXT")
        .ok_or("the context assigns no units")?;
    let mut length = None;
    let mut angle = None;
    for unit in list(at(assigned, 0)?)? {
        let unit = reference(unit)?;
        let factor = factor(exchange, unit)?;
        if exchange.part(unit, "LENGTH_UNIT").is_some() {
            length = Some(factor);
        }
        if exchange.part(unit, "PLANE_ANGLE_UNIT").is_some() {
            angle = Some(factor);
        }
    }
    let length = length.ok_or("the context assigns no length unit")?;
    let angle = angle.ok_or("the context assigns no plane angle unit")?;
    let uncertainty = exchange
        .part(context, "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT")
        .ok_or("the context declares no uncertainty")?;
    let uncertainty = list(at(uncertainty, 0)?)?
        .iter()
        .map(|measure| {
            let measure =
                exchange.simple(reference(measure)?, &["UNCERTAINTY_MEASURE_WITH_UNIT"])?;
            let distance = number_of(at(measure, 0)?)?;
            let unit = factor(exchange, reference(at(measure, 1)?)?)?;
            Ok(if unit == length {
                distance
            } else {
                distance * unit / length
            })
        })
        .collect::<Result<Vec<f64>, String>>()?
        .into_iter()
        .fold(f64::NAN, f64::min);
    if uncertainty.is_nan() || uncertainty <= 0.0 {
        return Err("the context declares no positive length uncertainty".into());
    }
    Ok((length, angle, uncertainty))
}

/// How many metres, or radians, one of `unit` is.
fn factor(exchange: &Exchange, unit: usize) -> Result<f64, String> {
    if let Some(si) = exchange.part(unit, "SI_UNIT") {
        let prefix = match at(si, 0)? {
            Value::Unset => 1.0,
            Value::Enumeration(prefix) => match prefix.as_str() {
                "KILO" => 1e3,
                "CENTI" => 1e-2,
                "MILLI" => 1e-3,
                "MICRO" => 1e-6,
                "NANO" => 1e-9,
                other => return Err(format!("#{unit}: the prefix {other} is not read here")),
            },
            other => return Err(format!("#{unit}: {other:?} is no prefix")),
        };
        return Ok(prefix);
    }
    if let Some(converted) = exchange.part(unit, "CONVERSION_BASED_UNIT") {
        let measure = exchange.records(reference(at(converted, 1)?)?)?;
        let [measure] = measure else {
            return Err(format!("#{unit}: its measure is not one record"));
        };
        if !measure.name.ends_with("MEASURE_WITH_UNIT") {
            return Err(format!("#{unit}: {} is no measure", measure.name));
        }
        return Ok(number_of(at(measure, 0)?)? * factor(exchange, reference(at(measure, 1)?)?)?);
    }
    Err(format!(
        "#{unit} is neither an SI unit nor converted from one"
    ))
}

fn face_of(
    exchange: &Exchange,
    id: usize,
    radians: f64,
    edges: &mut BTreeMap<usize, Edge>,
) -> Result<Face, String> {
    let face = exchange.simple(id, &["ADVANCED_FACE"])?;
    let surface = exchange.surface(reference(at(face, 2)?)?, radians)?;
    let same_sense = boolean(at(face, 3)?)?;
    let mut bounds = Vec::new();
    for bound in list(at(face, 1)?)? {
        let bound = exchange.simple(reference(bound)?, &["FACE_OUTER_BOUND", "FACE_BOUND"])?;
        let kept = boolean(at(bound, 2)?)?;
        let loop_id = reference(at(bound, 1)?)?;
        let records = exchange.simple(loop_id, &["EDGE_LOOP", "VERTEX_LOOP"])?;
        if records.name == "VERTEX_LOOP" {
            bounds.push(Bound::Vertex(
                vertex(exchange, reference(at(records, 1)?)?)?.point,
            ));
            continue;
        }
        let mut uses = Vec::new();
        for oriented in list(at(records, 1)?)? {
            let oriented = exchange.simple(reference(oriented)?, &["ORIENTED_EDGE"])?;
            let edge = reference(at(oriented, 3)?)?;
            let forward = boolean(at(oriented, 4)?)?;
            if let Entry::Vacant(slot) = edges.entry(edge) {
                slot.insert(edge_of(exchange, edge)?);
            }
            uses.push(Use {
                edge,
                along: forward,
            });
        }
        for (index, one) in uses.iter().enumerate() {
            let next = &uses[(index + 1) % uses.len()];
            let ends = |using: &Use| {
                let edge = &edges[&using.edge];
                if using.along {
                    (edge.start.id, edge.end.id)
                } else {
                    (edge.end.id, edge.start.id)
                }
            };
            if ends(one).1 != ends(next).0 {
                return Err(format!(
                    "#{loop_id}: the edge #{} ends on #{}, the next one, #{}, starts on #{}",
                    one.edge,
                    ends(one).1,
                    next.edge,
                    ends(next).0
                ));
            }
        }
        if !kept {
            uses.reverse();
            for using in &mut uses {
                using.along = !using.along;
            }
        }
        bounds.push(Bound::Edges(uses));
    }
    if bounds.is_empty() {
        return Err(format!("#{id}: a face bounded by nothing"));
    }
    Ok(Face {
        id,
        surface,
        same_sense,
        bounds,
    })
}

fn vertex(exchange: &Exchange, id: usize) -> Result<Vertex, String> {
    let record = exchange.simple(id, &["VERTEX_POINT"])?;
    Ok(Vertex {
        id,
        point: exchange.point(reference(at(record, 1)?)?)?,
    })
}

fn edge_of(exchange: &Exchange, id: usize) -> Result<Edge, String> {
    let record = exchange.simple(id, &["EDGE_CURVE"])?;
    let start = vertex(exchange, reference(at(record, 1)?)?)?;
    let end = vertex(exchange, reference(at(record, 2)?)?)?;
    let curve = exchange.curve(reference(at(record, 3)?)?)?;
    let same_sense = boolean(at(record, 4)?)?;
    let mut from = curve.parameter(start.point);
    let mut to = curve.parameter(end.point);
    match (&curve, curve.period()) {
        (Curve::Spline(spline), Some(_)) if start.id == end.id => {
            let (low, high) = spline.range();
            (from, to) = if same_sense { (low, high) } else { (high, low) };
        }
        (_, Some(period)) if start.id == end.id => {
            to = if same_sense {
                from + period
            } else {
                from - period
            };
        }
        (_, Some(period)) => {
            while same_sense && to <= from {
                to += period;
            }
            while !same_sense && to >= from {
                to -= period;
            }
        }
        (_, None) if (to > from) != same_sense => {
            return Err(format!(
                "#{id} runs against the sense it claims on its curve"
            ));
        }
        (_, None) => {}
    }
    Ok(Edge {
        start,
        end,
        curve,
        from,
        to,
    })
}

/// Every edge used twice, once each way, by the loops of the shell.
fn closed(faces: &[Face]) -> Result<(), String> {
    let mut uses: BTreeMap<usize, Vec<(usize, bool)>> = BTreeMap::new();
    for face in faces {
        for bound in &face.bounds {
            if let Bound::Edges(edges) = bound {
                for using in edges {
                    uses.entry(using.edge)
                        .or_default()
                        .push((face.id, using.along));
                }
            }
        }
    }
    let open: Vec<String> = uses
        .iter()
        .filter(|(_, uses)| !matches!(uses[..], [(_, one), (_, other)] if one != other))
        .map(|(edge, uses)| format!("#{edge} used by {uses:?}"))
        .collect();
    if open.is_empty() {
        Ok(())
    } else {
        Err(format!("the shell is not closed: {}", open.join(", ")))
    }
}

/// The furthest any vertex stands from its edge's curve, and any point of
/// any loop from its face's surface.
fn worst(faces: &[Face], edges: &BTreeMap<usize, Edge>) -> f64 {
    let mut worst: f64 = 0.0;
    for edge in edges.values() {
        worst = worst
            .max(edge.curve.point(edge.from).distance(edge.start.point))
            .max(edge.curve.point(edge.to).distance(edge.end.point));
    }
    for face in faces {
        for bound in &face.bounds {
            match bound {
                Bound::Vertex(point) => worst = worst.max(face.surface.distance(*point)),
                Bound::Edges(uses) => {
                    for using in uses {
                        let edge = &edges[&using.edge];
                        for index in 0..=256 {
                            let t = edge.from + (edge.to - edge.from) * index as f64 / 256.0;
                            worst = worst.max(face.surface.distance(edge.curve.point(t)));
                        }
                    }
                }
            }
        }
    }
    worst
}
