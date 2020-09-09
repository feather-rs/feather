use std::{sync::atomic::AtomicUsize, sync::atomic::Ordering, thread::sleep, time::Duration};

use feather_utils::BlockingPool;
use simple_logger::SimpleLogger;

#[test]
fn basic_blocking() {
    let _ = SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init();
    let pool = BlockingPool::new();

    static COUNT: AtomicUsize = AtomicUsize::new(0);
    let num_tasks = 1_000;

    for _ in 0..num_tasks {
        pool.spawn(async {
            do_some_io();
            COUNT.fetch_add(1, Ordering::Release);
        })
        .detach();
    }

    pool.shut_down();
    assert_eq!(COUNT.load(Ordering::Acquire), num_tasks);
}

/// Fake blocking task.
fn do_some_io() {
    sleep(Duration::from_millis(50));
}
