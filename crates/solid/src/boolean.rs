use glam::DVec3;

use crate::mesh::{Mesh, Polygon};

/// Adding and removing matter, by sorting faces against planes.
///
/// The method is the binary space partition: each solid is turned into a tree
/// of planes taken from its own faces, and the other solid's faces are pushed
/// through it. A face comes out labelled inside or outside, and cut in two
/// where the plane crosses it. Union and difference are then a matter of
/// keeping the right halves and turning one solid inside out.
///
/// It works on any shape, convex or not, which a simpler method could not do —
/// a pocket cut into a block is exactly the case that breaks them.
impl Mesh {
    /// Everything that is in either solid.
    pub fn union(&self, other: &Mesh) -> Mesh {
        if self.is_empty() {
            return other.clone();
        }
        if other.is_empty() {
            return self.clone();
        }

        let mut a = Tree::of(&self.polygons);
        let mut b = Tree::of(&other.polygons);
        a.clip_to(&b);
        b.clip_to(&a);
        // The faces of B left inside A's surface would be buried; those left on
        // it would be drawn twice. Turning B inside out twice around the clip
        // drops both without dropping the ones that matter.
        b.invert();
        b.clip_to(&a);
        b.invert();

        let mut polygons = a.polygons();
        polygons.extend(b.polygons());
        Mesh { polygons }
    }

    /// Everything that is in this solid and not in the other.
    pub fn difference(&self, other: &Mesh) -> Mesh {
        if self.is_empty() || other.is_empty() {
            return self.clone();
        }

        let mut a = Tree::of(&self.polygons);
        let mut b = Tree::of(&other.polygons);
        // Taking matter away is adding the *inside* of the tool: turn this
        // solid inside out, add, and turn the result back.
        a.invert();
        a.clip_to(&b);
        b.clip_to(&a);
        b.invert();
        b.clip_to(&a);
        b.invert();

        let mut polygons = a.polygons();
        polygons.extend(b.polygons());
        let mut result = Mesh { polygons };
        for polygon in &mut result.polygons {
            *polygon = polygon.flipped();
        }
        result
    }
}

/// How far off a plane a corner has to be to count as on one side of it,
/// relative to how far from the origin the geometry sits.
///
/// A fixed tolerance does not work: the error of a dot product grows with the
/// size of what it is taken on, so a corner fifty units out carries more of it
/// than one at the origin. Below a tolerance that small, a triangle comes out
/// straddling *its own* plane — and the partition then cuts it forever.
///
/// The value is what `f64` allows: it kept about seven digits when this was
/// `f32`, and needed a millionth of slack; it now keeps sixteen, so a
/// billionth leaves the same margin over the noise with far less of the
/// geometry blurred away.
const ON_PLANE: f64 = 1e-9;

fn tolerance(scale: f64) -> f64 {
    ON_PLANE * scale.abs().max(1.0)
}

#[derive(Clone, Copy, PartialEq)]
struct Plane {
    normal: DVec3,
    offset: f64,
}

impl Plane {
    fn of(polygon: &Polygon) -> Self {
        Self {
            normal: polygon.normal(),
            offset: polygon.plane_offset(),
        }
    }

    fn flip(&mut self) {
        self.normal = -self.normal;
        self.offset = -self.offset;
    }

    /// Sorts a face against the plane, cutting it where it straddles.
    fn split(&self, polygon: &Polygon, out: &mut Split) {
        const COPLANAR: u8 = 0;
        const FRONT: u8 = 1;
        const BACK: u8 = 2;

        let side = |corner: DVec3| {
            let distance = self.normal.dot(corner) - self.offset;
            let room = tolerance(corner.abs().max_element().max(self.offset));
            if distance < -room {
                BACK
            } else if distance > room {
                FRONT
            } else {
                COPLANAR
            }
        };

        let sides: Vec<u8> = polygon.corners.iter().map(|corner| side(*corner)).collect();
        let overall = sides.iter().fold(0, |all, each| all | each);

        match overall {
            COPLANAR => {
                if self.normal.dot(polygon.normal()) > 0.0 {
                    out.coplanar_front.push(polygon.clone());
                } else {
                    out.coplanar_back.push(polygon.clone());
                }
            }
            FRONT => out.front.push(polygon.clone()),
            BACK => out.back.push(polygon.clone()),
            _ => {
                let (mut in_front, mut behind) = (Vec::new(), Vec::new());
                for index in 0..polygon.corners.len() {
                    let next = (index + 1) % polygon.corners.len();
                    let (current, following) = (polygon.corners[index], polygon.corners[next]);

                    if sides[index] != BACK {
                        in_front.push(current);
                    }
                    if sides[index] != FRONT {
                        behind.push(current);
                    }
                    if (sides[index] | sides[next]) == (FRONT | BACK) {
                        let along = (self.offset - self.normal.dot(current))
                            / self.normal.dot(following - current);
                        let crossing = current.lerp(following, along);
                        in_front.push(crossing);
                        behind.push(crossing);
                    }
                }
                out.front.extend(Polygon::new(in_front));
                out.back.extend(Polygon::new(behind));
            }
        }
    }
}

/// Where the pieces of a face land once a plane has cut it.
#[derive(Default)]
struct Split {
    coplanar_front: Vec<Polygon>,
    coplanar_back: Vec<Polygon>,
    front: Vec<Polygon>,
    back: Vec<Polygon>,
}

/// One plane of the partition, with the faces lying on it and the two halves
/// of space on either side, named by their place in the tree's own store.
struct Node {
    plane: Option<Plane>,
    front: Option<usize>,
    back: Option<usize>,
    on_plane: Vec<Polygon>,
}

impl Node {
    fn empty() -> Self {
        Self {
            plane: None,
            front: None,
            back: None,
            on_plane: Vec::new(),
        }
    }
}

/// The partition itself: every node in one flat store, children named by
/// position rather than owned.
///
/// This is what keeps the whole thing off the call stack. A solid made of many
/// small facets — a cylinder, anything revolved — gives a tree that degenerates
/// into a chain hundreds or thousands of nodes long, since each facet's plane
/// leaves every other facet on the same side of it. Walking that by recursion
/// ends the program on a blown stack, with no error and nothing to go on; the
/// crash is real and it is what this shape is for.
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn of(polygons: &[Polygon]) -> Self {
        let mut tree = Self {
            nodes: vec![Node::empty()],
        };
        tree.add(polygons.to_vec());
        tree
    }

    fn add(&mut self, polygons: Vec<Polygon>) {
        let mut pending = vec![(0usize, polygons)];

        while let Some((index, mut polygons)) = pending.pop() {
            if polygons.is_empty() {
                continue;
            }

            let plane = match self.nodes[index].plane {
                Some(plane) => plane,
                None => {
                    let plane = Plane::of(&polygons[0]);
                    self.nodes[index].plane = Some(plane);
                    // The face that gave the plane lies on it by definition,
                    // and is set aside rather than sorted. Sorting it would
                    // rest on arithmetic saying a face is on one side of
                    // itself, and one wrong answer there is a walk that never
                    // ends — the shape stays on screen and the program stops.
                    self.nodes[index].on_plane.push(polygons.remove(0));
                    plane
                }
            };

            let mut split = Split::default();
            for polygon in &polygons {
                plane.split(polygon, &mut split);
            }
            self.nodes[index].on_plane.append(&mut split.coplanar_front);
            self.nodes[index].on_plane.append(&mut split.coplanar_back);

            for (half, polygons) in [(Half::Front, split.front), (Half::Back, split.back)] {
                if polygons.is_empty() {
                    continue;
                }
                let child = self.child(index, half);
                pending.push((child, polygons));
            }
        }
    }

    fn child(&mut self, index: usize, half: Half) -> usize {
        let existing = match half {
            Half::Front => self.nodes[index].front,
            Half::Back => self.nodes[index].back,
        };
        if let Some(child) = existing {
            return child;
        }
        self.nodes.push(Node::empty());
        let child = self.nodes.len() - 1;
        match half {
            Half::Front => self.nodes[index].front = Some(child),
            Half::Back => self.nodes[index].back = Some(child),
        }
        child
    }

    /// Turns the solid inside out: every face and every plane looks the other
    /// way, and the two halves of space swap.
    fn invert(&mut self) {
        for node in &mut self.nodes {
            for polygon in &mut node.on_plane {
                *polygon = polygon.flipped();
            }
            if let Some(plane) = &mut node.plane {
                plane.flip();
            }
            std::mem::swap(&mut node.front, &mut node.back);
        }
    }

    /// Drops whatever of these faces falls inside the solid.
    fn clip(&self, polygons: Vec<Polygon>) -> Vec<Polygon> {
        let mut kept = Vec::new();
        let mut pending = vec![(0usize, polygons)];

        while let Some((index, polygons)) = pending.pop() {
            let node = &self.nodes[index];
            let Some(plane) = node.plane else {
                kept.extend(polygons);
                continue;
            };

            let mut split = Split::default();
            for polygon in &polygons {
                plane.split(polygon, &mut split);
            }
            // A face lying on the plane belongs to the side it looks towards.
            split.front.append(&mut split.coplanar_front);
            split.back.append(&mut split.coplanar_back);

            match node.front {
                Some(child) => pending.push((child, split.front)),
                None => kept.extend(split.front),
            }
            // No half behind this plane means everything there is inside the
            // solid, and is dropped.
            if let Some(child) = node.back {
                pending.push((child, split.back));
            }
        }
        kept
    }

    fn clip_to(&mut self, other: &Tree) {
        for index in 0..self.nodes.len() {
            let faces = std::mem::take(&mut self.nodes[index].on_plane);
            self.nodes[index].on_plane = other.clip(faces);
        }
    }

    fn polygons(&self) -> Vec<Polygon> {
        self.nodes
            .iter()
            .flat_map(|node| node.on_plane.iter().cloned())
            .collect()
    }
}

#[derive(Clone, Copy)]
enum Half {
    Front,
    Back,
}

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use crate::mesh::tests::{box_of, volume};

    #[test]
    fn adding_two_solids_apart_keeps_both() {
        let a = box_of(10.0, 10.0, DVec3::ZERO);
        let b = box_of(10.0, 10.0, DVec3::new(30.0, 0.0, 0.0));
        assert!((volume(&a.union(&b)) - 2000.0).abs() < 1.0);
    }

    /// Overlapping matter is counted once, not twice.
    #[test]
    fn adding_two_solids_that_overlap_counts_the_middle_once() {
        let a = box_of(10.0, 10.0, DVec3::ZERO);
        let b = box_of(10.0, 10.0, DVec3::new(5.0, 0.0, 0.0));
        let joined = volume(&a.union(&b));
        assert!((joined - 1500.0).abs() < 1.0, "{joined}");
    }

    /// The case this is all for: a pocket cut into a block.
    #[test]
    fn cutting_a_pocket_takes_matter_away() {
        let block = box_of(10.0, 10.0, DVec3::ZERO);
        let tool = box_of(4.0, 20.0, DVec3::new(3.0, 3.0, -5.0));
        let cut = volume(&block.difference(&tool));
        assert!((cut - (1000.0 - 160.0)).abs() < 1.0, "{cut}");
    }

    /// Faces landing exactly on each other are where a boolean goes wrong, and
    /// two cuts in a row is when it shows. This has to come out clean, and above
    /// all it has to come back at all.
    #[test]
    fn cutting_twice_with_faces_that_land_on_each_other() {
        let block = box_of(20.0, 10.0, DVec3::ZERO);
        // Both tools start exactly on the block's bottom face and one shares a
        // side with it.
        let small = box_of(4.0, 10.0, DVec3::new(2.0, 2.0, 0.0));
        let wide = box_of(8.0, 10.0, DVec3::new(0.0, 8.0, 0.0));

        let once = block.difference(&small);
        let twice = once.difference(&wide);
        let left = volume(&twice);
        assert!((left - (4000.0 - 160.0 - 640.0)).abs() < 2.0, "{left}");
    }

    /// Two areas extruded together are one tool, whatever they overlap.
    #[test]
    fn two_tools_joined_then_cut_leave_one_shape() {
        let block = box_of(20.0, 10.0, DVec3::ZERO);
        let small = box_of(4.0, 30.0, DVec3::new(2.0, 2.0, -10.0));
        let wide = box_of(9.0, 30.0, DVec3::new(1.0, 1.0, -10.0));

        // The small one is entirely inside the wide one.
        let tool = small.union(&wide);
        let left = volume(&block.difference(&tool));
        assert!((left - (4000.0 - 810.0)).abs() < 2.0, "{left}");
    }

    /// The case that brought the application down: a part made of hundreds of
    /// small facets, dug into. The tree of planes degenerates there into a long
    /// chain, and a recursive version exhausts the stack.
    #[test]
    fn cutting_into_a_many_faceted_solid_comes_back() {
        let steps = 96;
        let outline: Vec<glam::DVec2> = vec![
            glam::DVec2::new(3.0, 0.0),
            glam::DVec2::new(9.0, 0.0),
            glam::DVec2::new(9.0, 6.0),
            glam::DVec2::new(3.0, 6.0),
        ];
        let triangles = vec![
            [outline[0], outline[1], outline[2]],
            [outline[0], outline[2], outline[3]],
        ];
        let cylinder = crate::mesh::revolution(
            &outline,
            &[],
            &triangles,
            |point| DVec3::new(point.x, point.y, 0.0),
            glam::DVec2::ZERO,
            glam::DVec2::Y,
            std::f64::consts::TAU,
        )
        .expect("un cylindre");
        assert!(cylinder.polygons.len() > steps, "assez de facettes");

        let tool = box_of(3.0, 30.0, DVec3::new(4.0, 1.0, -15.0));
        let cut = cylinder.difference(&tool);
        assert!(volume(&cut) < volume(&cylinder));
        assert!(volume(&cut) > 0.0);
    }

    #[test]
    fn cutting_with_something_that_misses_changes_nothing() {
        let block = box_of(10.0, 10.0, DVec3::ZERO);
        let tool = box_of(4.0, 4.0, DVec3::new(40.0, 40.0, 0.0));
        assert!((volume(&block.difference(&tool)) - 1000.0).abs() < 1.0);
    }

    /// Cutting a solid clean through leaves nothing behind.
    #[test]
    fn cutting_everything_away_leaves_nothing() {
        let block = box_of(10.0, 10.0, DVec3::ZERO);
        let tool = box_of(30.0, 30.0, DVec3::new(-10.0, -10.0, -10.0));
        let left = volume(&block.difference(&tool));
        assert!(left.abs() < 1.0, "{left}");
    }
}
