#![feature(float_interpolation)]
#![feature(derive_default_enum)]

use bevy::prelude::*;

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

use prelude::*;

/*
#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);
    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    // TODO: add all your other stuff to `app` as usual
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.02, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugins(FullPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ui::UiPlugin);
    app.run();
}
*/

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);

#[derive(Debug, Clone, Eq, Hash)]
pub enum GameState {
    Load,
    Menu,
    Main { mode: GameMode, map: Map },
    Pause,
    Edit,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl GameState {
    #[inline(always)]
    pub fn load() -> Self {
        Self::Load
    }

    #[inline(always)]
    pub fn menu() -> Self {
        Self::Menu
    }

    #[inline(always)]
    pub fn main() -> Self {
        Self::Main {
            mode: Default::default(),
            map: Default::default(),
        }
    }

    #[inline(always)]
    pub fn pause() -> Self {
        Self::Pause
    }

    #[inline(always)]
    pub fn edit() -> Self {
        Self::Edit
    }
}
