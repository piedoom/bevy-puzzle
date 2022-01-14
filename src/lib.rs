#![feature(derive_default_enum)]
#![feature(let_else)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod assets;
pub mod components;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod state;
pub mod utils;

pub mod prelude {
    use super::*;
    pub use assets::*;
    pub use components::*;
    pub use events::*;
    pub use plugins::*;
    pub use resources::*;
    pub use state::*;
    pub use utils::*;
}
