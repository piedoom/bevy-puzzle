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
    /// Attempt to commit the actively hovered tiles
    CommitActive {
        /// If value is true, a [`GameEvent::Loss`] event will be triggered
        loss_on_failure: bool,
        /// If value is true, a [`GameEvent::SetActivePattern`] event will be triggered with the next-up piece
        set_active_pattern: bool,
    },
    /// Resets the game and kicks us back to the main menu
    Loss,
}
