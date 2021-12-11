mod assets;
mod core;
mod edit;
mod input;
mod style;

pub use {
    self::core::CorePuzzlePlugin, assets::AssetPlugin, edit::EditPlugin, input::InputPlugin,
    style::StylePlugin,
};
