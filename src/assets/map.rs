use bevy::{prelude::*, reflect::TypeUuid};

#[derive(
    serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Eq, Default, Debug, Hash, Clone,
)]
#[uuid = "accdef12-3456-4fa8-adc4-78c5822268f8"]
pub struct Map {
    pub name: String,
    /// The pattern
    pub pattern: Vec<(usize, usize)>,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Map {
    pub fn default_name() -> &'static str {
        "default"
    }
}
