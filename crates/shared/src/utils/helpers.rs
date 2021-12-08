use crate::prelude::*;
use bevy::{asset::AssetPath, ecs::component::Component, prelude::*};
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

/// Create a new save file with the created-at (current) timestamp as the filename
///
/// * `save` - the [`Save`] to write to file
pub fn save_to_file(save: Save) -> Save {
    if let Ok(text) = ron::to_string(&save) {
        let path = AssetPath::from(PathBuf::from(format!(
            "assets/saves/{}.save",
            save.created_at.timestamp()
        )));
        let mut file = File::create(path.path()).unwrap();
        file.write_all(text.as_bytes()).ok();
    }
    save
}
