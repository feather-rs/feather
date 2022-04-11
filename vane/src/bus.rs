use flume::{Receiver, Sender};

use crate::Entities;

/// A `Bus` provides a connection to an `Entities` that allows
/// inserting events without having a mutable reference to the `Entities` structure.
///
/// A `Bus` may be useful in implementing custom change detection.
/// A component can contain a `Bus` and insert an event when it
/// detects a change.
///
/// Acquire a `Bus` by calling [`Entities::bus`](crate::Entities::bus).
#[derive(Clone)]
pub struct Bus {
    sender: Sender<Action>,
}

impl Bus {
    /// Executes a callback on the `Entities` immediately after the current system
    /// returns.
    pub fn defer(&self, action: impl FnOnce(&mut Entities) + 'static) {
        self.sender.send(Box::new(action)).ok();
    }
}

type Action = Box<dyn FnOnce(&mut Entities)>;

#[derive(Clone)]
pub(crate) struct BusReceiver {
    receiver: Receiver<Action>,
    bus: Bus,
}

impl Default for BusReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl BusReceiver {
    pub fn new() -> Self {
        let (sender, receiver) = flume::unbounded();
        Self {
            receiver,
            bus: Bus { sender },
        }
    }

    pub fn bus(&self) -> Bus {
        self.bus.clone()
    }

    pub fn drain(&self, entities: &mut Entities) {
        for action in self.receiver.try_iter() {
            (action)(entities);
        }
    }
}
