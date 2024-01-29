use bevy::render::texture::Image;
use buffer_graphics_lib::{Graphics, GraphicsError};
use image::{DynamicImage, RgbaImage};

pub struct BufferedImage {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl BufferedImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: iter::repeat(0).take(width * height * 4).collect(),
            width,
            height,
        }
    }

    pub fn graphics(&mut self) -> Result<Graphics, GraphicsError> {
        Graphics::new(&mut self.buffer, self.width, self.height)
    }

    pub fn image(&self) -> Image {
        Image::from_dynamic(
            DynamicImage::ImageRgba8(
                RgbaImage::from_raw(self.width as u32, self.height as u32, self.buffer.clone())
                    .unwrap(),
            ),
            true,
        )
    }
}
