mod game_class;

use crate::game_class::MyGame;

use ggez::ContextBuilder;
use ggez::conf;
use ggez::event::{self};

use engine_core::engine::Engine;
use engine_core::space::Space2D;

pub fn run() {
    println!("->>Engine_ggez main run");
    let (snapshot_snd, snapshot_rec) = std::sync::mpsc::channel();

    let engine: Engine<Space2D> = match Engine::<Space2D>::new(snapshot_snd) {
        Ok(en) => en,
        Err(_err) => {
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
        .window_mode(conf::WindowMode::default().dimensions(800.0, 800.0))
        .build()
        .expect("Could not create ggez context!");

    let my_game = MyGame::new(snapshot_rec, engine, &mut ctx).unwrap();

    event::run(ctx, event_loop, my_game);
}
