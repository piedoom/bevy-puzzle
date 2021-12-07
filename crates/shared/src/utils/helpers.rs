use std::{fs::File, io::Write, path::PathBuf};

use bevy::{asset::AssetPath, ecs::component::Component, prelude::*};

use crate::prelude::*;

/// Transition states in a fn as to avoid invalid states
#[inline(always)]
pub fn transition<F, T>(cmd: &mut Commands, entity: Entity)
where
    F: Component,
    T: Component + Default,
{
    cmd.entity(entity).remove::<F>().insert(T::default());
}

pub fn save_game(campaign: &Campaign, level: usize) -> Save {
    let save = Save::new(campaign, level);
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
