/// A picture of the part, as the 3D view shows it.
///
/// Rows one after another, four bytes a pixel, red first — what a renderer
/// hands back and what an interface takes to make a texture. No image format:
/// the archive already deflates what it holds, and a rendering is mostly flat
/// ground with a few strokes on it, which is what deflating is good at. A
/// codec on the way in and out would buy nothing and cost a dependency.
///
/// Kept in the file rather than drawn when a folder is listed. A `.caopart`
/// holds no geometry — it is rebuilt by replaying the history — so drawing one
/// while listing means replaying every history in the folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Picture {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Picture {
    /// Nothing when the pixels do not fill the size claimed: a picture read
    /// back short is not a picture, and an interface handed one would draw
    /// whatever sat next to it in memory.
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Option<Self> {
        let wanted = usize::try_from(width).ok()? * usize::try_from(height).ok()? * 4;
        (wanted > 0 && pixels.len() == wanted).then_some(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

#[cfg(test)]
mod tests;
