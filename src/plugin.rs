use bevy::{
    app::{First, Plugin},
    asset::{AssetServer, Handle},
    ecs::system::{Query, Res},
    render::texture::Image,
};

use crate::buffer::BufferedImage;

pub struct BufferedImageUpdatePlugin;

impl BufferedImageUpdatePlugin {
    pub fn update_images(
        asset_server: Res<AssetServer>,
        mut query: Query<(&mut BufferedImage, &mut Handle<Image>)>,
    ) {
        query.for_each_mut(|(mut buffered_image, mut image_handle)| {
            if buffered_image.dirty() {
                let image = buffered_image.image();
                *image_handle = asset_server.add(image);
            }
        });
    }
}

impl Plugin for BufferedImageUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(First, Self::update_images);
    }
}
