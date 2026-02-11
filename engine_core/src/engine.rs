use crate::errors::Error;
use crate::events::{self, EngineEvent, Event, EventResult};
use crate::space::Space;
use crate::world::{World, WorldSnapshot};

use std::collections::BinaryHeap;
use std::sync::mpsc::Receiver;

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
    pub fn new(snapshot_sender: std::sync::mpsc::Sender<WorldSnapshot<S>>) -> Result<Self, Error> {
        let (sender, receiver) = std::sync::mpsc::channel();

        let mut world = World::<S>::new()?;
        let snap_sender_copy = snapshot_sender.clone();
        std::thread::spawn(move || Self::dispatcher_loop(receiver, &mut world, snap_sender_copy));

        Ok(Engine::<S> {
            task_sender: sender,
            snapshot_sender: snapshot_sender,
        })
    }

    pub fn push_event(&mut self, event: events::EngineEvent<S>)
    where
        S::Vec: std::fmt::Debug,
    {
        // println!("Event is added {:?}", event);

        //Should i do some error handling
        self.task_sender.send(event).unwrap();
    }

    fn dispatcher_loop(
        receiver: Receiver<EngineEvent<S>>,
        world: &mut World<S>,
        snapshot_sender: std::sync::mpsc::Sender<WorldSnapshot<S>>,
    ) where
        <S as Space>::Vec: Clone,
    {
        let mut queue: BinaryHeap<events::EngineEvent<S>> = BinaryHeap::new();

        while let Ok(event) = receiver.recv() {
            queue.push(event);

            while let Some(event) = queue.pop() {
                Self::dispatcher_event(event, world, &snapshot_sender);
            }
        }
    }

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
            Event::ObjectCreation { position } => {
                println!("Worked event ObjectCreation with position {:?}", position);
                let id = world.create_object(position);
                EventResult::ObjectCreated { id }
            }

            Event::RenderSnapshotCreation() => {
                snapshot_sender.send(world.render_snapshot()).unwrap();
                EventResult::Nothing
            }
        }
    }
}
