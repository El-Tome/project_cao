use glam::Vec2;

use crate::sketch::Sketch;

/// A closed area of the drawing, ready to be tinted.
///
/// `depth` is how many other areas enclose this one: an outline drawn inside
/// another is one level deeper, which is what lets them be told apart at a
/// glance instead of reading as a single blob.
#[derive(Clone, Debug)]
pub struct Region {
    pub outline: Vec<Vec2>,
    pub depth: usize,
    pub triangles: Vec<[Vec2; 3]>,
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
        outlines.extend(self.circles().iter().map(|circle| {
            let center = self.point(circle.center);
            (0..CIRCLE_STEPS)
                .map(|step| {
                    let angle = std::f32::consts::TAU * step as f32 / CIRCLE_STEPS as f32;
                    center + Vec2::from_angle(angle) * circle.radius
                })
                .collect()
        }));

        let mut regions: Vec<Region> = outlines
            .into_iter()
            .filter_map(|outline| {
                let triangles = triangulate(&outline);
                (!triangles.is_empty()).then_some(Region {
                    outline,
                    depth: 0,
                    triangles,
                })
            })
            .collect();

        let insides: Vec<Vec2> = regions.iter().map(inside).collect();
        for (index, point) in insides.iter().enumerate() {
            regions[index].depth = regions
                .iter()
                .enumerate()
                .filter(|(other, region)| *other != index && encloses(&region.outline, *point))
                .count();
        }
        regions.sort_by_key(|region| region.depth);
        regions
    }

    /// Walks the segment graph and returns each area it encloses, as a loop of
    /// positions turning counter-clockwise.
    fn closed_outlines(&self) -> Vec<Vec<Vec2>> {
        let segments = self.segments();
        if segments.is_empty() {
            return Vec::new();
        }

        // Two half-edges per segment: a side is walked once in each direction,
        // once for the area on either side of it.
        let ends: Vec<(usize, usize)> = segments
            .iter()
            .flat_map(|segment| {
                [
                    (segment.start.0, segment.end.0),
                    (segment.end.0, segment.start.0),
                ]
            })
            .collect();

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

            let outline: Vec<Vec2> = loop_edges
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
            if distinct && signed_area(&outline) > 1e-9 {
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
fn inside(region: &Region) -> Vec2 {
    let outline = &region.outline;
    let count = outline.len();
    let corner = (0..count)
        .min_by(|a, b| {
            (outline[*a].y, outline[*a].x).partial_cmp(&(outline[*b].y, outline[*b].x)).unwrap()
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

fn signed_area(outline: &[Vec2]) -> f32 {
    let mut total = 0.0;
    for index in 0..outline.len() {
        let current = outline[index];
        let following = outline[(index + 1) % outline.len()];
        total += current.perp_dot(following);
    }
    total * 0.5
}

fn encloses(outline: &[Vec2], point: Vec2) -> bool {
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

/// Cuts a closed outline into triangles by clipping ears: repeatedly take a
/// corner no other corner sits in, and snip it off.
fn triangulate(outline: &[Vec2]) -> Vec<[Vec2; 3]> {
    if outline.len() < 3 || signed_area(outline).abs() < 1e-9 {
        return Vec::new();
    }
    let mut remaining: Vec<Vec2> = outline.to_vec();
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

fn in_triangle(point: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let side = |from: Vec2, to: Vec2| (to - from).perp_dot(point - from);
    side(a, b) >= 0.0 && side(b, c) >= 0.0 && side(c, a) >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::WorkPlane;

    fn rectangle(sketch: &mut Sketch, min: Vec2, max: Vec2) {
        let corners = [
            sketch.add_point(min),
            sketch.add_point(Vec2::new(max.x, min.y)),
            sketch.add_point(max),
            sketch.add_point(Vec2::new(min.x, max.y)),
        ];
        for index in 0..4 {
            sketch.add_segment(corners[index], corners[(index + 1) % 4]);
        }
    }

    #[test]
    fn an_open_shape_encloses_nothing() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(10.0, 0.0));
        let c = sketch.add_point(Vec2::new(10.0, 10.0));
        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        assert!(sketch.regions().is_empty());
    }

    #[test]
    fn a_closed_contour_is_one_region() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        rectangle(&mut sketch, Vec2::ZERO, Vec2::new(10.0, 4.0));
        let regions = sketch.regions();
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].depth, 0);
        let area: f32 = regions[0]
            .triangles
            .iter()
            .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
            .sum();
        assert!((area - 40.0).abs() < 1e-3, "aire {area}");
    }

    #[test]
    fn a_shape_inside_another_is_one_level_deeper() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        rectangle(&mut sketch, Vec2::ZERO, Vec2::new(20.0, 20.0));
        rectangle(&mut sketch, Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        let regions = sketch.regions();
        assert_eq!(regions.len(), 2);
        assert_eq!(regions[0].depth, 0);
        assert_eq!(regions[1].depth, 1);
    }

    #[test]
    fn two_shapes_sharing_a_side_are_two_regions() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(Vec2::ZERO);
        let b = sketch.add_point(Vec2::new(10.0, 0.0));
        let c = sketch.add_point(Vec2::new(10.0, 10.0));
        let d = sketch.add_point(Vec2::new(0.0, 10.0));
        let e = sketch.add_point(Vec2::new(20.0, 0.0));
        let f = sketch.add_point(Vec2::new(20.0, 10.0));
        for (from, to) in [(a, b), (b, c), (c, d), (d, a), (b, e), (e, f), (f, c)] {
            sketch.add_segment(from, to);
        }
        let regions = sketch.regions();
        assert_eq!(regions.len(), 2);
        assert!(regions.iter().all(|region| region.depth == 0));
    }

    #[test]
    fn a_circle_encloses_its_disc() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(Vec2::new(3.0, 3.0));
        sketch.add_circle(center, 2.0);
        let regions = sketch.regions();
        assert_eq!(regions.len(), 1);
        assert!(encloses(&regions[0].outline, Vec2::new(3.0, 3.0)));
    }
}
