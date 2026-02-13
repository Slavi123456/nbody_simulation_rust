use crate::errors::Error;
use crate::events::{self, EngineEvent, Event, EventResult};
use crate::space::Space;
use crate::world::{World, WorldSnapshot};

use std::collections::BinaryHeap;

use std::sync::mpsc::Receiver;
use std::time::Instant;

pub struct Engine<S: Space> {
    // world: World<S>,
    task_sender: std::sync::mpsc::Sender<EngineEvent<S>>,
    snapshot_sender: std::sync::mpsc::Sender<WorldSnapshot<S>>,
}

impl<S> Engine<S>
where
    S: Space + std::fmt::Debug + 'static,
    <S as Space>::Vec: Send + Clone,
{
    pub fn new(
        snapshot_sender: std::sync::mpsc::Sender<WorldSnapshot<S>>,
        window_dim: [f32; 2],
    ) -> Result<Self, Error> {
        let (sender, receiver) = std::sync::mpsc::channel();

        let snap_sender_copy = snapshot_sender.clone();
        let mut world = World::<S>::new(window_dim);

        std::thread::spawn(move || Self::engine_loop(receiver, &mut world, snap_sender_copy));

        Ok(Engine::<S> {
            task_sender: sender,
            snapshot_sender: snapshot_sender,
        })
    }

    pub fn push_event(&mut self, event: events::EngineEvent<S>)
    where
        S::Vec: std::fmt::Debug,
    {
        let mut repeat_try_event = 0;
        while repeat_try_event <= 3 {
            println!("Event to send {:?}", event);
            if self.task_sender.send(event.clone()).is_ok() {
                return;
            }
            // println!("Error {:?}", Error::source());
            repeat_try_event += 1;
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        eprintln!("Failed to send event after 3 retries");
    }

    pub fn engine_loop(
        receiver: Receiver<EngineEvent<S>>,
        mut world: &mut World<S>,
        snapshot_sender: std::sync::mpsc::Sender<WorldSnapshot<S>>,
    ) where
        <S as Space>::Vec: Clone,
    {
        // let mut world = World::<S>::new();
        let mut queue: BinaryHeap<events::EngineEvent<S>> = BinaryHeap::new();

        const FIXED_DT: f32 = 1.0 / 120.0;
        let mut accumulator = 0.0;
        let mut last = std::time::Instant::now();

        loop {
            // measure frame time
            let now = Instant::now();
            let frame_dt = (now - last).as_secs_f32();
            last = now;

            accumulator += frame_dt;

            //Process momentum tasks like Creation Object; ApplyForce etc.
            while let Ok(event) = receiver.try_recv() {
                queue.push(event);
            }

            while let Some(event) = queue.pop() {
                println!("Event to dispatch {:?}", event);
                Self::dispatcher_event(event, &mut world, &snapshot_sender);
            }

            // Fixed physics step
            while accumulator >= FIXED_DT {
                let snapshot = world.physics_snapshot();

                world.apply_gravity(frame_dt);
                world.move_objects(&snapshot, FIXED_DT);

                let collisions = world.handle_collision();

                world.resolve_collisions(collisions);

                accumulator -= FIXED_DT;
            }

            // Send RenderSnapshot
            let _ = snapshot_sender.send(world.render_snapshot());

            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    // fn dispatcher_loop(
    //     receiver: Receiver<EngineEvent<S>>,
    //     world: &mut World<S>,
    //     snapshot_sender: std::sync::mpsc::Sender<WorldSnapshot<S>>,
    // ) where
    //     <S as Space>::Vec: Clone,
    // {
    //     let mut queue: BinaryHeap<events::EngineEvent<S>> = BinaryHeap::new();

    //     while let Ok(event) = receiver.recv() {
    //         queue.push(event);

    //         while let Some(event) = queue.pop() {
    //             Self::dispatcher_event(event, world, &snapshot_sender);
    //         }
    //     }
    // }

    fn dispatcher_event(
        engine_event: events::EngineEvent<S>,
        world: &mut World<S>,
        snapshot_sender: &std::sync::mpsc::Sender<WorldSnapshot<S>>,
    ) where
        <S as Space>::Vec: Clone,
    {
        match engine_event {
            EngineEvent::Simple { event, .. } => {
                let _ = Self::handle_event(event, world, snapshot_sender);
            }

            EngineEvent::WithResponse {
                event, response_tx, ..
            } => {
                let result = Self::handle_event(event, world, snapshot_sender);
                let _ = response_tx.send(result);
            }
        }
    }

    fn handle_event(
        event: Event<S>,
        world: &mut World<S>,
        snapshot_sender: &std::sync::mpsc::Sender<WorldSnapshot<S>>,
    ) -> EventResult
    where
        S: Space,
    {
        match event {
            Event::ObjectCreation {
                position,
                radius,
                mass,
            } => {
                println!("Worked event ObjectCreation with position {:?}", position);
                let id = world.create_object(position, radius, mass);
                EventResult::ObjectCreated { id }
            }

            // Event::RenderSnapshotCreation() => {
            //     snapshot_sender.send(world.render_snapshot()).unwrap();
            //     EventResult::Nothing
            // }
            Event::ApplyForce {
                object_id,
                velocity,
            } => {
                println!(
                    "Worked event ApplyForce on objecgt {:?} with velocity {:?}",
                    object_id, velocity
                );
                world.apply_force(object_id, velocity);
                EventResult::Nothing
            }
        }
    }
}
