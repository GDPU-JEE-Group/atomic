// src/mutex.rs
use std::sync::atomic::{AtomicBool,Ordering};
pub struct Mutex {
    // your code here
    locked:AtomicBool,
}

impl Mutex {
    pub fn new()->Mutex{
        Mutex { locked: AtomicBool::new(false) }
    }

    pub fn lock(&self){
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
        }
    }

    pub fn unlock(&self){
        self.locked.store(false, Ordering::Release);
    }

}
// - `Ordering::Relaxed`：不做任何同步保证，只保证操作的原子性。
// - `Ordering::Acquire`：操作之前的所有读写不会被重排序到该操作之后。
// - `Ordering::Release`：操作之后的所有读写不会被重排序到该操作之前。
// - `Ordering::AcqRel`：结合 `Acquire` 和 `Release` 的效果。
// - `Ordering::SeqCst`：顺序一致性，确保所有操作按程序顺序执行。


// compare_and_swap，compare_exchange和compare_exchange_weak都是原子操作，用于在多线程环境中安全地读取和修改共享数据。他们的主要区别在于他们的行为和性能特性：

// compare_and_swap：这是一个较旧的方法，已经被弃用。它会比较当前值和预期值，如果它们相等，就用新值替换当前值1。
// compare_exchange：这是compare_and_swap的替代方法。它的行为和compare_and_swap类似，但是它提供了更多的灵活性，允许你指定在成功和失败时使用的内存顺序1。
// compare_exchange_weak：这个方法和compare_exchange类似，但是它可能会"虚假"地失败，即使当前值和预期值相等12。这是因为在某些处理器架构上，使用compare_exchange_weak可以提供更好的性能12。但是，由于它可能会虚假地失败，所以通常需要在一个循环中使用它，以便在失败时重试3。


#[cfg(test)]
mod tests {
    use super::Mutex;
    use std::{sync::{atomic::Ordering, Arc}, thread};

    #[test]
    fn test_mutex() {
        let mutex = Arc::new(Mutex::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let mutex = Arc::clone(&mutex);
            let handle = thread::spawn(move || {
                mutex.lock();
                mutex.unlock();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // If we reach this point, it means that no thread has panicked. Therefore, the test is successful.
    }

    #[test]
    fn test_mutex_is_mutex() {
        let m = Mutex::new();
        m.lock();
        let try_lock = m.locked.load(Ordering::Relaxed);
        assert_eq!(try_lock, true);
        m.unlock();
    }

    #[test]
    fn test_mutex_unlocks() {
        let m = Mutex::new();
        m.lock();
        m.unlock();
        let try_lock = m.locked.load(Ordering::Relaxed);
        assert_eq!(try_lock, false);
    }
}