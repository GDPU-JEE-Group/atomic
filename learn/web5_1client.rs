use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;
use std::io::Write;
fn main() {
    _clinet();
}
fn _v1_clinet(){
    match TcpStream::connect("127.0.0.1:8090") {
        Ok(_stream)=>{
            println!("成功连接服务器127.0.0.1:8090");
        }
        Err(e)=>{
            println!("连接失败 127.0.0.1:8090 {}",e);
        }
    }
}

fn _clinet(){
    match TcpStream::connect("127.0.0.1:8090") {
        Ok(mut _stream)=>{
            println!("成功连接服务器127.0.0.1:8090");

            // request
            let request="
                GET / HTTP/1.1\r\n\
                Host: 127.0.0.1:8090\r\n\
                User-Agent: curl/7.68.0\r\n\
                Accept: */*\r\n\
                \r\n";
            _stream.write_all(request.as_bytes()).unwrap();

            // response
            let buf_reader=BufReader::new(&mut _stream);
            let http_request:Vec<_>=buf_reader
            .lines()
            .map(|result|result.unwrap())
            .take_while(|line|!line.is_empty())
            .collect();
            println!("Request: {:#?}",http_request);
        }
        Err(e)=>{
            println!("连接失败 127.0.0.1:8090 {}",e);
        }
    }
}