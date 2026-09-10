use glam::DVec2;

use crate::sketch::Sketch;

/// A closed area of the drawing, ready to be tinted.
///
/// `depth` is how many other areas enclose this one: an outline drawn inside
/// another is one level deeper, which is what lets them be told apart at a
/// glance instead of reading as a single blob.
#[derive(Clone, Debug)]
pub struct Region {
    pub outline: Vec<DVec2>,
    /// The outlines drawn directly inside this one. They are what a shape
    /// leaves hollow when it becomes a solid — the middle of a tube.
    pub holes: Vec<Vec<DVec2>>,
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
        triangulate(&bridge_holes(&self.outline, &self.holes))
    }

    /// Whether the point is in the area itself, holes excluded.
    pub fn contains(&self, point: DVec2) -> bool {
        encloses(&self.outline, point) && !self.holes.iter().any(|hole| encloses(hole, point))
    }
}

/// How finely a circle is cut up when it is treated as a closed area.
const CIRCLE_STEPS: usize = 48;

impl Sketch {
    /// Every closed area the drawing encloses, innermost last.
    ///
    /// Contours are found the way a map finds its countries: walk each edge
    /// always turning as tightly as possible, and the walk comes back on itself
    /// around exactly one area. Counting segments could not do this — the same
    /// segment belongs to two areas when two shapes share a side.
    pub fn regions(&self) -> Vec<Region> {
        let mut outlines = self.closed_outlines();
        outlines.extend(self.live_circles().map(|(_, circle)| {
            let center = self.point(circle.center);
            (0..CIRCLE_STEPS)
                .map(|step| {
                    let angle = std::f64::consts::TAU * step as f64 / CIRCLE_STEPS as f64;
                    center + DVec2::from_angle(angle) * circle.radius
                })
                .collect()
        }));

        let mut regions: Vec<Region> = outlines
            .into_iter()
            .filter_map(|outline| {
                let triangles = triangulate(&outline);
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
                .filter(|(other, region)| *other != index && encloses(&region.outline, *point))
                .count();
        }
        regions.sort_by_key(|region| region.depth);

        // Only the outlines directly inside count as holes: what sits inside a
        // hole is matter again, and belongs to its own area.
        let outlines: Vec<(usize, Vec<DVec2>)> = regions
            .iter()
            .map(|region| (region.depth, region.outline.clone()))
            .collect();
        let insides: Vec<DVec2> = regions.iter().map(inside).collect();
        let holes: Vec<Vec<Vec<DVec2>>> = regions
            .iter()
            .enumerate()
            .map(|(index, region)| {
                outlines
                    .iter()
                    .enumerate()
                    .filter(|(other, (depth, _))| {
                        *other != index
                            && *depth == region.depth + 1
                            && encloses(&region.outline, insides[*other])
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

    /// Walks the segment graph and returns each area it encloses, as a loop of
    /// positions turning counter-clockwise.
    fn closed_outlines(&self) -> Vec<Vec<DVec2>> {
        // Only what is still drawn: a deleted side must not close an area that
        // is no longer there.
        let ends: Vec<(usize, usize)> = self
            .live_segments()
            .flat_map(|(_, segment)| {
                [
                    (segment.start.0, segment.end.0),
                    (segment.end.0, segment.start.0),
                ]
            })
            .collect();
        if ends.is_empty() {
            return Vec::new();
        }

        let mut leaving: Vec<Vec<usize>> = vec![Vec::new(); self.points().len()];
        for (half, (from, to)) in ends.iter().enumerate() {
            if from != to {
                leaving[*from].push(half);
            }
        }
        for (vertex, half_edges) in leaving.iter_mut().enumerate() {
            let from = self.points()[vertex];
            half_edges.sort_by(|a, b| {
                let angle = |half: usize| (self.points()[ends[half].1] - from).to_angle();
                angle(*a).total_cmp(&angle(*b))
            });
        }

        let next = |half: usize| -> Option<usize> {
            let twin = half ^ 1;
            let around = &leaving[ends[half].1];
            let position = around.iter().position(|candidate| *candidate == twin)?;
            // The neighbour just clockwise of the way we came: turning as
            // tightly as possible is what keeps the walk hugging one area.
            Some(around[(position + around.len() - 1) % around.len()])
        };

        let mut visited = vec![false; ends.len()];
        let mut outlines = Vec::new();
        for start in 0..ends.len() {
            if visited[start] || ends[start].0 == ends[start].1 {
                continue;
            }
            let mut loop_edges = Vec::new();
            let mut half = start;
            loop {
                if visited[half] {
                    break;
                }
                visited[half] = true;
                loop_edges.push(ends[half].0);
                let Some(following) = next(half) else { break };
                half = following;
                if half == start {
                    break;
                }
            }

            let outline: Vec<DVec2> = loop_edges
                .iter()
                .map(|vertex| self.points()[*vertex])
                .collect();
            // A dead-end branch is walked out and back, and the outermost walk
            // runs clockwise: neither encloses anything.
            let distinct = loop_edges.len() >= 3 && {
                let mut sorted = loop_edges.clone();
                sorted.sort_unstable();
                sorted.dedup();
                sorted.len() == loop_edges.len()
            };
            if distinct && signed_area(&outline) > 1e-9 && crate::crossing::is_simple(&outline) {
                outlines.push(outline);
            }
        }
        outlines
    }
}

/// A point inside the area and right up against its edge.
///
/// It has to hug the outline: a point taken well inside the shape would sit
/// inside whatever is drawn within it too, and every area would then count
/// itself as nested. The lowest corner is always a convex one, so stepping
/// just inside along its bisector lands in the area itself.
fn inside(region: &Region) -> DVec2 {
    let outline = &region.outline;
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

fn signed_area(outline: &[DVec2]) -> f64 {
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
fn bridge_holes(outline: &[DVec2], holes: &[Vec<DVec2>]) -> Vec<DVec2> {
    let mut path = counter_clockwise(outline);
    // Rightmost first: a hole further right can only ever bridge to the outline
    // or to a hole already spliced in, never to one still waiting.
    let mut pending: Vec<Vec<DVec2>> = holes.iter().map(|hole| clockwise(hole)).collect();
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
mod tests {
    use super::*;
    use crate::plane::WorkPlane;

    fn rectangle(sketch: &mut Sketch, min: DVec2, max: DVec2) {
        let corners = [
            sketch.add_point(min),
            sketch.add_point(DVec2::new(max.x, min.y)),
            sketch.add_point(max),
            sketch.add_point(DVec2::new(min.x, max.y)),
        ];
        for index in 0..4 {
            sketch.add_segment(corners[index], corners[(index + 1) % 4]);
        }
    }

    #[test]
    fn an_open_shape_encloses_nothing() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::ZERO);
        let b = sketch.add_point(DVec2::new(10.0, 0.0));
        let c = sketch.add_point(DVec2::new(10.0, 10.0));
        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        assert!(sketch.regions().is_empty());
    }

    #[test]
    fn a_closed_contour_is_one_region() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        rectangle(&mut sketch, DVec2::ZERO, DVec2::new(10.0, 4.0));
        let regions = sketch.regions();
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].depth, 0);
        let area: f64 = regions[0]
            .triangles
            .iter()
            .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
            .sum();
        assert!((area - 40.0).abs() < 1e-3, "area {area}");
    }

    #[test]
    fn a_shape_inside_another_is_one_level_deeper() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        rectangle(&mut sketch, DVec2::ZERO, DVec2::new(20.0, 20.0));
        rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(10.0, 10.0));
        let regions = sketch.regions();
        assert_eq!(regions.len(), 2);
        assert_eq!(regions[0].depth, 0);
        assert_eq!(regions[1].depth, 1);
    }

    #[test]
    fn two_shapes_sharing_a_side_are_two_regions() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::ZERO);
        let b = sketch.add_point(DVec2::new(10.0, 0.0));
        let c = sketch.add_point(DVec2::new(10.0, 10.0));
        let d = sketch.add_point(DVec2::new(0.0, 10.0));
        let e = sketch.add_point(DVec2::new(20.0, 0.0));
        let f = sketch.add_point(DVec2::new(20.0, 10.0));
        for (from, to) in [(a, b), (b, c), (c, d), (d, a), (b, e), (e, f), (f, c)] {
            sketch.add_segment(from, to);
        }
        let regions = sketch.regions();
        assert_eq!(regions.len(), 2);
        assert!(regions.iter().all(|region| region.depth == 0));
    }

    fn area(triangles: &[[DVec2; 3]]) -> f64 {
        triangles
            .iter()
            .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
            .sum()
    }

    /// Two circles one inside the other are a tube, not a rod: the face keeps
    /// the middle hollow.
    #[test]
    fn a_shape_inside_another_is_a_hole_in_its_face() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        rectangle(&mut sketch, DVec2::ZERO, DVec2::new(20.0, 20.0));
        rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(15.0, 15.0));
        let regions = sketch.regions();

        assert_eq!(regions[0].holes.len(), 1, "the outer contour is pierced");
        assert!(regions[1].holes.is_empty());

        let ring = area(&regions[0].face_triangles());
        assert!((ring - 300.0).abs() < 1e-2, "area of the ring: {ring}");
        assert!(
            (area(&regions[0].triangles) - 400.0).abs() < 1e-2,
            "the solid fill ignores the hole"
        );

        assert!(regions[0].contains(DVec2::new(2.0, 2.0)));
        assert!(
            !regions[0].contains(DVec2::new(10.0, 10.0)),
            "the hole is empty"
        );
        assert!(regions[1].contains(DVec2::new(10.0, 10.0)));
    }

    /// Matter inside a hole is matter again, and belongs to its own face.
    #[test]
    fn a_shape_inside_a_hole_is_not_a_hole_of_the_outer_one() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        rectangle(&mut sketch, DVec2::ZERO, DVec2::new(30.0, 30.0));
        rectangle(&mut sketch, DVec2::new(5.0, 5.0), DVec2::new(25.0, 25.0));
        rectangle(&mut sketch, DVec2::new(10.0, 10.0), DVec2::new(20.0, 20.0));
        let regions = sketch.regions();

        assert_eq!(regions[0].holes.len(), 1);
        assert_eq!(regions[1].holes.len(), 1);
        assert!(regions[2].holes.is_empty());
        assert!((area(&regions[2].face_triangles()) - 100.0).abs() < 1e-2);
    }

    /// Le cas cité : deux cercles concentriques font un tube.
    #[test]
    fn two_circles_make_a_tube() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(DVec2::new(10.0, 10.0));
        sketch.add_circle(center, 8.0);
        sketch.add_circle(center, 5.0);

        let regions = sketch.regions();
        assert_eq!(regions.len(), 2);
        let ring = area(&regions[0].face_triangles());
        let expected = std::f64::consts::PI * (8.0f64.powi(2) - 5.0f64.powi(2));
        assert!(
            (ring - expected).abs() / expected < 0.02,
            "ring {ring}, expected ~{expected}"
        );
        assert!(!regions[0].contains(DVec2::new(10.0, 10.0)));
    }

    #[test]
    fn a_circle_encloses_its_disc() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(DVec2::new(3.0, 3.0));
        sketch.add_circle(center, 2.0);
        let regions = sketch.regions();
        assert_eq!(regions.len(), 1);
        assert!(encloses(&regions[0].outline, DVec2::new(3.0, 3.0)));
    }
}
