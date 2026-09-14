use crate::{SceneFrame, SceneRenderer};

/// Draws a frame into an image, with no window and no surface.
///
/// Every pipeline of `SceneRenderer` is built with a depth test, so the pass
/// has to carry a depth buffer: without one, `wgpu` refuses the very first
/// `set_pipeline` and nothing is drawn at all.
///
/// Hands back the rows one after another, four bytes a pixel, red first — what
/// an image file wants, and what `egui` takes to make a texture.
pub fn draw(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    renderer: &mut SceneRenderer,
    frame: &SceneFrame,
    size: Size,
    background: wgpu::Color,
) -> Vec<u8> {
    let extent = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };
    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("offscreen_target"),
        size: extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: size.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let depth = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("offscreen_depth"),
        size: extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: SceneRenderer::DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let tight = size.width * 4;
    let padded = padded_row(size.width);
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("offscreen_readback"),
        size: u64::from(padded) * u64::from(size.height),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    renderer.prepare(device, queue, frame);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("offscreen_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target.create_view(&wgpu::TextureViewDescriptor::default()),
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(background),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth.create_view(&wgpu::TextureViewDescriptor::default()),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Discard,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        renderer.paint(&mut pass);
    }

    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &target,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded),
                rows_per_image: Some(size.height),
            },
        },
        extent,
    );
    queue.submit([encoder.finish()]);

    readback.slice(..).map_async(wgpu::MapMode::Read, |mapped| {
        mapped.expect("the readback buffer is mapped");
    });
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("the queue drains");

    let mapped = readback
        .slice(..)
        .get_mapped_range()
        .expect("the readback buffer is mapped");
    let mut pixels = Vec::with_capacity((tight * size.height) as usize);
    for row in 0..size.height {
        let start = (row * padded) as usize;
        pixels.extend_from_slice(&mapped[start..start + tight as usize]);
    }
    pixels
}

/// How wide a row of the copy is, in bytes.
///
/// A copy out of a texture wants its rows on a 256-byte boundary, whatever the
/// picture is wide; the padding comes back with the pixels and is dropped on
/// the way out. A width that already lands on the boundary is left alone — an
/// unconditional round-up would add a whole empty row of padding to it.
fn padded_row(width: u32) -> u32 {
    let tight = width * 4;
    tight.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
}

/// How big a picture is wanted, and in which arrangement of colours.
#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
    pub format: wgpu::TextureFormat,
}

impl Size {
    /// The rows of the image this describes, four bytes a pixel.
    pub fn bytes(&self) -> usize {
        (self.width * self.height * 4) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_already_on_the_boundary_is_left_where_it_is() {
        assert_eq!(padded_row(64), 256);
        assert_eq!(padded_row(128), 512);
        assert_eq!(padded_row(1024), 4096);
    }

    #[test]
    fn a_row_that_falls_short_is_carried_to_the_next_boundary() {
        assert_eq!(padded_row(1), 256, "four bytes still occupy a whole row");
        assert_eq!(padded_row(65), 512);
        assert_eq!(padded_row(100), 512);
    }

    #[test]
    fn a_padded_row_is_never_narrower_than_the_pixels_it_carries() {
        for width in 1..600u32 {
            assert!(
                padded_row(width) >= width * 4,
                "a row of {width} pixels is copied into fewer bytes than it holds",
            );
            assert_eq!(padded_row(width) % wgpu::COPY_BYTES_PER_ROW_ALIGNMENT, 0);
        }
    }

    #[test]
    fn a_picture_says_how_many_bytes_its_pixels_make() {
        let size = Size {
            width: 128,
            height: 128,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
        };

        assert_eq!(size.bytes(), 65_536);
    }
}
