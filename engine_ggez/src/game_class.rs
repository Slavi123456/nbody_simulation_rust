// use engine_core::engine::Engine;
// use engine_core::object_creation;
// use engine_core::space::Space2D;

// use ggez::event::EventHandler;
// use ggez::graphics::{self, Color, DrawParam};
// use ggez::input::keyboard::{KeyCode, KeyMods};
// use ggez::input::mouse::MouseButton;
// use ggez::{Context, GameResult};

// pub struct MyGame {
//     engine: Engine<Space2D>,
//     spawn_timer: f32,
//     spawn_interval: f32,
//     ready_to_spawn: bool,
//     rect: graphics::Mesh,
// }

// impl MyGame {
//     pub fn new(engine: Engine<Space2D>, ctx: &mut Context) -> Self {
//         Self {
//             engine,
//             spawn_timer: 0.0,
//             spawn_interval: 1.0,
//             ready_to_spawn: true,
//             rect: rect,
//         }
//     }
// }

// impl EventHandler for MyGame {
//     fn update(&mut self, ctx: &mut Context) -> GameResult {
//         let delta_time = &ctx.time.delta().as_secs_f32();
//         let k_ctx = &ctx.keyboard;
//         let m_ctx = &ctx.mouse;

//         if k_ctx.is_key_pressed(KeyCode::Right) {
//             if k_ctx.is_mod_active(KeyMods::SHIFT) {
//                 println!("RIGHT ARROW + SHIFT");
//             }
//             println!("RIGHT ARROW");
//         } else if k_ctx.is_key_pressed(KeyCode::Left) {
//             if k_ctx.is_mod_active(KeyMods::SHIFT) {
//                 println!("LEFT ARROW + SHIFT");
//             }
//             println!("LEFT ARROW");
//         }

//         if m_ctx.button_pressed(MouseButton::Left) {
//             if self.ready_to_spawn {
//                 let new_event =
//                     object_creation::<Space2D, engine_core::mint::Point2<f32>>(m_ctx.position());

//                 println!("Spawn event {:?}", new_event);
//                 self.engine.push_event(new_event);

//                 self.ready_to_spawn = false;
//             }
//         }

//         if !self.ready_to_spawn {
//             self.spawn_timer += delta_time;
//             if self.spawn_timer >= self.spawn_interval {
//                 self.spawn_timer = 0.0;
//                 self.ready_to_spawn = true;
//             }
//         }
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLUE);

//         canvas.finish(ctx)
//     }
// }

use engine_core::engine::Engine;
use engine_core::space::{Space, Space2D};
use engine_core::world::WorldSnapshot;
use engine_core::{events, object_creation};

use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam};
use ggez::input::keyboard::{self, KeyCode, KeyMods};
use ggez::input::mouse::{self, MouseButton};
use ggez::{Context, GameResult};

pub struct MyGame {
    engine: Engine<Space2D>,
    spawn_timer: f32,
    spawn_interval: f32,
    ready_to_spawn: bool,
    rect: graphics::Mesh,
    snapshot_rec: std::sync::mpsc::Receiver<WorldSnapshot<Space2D>>,
    latest_snapshot: Option<WorldSnapshot<Space2D>>,
}

impl MyGame {
    pub fn new(
        snapshot_rec: std::sync::mpsc::Receiver<WorldSnapshot<Space2D>>,
        engine: Engine<Space2D>,
        ctx: &mut Context,
    ) -> GameResult<Self> {
        // graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 800.0, 600.0))?;

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 50.0, 50.0),
            Color::RED,
        )?;

        Ok(Self {
            engine,
            spawn_timer: 0.0,
            spawn_interval: 1.0,
            ready_to_spawn: true,
            rect,
            snapshot_rec: snapshot_rec,
            latest_snapshot: None,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();

        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            if keyboard::active_mods(ctx).contains(KeyMods::SHIFT) {
                println!("RIGHT ARROW + SHIFT");
            }
            println!("RIGHT ARROW");
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            if keyboard::active_mods(ctx).contains(KeyMods::SHIFT) {
                println!("LEFT ARROW + SHIFT");
            }
            println!("LEFT ARROW");
        }

        if mouse::button_pressed(ctx, MouseButton::Left) && self.ready_to_spawn {
            let pos = mouse::position(ctx);

            let new_event = object_creation::<Space2D, engine_core::mint::Point2<f32>>(pos);

            println!("Spawn event {:?}", new_event);
            self.engine.push_event(new_event);

            self.ready_to_spawn = false;
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
        graphics::clear(ctx, Color::BLUE);

        let render_snap_ev = events::render_event_creation();
        self.engine.push_event(render_snap_ev);

        while let Ok(snapshot) = self.snapshot_rec.try_recv() {
            self.latest_snapshot = Some(snapshot);
        }

        if let Some(snapshot) = &self.latest_snapshot {
            for obj in &snapshot.objects {
                // println!("Draw object on position {:?}", obj);
                graphics::draw(
                    ctx,
                    &self.rect,
                    DrawParam::default().dest([obj.x, obj.y]).offset([0.0, 0.0]),
                )?;
            }
        }
        // graphics::draw(ctx, &self.rect, DrawParam::default().dest([100.0, 100.0]))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
