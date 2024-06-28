use std::sync::Arc;

// src/main.rs
use atomic::Mutex;
use atomic::{Condvar};
use atomic::Semaphore;


fn main() {
    // your code here
    println!("hello,chaixiang!");
    let mut i=1;
    //TODO
    let x=Arc::new(Mutex::new());
    x.lock();
    i=i-1;
    x.unlock();


}
