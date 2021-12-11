//! Defines a user created save, referencing a campaign

use bevy::reflect::TypeUuid;
use chrono::{DateTime, Local};

#[derive(serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Debug, Clone, Eq, Hash)]
#[uuid = "cdcdcc12-3456-4fa8-adc4-74C0B31169f8"]
pub struct Save {
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub campaign: String,
    pub level: usize,
}

impl Save {
    pub fn new<T>(campaign: T, level: usize) -> Self
    where
        T: ToString,
    {
        Self {
            created_at: chrono::offset::Local::now(),
            updated_at: chrono::offset::Local::now(),
            campaign: campaign.to_string(),
            level,
        }
    }
}
