use crate::body::{Body, BodySnapshot};
use crate::errors::Error;
use crate::mint_transform::IntoSpaceVec;
use crate::space::{Space, SpaceVec};

pub struct World<S: Space> {
    objects: Vec<Body<S>>,
}

impl<S: Space> World<S> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn create_object(&mut self, position: S::Vec, radius: f32) -> usize {
        println!("Created object on position {:?}", position);
        let id = self.objects.len();

        self.objects.push(Body::new(id, radius, position));

        id
    }

    pub fn physics_snapshot(&self) -> Self {
        let objects_copy = self
            .objects
            .iter()
            .map(|f| f.get_physics_snapshot())
            .collect();
        World {
            objects: objects_copy,
        }
    }
    pub fn render_snapshot(&self) -> WorldSnapshot<S>
    where
        <S as Space>::Vec: Clone,
    {
        let objects_copy = self
            .objects
            .iter()
            .map(|f| f.get_render_snapshot())
            .collect();
        WorldSnapshot {
            objects: objects_copy,
        }
    }

    pub fn move_objects(&mut self, snapshot: &World<S>, dt: f32) {
        for (body, snapshot_body) in self.objects.iter_mut().zip(snapshot.objects.iter()) {
            // println!(
            //     "Snapshot pos {:?}, Body vel {:?}, dt {:?}",
            //     snapshot_body.position, body.velocity, dt
            // );
            body.position = snapshot_body.position.add(&body.velocity.scale(dt));
        }
    }

    pub fn apply_force(&mut self, object_id: usize, new_vel: S::Vec) {
        self.objects[object_id].velocity = self.objects[object_id].velocity.add(&new_vel);
    }
}

#[derive(Debug)]
pub struct WorldSnapshot<S: Space> {
    pub objects: Vec<BodySnapshot<S>>,
}

impl<S: Space> WorldSnapshot<S> {
    pub fn is_click_on_object<P>(&self, click_position: P, radius_to_check: f32) -> bool
    where
        P: IntoSpaceVec<S>,
    {
        let click_pos: S::Vec = click_position.into_space_vec();

        let radius_sq = radius_to_check * radius_to_check;

        self.objects
            .iter()
            .any(|obj| obj.distance_squared(click_pos) <= radius_sq)
    }
}
