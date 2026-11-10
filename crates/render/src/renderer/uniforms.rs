//! What the shaders read besides the vertices: the view and the viewport.

use bytemuck::{Pod, Zeroable};
use glam::Mat4;

use super::ViewportRect;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Uniforms {
    view_projection: [[f32; 4]; 4],
    params: [f32; 4],
}

pub(super) struct UniformBinding {
    buffer: wgpu::Buffer,
    pub(super) bind_group: wgpu::BindGroup,
}

impl UniformBinding {
    pub(super) fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &str) -> Self {
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

    pub(super) fn write(
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
