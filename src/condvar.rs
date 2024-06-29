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
//     这个条件变量的实现有一些问题需要注意：

//     竞态条件：在wait函数中，你先解锁了互斥锁，然后调用了thread::park。这之间存在一个竞态条件，如果在这个时间窗口内notify_one被调用，那么通知就会丢失，因为线程还没有进入park状态。
//     唤醒的线程：在notify_one函数中，你唤醒的是当前线程，而不是等待在条件变量上的线程。你应该保存等待线程的句柄，并在notify_one中唤醒它。
//     互斥锁的使用：在wait函数中，你在循环中多次获取和释放互斥锁，这可能会导致性能问题。你应该在整个等待过程中保持互斥锁的锁定状态。
//     条件变量的复用：你的实现中，一旦条件变量被通知，flag就被设置为true，并且再也不能被重置为false。这意味着这个条件变量不能被复用。你可能需要添加一个reset方法来重置flag的状态。
}

//总结
// 1. 获取和通知操作都被锁了
// 2. 在线程让出来时，暂时解锁一会，不影响其他线程，提高的效率
// 3. 无论是条件变量还是sock的非阻塞，都是尽量减少线程占用资源但不做事的情况，减少不必要的停留