use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;

// src/main.rs
use atomic::Mutex;

use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    // _test();
    _server();
    // _clinet();
}

fn _test(){
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

fn _clinet(){
    match TcpStream::connect("127.0.0.1:8090") {
        Ok(_stream)=>{
            println!("成功连接服务器127.0.0.1:8090");
        }
        Err(e)=>{
            println!("连接失败 127.0.0.1:8090 {}",e);
        }
    }
}

fn _server(){
    println!("获取一个绑定127.0.0.1:8090的tcp监听者");
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    // unwrap 的使用是因为 bind 返回 Result<T,E>，毕竟监听是有可能报错的

    println!("开始监听循环，等待传入连接");
    for stream in listener.incoming(){
        println!("每当收到一个传入连接生成 tcpstream的结果类");
        let stream=stream.unwrap();
        println!("Connection established! 连接确立");

        println!("");
        handle_connection(stream);

    }
    println!("服务端结束!");
}
fn _v1_handle_connection(mut stream:TcpStream){
    let buf_reader=BufReader::new(&mut stream);
    let mut line= buf_reader.lines();
    let mut http_request=Vec::new();
    while let Some(result)=line.next() {
        match result {
            Ok(line)=>{
                if line.is_empty(){
                    break;
                }
                http_request.push(line);
            }
            Err(e)=>{
                eprintln!("Error reading {}",e);
                break;
            }
        
        }
    }

    println!("Request: {:#?}",http_request);
}

fn handle_connection(mut stream:TcpStream){
    let buf_reader=BufReader::new(&mut stream);
    let http_request:Vec<_>=buf_reader
        .lines()
        .map(|result|result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();
    println!("Request: {:#?}",http_request);
}
