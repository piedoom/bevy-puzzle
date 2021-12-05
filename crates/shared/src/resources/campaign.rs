use crate::prelude::*;
use bevy::{prelude::*, utils::Instant};

pub struct Campaign {
    pub name: String,
    pub levels: Vec<Level>,
}

pub struct Level {
    pub map: Handle<Map>,
    pub mode: Handle<GameMode>,
    pub objective: Objective,
}

/// Because of how assets work currently, the asset ([`CampaignDescription`]) refers to strings of other assets
/// which are then manually replaced and built into a ([`Campaign`]). `Campaigns` should be treated like an assets container.  
pub type Campaigns = Vec<Campaign>;
pub type CurrentLevel = Level;

/// The instant when the individual game started
pub type GameStarted = Instant;
