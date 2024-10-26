use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::SocketAddr,
};
use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream}; // 使用 mio 的 TcpStream
use crate::base::properties::Properties;

const SERVER: Token = Token(0);

pub fn main() -> io::Result<()> {
    only_epoll()
}

/* 
仅仅用epoll
*/
fn only_epoll() -> io::Result<()> {
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
                        Ok((mut stream, _)) => {
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
                    // 提前从 `clients` 中移除连接，防止多次操作
                    if let Some(mut client_stream) = clients.remove(&token) {
                        let mut buffer = vec![0; 1024];
                        match client_stream.read(&mut buffer) {
                            Ok(0) => {
                                // 连接已关闭
                                println!("连接已关闭: {:?}", client_stream.peer_addr());
                                if let Err(e) = poll.registry().deregister(&mut client_stream) {
                                    eprintln!("注销连接失败: {:?}", e);
                                }
                                // 彻底释放连接资源
                                drop(client_stream);
                            }
                            Ok(n) => {
                                // 将读取到的字节转换成字符串并打印
                                let message = String::from_utf8_lossy(&buffer[..n]);
                                println!("收到消息: {}", message);

                                // 将消息发送回客户端
                                if let Err(e) = client_stream.write_all(&buffer[..n]) {
                                    eprintln!("写入失败: {}", e);
                                    if let Err(deg_err) = poll.registry().deregister(&mut client_stream) {
                                        eprintln!("注销连接失败: {:?}", deg_err);
                                    }
                                    // 彻底释放连接资源
                                    drop(client_stream);
                                } else {
                                    // 若写入成功，将连接重新加入 clients
                                    clients.insert(token, client_stream);
                                }
                            }
                            Err(e) => {
                                // 读取失败，关闭连接
                                eprintln!("读取失败: {}", e);
                                if let Err(deg_err) = poll.registry().deregister(&mut client_stream) {
                                    eprintln!("注销连接失败: {:?}", deg_err);
                                }
                                // 彻底释放连接资源
                                drop(client_stream);
                            }
                        }
                    }
                }
            }
        }
    }
}
