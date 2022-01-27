use crate::prelude::*;
use bevy::{asset::AssetPath, ecs::component::Component, prelude::*};
use bevy_egui::egui::Color32;
use std::{fs::File, io::Write, path::PathBuf};

/// Transition states in a fn as to avoid invalid states
/// Types `F` & `T` are the initial and final transition states respectively.
///
/// * `cmd` - a mutable reference to a system's [`Commands`]
/// * `entity` - the entity to which the component state is removed and attached
#[inline(always)]
pub fn transition<F, T>(cmd: &mut Commands, entity: Entity)
where
    F: Component,
    T: Component + Default,
{
    cmd.entity(entity).remove::<F>().insert(T::default());
}

/// Create (or replace) a save file with the created-at timestamp as the filename
///
/// * `save` - the [`Save`] to write to file
pub fn save_to_file(save: Save) -> Save {
    // TODO: save with wasm
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(text) = ron::to_string(&save) {
            let path = AssetPath::from(PathBuf::from(format!(
                "assets/saves/{}.save",
                save.created_at.timestamp()
            )));

            let mut file = File::create(path.path()).unwrap();
            file.write_all(text.as_bytes()).ok();
        }
    }

    save
}

pub const fn from_rgb_array(color: [u8; 3]) -> Color32 {
    Color32::from_rgb(color[0], color[1], color[2])
}
