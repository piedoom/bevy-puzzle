use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};

#[derive(
    serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Default, Debug, Clone, Eq, Hash,
)]
#[uuid = "1df82c01-9c71-ffff-adc4-78c5822268fb"]
pub struct Theme {
    pub name: String,
    /// Skipped as this is attained by them being in the same folder
    #[serde(skip)]
    pub colors: ThemeColors,
    #[serde(skip)]
    pub states: ThemeStates,
}
#[derive(PartialEq, Default, Debug, Clone, Eq, Hash)]
pub struct ThemeColors {
    pub red: Handle<Texture>,
    pub orange: Handle<Texture>,
    pub yellow: Handle<Texture>,
    pub lime: Handle<Texture>,
    pub green: Handle<Texture>,
    pub purple: Handle<Texture>,
}
#[derive(PartialEq, Default, Debug, Clone, Eq, Hash)]
pub struct ThemeStates {
    pub hover: Handle<Texture>,
    pub invalid: Handle<Texture>,
    pub empty: Handle<Texture>,
    pub scored: Handle<Texture>,
}

impl Theme {
    pub fn default_name() -> &'static str {
        "default"
    }
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
            let asset = ron::de::from_bytes::<Theme>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["block"]
    }
}
