use std::path::PathBuf;

use bevy::{asset::LoadState, prelude::*};
use bevy_asset_ron::RonAssetPlugin;

use crate::{
    assets::{PreloadingAssets, SettingsAsset},
    prelude::*,
    resources::{BlockResource, BlockResources},
    states::GameState,
};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<PreloadingAssets>()
            .init_resource::<BlockResources>()
            .init_resource::<Handle<SettingsAsset>>()
            .add_asset::<Pattern>()
            .init_asset_loader::<PatternLoader>()
            .add_plugin(RonAssetPlugin::<SettingsAsset>::new(&["rfg"]))
            .add_system_set(
                // Load setup
                SystemSet::on_enter(GameState::Load).with_system(load_assets_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Load)
                    .with_system(assets_loaded_transition_system.system()),
            );
    }
}

/// Track any loading assets and transition to the next game state when ready
fn assets_loaded_transition_system(
    loading: Res<PreloadingAssets>,
    assets: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
) {
    if loading
        .0
        .iter()
        .filter(|h| assets.get_load_state(*h) == LoadState::Loading)
        .count()
        == 0
    {
        // We are done loading! transition state.
        state.set(GameState::Menu).ok();
    }
}

fn load_assets_system(
    assets: Res<AssetServer>,
    mut loading: ResMut<PreloadingAssets>,
    mut settings_handle: ResMut<Handle<SettingsAsset>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut style: ResMut<BlockResources>,
) {
    // load our settings file
    *settings_handle = assets.load("settings.rfg");
    loading.0.push(settings_handle.clone_untyped());

    // load all block patterns
    let patterns = &mut assets.load_folder("blocks").unwrap();
    loading.0.append(patterns);

    // load textures
    let mut load_tex = |path: &'static str| {
        let texture: Handle<Texture> = assets.load(PathBuf::from(format!("sprites/{}.png", path)));
        loading.0.push(texture.clone_untyped());
        let material: Handle<ColorMaterial> = materials.add(texture.clone().into());
        (texture.clone(), material.clone())
    };

    *style = BlockResources {
        empty: BlockResource::new(load_tex("empty")),
        full: BlockResource::new(load_tex("full")),
        hover: BlockResource::new(load_tex("hover")),
        invalid: BlockResource::new(load_tex("invalid")),
        scored: BlockResource::new(load_tex("scored")),
    };

    assets
        .watch_for_changes()
        .expect("could not watch for changes");
}
