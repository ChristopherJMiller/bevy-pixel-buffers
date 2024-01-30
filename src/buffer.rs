use std::iter;

use bevy::{ecs::component::Component, render::texture::Image};
use buffer_graphics_lib::{Graphics, GraphicsError};
use image::{DynamicImage, RgbaImage};

#[derive(Component)]
pub struct BufferedImage {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
    dirty: bool,
}

impl BufferedImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: iter::repeat(0).take(width * height * 4).collect(),
            width,
            height,
            dirty: false,
        }
    }

    pub fn graphics(&mut self) -> Result<Graphics, GraphicsError> {
        self.dirty = true;
        Graphics::new(&mut self.buffer, self.width, self.height)
    }

    pub fn image(&mut self) -> Image {
        self.dirty = false;
        Image::from_dynamic(
            DynamicImage::ImageRgba8(
                RgbaImage::from_raw(self.width as u32, self.height as u32, self.buffer.clone())
                    .unwrap(),
            ),
            true,
        )
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }
}
