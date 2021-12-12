use bevy::prelude::*;
use bevy_egui::egui::Color32;

pub trait TransformExt {
    /// Converts the game transform into whole-number coordinates corresponding to the
    /// game board position. This is useful for comparing game pieces.
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

pub trait F32Ext {
    /// Interpolate a value with another
    ///
    /// ## Example
    ///
    /// ```
    /// let a: f32 = 0.0.lerp(2.0, 0.5);
    /// assert_eq!(a, 1.0);
    /// ```
    fn lerp(&self, other: f32, scalar: f32) -> f32;
}

impl F32Ext for f32 {
    #[inline(always)]
    fn lerp(&self, other: f32, scalar: f32) -> f32 {
        self + (other - self) * scalar
    }
}

pub trait Color32Ext {
    fn rgb_from_array(color: [u8; 3]) -> Color32;
}

impl Color32Ext for Color32 {
    fn rgb_from_array(color: [u8; 3]) -> Color32 {
        Self::from_rgb(color[0], color[1], color[2])
    }
}
