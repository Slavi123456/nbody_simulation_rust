#[derive(Debug)]
pub struct Space2D;

// struct Space3D;

use std::fmt::Debug;

pub trait Space {
    type Vec: Debug;
}

impl Space for Space2D {
    type Vec = glam::Vec2;
}
