use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use crate::spinlock::SpinLock;


pub struct CondVar {
    flag: AtomicBool,
    waiters: AtomicUsize,
}

impl CondVar {
    pub fn new() -> Self {
        CondVar {
            flag: AtomicBool::new(false),
            waiters: AtomicUsize::new(0),
        }
    }

    pub fn wait(&self, lock: &SpinLock) {
        self.waiters.fetch_add(1, Ordering::SeqCst);
        while !self.flag.load(Ordering::Acquire) {
            lock.unlock(); // 释放锁
            thread::yield_now(); // 让出线程时间片
            lock.lock(); // 重新获取锁
        }
        self.waiters.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn notify_one(&self) {
        self.flag.store(true, Ordering::Release);
        // 暂时没有更好的裸机环境下的唤醒机制，只能依赖轮询和线程调度
    }

    pub fn reset(&self) {
        self.flag.store(false, Ordering::Release);
    }
}


//总结
// 1. 获取和通知操作都被锁了
// 2. 在线程让出来时，暂时解锁一会，不影响其他线程，提高的效率
// 3. 无论是条件变量还是sock的非阻塞，都是尽量减少线程占用资源但不做事的情况，减少不必要的停留