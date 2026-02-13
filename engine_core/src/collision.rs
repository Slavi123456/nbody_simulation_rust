use crate::space::Space;

pub enum Collision {
    WithWall { body_id: usize, wall: Wall },
    WithBody { a_body_id: usize, b_body_id: usize },
}

pub enum Wall {
    Left,
    Right,
    Top,
    Bottom,
}
