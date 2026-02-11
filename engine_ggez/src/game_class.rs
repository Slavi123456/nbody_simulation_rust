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
use engine_core::events::EventResult;
use engine_core::space::Space2D;
use engine_core::world::WorldSnapshot;
use engine_core::{events, object_creation};

use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam};
use ggez::input::keyboard::{self, KeyCode, KeyMods};
use ggez::{Context, GameResult};

use crate::custom_input;
use crate::render_body::RenderBody;
pub struct MyGame {
    engine: Engine<Space2D>,
    spawn_timer: f32,
    spawn_interval: f32,
    ready_to_spawn: bool,
    body_mesh: RenderBody,
    snapshot_rec: std::sync::mpsc::Receiver<WorldSnapshot<Space2D>>,
    latest_snapshot: Option<WorldSnapshot<Space2D>>,
    mouse_state: custom_input::MouseState,
    pending_event_resp: Vec<std::sync::mpsc::Receiver<EventResult>>,
}

impl MyGame {
    pub fn new(
        snapshot_rec: std::sync::mpsc::Receiver<WorldSnapshot<Space2D>>,
        engine: Engine<Space2D>,
        ctx: &mut Context,
    ) -> GameResult<Self> {
        // graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 800.0, 600.0))?;

        Ok(Self {
            engine,
            spawn_timer: 0.0,
            spawn_interval: 1.0,
            ready_to_spawn: true,
            body_mesh: RenderBody::new(10.0, ctx)?,
            snapshot_rec: snapshot_rec,
            latest_snapshot: None,
            mouse_state: custom_input::MouseState::new(),
            pending_event_resp: Vec::new(),
        })
    }

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        while let Ok(snapshot) = self.snapshot_rec.try_recv() {
            self.latest_snapshot = Some(snapshot);
        }

        if let Some(snapshot) = &self.latest_snapshot {
            for obj in &snapshot.objects {
                // println!("Draw object on position {:?}", obj);
                graphics::draw(
                    ctx,
                    &self.body_mesh.sphere,
                    DrawParam::default().dest([obj.x, obj.y]).offset([0.0, 0.0]),
                )?;
            }
        }
        Ok(())
    }

    fn update_timer(&mut self, delta_time: f32) {
        if !self.ready_to_spawn {
            self.spawn_timer += delta_time;
            if self.spawn_timer >= self.spawn_interval {
                self.spawn_timer = 0.0;
                self.ready_to_spawn = true;
            }
        }
    }
    fn process_pending_responses(&mut self) {
        self.pending_event_resp
            .retain_mut(|receiver| match receiver.try_recv() {
                Ok(result) => {
                    match result {
                        events::EventResult::ObjectCreated { id } => {
                            println!("Selected object is with id {:?}", id);
                        }
                        events::EventResult::Nothing => {}
                    }
                    false
                }
                Err(err) => true,
            });
    }
    fn handle_input(&mut self, ctx: &Context, delta_time: f32) {
        self.process_pending_responses();
        self.mouse_state.determine_input(delta_time, ctx);

        match self.mouse_state.state {
            custom_input::MouseInput::Idle => {}
            custom_input::MouseInput::Clicked { pos } => {
                println!("Mouse Clicked!!!");

                if let Some(snapshot) = self.latest_snapshot.as_ref() {
                    if self.ready_to_spawn
                        && !snapshot.is_click_on_object(pos, self.body_mesh.radius())
                    {
                        let (sender, receiver) = std::sync::mpsc::channel();
                        let new_event =
                            object_creation::<Space2D, engine_core::mint::Point2<f32>>(pos, sender);

                        println!("Spawn event {:?}", new_event);
                        self.engine.push_event(new_event);
                        self.pending_event_resp.push(receiver);

                        self.ready_to_spawn = false;
                    }
                }
            }
            custom_input::MouseInput::Holding { pos, dur } => {
                println!("Mouse Holding!!! in position {:?} with dur {:?}", pos, dur);
            }
            custom_input::MouseInput::Dragging {
                start_pos,
                curr_pos,
            } => {
                println!(
                    "Mouse Dragging!!! with starting pos {:?} and curr pos {:?}",
                    start_pos, curr_pos
                );
            }
            _ => {}
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();

        self.handle_input(ctx, delta_time);
        self.update_timer(delta_time);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        let render_snap_ev = events::render_event_creation();
        self.engine.push_event(render_snap_ev);

        if let Err(err) = self.render(ctx) {
            println!("->> Error in drawing {:?}", err);
        }
        // graphics::draw(ctx, &self.rect, DrawParam::default().dest([100.0, 100.0]))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
