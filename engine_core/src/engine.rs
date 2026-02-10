use crate::errors::Error;
use crate::events::{self, PrioritizedEvent};
use crate::space::Space;
use crate::world::World;

use std::collections::BinaryHeap;

pub struct Engine<S: Space> {
    world: World<S>,
    tasks: BinaryHeap<events::PrioritizedEvent<S>>,
    task_sender: std::sync::mpsc::Sender<PrioritizedEvent<S>>,
}

impl<S> Engine<S>
where
    S: Space + std::fmt::Debug,
{
    pub fn new() -> Result<Self, Error> {
        let (sender, receiver) = std::sync::mpsc::channel();

        Ok(Engine::<S> {
            world: World::<S>::new()?,
            tasks: BinaryHeap::<events::PrioritizedEvent<S>>::new(),
            task_sender: sender,
        })
    }

    pub fn push_event(&mut self, event: events::PrioritizedEvent<S>)
    where
        S::Vec: std::fmt::Debug,
    {
        println!("Event is added {:?}", event);
        self.tasks.push(event);
        println!("Priority queue of events {:?}", self.tasks);
    }
}
