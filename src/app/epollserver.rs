use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::SocketAddr,
};
use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};
use crate::base::properties::Properties;

const SERVER: Token = Token(0);

pub fn main() -> io::Result<()> {
    only_epoll()
}

fn only_epoll() -> io::Result<()> {
    let addr: SocketAddr = format!(
        "{}:{}",
        Properties::get("server.ip", "127.0.0.1"),
        Properties::get("server.port", "8090")
    )
    .parse()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let mut listener = TcpListener::bind(addr)?;
    println!("服务器正在运行，监听 {}", addr);

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;

    let mut clients = HashMap::new();

    loop {
        poll.poll(&mut events, None)?;

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
                    if let Some(mut client_stream) = clients.remove(&token) {  // 从 HashMap 中移除并获取所有权
                        let mut buffer = vec![0; 1024];
                        match client_stream.read(&mut buffer) {
                            Ok(0) => {
                                // 连接已关闭
                                println!("连接已关闭: {:?}", client_stream.peer_addr());
                                // 处理断开连接
                                handle_disconnect(client_stream, token, &mut poll, &mut clients);
                            }
                            Ok(n) => {
                                // 将读取到的字节转换成字符串并打印
                                let message = String::from_utf8_lossy(&buffer[..n]);
                                println!("收到消息: {}", message);
                                if let Err(e) = client_stream.write_all(&buffer[..n]) {
                                    eprintln!("写入失败: {}", e);
                                    // 处理写入失败
                                    handle_disconnect(client_stream, token, &mut poll, &mut clients);
                                }
                            }
                            Err(e) => {
                                // 读取失败，关闭连接
                                eprintln!("读取失败: {}", e);
                                // 处理读取失败
                                handle_disconnect(client_stream, token, &mut poll, &mut clients);
                            }
                        }
                    }
                }
            }
        }
    }
}

// 处理连接关闭和资源释放的函数
fn handle_disconnect(
    mut stream: TcpStream,  // 接受 TcpStream 的所有权并声明为可变
    token: Token,
    poll: &mut Poll,
    clients: &mut HashMap<Token, TcpStream>,
) {
    // 尝试关闭连接
    let _ = stream.shutdown(std::net::Shutdown::Both); // 忽略返回值
    // 从 poll 中注销
    let _ = poll.registry().deregister(&mut stream);  // 需要可变借用
    // 从 clients 中移除
    clients.remove(&token);
}