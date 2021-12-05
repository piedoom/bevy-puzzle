//! Defines a list of maps, modes, and other metadata that composes the actual game from our separate pieces

use bevy::reflect::TypeUuid;
use std::time::Duration;

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
    pub mode: String,
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
}
