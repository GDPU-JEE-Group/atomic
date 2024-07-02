use std::sync::Arc;

// src/main.rs
use atomic::Mutex;

use std::net::TcpListener;
fn main() {
    println!("11111111111111111111!");

    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    // unwrap 的使用是因为 bind 返回 Result<T,E>，毕竟监听是有可能报错的
    println!("222222222222222!");

    for stream in listener.incoming(){
        println!("3333333333333333!");
        let stream=stream.unwrap();
        println!("Connection established!");
    }
    println!("44444444444444444!");

}

fn test(){
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
