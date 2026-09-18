use std::f32::consts::{FRAC_PI_2, PI, TAU};

use glam::camera::rh::proj::directx;
use glam::{Mat4, Quat, Vec2, Vec3};

/// One of the six faces of the orientation cube, and by extension one of the
/// six axis-aligned standard views.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CubeFace {
    PlusX,
    MinusX,
    PlusY,
    MinusY,
    PlusZ,
    MinusZ,
}

impl CubeFace {
    pub const ALL: [Self; 6] = [
        Self::PlusX,
        Self::MinusX,
        Self::PlusY,
        Self::MinusY,
        Self::PlusZ,
        Self::MinusZ,
    ];

    pub fn normal(self) -> Vec3 {
        match self {
            Self::PlusX => Vec3::X,
            Self::MinusX => Vec3::NEG_X,
            Self::PlusY => Vec3::Y,
            Self::MinusY => Vec3::NEG_Y,
            Self::PlusZ => Vec3::Z,
            Self::MinusZ => Vec3::NEG_Z,
        }
    }

    /// The two world axes spanning the plane this face looks at: the basis of
    /// the work plane a sketch would sit on.
    pub fn plane_basis(self) -> (Vec3, Vec3) {
        match self {
            Self::PlusX | Self::MinusX => (Vec3::Y, Vec3::Z),
            Self::PlusY | Self::MinusY => (Vec3::X, Vec3::Z),
            Self::PlusZ | Self::MinusZ => (Vec3::X, Vec3::Y),
        }
    }

    /// Index used to order faces canonically, so that an edge or corner names
    /// its faces in one fixed order however it was picked.
    fn index(self) -> u8 {
        match self {
            Self::PlusX => 0,
            Self::MinusX => 1,
            Self::PlusY => 2,
            Self::MinusY => 3,
            Self::PlusZ => 4,
            Self::MinusZ => 5,
        }
    }
}

/// What the user aimed at on the orientation cube. A face gives an axis-aligned
/// view (and a work plane); an edge or a corner gives an oblique view, which by
/// definition is not aligned with any plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CubeZone {
    Face(CubeFace),
    Edge([CubeFace; 2]),
    Corner([CubeFace; 3]),
}

impl CubeZone {
    pub fn edge(a: CubeFace, b: CubeFace) -> Self {
        let mut faces = [a, b];
        faces.sort_by_key(|face| face.index());
        Self::Edge(faces)
    }

    pub fn corner(a: CubeFace, b: CubeFace, c: CubeFace) -> Self {
        let mut faces = [a, b, c];
        faces.sort_by_key(|face| face.index());
        Self::Corner(faces)
    }

    /// Direction from the centre of the cube towards this zone: the camera is
    /// placed along it looking back at the origin.
    pub fn direction(self) -> Vec3 {
        let sum = match self {
            Self::Face(face) => face.normal(),
            Self::Edge(faces) => faces.iter().map(|face| face.normal()).sum(),
            Self::Corner(faces) => faces.iter().map(|face| face.normal()).sum(),
        };
        sum.normalize()
    }

    /// The face this zone is, if it is one. An edge or a corner looks at no
    /// single plane, which is why they stay in wireframe mode.
    pub fn face(self) -> Option<CubeFace> {
        match self {
            Self::Face(face) => Some(face),
            Self::Edge(_) | Self::Corner(_) => None,
        }
    }
}

/// Camera (yaw, pitch) that looks from `direction` back at the target. At yaw 0
/// the camera looks along +Y, and yaw turns it towards -X.
pub fn view_angles_towards(direction: Vec3) -> (f32, f32) {
    let direction = direction.normalize_or(Vec3::NEG_Y);
    let pitch = direction.z.clamp(-1.0, 1.0).asin();

    // Looking straight up or down, the yaw is free — and deriving it anyway is
    // a trap: `atan2(0.0, -0.0)` is π, not 0, so a top view came out turned
    // half a turn and every sketch was drawn mirrored.
    let flat = Vec2::new(direction.x, direction.y);
    let yaw = if flat.length_squared() < 1e-12 {
        0.0
    } else {
        f32::atan2(flat.x, -flat.y)
    };
    (yaw, pitch)
}

/// Camera orbiting a target point, parameterised by yaw/pitch/distance rather
/// than a free transform: it can never roll, and `pitch = ±90°` (looking
/// straight down or up) stays well defined, which a `look_at` with a fixed up
/// vector would not.
#[derive(Clone, Copy, Debug)]
pub struct OrbitCamera {
    target: Vec3,
    distance: f32,
    yaw: f32,
    pitch: f32,
    fov_y: f32,
    min_distance: f32,
    max_distance: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 250.0,
            yaw: -PI * 0.7,
            pitch: 0.5,
            fov_y: 45f32.to_radians(),
            min_distance: 1e-3,
            max_distance: 1e9,
        }
    }
}

impl OrbitCamera {
    pub fn target(&self) -> Vec3 {
        self.target
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    /// World-space orientation of the camera: maps camera space (right, up,
    /// backward) onto world axes, with Z as the world up axis.
    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_z(self.yaw) * Quat::from_rotation_x(FRAC_PI_2 - self.pitch)
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation() * Vec3::NEG_Z
    }

    pub fn right(&self) -> Vec3 {
        self.rotation() * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation() * Vec3::Y
    }

    pub fn eye(&self) -> Vec3 {
        self.target - self.forward() * self.distance
    }

    pub fn view(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation(), self.eye()).inverse()
    }

    /// Near/far planes track the orbit distance so precision stays usable
    /// whether we are millimetres or kilometres away from the part.
    pub fn projection(&self, aspect: f32) -> Mat4 {
        let near = (self.distance * 0.001).max(1e-4);
        let far = self.distance * 1000.0;
        directx::perspective(self.fov_y, aspect.max(1e-4), near, far)
    }

    pub fn view_projection(&self, aspect: f32) -> Mat4 {
        self.projection(aspect) * self.view()
    }

    /// World units covered by one pixel at the orbit target's depth.
    pub fn world_units_per_pixel(&self, viewport_height_px: f32) -> f32 {
        2.0 * self.distance * (self.fov_y * 0.5).tan() / viewport_height_px.max(1.0)
    }

    /// `delta` is a mouse movement in pixels; dragging moves the part, so the
    /// camera goes the opposite way.
    pub fn orbit(&mut self, delta: Vec2, sensitivity: f32) {
        self.yaw -= delta.x * sensitivity;
        self.pitch = (self.pitch + delta.y * sensitivity).clamp(-FRAC_PI_2, FRAC_PI_2);
        self.yaw = self.yaw.rem_euclid(TAU);
    }

    pub fn pan(&mut self, delta: Vec2, viewport_height_px: f32) {
        let scale = self.world_units_per_pixel(viewport_height_px);
        self.target += (-self.right() * delta.x + self.up() * delta.y) * scale;
    }

    /// How far the camera may travel. A hard bound has to exist: positions are
    /// f32, and beyond a few million units the precision of the view falls
    /// apart.
    pub fn set_distance_limits(&mut self, min: f32, max: f32) {
        self.min_distance = min.max(1e-6);
        self.max_distance = max.max(self.min_distance);
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
    }

    /// Exponential zoom so each notch feels the same at every scale.
    pub fn zoom(&mut self, amount: f32, sensitivity: f32) {
        self.zoom_by_factor((amount * sensitivity).exp());
    }

    /// Zoom expressed as a direct scale factor, as a pinch gesture reports it:
    /// a factor above 1 brings the part closer.
    pub fn zoom_by_factor(&mut self, factor: f32) {
        if factor <= 0.0 {
            return;
        }
        self.distance = (self.distance / factor).clamp(self.min_distance, self.max_distance);
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    /// Frames a sphere: looks at its centre from far enough back that it fits
    /// the narrower of the two field-of-view angles, with a little margin.
    pub fn focus_on(&mut self, center: Vec3, radius: f32, aspect: f32) {
        self.target = center;
        let half_vertical = self.fov_y * 0.5;
        let half_horizontal = (half_vertical.tan() * aspect.max(1e-3)).atan();
        let half_angle = half_vertical.min(half_horizontal).max(1e-3);
        self.distance = (radius.max(1e-4) / half_angle.sin() * 1.25)
            .clamp(self.min_distance, self.max_distance);
    }

    /// The world-space ray under a point of the viewport, given in normalized
    /// device coordinates (-1..1, y up). Used to pick what the cursor is over.
    pub fn ray(&self, ndc: Vec2, aspect: f32) -> (Vec3, Vec3) {
        let inverse = self.view_projection(aspect).inverse();
        let unproject = |depth: f32| {
            let point = inverse * glam::Vec4::new(ndc.x, ndc.y, depth, 1.0);
            point.truncate() / point.w
        };
        // wgpu clip space puts the near plane at depth 0 and the far plane at 1.
        let near = unproject(0.0);
        let far = unproject(1.0);
        (near, (far - near).normalize_or(self.forward()))
    }

    pub fn set_view_angles(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw.rem_euclid(TAU);
        self.pitch = pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
    }
}

/// Animated move to a standard view, so clicking the orientation cube reads as
/// a rotation rather than a teleport.
#[derive(Clone, Copy, Debug)]
pub struct ViewTransition {
    from_yaw: f32,
    from_pitch: f32,
    to_yaw: f32,
    to_pitch: f32,
    elapsed: f32,
    duration: f32,
}

impl ViewTransition {
    pub fn to_zone(camera: &OrbitCamera, zone: CubeZone) -> Self {
        let (yaw, pitch) = view_angles_towards(zone.direction());
        Self::to_angles(camera, yaw, pitch)
    }

    pub fn to_angles(camera: &OrbitCamera, to_yaw: f32, to_pitch: f32) -> Self {
        let from_yaw = camera.yaw();
        Self {
            from_yaw,
            from_pitch: camera.pitch(),
            to_yaw: from_yaw + shortest_angle_delta(from_yaw, to_yaw),
            to_pitch,
            elapsed: 0.0,
            duration: 0.35,
        }
    }

    /// Advances by `dt` seconds and writes the interpolated view into
    /// `camera`. Returns false once the transition is over.
    pub fn advance(&mut self, camera: &mut OrbitCamera, dt: f32) -> bool {
        self.elapsed += dt;
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
        let eased = ease_in_out(t);
        camera.set_view_angles(
            self.from_yaw + (self.to_yaw - self.from_yaw) * eased,
            self.from_pitch + (self.to_pitch - self.from_pitch) * eased,
        );
        t < 1.0
    }
}

fn shortest_angle_delta(from: f32, to: f32) -> f32 {
    let diff = (to - from).rem_euclid(TAU);
    if diff > PI { diff - TAU } else { diff }
}

fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) * 0.5
    }
}

#[cfg(test)]
mod tests;
