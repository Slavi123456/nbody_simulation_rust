use crate::errors::Error;
use crate::space::Space;

pub struct World<S: Space> {
    objects: Vec<S::Vec>,
}

impl<S: Space> World<S> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            objects: Vec::new(),
        })
    }

    // тук по-късно:
    // - build_physics_snapshot
    // - build_render_snapshot
}
