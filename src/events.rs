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
}
