#![feature(float_interpolation)]

pub mod assets;
pub mod components;
mod events;
mod plugins;
pub mod resources;
pub mod ui;
pub mod utils;

pub mod prelude {
    pub use super::GameState;
    use super::*;
    pub use assets::*;
    pub use components::*;
    pub use events::*;
    pub use plugins::*;
    pub use resources::*;
    pub use utils::*;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Load,
    Menu,
    Main,
    Pause,
    Edit,
}
