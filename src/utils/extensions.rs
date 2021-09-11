use bevy::prelude::*;

pub trait TransformExt {
    /// Returns a two-dimentional rounded coordinates useful for comparing game pieces  
    fn board_position(&self) -> Vec2;
}

impl TransformExt for Transform {
    fn board_position(&self) -> Vec2 {
        let t = self.translation;
        Vec2::new(t.x.round(), t.y.round())
    }
}

impl TransformExt for GlobalTransform {
    fn board_position(&self) -> Vec2 {
        let t = self.translation;
        Vec2::new(t.x.round(), t.y.round())
    }
}
