use std::env;
use std::io::BufReader;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

mod config;
use config::Config;
use config::instance;
use config::init_config;
use config::update_config;
use config::print_config;
use config::read_config;

fn main() {
    let args: Vec<String> = env::args().collect();
    init_config(args);
    _clinet();
}

fn _clinet(){
    let config=instance();
    let address=format!("{}:{}",config.client.ip,config.client.port);

    match TcpStream::connect(&address) {
        Ok(mut _stream)=>{
            println!("成功连接服务器{}",    address);

            // request
            let request_line = "GET / HTTP/1.1\r\n";
            let host = format!("Host: {}\r\n",address);
            let user_agent = "User-Agent: curl/7.68.0\r\n";
            let accept = "Accept: */*\r\n\r\n";
            let space = "\r\n";
            let request = format!("{}{}{}{}{}", request_line, host, user_agent, accept,space);
            println!("=>\n{}",request);
            _stream.write_all(request.as_bytes()).unwrap();

            // read response
            let mut buf_reader = BufReader::new(&mut _stream);
            let mut response = String::new();
            buf_reader.read_to_string(&mut response).unwrap();

            // print response
            let mut part = response.splitn(2,"\r\n\r\n");// 根据2个换行判断 响应头、响应体
            let headers=part.next().unwrap_or("");
            let body=part.next().unwrap_or("");
            println!("Response.Headers: {}", headers);
            println!("Response.Body: \n{}", body);
        }
        Err(e)=>{
            println!("连接失败 {} {}",address,e);
        }
    }
}

//--------------------------old---------------------------
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

fn _v2_clinet(){//http
    match TcpStream::connect("127.0.0.1:8090") {
        Ok(mut _stream)=>{
            println!("成功连接服务器127.0.0.1:8090");

            // request
            let request = "GET / HTTP/1.1\r\n\
                Host: 127.0.0.1:8090\r\n\
                User-Agent: curl/7.68.0\r\n\
                Accept: */*\r\n\
                \r\n";
            _stream.write_all(request.as_bytes()).unwrap();

            // response
            let mut buf_reader = BufReader::new(&mut _stream);
            let mut response = String::new();
            buf_reader.read_to_string(&mut response).unwrap();
            println!("Response: {:#?}", response);
        }
        Err(e)=>{
            println!("连接失败 127.0.0.1:8090 {}",e);
        }
    }
}