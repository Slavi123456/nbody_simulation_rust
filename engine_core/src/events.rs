use crate::mint_transform::IntoSpaceVec;
use crate::space::Space;
use std::cmp::Ordering;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Priority {
    High = 2,
    Medium = 1,
    Low = 0,
}

#[derive(Debug)]
pub enum Event<S: Space> {
    ObjectCreation { position: S::Vec },
    RenderSnapshotCreation(),
}

#[derive(Debug)]
pub struct PrioritizedEvent<S: Space> {
    pub event: Event<S>,
    pub priority: Priority,
}

impl<S: Space> Ord for PrioritizedEvent<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse so High is popped first
        other.priority.cmp(&self.priority)
    }
}

impl<S: Space> PartialOrd for PrioritizedEvent<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: Space> PartialEq for PrioritizedEvent<S> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<S: Space> Eq for PrioritizedEvent<S> {}

///This could be factory pattern for creation

pub fn object_creation<S, P>(pos: P) -> PrioritizedEvent<S>
where
    S: Space,
    P: IntoSpaceVec<S>,
{
    PrioritizedEvent {
        event: Event::ObjectCreation {
            position: pos.into_space_vec(),
        },
        priority: Priority::High,
    }
}

pub fn render_event_creation<S>() -> PrioritizedEvent<S>
where
    S: Space,
{
    PrioritizedEvent {
        event: Event::RenderSnapshotCreation(),
        priority: Priority::Low,
    }
}
