use crate::body::{Body, BodySnapshot};
use crate::space::{Space, SpaceVec};

pub struct World<S: Space> {
    objects: Vec<Body<S>>,
    world_dim: S::Vec,
}

impl<S: Space> World<S> {
    pub fn new(world_dim: [f32; 2]) -> Self
where {
        Self {
            objects: Vec::new(),
            world_dim: S::Vec::from_array(world_dim),
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
            world_dim: self.world_dim.clone(),
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

    pub fn handle_wall_collisions(&mut self) {
        for body in &mut self.objects {
            let r = body.radius;

            let width = self.world_dim.x();
            let height = self.world_dim.y();

            let body_pos_x = body.position.x();
            let body_pos_y = body.position.y();

            // Left wall
            if body_pos_x - r <= 0.0 {
                body.position.set_x(r);
                body.velocity.set_x(body.velocity.scale(-1.0).x());
            }

            // Right wall
            if body_pos_x + r >= width {
                body.position.set_x(width - r);
                body.velocity.set_x(body.velocity.scale(-1.0).x());
            }

            // Top wall
            if body_pos_y - r <= 0.0 {
                body.position.set_y(r);
                body.velocity.set_y(body.velocity.scale(-1.0).y());
            }

            // Bottom wall
            if body_pos_y + r >= height {
                body.position.set_y(height - r);
                body.velocity.set_y(body.velocity.scale(-1.0).y());
            }
        }
    }
}

#[derive(Debug)]
pub struct WorldSnapshot<S: Space> {
    pub objects: Vec<BodySnapshot<S>>,
}

impl<S: Space> WorldSnapshot<S> {
    pub fn is_click_on_object(
        &self,
        click_position: mint::Point2<f32>,
        radius_to_check: f32,
    ) -> bool {
        let click_pos: S::Vec = S::Vec::from_point(click_position);

        let radius_sq = radius_to_check * radius_to_check;

        self.objects
            .iter()
            .any(|obj| obj.distance_squared(click_pos) <= radius_sq)
    }
}
