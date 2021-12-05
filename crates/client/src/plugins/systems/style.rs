use bevy::prelude::*;
use shared::prelude::*;

pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        let process = |state: GameState| -> SystemSet {
            SystemSet::on_update(state)
                .with_system(add_sprite_to_tiles_system)
                .label(Label::Process)
                .before(Label::React)
        };

        let react = |state: GameState| -> SystemSet {
            SystemSet::on_update(state)
                .with_system(style_blocks_system)
                .with_system(scored_effect_system)
                .with_system(animate_active_system)
                .label(Label::React)
                .after(Label::Process)
        };

        app.insert_resource(ClearColor(Color::hex("1B1920").unwrap()))
            .add_system_set(process(GameState::main()))
            .add_system_set(react(GameState::main()))
            .add_system_set(process(GameState::edit()))
            .add_system_set(react(GameState::edit()));
    }
}

fn animate_active_system(
    mut transforms: Query<&mut Transform>,
    active: Query<&Children, With<ActiveEntity>>,
    placement_timer: Query<&PlacementTimer, With<ActiveEntity>>,
) {
    active
        .get_single()
        .map(|p| {
            p.iter().for_each(|e| {
                transforms
                    .get_mut(*e)
                    .map(|mut t| {
                        t.scale = Vec3::new(0.95, 0.95, 0.0).lerp(
                            Vec3::ONE,
                            placement_timer
                                .get_single()
                                .map(|t| t.percent())
                                .unwrap_or(0f32),
                        )
                    })
                    .ok();
            })
        })
        .ok();
}

fn style_blocks_system(
    mut cmd: Commands,
    mut transforms: Query<&mut Transform>,
    theme: Option<Res<Theme>>,
    full: Query<(Entity, &PatternColor), Added<tile_states::Full>>,
    empty: Query<
        Entity,
        (
            Added<tile_states::Empty>,
            With<tile_styles::None>,
            // With<GameBoard>,
        ),
    >,
    scored: Query<Entity, Added<tile_states::Scored>>,
    invalid: Query<
        Entity,
        (
            With<tile_states::Full>,
            Added<tile_styles::Invalid>,
            With<GameBoard>,
        ),
    >,
    hovered: Query<
        Entity,
        (
            With<tile_states::Empty>,
            Added<tile_styles::Hover>,
            With<GameBoard>,
        ),
    >,
    unhovered: Query<
        Entity,
        (
            With<tile_states::Empty>,
            Added<tile_styles::None>,
            With<GameBoard>,
        ),
    >,
    uninvalidated: Query<
        (Entity, &PatternColor),
        (
            With<tile_states::Full>,
            Added<tile_styles::None>,
            With<GameBoard>,
        ),
    >,
) {
    if let Some(theme) = theme {
        full.iter()
            .chain(uninvalidated.iter())
            .for_each(|(entity, color)| {
                cmd.entity(entity).insert(theme.material_from_color(color));
                let mut t = transforms.get_mut(entity).unwrap();
                t.translation.z = 7.0;
            });

        empty.iter().chain(unhovered.iter()).for_each(|entity| {
            cmd.entity(entity).insert(theme.materials.empty.clone());
            let mut t = transforms.get_mut(entity).unwrap();
            t.translation.z = 7.0;
        });
        scored.for_each(|entity| {
            cmd.entity(entity).insert(theme.materials.scored.clone());
        });
        invalid.for_each(|entity| {
            cmd.entity(entity).insert(theme.materials.invalid.clone());
            let mut t = transforms.get_mut(entity).unwrap();
            t.translation.z = 8.0;
        });
        hovered.for_each(|entity| {
            cmd.entity(entity).insert(theme.materials.hover.clone());
            let mut t = transforms.get_mut(entity).unwrap();
            t.translation.z = 8.0;
        });
    }
}

pub(crate) fn scored_effect_system(
    mut cmd: Commands,
    time: Res<Time>,
    mut scored: Query<(Entity, &mut Transform, &mut Timer), With<tile_states::Scored>>,
) {
    scored.for_each_mut(|(e, mut t, mut timer)| {
        timer.tick(time.delta());
        // shrink and delete when scale is too small
        t.scale = t.scale.lerp(Vec3::new(0f32, 0f32, 2f32), timer.percent());
        if timer.finished() {
            cmd.entity(e).despawn_recursive();
        }
    });
}

pub(crate) fn add_sprite_to_tiles_system(
    mut cmd: Commands,
    query: Query<(Entity, &Transform), Added<Tile>>,
) {
    // add sprite bundle
    query.for_each(|(entity, transform)| {
        cmd.entity(entity).insert_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(1f32, 1f32)),
            transform: transform.clone(),
            global_transform: transform.clone().into(),
            ..Default::default()
        });
    });
}
