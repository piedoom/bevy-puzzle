pub(crate) mod assets;
pub(crate) mod core;
pub(crate) mod edit;
use bevy::ecs::schedule::SystemLabel;

#[derive(SystemLabel, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Label {
    /// The first stage (gathering input)
    Listen,
    /// The second stage (moving, modifying entities)
    Process,
    /// The third stage (styling)
    React,
}
