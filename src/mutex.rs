use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::cell::UnsafeCell;

pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        // 自旋锁实现
        while self.lock.compare_exchange(false, true, Ordering::Acquire,Ordering::Relaxed).is_err() {
            while self.lock.load(Ordering::Relaxed) {
                thread::yield_now();
            }
        }
        MutexGuard { mutex: self }
    }

    pub fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

impl<'a, T> std::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> std::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}


fn test() {
    let mutex = Mutex::new(0);
    let mutex = std::sync::Arc::new(mutex);

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let mutex = std::sync::Arc::clone(&mutex);
            thread::spawn(move || {
                for _ in 0..1000 {
                    let mut guard = mutex.lock();
                    *guard += 1;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *mutex.lock());
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
