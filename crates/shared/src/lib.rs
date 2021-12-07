#![feature(derive_default_enum)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

pub mod assets;
pub mod components;
mod events;
mod plugins;
pub mod resources;
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    PreLoad,
    Load,
    Menu,
    LoadOptions,
    StartOptions,
    EditOptions,
    PreGame(GameType),
    Game(GameType),
    PostGame(NextTransition),
    Pause,
    Edit,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum NextTransition {
    #[default]
    Menu,
    NewLevel(GameType),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GameDetails {
    pub map: Handle<Map>,
    pub options: GameOptions,
    pub objective: Objective,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CampaignDetails {
    pub campaign: Campaign,
    pub level_index: usize,
    pub campaign_scores: Vec<usize>,
}

impl CampaignDetails {
    pub fn current_level(&self) -> (&Level, usize) {
        (
            self.campaign.levels.get(self.level_index).unwrap(),
            self.level_index,
        )
    }
    pub fn next_level(&self) -> Option<(&Level, usize)> {
        let next_level_index = self.level_index + 1;
        self.campaign
            .levels
            .get(next_level_index)
            .map(|x| (x, next_level_index))
    }
    pub fn level_count(&self) -> usize {
        self.campaign.levels.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameType {
    Campaign(CampaignDetails),
    Other(GameDetails),
}

impl Default for GameType {
    fn default() -> Self {
        Self::Other(GameDetails::default())
    }
}

impl GameType {
    pub fn get_details(&self) -> GameDetails {
        match self {
            GameType::Campaign(c) => GameDetails {
                map: c.current_level().0.map.clone(),
                objective: c.current_level().0.objective.clone(),
                options: c.current_level().0.options.clone(),
            },
            GameType::Other(o) => o.clone(),
        }
    }

    pub fn get_campaign(&self) -> Option<CampaignDetails> {
        match self {
            GameType::Campaign(c) => Some(c.clone()),
            GameType::Other(_) => None,
        }
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl GameState {
    #[inline(always)]
    pub fn pre_load() -> Self {
        Self::PreLoad
    }

    #[inline(always)]
    pub fn load() -> Self {
        Self::Load
    }

    #[inline(always)]
    pub fn menu() -> Self {
        Self::Menu
    }

    #[inline(always)]
    pub fn pre_game() -> Self {
        Self::PreGame(GameType::default())
    }

    #[inline(always)]
    pub fn game() -> Self {
        Self::Game(GameType::default())
    }

    #[inline(always)]
    pub fn pause() -> Self {
        Self::Pause
    }

    #[inline(always)]
    pub fn edit() -> Self {
        Self::Edit
    }

    #[inline(always)]
    pub fn post_game() -> Self {
        Self::PostGame(Default::default())
    }
}
