use crate::space::{Space, Space2D};

pub trait IntoSpaceVec<S: Space> {
    fn into_space_vec(self) -> S::Vec;
}

impl IntoSpaceVec<Space2D> for mint::Point2<f32> {
    fn into_space_vec(self) -> glam::Vec2 {
        glam::Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}
