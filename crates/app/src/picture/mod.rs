pub mod scene;

use cao_part::{PartDocument, Picture};
use cao_render::SceneRenderer;

/// How wide a picture of a part is, in pixels.
///
/// Small on purpose: it is read at a glance beside a name, and every part in a
/// folder carries one. At this size a picture is 64 KB of pixels and about 3 KB
/// once the archive has deflated it.
pub const SIDE: u32 = 128;

/// What the picture is drawn over: the ground of the viewport, so a part put
/// away looks like the part that was being drawn.
const GROUND: wgpu::Color = wgpu::Color {
    r: 0.11,
    g: 0.12,
    b: 0.14,
    a: 1.0,
};

/// The device the pictures are drawn with, kept from start-up.
///
/// Held rather than asked for each time because the picture is taken as the
/// part is put away — on the way out of the application, where there is no
/// frame left to ask.
pub struct Painter {
    device: wgpu::Device,
    queue: wgpu::Queue,
    renderer: SceneRenderer,
}

impl Painter {
    /// Nothing when the platform gave no GPU to draw with: the part is then
    /// put away without a picture, which is a part like any older one.
    pub fn new(render_state: Option<&egui_wgpu::RenderState>) -> Option<Self> {
        let state = render_state?;
        Some(Self {
            device: state.device.clone(),
            queue: state.queue.clone(),
            renderer: SceneRenderer::new(&state.device, FORMAT, 1),
        })
    }

    /// Takes the part's picture, as the default 3D view shows it.
    pub fn take(&mut self, document: &PartDocument) -> Option<Picture> {
        let frame = scene::of(document, SIDE);
        let pixels = cao_render::draw(
            &self.device,
            &self.queue,
            &mut self.renderer,
            &frame,
            cao_render::Size {
                width: SIDE,
                height: SIDE,
                format: FORMAT,
            },
            GROUND,
        );
        Picture::new(SIDE, SIDE, pixels)
    }
}

/// Four bytes a pixel, red first, and the shader's own understanding of what a
/// colour means — the same arrangement the window is drawn in.
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
