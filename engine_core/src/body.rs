use crate::space::{self, Space, SpaceVec};

#[derive(Debug, Clone)]
pub struct Body<S: Space> {
    pub id: usize,
    pub position: S::Vec,
    pub velocity: S::Vec,
    pub radius: f32,
}

impl<S: Space> Body<S> {
    pub fn new(id: usize, radius: f32, position: S::Vec) -> Self {
        Body {
            id: id,
            position,
            velocity: S::Vec::new(1.0, 1.0),
            radius,
        }
    }

    pub fn get_render_snapshot(&self) -> BodySnapshot<S> {
        BodySnapshot {
            id: self.id,
            position: self.position.clone(),
            radius: self.radius,
        }
    }

    pub fn get_physics_snapshot(&self) -> Self {
        Body {
            id: self.id,
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            radius: self.radius,
        }
    }
}

#[derive(Debug)]
pub struct BodySnapshot<S: Space> {
    pub id: usize,
    pub position: S::Vec,
    pub radius: f32,
}

impl<S: Space> BodySnapshot<S> {
    pub fn distance_squared(&self, other: S::Vec) -> f32 {
        self.position.distance_squared(other)
    }

    pub fn position(&self) -> S::Vec {
        self.position
    }
}
