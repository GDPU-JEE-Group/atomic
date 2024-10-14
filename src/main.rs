use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use atomic::base::properties::Properties;
use atomic::util::log::Log;

fn main() {
    // Parameter processing
    let env_args:Vec<String>=env::args().collect();
    if env_args.len() != 3 || env_args[1] != "-c" {
        eprintln!("Usage: {} -c <config_file_path>", env_args[0]);
        std::process::exit(1);
    }

    let config_path=env_args[2].as_str();
    Properties::init(config_path);

    let log_path=PathBuf::from(Properties::get( "log.path", "./log"));
    println!("log_path:{}",Properties::get( "log.path", "./log"));
    println!("log_pathexists:{}",log_path.exists());
    // 初始化日志系统，设置最大日志池大小和最大缓冲区大小
    Log::init(100, 1024, log_path);

    // 测试不同级别的日志
    test_init();
}

pub fn test_init(){
    Log::d("MainModule", "This is a debug message.");
    Log::i("MainModule", "This is an info message.");
    Log::w("MainModule", "This is a warning message.");
    Log::e("MainModule", "This is an error message.");
    Log::f("MainModule", "This is a fatal error message.");
    Properties::print();
    
    println!("Hello, world!");
    thread::sleep(Duration::from_secs(2));
}



