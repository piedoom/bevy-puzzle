use bevy::{reflect::TypeUuid, utils::HashMap};

#[derive(serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Default, Debug, Clone, Eq)]
#[uuid = "fccfcc12-3456-4fa8-adc4-78c5822269f8"]
pub struct AssetManifest(pub HashMap<String, Vec<String>>);
