pub(crate) mod assets;
pub(crate) mod core;
pub mod helpers;
pub(crate) mod input;
pub(crate) mod style;
use bevy::ecs::schedule::SystemLabel;

#[derive(SystemLabel, Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum Label {
    /// The first stage (gathering input)
    Listen,
    /// The second stage (moving, modifying entities)
    Process,
    /// The third stage (styling)
    React,
}
