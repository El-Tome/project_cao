//! Renders the viewport to PNG files without opening a window, so the scene
//! can be inspected (and diffed) without a GPU-enabled test harness.
//!
//! `cargo run -p cao_render --example offscreen -- <output directory>`

use std::path::PathBuf;

use cao_render::camera::{CubeFace, CubeZone};
use cao_render::{
    AxisStyle, GridStyle, OrbitCamera, SceneFrame, SceneRenderer, ViewportRect, adaptive_step,
    cube, push_axes, push_grid,
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

fn main() {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let (device, queue) = pollster::block_on(request_device());
    let mut renderer = SceneRenderer::new(&device, FORMAT, 1);

    let free_view = OrbitCamera::default();
    render(
        &device,
        &queue,
        &mut renderer,
        &build_frame(&free_view, None),
        &out_dir.join("viewport_free.png"),
    );

    let mut plane_view = OrbitCamera::default();
    plane_view.set_view_angles(0.0, std::f32::consts::FRAC_PI_2);
    render(
        &device,
        &queue,
        &mut renderer,
        &build_frame(&plane_view, Some((glam::Vec3::X, glam::Vec3::Y))),
        &out_dir.join("viewport_plane_xy.png"),
    );

    // The plane chooser: three translucent patches, one under the cursor.
    let chooser = OrbitCamera::default();
    let mut frame = build_frame(&chooser, None);
    let half_size = chooser.distance() * 0.3;
    for (index, (u, v)) in [
        (glam::Vec3::X, glam::Vec3::Y),
        (glam::Vec3::X, glam::Vec3::Z),
        (glam::Vec3::Y, glam::Vec3::Z),
    ]
    .into_iter()
    .enumerate()
    {
        let hovered = index == 0;
        let fill = if hovered {
            cao_render::srgb(0.30, 0.60, 0.95, 0.35)
        } else {
            cao_render::srgb(0.55, 0.60, 0.68, 0.12)
        };
        let outline = if hovered {
            cao_render::srgb(0.45, 0.75, 1.0, 1.0)
        } else {
            cao_render::srgb(0.65, 0.70, 0.78, 0.7)
        };
        cao_render::push_plane_quad(
            &mut frame.scene_surfaces,
            glam::Vec3::ZERO,
            u,
            v,
            half_size,
            fill,
        );
        cao_render::push_plane_outline(
            &mut frame.scene_lines,
            glam::Vec3::ZERO,
            u,
            v,
            half_size,
            outline,
            if hovered { 2.5 } else { 1.5 },
        );
    }
    render(
        &device,
        &queue,
        &mut renderer,
        &frame,
        &out_dir.join("viewport_plane_chooser.png"),
    );

    // Panned away from the origin: the heavy lines must stay on the axes.
    let mut panned = OrbitCamera::default();
    panned.set_view_angles(0.0, std::f32::consts::FRAC_PI_2);
    panned.pan(glam::Vec2::new(-260.0, 170.0), HEIGHT as f32);
    render(
        &device,
        &queue,
        &mut renderer,
        &build_frame(&panned, Some((glam::Vec3::X, glam::Vec3::Y))),
        &out_dir.join("viewport_plane_panned.png"),
    );

    // Grazing angles: the axes must stay visible however the camera is turned.
    for (index, (yaw, pitch)) in [(0.05_f32, 0.02_f32), (0.8, 0.05), (1.55, 0.6), (2.4, 0.9)]
        .into_iter()
        .enumerate()
    {
        let mut camera = OrbitCamera::default();
        camera.set_view_angles(yaw, pitch);
        render(
            &device,
            &queue,
            &mut renderer,
            &build_frame(&camera, None),
            &out_dir.join(format!("viewport_grazing_{index}.png")),
        );
    }

    let mut hovered_view = OrbitCamera::default();
    hovered_view.set_view_angles(-0.9, 0.5);
    let mut frame = build_frame(&hovered_view, None);
    frame.cube_triangles.clear();
    cube::push_faces(
        &mut frame.cube_triangles,
        Some(CubeZone::corner(
            CubeFace::PlusZ,
            CubeFace::MinusY,
            CubeFace::PlusX,
        )),
    );
    render(
        &device,
        &queue,
        &mut renderer,
        &frame,
        &out_dir.join("viewport_cube_hover.png"),
    );
}

async fn request_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("no GPU adapter available");
    adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("could not create device")
}

fn build_frame(camera: &OrbitCamera, plane: Option<(glam::Vec3, glam::Vec3)>) -> SceneFrame {
    let mut lines = Vec::new();

    if let Some((u, v)) = plane {
        let units_per_pixel = camera.world_units_per_pixel(HEIGHT as f32);
        let step = adaptive_step(units_per_pixel, 48.0);
        let diagonal = ((WIDTH * WIDTH + HEIGHT * HEIGHT) as f32).sqrt();
        let half_extent = units_per_pixel * diagonal * 1.5;
        let normal = u.cross(v).normalize();
        let center = camera.target() - normal * camera.target().dot(normal);
        push_grid(
            &mut lines,
            u,
            v,
            center,
            step,
            half_extent,
            &GridStyle::default(),
        );
    }

    push_axes(
        &mut lines,
        camera.distance() * 50.0,
        &AxisStyle::default(),
        None,
    );

    let mut cube_triangles = Vec::new();
    let mut cube_edges = Vec::new();
    cube::push_faces(&mut cube_triangles, None);
    cube::push_edges(&mut cube_edges, camera.forward(), 1.5);

    let cube_size = 288.0;
    SceneFrame {
        scene_surfaces: Vec::new(),
        scene_view_projection: camera.view_projection(WIDTH as f32 / HEIGHT as f32),
        scene_viewport: ViewportRect {
            x: 0.0,
            y: 0.0,
            width: WIDTH as f32,
            height: HEIGHT as f32,
        },
        scene_background: Vec::new(),
        scene_world_lines: Vec::new(),
        scene_solids: Vec::new(),
        scene_lines: lines,
        cube_view_projection: cube::view_projection(camera.rotation()),
        cube_triangles,
        cube_edges,
        cube_viewport: ViewportRect {
            x: WIDTH as f32 - cube_size - 16.0,
            y: 16.0,
            width: cube_size,
            height: cube_size,
        },
    }
}

fn render(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    renderer: &mut SceneRenderer,
    frame: &SceneFrame,
    path: &std::path::Path,
) {
    let pixels = cao_render::draw(
        device,
        queue,
        renderer,
        frame,
        cao_render::Size {
            width: WIDTH,
            height: HEIGHT,
            format: FORMAT,
        },
        wgpu::Color {
            r: 0.11,
            g: 0.12,
            b: 0.14,
            a: 1.0,
        },
    );
    image::save_buffer(path, &pixels, WIDTH, HEIGHT, image::ColorType::Rgba8)
        .expect("failed to write PNG");
    println!("wrote {}", path.display());
}
