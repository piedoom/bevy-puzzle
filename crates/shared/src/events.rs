use std::path::PathBuf;

use bevy::math::Vec2;

use crate::prelude::*;

pub enum GameEvent {
    /// Adds an active pattern. Ensure the existing active pattern is despawned before calling this.
    /// If `true`, the [`Unswappable`] component is added as well.
    SetActivePattern { pattern: Pattern, unswappable: bool },
    // SetGameMode(Handle<GameMode>),
    /// Attempt to commit the actively hovered tiles. Resets the active pattern if successful
    CommitActive {
        /// If value is true, a [`GameEvent::Loss`] event will be triggered
        loss_on_failure: bool,
    },
    /// Resets the game and kicks us back to the main menu
    Loss,
}

pub enum EditEvent {
    PlaceActive,
    Clear(Vec2),
    SaveCurrentMap { name: String, path: PathBuf },
    RunCurrentMap { options: GameOptions },
}
