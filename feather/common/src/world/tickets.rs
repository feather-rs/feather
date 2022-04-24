use ahash::AHashMap;
use flume::{Receiver, Sender};
use libcraft::ChunkPosition;
use quill::world::ChunkTicket;

pub struct ChunkTickets {
    id_to_chunk: AHashMap<u64, ChunkPosition>,
    next_id: u64,
    dropped_receiver: Receiver<u64>,
    dropped_sender: Sender<u64>,

    chunk_ticket_counts: AHashMap<ChunkPosition, usize>,
}

impl ChunkTickets {
    pub fn new() -> Self {
        let (dropped_sender, dropped_receiver) = flume::unbounded();
        Self {
            id_to_chunk: AHashMap::new(),
            dropped_receiver,
            dropped_sender,
            chunk_ticket_counts: AHashMap::new(),
            next_id: 0,
        }
    }

    pub fn create_ticket(&mut self, chunk: ChunkPosition) -> ChunkTicket {
        let id = self.next_id();
        let ticket = ChunkTicket::new(id, self.dropped_sender.clone());

        self.id_to_chunk.insert(id, chunk);
        *self.chunk_ticket_counts.entry(chunk).or_default() += 1;

        ticket
    }

    pub fn poll_chunk_with_no_tickets(&mut self) -> Option<ChunkPosition> {
        for dropped_ticket_id in self.dropped_receiver.try_iter() {
            let chunk = self
                .id_to_chunk
                .remove(&dropped_ticket_id)
                .expect("ticket removed twice");
            let ticket_count = self.chunk_ticket_counts.get_mut(&chunk).unwrap();
            *ticket_count -= 1;

            if *ticket_count == 0 {
                self.chunk_ticket_counts.remove(&chunk);
                return Some(chunk);
            }
        }

        None
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
