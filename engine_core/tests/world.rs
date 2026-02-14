#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::{space::*, world::World};

    const WORLD_DIMENSIONS: [f32; 2] = [400.0, 400.0];

    type Vec2 = <Space2D as Space>::Vec;

    #[test]
    fn test_body_collision() {
        let mut world = World::<Space2D>::new(WORLD_DIMENSIONS);
        let p: Vec2 = Vec2::new(0.0, 0.0);
        let id = world.create_object(p.clone(), 20.0, 10.0);

        let id_2 = world.create_object(p, 10.0, 10.0);

        let mut collisions = Vec::<engine_core::collision::Collision>::new();
        world.handle_body_collisions(&mut collisions);

        assert!(
            collisions.len() > 0 && collisions.len() < 2,
            "Should have collisions between the two bodies"
        )
    }

    #[test]
    fn test_wall_collision() {
        let mut world = World::<Space2D>::new(WORLD_DIMENSIONS);
        let p: Vec2 = Vec2::new(0.0, 0.0);
        let id = world.create_object(p, 10.0, 10.0);

        let p: Vec2 = Vec2::new(WORLD_DIMENSIONS[0], WORLD_DIMENSIONS[1]);
        let id_2 = world.create_object(p, 10.0, 10.0);

        let mut collisions = Vec::<engine_core::collision::Collision>::new();
        world.handle_wall_collisions(&mut collisions);

        assert!(
            collisions.len() > 0 && collisions.len() < 5,
            "Should have collisions for the four walls"
        )
    }
    #[test]
    fn apply_force_test() {
        let mut world = World::<Space2D>::new(WORLD_DIMENSIONS);
        let p: Vec2 = Vec2::new(100.0, 200.0);
        let id = world.create_object(p, 10.0, 10.0);

        let v0 = world.get_body(id).unwrap().velocity.length();
        world.apply_force(id, Vec2::new(50.0, 50.0));

        let v1 = world.get_body(id).unwrap().velocity.length();

        assert!(
            v1 > v0,
            "Velocity should increased due apply force event (v0={}, v1={})",
            v0,
            v1
        );
    }

    #[test]
    fn inertia_is_reduced_by_world_friction() {
        let mut world = World::<Space2D>::new(WORLD_DIMENSIONS);
        let p: Vec2 = Vec2::new(100.0, 200.0);
        let id = world.create_object(p, 10.0, 10.0);

        {
            let body = world.get_body_mut(id).unwrap();
            body.velocity = Vec2::new(10.0, 0.0);
        }

        let v0 = world.get_body(id).unwrap().velocity.length();

        for _ in 0..10 {
            world.step(0.016);
        }

        let v1 = world.get_body(id).unwrap().velocity.length();

        assert!(
            v1 < v0,
            "Velocity should decrease due to world friction (v0={}, v1={})",
            v0,
            v1
        );
    }

    #[test]
    fn single_body_without_velocity_stays_still() {
        let mut world = World::<Space2D>::new(WORLD_DIMENSIONS);
        let p = Vec2::new(100.0, 200.0);
        let id = world.create_object(p, 10.0, 10.0);

        for _ in 0..10 {
            world.step(0.016);
        }

        let body = world.get_body(id).unwrap();

        assert!(
            body.velocity.vec_length() < 1e-5,
            "Body should not gain velocity"
        );
        assert!(
            body.position.distance(Vec2::new(100.0, 200.0)) < 1e-4,
            "Body should not move without forces"
        );
    }

    #[test]
    fn two_bodies_attract_each_other() {
        let mut world = World::<Space2D>::new(WORLD_DIMENSIONS);

        let id_a = world.create_object(Vec2::new(150.0, 200.0), 10.0, 10.0);
        let id_b = world.create_object(Vec2::new(250.0, 200.0), 10.0, 10.0);

        for _ in 0..5 {
            world.step(0.016);
        }

        let body_a = world.get_body(id_a).unwrap();
        let body_b = world.get_body(id_b).unwrap();

        let dir_ab = (body_b.position - body_a.position).normalize();
        let vel_a_dir = body_a.velocity.normalize_or_zero();
        let vel_b_dir = body_b.velocity.normalize_or_zero();

        assert!(
            vel_a_dir.dot(dir_ab) > 0.0,
            "Body A should move towards body B"
        );
        assert!(
            vel_b_dir.dot(-dir_ab) > 0.0,
            "Body B should move towards body A"
        );
    }
}
