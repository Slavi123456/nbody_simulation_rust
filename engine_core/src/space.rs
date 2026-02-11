#[derive(Debug, Copy, Clone)]
pub struct Space2D;

// struct Space3D;

use std::fmt::Debug;

pub trait Space {
    type Vec: Debug + SpaceVec + Clone;
}

impl Space for Space2D {
    type Vec = glam::Vec2;
}

pub trait SpaceVec: Copy + Debug {
    fn distance_squared(self, other: Self) -> f32;
}

impl SpaceVec for glam::Vec2 {
    fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }
}
