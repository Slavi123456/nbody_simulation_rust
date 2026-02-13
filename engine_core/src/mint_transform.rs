// pub trait IntoSpaceVec<V> {
//     fn into_space_vec(self) -> V;
// }

// impl IntoSpaceVec<glam::Vec2> for mint::Point2<f32> {
//     fn into_space_vec(self) -> glam::Vec2 {
//         glam::Vec2 {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }

// impl IntoSpaceVec<glam::Vec2> for [f32; 2] {
//     fn into_space_vec(self) -> glam::Vec2 {
//         glam::Vec2 {
//             x: self[0],
//             y: self[1],
//         }
//     }
// }
