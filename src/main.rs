use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use pz::prelude::*;
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            cursor_visible: true,
            cursor_locked: true,
            width: 1920f32,
            height: 1080f32,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.02, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugins(FullPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(pz::ui::UiPlugin)
        .run();
    //     .add_plugin(ShapePlugin)
    //

    //     .add_state(GameState::Load)
    //     .init_resource::<Score>()
    //     .init_resource::<ActiveEntity>()
    //     .insert_resource(PlacementTimer::from(Duration::from_millis(3000)))
    //     .init_resource::<Bag>()
    //
    //     .init_resource::<CursorPosition>()
    //     .init_resource::<Hold>()
    //     .init_resource::<NextUp>()
    //     .add_asset::<Pattern>()
    //     .insert_resource(ClearColor(Color::hex("1B1920").unwrap()))
    //     .init_asset_loader::<PatternLoader>()
    //     .add_plugin(RonAssetPlugin::<SettingsAsset>::new(&["rfg"]))
    //     .add_system_set(
    //         // Load setup
    //         SystemSet::on_enter(GameState::Load).with_system(load_setup.system()),
    //     )
    //     .add_system_set(
    //         SystemSet::on_update(GameState::Load)
    //             .with_system(assets_loaded_transition_system.system()),
    //     )
    //     .add_system_set(SystemSet::on_enter(GameState::Main).with_system(game_setup.system()))
    //     .add_system_set(
    //         SystemSet::on_update(GameState::Main)
    //             .with_system(update_hovered_board_pieces.system())
    //             .label("main"),
    //     )
    //     .add_system_set(
    //         SystemSet::on_update(GameState::Main)
    //             .with_system(scorer.system())
    //             .with_system(scored_effect.system())
    //             .with_system(animate_active.system())
    //             .after("main")
    //             .before("styles"),
    //     )
    //     .add_system_set(
    //         SystemSet::on_update(GameState::Main)
    //             .with_system(style_blocks.system())
    //             .label("styles")
    //             .after("main"),
    //     );

    //     .run();
}
