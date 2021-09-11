use bevy::prelude::*;

/// Saves the handles of all textures we will need
#[derive(Default, Clone)]
pub struct BlockResources {
    pub empty: BlockResource,
    pub full: BlockResource,
    pub hover: BlockResource,
    pub invalid: BlockResource,
    pub scored: BlockResource,
}

#[derive(Default, Clone)]
pub struct BlockResource {
    pub texture: Handle<Texture>,
    pub material: Handle<ColorMaterial>,
}

impl BlockResource {
    pub fn new(tex_mat: (Handle<Texture>, Handle<ColorMaterial>)) -> Self {
        Self {
            texture: tex_mat.0.clone(),
            material: tex_mat.1.clone(),
        }
    }
}
