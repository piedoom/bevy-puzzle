use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
use bevy_kira_audio::AudioSource;

#[derive(
    serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Default, Debug, Clone, Eq, Hash,
)]
#[uuid = "1dffade1-9c71-ffff-adc4-78c5822268fb"]
pub struct ThemeDescriptionAsset {
    pub name: String,
    pub sfx: ThemeSfx<String>,
    pub sprites: ThemeSprites<String>,
}

/// Because we cannot yet reference assets in other assets, we are using a [`ThemeDescription`] to tell us what assets to load,
/// and then building that into a [`Theme`] itself. Note that this is not an asset, but a resource - but we are using it like an asset.
#[derive(PartialEq, Clone, Debug, Hash, Eq, Default)]
pub struct Theme {
    pub name: String,
    pub sfx: ThemeSfx<Handle<AudioSource>>,
    pub sprites: ThemeSprites<Handle<Image>>,
}

pub type Themes = Vec<Theme>;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Default, Debug, Clone, Eq, Hash)]
pub struct ThemeSprites<T> {
    pub red: T,
    pub orange: T,
    pub yellow: T,
    pub green: T,
    pub light_blue: T,
    pub lime: T,
    pub blue: T,
    pub indigo: T,
    pub purple: T,
    pub scored: T,
    pub empty: T,
    pub hover: T,
    pub invalid: T,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Default, Debug, Clone, Eq, Hash)]
pub struct ThemeSfx<T> {
    pub place: T,
    pub select: T,
    pub swap: T,
    pub grip: T,
}

#[derive(Default)]
pub struct ThemeLoader;

impl AssetLoader for ThemeLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset = ron::de::from_bytes::<ThemeDescriptionAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["block"]
    }
}
impl Theme {
    pub fn sprite_from_color(&self, color: &super::PatternColor) -> Handle<Image> {
        match color {
            super::PatternColor::Red => self.sprites.red.clone(),
            super::PatternColor::Orange => self.sprites.orange.clone(),
            super::PatternColor::Yellow => self.sprites.yellow.clone(),
            super::PatternColor::Lime => self.sprites.lime.clone(),
            super::PatternColor::Green => self.sprites.green.clone(),
            super::PatternColor::LightBlue => self.sprites.light_blue.clone(),
            super::PatternColor::Blue => self.sprites.blue.clone(),
            super::PatternColor::Indigo => self.sprites.indigo.clone(),
            super::PatternColor::Purple => self.sprites.purple.clone(),
        }
    }
}
