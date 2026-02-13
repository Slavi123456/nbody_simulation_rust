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
    fn x(self) -> f32;
    fn y(self) -> f32;

    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);

    fn distance_squared(self, other: Self) -> f32;
    fn new(x: f32, y: f32) -> Self;
    fn scale(&self, factor: f32) -> Self;
    fn add(&self, other: &Self) -> Self;

    fn from_array(arr: [f32; 2]) -> Self;
    fn from_point(p: mint::Point2<f32>) -> Self;
}

impl SpaceVec for glam::Vec2 {
    fn x(self) -> f32 {
        self.x
    }
    fn y(self) -> f32 {
        self.y
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

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

    fn from_array(arr: [f32; 2]) -> Self {
        glam::Vec2 {
            x: arr[0],
            y: arr[1],
        }
    }
    fn from_point(p: mint::Point2<f32>) -> Self {
        glam::Vec2 { x: p.x, y: p.y }
    }
}
