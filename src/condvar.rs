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
        let _guard=self.mutex.lock();
        while !self.flag.load(Ordering::Acquire) {
            thread::yield_now();
        }
    }

    pub fn notify_one(&self){
        self.flag.store(true, Ordering::Release);
    }

    //TODO
    // 然而，这个实现有一个问题。在wait函数中，你在检查标志后立即让出线程，
    // 这可能会导致竞态条件。如果在检查标志和让出线程之间，另一个线程调用了notify_one，
    // 那么wait函数可能会错过通知并无限期地等待。
}