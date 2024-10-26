use std::{collections::HashMap, io::{self, Read, Write}, net::SocketAddr};
use mio::{event, Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream}; // 使用 mio 的 TcpStream
use crate::base::properties::Properties;

const SERVER: Token = Token(0);

pub fn main() -> io::Result<()> {
    only_epoll()
}


/* 
仅仅用epoll，
*/
fn only_epoll()->io::Result<()>{
    // 使用从配置中获取的 IP 和端口创建 SocketAddr
    let addr: SocketAddr = format!(
        "{}:{}",
        Properties::get("server.ip", "127.0.0.1"),
        Properties::get("server.port", "8090")
    )
    .parse()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?; // 转换错误类型

    let mut listener = TcpListener::bind(addr)?; // 传递 SocketAddr
    println!("服务器正在运行，监听 {}", addr);

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;

    let mut clients = HashMap::new();

    loop {
        poll.poll(&mut events, None)?; // 等待事件

        for event in &events {
            match event.token() {
                SERVER => {
                    match listener.accept() {
                        Ok((mut stream, _)) => { // 这里是 mio::net::TcpStream
                            println!("新连接: {:?}", stream.peer_addr());
                            let token = Token(clients.len() + 1);
                            poll.registry().register(&mut stream, token, Interest::READABLE)?;
                            clients.insert(token, stream);
                        }
                        Err(e) => {
                            eprintln!("接受连接时出错: {}", e);
                        }
                    }
                }
                token => {
                    let client_stream = clients.get_mut(&token).unwrap();
                    let mut buffer = vec![0; 1024];
                    match client_stream.read(&mut buffer) {
                        Ok(0) => {
                            // 连接已关闭
                            println!("连接已关闭: {:?}", client_stream.peer_addr());
                            clients.remove(&token);
                        }
                        Ok(n) => {
                            if let Err(e) = client_stream.write_all(&buffer[..n]) {
                                eprintln!("写入失败: {}", e);
                                clients.remove(&token); // 处理写入失败时的连接移除
                            }
                        }
                        Err(e) => {
                            eprintln!("读取失败: {}", e);
                            clients.remove(&token);
                        }
                    }
                }
            }
        }
    }
}
