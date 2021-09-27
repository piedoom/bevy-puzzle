use bevy::{asset::*, prelude::*, reflect::TypeUuid};
use bevy_egui::egui::Color32;

#[derive(Default, Debug, Clone, TypeUuid, serde::Deserialize)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b505b"]
pub struct Pattern {
    pub name: String,
    pub color: PatternColor,
    pub blocks: Vec<Vec2>,
}

#[derive(Default, Debug, Clone, serde::Deserialize)]
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
            "red" => Self::Red,
            "orange" => Self::Orange,
            "yellow" => Self::Yellow,
            "lime" => Self::Lime,
            "green" => Self::Green,
            "light blue" => Self::LightBlue,
            "blue" => Self::Blue,
            "indigo" => Self::Indigo,
            "purple" => Self::Purple,
            _ => Self::Red,
        }
    }
}

impl Into<Color32> for PatternColor {
    fn into(self) -> Color32 {
        match self {
            PatternColor::Red => Color32::from_rgb(235, 71, 111),
            PatternColor::Orange => Color32::from_rgb(250, 103, 56),
            PatternColor::Yellow => Color32::from_rgb(255, 210, 51),
            PatternColor::Lime => Color32::from_rgb(197, 245, 61),
            PatternColor::Green => Color32::from_rgb(73, 233, 137),
            PatternColor::LightBlue => Color32::from_rgb(61, 223, 245),
            PatternColor::Blue => Color32::from_rgb(51, 145, 255),
            PatternColor::Indigo => Color32::from_rgb(93, 89, 255),
            PatternColor::Purple => Color32::from_rgb(120, 61, 245),
        }
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
        &["block"]
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
        Self {
            name: name.into(),
            color: PatternColor::from(color),
            blocks,
        }
    }
}
