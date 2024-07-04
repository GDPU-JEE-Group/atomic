use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;
use crate::spinlock::SpinLock;
use crate::condvar::CondVar;

use std::thread;
// src/semaphore.rs
pub struct Semaphore {
    // your code here
    //TODO 信号量实现需要注意的2个问题
    // 1. 保证 信号量不会变成负数 ，越界了
    // 2. 东西多时，尽量不要锁的太厉害导致明明多线程，却变成一个一个拿
    tickets: AtomicIsize,
    condvar: Arc<(SpinLock, CondVar)>,
}

impl Semaphore {
    pub fn new(tickets: isize) -> Self {
        Semaphore {
            tickets: AtomicIsize::new(tickets),
            condvar: Arc::new((SpinLock::new(), CondVar::new())),
        }
    }

    pub fn acquire(&self) {
        let (lock, cvar) = &*self.condvar;
        let mut _guard = lock.lock();
        while self.tickets.fetch_sub(1, Ordering::AcqRel) <= 0 {
            _guard = cvar.wait(lock);
        }
    }

    pub fn release(&self) {
        let (_lock, cvar) = &*self.condvar;
        self.tickets.fetch_add(1, Ordering::Release);
        cvar.notify_one();
    }
}

fn _test() {
    let semaphore = Semaphore::new(1);
    let semaphore_arc = Arc::new(semaphore);

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let semaphore = Arc::clone(&semaphore_arc);
            thread::spawn(move || {
                semaphore.acquire();
                println!("Thread acquired semaphore");
                thread::sleep(std::time::Duration::from_secs(1));
                semaphore.release();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}