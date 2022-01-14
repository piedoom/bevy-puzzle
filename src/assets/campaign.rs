//! Defines a list of maps, modes, and other metadata that composes the actual game from our separate pieces

use crate::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::{
    prelude::*,
    utils::{Duration, Instant},
};
use std::fmt::{Display, Formatter};

#[derive(
    serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Default, Debug, Clone, Eq, Hash,
)]
#[uuid = "cccccc12-3456-4fa8-adc4-78c5822269f8"]
pub struct CampaignDescription {
    pub name: String,
    pub levels: Vec<LevelDescription>,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Default, Debug, Clone, Eq, Hash)]
pub struct LevelDescription {
    pub map: String,
    #[serde(default)]
    pub options: GameOptions,
    pub objective: Objective,
}

#[derive(serde::Deserialize, serde::Serialize, Default, PartialEq, Debug, Clone, Eq, Hash)]
pub enum Objective {
    /// No objective
    #[default]
    FreePlay,
    /// Last until the timer runs out
    Survive(Duration),
    /// Reach a score in a given period of time
    /// This is technically just survival with an extra requirement
    /// so levels should adjust difficulty according to the objective.
    /// If the time is up and the required score is not met, the game will be a loss
    TimeLimit {
        required_score: usize,
        duration: Duration,
    },
    /// Play until a score is reached
    Score(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct Campaign {
    pub name: String,
    pub levels: Vec<Level>,
}

impl Display for Campaign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct Level {
    pub map: Handle<Map>,
    pub options: GameOptions,
    pub objective: Objective,
}

/// Because of how assets work currently, the asset ([`CampaignDescription`]) refers to strings of other assets
/// which are then manually replaced and built into a ([`Campaign`]). `Campaigns` should be treated like an assets container.  
pub type Campaigns = Vec<Campaign>;

/// The instant when the individual game started
pub type GameStarted = Instant;
