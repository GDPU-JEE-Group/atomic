use std::f32::consts::E;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    _server();
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
fn handle_connection(mut stream:TcpStream){
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

    //listener.incoming() 返回一个迭代器，它在接收到新的连接时生成 Result<TcpStream, io::Error>。
    //由于没有任何连接进来，迭代器不会生成任何元素，因此循环体内的代码不会执行。