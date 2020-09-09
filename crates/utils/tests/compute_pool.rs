use std::sync::atomic::{AtomicUsize, Ordering};

use feather_utils::ComputePool;
use simple_logger::SimpleLogger;

#[test]
fn basic_compute() {
    let _ = SimpleLogger::new().init();
    let pool = ComputePool::new(4);

    static COUNT: AtomicUsize = AtomicUsize::new(0);
    let num_tasks = 100_000;
    for _ in 0..num_tasks {
        pool.spawn(async {
            COUNT.fetch_add(1, Ordering::Release);
        })
        .detach();
    }

    pool.shut_down();
    assert_eq!(COUNT.load(Ordering::Acquire), num_tasks);
}
