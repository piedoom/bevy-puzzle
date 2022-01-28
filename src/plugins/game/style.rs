use crate::prelude::*;
use bevy::prelude::*;

const BG_COLOR: &str = "1B1920";
pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        let process = |state: GameState| -> SystemSet {
            SystemSet::on_update(state)
                .with_system(add_sprite_to_tiles_system)
                .label(Label::Process)
                .before(Label::React)
        };

        // Put systems in a reusable closure so we can easily add it to both
        // main game and edit states, which both use tile styling.
        let react = |state: GameState| -> SystemSet {
            SystemSet::on_update(state)
                .with_system(style_blocks_system)
                .with_system(scored_effect_system)
                .with_system(animate_active_system)
                .label(Label::React)
                .after(Label::Process)
        };

        app.insert_resource(ClearColor(Color::hex(BG_COLOR).unwrap()))
            .add_system_set(process(GameState::game()))
            .add_system_set(react(GameState::game()))
            .add_system_set(process(GameState::edit()))
            .add_system_set(react(GameState::edit()));
    }
}

/// Animate the active tiles to slowly grow to full size depending on
/// [`PlacementTimer`] percentage completion
fn animate_active_system(
    mut transforms: Query<&mut Transform>,
    active: Query<&Children, With<ActiveEntity>>,
    placement_timer: Query<&PlacementTimer, With<ActiveEntity>>,
) {
    // Get the single active and iterate over its children
    active
        .get_single()
        .map(|p| {
            p.iter().for_each(|e| {
                // Get the transform of each child and modify its scale
                transforms
                    .get_mut(*e)
                    .map(|mut t| {
                        // Interpolate the scale between 0.95 and 1.0
                        t.scale = Vec3::new(0.95, 0.95, 0.0).lerp(
                            Vec3::ONE,
                            placement_timer
                                .get_single()
                                // get the `percent()` of this timer as the scalar
                                .map(|t| t.percent())
                                .unwrap_or(0f32),
                        )
                    })
                    .ok();
            })
        })
        .ok();
}

/// Adds and removes components that adjust the visual stylle of tile patterns
/// All parameters operated on tiles that have been modified in the last step.
/// * `empty` - Tiles that have been marked as empty
/// * `scored` - Tiles that have been marked as scored
/// * `invalid` - Tiles that have been marked as invalid this is a combination
/// of finding pieces underneath the active and seeing if they are full. If
/// they are full, it is invalid. If not, it is hovered.
/// * `hovered` - Tiles that are underneath the active pattern but not full
/// * `unhovered` - Tiles that have just stopped being hovered
/// * `uninvalidated` - like unhovered but for invalid
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
                cmd.entity(entity).insert(theme.sprite_from_color(color));
                let mut t = transforms.get_mut(entity).unwrap();
                t.translation.z = 7.0;
            });

        empty.iter().chain(unhovered.iter()).for_each(|entity| {
            cmd.entity(entity).insert(theme.sprites.empty.clone());
            let mut t = transforms.get_mut(entity).unwrap();
            t.translation.z = 7.0;
        });
        scored.for_each(|entity| {
            cmd.entity(entity).insert(theme.sprites.scored.clone());
        });
        invalid.for_each(|entity| {
            cmd.entity(entity).insert(theme.sprites.invalid.clone());
            let mut t = transforms.get_mut(entity).unwrap();
            t.translation.z = 8.0;
        });
        hovered.for_each(|entity| {
            cmd.entity(entity).insert(theme.sprites.hover.clone());
            let mut t = transforms.get_mut(entity).unwrap();
            t.translation.z = 8.0;
        });
    }
}

/// Animate scored tiles to shrink out
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

/// Helper system to add sprites to new tiles
pub(crate) fn add_sprite_to_tiles_system(
    mut cmd: Commands,
    query: Query<(Entity, &Transform), Added<Tile>>,
) {
    // add sprite bundle
    query.for_each(|(entity, transform)| {
        cmd.entity(entity).insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1f32, 1f32)),
                ..Default::default()
            },
            transform: *transform,
            global_transform: (*transform).into(),
            ..Default::default()
        });
    });
}
