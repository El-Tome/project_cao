use glam::Vec3;

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

        let mut a = Node::build(&self.polygons);
        let mut b = Node::build(&other.polygons);
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

        let mut a = Node::build(&self.polygons);
        let mut b = Node::build(&other.polygons);
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

/// How far off a plane a corner has to be to count as on one side of it.
const ON_PLANE: f32 = 1e-5;

#[derive(Clone, Copy, PartialEq)]
struct Plane {
    normal: Vec3,
    offset: f32,
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

        let side = |corner: Vec3| {
            let distance = self.normal.dot(corner) - self.offset;
            if distance < -ON_PLANE {
                BACK
            } else if distance > ON_PLANE {
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
/// of space on either side.
struct Node {
    plane: Option<Plane>,
    front: Option<Box<Node>>,
    back: Option<Box<Node>>,
    on_plane: Vec<Polygon>,
}

impl Node {
    fn build(polygons: &[Polygon]) -> Self {
        let mut node = Self {
            plane: None,
            front: None,
            back: None,
            on_plane: Vec::new(),
        };
        node.add(polygons.to_vec());
        node
    }

    fn add(&mut self, polygons: Vec<Polygon>) {
        if polygons.is_empty() {
            return;
        }
        let plane = *self.plane.get_or_insert_with(|| Plane::of(&polygons[0]));

        let mut split = Split::default();
        for polygon in &polygons {
            plane.split(polygon, &mut split);
        }
        self.on_plane.append(&mut split.coplanar_front);
        self.on_plane.append(&mut split.coplanar_back);

        if !split.front.is_empty() {
            self.front
                .get_or_insert_with(|| Box::new(Node::empty()))
                .add(split.front);
        }
        if !split.back.is_empty() {
            self.back
                .get_or_insert_with(|| Box::new(Node::empty()))
                .add(split.back);
        }
    }

    fn empty() -> Self {
        Self {
            plane: None,
            front: None,
            back: None,
            on_plane: Vec::new(),
        }
    }

    /// Turns the solid inside out: every face and every plane looks the other
    /// way, and the two halves of space swap.
    fn invert(&mut self) {
        for polygon in &mut self.on_plane {
            *polygon = polygon.flipped();
        }
        if let Some(plane) = &mut self.plane {
            plane.flip();
        }
        if let Some(front) = &mut self.front {
            front.invert();
        }
        if let Some(back) = &mut self.back {
            back.invert();
        }
        std::mem::swap(&mut self.front, &mut self.back);
    }

    /// Drops whatever of these faces falls inside the solid.
    fn clip(&self, polygons: Vec<Polygon>) -> Vec<Polygon> {
        let Some(plane) = self.plane else {
            return polygons;
        };

        let mut split = Split::default();
        for polygon in &polygons {
            plane.split(polygon, &mut split);
        }
        // A face lying on the plane belongs to the side it looks towards.
        split.front.append(&mut split.coplanar_front);
        split.back.append(&mut split.coplanar_back);

        let mut kept = match &self.front {
            Some(node) => node.clip(split.front),
            None => split.front,
        };
        // No half behind this plane means everything there is inside the solid.
        if let Some(node) = &self.back {
            kept.extend(node.clip(split.back));
        }
        kept
    }

    fn clip_to(&mut self, other: &Node) {
        self.on_plane = other.clip(std::mem::take(&mut self.on_plane));
        if let Some(front) = &mut self.front {
            front.clip_to(other);
        }
        if let Some(back) = &mut self.back {
            back.clip_to(other);
        }
    }

    fn polygons(&self) -> Vec<Polygon> {
        let mut all = self.on_plane.clone();
        if let Some(front) = &self.front {
            all.extend(front.polygons());
        }
        if let Some(back) = &self.back {
            all.extend(back.polygons());
        }
        all
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::mesh::tests::{box_of, volume};

    #[test]
    fn adding_two_solids_apart_keeps_both() {
        let a = box_of(10.0, 10.0, Vec3::ZERO);
        let b = box_of(10.0, 10.0, Vec3::new(30.0, 0.0, 0.0));
        assert!((volume(&a.union(&b)) - 2000.0).abs() < 1.0);
    }

    /// Overlapping matter is counted once, not twice.
    #[test]
    fn adding_two_solids_that_overlap_counts_the_middle_once() {
        let a = box_of(10.0, 10.0, Vec3::ZERO);
        let b = box_of(10.0, 10.0, Vec3::new(5.0, 0.0, 0.0));
        let joined = volume(&a.union(&b));
        assert!((joined - 1500.0).abs() < 1.0, "{joined}");
    }

    /// The case this is all for: a pocket cut into a block.
    #[test]
    fn cutting_a_pocket_takes_matter_away() {
        let block = box_of(10.0, 10.0, Vec3::ZERO);
        let tool = box_of(4.0, 20.0, Vec3::new(3.0, 3.0, -5.0));
        let cut = volume(&block.difference(&tool));
        assert!((cut - (1000.0 - 160.0)).abs() < 1.0, "{cut}");
    }

    #[test]
    fn cutting_with_something_that_misses_changes_nothing() {
        let block = box_of(10.0, 10.0, Vec3::ZERO);
        let tool = box_of(4.0, 4.0, Vec3::new(40.0, 40.0, 0.0));
        assert!((volume(&block.difference(&tool)) - 1000.0).abs() < 1.0);
    }

    /// Cutting a solid clean through leaves nothing behind.
    #[test]
    fn cutting_everything_away_leaves_nothing() {
        let block = box_of(10.0, 10.0, Vec3::ZERO);
        let tool = box_of(30.0, 30.0, Vec3::new(-10.0, -10.0, -10.0));
        let left = volume(&block.difference(&tool));
        assert!(left.abs() < 1.0, "{left}");
    }
}
