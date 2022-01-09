use crate::prelude::*;
use bevy::prelude::*;

/// The bevy [`State`] of our game. All states the game can be in are here, and
/// contained in the resource [`Res<State<GameState>>`].
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Eq, Hash, Default)]
pub enum GameState {
    /// Assets are loaded from files. This state progresses once all assets are
    /// finished loading.
    #[default]
    PreLoad,
    /// Assets that require extra assembly due to the current asset loader
    /// limitations (e.g. [`CampaignDescription`] into a [`Campaign`]) reside here.
    Load,
    /// The "start" or "main" menu of the game
    Menu,
    /// The state before the main game is launched. This state can show info such
    /// as objectives or rules before the main state begins.
    PreGame(GameType),
    /// The main game state, where all the fun stuff happens
    Game(GameType),
    /// A post-game state that can show info like post-game stats
    PostGame(PostGameDetails),
    /// A pause state. Usually this should be `push`ed and `pop`ped
    Pause,
    /// Edit state (to be changed)
    Edit,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PostGameDetails {
    pub game_type: GameType,
    pub score: usize,
    pub result: GameResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameResult {
    #[default]
    Lose,
    Win,
}

/// Additional information that is required to run the game. This may be provided
/// one-off, or can be embedded as part of a campaign level.
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

/// Necessary information contained within the main game state
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameType {
    /// Campaign game type and details
    Campaign(CampaignDetails),
    /// All other game types (like custom 1-off plays or anything that isn't a
    /// campaign) are `Other`.
    Other(GameDetails),
}

impl Default for GameType {
    fn default() -> Self {
        Self::Other(GameDetails::default())
    }
}

impl GameType {
    /// Retrieves details from enum data without needing to manually destructure
    #[inline(always)]
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

    /// Retrieves campaign details from enum data without needing to manually destructure
    pub fn get_campaign(&self) -> Option<CampaignDetails> {
        match self {
            GameType::Campaign(c) => Some(c.clone()),
            GameType::Other(_) => None,
        }
    }
}

impl PartialEq for GameState {
    /// Set a custom equality method that only compares the enum variant,
    /// ignoring any attached data.
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
