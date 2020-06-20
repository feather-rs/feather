use crate::ScheduledBlockUpdateEvent;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct ScheduledTask {
    // The tick number at which to schedule this task
    at: u64,
    event: ScheduledBlockUpdateEvent,
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // The priority becomes greater when the target tick time is smaller
        self.at.cmp(&other.at).reverse()
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.at == other.at
    }
}

impl Eq for ScheduledTask {}

// ToDo: If there are still events scheduled when the server shuts down, they must be serialized
#[derive(Debug, Default)]
pub struct EventScheduler(BinaryHeap<ScheduledTask>);

impl EventScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn schedule_at(&mut self, ticks: u64, event: ScheduledBlockUpdateEvent) {
        self.0.push(ScheduledTask { at: ticks, event });
    }

    pub fn poll(&mut self, tick_count: u64) -> Vec<ScheduledBlockUpdateEvent> {
        let mut events = Vec::new();

        while self.hash_elements(tick_count) {
            events.push(
                self.0
                    .pop()
                    .expect("It was verified that this element exists!")
                    .event,
            );
        }

        events
    }

    pub fn hash_elements(&self, tick_count: u64) -> bool {
        self.0
            .peek()
            .map(|element| element.at <= tick_count)
            .unwrap_or(false)
    }
}
