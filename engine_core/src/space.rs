#[derive(Debug, Clone)]
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
    fn new(x: f32, y: f32) -> Self;
    fn scale(&self, factor: f32) -> Self;
    fn add(&self, other: &Self) -> Self;
}

impl SpaceVec for glam::Vec2 {
    fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    fn new(x: f32, y: f32) -> Self {
        glam::Vec2 { x, y }
    }

    fn scale(&self, scalar: f32) -> Self {
        glam::Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn add(&self, other: &Self) -> Self {
        glam::Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
