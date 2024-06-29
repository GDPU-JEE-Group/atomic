use std::sync::Arc;

// src/main.rs
use atomic::Mutex;


fn main() {
    // your code here
    println!("hello,chaixiang!");
    let mut i=1;
    //TODO
    let x=Arc::new(Mutex::new(0));
    x.lock();
    i=i-1;
    println!("{}",i);
    x.unlock();


}
