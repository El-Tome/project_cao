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

    /// The work plane a sketch would sit on when looking straight at this face.
    pub fn plane(self) -> GridPlane {
        match self {
            Self::PlusX | Self::MinusX => GridPlane::Yz,
            Self::PlusY | Self::MinusY => GridPlane::Xz,
            Self::PlusZ | Self::MinusZ => GridPlane::Xy,
        }
    }

    /// Camera (yaw, pitch) that looks straight at this face, i.e. whose
    /// forward direction is the face normal reversed. At yaw 0 the camera
    /// looks along +Y, and yaw turns it towards -X.
    fn view_angles(self) -> (f32, f32) {
        match self {
            Self::MinusY => (0.0, 0.0),
            Self::PlusX => (FRAC_PI_2, 0.0),
            Self::PlusY => (PI, 0.0),
            Self::MinusX => (-FRAC_PI_2, 0.0),
            Self::PlusZ => (0.0, FRAC_PI_2),
            Self::MinusZ => (0.0, -FRAC_PI_2),
        }
    }
}

/// The plane a grid is drawn on, named after the two axes it contains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridPlane {
    Xy,
    Xz,
    Yz,
}

impl GridPlane {
    /// The two in-plane basis vectors, and the plane normal.
    pub fn basis(self) -> (Vec3, Vec3, Vec3) {
        match self {
            Self::Xy => (Vec3::X, Vec3::Y, Vec3::Z),
            Self::Xz => (Vec3::X, Vec3::Z, Vec3::Y),
            Self::Yz => (Vec3::Y, Vec3::Z, Vec3::X),
        }
    }
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
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 250.0,
            yaw: -PI * 0.7,
            pitch: 0.5,
            fov_y: 45f32.to_radians(),
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

    /// Exponential zoom so each notch feels the same at every scale.
    pub fn zoom(&mut self, scroll: f32, sensitivity: f32) {
        self.distance = (self.distance * (-scroll * sensitivity).exp()).clamp(1e-3, 1e7);
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
    pub fn to_face(camera: &OrbitCamera, face: CubeFace) -> Self {
        let (yaw, pitch) = face.view_angles();
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
mod tests {
    use super::*;

    #[test]
    fn face_views_look_straight_at_the_face() {
        for face in CubeFace::ALL {
            let mut camera = OrbitCamera::default();
            let mut transition = ViewTransition::to_face(&camera, face);
            while transition.advance(&mut camera, 0.1) {}

            let forward = camera.forward();
            assert!(
                (forward + face.normal()).length() < 1e-4,
                "{face:?}: looking along {forward:?}, expected {:?}",
                -face.normal()
            );
        }
    }

    #[test]
    fn top_and_bottom_views_stay_well_defined() {
        for pitch in [FRAC_PI_2, -FRAC_PI_2] {
            let mut camera = OrbitCamera::default();
            camera.set_view_angles(0.0, pitch);
            assert!(camera.view().is_finite());
            assert!((camera.up().length() - 1.0).abs() < 1e-5);
        }
    }

    #[test]
    fn orbit_cannot_flip_past_the_poles() {
        let mut camera = OrbitCamera::default();
        camera.orbit(Vec2::new(0.0, 10_000.0), 0.01);
        assert!(camera.pitch() <= FRAC_PI_2 + 1e-6);
        camera.orbit(Vec2::new(0.0, -20_000.0), 0.01);
        assert!(camera.pitch() >= -FRAC_PI_2 - 1e-6);
    }

    #[test]
    fn zoom_is_symmetric_and_bounded() {
        let mut camera = OrbitCamera::default();
        let start = camera.distance();
        camera.zoom(100.0, 0.0015);
        camera.zoom(-100.0, 0.0015);
        assert!((camera.distance() - start).abs() < 1e-2);

        camera.zoom(1e6, 0.0015);
        assert!(camera.distance() >= 1e-3);
    }
}
