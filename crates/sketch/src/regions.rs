use glam::DVec2;

use crate::naming::CurveId;
use crate::sketch::Sketch;

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
    /// The curves of the drawing this loop is bounded by, once each and in
    /// order — which is what names the area it encloses. `curves` above tells
    /// one *run* from the next within the loop; this says which curve of the
    /// drawing each run was cut out of, and outlives being cut again.
    pub bounds: Vec<CurveId>,
}

/// A closed area of the drawing, ready to be tinted.
///
/// `depth` is how many other areas enclose this one: an outline drawn inside
/// another is one level deeper, which is what lets them be told apart at a
/// glance instead of reading as a single blob.
#[derive(Clone, Debug)]
pub struct Region {
    pub outline: Outline,
    /// The outlines drawn directly inside this one. They are what a shape
    /// leaves hollow when it becomes a solid — the middle of a tube.
    pub holes: Vec<Outline>,
    pub depth: usize,
    pub triangles: Vec<[DVec2; 3]>,
}

impl Region {
    /// The area as a solid face: the outline with what sits inside it taken
    /// out. This is what an extrusion turns into matter, so that two circles
    /// one inside the other give a tube and not a rod.
    pub fn face_triangles(&self) -> Vec<[DVec2; 3]> {
        if self.holes.is_empty() {
            return self.triangles.clone();
        }
        triangulate(&bridge_holes(&self.outline.points, &self.holes))
    }

    /// The curves of the drawing that bound the area, which is its name.
    ///
    /// What it is cut out of and not what it leaves hollow: a hole is an area
    /// of its own, and bounds itself.
    pub fn bounds(&self) -> &[CurveId] {
        &self.outline.bounds
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
        let mut regions: Vec<Region> = self
            .closed_outlines()
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

        let insides: Vec<DVec2> = regions.iter().map(inside).collect();
        for (index, point) in insides.iter().enumerate() {
            regions[index].depth = regions
                .iter()
                .enumerate()
                .filter(|(other, region)| {
                    *other != index && encloses(&region.outline.points, *point)
                })
                .count();
        }
        regions.sort_by_key(|region| region.depth);

        // Only the outlines directly inside count as holes: what sits inside a
        // hole is matter again, and belongs to its own area.
        let outlines: Vec<(usize, Outline)> = regions
            .iter()
            .map(|region| (region.depth, region.outline.clone()))
            .collect();
        let insides: Vec<DVec2> = regions.iter().map(inside).collect();
        let holes: Vec<Vec<Outline>> = regions
            .iter()
            .enumerate()
            .map(|(index, region)| {
                outlines
                    .iter()
                    .enumerate()
                    .filter(|(other, (depth, _))| {
                        *other != index
                            && *depth == region.depth + 1
                            && encloses(&region.outline.points, insides[*other])
                    })
                    .map(|(_, (_, outline))| outline.clone())
                    .collect()
            })
            .collect();
        for (region, holes) in regions.iter_mut().zip(holes) {
            region.holes = holes;
        }
        regions
    }
}

/// A point inside the area and right up against its edge.
///
/// It has to hug the outline: a point taken well inside the shape would sit
/// inside whatever is drawn within it too, and every area would then count
/// itself as nested. The lowest corner is always a convex one, so stepping
/// just inside along its bisector lands in the area itself.
fn inside(region: &Region) -> DVec2 {
    let outline = &region.outline.points;
    let count = outline.len();
    // `total_cmp` rather than `partial_cmp`: a stray NaN would make the
    // comparison return None, and unwrapping it would take the whole
    // application down over one bad coordinate.
    let corner = (0..count)
        .min_by(|a, b| {
            outline[*a]
                .y
                .total_cmp(&outline[*b].y)
                .then(outline[*a].x.total_cmp(&outline[*b].x))
        })
        .unwrap_or(0);
    let (previous, here, following) = (
        outline[(corner + count - 1) % count],
        outline[corner],
        outline[(corner + 1) % count],
    );
    let bisector = ((previous - here).normalize_or_zero() + (following - here).normalize_or_zero())
        .normalize_or_zero();
    let reach = here.distance(previous).min(here.distance(following)) * 1e-3;
    here + bisector * reach
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
fn triangulate(outline: &[DVec2]) -> Vec<[DVec2; 3]> {
    if outline.len() < 3 || signed_area(outline).abs() < 1e-9 {
        return Vec::new();
    }
    let mut remaining: Vec<DVec2> = outline.to_vec();
    if signed_area(&remaining) < 0.0 {
        remaining.reverse();
    }

    let mut triangles = Vec::new();
    let mut guard = remaining.len() * remaining.len();
    while remaining.len() > 3 && guard > 0 {
        guard -= 1;
        let count = remaining.len();
        let mut clipped = false;
        for index in 0..count {
            let (a, b, c) = (
                remaining[(index + count - 1) % count],
                remaining[index],
                remaining[(index + 1) % count],
            );
            if (b - a).perp_dot(c - b) <= 0.0 {
                continue;
            }
            let clear = remaining
                .iter()
                .enumerate()
                .filter(|(other, _)| {
                    *other != index
                        && *other != (index + count - 1) % count
                        && *other != (index + 1) % count
                })
                // A corner of the ear itself, met a second time, is the seam of
                // a corridor cut out to a hole: it sits on the ear by
                // construction and must not be read as blocking it.
                .filter(|(_, point)| {
                    point.distance_squared(a) > EPSILON
                        && point.distance_squared(b) > EPSILON
                        && point.distance_squared(c) > EPSILON
                })
                .all(|(_, point)| !in_triangle(*point, a, b, c));
            if clear {
                triangles.push([a, b, c]);
                remaining.remove(index);
                clipped = true;
                break;
            }
        }
        if !clipped {
            break;
        }
    }
    if remaining.len() == 3 {
        triangles.push([remaining[0], remaining[1], remaining[2]]);
    }
    triangles
}

/// How close two positions have to be to count as the same corner.
const EPSILON: f64 = 1e-12;

fn in_triangle(point: DVec2, a: DVec2, b: DVec2, c: DVec2) -> bool {
    let side = |from: DVec2, to: DVec2| (to - from).perp_dot(point - from);
    side(a, b) >= 0.0 && side(b, c) >= 0.0 && side(c, a) >= 0.0
}

#[cfg(test)]
mod tests;
