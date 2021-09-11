use bevy::prelude::*;

/// Saves the handles of all textures needed for styling tiles
#[derive(Default, Clone)]
pub struct TileResources {
    pub empty: TileResource,
    pub full: TileResource,
    pub hover: TileResource,
    pub invalid: TileResource,
    pub scored: TileResource,
}

#[derive(Default, Clone)]
pub struct TileResource {
    pub texture: Handle<Texture>,
    pub material: Handle<ColorMaterial>,
}

impl TileResource {
    pub fn new(tex_mat: (Handle<Texture>, Handle<ColorMaterial>)) -> Self {
        Self {
            texture: tex_mat.0.clone(),
            material: tex_mat.1.clone(),
        }
    }
}
