#![feature(duration_zero)]

pub mod assets;
pub mod components;
mod plugins;
pub mod resources;
pub mod states;
pub mod ui;
pub mod utils;

pub mod prelude {
    use super::*;
    pub use assets::*;
    pub use components::{tiles::*, *};
    pub use plugins::*;
    pub use resources::*;
    pub use states::*;
    pub use utils::*;
}
