use std::collections::{BTreeMap, BTreeSet};

use glam::DVec2;

use super::triangles;

/// A region given as loops of points, each with the region on its left.
struct Region {
    points: Vec<DVec2>,
    segments: Vec<[usize; 2]>,
}

impl Region {
    fn new() -> Region {
        Region {
            points: Vec::new(),
            segments: Vec::new(),
        }
    }

    /// The point at `at`, the same one each time it is asked for.
    fn point(&mut self, at: DVec2) -> usize {
        match self.points.iter().position(|known| *known == at) {
            Some(index) => index,
            None => {
                self.points.push(at);
                self.points.len() - 1
            }
        }
    }

    fn with_loop(mut self, corners: &[DVec2]) -> Region {
        let ids: Vec<usize> = corners.iter().map(|at| self.point(*at)).collect();
        for (at, from) in ids.iter().enumerate() {
            self.segments.push([*from, ids[(at + 1) % ids.len()]]);
        }
        self
    }

    fn area(&self) -> f64 {
        self.segments
            .iter()
            .map(|[from, to]| self.points[*from].perp_dot(self.points[*to]) / 2.0)
            .sum()
    }

    /// Cut, and held to what a cut must be: every triangle turning
    /// counterclockwise, their areas adding up to the region's, every side of
    /// the boundary a side of a triangle along its own way, and every other
    /// side of a triangle the side of another the other way.
    fn cut(&self) -> Vec<[usize; 3]> {
        let cut = triangles(&self.points, &self.segments).expect("the region is cut");
        let mut sides: BTreeMap<(usize, usize), i64> = BTreeMap::new();
        let mut area = 0.0;
        for &[a, b, c] in &cut {
            let [pa, pb, pc] = [a, b, c].map(|index| self.points[index]);
            let doubled = (pb - pa).perp_dot(pc - pa);
            assert!(doubled > 0.0, "{pa} {pb} {pc} turn clockwise or not at all");
            area += doubled / 2.0;
            for (from, to) in [(a, b), (b, c), (c, a)] {
                *sides.entry((from, to)).or_default() += 1;
            }
        }
        for [from, to] in &self.segments {
            *sides.entry((*from, *to)).or_default() -= 1;
        }
        for (&(from, to), &count) in &sides {
            assert!(
                count >= 0,
                "the side {from} to {to} of the boundary is missing"
            );
            assert_eq!(
                count,
                sides.get(&(to, from)).copied().unwrap_or(0),
                "the side {from} to {to} is not matched the other way",
            );
        }
        assert!(
            (area - self.area()).abs() < 1e-9 * self.area().abs().max(1.0),
            "the triangles cover {area}, the region {}",
            self.area(),
        );
        cut
    }
}

fn square(center: DVec2, half: f64) -> Vec<DVec2> {
    [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
        .map(|(x, y)| center + DVec2::new(x, y) * half)
        .to_vec()
}

#[test]
fn a_square_is_cut_into_two_triangles() {
    let region = Region::new().with_loop(&square(DVec2::ZERO, 20.0));
    assert_eq!(region.cut().len(), 2);
}

fn reversed(mut corners: Vec<DVec2>) -> Vec<DVec2> {
    corners.reverse();
    corners
}

/// `count` points round a circle, from `start` and turning `way`.
fn round(center: DVec2, radius: f64, count: usize, start: f64, way: f64) -> Vec<DVec2> {
    (0..count)
        .map(|at| {
            let angle = start + way * std::f64::consts::TAU * at as f64 / count as f64;
            center + DVec2::from_angle(angle) * radius
        })
        .collect()
}

#[test]
fn a_square_with_several_holes_is_cut_round_every_one() {
    let region = Region::new()
        .with_loop(&square(DVec2::ZERO, 20.0))
        .with_loop(&reversed(square(DVec2::new(-10.0, 5.0), 3.0)))
        .with_loop(&reversed(square(DVec2::new(8.0, -6.0), 4.0)))
        .with_loop(&round(DVec2::new(8.0, 9.0), 5.0, 36, 0.0, -1.0));
    region.cut();
}

#[test]
fn a_comb_whose_teeth_hang_down_is_cut_between_its_teeth() {
    let mut corners = vec![DVec2::new(0.0, 0.0)];
    for tooth in 0..6 {
        let left = 4.0 * tooth as f64;
        corners.extend([
            DVec2::new(left + 1.0, 0.0),
            DVec2::new(left + 2.0, 9.0 - tooth as f64),
            DVec2::new(left + 3.0, 0.0),
        ]);
    }
    corners.extend([
        DVec2::new(24.0, 0.0),
        DVec2::new(24.0, 12.0),
        DVec2::new(0.0, 12.0),
    ]);
    Region::new().with_loop(&corners).cut();
}

#[test]
fn a_slit_inside_a_square_is_a_side_of_the_triangles_on_both_of_its_sides() {
    let (from, to) = (DVec2::new(-10.0, 0.0), DVec2::new(10.0, 0.0));
    let region = Region::new()
        .with_loop(&square(DVec2::ZERO, 20.0))
        .with_loop(&[from, to]);
    region.cut();
}

#[test]
fn a_slit_running_in_from_the_boundary_is_cut_round() {
    let corners = [
        DVec2::new(-20.0, -20.0),
        DVec2::new(20.0, -20.0),
        DVec2::new(20.0, 0.0),
        DVec2::new(0.0, 0.0),
        DVec2::new(20.0, 0.0),
        DVec2::new(20.0, 20.0),
        DVec2::new(-20.0, 20.0),
    ];
    Region::new().with_loop(&corners).cut();
}

#[test]
fn two_holes_touching_at_a_point_are_cut_round_both() {
    let touching = DVec2::ZERO;
    let region = Region::new()
        .with_loop(&square(DVec2::ZERO, 20.0))
        .with_loop(&round(DVec2::new(-5.0, 0.0), 5.0, 24, 0.0, -1.0))
        .with_loop(&round(
            DVec2::new(5.0, 0.0),
            5.0,
            24,
            std::f64::consts::PI,
            -1.0,
        ));
    assert!(region.points.contains(&touching));
    region.cut();
}

#[test]
fn a_loop_that_visits_a_corner_twice_is_cut_on_both_sides_of_it() {
    let mut corners = round(DVec2::ZERO, 20.0, 72, 0.0, 1.0);
    corners.extend(round(DVec2::new(15.0, 0.0), 5.0, 36, 0.0, -1.0));
    Region::new().with_loop(&corners).cut();
}

#[test]
fn the_crescent_between_two_polygons_touching_inside_is_cut_to_its_tip() {
    let mut corners = round(DVec2::ZERO, 20.0, 1024, 0.0, 1.0);
    corners.extend(round(DVec2::new(19.0, 0.0), 1.0, 64, 0.0, -1.0));
    Region::new().with_loop(&corners).cut();
}

#[test]
fn polygons_touching_inside_whose_chords_cross_near_the_touch_are_refused() {
    let mut corners = round(DVec2::ZERO, 20.0, 16, 0.0, 1.0);
    corners.extend(round(DVec2::new(10.0, 0.0), 10.0, 1024, 0.0, -1.0));
    let region = Region::new().with_loop(&corners);
    assert!(triangles(&region.points, &region.segments).is_none());
}

#[test]
fn points_in_a_row_along_the_boundary_make_no_flat_triangle() {
    let mut corners = Vec::new();
    for step in 0..8 {
        corners.push(DVec2::new(-20.0 + 5.0 * step as f64, -20.0));
    }
    for step in 0..8 {
        corners.push(DVec2::new(20.0, -20.0 + 5.0 * step as f64));
    }
    for step in 0..8 {
        corners.push(DVec2::new(20.0 - 5.0 * step as f64, 20.0));
    }
    for step in 0..8 {
        corners.push(DVec2::new(-20.0, 20.0 - 5.0 * step as f64));
    }
    Region::new().with_loop(&corners).cut();
}

#[test]
fn a_band_whose_two_boundaries_share_every_abscissa_is_cut_into_quadrilaterals() {
    let count = 16;
    let bottom = (0..=count).map(|at| DVec2::new(at as f64, (at as f64 * 0.7).sin()));
    let top = (0..=count)
        .rev()
        .map(|at| DVec2::new(at as f64, 5.0 + (at as f64).cos()));
    let corners: Vec<DVec2> = bottom.chain(top).collect();
    assert_eq!(Region::new().with_loop(&corners).cut().len(), 2 * count);
}

#[test]
fn boundaries_that_cross_are_refused() {
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 10.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(0.0, 10.0),
    ];
    let region = Region::new().with_loop(&corners);
    assert!(triangles(&region.points, &region.segments).is_none());
}

/// A star round `center`: `count` points at angles evenly spread, each at a
/// distance drawn between `near` and `far`.
fn star(center: DVec2, near: f64, far: f64, count: usize, seed: &mut u64, way: f64) -> Vec<DVec2> {
    (0..count)
        .map(|at| {
            *seed = seed
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let drawn = (*seed >> 11) as f64 / (1u64 << 53) as f64;
            let angle = way * std::f64::consts::TAU * at as f64 / count as f64;
            center + DVec2::from_angle(angle) * (near + (far - near) * drawn)
        })
        .collect()
}

#[test]
fn stars_of_any_spikiness_are_cut_with_a_star_shaped_hole_inside() {
    let mut seed = 498;
    let center = DVec2::new(0.5, -0.25);
    for count in [3, 5, 8, 13, 40, 200] {
        let clear = 10.0 * (std::f64::consts::PI / count as f64).cos() - center.length();
        for _ in 0..20 {
            let outline = star(DVec2::ZERO, 10.0, 20.0, count, &mut seed, 1.0);
            let hole = star(
                center,
                clear / 4.0,
                0.99 * clear,
                count + 2,
                &mut seed,
                -1.0,
            );
            Region::new().with_loop(&outline).with_loop(&hole).cut();
        }
    }
}

/// A corner of the lattice the squares below stand on.
type Corner = (i64, i64);

/// A region of whole squares of a lattice, drawn with `draw`: its boundary
/// is every side of a filled square not shared with another, cut into
/// `parts` in a row, and some shared sides are kept both ways as slits — none
/// closing a loop, which would part the region in two. Holes, pinches where
/// two squares meet at a corner, slits from the boundary, between two holes
/// and hanging free all come out of it.
fn squares(draw: &mut impl FnMut(u64) -> u64) -> Region {
    let size = 2 + draw(6) as i64;
    let fill = 30 + draw(60);
    let parts = [1, 2, 4][draw(3) as usize];
    let slits = draw(2) == 0;
    let mut filled = BTreeSet::new();
    for x in 0..size {
        for y in 0..size {
            if draw(100) < fill {
                filled.insert((x, y));
            }
        }
    }
    let mut sides: BTreeSet<[Corner; 2]> = BTreeSet::new();
    let mut shared = Vec::new();
    for &(x, y) in &filled {
        let corners = [(x, y), (x + 1, y), (x + 1, y + 1), (x, y + 1)];
        for at in 0..4 {
            let (from, to) = (corners[at], corners[(at + 1) % 4]);
            if !sides.remove(&[to, from]) {
                sides.insert([from, to]);
            } else {
                shared.push([from, to]);
            }
        }
    }
    let mut parent: BTreeMap<Corner, Corner> = BTreeMap::new();
    for &[from, to] in &sides {
        let (one, other) = (root(&parent, from), root(&parent, to));
        if one != other {
            parent.insert(one, other);
        }
    }
    for [from, to] in shared {
        let (one, other) = (root(&parent, from), root(&parent, to));
        if slits && one != other && draw(100) < 40 {
            parent.insert(one, other);
            sides.insert([from, to]);
            sides.insert([to, from]);
        }
    }
    let mut region = Region::new();
    for &[(fx, fy), (tx, ty)] in &sides {
        let low = (fx, fy).min((tx, ty));
        let high = (fx, fy).max((tx, ty));
        let place = |part: i64| {
            let from_low = if (fx, fy) == low { part } else { parts - part };
            let along = from_low as f64 / parts as f64;
            DVec2::new(
                low.0 as f64 + (high.0 - low.0) as f64 * along,
                low.1 as f64 + (high.1 - low.1) as f64 * along,
            )
        };
        for part in 0..parts {
            let (from, to) = (region.point(place(part)), region.point(place(part + 1)));
            region.segments.push([from, to]);
        }
    }
    region
}

/// The corner standing for every corner joined to `at` so far.
fn root(parent: &BTreeMap<Corner, Corner>, at: Corner) -> Corner {
    let mut at = at;
    while let Some(&up) = parent.get(&at) {
        at = up;
    }
    at
}

#[test]
fn any_region_of_whole_squares_with_its_holes_pinches_and_slits_is_cut_exactly() {
    let mut seed: u64 = 498;
    let mut draw = |below: u64| {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (seed >> 33) % below
    };
    let mut slits = 0;
    for _ in 0..2000 {
        let region = squares(&mut draw);
        if !region.segments.is_empty() {
            region.cut();
        }
        let both_ways = |[from, to]: &[usize; 2]| region.segments.contains(&[*to, *from]);
        slits += usize::from(region.segments.iter().any(both_ways));
    }
    assert!(slits > 100, "only {slits} regions with a slit");
}
