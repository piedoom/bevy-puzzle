use std::path::PathBuf;

use crate::{assets::UserPreferencesAsset, prelude::*};
use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::AudioSource;

pub struct AssetPlugin;

/// Our loading will take multiple stages since some assets have dependencies to make developing easier
/// We can specify the current stage here and keep it in state
#[derive(Default)]
pub struct Stage(usize);

impl Stage {
    #[inline(always)]
    pub fn current(&self) -> usize {
        self.0
    }
    #[inline(always)]
    pub fn next(&mut self) {
        self.0 += 1
    }
}

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreloadingAssets>()
            .init_resource::<Handle<UserPreferencesAsset>>()
            .init_resource::<Themes>()
            .init_resource::<Campaigns>()
            .init_resource::<Stage>()
            .add_asset::<AssetManifest>()
            .add_asset::<ThemeDescription>()
            .add_asset::<Pattern>()
            .add_asset::<Map>()
            .add_asset::<CampaignDescription>()
            .add_asset::<Save>()
            .add_plugin(RonAssetPlugin::<Map>::new(&["map"]))
            .add_plugin(RonAssetPlugin::<UserPreferencesAsset>::new(&["rfg"]))
            .add_plugin(RonAssetPlugin::<ThemeDescription>::new(&["theme"]))
            .add_plugin(RonAssetPlugin::<CampaignDescription>::new(&["campaign"]))
            .add_plugin(RonAssetPlugin::<Save>::new(&["save"]))
            .add_plugin(RonAssetPlugin::<AssetManifest>::new(&["manifest"]))
            .init_asset_loader::<PatternLoader>()
            .add_system_set(
                // Load setup
                SystemSet::on_enter(GameState::load()).with_system(init_load_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::load()).with_system(load_assets_system),
            );
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);

// Loads prefab-like assets that need to be loaded before our main stuff
fn init_load_system(mut loading: ResMut<PreloadingAssets>, assets: Res<AssetServer>) {
    loading.0.push(assets.load_untyped("assets.manifest"));
}

fn load_assets_system(
    mut state: ResMut<State<GameState>>,
    mut loading: ResMut<PreloadingAssets>,
    mut stage: ResMut<Stage>,
    mut themes: ResMut<Themes>,
    mut campaigns: ResMut<Campaigns>,
    mut settings_handle: ResMut<Handle<UserPreferencesAsset>>,
    campaign_descriptions: Res<Assets<CampaignDescription>>,
    theme_assets: Res<Assets<ThemeDescription>>,
    assets: Res<AssetServer>,
    manifests: Res<Assets<AssetManifest>>,
    maps: Res<Assets<Map>>,
) {
    let done_loading = loading
        .0
        .iter()
        .filter(|h| assets.get_load_state(*h) == LoadState::Loading)
        .count()
        == 0;

    let mut theme_from_description = |desc: &ThemeDescription| -> Theme {
        let load_audio = |path: &String, loading: &mut PreloadingAssets| {
            let handle: Handle<AudioSource> = assets.load(format!("sounds/{}.ogg", path).as_str());
            loading.0.push(handle.clone_untyped());
            handle
        };
        let mut load_sprite = |path: &String, loading: &mut PreloadingAssets| {
            let handle = assets.load(format!("sprites/{}.png", path).as_str());
            loading.0.push(handle.clone_untyped());
            handle
        };

        Theme {
            sfx: ThemeSfx {
                place: load_audio(&desc.sfx.place, &mut loading),
                select: load_audio(&desc.sfx.select, &mut loading),
                swap: load_audio(&desc.sfx.swap, &mut loading),
                grip: load_audio(&desc.sfx.grip, &mut loading),
            },
            sprites: ThemeSprites {
                red: load_sprite(&desc.sprites.red, &mut loading),
                orange: load_sprite(&desc.sprites.orange, &mut loading),
                yellow: load_sprite(&desc.sprites.yellow, &mut loading),
                green: load_sprite(&desc.sprites.green, &mut loading),
                light_blue: load_sprite(&desc.sprites.light_blue, &mut loading),
                lime: load_sprite(&desc.sprites.lime, &mut loading),
                blue: load_sprite(&desc.sprites.blue, &mut loading),
                indigo: load_sprite(&desc.sprites.indigo, &mut loading),
                purple: load_sprite(&desc.sprites.purple, &mut loading),
                scored: load_sprite(&desc.sprites.scored, &mut loading),
                empty: load_sprite(&desc.sprites.empty, &mut loading),
                hover: load_sprite(&desc.sprites.hover, &mut loading),
                invalid: load_sprite(&desc.sprites.invalid, &mut loading),
            },
            name: desc.name.clone(),
        }
    };

    let campaign_from_description = |desc: &CampaignDescription| -> Campaign {
        Campaign {
            name: desc.name.clone(),
            levels: desc
                .levels
                .iter()
                .map(|level| Level {
                    map: maps
                        .iter()
                        .find_map(|(handle, map)| match map.name == level.map {
                            true => Some(maps.get_handle(handle)),
                            false => None,
                        })
                        .unwrap(),
                    options: level.options.clone(),
                    objective: level.objective.clone(),
                })
                .collect(),
        }
    };

    if done_loading {
        match stage.current() {
            0 => {
                stage.next();
            }
            1 => {
                // Able to load prefabs
                let mut theme_handles = load_folder(&assets, "themes", &manifests);
                loading.0.append(&mut theme_handles);

                let mut campaign_handles = load_folder(&assets, "campaigns", &manifests);
                loading.0.append(&mut campaign_handles);

                stage.next();
            }
            2 => {
                // load our settings file
                *settings_handle = assets.load("settings.rfg");
                loading.0.push(settings_handle.clone_untyped());

                // load everything else
                let mut handles: Vec<HandleUntyped> = load_folder(&assets, "patterns", &manifests)
                    .iter()
                    .chain(load_folder(&assets, "maps", &manifests).iter())
                    .chain(load_folder(&assets, "saves", &manifests).iter())
                    .cloned()
                    .collect();
                loading.0.append(&mut handles);

                stage.next();
            }
            3 => {
                // able to build prefabs
                // load the real "Themes" instead of the descriptions. The description is just a bunch of paths. We want to load all the handles and stuff.
                *themes = theme_assets
                    .iter()
                    .map(|(_, theme)| theme_from_description(theme))
                    .collect();

                *campaigns = campaign_descriptions
                    .iter()
                    .map(|desc| campaign_from_description(desc.1))
                    .collect();

                stage.next();
            }
            _ => {
                // Watch for changes
                assets
                    .watch_for_changes()
                    .expect("could not watch for changes");
                // Transition states to the menu
                state.set(GameState::Menu).ok();
            }
        }
    }
}

/// Works with web
fn load_folder(
    assets: &AssetServer,
    folder: &str,
    manifests: &Assets<AssetManifest>,
) -> Vec<HandleUntyped> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        assets.load_folder(folder).expect("Could not load")
    }

    #[cfg(target_arch = "wasm32")]
    {
        let manifest = &manifests.iter().next().unwrap().1 .0;
        manifest
            .get(folder)
            .unwrap()
            .iter()
            .map(|path| assets.load_untyped(PathBuf::from(format!("{}/{}", folder, path))))
            .collect()
    }
}
