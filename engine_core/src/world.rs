use crate::body::{Body, BodySnapshot};
use crate::collision::Collision;
use crate::space::{Space, SpaceVec};

pub struct World<S: Space> {
    objects: Vec<Body<S>>,
    world_dim: S::Vec,
}

impl<S: Space> World<S> {
    const WORLD_FRICTION: f32 = 0.998;
    const GRAVITAIONAL_PULL: f32 = 10_000.0;
    const GRAVITAIONAL_SOFTNING: f32 = 0.1;
    const MIN_DIST_FOR_GRAVITAION: f32 = 10.0;
    const IMPULSE_MAX: f32 = 800.0;
    const DAMAGE_ON_BODY_COLLISON: f32 = 1000.0;
    const DAMAGE_ON_WALL_COLLISION: f32 = 800.0;

    pub fn new(world_dim: [f32; 2]) -> Self
where {
        Self {
            objects: Vec::new(),
            world_dim: S::Vec::from_array(world_dim),
        }
    }
    pub fn get_body(&self, id: usize) -> Option<&Body<S>> {
        if id > self.objects.len() {
            return None;
        }
        Some(&self.objects[id])
    }
    pub fn get_body_mut(&mut self, id: usize) -> Option<&mut Body<S>> {
        if id > self.objects.len() {
            return None;
        }
        Some(&mut self.objects[id])
    }
    pub fn step(&mut self, dt: f32) {
        let snapshot = self.physics_snapshot();

        self.apply_gravity(dt);
        self.move_objects(&snapshot, dt);

        let collisions = self.handle_collision();

        self.resolve_collisions(collisions);
        self.destroy_objects();
    }

    pub fn create_object(&mut self, position: S::Vec, radius: f32, mass: f32) -> usize {
        println!("Created object on position {:?}", position);
        let id = self.objects.len();

        self.objects.push(Body::new(id, radius, mass, position));

        id
    }
    pub fn apply_force(&mut self, object_id: usize, new_vel: S::Vec) {
        let body = self
            .objects
            .iter_mut()
            .find(|body| body.id == object_id)
            .unwrap();
        body.velocity = body.velocity.add(&new_vel);
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

    fn move_objects(&mut self, snapshot: &World<S>, dt: f32) {
        for (body, snapshot_body) in self.objects.iter_mut().zip(snapshot.objects.iter()) {
            // println!(
            //     "Snapshot pos {:?}, Body vel {:?}, dt {:?}",
            //     snapshot_body.position, body.velocity, dt
            // );
            body.position = snapshot_body.position.add(&body.velocity.scale(dt));
            body.velocity = body.velocity.scale(Self::WORLD_FRICTION);
        }
    }
    fn handle_collision(&mut self) -> Vec<Collision> {
        let mut collisions = Vec::new();

        self.handle_body_collisions(&mut collisions);
        self.handle_wall_collisions(&mut collisions);

        collisions
    }

    //pub only for the tests
    pub fn handle_body_collisions(&mut self, collision: &mut Vec<Collision>) {
        for i in 0..self.objects.len() {
            for j in i + 1..self.objects.len() {
                let body_a = &self.objects[i];
                let body_b = &self.objects[j];
                let sum_rad = body_a.radius + body_b.radius;
                if body_a.position.distance_squared(body_b.position) <= sum_rad * sum_rad {
                    collision.push(Collision::WithBody {
                        a_body_id: i,
                        b_body_id: j,
                    });
                    // println!("Body {:?} and body {:?} have collided!!", i, j);
                }
            }
        }
    }

    //pub only for the tests
    pub fn handle_wall_collisions(&mut self, collision: &mut Vec<Collision>) {
        for body in &mut self.objects {
            let r = body.radius;

            let width = self.world_dim.x();
            let height = self.world_dim.y();

            let body_pos_x = body.position.x();
            let body_pos_y = body.position.y();

            if body_pos_x - r <= 0.0 {
                // body.position.set_x(r);
                // body.velocity.set_x(body.velocity.scale(-1.0).x());
                collision.push(Collision::WithWall {
                    body_id: body.id,
                    wall: crate::collision::Wall::Left,
                });
            }

            if body_pos_x + r >= width {
                // body.position.set_x(width - r);
                // body.velocity.set_x(body.velocity.scale(-1.0).x());
                collision.push(Collision::WithWall {
                    body_id: body.id,
                    wall: crate::collision::Wall::Right,
                });
            }

            // Top wall
            if body_pos_y - r <= 0.0 {
                // body.position.set_y(r);
                // body.velocity.set_y(body.velocity.scale(-1.0).y());
                collision.push(Collision::WithWall {
                    body_id: body.id,
                    wall: crate::collision::Wall::Top,
                });
            }

            // Bottom wall
            if body_pos_y + r >= height {
                // body.position.set_y(height - r);
                // body.velocity.set_y(body.velocity.scale(-1.0).y());
                collision.push(Collision::WithWall {
                    body_id: body.id,
                    wall: crate::collision::Wall::Bottom,
                });
            }
        }
    }

    fn apply_gravity(&mut self, dt: f32) {
        for i in 0..self.objects.len() {
            for j in i + 1..self.objects.len() {
                let (a_slice, b_slice) = self.objects.split_at_mut(j);

                let body_a = &mut a_slice[i];
                let body_b = &mut b_slice[0];

                let distance = body_b.position.substract(&body_a.position);
                let distance_squared = distance.vec_length_squared()
                    + Self::GRAVITAIONAL_SOFTNING * Self::GRAVITAIONAL_SOFTNING;
                if distance_squared <= Self::MIN_DIST_FOR_GRAVITAION {
                    continue;
                }

                let force_magnitude =
                    Self::GRAVITAIONAL_PULL * body_a.mass * body_b.mass / distance_squared;
                let direction = distance.vec_normalize();

                let accel_a = direction.scale(force_magnitude / body_a.mass);
                let accel_b = direction.scale(-force_magnitude / body_b.mass);

                body_a.velocity = body_a.velocity.add(&accel_a.scale(dt));
                body_b.velocity = body_b.velocity.add(&accel_b.scale(dt));
            }
        }
    }
    fn resolve_collisions(&mut self, collisions: Vec<Collision>) {
        for collision in collisions {
            match collision {
                Collision::WithBody {
                    a_body_id,
                    b_body_id,
                } => {
                    // println!("Body {:?} and {:?} collidided", a_body_id, b_body_id);
                    let (a_slice, b_slice) = if a_body_id < b_body_id {
                        self.objects.split_at_mut(b_body_id)
                    } else {
                        self.objects.split_at_mut(a_body_id)
                    };

                    let body_a = &mut a_slice[a_body_id];
                    let body_b = &mut b_slice[0];
                    // body_a.velocity = body_a
                    //     .velocity
                    //     .substract(&body_a.velocity.substract(&body_b.velocity.clone()));
                    // body_b.velocity = body_b
                    //     .velocity
                    //     .substract(&body_b.velocity.substract(&body_a.velocity.clone()));

                    ////Inelastic collision
                    // let vel_per_mass_a = body_a.velocity.scale(body_a.mass);
                    // let vel_per_mass_b = body_b.velocity.scale(body_b.mass);
                    // let new_vel = (vel_per_mass_a.add(&vel_per_mass_b))
                    //     .scale(1.0 / (body_a.mass + body_b.mass));
                    // body_a.velocity = new_vel;
                    // body_b.velocity = new_vel;

                    ////Elastic collision
                    // println!("vel_a {:?} vel_b {:?}", body_a.velocity, body_b.velocity);
                    // let vel_per_mass_a = body_a.velocity.scale(2.0 * body_a.mass);
                    // let vel_per_mass_b = body_b.velocity.scale(2.0 * body_b.mass);
                    // let mass_sum = body_a.mass + body_b.mass;
                    // let new_vel_a = body_a
                    //     .velocity
                    //     .scale(body_b.mass - body_a.mass)
                    //     .add(&vel_per_mass_a)
                    //     .scale(1.0 / mass_sum);
                    // let new_vel_b = body_b
                    //     .velocity
                    //     .scale(body_a.mass - body_b.mass)
                    //     .add(&vel_per_mass_b)
                    //     .scale(1.0 / mass_sum);

                    // body_a.velocity = new_vel_a;
                    // body_b.velocity = new_vel_b;

                    //// With Impulse involved
                    let delta = body_b.position.substract(&body_a.position);
                    let normal = delta.vec_normalize();

                    let relative_velocity = body_b.velocity.substract(&body_a.velocity);
                    let velocity_along_normal = relative_velocity.vec_dot(&normal);

                    if velocity_along_normal > 0.0 {
                        return;
                    }

                    let mass_a = body_a.mass;
                    let mass_b = body_b.mass;

                    let impulse_raw = -(1.0 + Body::<S>::ELASTICITY) * velocity_along_normal
                        / (1.0 / mass_a + 1.0 / mass_b);
                    let impulse = impulse_raw.clamp(-Self::IMPULSE_MAX, Self::IMPULSE_MAX);

                    body_a.velocity = body_a.velocity.add(&normal.scale(impulse / mass_a));
                    body_b.velocity = body_b.velocity.substract(&normal.scale(impulse / mass_b));

                    let impulse_strength = impulse_raw.abs();
                    // println!(
                    //     "Body a will destroy itself and body b on impulse_clamp{:?} and raw {:?}",
                    //     impulse_strength, impulse_raw
                    // );

                    let pos_a = body_a.position;
                    let pos_b = body_b.position;
                    let normal_a = normal;
                    let normal_b = normal.scale(-1.0);
                    let impulse_strength = impulse_raw.abs();
                    // body_a.damage += Self::DAMAGE_ON_BODY_COLLISON;
                    // body_b.damage += Self::DAMAGE_ON_BODY_COLLISON;
                    println!(
                        "Dmg on a body ind:{:?}, damage:{:?}",
                        a_body_id, body_a.damage
                    );

                    let have_split_a = self.split_body_into_particles(
                        a_body_id,
                        pos_a,
                        normal_a,
                        impulse_strength,
                    );
                    let have_split_b = self.split_body_into_particles(
                        b_body_id,
                        pos_b,
                        normal_b,
                        impulse_strength,
                    );
                    println!("Count objects in world {:?}", self.objects.len());
                }
                Collision::WithWall { body_id, wall } => {
                    let body = self
                        .objects
                        .iter_mut()
                        .find(|body| body.id == body_id)
                        .unwrap();
                    let r = body.radius;
                    body.damage += Self::DAMAGE_ON_WALL_COLLISION;
                    match wall {
                        crate::collision::Wall::Left => {
                            body.position.set_x(r);
                            body.velocity.set_x(body.velocity.scale(-1.0).x());
                        }
                        crate::collision::Wall::Right => {
                            body.position.set_x(self.world_dim.x() - r);
                            body.velocity.set_x(body.velocity.scale(-1.0).x());
                        }
                        crate::collision::Wall::Top => {
                            body.position.set_y(r);
                            body.velocity.set_y(body.velocity.scale(-1.0).y());
                        }
                        crate::collision::Wall::Bottom => {
                            body.position.set_y(self.world_dim.y() - r);
                            body.velocity.set_y(body.velocity.scale(-1.0).y());
                        }
                    }
                }
            }
        }
    }
    fn physics_snapshot(&self) -> Self {
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
    fn destroy_objects(&mut self) {
        let mut bodies_to_destroy = Vec::<usize>::new();
        for (body) in self.objects.iter() {
            // println!(
            //     "Snapshot pos {:?}, Body vel {:?}, dt {:?}",
            //     snapshot_body.position, body.velocity, dt
            // );
            if body.damage > Body::<S>::DMG_THRESHOLD {
                bodies_to_destroy.push(body.id);
            }
        }
        self.objects
            .retain_mut(|body| !bodies_to_destroy.contains(&body.id));
    }
    fn split_body_into_particles(
        &mut self,
        body_id: usize,
        position: S::Vec,
        normal: S::Vec,
        impulse_strenght: f32,
    ) -> bool {
        let (body_velocity, damage) = {
            let body = match self.objects.iter().find(|el| el.id == body_id) {
                Some(b) => b,
                None => return false,
            };

            (body.velocity, body.damage)
        };

        let (count, mass, radius) = if impulse_strenght < 8000.0 {
            return false;
        } else if impulse_strenght < 12_000.0 {
            (2, 20.0, 10.0)
        } else if impulse_strenght < 16_000.0 {
            (2, 10.0, 10.0)
        } else {
            (2, 5.0, 10.0)
        };

        for _ in 0..count {
            let random_dir = self._random_dir_around_normal(normal, 0.6);
            let speed = body_velocity.vec_length() * 0.5;
            let particle_velocity = random_dir.scale(speed);

            let particle_id = self.create_object(position, radius, mass);
            self.objects[particle_id].set_damage(damage);

            println!("Created particle {:?}", particle_id);
            self.apply_force(particle_id, particle_velocity);
        }

        true
    }
    fn _random_dir_around_normal(&self, normal: S::Vec, spread: f32) -> S::Vec {
        let angle = rand::random_range(-spread..spread);
        let cos = angle.cos();
        let sin = angle.sin();

        S::Vec::new(
            normal.x() * cos - normal.y() * sin,
            normal.x() * sin + normal.y() * cos,
        )
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
