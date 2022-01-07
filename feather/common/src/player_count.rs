use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

#[derive(Debug)]
pub struct MaxPlayersReached;

/// Maintains the server player count.
///
/// Can be cloned to create a new handle.
#[derive(Clone)]
pub struct PlayerCount {
    inner: Arc<Inner>,
}

impl PlayerCount {
    pub fn new(max_players: u32) -> Self {
        Self {
            inner: Arc::new(Inner {
                count: AtomicU32::new(0),
                max_players,
            }),
        }
    }

    pub fn try_add_player(&self) -> Result<(), MaxPlayersReached> {
        loop {
            let current_count = self.inner.count.load(Ordering::SeqCst);
            let new_count = current_count + 1;
            if new_count > self.inner.max_players {
                return Err(MaxPlayersReached);
            }

            if self
                .inner
                .count
                .compare_exchange(current_count, new_count, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return Ok(());
            }
        }
    }

    pub fn remove_player(&self) {
        self.inner.count.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn get(&self) -> u32 {
        self.inner.count.load(Ordering::Acquire)
    }
}

struct Inner {
    count: AtomicU32,
    max_players: u32,
}

#[cfg(test)]
mod tests {
    use crossbeam_utils::thread;

    use super::*;

    #[test]
    fn try_add() {
        let count = PlayerCount::new(1);
        assert_eq!(count.get(), 0);
        count.try_add_player().unwrap();
        assert_eq!(count.get(), 1);

        for _ in 0..10 {
            count.try_add_player().unwrap_err();
            assert_eq!(count.get(), 1);
        }
    }

    #[test]
    fn no_race_conditions() {
        let threads = 8;
        let players_per_thread = 100000;
        let max_players = threads * players_per_thread / 2;

        let count = PlayerCount::new(max_players);

        let num_added = AtomicU32::new(0);

        thread::scope(|s| {
            for _ in 0..threads {
                s.spawn(|_| {
                    for _ in 0..players_per_thread {
                        if count.try_add_player().is_ok() {
                            num_added.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                });
            }
        })
        .unwrap();

        assert_eq!(num_added.load(Ordering::SeqCst), max_players);
        assert_eq!(count.get(), max_players);
    }
}
