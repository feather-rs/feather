use std::time::Instant;

use base::TICK_DURATION;

/// Utility to invoke a function in a tick loop, once
/// every 50ms.
pub struct TickLoop {
    function: Box<dyn FnMut() -> bool>,
}

impl TickLoop {
    /// Creates a `TickLoop`. The given `function` is called
    /// each tick. Returning `true` from `function` causes the
    /// tick loop to exit.
    pub fn new(function: impl FnMut() -> bool + 'static) -> Self {
        Self {
            function: Box::new(function),
        }
    }

    /// Runs the tick loop until the callback returns `true`.
    pub fn run(mut self) {
        loop {
            let start = Instant::now();
            let should_exit = (self.function)();
            if should_exit {
                return;
            }

            let elapsed = start.elapsed();
            if elapsed > TICK_DURATION {
                log::warn!("Tick took too long ({:?})", elapsed);
            } else {
                std::thread::sleep(TICK_DURATION - elapsed);
            }
        }
    }
}
