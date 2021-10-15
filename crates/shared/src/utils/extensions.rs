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

pub trait Rectangle<T> {
    fn expand(&mut self, amount: f32);
    fn center(&self) -> T;
    fn left_bottom(&self) -> T;
    fn right_top(&self) -> T;
}

impl<T> Rectangle<T> for (T, T)
where
    T: Into<Vec2> + From<Vec2> + std::ops::AddAssign<Vec2> + Copy,
{
    fn expand(&mut self, amount: f32) {
        self.0 += Vec2::new(-amount, -amount);
        self.1 += Vec2::new(amount, amount);
    }

    fn center(&self) -> T {
        let left_bottom: Vec2 = self.0.into();
        let right_top: Vec2 = self.1.into();
        let width = right_top.x - left_bottom.x;
        let height = right_top.y - left_bottom.y;
        T::from(Vec2::new(width / 2f32, height / 2f32))
    }

    fn left_bottom(&self) -> T {
        self.0
    }

    fn right_top(&self) -> T {
        self.1
    }
}
