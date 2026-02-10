mod game_class;

use crate::game_class::MyGame;

use ggez::event::{self, EventHandler};
use ggez::ContextBuilder;

use engine_core::engine::Engine;
use engine_core::space::Space2D;

pub fn run() {
    println!("->>Engine_ggez main run");

    let engine: Engine<Space2D> = match Engine::<Space2D>::new() {
        Ok(en) => en,
        Err(err) => {
            return;
        }
    };
    println!("->>Successfully builded Engine");

    // ggez setup
    // event loop:
    //  - input → engine.push_event
    //  - engine.tick(dt)
    //  - snapshot → draw

    // Usual ggez start
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "N-body Sandbox Simulation")
        .build()
        .expect("Could not create ggez context!");

    let my_game = MyGame::new(engine);

    event::run(ctx, event_loop, my_game);
}
