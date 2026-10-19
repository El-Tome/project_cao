use glam::DVec2;

use crate::arc_regions::Walked;
use crate::edges::half_edge::Bend;
use crate::naming::CurveId;
use crate::sketch::Sketch;
use ears::clipped_ears;
pub use runs::Leg;

/// One closed loop of an area, and what drew each of its segments.
///
/// Segment `index` runs from `points[index]` to the point after it, and
/// `curves[index]` names the curve it was sampled from, `None` for a trait
/// drawn straight. Segments carrying the same number came from one curve.
///
/// The points alone do not say where one curve ends and the next begins, and a
/// wall raised per sampled point would cut a circle's wall into as many faces
/// as the circle was sampled into.
#[derive(Clone, Debug, Default)]
pub struct Outline {
    pub points: Vec<DVec2>,
    pub curves: Vec<Option<usize>>,
    /// The borders of this loop, once each and in order, each as the curves
    /// of the drawing running along it — one, or every curve laid along that
    /// stretch. That is what names the area it encloses. `curves` above tells
    /// one *run* from the next within the loop; this says which curves of the
    /// drawing each run was cut out of, and outlives being cut again.
    pub bounds: Vec<Vec<CurveId>>,
    /// What each run actually curves along, indexed by the run number in
    /// `curves`.
    ///
    /// The sampled points say where a curve went, never what it was: a circle
    /// read off its own steps comes out a little small and a little short, and
    /// a measuring tool that answers 313.9 mm² for a circle of radius ten is
    /// one nobody trusts twice. Tinting and extruding want the steps; measuring
    /// wants the curve, and this is where the walk leaves it.
    pub(crate) bends: Vec<Bend>,
}

/// A closed area of the drawing, ready to be tinted.
///
/// `depth` is how many other areas enclose this one: an outline drawn inside
/// another is one level deeper, which is what lets them be told apart at a
/// glance instead of reading as a single blob.
#[derive(Clone, Debug)]
pub struct Region {
    pub outline: Outline,
    /// The outlines drawn directly inside this one, or the outside several of
    /// them make together where they touch. They are what a shape leaves
    /// hollow when it becomes a solid — the middle of a tube, or one opening
    /// for two windows touching.
    pub holes: Vec<Outline>,
    pub depth: usize,
    pub triangles: Vec<[DVec2; 3]>,
}

impl Region {
    /// The area as a solid face: the outline with what sits inside it taken
    /// out. This is what an extrusion turns into matter, so that two circles
    /// one inside the other give a tube and not a rod.
    ///
    /// The corridors bridged out to the holes ask the holes to stand clear of
    /// the outline, and a hole touching it — a circle tangent inside a
    /// rectangle — can lose its splice or wedge the clipper, either of which
    /// leaves the matter wrong. The result is therefore held to what the
    /// outline and holes measure, and a face that fails it is cut again by
    /// the winding sweep, which touching loops cannot upset.
    pub fn face_triangles(&self) -> Vec<[DVec2; 3]> {
        if self.holes.is_empty() {
            return self.triangles.clone();
        }
        let hollowed = signed_area(&self.outline.points).abs()
            - self
                .holes
                .iter()
                .map(|hole| signed_area(&hole.points).abs())
                .sum::<f64>();
        let bridged = triangulate(&bridge_holes(&self.outline.points, &self.holes));
        if covers(&bridged, hollowed) {
            return bridged;
        }
        let mut shell = self.outline.points.clone();
        if signed_area(&shell) < 0.0 {
            shell.reverse();
        }
        let turned: Vec<Vec<DVec2>> = self
            .holes
            .iter()
            .map(|hole| clockwise(&hole.points))
            .collect();
        let mut loops: Vec<&[DVec2]> = vec![&shell];
        loops.extend(turned.iter().map(|hole| hole.as_slice()));
        swept::swept(&loops)
    }

    /// The borders of the area, each as the curves of the drawing running
    /// along it, which is its name.
    ///
    /// What it is cut out of and not what it leaves hollow: a hole is an area
    /// of its own, and bounds itself.
    pub fn bounds(&self) -> &[Vec<CurveId>] {
        &self.outline.bounds
    }

    /// Whether a curve of the drawing runs along some border of the area.
    pub(crate) fn runs_along(&self, curve: CurveId) -> bool {
        self.outline
            .bounds
            .iter()
            .any(|border| border.contains(&curve))
    }

    /// Whether the point is in the area itself, holes excluded.
    pub fn contains(&self, point: DVec2) -> bool {
        encloses(&self.outline.points, point)
            && !self.holes.iter().any(|hole| encloses(&hole.points, point))
    }
}

impl Sketch {
    /// Every closed area the drawing encloses, innermost last.
    ///
    /// Contours are found the way a map finds its countries: walk each edge
    /// always turning as tightly as possible, and the walk comes back on itself
    /// around exactly one area. Counting segments could not do this — the same
    /// segment belongs to two areas when two shapes share a side.
    pub fn regions(&self) -> Vec<Region> {
        let Walked { areas, outsides } = self.closed_outlines();
        let mut regions: Vec<Region> = areas
            .into_iter()
            .filter_map(|outline| {
                let triangles = triangulate(&outline.points);
                (!triangles.is_empty()).then_some(Region {
                    outline,
                    holes: Vec::new(),
                    depth: 0,
                    triangles,
                })
            })
            .collect();

        hollow::nest(&mut regions, outsides);
        regions.sort_by_key(|region| region.depth);
        regions
    }
}

/// Whether one outline lies inside another, read off the corner of it that
/// stands furthest from the other.
///
/// Any point of the outline would do were the two apart, but they may touch:
/// two circles one inside the other meeting at a point are two whole loops, and
/// a place picked near the touch can fall on either side of the other's
/// sampled outline. The corner furthest away is the one rounding cannot move
/// across. An outline lying wholly along the other — a shape pinched where it
/// touches its own outline — has no such corner, and is not inside it.
fn lies_within(inner: &[DVec2], outer: &[DVec2]) -> bool {
    let (Some((inner_low, inner_high)), Some((outer_low, outer_high))) =
        (bounds(inner), bounds(outer))
    else {
        return false;
    };
    // Overlapping boxes, not one box inside the other: where the two touch,
    // each is sampled its own way, and the inner one may stand a little out.
    if inner_low.cmpgt(outer_high).any() || inner_high.cmplt(outer_low).any() {
        return false;
    }
    let tolerance = (outer_high - outer_low).length() * 1e-9;
    inner
        .iter()
        .map(|corner| (*corner, distance_to_outline(outer, *corner)))
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .is_some_and(|(corner, clear)| clear > tolerance && encloses(outer, corner))
}

fn bounds(outline: &[DVec2]) -> Option<(DVec2, DVec2)> {
    let first = *outline.first()?;
    Some(outline.iter().fold((first, first), |(low, high), point| {
        (low.min(*point), high.max(*point))
    }))
}

fn distance_to_outline(outline: &[DVec2], point: DVec2) -> f64 {
    (0..outline.len())
        .map(|index| {
            let (a, b) = (outline[index], outline[(index + 1) % outline.len()]);
            let along = (point - a).dot(b - a) / (b - a).length_squared().max(f64::MIN_POSITIVE);
            point.distance(a + (b - a) * along.clamp(0.0, 1.0))
        })
        .fold(f64::INFINITY, f64::min)
}

pub(crate) fn signed_area(outline: &[DVec2]) -> f64 {
    let mut total = 0.0;
    for index in 0..outline.len() {
        let current = outline[index];
        let following = outline[(index + 1) % outline.len()];
        total += current.perp_dot(following);
    }
    total * 0.5
}

fn encloses(outline: &[DVec2], point: DVec2) -> bool {
    let mut inside = false;
    for index in 0..outline.len() {
        let a = outline[index];
        let b = outline[(index + 1) % outline.len()];
        if (a.y > point.y) != (b.y > point.y) {
            let crossing = a.x + (point.y - a.y) / (b.y - a.y) * (b.x - a.x);
            if crossing > point.x {
                inside = !inside;
            }
        }
    }
    inside
}

/// Splices every hole into the outline so one closed path describes the face.
///
/// A ring cannot be cut into triangles as it stands: there is no way round it
/// that does not either leave the hole filled or leave the outline open. The
/// classic answer is to cut a corridor from the hole out to the outline and
/// walk down one side and back up the other — the two sides lie on top of each
/// other, so the corridor has no area and the face is unchanged.
fn bridge_holes(outline: &[DVec2], holes: &[Outline]) -> Vec<DVec2> {
    let mut path = counter_clockwise(outline);
    // Rightmost first: a hole further right can only ever bridge to the outline
    // or to a hole already spliced in, never to one still waiting.
    let mut pending: Vec<Vec<DVec2>> = holes.iter().map(|hole| clockwise(&hole.points)).collect();
    pending.sort_by(|a, b| rightmost(b).x.total_cmp(&rightmost(a).x));

    for hole in pending {
        let Some(path_with_hole) = splice(&path, &hole) else {
            continue;
        };
        path = path_with_hole;
    }
    path
}

fn splice(path: &[DVec2], hole: &[DVec2]) -> Option<Vec<DVec2>> {
    let entry = hole
        .iter()
        .copied()
        .enumerate()
        .max_by(|a, b| a.1.x.total_cmp(&b.1.x))?;
    let (entry_index, entry_point) = entry;

    // The corridor goes to whichever corner of the path is both to the right of
    // the hole and closest to it: going left would cross the hole itself.
    let exit = (0..path.len())
        .filter(|index| path[*index].x >= entry_point.x)
        .min_by(|a, b| {
            path[*a]
                .distance_squared(entry_point)
                .total_cmp(&path[*b].distance_squared(entry_point))
        })
        .or_else(|| {
            (0..path.len()).min_by(|a, b| {
                path[*a]
                    .distance_squared(entry_point)
                    .total_cmp(&path[*b].distance_squared(entry_point))
            })
        })?;

    let mut spliced: Vec<DVec2> = path[..=exit].to_vec();
    for step in 0..hole.len() {
        spliced.push(hole[(entry_index + step) % hole.len()]);
    }
    spliced.push(entry_point);
    spliced.extend_from_slice(&path[exit..]);
    Some(spliced)
}

fn rightmost(loop_points: &[DVec2]) -> DVec2 {
    loop_points
        .iter()
        .copied()
        .fold(DVec2::new(f64::MIN, 0.0), |best, point| {
            if point.x > best.x { point } else { best }
        })
}

fn counter_clockwise(loop_points: &[DVec2]) -> Vec<DVec2> {
    let mut points = loop_points.to_vec();
    if signed_area(&points) < 0.0 {
        points.reverse();
    }
    points
}

/// A hole runs the opposite way round to the face it is cut out of, so that
/// walking the spliced path keeps the matter on the same side throughout.
fn clockwise(loop_points: &[DVec2]) -> Vec<DVec2> {
    let mut points = loop_points.to_vec();
    if signed_area(&points) > 0.0 {
        points.reverse();
    }
    points
}

/// Cuts a closed outline into triangles by clipping ears: repeatedly take a
/// corner no other corner sits in, and snip it off.
///
/// An outline pinched where two curves touch runs through one corner twice,
/// and near the touch its sampled walls can cross each other by less than
/// their own sampling; against some orderings of such an outline the clipper
/// wedges itself and leaves a bite of the face uncut. What comes out is
/// therefore held to what the outline measures, and an outline that fails it
/// is cut again by the winding sweep.
fn triangulate(outline: &[DVec2]) -> Vec<[DVec2; 3]> {
    if outline.len() < 3 || signed_area(outline).abs() < 1e-9 {
        return Vec::new();
    }
    let mut remaining: Vec<DVec2> = outline.to_vec();
    if signed_area(&remaining) < 0.0 {
        remaining.reverse();
    }
    let triangles = clipped_ears(&remaining);
    match covers(&triangles, signed_area(&remaining)) {
        true => triangles,
        false => swept::swept(&[&remaining]),
    }
}

/// Whether the triangles cover what the face measures, to rounding.
fn covers(triangles: &[[DVec2; 3]], measure: f64) -> bool {
    let raised: f64 = triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).perp_dot(*c - *a).abs() * 0.5)
        .sum();
    (raised - measure).abs() <= 1e-9 * measure.abs().max(1.0)
}

mod across;
mod ears;
mod hollow;
mod measure;
mod runs;
mod side;
mod swept;

#[cfg(test)]
mod tests;
