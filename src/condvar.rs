// src/condvar.rs

use std::{sync::atomic::{AtomicBool, Ordering}};
use std::thread;
use crate::mutex::{Mutex};

pub struct Condvar {
    // your code here
    //TODO 条件变量 为了防止群起群停
    flag:AtomicBool,
    mutex: Mutex
}

impl Condvar {
    pub fn new()->Condvar{
        Condvar{
            flag: AtomicBool::new(false),
            mutex: Mutex::new(),
        }
    }

    pub fn wait(&self){
        let guard=self.mutex.lock();//这样当_guard结束时锁会被释放
        while !self.flag.load(Ordering::Acquire) {
            self.mutex.unlock();
            thread::park();
            // 在你的原始实现中，thread::yield_now() 只是简单地将当前线程的时间片让给其他线程。
            // 这个操作本身并不能确保当前线程在条件满足后立即被唤醒，尤其是在高负载的情况下，
            // 操作系统可能会长时间不调度当前线程，导致等待时间变长。
            let guard=self.mutex.lock();
        }
    }

    pub fn notify_one(&self){
        let _guard=self.mutex.lock();
        self.flag.store(true, Ordering::Release);
        let handle=thread::current();

        // 唤醒等待的线程
        // 找到需要唤醒的线程并调用 unpark
        handle.unpark();
    }

    //TODO
    // 然而，这个实现有一个问题。在wait函数中，你在检查标志后立即让出线程，
    // 这可能会导致竞态条件。如果在检查标志和让出线程之间，另一个线程调用了notify_one，
    // 那么wait函数可能会错过通知并无限期地等待。
}

//总结
// 1. 获取和通知操作都被锁了
// 2. 在线程让出来时，暂时解锁一会，不影响其他线程，提高的效率
// 3. 无论是条件变量还是sock的非阻塞，都是尽量减少线程占用资源但不做事的情况，减少不必要的停留