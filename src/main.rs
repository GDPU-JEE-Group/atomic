use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::sync::Arc;

// src/main.rs
use atomic::Mutex;
mod config;
use config::read_config;

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

fn _server(){
    // 读取配置文件
    let config=read_config("config/app_config.toml");

    println!("获取一个绑定{}:{}的tcp监听者",config.server.ip,config.server.port);
    let listener = TcpListener::bind(format!("{}:{}",config.server.ip,config.server.port)).unwrap();
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

fn _v2_handle_connection(mut stream:TcpStream){
    let buf_reader=BufReader::new(&mut stream);
    let http_request:Vec<_>=buf_reader
        .lines()
        .map(|result|result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();
    println!("Request: {:#?}",http_request);
}

fn _v3_handle_connection(mut stream:TcpStream){
    //request
    let buf_reader=BufReader::new(&mut stream);
    let http_request:Vec<_>=buf_reader
        .lines()
        .map(|result|result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();
    println!("Request: {:#?}",http_request);

    //response
    let status_line="HTTP/1.1 200 OK";
    let contents =fs::read_to_string("res/index.html").unwrap();
    let length =contents.len();

    let response= format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\
        \r\n\
        {contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();

}

fn handle_connection(mut stream:TcpStream){
    //request
    let buf_reader=BufReader::new(&mut stream);
    let mut lines=buf_reader.lines();
    let request_line =lines.next().unwrap().unwrap();//先读请求行
    let (status_line,filename) = match request_line.as_str() {
        "GET / HTTP/1.1"|"GET /index.html HTTP/1.1" =>{("HTTP/1.1 200 OK", "index.html")},
        _=>{("HTTP/1.1 404 NOT FOUND", "404.html")}
    };
    //
    let request_body:Vec<_>=lines
        .map(|result|result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();
    println!("Request: {}\n{:#?}",request_line,request_body);

    //response
    let contents =fs::read_to_string(format!("res/{}",filename)).unwrap();
    let length =contents.len();

    let response= format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\
        \r\n\
        {contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();

}
// 大家可能会比较好奇，该如何判断客户端发来的 HTTP 数据是否读取完成，
// 答案就在于客户端会在请求数据的结尾附上两个换行符，当我们检测到某一行字符串为空时，
// 就意味着请求数据已经传输完毕，可以 collect 了。