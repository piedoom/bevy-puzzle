use bevy::prelude::*;

use crate::prelude::*;

pub enum GameEvent {
    /// Sets the active pattern.
    /// If `true`, the [`Unswappable`] component is added as well.
    SetActivePattern {
        pattern: Pattern,
        unswappable: bool,
    },
    SetGameMode(Handle<GameMode>),
    /// Attempt to commit the actively hovered tiles. If value is true, a [`GameEvent::Loss`] event will be triggered.
    CommitActive(bool),
    /// Resets the game and kicks us back to the main menu
    Loss,
    /// Reset the play area, score, etc. back to initial values
    Reset,
}
