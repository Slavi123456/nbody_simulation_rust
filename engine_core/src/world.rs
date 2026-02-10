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

    pub fn create_object(&mut self, position: S::Vec) {
        println!("Created object on position {:?}", position);
        self.objects.push(position);
    }

    // тук по-късно:
    // - build_physics_snapshot
    // - build_render_snapshot

    pub fn render_snapshot(&self) -> WorldSnapshot<S>
    where
        <S as Space>::Vec: Clone,
    {
        let objects_copy = self.objects.iter().cloned().collect();
        WorldSnapshot {
            objects: objects_copy,
        }
    }
}

#[derive(Debug)]
pub struct WorldSnapshot<S: Space> {
    pub objects: Vec<S::Vec>,
}
