use std::f32::consts::E;
use std::sync::Arc;

// src/main.rs
use atomic::Mutex;

use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    match TcpStream::connect("127.0.0.1:8090") {
        Ok(_stream)=>{
            println!("成功连接服务器127.0.0.1:8090");
        }
        Err(e)=>{
            println!("连接失败 127.0.0.1:8090 {}",e);
        }
    }
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

fn _server(){
    println!("11111111111111111111!");

    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    // unwrap 的使用是因为 bind 返回 Result<T,E>，毕竟监听是有可能报错的
    println!("222222222222222!");


    //listener.incoming() 返回一个迭代器，它在接收到新的连接时生成 Result<TcpStream, io::Error>。
    //由于没有任何连接进来，迭代器不会生成任何元素，因此循环体内的代码不会执行。
    for stream in listener.incoming(){
        println!("3333333333333333!");
        let stream=stream.unwrap();
        println!("Connection established!");
    }
    println!("44444444444444444!");
}
