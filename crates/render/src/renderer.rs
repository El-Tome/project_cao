use bytemuck::{Pod, Zeroable};
use glam::Mat4;

use crate::geometry::Vertex;

/// A rectangle in physical pixels, as `wgpu`'s viewport expects it.
#[derive(Clone, Copy, Debug, Default)]
pub struct ViewportRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Everything the renderer needs for one frame. Built on the CPU by the UI
/// layer, so this crate never has to know about egui or input handling.
pub struct SceneFrame {
    pub scene_view_projection: Mat4,
    pub scene_viewport: ViewportRect,
    pub scene_lines: Vec<Vertex>,
    pub cube_view_projection: Mat4,
    pub cube_triangles: Vec<Vertex>,
    pub cube_edges: Vec<Vertex>,
    pub cube_viewport: ViewportRect,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Uniforms {
    view_projection: [[f32; 4]; 4],
    params: [f32; 4],
}

struct UniformBinding {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl UniformBinding {
    fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &str) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Self { buffer, bind_group }
    }

    fn write(
        &self,
        queue: &wgpu::Queue,
        view_projection: Mat4,
        viewport: ViewportRect,
        encode_srgb: bool,
    ) {
        let uniforms = Uniforms {
            view_projection: view_projection.to_cols_array_2d(),
            params: [
                if encode_srgb { 1.0 } else { 0.0 },
                viewport.width,
                viewport.height,
                0.0,
            ],
        };
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniforms));
    }
}

/// Vertex buffer that grows to fit whatever the frame needs and is reused
/// across frames otherwise.
struct DynamicVertexBuffer {
    buffer: wgpu::Buffer,
    capacity: u64,
    len: u32,
    label: &'static str,
}

impl DynamicVertexBuffer {
    const INITIAL_CAPACITY: u64 = 4096;

    fn new(device: &wgpu::Device, label: &'static str) -> Self {
        Self {
            buffer: Self::allocate(device, label, Self::INITIAL_CAPACITY),
            capacity: Self::INITIAL_CAPACITY,
            len: 0,
            label,
        }
    }

    fn allocate(device: &wgpu::Device, label: &'static str, capacity: u64) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: capacity * size_of::<Vertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, vertices: &[Vertex]) {
        self.len = vertices.len() as u32;
        if vertices.is_empty() {
            return;
        }
        if vertices.len() as u64 > self.capacity {
            self.capacity = (vertices.len() as u64).next_power_of_two();
            self.buffer = Self::allocate(device, self.label, self.capacity);
        }
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(vertices));
    }

    /// Draws the buffer as triangles, one vertex each.
    fn draw_triangles(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.len == 0 {
            return;
        }
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(0..self.len, 0..1);
    }

    /// Draws the buffer as lines: every pair of vertices is one instance, and
    /// the shader expands it into a six-vertex quad.
    fn draw_lines(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.len < 2 {
            return;
        }
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(0..6, 0..self.len / 2);
    }
}

/// Draws the viewport: world axes and grid as lines, plus the orientation cube
/// in its own corner viewport.
///
/// No depth buffer is involved. The scene is only lines, and the cube is a
/// convex solid drawn last, so back-face culling alone resolves it correctly.
pub struct SceneRenderer {
    line_pipeline: wgpu::RenderPipeline,
    triangle_pipeline: wgpu::RenderPipeline,
    scene_uniform: UniformBinding,
    cube_uniform: UniformBinding,
    scene_lines: DynamicVertexBuffer,
    cube_triangles: DynamicVertexBuffer,
    cube_edges: DynamicVertexBuffer,
    cube_viewport: ViewportRect,
    encode_srgb: bool,
}

impl SceneRenderer {
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cao_scene_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/scene.wgsl").into()),
        });

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cao_scene_uniforms"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cao_scene_pipeline_layout"),
            bind_group_layouts: &[Some(&uniform_layout)],
            immediate_size: 0,
        });

        // One line segment per instance: the two endpoints are read from a
        // single stride, so the geometry builders can keep emitting plain
        // vertex pairs.
        let line_layout = wgpu::VertexBufferLayout {
            array_stride: 2 * size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![
                0 => Float32x3,
                1 => Float32x4,
                2 => Float32,
                3 => Float32x3,
                4 => Float32x4,
                5 => Float32,
            ],
        };
        let solid_layout = wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4],
        };

        let make_pipeline =
            |label: &str, entry_point, layout: &wgpu::VertexBufferLayout, cull_mode| {
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some(entry_point),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[Some(layout.clone())],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode,
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: sample_count,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: target_format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview_mask: None,
                    cache: None,
                })
            };

        Self {
            line_pipeline: make_pipeline("cao_line_pipeline", "vs_line", &line_layout, None),
            triangle_pipeline: make_pipeline(
                "cao_triangle_pipeline",
                "vs_solid",
                &solid_layout,
                Some(wgpu::Face::Back),
            ),
            scene_uniform: UniformBinding::new(device, &uniform_layout, "cao_scene_uniform"),
            cube_uniform: UniformBinding::new(device, &uniform_layout, "cao_cube_uniform"),
            scene_lines: DynamicVertexBuffer::new(device, "cao_scene_lines"),
            cube_triangles: DynamicVertexBuffer::new(device, "cao_cube_triangles"),
            cube_edges: DynamicVertexBuffer::new(device, "cao_cube_edges"),
            cube_viewport: ViewportRect::default(),
            encode_srgb: !target_format.is_srgb(),
        }
    }

    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, frame: &SceneFrame) {
        self.scene_uniform.write(
            queue,
            frame.scene_view_projection,
            frame.scene_viewport,
            self.encode_srgb,
        );
        self.cube_uniform.write(
            queue,
            frame.cube_view_projection,
            frame.cube_viewport,
            self.encode_srgb,
        );
        self.scene_lines.upload(device, queue, &frame.scene_lines);
        self.cube_triangles
            .upload(device, queue, &frame.cube_triangles);
        self.cube_edges.upload(device, queue, &frame.cube_edges);
        self.cube_viewport = frame.cube_viewport;
    }

    pub fn paint(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.line_pipeline);
        pass.set_bind_group(0, &self.scene_uniform.bind_group, &[]);
        self.scene_lines.draw_lines(pass);

        if self.cube_viewport.width < 1.0 || self.cube_viewport.height < 1.0 {
            return;
        }

        pass.set_viewport(
            self.cube_viewport.x,
            self.cube_viewport.y,
            self.cube_viewport.width,
            self.cube_viewport.height,
            0.0,
            1.0,
        );
        pass.set_bind_group(0, &self.cube_uniform.bind_group, &[]);
        pass.set_pipeline(&self.triangle_pipeline);
        self.cube_triangles.draw_triangles(pass);
        pass.set_pipeline(&self.line_pipeline);
        self.cube_edges.draw_lines(pass);
    }
}
