use bevy::{asset::*, prelude::*, reflect::TypeUuid};
use bevy_egui::egui::Color32;

use crate::prelude::colors::*;

#[derive(Default, Debug, Clone, TypeUuid, serde::Deserialize, Component)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b505b"]
pub struct Pattern {
    pub name: String,
    pub color: PatternColor,
    pub blocks: Vec<Vec2>,
}

#[derive(Default, Debug, Clone, Copy, serde::Deserialize, Component)]
pub enum PatternColor {
    #[default]
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    LightBlue,
    Blue,
    Indigo,
    Purple,
}

impl From<&str> for PatternColor {
    fn from(s: &str) -> Self {
        match s {
            "red" => PatternColor::Red,
            "orange" => PatternColor::Orange,
            "yellow" => PatternColor::Yellow,
            "lime" => PatternColor::Lime,
            "green" => PatternColor::Green,
            "light blue" => PatternColor::LightBlue,
            "blue" => PatternColor::Blue,
            "indigo" => PatternColor::Indigo,
            "purple" => PatternColor::Purple,
            _ => PatternColor::Red,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<[u8; 3]> for PatternColor {
    fn into(self) -> [u8; 3] {
        match self {
            PatternColor::Red => RED,
            PatternColor::Orange => ORANGE,
            PatternColor::Yellow => YELLOW,
            PatternColor::Lime => LIME,
            PatternColor::Green => GREEN,
            PatternColor::LightBlue => LIGHT_BLUE,
            PatternColor::Blue => BLUE,
            PatternColor::Indigo => INDIGO,
            PatternColor::Purple => PURPLE,
        }
    }
}

impl Into<Color32> for PatternColor {
    fn into(self) -> Color32 {
        let bytes: [u8; 3] = self.into();
        Color32::from_rgb(bytes[0], bytes[1], bytes[2])
    }
}

#[derive(Default)]
pub struct PatternLoader;

impl AssetLoader for PatternLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let input = String::from_utf8(bytes.to_vec())?;
            let asset = Pattern::from_emoji(input);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["pattern"]
    }
}

impl Pattern {
    pub fn from_emoji(input: impl ToString) -> Self {
        // split at first newline
        let input = input.to_string();
        let (name, rest) = input.split_once('\n').unwrap();
        let (color, pattern) = rest.split_once('\n').unwrap();
        let mut blocks = Vec::<Vec2>::default();
        let mut cur = Vec2::ZERO;
        pattern.to_string().chars().for_each(|c| {
            match c {
                '⬛' => {
                    cur.x += 1.0;
                }
                '⬜' => {
                    blocks.push(cur);
                    cur.x += 1.0;
                }
                '\n' => {
                    cur.x = 0f32;
                    cur.y -= 1.0;
                }
                e => warn!("unrecognized char \"{}\" in pattern", e),
            };
        });
        let c = PatternColor::from(color);
        Self {
            name: name.into(),
            color: c,
            blocks,
        }
    }
}
