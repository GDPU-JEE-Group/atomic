use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct SpinLock {
    lock: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        SpinLock {
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.lock.compare_exchange(false, true, Ordering::Acquire,Ordering::Relaxed).is_err() {
            while self.lock.load(Ordering::Relaxed) {
                thread::yield_now(); // 自旋等待
            }
        }
    }

    pub fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}
fn test() {
    let spinlock = Arc::new(SpinLock::new());

    let spinlock_clone = Arc::clone(&spinlock);
    let handle = thread::spawn(move || {
        spinlock_clone.lock();
        println!("Thread 1: acquired spinlock");
        thread::sleep(std::time::Duration::from_secs(2));
        println!("Thread 1: releasing spinlock");
        spinlock_clone.unlock();
    });

    thread::sleep(std::time::Duration::from_millis(100)); // 确保 Thread 1 先获取锁

    let spinlock_clone = Arc::clone(&spinlock);
    let handle2 = thread::spawn(move || {
        spinlock_clone.lock();
        println!("Thread 2: acquired spinlock");
        spinlock_clone.unlock();
    });

    handle.join().unwrap();
    handle2.join().unwrap();
}