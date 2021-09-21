use std::path::PathBuf;

use bevy::{asset::LoadState, prelude::*};
use bevy_asset_ron::RonAssetPlugin;

use crate::{
    assets::SettingsAsset,
    prelude::*,
    resources::{TileResource, TileResources},
    GameState, PreloadingAssets,
};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<TileResources>()
            .init_resource::<PreloadingAssets>()
            .init_resource::<Handle<SettingsAsset>>()
            .add_asset::<Pattern>()
            .add_asset::<GameMode>()
            .add_asset::<Map>()
            .add_plugin(RonAssetPlugin::<GameMode>::new(&["mode"]))
            .add_plugin(RonAssetPlugin::<Map>::new(&["map"]))
            .add_plugin(RonAssetPlugin::<SettingsAsset>::new(&["rfg"]))
            .init_asset_loader::<PatternLoader>()
            .add_system_set(
                // Load setup
                SystemSet::on_enter(GameState::load()).with_system(init_load_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::load())
                    .with_system(assets_loaded_transition_system.system()),
            );
    }
}

fn init_load_system(
    mut state: ResMut<State<GameState>>,
    mut settings_handle: ResMut<Handle<SettingsAsset>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut style: ResMut<TileResources>,
    mut loading: ResMut<PreloadingAssets>,
    assets: Res<AssetServer>,
) {
    // load our settings file
    *settings_handle = assets.load("settings.rfg");

    loading.0.push(settings_handle.clone_untyped());

    // load all block patterns
    let patterns = &mut assets
        .load_folder("blocks")
        .expect("Could not load patterns");
    loading.0.append(patterns);

    // load textures
    let mut load_tex = |path: &'static str| {
        let texture: Handle<Texture> = assets.load(PathBuf::from(format!("sprites/{}.png", path)));
        loading.0.push(texture.clone_untyped());
        let material: Handle<ColorMaterial> = materials.add(texture.clone().into());
        (texture.clone(), material.clone())
    };

    *style = TileResources {
        empty: TileResource::new(load_tex("empty")),
        full: TileResource::new(load_tex("full")),
        hover: TileResource::new(load_tex("hover")),
        invalid: TileResource::new(load_tex("invalid")),
        scored: TileResource::new(load_tex("scored")),
    };

    // load game modes
    let mode_handles = &mut assets.load_folder("modes").expect("Could not load modes");
    loading.0.append(mode_handles);

    // load maps
    let map_handles = &mut assets.load_folder("maps").expect("Could not load maps");
    loading.0.append(map_handles);

    assets
        .watch_for_changes()
        .expect("could not watch for changes");

    // add all loading to state
    state.set(GameState::Load).ok();
}

/// Track any loading assets and transition to the next game state when ready
fn assets_loaded_transition_system(
    mut state: ResMut<State<GameState>>,
    loading: Res<PreloadingAssets>,
    assets: Res<AssetServer>,
) {
    if loading
        .0
        .iter()
        .filter(|h| assets.get_load_state(*h) == LoadState::Loading)
        .count()
        == 0
    {
        // Transition states to the men
        state.set(GameState::Menu).ok();
    }
}
