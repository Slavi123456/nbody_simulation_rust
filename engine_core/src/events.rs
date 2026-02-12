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
    ObjectCreation { position: S::Vec, radius: f32 },
    ApplyForce { object_id: usize, velocity: S::Vec },
    // RenderSnapshotCreation(),
}

impl<S: Space> Clone for Event<S>
where
    S::Vec: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Event::ObjectCreation { position, radius } => Event::ObjectCreation {
                position: position.clone(),
                radius: *radius,
            },
            Event::ApplyForce {
                object_id,
                velocity,
            } => Event::ApplyForce {
                object_id: *object_id,
                velocity: velocity.clone(),
            },
            // Event::RenderSnapshotCreation() => Event::RenderSnapshotCreation(),
        }
    }
}
pub struct EventWithResponse<S: Space> {
    event: Event<S>,
    response: std::sync::mpsc::Receiver<EventResult>,
}
#[derive(Debug)]
pub enum EngineEvent<S: Space> {
    Simple {
        event: Event<S>,
        priority: Priority,
    },
    WithResponse {
        event: Event<S>,
        priority: Priority,
        response_tx: std::sync::mpsc::Sender<EventResult>,
    },
}

impl<S: Space> EngineEvent<S> {
    fn priority(&self) -> Priority {
        match self {
            EngineEvent::Simple { priority, .. } => *priority,
            EngineEvent::WithResponse { priority, .. } => *priority,
        }
    }

    pub fn event(&self) -> Event<S> {
        match self {
            EngineEvent::Simple { event, .. } => event.clone(),
            EngineEvent::WithResponse { event, .. } => event.clone(),
        }
    }
}

impl<S: Space> Ord for EngineEvent<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority().cmp(&self.priority())
    }
}
impl<S: Space> PartialOrd for EngineEvent<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<S: Space> PartialEq for EngineEvent<S> {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}
impl<S: Space> Eq for EngineEvent<S> {}

pub enum EventResult {
    ObjectCreated { id: usize },
    Nothing,
}

///This could be factory pattern for creation
pub fn object_creation<S, P>(
    pos: P,
    radius: f32,
    sender_event_result: std::sync::mpsc::Sender<EventResult>,
) -> EngineEvent<S>
where
    S: Space,
    P: IntoSpaceVec<S>,
{
    EngineEvent::WithResponse {
        event: Event::ObjectCreation {
            position: pos.into_space_vec(),
            radius: radius,
        },
        priority: Priority::High,
        response_tx: sender_event_result,
    }
}

// pub fn render_event_creation<S>() -> EngineEvent<S>
// where
//     S: Space,
// {
//     EngineEvent::Simple {
//         event: Event::RenderSnapshotCreation(),
//         priority: Priority::Low,
//     }
// }

pub fn apply_force_event_creation<S>(target_id: usize, velocity: S::Vec) -> EngineEvent<S>
where
    S: Space,
{
    EngineEvent::Simple {
        event: Event::ApplyForce {
            object_id: target_id,
            velocity: velocity,
        },
        priority: Priority::Medium,
    }
}
