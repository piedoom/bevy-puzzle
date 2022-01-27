/*
Parts of this code taken from:
https://github.com/Bobox214/Kataster/blob/main/LICENSE.txt
Kataster is dual-licensed under either

* MIT License (docs/LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (docs/LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* at your option.
*/
use crate::prelude::*;
use bevy::render::render_resource::std140::{AsStd140, Std140};
use bevy::render::render_resource::{
    BindGroupEntry, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, ShaderStages,
};
use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, Buffer,
            BufferInitDescriptor, BufferUsages,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
};
use rand::Rng;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BackgroundMaterial>::default())
            .add_system_set(
                SystemSet::on_enter(GameState::game()).with_system(spawn_background_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::game())
                    .with_system(update_background_material_system),
            );
    }
}

// Spawn a simple stretched quad that will use of backgound shader
fn spawn_background_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    camera: Query<Entity, With<Camera>>,
    windows: Res<Windows>,
) {
    // Choose a random f32 for start_time, to have different background
    let mut rng = rand::thread_rng();
    let start_time = rng.gen_range(0.0..100.0f32);
    println!("{}", &camera.iter().count());
    if let Some(window) = windows.get_primary() {
        //    if let Ok(camera) = camera.get_single() {
        // let scale = 0.5; //camera.scale.x;
        let (x, y) = (window.width(), window.height()); //(window.width() * scale, window.height() * scale);
        let translation = Vec3::new(0f32, 0f32, -10f32); //camera.translation.truncate().extend(-10f32);
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation,
                scale: Vec3::new(x, y, 1.0),
                ..Default::default()
            },
            material: materials.add(BackgroundMaterial { time: start_time }),
            ..Default::default()
        });
        //    }
    }
}

// Currently the time is passed through our BackgroundMaterial
// So we need to use its time attribute
fn update_background_material_system(
    time: Res<Time>,
    mut background_materials: ResMut<Assets<BackgroundMaterial>>,
) {
    for (_id, mut background_material) in background_materials.iter_mut() {
        background_material.time += time.delta_seconds();
    }
}

#[derive(Component, Debug, Clone, TypeUuid)]
#[uuid = "e1276de8-712a-11ec-90d6-2424ca122342"]
struct BackgroundMaterial {
    time: f32,
}

#[derive(Clone)]
struct GpuBackgroundMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for BackgroundMaterial {
    type ExtractedAsset = BackgroundMaterial;
    type PreparedAsset = GpuBackgroundMaterial;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: extracted_asset.time.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuBackgroundMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}
impl Material2d for BackgroundMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/background.wgsl"))
    }
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/background.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(f32::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}
