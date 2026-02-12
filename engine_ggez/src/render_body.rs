use ggez::{
    Context, GameResult,
    graphics::{self, Color},
    mint::Point2,
};

pub struct RenderBody {
    pub sphere: graphics::Mesh,
    radius: f32,
}

impl RenderBody {
    pub fn new(radius: f32, ctx: &mut Context) -> GameResult<Self> {
        let sphere = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2 { x: 0.0, y: 0.0 },
            radius,
            2.0,
            Color::new(0.51, 0.294, 0.016, 1.0),
        )?;

        Ok(RenderBody { sphere, radius })
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}
