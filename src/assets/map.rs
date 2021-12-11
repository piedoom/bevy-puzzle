use bevy::{math::Vec2, reflect::TypeUuid};
// use bevy_egui::egui::{self, Pos2};

#[derive(
    serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Default, Debug, Clone, Eq, Hash,
)]
#[uuid = "accdef12-3456-4fa8-adc4-78c5822268f8"]
pub struct Map {
    pub name: String,
    /// The pattern
    pub pattern: Vec<(isize, isize)>,
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

    /// Spits out a bevy thing so we don't need to import EGUI into our main lib. You should be
    /// able to make this into an egui thing later. Order is `(Bottom left, Top right)`.
    pub fn calculate_rect(&self) -> (Vec2, Vec2) {
        let mut x = self.pattern.clone();
        x.sort_by(|(xa, _), (xb, _)| xa.cmp(xb));
        let mut y = self.pattern.clone();
        y.sort_by(|(_, ya), (_, yb)| ya.cmp(yb));

        let left = x.first().unwrap().0 as f32;
        let right = x.last().unwrap().0 as f32;
        let top = y.last().unwrap().1 as f32;
        let bottom = y.first().unwrap().1 as f32;

        (Vec2::new(left, bottom), Vec2::new(right, top))
    }
}
