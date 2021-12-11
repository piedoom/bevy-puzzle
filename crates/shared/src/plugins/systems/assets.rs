use crate::{assets::UserPreferencesAsset, prelude::*, GameState, PreloadingAssets};
use bevy::{asset::LoadState, prelude::*};
use bevy_asset_ron::RonAssetPlugin;
use bevy_kira_audio::AudioSource;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreloadingAssets>()
            .init_resource::<Handle<UserPreferencesAsset>>()
            .init_resource::<Themes>()
            .init_resource::<Campaigns>()
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
            .init_asset_loader::<PatternLoader>()
            .add_system_set(
                // Load setup
                SystemSet::on_enter(GameState::pre_load()).with_system(init_pre_load_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::pre_load())
                    .with_system(pre_load_assets_loaded_transition_system),
            )
            .add_system_set(
                // Load setup
                SystemSet::on_enter(GameState::load()).with_system(init_load_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::load())
                    .with_system(assets_loaded_transition_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::load()).with_system(assemble_after_loaded_system),
            );
    }
}

// Loads prefab-like assets that need to be loaded before our main stuff
fn init_pre_load_system(mut loading: ResMut<PreloadingAssets>, assets: Res<AssetServer>) {
    let theme_handles = &mut assets.load_folder("themes").expect("Could not load modes");
    loading.0.append(theme_handles);

    let campaign_handles = &mut assets
        .load_folder("campaigns")
        .expect("Could not load campaigns");
    loading.0.append(campaign_handles);
}

fn pre_load_assets_loaded_transition_system(
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
        // Transition states to the menu
        state.set(GameState::Load).ok();
    }
}

fn init_load_system(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    mut settings_handle: ResMut<Handle<UserPreferencesAsset>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut loading: ResMut<PreloadingAssets>,
    mut themes: ResMut<Themes>,
    assets: Res<AssetServer>,
    theme_assets: Res<Assets<ThemeDescription>>,
) {
    // load our settings file
    *settings_handle = assets.load("settings.rfg");
    loading.0.push(settings_handle.clone_untyped());

    // load all block patterns
    let patterns = &mut assets
        .load_folder("patterns")
        .expect("Could not load patterns");
    loading.0.append(patterns);

    let mut theme_from_description = |desc: &ThemeDescription| -> Theme {
        let load_audio = |path: &String, loading: &mut PreloadingAssets| {
            let handle: Handle<AudioSource> = assets.load(format!("sounds/{}.ogg", path).as_str());
            loading.0.push(handle.clone_untyped());
            handle
        };
        let mut load_sprite = |path: &String, loading: &mut PreloadingAssets| {
            let handle = assets.load(format!("sprites/{}.png", path).as_str());
            loading.0.push(handle.clone_untyped());
            materials.add(handle.into())
        };

        Theme {
            sfx: ThemeSfx {
                place: load_audio(&desc.sfx.place, &mut loading),
                select: load_audio(&desc.sfx.select, &mut loading),
                swap: load_audio(&desc.sfx.swap, &mut loading),
                grip: load_audio(&desc.sfx.grip, &mut loading),
            },
            materials: ThemeSprites {
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

    // load the real "Themes" instead of the descriptions. The description is just a bunch of paths. We want to load all the handles and stuff.
    *themes = theme_assets
        .iter()
        .map(|(_, theme)| theme_from_description(theme))
        .collect();

    // set the default theme resource
    cmd.insert_resource(
        themes
            .iter()
            .find(|x| x.name == "default")
            .expect("no default asset. what did you do with it?")
            // it's fine to clone cause this is just like a bunch of handles anyways i think
            .clone(),
    );

    // load maps
    let map_handles = &mut assets.load_folder("maps").expect("Could not load maps");
    loading.0.append(map_handles);

    // Load all saves
    let save_handles = &mut assets.load_folder("saves").expect("Could not load saves");
    loading.0.append(save_handles);

    // Watch for changes
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
        // Transition states to the menu
        state.set(GameState::Menu).ok();
    }
}

/// Assemble assets that need everything to be loaded first. Call this on exit.
fn assemble_after_loaded_system(
    mut campaigns: ResMut<Campaigns>,
    maps: Res<Assets<Map>>,
    campaign_descriptions: Res<Assets<CampaignDescription>>,
) {
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

    *campaigns = campaign_descriptions
        .iter()
        .map(|desc| campaign_from_description(desc.1))
        .collect();
}
