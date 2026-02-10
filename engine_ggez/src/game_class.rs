use engine_core::engine::Engine;
use engine_core::mintTransform::IntoSpaceVec;
use engine_core::object_creation;
use engine_core::space::Space2D;

//for both crates to be on the same version
use engine_core::mint::Point2;

use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{KeyCode, KeyInput, KeyMods};
use ggez::input::mouse::{MouseButton, MouseContext};
use ggez::{Context, GameResult};

pub struct MyGame {
    engine: Engine<Space2D>,
    spawn_timer: f32,
    spawn_interval: f32,
    ready_to_spawn: bool,
}

impl MyGame {
    pub fn new(engine: Engine<Space2D>) -> Self {
        Self {
            engine,
            spawn_timer: 0.0,
            spawn_interval: 1.0,
            ready_to_spawn: true,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = &ctx.time.delta().as_secs_f32();
        let k_ctx = &ctx.keyboard;
        let m_ctx = &ctx.mouse;

        if k_ctx.is_key_pressed(KeyCode::Right) {
            if k_ctx.is_mod_active(KeyMods::SHIFT) {
                println!("RIGHT ARROW + SHIFT");
            }
            println!("RIGHT ARROW");
        } else if k_ctx.is_key_pressed(KeyCode::Left) {
            if k_ctx.is_mod_active(KeyMods::SHIFT) {
                println!("LEFT ARROW + SHIFT");
            }
            println!("LEFT ARROW");
        }

        if m_ctx.button_pressed(MouseButton::Left) {
            if self.ready_to_spawn {
                let new_event =
                    object_creation::<Space2D, engine_core::mint::Point2<f32>>(m_ctx.position());

                println!("Spawn event {:?}", new_event);
                self.engine.push_event(new_event);

                self.ready_to_spawn = false;
            }
        }

        if !self.ready_to_spawn {
            self.spawn_timer += delta_time;
            if self.spawn_timer >= self.spawn_interval {
                self.spawn_timer = 0.0;
                self.ready_to_spawn = true;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        canvas.finish(ctx)
    }
}
